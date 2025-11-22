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
          <h3 className="text-xl font-semibold text-gray-900 mb-1">{task.title}</h3>
          {task.description && (
            <p className="text-gray-600 text-sm line-clamp-2">{task.description}</p>
          )}
        </div>
        <span className={`px-2 py-1 rounded text-xs font-medium capitalize ${
          task.status === 'pending' ? 'bg-yellow-100 text-yellow-800' :
          task.status === 'in_progress' ? 'bg-blue-100 text-blue-800' :
          task.status === 'completed' ? 'bg-green-100 text-green-800' :
          'bg-red-100 text-red-800'
        }`}>
          {task.status.replace('_', ' ')}
        </span>
      </div>

      <div className="space-y-2 mb-4">
        <div className="flex items-center justify-between text-sm">
          <span className="text-gray-500">Due:</span>
          <span className={`font-medium ${urgencyColor}`}>
            {overdue ? '⚠️ Overdue' : countdown}
          </span>
        </div>
        <div className="flex items-center justify-between text-sm">
          <span className="text-gray-500">Min Duration:</span>
          <span className="font-medium">{formatDuration(task.min_duration)}</span>
        </div>
      </div>

      <button
        onClick={() => task.id && onStartRecording(task.id)}
        className="btn btn-primary w-full"
        disabled={task.status !== 'pending'}
      >
        {task.status === 'pending' ? '🎥 Start Recording' : 'Completed'}
      </button>
    </div>
  );
}
