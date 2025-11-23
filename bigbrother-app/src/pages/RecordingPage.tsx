import { useEffect, useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useRecordingStore } from '../store/recordingStore';
import { useTaskStore } from '../store/taskStore';
import RecordingControls from '../components/recording/RecordingControls';
import RecordingIndicator from '../components/recording/RecordingIndicator';
import DurationDisplay from '../components/recording/DurationDisplay';
import { verificationApi } from '../lib/api';
import type { Task } from '../lib/types';

export default function RecordingPage() {
  const { taskId } = useParams<{ taskId: string }>();
  const navigate = useNavigate();
  const { status, startRecording, pauseRecording, resumeRecording, stopRecording, updateDuration } = useRecordingStore();
  const { tasks } = useTaskStore();
  const [task, setTask] = useState<Task | null>(null);
  const [isVerifying, setIsVerifying] = useState(false);
  const [verificationComplete, setVerificationComplete] = useState(false);
  const [isStopping, setIsStopping] = useState(false);

  useEffect(() => {
    if (taskId) {
      const foundTask = tasks.find(t => t.id === parseInt(taskId));
      if (foundTask) {
        setTask(foundTask);
      }
    }
  }, [taskId, tasks]);

  useEffect(() => {
    // Update duration every second when recording
    let interval: number | undefined;
    if (status.is_recording && !status.is_paused && !isStopping) {
      interval = window.setInterval(() => {
        updateDuration(status.duration + 1);
      }, 1000);
    }
    return () => {
      if (interval) clearInterval(interval);
    };
  }, [status.is_recording, status.is_paused, status.duration, updateDuration, isStopping]);

  const handleStart = async () => {
    if (taskId) {
      await startRecording(parseInt(taskId));
    }
  };

  const handlePause = async () => {
    await pauseRecording();
  };

  const handleResume = async () => {
    await resumeRecording();
  };

  const handleStop = async () => {
    if (!task) return;

    const meetsMinDuration = status.duration >= task.min_duration;

    if (!meetsMinDuration) {
      const confirmed = window.confirm(
        `You haven't met the minimum duration of ${Math.floor(task.min_duration / 60)} minutes yet. Are you sure you want to stop?`
      );
      if (!confirmed) return;
    }

    // Immediately stop the duration timer
    setIsStopping(true);

    try {
      const videoPath = await stopRecording();

      // Start AI verification
      setIsVerifying(true);
      try {
        const result = await verificationApi.verify(parseInt(taskId!));
        setIsVerifying(false);
        setVerificationComplete(true);

        // Show result
        alert(
          result.verified
            ? `✅ Task Verified!\n\nConfidence: ${result.confidence}%\nTime on task: ${result.time_on_task_minutes.toFixed(1)} minutes\n\n${result.explanation}`
            : `❌ Task Not Completed\n\nConfidence: ${result.confidence}%\nTime on task: ${result.time_on_task_minutes.toFixed(1)} minutes\n\n${result.explanation}\n\nIssues:\n${result.issues.join('\n')}`
        );

        // Navigate back to dashboard
        navigate('/');
      } catch (error) {
        setIsVerifying(false);
        alert(`Verification failed: ${error}`);
        navigate('/');
      }
    } catch (error) {
      alert(`Failed to stop recording: ${error}`);
      // Still navigate away even if stop failed
      navigate('/');
    }
  };

  if (!task) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-gray-500">Loading task...</div>
      </div>
    );
  }

  if (isVerifying) {
    return (
      <div className="min-h-screen bg-gray-50 flex items-center justify-center">
        <div className="text-center">
          <div className="text-2xl font-bold text-gray-900 mb-4">Verifying with AI...</div>
          <div className="text-gray-600 mb-4">Extracting frames and analyzing video</div>
          <div className="animate-spin rounded-full h-16 w-16 border-b-2 border-primary mx-auto"></div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-900 text-white">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="mb-8">
          <button
            onClick={() => navigate('/')}
            className="text-gray-400 hover:text-white mb-4"
          >
            ← Back to Dashboard
          </button>
          <h1 className="text-3xl font-bold">{task.title}</h1>
          {task.description && (
            <p className="text-gray-400 mt-2">{task.description}</p>
          )}
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          <div className="lg:col-span-2">
            <div className="bg-gray-800 rounded-lg p-8 aspect-video flex items-center justify-center">
              <div className="text-center">
                <RecordingIndicator
                  isRecording={status.is_recording}
                  isPaused={status.is_paused}
                />
                <div className="mt-8 text-gray-400">
                  Screen recording preview will appear here
                </div>
              </div>
            </div>

            <div className="mt-6">
              <RecordingControls
                isRecording={status.is_recording}
                isPaused={status.is_paused}
                onStart={handleStart}
                onPause={handlePause}
                onResume={handleResume}
                onStop={handleStop}
              />
            </div>
          </div>

          <div className="space-y-6">
            <div className="bg-gray-800 rounded-lg p-6">
              <h3 className="text-lg font-semibold mb-4">Recording Duration</h3>
              <DurationDisplay
                currentDuration={status.duration}
                minDuration={task.min_duration}
              />
            </div>

            <div className="bg-gray-800 rounded-lg p-6">
              <h3 className="text-lg font-semibold mb-4">Task Details</h3>
              <div className="space-y-2 text-sm">
                <div className="flex justify-between">
                  <span className="text-gray-400">Minimum Duration:</span>
                  <span>{Math.floor(task.min_duration / 60)} minutes</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-400">Status:</span>
                  <span className="capitalize">{task.status}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
