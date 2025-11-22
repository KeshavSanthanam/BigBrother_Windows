# BigBrother - Application Design Document

## Overview
BigBrother is a productivity accountability app built with Tauri (Rust + React) that records multi-display and webcam footage to create proof-of-work videos for self-assigned tasks.

## Technology Stack
- **Frontend**: React with TypeScript
- **Backend**: Rust (Tauri)
- **UI Framework**: TailwindCSS + shadcn/ui (recommended)
- **State Management**: Zustand or Context API
- **Database**: SQLite (via Tauri SQL plugin)
- **Screen Recording**: Windows Graphics Capture API (via Rust)
- **Video Processing**: FFmpeg
- **Authentication**: JWT-based auth with backend API

## Architecture

### Frontend Structure
```
src/
├── App.tsx                 # Main app component with routing
├── main.tsx               # Entry point
├── styles/
│   └── globals.css        # Global styles & Tailwind
├── components/
│   ├── layout/
│   │   ├── Header.tsx
│   │   ├── Sidebar.tsx
│   │   └── Layout.tsx
│   ├── tasks/
│   │   ├── TaskCard.tsx
│   │   ├── TaskList.tsx
│   │   ├── CreateTaskModal.tsx
│   │   └── TaskCountdown.tsx
│   ├── recording/
│   │   ├── RecordingControls.tsx
│   │   ├── RecordingIndicator.tsx
│   │   ├── DurationDisplay.tsx
│   │   └── DisplayPreview.tsx
│   └── auth/
│       ├── LoginForm.tsx
│       └── RegisterForm.tsx
├── pages/
│   ├── Dashboard.tsx      # Main page with pending tasks
│   ├── RecordingPage.tsx  # Active recording interface
│   ├── TaskHistory.tsx    # Completed tasks
│   ├── Settings.tsx       # User settings
│   └── Login.tsx          # Authentication
├── hooks/
│   ├── useTasks.ts
│   ├── useRecording.ts
│   └── useAuth.ts
├── lib/
│   ├── api.ts            # Tauri command wrappers
│   ├── types.ts          # TypeScript interfaces
│   └── utils.ts          # Utility functions
└── store/
    ├── taskStore.ts
    └── authStore.ts
```

### Backend Structure (Rust/Tauri)
```
src-tauri/
├── src/
│   ├── main.rs           # Tauri setup & command registration
│   ├── lib.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── tasks.rs      # Task CRUD operations
│   │   ├── recording.rs  # Recording management
│   │   └── auth.rs       # Authentication handlers
│   ├── recording/
│   │   ├── mod.rs
│   │   ├── capture.rs    # Screen & webcam capture
│   │   ├── combiner.rs   # Video combination logic
│   │   └── encoder.rs    # Video encoding
│   ├── database/
│   │   ├── mod.rs
│   │   ├── schema.rs     # Database schema
│   │   └── models.rs     # Data models
│   └── utils/
│       ├── mod.rs
│       └── time.rs       # Time/duration utilities
├── Cargo.toml
└── tauri.conf.json
```

## Database Schema

### Tasks Table
```sql
CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    due_date TEXT NOT NULL,        -- ISO 8601 format
    min_duration INTEGER NOT NULL, -- in seconds
    status TEXT NOT NULL,          -- 'pending', 'in_progress', 'completed', 'failed'
    video_path TEXT,               -- Path to completed video
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
```

### Users Table
```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email TEXT UNIQUE NOT NULL,
    username TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL
);
```

### Verifiers Table (for accountability partners)
```sql
CREATE TABLE verifiers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    verifier_user_id INTEGER NOT NULL,
    status TEXT NOT NULL,          -- 'pending', 'accepted', 'rejected'
    created_at TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (verifier_user_id) REFERENCES users(id)
);
```

### Task Verifications Table
```sql
CREATE TABLE task_verifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER NOT NULL,
    verifier_id INTEGER NOT NULL,
    verified BOOLEAN NOT NULL,
    comment TEXT,
    verified_at TEXT NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id),
    FOREIGN KEY (verifier_id) REFERENCES users(id)
);
```

### Recordings Table
```sql
CREATE TABLE recordings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    task_id INTEGER NOT NULL,
    duration INTEGER NOT NULL,     -- in seconds
    start_time TEXT NOT NULL,
    end_time TEXT,
    file_path TEXT NOT NULL,
    status TEXT NOT NULL,          -- 'recording', 'paused', 'completed', 'processing'
    FOREIGN KEY (task_id) REFERENCES tasks(id)
);
```

