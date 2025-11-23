mod database;
mod commands;
mod recording;

use commands::{tasks, recording as recording_commands, verification, settings, utils};
use std::sync::Arc;
use tauri::Manager;

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
            let recording_state = Arc::new(recording_commands::RecordingState::new());
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
            tasks::get_completed_tasks,
            // Recording commands
            recording_commands::start_recording,
            recording_commands::pause_recording,
            recording_commands::resume_recording,
            recording_commands::stop_recording,
            recording_commands::get_recording_status,
            recording_commands::update_recording_duration,
            recording_commands::enumerate_displays,
            recording_commands::enumerate_webcams,
            // Verification commands
            verification::verify_task_with_claude,
            verification::get_verification_status,
            verification::extract_video_frames,
            verification::get_verification_cost_estimate,
            // Settings commands
            settings::set_claude_api_key,
            settings::get_claude_api_key,
            // Utility commands
            utils::open_video_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
