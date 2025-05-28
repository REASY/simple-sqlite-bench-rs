#!/bin/bash

set -o errexit -o pipefail -o noclobber -o nounset -o xtrace

declare -a BATCH_SIZES=("1000" "10000")
declare -a UNIQUE_TAG_VALUES=("10000" "20000")
declare -a TYPE_SCHEMAS=("single-table" "mapping-and-data-tables" "fts5-table")
declare -a SYNCHRONOUS_FLAGS=("normal" "off")
declare -a JOURNAL_MODES=("wal" "off")

cargo build --release

for batch_size in "${BATCH_SIZES[@]}"
do
    for unique_tag_value in "${UNIQUE_TAG_VALUES[@]}"
    do
      for type_schema in "${TYPE_SCHEMAS[@]}"
      do
        for synchronous_flag in "${SYNCHRONOUS_FLAGS[@]}"
        do
          for journal_mode in "${JOURNAL_MODES[@]}"
          do
              /usr/bin/time -pv ./target/release/simple-sqlite-bench-rs --batch-size "$batch_size" --unique-tag-values "$unique_tag_value" --type-schema "$type_schema" --synchronous-flag "$synchronous_flag" --journal-mode "$journal_mode" --count 10
          done
        done
      done
    done
done