// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app::{get_app, use_app};
use duckdb::{
    arrow::{array::RecordBatch, util::pretty::print_batches},
    Result,
};

use std::fs::{self, read_dir};

mod app;
mod error;
mod prelude;
mod task;
mod utils;
use utils::sql_tables::*;

fn main() -> Result<()> {
    use_app();

    let app = get_app().lock().unwrap();

    app.connection.execute_batch(
        r"CREATE SEQUENCE seq;
          CREATE TABLE person (
                  id              INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  name            TEXT NOT NULL
                  );
        ",
    )?;

    app.connection.execute_batch(
        r"CREATE SEQUENCE seq;
          CREATE TABLE customer (
                  id              INTEGER PRIMARY KEY DEFAULT NEXTVAL('seq'),
                  customerName            TEXT NOT NULL
                  );
        ",
    )?;

    app.connection.execute(
        "INSERT INTO person (name) VALUES (?)",
        [String::from("J").to_string()],
    )?;

    let mut stmt = app.connection.prepare(
        "
    SELECT TABLE_NAME
    FROM INFORMATION_SCHEMA.TABLES
    WHERE TABLE_TYPE = 'BASE TABLE'",
    )?;
    let rbs: Vec<RecordBatch> = stmt.query_arrow([])?.collect();
    print_batches(&rbs).unwrap();

    std::mem::drop(app);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_tables_from_db,
            get_table_details_from_db
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
