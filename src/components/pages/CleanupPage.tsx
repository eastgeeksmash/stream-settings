import type React from 'react';
import { Button } from '../ui/button';
import { useState } from 'react';
import { Loader2 } from 'lucide-react';
import { ActionTooltip } from '@/components/action-tooltip';
import { invokeAction, showErrorToast, showSuccessToast } from '@/lib/invoke';

type Action = {
  command: string;
  label: string;
  success: string;
  failure: string;
  description: string;
};

const actions: Action[] = [
  {
    command: 'logout_discord',
    label: 'Discordログアウト',
    success: 'Discordのログアウトが完了しました',
    failure: 'Discordのログアウトに失敗しました',
    description: 'Discordを終了し、このPCからログアウトします。次の人が入れなくなります。',
  },
  {
    command: 'logout_chrome',
    label: 'Chromeログアウト',
    success: 'Chromeのログアウトが完了しました',
    failure: 'Chromeのログアウトに失敗しました',
    description: 'ブラウザでGoogleアカウントからログアウトします。',
  },
  {
    command: 'delete_download_directory',
    label: 'ダウンロードディレクトリを削除',
    success: 'ダウンロードディレクトリの削除が完了しました',
    failure: 'ダウンロードディレクトリの削除に失敗しました',
    description: 'ダウンロードフォルダの中身をすべて消します。フォルダ自体は残ります。',
  },
  {
    command: 'purge_obs_settings',
    label: 'OBS設定を削除',
    success: 'OBS設定の削除が完了しました',
    failure: 'OBS設定の削除に失敗しました',
    description: 'OBSを終了し、配信ソフトの設定を初期状態に戻します。シーンなども消えます。',
  },
  {
    command: 'purge_vmix_settings',
    label: 'vMix設定を削除',
    success: 'vMix設定の削除が完了しました',
    failure: 'vMix設定の削除に失敗しました',
    description: 'vMixを終了し、設定を初期状態に戻します。登録キーは残ります。',
  },
  {
    command: 'reset_vmix_registration',
    label: 'vMix登録キーを削除',
    success: 'vMix登録キーの削除が完了しました',
    failure: 'vMix登録キーの削除に失敗しました',
    description: 'vMixを終了し、このPCに保存された登録キーを消します。次回起動時に再入力が必要です。',
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
              <ActionTooltip text={action.description}>
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
              </ActionTooltip>
            </li>
          );
        })}
      </ul>
    </>
  );
};
