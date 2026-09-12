import { invoke } from '@tauri-apps/api/core';
import { toast } from 'sonner';

export function formatInvokeError(error: unknown): string {
  if (typeof error === 'string') {
    const trimmed = error.trim();
    if (trimmed && trimmed !== 'undefined' && trimmed !== 'null') {
      return trimmed;
    }
  }

  if (error instanceof Error) {
    const message = error.message?.trim();
    if (message && message !== 'undefined') {
      return message;
    }
  }

  if (error && typeof error === 'object') {
    const record = error as Record<string, unknown>;
    for (const key of ['message', 'error', 'msg']) {
      const value = record[key];
      if (typeof value === 'string') {
        const trimmed = value.trim();
        if (trimmed && trimmed !== 'undefined' && trimmed !== 'null') {
          return trimmed;
        }
      }
    }
  }

  return '詳細を取得できませんでした';
}

export function formatActionError(failure: string, error: unknown): string {
  const detail = formatInvokeError(error);
  if (detail.startsWith(failure) || detail.includes(failure)) {
    return detail;
  }
  return `${failure}: ${detail}`;
}

export function showErrorToast(message: string) {
  toast.error(message, {
    duration: 12000,
    action: {
      label: 'コピー',
      onClick: () => {
        void navigator.clipboard.writeText(message);
      },
    },
  });
}

export function showSuccessToast(message: string) {
  toast.success(message);
}

export async function invokeAction(command: string, failure: string, args?: Record<string, unknown>) {
  try {
    await invoke(command, args);
    return { ok: true as const };
  } catch (error) {
    console.error(error);
    return { ok: false as const, message: formatActionError(failure, error) };
  }
}

