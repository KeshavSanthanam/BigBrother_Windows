use std::process::Command;
use std::path::Path;

pub struct VideoCombiner {
    pub input_files: Vec<String>,
    pub output_file: String,
}

impl VideoCombiner {
    pub fn new(input_files: Vec<String>, output_file: String) -> Self {
        Self {
            input_files,
            output_file,
        }
    }

    /// Combine videos in a grid layout based on number of inputs
    /// - 1 video: full screen
    /// - 2 videos: side by side (1x2)
    /// - 3 videos: 2 top, 1 bottom centered (2x2 with empty cell)
    /// - 4+ videos: grid (2x2, 2x3, 3x3, etc.)
    pub fn combine_grid(&self) -> Result<(), String> {
        let num_videos = self.input_files.len();
        if num_videos == 0 {
            return Err("No input files provided".to_string());
        }

        println!("Combining {} videos into grid layout", num_videos);

        // Calculate grid dimensions
        let (rows, cols) = self.calculate_grid_dimensions(num_videos);

        // Build FFmpeg filter complex for grid layout
        let filter_complex = self.build_grid_filter(rows, cols);

        // Build FFmpeg command
        let mut cmd = Command::new("ffmpeg");

        // Add input files
        for file in &self.input_files {
            cmd.arg("-i").arg(file);
        }

        // Add filter complex
        cmd.arg("-filter_complex").arg(&filter_complex);

        // Add output options
        cmd.arg("-c:v")
            .arg("libx264")
            .arg("-preset")
            .arg("medium")
            .arg("-crf")
            .arg("23")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg(&self.output_file);

        println!("Running FFmpeg command...");
        println!("Grid: {}x{}", rows, cols);

        let output = cmd.output()
            .map_err(|e| format!("Failed to run FFmpeg: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("FFmpeg failed: {}", stderr));
        }

        println!("Videos combined successfully!");
        Ok(())
    }

    fn calculate_grid_dimensions(&self, num_videos: usize) -> (usize, usize) {
        match num_videos {
            1 => (1, 1),
            2 => (1, 2),
            3 => (2, 2), // 2x2 grid with one empty slot
            4 => (2, 2),
            5 | 6 => (2, 3),
            7 | 8 | 9 => (3, 3),
            10 | 11 | 12 => (3, 4),
            _ => {
                let cols = (num_videos as f64).sqrt().ceil() as usize;
                let rows = ((num_videos as f64) / (cols as f64)).ceil() as usize;
                (rows, cols)
            }
        }
    }

    fn build_grid_filter(&self, rows: usize, cols: usize) -> String {
        let num_videos = self.input_files.len();

        if num_videos == 1 {
            // Single video, no grid needed
            return format!("[0:v]scale=1920:1080[v]");
        }

        let cell_width = 1920 / cols;
        let cell_height = 1080 / rows;

        let mut filter = String::new();

        // Scale each input video to cell size
        for i in 0..num_videos {
            filter.push_str(&format!(
                "[{}:v]scale={}:{}:force_original_aspect_ratio=decrease,pad={}:{}:(ow-iw)/2:(oh-ih)/2,setsar=1[v{}];",
                i, cell_width, cell_height, cell_width, cell_height, i
            ));
        }

        // Create grid layout using xstack
        // Build xstack inputs
        let mut xstack_inputs = String::new();
        for i in 0..num_videos {
            xstack_inputs.push_str(&format!("[v{}]", i));
        }

        // Build grid layout string
        let mut layout = String::new();
        for row in 0..rows {
            for col in 0..cols {
                let idx = row * cols + col;
                if idx < num_videos {
                    let x = col * cell_width;
                    let y = row * cell_height;
                    if !layout.is_empty() {
                        layout.push('|');
                    }
                    layout.push_str(&format!("{}_{}", x, y));
                }
            }
        }

        // Add xstack filter
        filter.push_str(&format!(
            "{}xstack=inputs={}:layout={}[v]",
            xstack_inputs, num_videos, layout
        ));

        filter
    }

    /// Combine videos in a simple horizontal layout
    pub fn combine_horizontal(&self) -> Result<(), String> {
        let num_videos = self.input_files.len();
        if num_videos == 0 {
            return Err("No input files provided".to_string());
        }

        println!("Combining {} videos horizontally", num_videos);

        let cell_width = 1920 / num_videos;
        let cell_height = 1080;

        let mut filter = String::new();

        // Scale each input
        for i in 0..num_videos {
            filter.push_str(&format!(
                "[{}:v]scale={}:{}:force_original_aspect_ratio=decrease,pad={}:{}:(ow-iw)/2:(oh-ih)/2[v{}];",
                i, cell_width, cell_height, cell_width, cell_height, i
            ));
        }

        // Horizontal stack
        let mut hstack_inputs = String::new();
        for i in 0..num_videos {
            hstack_inputs.push_str(&format!("[v{}]", i));
        }

        filter.push_str(&format!("{}hstack=inputs={}[v]", hstack_inputs, num_videos));

        // Run FFmpeg
        let mut cmd = Command::new("ffmpeg");

        for file in &self.input_files {
            cmd.arg("-i").arg(file);
        }

        cmd.arg("-filter_complex")
            .arg(&filter)
            .arg("-c:v")
            .arg("libx264")
            .arg("-preset")
            .arg("medium")
            .arg("-crf")
            .arg("23")
            .arg("-pix_fmt")
            .arg("yuv420p")
            .arg(&self.output_file);

        let output = cmd.output()
            .map_err(|e| format!("Failed to run FFmpeg: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("FFmpeg failed: {}", stderr));
        }

        println!("Videos combined successfully!");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_dimensions() {
        let combiner = VideoCombiner::new(vec![], String::new());

        assert_eq!(combiner.calculate_grid_dimensions(1), (1, 1));
        assert_eq!(combiner.calculate_grid_dimensions(2), (1, 2));
        assert_eq!(combiner.calculate_grid_dimensions(3), (2, 2));
        assert_eq!(combiner.calculate_grid_dimensions(4), (2, 2));
        assert_eq!(combiner.calculate_grid_dimensions(5), (2, 3));
        assert_eq!(combiner.calculate_grid_dimensions(9), (3, 3));
    }
}
