use crate::database::get_connection;
use tauri::AppHandle;

#[tauri::command]
pub async fn set_claude_api_key(app: AppHandle, api_key: String) -> Result<(), String> {
    let conn = get_connection(&app).map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE users SET claude_api_key = ?1 WHERE id = 1",
        rusqlite::params![api_key],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_claude_api_key(app: AppHandle) -> Result<Option<String>, String> {
    let conn = get_connection(&app).map_err(|e| e.to_string())?;

    let api_key: Option<String> = conn
        .query_row("SELECT claude_api_key FROM users WHERE id = 1", [], |row| {
            row.get(0)
        })
        .map_err(|e| e.to_string())?;

    Ok(api_key)
}
