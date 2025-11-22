interface RecordingIndicatorProps {
  isRecording: boolean;
  isPaused: boolean;
}

export default function RecordingIndicator({ isRecording, isPaused }: RecordingIndicatorProps) {
  if (!isRecording) {
    return (
      <div className="text-center">
        <div className="text-gray-400 text-xl">Ready to Record</div>
        <div className="text-gray-500 text-sm mt-2">Click Start to begin</div>
      </div>
    );
  }

  if (isPaused) {
    return (
      <div className="text-center">
        <div className="flex items-center justify-center gap-3 text-yellow-500 text-xl">
          <div className="w-4 h-4 rounded-full bg-yellow-500"></div>
          <span>Recording Paused</span>
        </div>
        <div className="text-gray-400 text-sm mt-2">Click Resume to continue</div>
      </div>
    );
  }

  return (
    <div className="text-center">
      <div className="flex items-center justify-center gap-3 text-red-500 text-xl">
        <div className="recording-dot"></div>
        <span className="font-bold">RECORDING</span>
      </div>
      <div className="text-gray-400 text-sm mt-2">Capturing all displays and webcam</div>
    </div>
  );
}
