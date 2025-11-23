import { useState, useEffect } from 'react';
import type { Task } from '../../lib/types';
import { formatCountdown, isOverdue, getUrgencyColor, formatDuration } from '../../lib/utils';

interface TaskCardProps {
  task: Task;
  onStartRecording: (taskId: number) => void;
}

export default function TaskCard({ task, onStartRecording }: TaskCardProps) {
  const [countdown, setCountdown] = useState(formatCountdown(task.due_date));

  useEffect(() => {
    const interval = setInterval(() => {
      setCountdown(formatCountdown(task.due_date));
    }, 1000);

    return () => clearInterval(interval);
  }, [task.due_date]);

  const urgencyColor = getUrgencyColor(task.due_date);
  const overdue = isOverdue(task.due_date);

  return (
    <div className="card hover:shadow-lg transition-shadow">
      <div className="flex justify-between items-start mb-4">
        <div className="flex-1">
          <h3 className="text-xl font-semibold text-gray-900 dark:text-white mb-1">{task.title}</h3>
          {task.description && (
            <p className="text-gray-600 dark:text-gray-400 text-sm line-clamp-2">{task.description}</p>
          )}
        </div>
        <span className={`px-2 py-1 rounded text-xs font-medium capitalize ${
          task.status === 'pending' ? 'bg-yellow-100 dark:bg-yellow-900 text-yellow-800 dark:text-yellow-200' :
          task.status === 'in_progress' ? 'bg-blue-100 dark:bg-blue-900 text-blue-800 dark:text-blue-200' :
          task.status === 'completed' ? 'bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200' :
          'bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200'
        }`}>
          {task.status.replace('_', ' ')}
        </span>
      </div>

      <div className="space-y-2 mb-4">
        <div className="flex items-center justify-between text-sm">
          <span className="text-gray-500 dark:text-gray-400">Due:</span>
          <span className={`font-medium ${urgencyColor} dark:text-orange-light`}>
            {overdue ? '⚠️ Overdue' : countdown}
          </span>
        </div>
        <div className="flex items-center justify-between text-sm">
          <span className="text-gray-500 dark:text-gray-400">Min Duration:</span>
          <span className="font-medium dark:text-gray-300">{formatDuration(task.min_duration)}</span>
        </div>
      </div>

      <button
        onClick={() => task.id && onStartRecording(task.id)}
        className="btn btn-primary w-full bg-purple hover:bg-purple-dark"
        disabled={task.status !== 'pending'}
      >
        {task.status === 'pending' ? '🎥 Start Recording' : 'Completed'}
      </button>
    </div>
  );
}
