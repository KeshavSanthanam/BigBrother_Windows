import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import { useTaskStore } from '../store/taskStore';
import type { Task } from '../lib/types';
import { formatDateTime, formatDuration } from '../lib/utils';
import { verificationApi } from '../lib/api';
import ThemeToggle from '../components/ThemeToggle';

export default function TaskHistory() {
  const navigate = useNavigate();
  const { tasks, fetchTasks } = useTaskStore();
  const [selectedTask, setSelectedTask] = useState<Task | null>(null);
  const [verification, setVerification] = useState<any>(null);

  useEffect(() => {
    fetchTasks();
  }, [fetchTasks]);

  const completedTasks = tasks
    .filter(t => t.status === 'completed' || t.status === 'failed')
    .sort((a, b) => {
      const dateA = new Date(a.updated_at || a.created_at || 0).getTime();
      const dateB = new Date(b.updated_at || b.created_at || 0).getTime();
      return dateB - dateA; // Most recent first
    });

  const handleViewDetails = async (task: Task) => {
    setSelectedTask(task);
    if (task.id) {
      const verif = await verificationApi.getStatus(task.id);
      setVerification(verif);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex justify-between items-center mb-8">
          <div>
            <button
              onClick={() => navigate('/')}
              className="text-primary dark:text-purple hover:underline mb-4"
            >
              ← Back to Dashboard
            </button>
            <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Task History</h1>
            <p className="text-gray-600 dark:text-gray-300 mt-1">View your completed and failed tasks</p>
          </div>
          <ThemeToggle />
        </div>

        {completedTasks.length === 0 ? (
          <div className="text-center py-12">
            <div className="text-gray-400 text-lg mb-4">No completed tasks yet</div>
            <button
              onClick={() => navigate('/')}
              className="btn btn-primary"
            >
              Create your first task
            </button>
          </div>
        ) : (
          <div className="space-y-4">
            {completedTasks.map((task) => (
              <div
                key={task.id}
                className="card hover:shadow-lg transition-shadow cursor-pointer"
                onClick={() => handleViewDetails(task)}
              >
                <div className="flex justify-between items-start">
                  <div className="flex-1">
                    <div className="flex items-center gap-3 mb-2">
                      <h3 className="text-xl font-semibold text-gray-900 dark:text-white">{task.title}</h3>
                      <span className={`px-3 py-1 rounded-full text-sm font-medium ${task.status === 'completed'
                        ? 'bg-green-100 dark:bg-green-900 text-green-800 dark:text-green-200'
                        : 'bg-red-100 dark:bg-red-900 text-red-800 dark:text-red-200'
                        }`}>
                        {task.status === 'completed' ? '✓ Verified' : '✗ Failed'}
                      </span>
                    </div>
                    {task.description && (
                      <p className="text-gray-600 dark:text-gray-400 text-sm mb-3">{task.description}</p>
                    )}
                    <div className="flex gap-6 text-sm text-gray-500 dark:text-gray-400">
                      <span>Completed: {formatDateTime(task.updated_at || '')}</span>
                      <span>Required: {formatDuration(task.min_duration)}</span>
                      {task.video_path && (
                        <span className="text-primary dark:text-purple">📹 Video available</span>
                      )}
                    </div>
                  </div>
                  <button
                    onClick={(e) => {
                      e.stopPropagation();
                      handleViewDetails(task);
                    }}
                    className="btn btn-secondary"
                  >
                    View Details
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}

        {/* Details Modal */}
        {selectedTask && (
          <div
            className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
            onClick={() => {
              setSelectedTask(null);
              setVerification(null);
            }}
          >
            <div
              className="bg-white dark:bg-gray-800 rounded-lg p-6 max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto"
              onClick={(e) => e.stopPropagation()}
            >
              <div className="flex justify-between items-start mb-4">
                <h2 className="text-2xl font-bold dark:text-white">{selectedTask.title}</h2>
                <button
                  onClick={() => {
                    setSelectedTask(null);
                    setVerification(null);
                  }}
                  className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-300 text-2xl"
                >
                  ×
                </button>
              </div>

              {selectedTask.description && (
                <p className="text-gray-600 dark:text-gray-400 mb-4">{selectedTask.description}</p>
              )}

              <div className="space-y-3 mb-6">
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500 dark:text-gray-400">Status:</span>
                  <span className={`font-medium ${selectedTask.status === 'completed' ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'
                    }`}>
                    {selectedTask.status === 'completed' ? 'Verified Complete' : 'Failed Verification'}
                  </span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500 dark:text-gray-400">Due Date:</span>
                  <span className="dark:text-gray-300">{formatDateTime(selectedTask.due_date)}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500 dark:text-gray-400">Completed:</span>
                  <span className="dark:text-gray-300">{formatDateTime(selectedTask.updated_at || '')}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500 dark:text-gray-400">Required Duration:</span>
                  <span className="dark:text-gray-300">{formatDuration(selectedTask.min_duration)}</span>
                </div>
                {selectedTask.video_path && (
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-500 dark:text-gray-400">Video:  </span>
                    <button
                      onClick={async () => {
                        try {
                          await invoke('open_video_file', { path: selectedTask.video_path });
                        } catch (error) {
                          console.error('Failed to open video', error);
                          alert('Unable to open video recording.');
                        }
                      }}
                      className="text-primary dark:text-purple hover:underline cursor-pointer text-left break-all"
                    >
                      {selectedTask.video_path}
                    </button>
                  </div>
                )}
              </div>

              {verification && verification.ai_verification && (
                <div className="border-t pt-4">
                  <h3 className="text-lg font-semibold mb-3">AI Verification Report</h3>

                  {(() => {
                    try {
                      const report = JSON.parse(verification.ai_verification);
                      return (
                        <div className="space-y-3">
                          <div className="flex justify-between">
                            <span className="text-gray-600">Confidence:</span>
                            <span className="font-medium">{report.confidence}%</span>
                          </div>
                          <div className="flex justify-between">
                            <span className="text-gray-600">Time on Task:</span>
                            <span className="font-medium">{report.time_on_task_minutes.toFixed(1)} minutes</span>
                          </div>
                          <div>
                            <span className="text-gray-600 block mb-2">Explanation:</span>
                            <p className="text-sm bg-gray-50 p-3 rounded">{report.explanation}</p>
                          </div>
                          {report.issues && report.issues.length > 0 && (
                            <div>
                              <span className="text-gray-600 block mb-2">Issues Found:</span>
                              <ul className="list-disc list-inside text-sm bg-red-50 p-3 rounded">
                                {report.issues.map((issue: string, idx: number) => (
                                  <li key={idx}>{issue}</li>
                                ))}
                              </ul>
                            </div>
                          )}
                          {report.timeline && report.timeline.length > 0 && (
                            <div>
                              <span className="text-gray-600 block mb-2">Activity Timeline:</span>
                              <div className="space-y-2 text-sm bg-gray-50 p-3 rounded max-h-48 overflow-y-auto">
                                {report.timeline.map((entry: any, idx: number) => (
                                  <div key={idx} className="flex gap-3">
                                    <span className="font-mono text-gray-500">{entry.timestamp}</span>
                                    <span>{entry.activity}</span>
                                  </div>
                                ))}
                              </div>
                            </div>
                          )}
                        </div>
                      );
                    } catch (e) {
                      return <p className="text-gray-500">Unable to parse verification report</p>;
                    }
                  })()}
                </div>
              )}

              <div className="mt-6 flex justify-end">
                <button
                  onClick={() => {
                    setSelectedTask(null);
                    setVerification(null);
                  }}
                  className="btn btn-primary"
                >
                  Close
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
