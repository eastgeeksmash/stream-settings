import type React from 'react';
import { Button } from '../ui/button';
import { toast } from "sonner"
import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';
import { Loader2 } from 'lucide-react';

type Action = {
  command: string;
  label: string;
  success: string;
  failure: string;
  title?: string;
};

const actions: Action[] = [
  {
    command: 'make_network_private',
    label: '全ネットワークをプライベート化',
    success: 'ネットワークのプライベート化が完了しました',
    failure: 'ネットワークのプライベート化に失敗しました',
  },
  {
    command: 'change_power_settings',
    label: '電源設定を変更(高パフォーマンス・スリープ/自動電源オフ無効)',
    success: '電源設定の変更が完了しました',
    failure: '電源設定の変更に失敗しました',
    title: '高パフォーマンスモードに変更し、スリープ・自動電源オフ・電源ボタンを無効化します',
  },
  {
    command: 'disable_windows_notifications',
    label: '通知を無効化',
    success: '通知センターの無効化が完了しました',
    failure: '通知センターの無効化に失敗しました',
  },
  {
    command: 'disable_aero',
    label: 'Windows Aeroを無効化',
    success: 'Windows Aeroの無効化が完了しました',
    failure: 'Windows Aeroの無効化に失敗しました',
  },
  {
    command: 'set_solid_wallpaper',
    label: '壁紙を単色(黒)に変更',
    success: '壁紙の変更が完了しました',
    failure: '壁紙の変更に失敗しました',
  },
  {
    command: 'hide_desktop_icons_and_taskbar',
    label: 'デスクトップアイコン/タスクバーを非表示',
    success: 'デスクトップアイコンとタスクバーの非表示が完了しました',
    failure: 'デスクトップアイコン/タスクバーの非表示に失敗しました',
  },
  {
    command: 'disable_sticky_keys',
    label: '固定キーを無効化',
    success: '固定キー設定の無効化が完了しました',
    failure: '固定キー設定の無効化に失敗しました',
  },
  {
    command: 'disable_onedrive_sync',
    label: 'OneDrive同期を無効化',
    success: 'OneDrive同期の無効化が完了しました',
    failure: 'OneDrive同期の無効化に失敗しました',
  },
  {
    command: 'disable_delivery_optimization',
    label: 'Windows Updateのファイル配信を無効化',
    success: 'Windows Updateのファイル配信無効化が完了しました',
    failure: 'Windows Updateのファイル配信無効化に失敗しました',
  },
  {
    command: 'defer_windows_update',
    label: 'Windows Updateを延期',
    success: 'Windows Updateの延期が完了しました',
    failure: 'Windows Updateの延期に失敗しました',
  },
];

export const SetupPage: React.FC = () => {
  const [loadingCommand, setLoadingCommand] = useState<string | null>(null);

  const run = (action: Action) => {
    setLoadingCommand(action.command);
    invoke(action.command)
      .then(() => {
        toast.success(action.success);
      })
      .catch((error: Error) => {
        console.error(error);
        toast.error(`${action.failure}: ${error.message}`);
      })
      .finally(() => {
        setLoadingCommand(null);
      });
  };

  const renderAction = (action: Action) => {
    const loading = loadingCommand === action.command;
    return (
      <li key={action.command}>
        <Button
          type="button"
          className="w-full"
          variant="default"
          title={action.title}
          disabled={loading}
          onClick={() => run(action)}
        >
          {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
          {action.label}
        </Button>
      </li>
    );
  };

  return (
    <>
      <h2 className="text-2xl font-bold mb-4">Setup</h2>
      <ul className="space-y-3">
        {actions.slice(0, 1).map(renderAction)}
        <li>
          <Button type="button" className="w-full" variant="default" disabled>
            OBSの初期設定を実施
          </Button>
        </li>
        {actions.slice(1).map(renderAction)}
      </ul>
    </>
  );
};
