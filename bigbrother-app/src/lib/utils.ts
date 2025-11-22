import { formatDistanceToNow, isPast, differenceInSeconds, format } from 'date-fns';

export function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  if (hours > 0) {
    return `${hours}h ${minutes}m ${secs}s`;
  } else if (minutes > 0) {
    return `${minutes}m ${secs}s`;
  } else {
    return `${secs}s`;
  }
}

export function formatCountdown(dueDate: string): string {
  try {
    return formatDistanceToNow(new Date(dueDate), { addSuffix: true });
  } catch {
    return 'Invalid date';
  }
}

export function isOverdue(dueDate: string): boolean {
  try {
    return isPast(new Date(dueDate));
  } catch {
    return false;
  }
}

export function getUrgencyColor(dueDate: string): string {
  try {
    const now = new Date();
    const due = new Date(dueDate);
    const secondsUntilDue = differenceInSeconds(due, now);

    if (secondsUntilDue < 0) {
      return 'text-danger'; // Overdue
    } else if (secondsUntilDue < 3600) {
      return 'text-danger'; // Less than 1 hour
    } else if (secondsUntilDue < 86400) {
      return 'text-warning'; // Less than 24 hours
    } else {
      return 'text-success'; // More than 24 hours
    }
  } catch {
    return 'text-gray-500';
  }
}

export function formatDateTime(date: string): string {
  try {
    return format(new Date(date), 'MMM d, yyyy h:mm a');
  } catch {
    return 'Invalid date';
  }
}

export function formatDate(date: string): string {
  try {
    return format(new Date(date), 'MMM d, yyyy');
  } catch {
    return 'Invalid date';
  }
}

export function formatTime(date: string): string {
  try {
    return format(new Date(date), 'h:mm a');
  } catch {
    return 'Invalid time';
  }
}
