mod errors;

use crate::errors::AppResult;
use clap::{Parser, ValueEnum};
use log::LevelFilter;
use sqlx::{
    ConnectOptions, Connection, QueryBuilder, Sqlite, SqliteConnection, Transaction,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
};
use std::fs;
use std::fs::{exists, remove_file};
use std::ops::DerefMut;
use std::str::FromStr;
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug, Clone, ValueEnum)]
enum SqliteSchemaType {
    SingleTable,
    MappingAndDataTables,
    Fts5Table,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of millions of records to insert
    #[arg(short, long, default_value_t = 10)]
    count: usize,
    /// The batch size
    #[arg(short, long, default_value_t = 1000, value_parser=clap::value_parser!(u16).range(10..50001))]
    batch_size: u16,
    /// The number of unique tag values
    #[arg(short, long, default_value_t = 20000)]
    unique_tag_values: u32,
    /// The type of SQLite schema
    #[arg(value_enum, short, long, default_value_t = SqliteSchemaType::SingleTable)]
    type_schema: SqliteSchemaType,
    /// The synchronous flag, https://www.sqlite.org/pragma.html#pragma_synchronous
    #[arg(value_enum, short, long, default_value = "normal")]
    synchronous_flag: SqliteSynchronous,
    /// The journal mode, https://www.sqlite.org/pragma.html#pragma_journal_mode
    #[arg(value_enum, short, long, default_value = "wal")]
    journal_mode: SqliteJournalMode,
}

#[derive(Debug)]
struct Row {
    tag_value: String,
    block_id: i16,
    offset: i64,
}

