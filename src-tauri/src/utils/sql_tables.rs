use serde::{Deserialize, Serialize};

use crate::app::get_app;

#[derive(Serialize, Deserialize, Debug)]
pub struct Table {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TableDetails {
    name: String,
    column_type: String,
}

#[tauri::command]
pub fn get_tables_from_db() -> Vec<Table> {
    let app = get_app().lock().unwrap();

    let mut return_tables: Vec<Table> = Vec::new();

    let mut stmt = app
        .connection
        .prepare(
            "
    SELECT TABLE_NAME
    FROM INFORMATION_SCHEMA.TABLES
    WHERE TABLE_TYPE = 'BASE TABLE'",
        )
        .unwrap();

    let person_iter = stmt
        .query_map([], |row| {
            Ok(Table {
                name: row.get(0)?, // Assuming name is a string
            })
        })
        .unwrap();

    for person in person_iter {
        let p = person.unwrap();
        return_tables.push(p);
    }
    std::mem::drop(app);
    return_tables
}

#[tauri::command]
pub fn get_table_details_from_db(table_name: &str) -> Vec<TableDetails> {
    let app = get_app().lock().unwrap();

    let mut return_tables: Vec<TableDetails> = Vec::new();

    let query = format!(
        "SELECT column_name, data_type
        FROM information_schema.columns
        WHERE table_name = '{}'",
        table_name.to_string()
    );

    let mut stmt = app.connection.prepare(query.as_str()).unwrap();

    let person_iter = stmt
        .query_map([], |row| {
            Ok(TableDetails {
                name: row.get(0)?,
                column_type: row.get(1)?,
            })
        })
        .unwrap();

    for person in person_iter {
        let p = person.unwrap();
        return_tables.push(p);
    }
    std::mem::drop(app);
    return_tables
}
