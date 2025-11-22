import { invoke } from '@tauri-apps/api/core';
import type { Task, RecordingStatus, VerificationResult, Verification, CostEstimate } from './types';

// Task APIs
export const taskApi = {
  create: (title: string, description: string | null, due_date: string, min_duration: number): Promise<Task> =>
    invoke('create_task', { title, description, dueDate: due_date, minDuration: min_duration }),

  getAll: (): Promise<Task[]> =>
    invoke('get_all_tasks'),

  getPending: (): Promise<Task[]> =>
    invoke('get_pending_tasks'),

  getCompleted: (): Promise<Task[]> =>
    invoke('get_completed_tasks'),

  get: (id: number): Promise<Task> =>
    invoke('get_task', { id }),

  update: (id: number, task: Task): Promise<Task> =>
    invoke('update_task', { id, task }),

  delete: (id: number): Promise<void> =>
    invoke('delete_task', { id }),
};

// Recording APIs
export const recordingApi = {
  start: (taskId: number): Promise<string> =>
    invoke('start_recording', { taskId }),

  pause: (): Promise<void> =>
    invoke('pause_recording'),

  resume: (): Promise<void> =>
    invoke('resume_recording'),

  stop: (): Promise<string> =>
    invoke('stop_recording'),

  getStatus: (): Promise<RecordingStatus> =>
    invoke('get_recording_status'),

  updateDuration: (duration: number): Promise<void> =>
    invoke('update_recording_duration', { duration }),

  enumerateDisplays: (): Promise<any[]> =>
    invoke('enumerate_displays'),

  enumerateWebcams: (): Promise<any[]> =>
    invoke('enumerate_webcams'),
};

// Verification APIs
export const verificationApi = {
  verify: (taskId: number): Promise<VerificationResult> =>
    invoke('verify_task_with_claude', { taskId }),

  getStatus: (taskId: number): Promise<Verification | null> =>
    invoke('get_verification_status', { taskId }),

  getCostEstimate: (videoDuration: number): Promise<CostEstimate> =>
    invoke('get_verification_cost_estimate', { videoDuration }),

  extractFrames: (videoPath: string, intervalSeconds: number): Promise<string[]> =>
    invoke('extract_video_frames', { videoPath, intervalSeconds }),
};

// Settings APIs
export const settingsApi = {
  setClaudeApiKey: (apiKey: string): Promise<void> =>
    invoke('set_claude_api_key', { apiKey }),

  getClaudeApiKey: (): Promise<string | null> =>
    invoke('get_claude_api_key'),
};