fn remove_file_if_exists(path: &str) -> AppResult<()> {
    if exists(path)? {
        remove_file(path)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> AppResult<()> {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let args = Args::parse();

    clean_up().expect("Failed to clean up database");

    run(args).await?;

    Ok(())
}

async fn run(args: Args) -> AppResult<()> {
    let db_name = match &args.type_schema {
        SqliteSchemaType::SingleTable => "single_table.db",
        SqliteSchemaType::MappingAndDataTables => "mapping_and_data_tables.db",
        SqliteSchemaType::Fts5Table => "fts_5_table.db",
    };

    // Configure database connection
    let mut conn = SqliteConnectOptions::from_str(format!("sqlite:{}", db_name).as_str())?
        .journal_mode(args.journal_mode)
        .synchronous(args.synchronous_flag)
        .create_if_missing(true)
        .connect()
        .await?;

    prepare_tables(&args, &mut conn).await?;

    let total_records = args.count * 1_000_000;
    let batch_size = args.batch_size as usize;
    let num_batches = (total_records + batch_size - 1) / batch_size;
    let unique_values: Vec<String> = (0..args.unique_tag_values)
        .map(|_| Uuid::new_v4().to_string())
        .collect();

    let start_time = Instant::now();

    for batch_idx in 0..num_batches {
        let start = batch_idx * batch_size;
        let end = (start + batch_size).min(total_records);
        let current_batch_size = end - start;

        let mut rows = Vec::with_capacity(current_batch_size * 3);

        // Generate batch data
        for i in start..end {
            let unique_value = &unique_values[i % unique_values.len()];
            let row = Row {
                tag_value: unique_value.clone(),
                block_id: (i % 32768) as i16,
                offset: i as i64,
            };
            rows.push(row);
        }

        // Execute in transaction
        let mut tx = conn.begin().await?;

        match &args.type_schema {
            SqliteSchemaType::SingleTable => {
                insert_batch_single_table(&rows, &mut tx).await?;
            }
            SqliteSchemaType::MappingAndDataTables => {
                insert_batch_mapping_and_data_tables(&rows, &mut tx).await?;
            }
            SqliteSchemaType::Fts5Table => {
                insert_batch_fts5_table(&rows, &mut tx).await?;
            }
        }
        tx.commit().await?;
    }
    conn.close().await?;

    let duration = start_time.elapsed();
    let file_size = fs::metadata(db_name)?.len() as f64 / 1024.0 / 1024.0;

    println!("Insert benchmark results:");
    println!("Records inserted: {}M", args.count);
    println!("Batch size: {}", args.batch_size);
    println!("Unique tag values: {}", args.unique_tag_values);
    println!("The type of schema: {:?}", args.type_schema);
    println!("Synchronous flag: {:?}", args.synchronous_flag);
    println!("Journal mode: {:?}", args.journal_mode);
    println!("Total time: {:.2?}", duration);
    println!("File size (MiB): {:.2?}", file_size);
    println!(
        "Average write throughput: {:.2} records/sec",
        total_records as f64 / duration.as_secs_f64()
    );

    Ok(())
}

async fn insert_batch_fts5_table(batch: &[Row], tx: &mut Transaction<'_, Sqlite>) -> AppResult<()> {
    let conn = tx.deref_mut();
    // ------------------------------------------------------------------
    // bulk-insert the incoming rows
    // QueryBuilder expands the VALUES list and binds for us
    // ------------------------------------------------------------------
    let mut qb = QueryBuilder::<Sqlite>::new("INSERT INTO tag_data(tag_value, block_id, offset) ");
    qb.push_values(batch, |mut q, row| {
        q.push_bind(&row.tag_value)
            .push_bind(row.block_id.to_string())
            .push_bind(row.offset.to_string());
    });
    qb.build().execute(&mut *conn).await?;
    Ok(())
}

async fn insert_batch_single_table(
    batch: &[Row],
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {
    let conn = tx.deref_mut();
    // ------------------------------------------------------------------
    // bulk-insert the incoming rows
    // QueryBuilder expands the VALUES list and binds for us
    // ------------------------------------------------------------------
    let mut qb = QueryBuilder::<Sqlite>::new("INSERT INTO tag_data(tag_value, block_id, offset) ");
    qb.push_values(batch, |mut q, row| {
        q.push_bind(&row.tag_value)
            .push_bind(row.block_id)
            .push_bind(row.offset);
    });
    qb.build().execute(&mut *conn).await?;
    Ok(())
}

async fn insert_batch_mapping_and_data_tables(
    batch: &[Row],
    tx: &mut Transaction<'_, Sqlite>,
) -> AppResult<()> {
    let conn = tx.deref_mut();
    // ------------------------------------------------------------------
    // 1) make sure the temp table exists (run once per connection)
    // ------------------------------------------------------------------
    sqlx::query(
        r#"
        CREATE TEMP TABLE IF NOT EXISTS temp_batch
        ( tag_value TEXT NOT NULL
        , block_id INTEGER NOT NULL
        , offset   INTEGER NOT NULL
        );
        "#,
    )
    .execute(&mut *conn)
    .await?;

    // ------------------------------------------------------------------
    // 2) clear previous contents
    // ------------------------------------------------------------------
    sqlx::query("DELETE FROM temp_batch;")
        .execute(&mut *conn)
        .await?;

    // ------------------------------------------------------------------
    // 3) bulk-insert the incoming rows
    //    QueryBuilder expands the VALUES list and binds for us
    // ------------------------------------------------------------------
    let mut qb =
        QueryBuilder::<Sqlite>::new("INSERT INTO temp_batch(tag_value, block_id, offset) ");
    qb.push_values(batch, |mut q, row| {
        q.push_bind(&row.tag_value)
            .push_bind(row.block_id)
            .push_bind(row.offset);
    });
    qb.build().execute(&mut *conn).await?;

    // ------------------------------------------------------------------
    // 4) UPSERT names into tag_mapping
    // ------------------------------------------------------------------
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO tag_mapping(tag_value)
        SELECT tag_value
        FROM   temp_batch;
        "#,
    )
    .execute(&mut *conn)
    .await?;

    // ------------------------------------------------------------------
    // 5) finally insert rows into tag_data
    // ------------------------------------------------------------------
    sqlx::query(
        r#"
        INSERT INTO tag_data(tag_id, block_id, offset)
        SELECT tm.id, tb.block_id, tb.offset
        FROM   temp_batch AS tb
        JOIN   tag_mapping AS tm USING(tag_value);
        "#,
    )
    .execute(conn)
    .await?;

    Ok(())
}

async fn prepare_tables(args: &Args, conn: &mut SqliteConnection) -> AppResult<()> {
    match args.type_schema {
        SqliteSchemaType::SingleTable => {
            prepare_single_table(conn).await?;
        }
        SqliteSchemaType::MappingAndDataTables => {
            prepare_mapping_and_data_tables(conn).await?;
        }
        SqliteSchemaType::Fts5Table => {
            prepare_fts5_table(conn).await?;
        }
    }
    Ok(())
}

async fn prepare_fts5_table(conn: &mut SqliteConnection) -> AppResult<()> {
    // Create table
    sqlx::query(
        r#"
        CREATE VIRTUAL TABLE IF NOT EXISTS tag_data USING fts5(tag_value,block_id,offset)"#,
    )
    .execute(conn)
    .await?;

    Ok(())
}

async fn prepare_single_table(conn: &mut SqliteConnection) -> AppResult<()> {
    // Create table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tag_data (
            tag_value TEXT NOT NULL,
            block_id INTEGER NOT NULL,
            offset INTEGER NOT NULL
        )"#,
    )
    .execute(&mut *conn)
    .await?;

    // Create index
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_tagvalue ON tag_data (tag_value)"#,
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

async fn prepare_mapping_and_data_tables(conn: &mut SqliteConnection) -> AppResult<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tag_mapping (
            id INTEGER PRIMARY KEY,
            tag_value TEXT UNIQUE NOT NULL
        )"#,
    )
    .execute(&mut *conn)
    .await?;

    // Create table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tag_data (
            tag_id INTEGER NOT NULL,
            block_id INTEGER NOT NULL,
            offset INTEGER NOT NULL,
            FOREIGN KEY(tag_id) REFERENCES tag_mapping(id)
        )"#,
    )
    .execute(&mut *conn)
    .await?;

    // Create optimized index on integer column
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_tag_id ON tag_data (tag_id)"#,
    )
    .execute(conn)
    .await?;
    Ok(())
}

fn clean_up() -> AppResult<()> {
    remove_file_if_exists("single_table.db")?;
    remove_file_if_exists("single_table.db-shm")?;
    remove_file_if_exists("single_table.db-wal")?;

    remove_file_if_exists("mapping_and_data_tables.db")?;
    remove_file_if_exists("mapping_and_data_tables.db-shm")?;
    remove_file_if_exists("mapping_and_data_tables.db-wal")?;

    remove_file_if_exists("fts_5_table.db")?;
    remove_file_if_exists("fts_5_table.db-shm")?;
    remove_file_if_exists("fts_5_table.db-wal")?;
    Ok(())
}
