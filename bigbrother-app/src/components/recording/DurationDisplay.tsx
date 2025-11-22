import { formatDuration } from '../../lib/utils';

interface DurationDisplayProps {
  currentDuration: number;
  minDuration: number;
}

export default function DurationDisplay({ currentDuration, minDuration }: DurationDisplayProps) {
  const progress = (currentDuration / minDuration) * 100;
  const meetsMinimum = currentDuration >= minDuration;

  return (
    <div className="space-y-4">
      <div className="text-center">
        <div className="text-4xl font-bold mb-2">
          {formatDuration(currentDuration)}
        </div>
        <div className="text-gray-400 text-sm">
          Required: {formatDuration(minDuration)}
        </div>
      </div>

      <div className="relative pt-1">
        <div className="flex mb-2 items-center justify-between">
          <div>
            <span className={`text-xs font-semibold inline-block py-1 px-2 uppercase rounded-full ${
              meetsMinimum ? 'text-green-600 bg-green-200' : 'text-blue-600 bg-blue-200'
            }`}>
              {Math.min(progress, 100).toFixed(0)}%
            </span>
          </div>
          {meetsMinimum && (
            <div className="text-green-500 text-sm">✓ Minimum reached!</div>
          )}
        </div>
        <div className="overflow-hidden h-2 mb-4 text-xs flex rounded bg-gray-700">
          <div
            style={{ width: `${Math.min(progress, 100)}%` }}
            className={`shadow-none flex flex-col text-center whitespace-nowrap text-white justify-center transition-all duration-500 ${
              meetsMinimum ? 'bg-green-500' : 'bg-blue-500'
            }`}
          ></div>
        </div>
      </div>
    </div>
  );
}
