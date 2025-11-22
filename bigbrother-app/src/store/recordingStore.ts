import { create } from 'zustand';
import type { RecordingStatus } from '../lib/types';
import { recordingApi } from '../lib/api';

interface RecordingStore {
  status: RecordingStatus;
  error: string | null;
  startRecording: (taskId: number) => Promise<void>;
  pauseRecording: () => Promise<void>;
  resumeRecording: () => Promise<void>;
  stopRecording: () => Promise<string>;
  updateStatus: () => Promise<void>;
  updateDuration: (duration: number) => Promise<void>;
}

export const useRecordingStore = create<RecordingStore>((set, get) => ({
  status: {
    is_recording: false,
    is_paused: false,
    duration: 0,
    task_id: undefined,
  },
  error: null,

  startRecording: async (taskId: number) => {
    try {
      await recordingApi.start(taskId);
      await get().updateStatus();
    } catch (error) {
      set({ error: (error as Error).message });
      throw error;
    }
  },

  pauseRecording: async () => {
    try {
      await recordingApi.pause();
      await get().updateStatus();
    } catch (error) {
      set({ error: (error as Error).message });
      throw error;
    }
  },

  resumeRecording: async () => {
    try {
      await recordingApi.resume();
      await get().updateStatus();
    } catch (error) {
      set({ error: (error as Error).message });
      throw error;
    }
  },

  stopRecording: async () => {
    try {
      const videoPath = await recordingApi.stop();
      await get().updateStatus();
      return videoPath;
    } catch (error) {
      set({ error: (error as Error).message });
      throw error;
    }
  },

  updateStatus: async () => {
    try {
      const status = await recordingApi.getStatus();
      set({ status, error: null });
    } catch (error) {
      set({ error: (error as Error).message });
    }
  },

  updateDuration: async (duration: number) => {
    try {
      await recordingApi.updateDuration(duration);
      set(state => ({
        status: { ...state.status, duration }
      }));
    } catch (error) {
      set({ error: (error as Error).message });
    }
  },
}));
