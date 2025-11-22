# BigBrother - Productivity Accountability App

A desktop application built with Tauri (Rust + React + TypeScript) that records multi-display and webcam footage to create proof-of-work videos for self-assigned tasks. Features AI-powered verification using Claude API to analyze completed recordings.

## Features

- **Task Management**: Create tasks with titles, descriptions, due dates, and minimum durations
- **Live Countdowns**: Real-time countdown timers showing urgency with color coding  
- **Screen Recording**: Capture all displays and webcam simultaneously (placeholder - to be implemented)
- **AI Verification**: Claude API analyzes video frames to verify task completion
- **Smart Analysis**: AI checks if you actually completed the task as described
- **Detailed Reports**: Get confidence scores, time-on-task estimates, and explanations

## Prerequisites

Before you begin, ensure you have the following installed:

1. **Node.js** (v20 or higher) - Download from [nodejs.org](https://nodejs.org/)

2. **Rust** (latest stable) - Install from [rustup.rs](https://rustup.rs/)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. **Claude API Key** - Get from [Anthropic Console](https://console.anthropic.com/)

## Installation

```bash
# Install frontend dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## Quick Start

1. Run the app with `npm run tauri dev`
2. Click "Create Task" and fill in your task details
3. Click "Start Recording" on a task
4. Work on your task, then click "Stop"
5. AI will automatically verify your recording

## Project Structure

See [DESIGN.md](../DESIGN.md) for complete architecture documentation.

## Database Location

- Windows: `%APPDATA%\com.bigbrother.app\bigbrother.db`
- macOS: `~/Library/Application Support/com.bigbrother.app/bigbrother.db`  
- Linux: `~/.local/share/com.bigbrother.app/bigbrother.db`

## To-Do

- [ ] Implement actual screen recording (Windows Graphics Capture API)
- [ ] Implement webcam capture
- [ ] Implement video combination with FFmpeg
- [ ] Add Settings UI for Claude API key
- [ ] Implement frame extraction from videos

## License

MIT
