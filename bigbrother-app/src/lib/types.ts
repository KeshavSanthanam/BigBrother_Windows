export interface Task {
  id?: number;
  user_id: number;
  title: string;
  description?: string;
  due_date: string; // ISO 8601 format
  min_duration: number; // in seconds
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
  video_path?: string;
  created_at?: string;
  updated_at?: string;
}

export interface Recording {
  id?: number;
  task_id: number;
  duration: number; // in seconds
  start_time: string;
  end_time?: string;
  file_path: string;
  status: 'recording' | 'paused' | 'completed' | 'processing';
}

export interface RecordingStatus {
  is_recording: boolean;
  is_paused: boolean;
  duration: number;
  task_id?: number;
}

export interface Verification {
  id?: number;
  task_id: number;
  verified: boolean;
  ai_verification?: string; // JSON blob
  ai_confidence?: number;
  time_on_task?: number; // in seconds
  explanation?: string;
  verified_at?: string;
}

export interface VerificationResult {
  verified: boolean;
  confidence: number;
  time_on_task_minutes: number;
  explanation: string;
  issues: string[];
  timeline: TimelineEntry[];
}

export interface TimelineEntry {
  timestamp: string;
  activity: string;
}

export interface CostEstimate {
  estimated_tokens: number;
  estimated_cost_usd: number;
}
