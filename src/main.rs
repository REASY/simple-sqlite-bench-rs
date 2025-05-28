mod errors;

use crate::errors::AppResult;
use clap::Parser;
use sqlx::{
    ConnectOptions, Connection,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous},
};
use std::fs::{exists, remove_file};
use std::ops::DerefMut;
use std::str::FromStr;
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug)]
struct Row {
    tag_valie: String,
    block_id: i16,
    offset: i32,
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
    /// The synchronous flag, https://www.sqlite.org/pragma.html#pragma_synchronous
    #[arg(value_enum, short, long, default_value = "normal")]
    synchronous_flag: SqliteSynchronous,
    /// The journal mode, https://www.sqlite.org/pragma.html#pragma_journal_mode
    #[arg(value_enum, short, long, default_value = "wal")]
    journal_mode: SqliteJournalMode,
}

fn remove_file_if_exists(path: &str) -> AppResult<()> {
    if exists(path)? {
        remove_file(path)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> AppResult<()> {
    let args = Args::parse();

    clean_up().expect("Failed to clean up database");

    run(args).await?;

    Ok(())
}

async fn run(args: Args) -> AppResult<()> {
    // Configure database connection
    let mut conn = SqliteConnectOptions::from_str("sqlite:benchmark.db")?
        .journal_mode(args.journal_mode)
        .synchronous(args.synchronous_flag)
        .create_if_missing(true)
        .connect()
        .await?;

    // Create table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tag_data (
            tagvalue TEXT NOT NULL,
            block_id INTEGER NOT NULL,
            offset INTEGER NOT NULL
        )"#,
    )
    .execute(&mut conn)
    .await?;

    // Create index
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_tagvalue ON tag_data (tagvalue)"#,
    )
    .execute(&mut conn)
    .await?;

    let total_records = args.count * 1_000_000;

    // Insertion configuration
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
                tag_valie: unique_value.clone(),
                block_id: (i % 32768) as i16,
                offset: i as i32,
            };
            rows.push(row);
        }

        // Build SQL query
        let placeholders = vec!["(?, ?, ?)".to_string(); current_batch_size];
        let sql = format!(
            "INSERT INTO tag_data (tagvalue, block_id, offset) VALUES {}",
            placeholders.join(", ")
        );

        // Prepare query
        let mut query = sqlx::query(&sql);
        for row in rows {
            query = query
                .bind(row.tag_valie)
                .bind(row.block_id)
                .bind(row.offset);
        }

        // Execute in transaction
        let mut tx = conn.begin().await?;
        query.execute(tx.deref_mut()).await?;
        tx.commit().await?;
    }

    let duration = start_time.elapsed();
    println!("Insert benchmark results:");
    println!("Records inserted: {}M", args.count);
    println!("Batch size: {}", args.batch_size);
    println!("Unique tag values: {}", args.unique_tag_values);
    println!("Synchronous flag: {:?}", args.synchronous_flag);
    println!("Journal mode: {:?}", args.journal_mode);
    println!("Total time: {:.2?}", duration);
    println!(
        "Average write throughput: {:.2} records/sec",
        total_records as f64 / duration.as_secs_f64()
    );

    Ok(())
}

fn clean_up() -> AppResult<()> {
    remove_file_if_exists("benchmark.db")?;
    remove_file_if_exists("benchmark.db-shm")?;
    remove_file_if_exists("benchmark.db-wal")?;
    Ok(())
}
