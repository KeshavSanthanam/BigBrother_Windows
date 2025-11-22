use crate::database::{get_connection, models::Task};
use tauri::AppHandle;

#[tauri::command]
pub async fn create_task(
    app: AppHandle,
    title: String,
    description: Option<String>,
    due_date: String,
    min_duration: i64,
) -> Result<Task, String> {
    let conn = get_connection(&app).map_err(|e| e.to_string())?;

    let result = conn.execute(
        "INSERT INTO tasks (user_id, title, description, due_date, min_duration, status)
         VALUES (1, ?1, ?2, ?3, ?4, 'pending')",
        rusqlite::params![title, description, due_date, min_duration],
    );

    match result {
        Ok(_) => {
            let task_id = conn.last_insert_rowid();
            get_task(app, task_id).await
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub async fn get_all_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let conn = get_connection(&app).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, user_id, title, description, due_date, min_duration, status, video_path, created_at, updated_at
             FROM tasks
             ORDER BY due_date ASC",
        )
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map([], |row| {
            Ok(Task {
                id: Some(row.get(0)?),
                user_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                due_date: row.get(4)?,
                min_duration: row.get(5)?,
                status: row.get(6)?,
                video_path: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<Task>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(tasks)
}

#[tauri::command]
pub async fn get_task(app: AppHandle, id: i64) -> Result<Task, String> {
    let conn = get_connection(&app).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, user_id, title, description, due_date, min_duration, status, video_path, created_at, updated_at
             FROM tasks WHERE id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let task = stmt
        .query_row([id], |row| {
            Ok(Task {
                id: Some(row.get(0)?),
                user_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                due_date: row.get(4)?,
                min_duration: row.get(5)?,
                status: row.get(6)?,
                video_path: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(task)
}

#[tauri::command]
pub async fn update_task(app: AppHandle, id: i64, task: Task) -> Result<Task, String> {
    let conn = get_connection(&app).map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE tasks
         SET title = ?1, description = ?2, due_date = ?3, min_duration = ?4, status = ?5, video_path = ?6, updated_at = datetime('now')
         WHERE id = ?7",
        rusqlite::params![
            task.title,
            task.description,
            task.due_date,
            task.min_duration,
            task.status,
            task.video_path,
            id
        ],
    )
    .map_err(|e| e.to_string())?;

    get_task(app, id).await
}

#[tauri::command]
pub async fn delete_task(app: AppHandle, id: i64) -> Result<(), String> {
    let conn = get_connection(&app).map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM tasks WHERE id = ?1", [id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_pending_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let conn = get_connection(&app).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, user_id, title, description, due_date, min_duration, status, video_path, created_at, updated_at
             FROM tasks
             WHERE status = 'pending'
             ORDER BY due_date ASC",
        )
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map([], |row| {
            Ok(Task {
                id: Some(row.get(0)?),
                user_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                due_date: row.get(4)?,
                min_duration: row.get(5)?,
                status: row.get(6)?,
                video_path: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<Task>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(tasks)
}

#[tauri::command]
pub async fn get_completed_tasks(app: AppHandle) -> Result<Vec<Task>, String> {
    let conn = get_connection(&app).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, user_id, title, description, due_date, min_duration, status, video_path, created_at, updated_at
             FROM tasks
             WHERE status IN ('completed', 'failed')
             ORDER BY updated_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let tasks = stmt
        .query_map([], |row| {
            Ok(Task {
                id: Some(row.get(0)?),
                user_id: row.get(1)?,
                title: row.get(2)?,
                description: row.get(3)?,
                due_date: row.get(4)?,
                min_duration: row.get(5)?,
                status: row.get(6)?,
                video_path: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<Task>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(tasks)
}