## Key Features & Implementation

### 1. Dashboard (Main Page)
**Components:**
- Task list with filtering (pending, today, overdue)
- "Create Task" button (opens modal)
- Task cards showing:
  - Title & description
  - Due date/time with live countdown
  - Minimum duration required
  - "Start Recording" button
  - Status indicators

**State Management:**
- Real-time countdown updates (1-second intervals)
- Task CRUD operations via Tauri commands
- Filtering and sorting logic

### 2. Create Task Modal
**Fields:**
- Title (required, text input)
- Description (optional, textarea)
- Due date & time (datetime picker)
- Minimum duration (number input in minutes)

**Validation:**
- Due date must be in the future
- Minimum duration > 0
- Title required

### 3. Recording Page
**Layout:**
- Large recording indicator (recording/paused/stopped)
- Live preview of displays being captured
- Webcam preview (picture-in-picture style)
- Control buttons:
  - Start Recording
  - Pause/Resume
  - Stop Recording
- Duration display:
  - Current recording time
  - Minimum required time
  - Progress bar (current/minimum)
  - Visual indicator when minimum is reached

**Recording Logic:**
- Capture all connected displays
- Capture webcam feed
- Combine into single video with layout:
  - Displays arranged in grid/horizontal layout
  - Webcam as overlay (bottom-right corner)
- Save temporary files during recording
- Final processing on stop

### 4. Screen Recording Implementation

**Windows-specific capture using:**
- `windows-rs` crate for Windows Graphics Capture API
- `webrtc` or `gstreamer` for webcam capture
- `ffmpeg-next` for video encoding and combining

**Recording Flow:**
1. Enumerate all displays and their resolutions
2. Start capture sessions for each display
3. Start webcam capture
4. Write frames to temporary buffers
5. On pause: stop writing frames but keep sessions open
6. On resume: continue writing frames
7. On stop: finalize all streams
8. Combine all streams into single video using FFmpeg
9. Save to user's designated folder
10. Update task with video path

**Video Layout Options:**
- Horizontal split for multiple displays
- Grid layout for 3+ displays
- Webcam overlay (configurable size & position)

### 5. Claude API Video Verification

**Purpose:**
Intelligent AI-powered verification that analyzes completed video recordings to ensure the user actually completed the task as described.

**Example Use Cases:**
- Task: "Look at calculus YouTube videos for 30 minutes"
- Video: 33-minute recording submitted
- Claude AI: Analyzes video frames, verifies >= 30 minutes of calculus content
- Result: Marks as complete/incomplete with detailed explanation

**Implementation:**

**Video Processing for Claude:**
1. Extract key frames from video (e.g., 1 frame per 5-10 seconds)
2. Send frames to Claude API with vision capabilities
3. Include task description and requirements
4. Claude analyzes frames and provides verification report

**Verification Flow:**
1. User stops recording
2. Video processing completes
3. System extracts frames at regular intervals
4. Frames + task description sent to Claude API
5. Claude returns:
   - `verified`: boolean (true/false)
   - `confidence`: number (0-100)
   - `explanation`: detailed analysis
   - `time_on_task`: estimated time actually spent on task
   - `issues`: array of problems found (if any)
6. Result stored in database
7. User sees verification status with AI explanation

**Claude API Prompt Structure:**
```
You are verifying a productivity task completion.

Task Details:
- Title: {task.title}
- Description: {task.description}
- Required Duration: {task.min_duration} minutes
- Video Duration: {actual_duration} minutes

Analyze the provided video frames (1 frame every 10 seconds) and determine:
1. Was the user engaged in the described task?
2. For what percentage of the video was the task being performed?
3. Did they meet the minimum duration requirement?
4. Were there significant distractions or off-task behavior?

Provide your response in JSON format:
{
  "verified": true/false,
  "confidence": 0-100,
  "time_on_task_minutes": number,
  "explanation": "detailed explanation",
  "issues": ["issue 1", "issue 2"],
  "timeline": [
    {"timestamp": "00:00", "activity": "description"},
    ...
  ]
}
```

**Database Schema Updates:**

