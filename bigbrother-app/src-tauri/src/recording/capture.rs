use super::{DisplayInfo, WebcamInfo};
use std::process::Command;
use std::path::Path;

#[cfg(target_os = "windows")]
pub fn enumerate_displays() -> Result<Vec<DisplayInfo>, String> {
    // Use Windows API to enumerate displays
    // For now, return mock data - full implementation would use windows-rs crate

    // TODO: Implement using Windows Graphics Capture API
    // This requires:
    // 1. windows::Graphics::Capture::GraphicsCaptureItem
    // 2. Enumerate monitors using EnumDisplayMonitors
    // 3. Get monitor info for each display

    println!("Enumerating displays...");

    // Placeholder implementation
    Ok(vec![
        DisplayInfo {
            id: 0,
            name: "Primary Display".to_string(),
            width: 1920,
            height: 1080,
            is_primary: true,
        },
    ])
}

#[cfg(not(target_os = "windows"))]
pub fn enumerate_displays() -> Result<Vec<DisplayInfo>, String> {
    Err("Display enumeration only supported on Windows".to_string())
}

pub fn enumerate_webcams() -> Result<Vec<WebcamInfo>, String> {
    println!("Enumerating webcams...");

    // Use FFmpeg to list video devices
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("ffmpeg")
            .args(&["-list_devices", "true", "-f", "dshow", "-i", "dummy"])
            .output();

        match output {
            Ok(result) => {
                let stderr = String::from_utf8_lossy(&result.stderr);
                // Parse webcams from FFmpeg output
                // Lines like: [dshow @ ...] "Integrated Camera" (video)

                let mut webcams = Vec::new();
                for line in stderr.lines() {
                    if line.contains("(video)") && line.contains("\"") {
                        if let Some(start) = line.find('"') {
                            if let Some(end) = line[start+1..].find('"') {
                                let name = &line[start+1..start+1+end];
                                webcams.push(WebcamInfo {
                                    id: name.to_string(),
                                    name: name.to_string(),
                                });
                            }
                        }
                    }
                }

                if webcams.is_empty() {
                    // Return placeholder if none found
                    Ok(vec![WebcamInfo {
                        id: "0".to_string(),
                        name: "Default Webcam".to_string(),
                    }])
                } else {
                    Ok(webcams)
                }
            }
            Err(e) => {
                // FFmpeg not installed or error
                println!("FFmpeg error: {}", e);
                Ok(vec![WebcamInfo {
                    id: "0".to_string(),
                    name: "Default Webcam".to_string(),
                }])
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        Ok(vec![])
    }
}

pub struct ScreenRecorder {
    pub displays: Vec<DisplayInfo>,
    pub webcam: Option<WebcamInfo>,
    pub output_path: String,
    display_processes: Vec<std::process::Child>,
    webcam_process: Option<std::process::Child>,
}

impl ScreenRecorder {
    pub fn new(displays: Vec<DisplayInfo>, webcam: Option<WebcamInfo>, output_path: String) -> Self {
        Self {
            displays,
            webcam,
            output_path,
            display_processes: Vec::new(),
            webcam_process: None,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("Starting screen recording...");
        println!("Recording {} displays", self.displays.len());

        #[cfg(target_os = "windows")]
        {
            // Record each display to separate file
            for (idx, display) in self.displays.iter().enumerate() {
                let temp_file = format!("{}_display_{}.mp4", self.output_path, idx);

                // Use FFmpeg with gdigrab (Windows screen capture)
                // Reduced framerate to 5fps for minimal performance impact while maintaining AI analysis quality
                let child = Command::new("ffmpeg")
                    .args(&[
                        "-y",  // Overwrite output files
                        "-f", "gdigrab",
                        "-framerate", "5",  // Reduced from 30fps to 5fps for better performance
                        "-i", "desktop",
                        "-c:v", "libx264",
                        "-preset", "veryfast",  // Changed from ultrafast for better compression
                        "-crf", "28",  // Higher CRF for smaller file size (still acceptable quality)
                        "-pix_fmt", "yuv420p",
                        &temp_file,
                    ])
                    .stdin(std::process::Stdio::piped())  // Enable stdin for 'q' command
                    .spawn()
                    .map_err(|e| format!("Failed to start display recording: {}", e))?;

                self.display_processes.push(child);
                println!("Started recording display {} to {}", idx, temp_file);
            }

            // Record webcam if available
            if let Some(webcam) = &self.webcam {
                let temp_file = format!("{}_webcam.mp4", self.output_path);
                println!("Attempting to record webcam: {}", webcam.name);

                // Try with webcam name first, fallback to index if it fails
                let webcam_input = if webcam.name == "Default Webcam" {
                    "0".to_string()  // Use index for default
                } else {
                    format!("video={}", webcam.name)
                };

                let child = Command::new("ffmpeg")
                    .args(&[
                        "-y",  // Overwrite output files
                        "-f", "dshow",
                        "-video_size", "640x480",
                        "-framerate", "5",  // Reduced from 30fps to 5fps for better performance
                        "-i", &webcam_input,
                        "-c:v", "libx264",
                        "-preset", "veryfast",  // Changed from ultrafast for better compression
                        "-crf", "28",  // Higher CRF for smaller file size
                        "-pix_fmt", "yuv420p",
                        &temp_file,
                    ])
                    .stdin(std::process::Stdio::piped())  // Enable stdin for 'q' command
                    .stderr(std::process::Stdio::piped())  // Capture errors
                    .spawn();

                match child {
                    Ok(child) => {
                        self.webcam_process = Some(child);
                        println!("Started recording webcam to {}", temp_file);
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to start webcam recording: {}", e);
                        eprintln!("Continuing without webcam...");
                    }
                }
            } else {
                println!("No webcam selected for recording");
            }
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        println!("Stopping screen recording...");

        // Stop all display recordings by sending 'q' to stdin
        for child in self.display_processes.iter_mut() {
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                // Send 'q' command to FFmpeg to gracefully stop
                let _ = stdin.write_all(b"q");
                let _ = stdin.flush();
            }
            // Give FFmpeg a moment to process the command
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Stop webcam recording
        if let Some(ref mut child) = self.webcam_process {
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                let _ = stdin.write_all(b"q");
                let _ = stdin.flush();
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }

        // Wait for processes to finish gracefully (with timeout)
        for child in self.display_processes.iter_mut() {
            let _ = child.wait();
        }

        if let Some(ref mut child) = self.webcam_process {
            let _ = child.wait();
        }

        println!("Recording stopped gracefully");
        Ok(())
    }
}

impl Drop for ScreenRecorder {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}
