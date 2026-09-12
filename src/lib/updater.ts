import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { formatInvokeError, showErrorToast, showSuccessToast } from '@/lib/invoke';

export type CheckAndApplyUpdateOptions = {
  notifyWhenLatest?: boolean;
  notifyOnError?: boolean;
};

export async function checkAndApplyUpdate(
  options: CheckAndApplyUpdateOptions = {},
) {
  const { notifyWhenLatest = false, notifyOnError = true } = options;

  try {
    const update = await check();
    if (!update) {
      if (notifyWhenLatest) {
        showSuccessToast('最新バージョンです');
      }
      return { status: 'latest' as const };
    }

    await update.downloadAndInstall();
    showSuccessToast('更新をインストールしました。再起動します');
    await relaunch();
    return { status: 'installed' as const };
  } catch (error) {
    if (notifyOnError) {
      showErrorToast(`更新の確認に失敗しました: ${formatInvokeError(error)}`);
    }
    return { status: 'error' as const, error };
  }
}