Add to task_verifications table:
```sql
ALTER TABLE task_verifications ADD COLUMN ai_verification TEXT; -- JSON blob
ALTER TABLE task_verifications ADD COLUMN ai_confidence INTEGER;
ALTER TABLE task_verifications ADD COLUMN time_on_task INTEGER; -- seconds
```

**API Integration:**
- Use Anthropic Claude API (claude-3.5-sonnet or claude-3-opus)
- Store API key securely (environment variable or settings)
- Rate limiting and error handling
- Batch frame uploads for efficiency
- Cost tracking (tokens used per verification)

**User Experience:**
1. After recording stops: "Processing video..."
2. "Extracting frames for AI verification..."
3. "Analyzing with Claude AI..." (with progress indicator)
4. Show verification result:
   - ✅ Task Verified! (with confidence score)
   - ❌ Task Not Completed (with explanation)
   - Timeline breakdown of activity
   - Option to appeal/resubmit

### 6. Authentication System

**Phase 1 (Local):**
- Local user account stored in SQLite
- Bcrypt password hashing
- Session management via Tauri store

**Phase 2 (Future - Internet Connected):**
- REST API backend (Node.js/Rust)
- JWT tokens for authentication
- OAuth support (Google, GitHub)
- WebSocket for real-time notifications
- Cloud storage for videos

### 6. Verifier System (Future Feature)

**Flow:**
- User sends verification request to another user (by email/username)
- Verifier receives notification
- Verifier can view completed task videos
- Verifier approves/rejects with optional comments
- User sees verification status on task

## UI/UX Design Principles

