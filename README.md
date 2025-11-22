# BigBrother - Productivity Accountability App

> **Status**: 70% Complete - Core functionality implemented, screen recording needs implementation

A desktop productivity app that records your screen and webcam to create proof-of-work videos, then uses AI (Claude) to verify you actually completed your self-assigned tasks.

## ✨ What's Been Built

### ✅ Fully Implemented
- **Task Management System** - Create, view, edit, delete tasks with SQLite database
- **Live Dashboard** - Real-time countdown timers with color-coded urgency
- **Recording Interface** - Complete UI with start/pause/resume/stop controls
- **AI Verification** - Claude API integration to analyze video frames and verify task completion
- **Progress Tracking** - Duration display showing current time vs. minimum required
- **Modern UI** - React + TypeScript + TailwindCSS with responsive design
- **State Management** - Zustand stores for tasks and recordings
- **Database Schema** - Complete with tasks, recordings, verifications, and users

### 🚧 Not Yet Implemented
- **Actual Screen Recording** - Windows Graphics Capture API integration needed
- **Webcam Capture** - Video device capture needed
- **Video Encoding** - FFmpeg integration for combining streams needed
- **Frame Extraction** - Extract frames from video for AI analysis needed

## 📁 Project Structure

```
BigBrother_Windows/
├── bigbrother-app/          # Main Tauri application
│   ├── src/                 # React frontend (TypeScript)
│   ├── src-tauri/          # Rust backend
│   ├── DESIGN.md           # Complete architecture documentation
│   ├── PROJECT_STATUS.md   # Detailed development status
│   └── README.md           # User documentation
└── README.md               # This file
```

## 🚀 Quick Start

### Prerequisites
1. **Node.js** v20+ - [Download](https://nodejs.org/)
2. **Rust** - [Install from rustup.rs](https://rustup.rs/)
3. **Claude API Key** - [Get from Anthropic](https://console.anthropic.com/)

### Run the App

```bash
cd bigbrother-app
npm install
npm run tauri dev
```

## 🎯 How It Works

1. **Create a Task**: Set a title, description, due date, and minimum duration
   - Example: "Study calculus for 30 minutes by watching YouTube videos"

2. **Start Recording**: Click the record button when ready to work
   - Records all displays + webcam (once implemented)
   - Shows live duration and progress

3. **AI Verification**: When you stop, Claude analyzes your video
   - Extracts frames every 10 seconds
   - Checks if you actually did what you claimed
   - Gives confidence score, time-on-task, and detailed explanation

4. **Get Feedback**:
   - ✅ "Task Verified! You spent 31 minutes watching calculus videos"
   - ❌ "Task Not Completed: Only 12 minutes on calculus, 18 minutes on social media"

## 💡 Technology Stack

- **Frontend**: React 19, TypeScript, Vite, TailwindCSS, React Router, Zustand
- **Backend**: Rust, Tauri 2, SQLite (rusqlite), Reqwest
- **AI**: Claude 3.5 Sonnet (vision API)
- **Future**: Screen capture (Windows API), FFmpeg (video processing)

## 📊 Development Progress

**Backend (Rust)**: 90% Complete
- ✅ Database operations
- ✅ Task CRUD
- ✅ Recording state management
- ✅ Claude API integration
- ❌ Screen capture implementation

**Frontend (React)**: 95% Complete
- ✅ All pages and components
- ✅ State management
- ✅ API integration
- ✅ Styling and UX
- ⚠️ Needs settings page UI

**Features**: 70% Complete
- ✅ Task management
- ✅ UI/UX flow
- ✅ AI verification logic
- ❌ Actual video recording
- ❌ Frame extraction

## 🔍 Files Created

**32 new files** across Rust backend (17) and React frontend (15):

- Database schema with 5 tables
- 20 Tauri commands (tasks, recording, verification, settings)
- Complete React component library
- TypeScript types and API wrappers
- Zustand stores for state
- TailwindCSS styling

See [PROJECT_STATUS.md](bigbrother-app/PROJECT_STATUS.md) for complete file listing.

## 📖 Documentation

- **[DESIGN.md](bigbrother-app/DESIGN.md)** - Complete architecture, database schema, API specs
- **[README.md](bigbrother-app/README.md)** - User guide and setup instructions
- **[PROJECT_STATUS.md](bigbrother-app/PROJECT_STATUS.md)** - Development status and next steps

## 🎉 What Works Right Now

You can run the app and:
- ✅ Create tasks with all details
- ✅ See live countdown timers
- ✅ Navigate to recording page
- ✅ See recording UI and controls
- ✅ Database automatically created
- ⚠️ Recording returns placeholder (no actual video)
- ⚠️ AI verification ready but needs real frames

## 🔜 Next Steps

1. **Implement Windows Graphics Capture API** for screen recording
2. **Add webcam capture** using video device APIs
3. **Integrate FFmpeg** for video encoding and combining
4. **Implement frame extraction** from saved videos
5. **Create Settings UI** for Claude API key input

## 🏗️ Architecture Highlights

- Clean separation: Rust backend ↔ Tauri IPC ↔ React frontend
- Type-safe API layer with TypeScript
- SQLite for local storage
- Zustand for reactive state
- Claude API for intelligent verification
- Designed for future cloud sync and multi-user features

## 📝 Future Features (Planned)

- **Phase 2**: Cloud sync, user accounts, human verifiers
- **Phase 3**: Analytics, streaks, calendar integration
- **Phase 4**: AI insights, mobile app, team features

---

**Note**: This app is designed as a personal accountability tool. The AI verification helps you stay honest with yourself about your productivity! 