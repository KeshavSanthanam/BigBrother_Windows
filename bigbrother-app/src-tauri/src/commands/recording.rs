use crate::database::{get_connection, models::{RecordingStatus, Recording}};
use tauri::{AppHandle, State};
use std::sync::{Arc, Mutex};
use chrono::Utc;

pub struct RecordingState {
    pub status: Mutex<RecordingStatus>,
    pub current_recording: Mutex<Option<Recording>>,
}

impl RecordingState {
    pub fn new() -> Self {
        RecordingState {
            status: Mutex::new(RecordingStatus {
                is_recording: false,
                is_paused: false,
                duration: 0,
                task_id: None,
            }),
            current_recording: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub async fn start_recording(
    app: AppHandle,
    state: State<'_, Arc<RecordingState>>,
    task_id: i64,
) -> Result<String, String> {
    let mut status = state.status.lock().unwrap();

    if status.is_recording {
        return Err("Recording already in progress".to_string());
    }

    // Initialize recording
    status.is_recording = true;
    status.is_paused = false;
    status.duration = 0;
    status.task_id = Some(task_id);

    // Create recording record in database
    let conn = get_connection(&app).map_err(|e| e.to_string())?;
    let start_time = Utc::now().to_rfc3339();
    let file_path = format!("recording_task_{}.mp4", task_id);

    conn.execute(
        "INSERT INTO recordings (task_id, start_time, file_path, status)
         VALUES (?1, ?2, ?3, 'recording')",
        rusqlite::params![task_id, start_time, file_path],
    )
    .map_err(|e| e.to_string())?;

    let recording_id = conn.last_insert_rowid();

    let mut current = state.current_recording.lock().unwrap();
    *current = Some(Recording {
        id: Some(recording_id),
        task_id,
        duration: 0,
        start_time: start_time.clone(),
        end_time: None,
        file_path: file_path.clone(),
        status: "recording".to_string(),
    });

    // TODO: Start actual screen recording
    println!("Starting recording for task {}", task_id);

    Ok(format!("Recording started for task {}", task_id))
}

#[tauri::command]
pub async fn pause_recording(state: State<'_, Arc<RecordingState>>) -> Result<(), String> {
    let mut status = state.status.lock().unwrap();

    if !status.is_recording {
        return Err("No recording in progress".to_string());
    }

    status.is_paused = true;

    // TODO: Pause actual screen recording
    println!("Recording paused");

    Ok(())
}

#[tauri::command]
pub async fn resume_recording(state: State<'_, Arc<RecordingState>>) -> Result<(), String> {
    let mut status = state.status.lock().unwrap();

    if !status.is_recording {
        return Err("No recording in progress".to_string());
    }

    status.is_paused = false;

    // TODO: Resume actual screen recording
    println!("Recording resumed");

    Ok(())
}

#[tauri::command]
pub async fn stop_recording(
    app: AppHandle,
    state: State<'_, Arc<RecordingState>>,
) -> Result<String, String> {
    let mut status = state.status.lock().unwrap();

    if !status.is_recording {
        return Err("No recording in progress".to_string());
    }

    let task_id = status.task_id.ok_or("No task associated with recording")?;

    // TODO: Stop actual screen recording and get final duration
    let final_duration = status.duration;

    status.is_recording = false;
    status.is_paused = false;
    status.duration = 0;
    status.task_id = None;

    // Update recording in database
    let conn = get_connection(&app).map_err(|e| e.to_string())?;
    let end_time = Utc::now().to_rfc3339();

    let current = state.current_recording.lock().unwrap();
    if let Some(recording) = current.as_ref() {
        conn.execute(
            "UPDATE recordings
             SET duration = ?1, end_time = ?2, status = 'completed'
             WHERE id = ?3",
            rusqlite::params![final_duration, end_time, recording.id],
        )
        .map_err(|e| e.to_string())?;

        // Update task status and video path
        conn.execute(
            "UPDATE tasks
             SET status = 'completed', video_path = ?1, updated_at = datetime('now')
             WHERE id = ?2",
            rusqlite::params![&recording.file_path, task_id],
        )
        .map_err(|e| e.to_string())?;

        println!("Recording stopped for task {}", task_id);

        Ok(recording.file_path.clone())
    } else {
        Err("No current recording found".to_string())
    }
}

#[tauri::command]
pub async fn get_recording_status(
    state: State<'_, Arc<RecordingState>>,
) -> Result<RecordingStatus, String> {
    let status = state.status.lock().unwrap();
    Ok(status.clone())
}

#[tauri::command]
pub async fn update_recording_duration(
    state: State<'_, Arc<RecordingState>>,
    duration: i64,
) -> Result<(), String> {
    let mut status = state.status.lock().unwrap();
    status.duration = duration;
    Ok(())
}
