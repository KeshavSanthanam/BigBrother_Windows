use crate::database::{get_connection, models::{VerificationResult, CostEstimate, Verification}};
use tauri::AppHandle;
use serde_json::json;
use rusqlite::OptionalExtension;

#[tauri::command]
pub async fn verify_task_with_claude(
    app: AppHandle,
    task_id: i64,
) -> Result<VerificationResult, String> {
    // Get task details and API key first, then drop connection before async call
    let (title, description, min_duration, video_path, api_key) = {
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

        (title, description, min_duration, video_path, api_key)
    }; // conn is dropped here

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

    let conn = get_connection(&app).map_err(|e| e.to_string())?;
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
        "UPDATE tasks SET status = ?1, updated_at = datetime('now', 'localtime') WHERE id = ?2",
        rusqlite::params![new_status, task_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(result)
}

async fn get_available_models(
    client: &reqwest::Client,
    api_key: &str,
) -> Result<Vec<String>, String> {
    // Query the Anthropic API for available models
    let response = client
        .get("https://api.anthropic.com/v1/models")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .send()
        .await
        .map_err(|e| format!("Failed to query models API: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Models API returned error: {}", response.status()));
    }

    let models_json: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse models response: {}", e))?;

    // Extract model IDs from response
    let mut available_models: Vec<String> = models_json["data"]
        .as_array()
        .ok_or("Invalid models response format")?
        .iter()
        .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
        .filter(|id| {
            // Prefer Sonnet models, but also include Opus as fallback
            (id.contains("claude") && id.contains("sonnet")) ||
            (id.contains("claude") && id.contains("opus"))
        })
        .collect();

    // Sort by model name (descending) to get newest first
    // Model names are like "claude-3-5-sonnet-20241022" so alphabetical sort works
    available_models.sort_by(|a, b| b.cmp(a));

    println!("Available Claude models: {:?}", available_models);

    if available_models.is_empty() {
        return Err("No suitable Claude models found".to_string());
    }

    Ok(available_models)
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
    for frame_base64 in frames.iter().take(20) {
        content_parts.push(json!({
            "type": "image",
            "source": {
                "type": "base64",
                "media_type": "image/jpeg",
                "data": frame_base64
            }
        }));
    }

    // Query API for available models, fallback to hardcoded list if that fails
    let models_to_try = get_available_models(&client, api_key).await.unwrap_or_else(|e| {
        eprintln!("Failed to query available models: {}. Using fallback list.", e);
        vec![
            "claude-3-5-sonnet-20241022".to_string(),  // Latest as of Oct 2024
            "claude-3-5-sonnet-20240620".to_string(),  // June 2024
            "claude-3-opus-20240229".to_string(),      // Fallback to Opus
        ]
    });

    let mut last_error = String::new();

    for model in &models_to_try {
        let request_body = json!({
            "model": model,
            "max_tokens": 2048,
            "messages": [{
                "role": "user",
                "content": content_parts.clone()
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

        if response.status().is_success() {
            // Model worked, continue with processing
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

            return Ok(verification);
        } else {
            // Model didn't work, save error and try next
            let error_text = response.text().await.unwrap_or_default();
            last_error = format!("Model {} failed: {}", model, error_text);
            eprintln!("{}", last_error);
            continue;
        }
    }

    // If we got here, all models failed
    Err(format!("All models failed. Last error: {}", last_error))
}

#[tauri::command]
pub async fn extract_video_frames(
    _app: AppHandle,
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
