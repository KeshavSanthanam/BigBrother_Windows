interface RecordingControlsProps {
  isRecording: boolean;
  isPaused: boolean;
  onStart: () => void;
  onPause: () => void;
  onResume: () => void;
  onStop: () => void;
}

export default function RecordingControls({
  isRecording,
  isPaused,
  onStart,
  onPause,
  onResume,
  onStop,
}: RecordingControlsProps) {
  if (!isRecording) {
    return (
      <div className="flex justify-center">
        <button onClick={onStart} className="btn btn-danger text-lg px-8 py-3">
          ● Start Recording
        </button>
      </div>
    );
  }

  return (
    <div className="flex justify-center gap-4">
      {isPaused ? (
        <button onClick={onResume} className="btn btn-success px-6">
          ▶ Resume
        </button>
      ) : (
        <button onClick={onPause} className="btn btn-secondary px-6">
          ⏸ Pause
        </button>
      )}
      <button onClick={onStop} className="btn btn-danger px-6">
        ■ Stop Recording
      </button>
    </div>
  );
}
