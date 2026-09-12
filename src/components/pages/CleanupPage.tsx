import type React from 'react';
import { Button } from '../ui/button';
import { useState } from 'react';
import { Loader2 } from 'lucide-react';
import { invokeAction, showErrorToast, showSuccessToast } from '@/lib/invoke';

type Action = {
  command: string;
  label: string;
  success: string;
  failure: string;
};

const actions: Action[] = [
  {
    command: 'logout_discord',
    label: 'Discordログアウト',
    success: 'Discordのログアウトが完了しました',
    failure: 'Discordのログアウトに失敗しました',
  },
  {
    command: 'logout_chrome',
    label: 'Chromeログアウト',
    success: 'Chromeのログアウトが完了しました',
    failure: 'Chromeのログアウトに失敗しました',
  },
  {
    command: 'delete_download_directory',
    label: 'ダウンロードディレクトリを削除',
    success: 'ダウンロードディレクトリの削除が完了しました',
    failure: 'ダウンロードディレクトリの削除に失敗しました',
  },
  {
    command: 'purge_obs_settings',
    label: 'OBS設定を削除',
    success: 'OBS設定の削除が完了しました',
    failure: 'OBS設定の削除に失敗しました',
  },
  {
    command: 'purge_vmix_settings',
    label: 'vMix設定を削除',
    success: 'vMix設定の削除が完了しました',
    failure: 'vMix設定の削除に失敗しました',
  },
];

export const CleanupPage: React.FC = () => {
  const [loadingCommand, setLoadingCommand] = useState<string | null>(null);

  const run = async (action: Action) => {
    setLoadingCommand(action.command);
    const result = await invokeAction(action.command, action.failure);
    if (result.ok) {
      showSuccessToast(action.success);
    } else {
      showErrorToast(result.message);
    }
    setLoadingCommand(null);
  };

  return (
    <>
      <h2 className="text-2xl font-bold mb-4">Cleanup</h2>
      
      <ul className="space-y-3">
        {actions.map((action) => {
          const loading = loadingCommand === action.command;
          return (
            <li key={action.command}>
              <Button
                type="button"
                className="w-full"
                variant="default"
                disabled={loading}
                onClick={() => {
                  void run(action);
                }}
              >
                {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                {action.label}
              </Button>
            </li>
          );
        })}
      </ul>
    </>
  );
};
