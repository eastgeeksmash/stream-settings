import type React from 'react';
import { Button } from '../ui/button';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useMemo, useState } from 'react';
import { Loader2 } from 'lucide-react';
import { invokeAction, showErrorToast, showSuccessToast } from '@/lib/invoke';

type Action = {
  command: string;
  label: string;
  success: string;
  failure: string;
  title?: string;
  batch?: boolean;
};

type NetworkAdapter = {
  id: string;
  name: string;
  description: string;
  dhcp_enabled: boolean;
  connected: boolean;
};

const actions: Action[] = [
  {
    command: 'make_network_private',
    label: '全ネットワークをプライベート化',
    success: 'ネットワークのプライベート化が完了しました',
    failure: 'ネットワークのプライベート化に失敗しました',
    batch: true,
  },
  {
    command: 'change_power_settings',
    label: '電源設定を変更(高パフォーマンス・スリープ/自動電源オフ無効)',
    success: '電源設定の変更が完了しました',
    failure: '電源設定の変更に失敗しました',
    title: '高パフォーマンスモードに変更し、スリープ・自動電源オフ・電源ボタンを無効化します',
    batch: true,
  },
  {
    command: 'disable_windows_notifications',
    label: '通知を無効化',
    success: '通知センターの無効化が完了しました',
    failure: '通知センターの無効化に失敗しました',
    batch: true,
  },
  {
    command: 'disable_aero',
    label: 'Windows Aeroを無効化',
    success: 'Windows Aeroの無効化が完了しました',
    failure: 'Windows Aeroの無効化に失敗しました',
    batch: true,
  },
  {
    command: 'restore_aero',
    label: 'Windows Aeroを元に戻す',
    success: 'Windows Aeroの復元が完了しました',
    failure: 'Windows Aeroの復元に失敗しました',
  },
  {
    command: 'set_solid_wallpaper',
    label: '壁紙を単色(黒)に変更',
    success: '壁紙の変更が完了しました',
    failure: '壁紙の変更に失敗しました',
    batch: true,
  },
  {
    command: 'hide_desktop_icons_and_taskbar',
    label: 'デスクトップアイコン/タスクバーを非表示',
    success: 'デスクトップアイコンとタスクバーの非表示が完了しました',
    failure: 'デスクトップアイコン/タスクバーの非表示に失敗しました',
    batch: true,
  },
  {
    command: 'restore_desktop_icons_and_taskbar',
    label: 'デスクトップアイコン/タスクバーを再表示',
    success: 'デスクトップアイコンとタスクバーの再表示が完了しました',
    failure: 'デスクトップアイコン/タスクバーの再表示に失敗しました',
  },
  {
    command: 'disable_sticky_keys',
    label: '固定キーを無効化',
    success: '固定キー設定の無効化が完了しました',
    failure: '固定キー設定の無効化に失敗しました',
    batch: true,
  },
  {
    command: 'disable_onedrive_sync',
    label: 'OneDrive同期を無効化',
    success: 'OneDrive同期の無効化が完了しました',
    failure: 'OneDrive同期の無効化に失敗しました',
    batch: true,
  },
  {
    command: 'disable_delivery_optimization',
    label: 'Windows Updateのファイル配信を無効化',
    success: 'Windows Updateのファイル配信無効化が完了しました',
    failure: 'Windows Updateのファイル配信無効化に失敗しました',
    batch: true,
  },
  {
    command: 'defer_windows_update',
    label: 'Windows Updateを延期',
    success: 'Windows Updateの延期が完了しました',
    failure: 'Windows Updateの延期に失敗しました',
    batch: true,
  },
];

