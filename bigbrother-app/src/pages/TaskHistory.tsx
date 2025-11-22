import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTaskStore } from '../store/taskStore';
import type { Task } from '../lib/types';
import { formatDateTime, formatDuration } from '../lib/utils';
import { verificationApi } from '../lib/api';

export default function TaskHistory() {
  const navigate = useNavigate();
  const { tasks, fetchTasks } = useTaskStore();
  const [selectedTask, setSelectedTask] = useState<Task | null>(null);
  const [verification, setVerification] = useState<any>(null);

  useEffect(() => {
    fetchTasks();
  }, [fetchTasks]);

  const completedTasks = tasks.filter(t => t.status === 'completed' || t.status === 'failed');

  const handleViewDetails = async (task: Task) => {
    setSelectedTask(task);
    if (task.id) {
      const verif = await verificationApi.getStatus(task.id);
      setVerification(verif);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex justify-between items-center mb-8">
          <div>
            <button
              onClick={() => navigate('/')}
              className="text-primary hover:underline mb-4"
            >
              ← Back to Dashboard
            </button>
            <h1 className="text-3xl font-bold text-gray-900">Task History</h1>
            <p className="text-gray-600 mt-1">View your completed and failed tasks</p>
          </div>
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
                      <h3 className="text-xl font-semibold text-gray-900">{task.title}</h3>
                      <span className={`px-3 py-1 rounded-full text-sm font-medium ${task.status === 'completed'
                          ? 'bg-green-100 text-green-800'
                          : 'bg-red-100 text-red-800'
                        }`}>
                        {task.status === 'completed' ? '✓ Verified' : '✗ Failed'}
                      </span>
                    </div>
                    {task.description && (
                      <p className="text-gray-600 text-sm mb-3">{task.description}</p>
                    )}
                    <div className="flex gap-6 text-sm text-gray-500">
                      <span>Completed: {formatDateTime(task.updated_at || '')}</span>
                      <span>Required: {formatDuration(task.min_duration)}</span>
                      {task.video_path && (
                        <span className="text-primary">📹 Video available</span>
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
              className="bg-white rounded-lg p-6 max-w-2xl w-full mx-4 max-h-[80vh] overflow-y-auto"
              onClick={(e) => e.stopPropagation()}
            >
              <div className="flex justify-between items-start mb-4">
                <h2 className="text-2xl font-bold">{selectedTask.title}</h2>
                <button
                  onClick={() => {
                    setSelectedTask(null);
                    setVerification(null);
                  }}
                  className="text-gray-400 hover:text-gray-600 text-2xl"
                >
                  ×
                </button>
              </div>

              {selectedTask.description && (
                <p className="text-gray-600 mb-4">{selectedTask.description}</p>
              )}

              <div className="space-y-3 mb-6">
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500">Status:</span>
                  <span className={`font-medium ${selectedTask.status === 'completed' ? 'text-green-600' : 'text-red-600'
                    }`}>
                    {selectedTask.status === 'completed' ? 'Verified Complete' : 'Failed Verification'}
                  </span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500">Due Date:</span>
                  <span>{formatDateTime(selectedTask.due_date)}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500">Completed:</span>
                  <span>{formatDateTime(selectedTask.updated_at || '')}</span>
                </div>
                <div className="flex justify-between text-sm">
                  <span className="text-gray-500">Required Duration:</span>
                  <span>{formatDuration(selectedTask.min_duration)}</span>
                </div>
                {selectedTask.video_path && (
                  <div className="flex justify-between text-sm">
                    <span className="text-gray-500">Video:  </span>
                    <button
                      onClick={() => {
                        // Open video in default player
                        const { open } = require('@tauri-apps/plugin-opener');
                        open(selectedTask.video_path!);
                      }}
                      className="text-primary hover:underline cursor-pointer text-left break-all"
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
