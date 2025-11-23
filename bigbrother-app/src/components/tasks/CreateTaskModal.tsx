import { useState } from 'react';
import { useTaskStore } from '../../store/taskStore';

interface CreateTaskModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export default function CreateTaskModal({ isOpen, onClose }: CreateTaskModalProps) {
  const { createTask } = useTaskStore();
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [dueDate, setDueDate] = useState('');
  const [dueTime, setDueTime] = useState('');
  const [minDuration, setMinDuration] = useState('30');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!title || !dueDate || !dueTime) {
      alert('Please fill in all required fields');
      return;
    }

    // Validate due date/time is not in the past
    const dueDateTimeStr = `${dueDate}T${dueTime}:00`;
    const dueDateTime = new Date(dueDateTimeStr);
    const now = new Date();

    if (dueDateTime <= now) {
      alert('Due date and time must be in the future');
      return;
    }

    // Validate duration is not over 4 hours (240 minutes)
    const durationMinutes = parseInt(minDuration);
    if (durationMinutes > 240) {
      alert('Recording duration cannot exceed 4 hours (240 minutes)');
      return;
    }

    const minDurationSeconds = durationMinutes * 60;

    try {
      await createTask(title, description || null, dueDateTimeStr, minDurationSeconds);
      setTitle('');
      setDescription('');
      setDueDate('');
      setDueTime('');
      setMinDuration('30');
      onClose();
    } catch (error) {
      alert(`Failed to create task: ${error}`);
    }
  };

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
      <div className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-md w-full mx-4">
        <h2 className="text-2xl font-bold dark:text-white mb-4">Create New Task</h2>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="label">Title *</label>
            <input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              className="input"
              placeholder="e.g., Study calculus"
              required
            />
          </div>

          <div>
            <label className="label">Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              className="input"
              rows={3}
              placeholder="e.g., Watch calculus YouTube videos and take notes"
            />
          </div>

          <div className="grid grid-cols-2 gap-4">
            <div>
              <label className="label">Due Date *</label>
              <input
                type="date"
                value={dueDate}
                onChange={(e) => setDueDate(e.target.value)}
                className="input"
                required
              />
            </div>
            <div>
              <label className="label">Due Time *</label>
              <input
                type="time"
                value={dueTime}
                onChange={(e) => setDueTime(e.target.value)}
                className="input"
                required
              />
            </div>
          </div>

          <div>
            <label className="label">Minimum Duration (minutes) *</label>
            <input
              type="number"
              value={minDuration}
              onChange={(e) => setMinDuration(e.target.value)}
              className="input"
              min="1"
              max="240"
              placeholder="30"
              required
            />
            <p className="text-xs text-gray-500 mt-1">Maximum 4 hours (240 minutes)</p>
          </div>

          <div className="flex gap-3 mt-6">
            <button type="button" onClick={onClose} className="btn btn-secondary flex-1">
              Cancel
            </button>
            <button type="submit" className="btn btn-primary flex-1">
              Create Task
            </button>
          </div>
        </form>
      </div>
    </div>
  );
}
