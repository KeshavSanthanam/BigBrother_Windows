use crate::database::{get_connection, models::{VerificationResult, TimelineEntry, CostEstimate, Verification}};
use tauri::AppHandle;
use serde_json::json;
use std::path::Path;

#[tauri::command]
pub async fn verify_task_with_claude(
    app: AppHandle,
    task_id: i64,
) -> Result<VerificationResult, String> {
    // Get task details
    let conn = get_connection(&app).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT title, description, min_duration, video_path FROM tasks WHERE id = ?1")
        .map_err(|e| e.to_string())?;

    let (title, description, min_duration, video_path): (String, Option<String>, i64, Option<String>) = stmt
        .query_row([task_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        })
        .map_err(|e| e.to_string())?;

    let video_path = video_path.ok_or("No video found for this task")?;

    // Get Claude API key
    let mut stmt = conn
        .prepare("SELECT claude_api_key FROM users WHERE id = 1")
        .map_err(|e| e.to_string())?;

    let api_key: Option<String> = stmt
        .query_row([], |row| row.get(0))
        .map_err(|e| e.to_string())?;

    let api_key = api_key.ok_or("Claude API key not set. Please configure it in settings.")?;

    // Extract frames from video
    let frames = extract_video_frames(app.clone(), video_path.clone(), 10).await?;

    // Get video duration (for now, we'll use min_duration as placeholder)
    let actual_duration_minutes = min_duration as f64 / 60.0;

    // Send frames to Claude API
    let result = send_to_claude_api(
        &api_key,
        &title,
        description.as_deref(),
        min_duration / 60,
        actual_duration_minutes,
        frames,
    )
    .await?;

    // Store verification result in database
    let verification_json = serde_json::to_string(&result).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO task_verifications (task_id, verified, ai_verification, ai_confidence, time_on_task, explanation)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            task_id,
            result.verified,
            verification_json,
            result.confidence,
            (result.time_on_task_minutes * 60.0) as i64,
            result.explanation
        ],
    )
    .map_err(|e| e.to_string())?;

    // Update task status based on verification
    let new_status = if result.verified { "completed" } else { "failed" };
    conn.execute(
        "UPDATE tasks SET status = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![new_status, task_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(result)
}

async fn send_to_claude_api(
    api_key: &str,
    title: &str,
    description: Option<&str>,
    required_duration_minutes: i64,
    actual_duration_minutes: f64,
    frames: Vec<String>, // Base64 encoded frames
) -> Result<VerificationResult, String> {
    let client = reqwest::Client::new();

    // Build messages with vision content
    let mut content_parts: Vec<serde_json::Value> = vec![json!({
        "type": "text",
        "text": format!(
            "You are verifying a productivity task completion.\n\n\
            Task Details:\n\
            - Title: {}\n\
            - Description: {}\n\
            - Required Duration: {} minutes\n\
            - Video Duration: {:.1} minutes\n\n\
            Analyze the provided video frames (1 frame every 10 seconds) and determine:\n\
            1. Was the user engaged in the described task?\n\
            2. For what percentage of the video was the task being performed?\n\
            3. Did they meet the minimum duration requirement?\n\
            4. Were there significant distractions or off-task behavior?\n\n\
            Provide your response in JSON format:\n\
            {{\n\
              \"verified\": true/false,\n\
              \"confidence\": 0-100,\n\
              \"time_on_task_minutes\": number,\n\
              \"explanation\": \"detailed explanation\",\n\
              \"issues\": [\"issue 1\", \"issue 2\"],\n\
              \"timeline\": [\n\
                {{\"timestamp\": \"00:00\", \"activity\": \"description\"}}\n\
              ]\n\
            }}",
            title,
            description.unwrap_or("N/A"),
            required_duration_minutes,
            actual_duration_minutes
        )
    })];

    // Add frames (limit to avoid token limits)
    for (i, frame_base64) in frames.iter().take(20).enumerate() {
        content_parts.push(json!({
            "type": "image",
            "source": {
                "type": "base64",
                "media_type": "image/jpeg",
                "data": frame_base64
            }
        }));
    }

    let request_body = json!({
        "model": "claude-3-5-sonnet-20241022",
        "max_tokens": 2048,
        "messages": [{
            "role": "user",
            "content": content_parts
        }]
    });

    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to send request to Claude API: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Claude API error: {}", error_text));
    }

    let response_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Claude API response: {}", e))?;

    // Extract text content from Claude's response
    let text_content = response_json["content"][0]["text"]
        .as_str()
        .ok_or("Failed to extract text from Claude response")?;

    // Parse JSON from Claude's response
    let verification: VerificationResult = serde_json::from_str(text_content)
        .map_err(|e| format!("Failed to parse verification result: {}. Response: {}", e, text_content))?;

    Ok(verification)
}

#[tauri::command]
pub async fn extract_video_frames(
    app: AppHandle,
    video_path: String,
    interval_seconds: u32,
) -> Result<Vec<String>, String> {
    // For now, return empty vec - will implement actual frame extraction with ffmpeg later
    // TODO: Use ffmpeg to extract frames at intervals and convert to base64
    println!("Extracting frames from video: {} at {} second intervals", video_path, interval_seconds);

    // Placeholder: In real implementation, we would:
    // 1. Use ffmpeg to extract frames: ffmpeg -i video.mp4 -vf "fps=1/10" frame_%04d.jpg
    // 2. Load each frame with image crate
    // 3. Encode to base64
    // 4. Return vector of base64 strings

    Ok(vec![])
}

#[tauri::command]
pub async fn get_verification_status(
    app: AppHandle,
    task_id: i64,
) -> Result<Option<Verification>, String> {
    let conn = get_connection(&app).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, task_id, verified, ai_verification, ai_confidence, time_on_task, explanation, verified_at
             FROM task_verifications
             WHERE task_id = ?1
             ORDER BY verified_at DESC
             LIMIT 1",
        )
        .map_err(|e| e.to_string())?;

    let verification = stmt
        .query_row([task_id], |row| {
            Ok(Verification {
                id: Some(row.get(0)?),
                task_id: row.get(1)?,
                verified: row.get(2)?,
                ai_verification: row.get(3)?,
                ai_confidence: row.get(4)?,
                time_on_task: row.get(5)?,
                explanation: row.get(6)?,
                verified_at: row.get(7)?,
            })
        })
        .optional()
        .map_err(|e| e.to_string())?;

    Ok(verification)
}

#[tauri::command]
pub async fn get_verification_cost_estimate(
    video_duration: u32,
) -> Result<CostEstimate, String> {
    // Rough estimate: 1 frame per 10 seconds
    // Each frame ~1000 tokens (for vision)
    // Plus ~500 tokens for text
    let frames_count = (video_duration / 10).min(20); // Cap at 20 frames
    let estimated_tokens = (frames_count * 1000 + 500) as i64;

    // Claude 3.5 Sonnet pricing (approximate)
    // Input: $3 per million tokens
    let estimated_cost_usd = (estimated_tokens as f64 / 1_000_000.0) * 3.0;

    Ok(CostEstimate {
        estimated_tokens,
        estimated_cost_usd,
    })
}
