pub mod schema;
pub mod models;

use rusqlite::{Connection, Result};
use std::path::PathBuf;
use tauri::AppHandle;

pub fn init_database(app: &AppHandle) -> Result<Connection> {
    let app_dir = app.path().app_data_dir()
        .expect("Failed to get app data directory");

    std::fs::create_dir_all(&app_dir)
        .expect("Failed to create app directory");

    let db_path = app_dir.join("bigbrother.db");
    let conn = Connection::open(db_path)?;

    schema::create_tables(&conn)?;

    Ok(conn)
}

pub fn get_connection(app: &AppHandle) -> Result<Connection> {
    let app_dir = app.path().app_data_dir()
        .expect("Failed to get app data directory");
    let db_path = app_dir.join("bigbrother.db");
    Connection::open(db_path)
}
