use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: Option<i64>,
    pub user_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub due_date: String,  // ISO 8601 format
    pub min_duration: i64, // in seconds
    pub status: String,    // 'pending', 'in_progress', 'completed', 'failed'
    pub video_path: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Recording {
    pub id: Option<i64>,
    pub task_id: i64,
    pub duration: i64,     // in seconds
    pub start_time: String,
    pub end_time: Option<String>,
    pub file_path: String,
    pub status: String,    // 'recording', 'paused', 'completed', 'processing'
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Verification {
    pub id: Option<i64>,
    pub task_id: i64,
    pub verified: bool,
    pub ai_verification: Option<String>, // JSON blob
    pub ai_confidence: Option<i64>,
    pub time_on_task: Option<i64>,       // in seconds
    pub explanation: Option<String>,
    pub verified_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationResult {
    pub verified: bool,
    pub confidence: i64,
    pub time_on_task_minutes: f64,
    pub explanation: String,
    pub issues: Vec<String>,
    pub timeline: Vec<TimelineEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimelineEntry {
    pub timestamp: String,
    pub activity: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordingStatus {
    pub is_recording: bool,
    pub is_paused: bool,
    pub duration: i64,
    pub task_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<i64>,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub claude_api_key: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostEstimate {
    pub estimated_tokens: i64,
    pub estimated_cost_usd: f64,
}