export const SetupPage: React.FC = () => {
  const batchable = useMemo(() => actions.filter((action) => action.batch), []);
  const [loadingCommand, setLoadingCommand] = useState<string | null>(null);
  const [selected, setSelected] = useState<string[]>(() => batchable.map((action) => action.command));
  const [adapters, setAdapters] = useState<NetworkAdapter[]>([]);

  useEffect(() => {
    invoke<NetworkAdapter[]>('list_network_adapters')
      .then(setAdapters)
      .catch((error) => {
        console.error(error);
      });
  }, []);

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

  const runSelected = async () => {
    const targets = actions.filter((action) => selected.includes(action.command));
    if (targets.length === 0) {
      showErrorToast('まとめて実行する項目を選択してください。');
      return;
    }

    setLoadingCommand('batch');
    const failures: string[] = [];
    for (const action of targets) {
      const result = await invokeAction(action.command, action.failure);
      if (!result.ok) {
        failures.push(result.message);
      }
    }
    if (failures.length === 0) {
      showSuccessToast('選択した最適化項目の実行が完了しました');
    } else {
      showErrorToast(failures.join('\n'));
    }
    setLoadingCommand(null);
  };

  const enableAdapterDhcp = async (adapter: NetworkAdapter) => {
    setLoadingCommand(adapter.id);
    const result = await invokeAction('enable_dhcp', 'DHCP設定の変更に失敗しました', {
      adapterId: adapter.id,
    });
    if (result.ok) {
      showSuccessToast(`${adapter.name} をDHCPに変更しました`);
      const next = await invoke<NetworkAdapter[]>('list_network_adapters').catch(() => adapters);
      setAdapters(next);
    } else {
      showErrorToast(result.message);
    }
    setLoadingCommand(null);
  };

  const enableConnectedDhcp = async () => {
    setLoadingCommand('dhcp-all');
    const result = await invokeAction(
      'enable_dhcp_for_connected_adapters',
      'DHCP設定の変更に失敗しました'
    );
    if (result.ok) {
      showSuccessToast('接続中アダプターのDHCP設定が完了しました');
      const next = await invoke<NetworkAdapter[]>('list_network_adapters').catch(() => adapters);
      setAdapters(next);
    } else {
      showErrorToast(result.message);
    }
    setLoadingCommand(null);
  };

  const renderAction = (action: Action) => {
    const loading = loadingCommand === action.command || loadingCommand === 'batch';
    return (
      <li key={action.command} className="flex items-center gap-3">
        {action.batch ? (
          <input
            type="checkbox"
            className="size-4 shrink-0"
            checked={selected.includes(action.command)}
            onChange={(event) => {
              setSelected((current) =>
                event.target.checked
                  ? [...current, action.command]
                  : current.filter((command) => command !== action.command)
              );
            }}
            aria-label={`${action.label}をまとめて実行に含める`}
          />
        ) : (
          <span className="size-4 shrink-0" />
        )}
        <Button
          type="button"
          className="w-full"
          variant="default"
          title={action.title}
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
  };

  return (
    <>
      <h2 className="text-2xl font-bold mb-4">Setup</h2>
      <ul className="space-y-3">
        <li>
          <Button
            type="button"
            className="w-full"
            variant="default"
            disabled={loadingCommand !== null}
            onClick={() => {
              void runSelected();
            }}
          >
            {loadingCommand === 'batch' && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            選択した項目をまとめて実行
          </Button>
        </li>
        {actions.slice(0, 1).map(renderAction)}
        <li>
          <Button type="button" className="w-full" variant="default" disabled>
            OBSの初期設定を実施
          </Button>
        </li>
        {actions.slice(1).map(renderAction)}
        <li>
          <Button
            type="button"
            className="w-full"
            variant="default"
            disabled={loadingCommand !== null}
            onClick={() => {
              void enableConnectedDhcp();
            }}
          >
            {loadingCommand === 'dhcp-all' && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            接続中アダプターをDHCPにする
          </Button>
        </li>
        {adapters.map((adapter) => (
          <li key={adapter.id}>
            <Button
              type="button"
              className="w-full"
              variant="default"
              disabled={loadingCommand !== null || adapter.dhcp_enabled}
              title={adapter.description}
              onClick={() => {
                void enableAdapterDhcp(adapter);
              }}
            >
              {loadingCommand === adapter.id && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              {adapter.name} をDHCPにする
              {adapter.dhcp_enabled ? ' (設定済み)' : ''}
            </Button>
          </li>
        ))}
      </ul>
    </>
  );
};