### Color Scheme
- Primary: Blue (#3B82F6) - Trust and productivity
- Success: Green (#10B981) - Completed tasks
- Warning: Amber (#F59E0B) - Due soon
- Danger: Red (#EF4444) - Overdue/recording
- Neutral: Gray scale for backgrounds

### Recording Indicator
- Large, unmissable red dot when recording
- Pulsing animation
- Sound notification on start/stop (optional)

### Countdown Display
- Green: > 24 hours remaining
- Yellow: < 24 hours, > 1 hour
- Red: < 1 hour or overdue
- Smooth color transitions

### Recording Duration Display
- Progress bar filling as duration increases
- Green checkmark when minimum duration reached
- Prevent accidental stop before minimum (confirmation dialog)

## Tauri Commands API

### Task Commands
```rust
#[tauri::command]
async fn create_task(title: String, description: String, due_date: String, min_duration: u32) -> Result<Task, String>

#[tauri::command]
async fn get_all_tasks() -> Result<Vec<Task>, String>

#[tauri::command]
async fn get_task(id: i32) -> Result<Task, String>

#[tauri::command]
async fn update_task(id: i32, task: Task) -> Result<Task, String>

#[tauri::command]
async fn delete_task(id: i32) -> Result<(), String>
```

### Recording Commands
```rust
#[tauri::command]
async fn start_recording(task_id: i32) -> Result<String, String>

#[tauri::command]
async fn pause_recording() -> Result<(), String>

#[tauri::command]
async fn resume_recording() -> Result<(), String>

#[tauri::command]
async fn stop_recording() -> Result<String, String> // Returns video path

#[tauri::command]
async fn get_recording_status() -> Result<RecordingStatus, String>
```

### Verification Commands
```rust
#[tauri::command]
async fn verify_task_with_claude(task_id: i32) -> Result<VerificationResult, String>

#[tauri::command]
async fn get_verification_status(task_id: i32) -> Result<VerificationStatus, String>

#[tauri::command]
async fn extract_video_frames(video_path: String, interval_seconds: u32) -> Result<Vec<String>, String>

#[tauri::command]
async fn set_claude_api_key(api_key: String) -> Result<(), String>

#[tauri::command]
async fn get_verification_cost_estimate(video_duration: u32) -> Result<CostEstimate, String>
```

### Display Commands
```rust
#[tauri::command]
async fn get_displays() -> Result<Vec<Display>, String>

#[tauri::command]
async fn get_webcams() -> Result<Vec<Webcam>, String>
```

### Auth Commands
```rust
#[tauri::command]
async fn login(email: String, password: String) -> Result<User, String>

#[tauri::command]
async fn register(email: String, username: String, password: String) -> Result<User, String>

#[tauri::command]
async fn logout() -> Result<(), String>

#[tauri::command]
async fn get_current_user() -> Result<User, String>
```

## Data Flow Examples

### Creating a Task
1. User fills form in CreateTaskModal
2. Frontend validates input
3. Frontend calls `create_task()` Tauri command
4. Rust inserts into SQLite database
5. Returns Task object
6. Frontend updates UI with new task
7. Countdown timer starts automatically

### Recording Flow
1. User clicks "Start Recording" on task
2. Navigate to RecordingPage with task_id
3. Call `get_displays()` and `get_webcams()`
4. User selects sources (or use all by default)
5. Click "Start" button
6. Call `start_recording(task_id)`
7. Backend initializes capture sessions
8. Frontend polls `get_recording_status()` every second
9. Display current duration and update progress bar
10. User clicks "Stop"
11. Call `stop_recording()`
12. Backend processes and combines video
13. Shows processing indicator
14. Returns video path
15. Update task status to 'completed'
16. Navigate back to dashboard

## Future Enhancements

### Phase 1 (Current Scope)
- Local task management
- Multi-display + webcam recording
- Basic UI with countdown timers

### Phase 2 (Near Future)
- Cloud synchronization
- Account system with authentication
- Video upload to cloud storage
- Verifier system
- Mobile companion app (view only)

### Phase 3 (Advanced)
- AI-powered productivity insights
- Automatic task suggestions
- Integration with calendars (Google Calendar, Outlook)
- Team accountability features
- Streaks and gamification
- Video highlights/summary generation

## Security Considerations

1. **Local Storage**: Videos stored in user-designated folder with proper permissions
2. **Password Security**: Bcrypt hashing with proper salt rounds
3. **Video Privacy**: Clear indicators when recording is active
4. **Data Encryption**: Consider encrypting sensitive task data at rest
5. **API Security**: JWT with refresh tokens, rate limiting
6. **Input Validation**: Sanitize all user inputs

## Performance Considerations

1. **Video Recording**: Use hardware encoding when available (H.264/H.265)
2. **Database**: Index on user_id, due_date, status columns
3. **UI Updates**: Debounce countdown timers, use React.memo for task cards
4. **Video Storage**: Implement compression settings (quality vs file size)
5. **Memory**: Release capture sessions properly when stopped
6. **Background Processing**: Process video in separate thread to avoid UI blocking

## Development Phases

### Phase 1: MVP (Core Features)
1. Initialize Tauri + React project
2. Set up SQLite database
3. Implement task CRUD operations
4. Build dashboard UI with task list
5. Implement basic screen recording (single display)
6. Build recording UI with controls
7. Video file management

### Phase 2: Enhanced Recording
1. Multi-display capture
2. Webcam integration
3. Video combination logic
4. Pause/Resume functionality
5. Recording settings (quality, format)

### Phase 3: Polish & Auth
1. User authentication (local)
2. UI/UX improvements
3. Error handling and validation
4. Settings page
5. Testing and bug fixes

### Phase 4: Cloud Features
1. Backend API development
2. Cloud authentication
3. Video upload functionality
4. Verifier system
5. Notifications

## Configuration

### tauri.conf.json Settings
- Enable necessary permissions (filesystem, window control)
- Set up window properties (min size, resizable)
- Configure security policies
- Set up allowlist for Tauri commands

### Recommended Dependencies

**Frontend:**
- react-router-dom (routing)
- tailwindcss (styling)
- date-fns (date manipulation)
- react-hook-form (form handling)
- zod (validation)
- zustand (state management)

**Backend (Cargo.toml):**
- tauri
- serde (serialization)
- serde_json
- tokio (async runtime)
- rusqlite (SQLite)
- bcrypt (password hashing)
- chrono (date/time)
- windows-rs (Windows API)
- ffmpeg-next (video processing)
- reqwest (HTTP client for Claude API)
- base64 (encode frames for API)
- image (frame extraction and processing)

## Testing Strategy

1. **Unit Tests**: Test Rust commands independently
2. **Integration Tests**: Test database operations
3. **E2E Tests**: Test complete user flows
4. **Recording Tests**: Verify video output quality and correctness
5. **Performance Tests**: Ensure smooth recording without frame drops

## Documentation

1. User guide for basic operations
2. Developer documentation for code structure
3. API documentation for Tauri commands
4. Troubleshooting guide for common issues
