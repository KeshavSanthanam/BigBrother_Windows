use crate::database::{get_connection, models::{RecordingStatus, Recording}};
use crate::recording::capture::{ScreenRecorder, enumerate_displays as enum_displays, enumerate_webcams as enum_webcams};
use crate::recording::combiner::VideoCombiner;
use crate::recording::{DisplayInfo, WebcamInfo};
use tauri::{AppHandle, State, Manager};
use std::sync::{Arc, Mutex};
use chrono::Utc;

pub struct RecordingState {
    pub status: Mutex<RecordingStatus>,
    pub current_recording: Mutex<Option<Recording>>,
    pub recorder: Mutex<Option<ScreenRecorder>>,
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
            recorder: Mutex::new(None),
        }
    }
}

#[tauri::command]
pub async fn enumerate_displays() -> Result<Vec<DisplayInfo>, String> {
    enum_displays()
}

#[tauri::command]
pub async fn enumerate_webcams() -> Result<Vec<WebcamInfo>, String> {
    enum_webcams()
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

    let app_dir = app.path().app_data_dir()
        .map_err(|e| format!("Failed to get app directory: {}", e))?;

    let videos_dir = app_dir.join("videos");
    std::fs::create_dir_all(&videos_dir)
        .map_err(|e| format!("Failed to create videos directory: {}", e))?;

    let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
    let output_base = videos_dir.join(format!("task_{}_rec_{}", task_id, timestamp));
    let output_path = output_base.to_str().ok_or("Invalid path")?.to_string();

    let displays = enum_displays()?;
    let webcams = enum_webcams()?;
    let webcam = webcams.first().cloned();

    let mut recorder = ScreenRecorder::new(
        displays.clone(),
        webcam.clone(),
        output_path.clone(),
    );

    recorder.start()?;

    status.is_recording = true;
    status.is_paused = false;
    status.duration = 0;
    status.task_id = Some(task_id);

    let mut recorder_guard = state.recorder.lock().unwrap();
    *recorder_guard = Some(recorder);

    let conn = get_connection(&app).map_err(|e| e.to_string())?;
    let start_time = Utc::now().to_rfc3339();
    let file_path = format!("{}_combined.mp4", output_path);

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

    println!("Started recording for task {}", task_id);

    Ok(format!("Recording started for task {}", task_id))
}

#[tauri::command]
pub async fn pause_recording(state: State<'_, Arc<RecordingState>>) -> Result<(), String> {
    let mut status = state.status.lock().unwrap();

    if !status.is_recording {
        return Err("No recording in progress".to_string());
    }

    status.is_paused = true;
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
    println!("Recording resumed");

    Ok(())
}

#[tauri::command]
pub async fn stop_recording(
    app: AppHandle,
    state: State<'_, Arc<RecordingState>>,
) -> Result<String, String> {
    // Early check and immediately set is_recording to false to prevent multiple calls
    let (task_id, final_duration, output_path) = {
        let mut status = state.status.lock().unwrap();

        if !status.is_recording {
            return Err("No recording in progress".to_string());
        }

        let task_id = status.task_id.ok_or("No task associated with recording")?;
        let final_duration = status.duration;

        // Immediately set to false to prevent re-entry
        status.is_recording = false;
        status.is_paused = false;
        let temp_duration = status.duration;
        status.duration = 0;
        let temp_task_id = status.task_id;
        status.task_id = None;

        println!("Stopping screen recording...");

        // Get output path from current_recording
        let current = state.current_recording.lock().unwrap();
        let output_path = if let Some(ref recording) = *current {
            recording.file_path.clone()
        } else {
            return Err("No current recording found".to_string());
        };

        (task_id, final_duration, output_path)
    }; // Release status lock here

    // Stop the recorder (outside of status lock)
    {
        let mut recorder_guard = state.recorder.lock().unwrap();
        if let Some(recorder) = recorder_guard.take() {
            drop(recorder);
        }
    }

    // Wait for FFmpeg to finish writing files - give it more time
    println!("Waiting for video files to finish writing...");
    std::thread::sleep(std::time::Duration::from_secs(5));

    let output_base = output_path.trim_end_matches("_combined.mp4");

    // Collect temp files and verify they're valid
    let mut temp_files = Vec::new();

    let mut idx = 0;
    loop {
        let temp_file = format!("{}_display_{}.mp4", output_base, idx);
        let path = std::path::Path::new(&temp_file);

        if path.exists() {
            // Check file size to ensure it's not empty/corrupted
            if let Ok(metadata) = std::fs::metadata(&temp_file) {
                if metadata.len() > 0 {
                    println!("Found display file: {} ({} bytes)", temp_file, metadata.len());
                    temp_files.push(temp_file);
                } else {
                    println!("Warning: Display file {} is empty, skipping", temp_file);
                }
            }
            idx += 1;
        } else {
            break;
        }
    }

    let webcam_file = format!("{}_webcam.mp4", output_base);
    let webcam_path = std::path::Path::new(&webcam_file);
    if webcam_path.exists() {
        if let Ok(metadata) = std::fs::metadata(&webcam_file) {
            if metadata.len() > 0 {
                println!("Found webcam file: {} ({} bytes)", webcam_file, metadata.len());
                temp_files.push(webcam_file);
            } else {
                println!("Warning: Webcam file is empty, skipping");
            }
        }
    }

    println!("Found {} valid video files to combine", temp_files.len());
    for (idx, file) in temp_files.iter().enumerate() {
        println!("  [{}] {}", idx, file);
    }

    // Combine videos if any were recorded
    if !temp_files.is_empty() {
        println!("Combining {} videos into grid layout...", temp_files.len());
        println!("Output file will be: {}", output_path);
        let combiner = VideoCombiner::new(temp_files.clone(), output_path.clone());

        match combiner.combine_grid() {
            Ok(_) => {
                println!("Videos combined successfully");
                // Clean up temporary files only if combine succeeded
                println!("Cleaning up temporary files...");
                for temp_file in &temp_files {
                    let _ = std::fs::remove_file(temp_file);
                }
            }
            Err(e) => {
                eprintln!("Error combining videos: {}", e);
                // Don't delete temp files if combining failed - user might want to recover them
                println!("Temporary files preserved at: {}_*.mp4", output_base);

                // For now, just use the first temp file as the output if combining fails
                if let Some(first_file) = temp_files.first() {
                    println!("Using first recording file as fallback: {}", first_file);
                    // Copy first file to the expected output path
                    let _ = std::fs::copy(first_file, &output_path);
                }
            }
        }
    } else {
        println!("Warning: No valid video files found to combine");
        return Err("No valid video files were recorded".to_string());
    }

    // Update database
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

        conn.execute(
            "UPDATE tasks
             SET status = 'completed', video_path = ?1, updated_at = datetime('now')
             WHERE id = ?2",
            rusqlite::params![&recording.file_path, task_id],
        )
        .map_err(|e| e.to_string())?;

        println!("Recording stopped successfully for task {}", task_id);

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
