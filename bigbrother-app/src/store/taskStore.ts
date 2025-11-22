import { create } from 'zustand';
import type { Task } from '../lib/types';
import { taskApi } from '../lib/api';

interface TaskStore {
  tasks: Task[];
  loading: boolean;
  error: string | null;
  fetchTasks: () => Promise<void>;
  fetchPendingTasks: () => Promise<void>;
  createTask: (title: string, description: string | null, dueDate: string, minDuration: number) => Promise<Task>;
  updateTask: (id: number, task: Task) => Promise<void>;
  deleteTask: (id: number) => Promise<void>;
}

export const useTaskStore = create<TaskStore>((set, get) => ({
  tasks: [],
  loading: false,
  error: null,

  fetchTasks: async () => {
    set({ loading: true, error: null });
    try {
      const tasks = await taskApi.getAll();
      set({ tasks, loading: false });
    } catch (error) {
      set({ error: (error as Error).message, loading: false });
    }
  },

  fetchPendingTasks: async () => {
    set({ loading: true, error: null });
    try {
      const tasks = await taskApi.getPending();
      set({ tasks, loading: false });
    } catch (error) {
      set({ error: (error as Error).message, loading: false });
    }
  },

  createTask: async (title, description, dueDate, minDuration) => {
    set({ loading: true, error: null });
    try {
      const task = await taskApi.create(title, description, dueDate, minDuration);
      set(state => ({ tasks: [...state.tasks, task], loading: false }));
      return task;
    } catch (error) {
      set({ error: (error as Error).message, loading: false });
      throw error;
    }
  },

  updateTask: async (id, task) => {
    set({ loading: true, error: null });
    try {
      const updatedTask = await taskApi.update(id, task);
      set(state => ({
        tasks: state.tasks.map(t => t.id === id ? updatedTask : t),
        loading: false
      }));
    } catch (error) {
      set({ error: (error as Error).message, loading: false });
      throw error;
    }
  },

  deleteTask: async (id) => {
    set({ loading: true, error: null });
    try {
      await taskApi.delete(id);
      set(state => ({
        tasks: state.tasks.filter(t => t.id !== id),
        loading: false
      }));
    } catch (error) {
      set({ error: (error as Error).message, loading: false });
      throw error;
    }
  },
}));
