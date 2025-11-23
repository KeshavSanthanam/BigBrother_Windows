import { useEffect, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTaskStore } from '../store/taskStore';
import TaskCard from '../components/tasks/TaskCard';
import CreateTaskModal from '../components/tasks/CreateTaskModal';
import ThemeToggle from '../components/ThemeToggle';

export default function Dashboard() {
  const navigate = useNavigate();
  const { tasks, loading, fetchPendingTasks } = useTaskStore();
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);

  useEffect(() => {
    fetchPendingTasks();
  }, [fetchPendingTasks]);

  return (
    <div className="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="flex justify-between items-center mb-8">
          <div>
            <h1 className="text-3xl font-bold text-gray-900 dark:text-white">BigBrother</h1>
            <p className="text-gray-600 dark:text-gray-300 mt-1">Productivity Accountability Tracker</p>
          </div>
          <div className="flex gap-3">
            <ThemeToggle />
            <button
              onClick={() => navigate('/settings')}
              className="btn btn-secondary"
            >
              ⚙️ Settings
            </button>
            <button
              onClick={() => navigate('/history')}
              className="btn btn-secondary"
            >
              📋 Task History
            </button>
            <button
              onClick={() => setIsCreateModalOpen(true)}
              className="btn btn-primary bg-purple hover:bg-purple-dark"
            >
              + Create Task
            </button>
          </div>
        </div>

        {loading ? (
          <div className="flex justify-center items-center h-64">
            <div className="text-gray-500 dark:text-gray-400">Loading tasks...</div>
          </div>
        ) : tasks.length === 0 ? (
          <div className="text-center py-12">
            <div className="text-gray-400 dark:text-gray-500 text-lg mb-4">No pending tasks</div>
            <button
              onClick={() => setIsCreateModalOpen(true)}
              className="btn btn-primary bg-purple hover:bg-purple-dark"
            >
              Create your first task
            </button>
          </div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {tasks.map((task) => (
              <TaskCard
                key={task.id}
                task={task}
                onStartRecording={(taskId) => navigate(`/recording/${taskId}`)}
              />
            ))}
          </div>
        )}

        <CreateTaskModal
          isOpen={isCreateModalOpen}
          onClose={() => setIsCreateModalOpen(false)}
        />
      </div>
    </div>
  );
}
