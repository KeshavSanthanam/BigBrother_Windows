mod database;
mod commands;

use commands::{tasks, recording, verification, settings};
use std::sync::Arc;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Initialize database
            database::init_database(app.handle())?;

            // Initialize recording state
            let recording_state = Arc::new(recording::RecordingState::new());
            app.manage(recording_state);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Task commands
            tasks::create_task,
            tasks::get_all_tasks,
            tasks::get_task,
            tasks::update_task,
            tasks::delete_task,
            tasks::get_pending_tasks,
            // Recording commands
            recording::start_recording,
            recording::pause_recording,
            recording::resume_recording,
            recording::stop_recording,
            recording::get_recording_status,
            recording::update_recording_duration,
            // Verification commands
            verification::verify_task_with_claude,
            verification::get_verification_status,
            verification::extract_video_frames,
            verification::get_verification_cost_estimate,
            // Settings commands
            settings::set_claude_api_key,
            settings::get_claude_api_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
