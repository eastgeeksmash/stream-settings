import type React from 'react';
import { Button } from '../ui/button';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useMemo, useRef, useState } from 'react';
import { Loader2 } from 'lucide-react';
import { ActionTooltip } from '@/components/action-tooltip';
import { invokeAction, showErrorToast, showSuccessToast } from '@/lib/invoke';

type Action = {
  command: string;
  label: string;
  success: string;
  failure: string;
  description: string;
  batch?: boolean;
  defaultSelected?: boolean;
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
    command: 'change_power_settings',
    label: '電源設定を変更(高パフォーマンス・スリープ/自動電源オフ無効)',
    success: '電源設定の変更が完了しました',
    failure: '電源設定の変更に失敗しました',
    description: '配信中に画面が消えたり、スリープしたりしないようにします。電源ボタンを押しても、すぐには切れません。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_windows_notifications',
    label: '通知を無効化',
    success: '通知センターの無効化が完了しました',
    failure: '通知センターの無効化に失敗しました',
    description: '画面の端に出る通知を止めます。配信中の割り込みを減らせます。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'make_network_private',
    label: '全ネットワークをプライベート化',
    success: 'ネットワークのプライベート化が完了しました',
    failure: 'ネットワークのプライベート化に失敗しました',
    description: 'このPCのネットワークを、会場で機材とつなぎやすい設定にします。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_sticky_keys',
    label: '固定キーを無効化',
    success: '固定キー設定の無効化が完了しました',
    failure: '固定キー設定の無効化に失敗しました',
    description: 'Shiftキーを連続で押したときに出る確認画面を出さないようにします。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_onedrive_sync',
    label: 'OneDrive同期を無効化',
    success: 'OneDrive同期の無効化が完了しました',
    failure: 'OneDrive同期の無効化に失敗しました',
    description: 'ファイルの自動保存を止めます。配信中の余計な通信を減らします。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_delivery_optimization',
    label: 'Windows Updateのファイル配信を無効化',
    success: 'Windows Updateのファイル配信無効化が完了しました',
    failure: 'Windows Updateのファイル配信無効化に失敗しました',
    description: 'このPCがほかのPCへ更新ファイルを配らないようにします。回線を専有されにくくなります。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'defer_windows_update',
    label: 'Windows Updateを延期',
    success: 'Windows Updateの延期が完了しました',
    failure: 'Windows Updateの延期に失敗しました',
    description: 'Windowsの更新を後回しにします。配信中に突然の再起動が入りにくくなります。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_aero',
    label: 'Windows Aeroを無効化',
    success: 'Windows Aeroの無効化が完了しました',
    failure: 'Windows Aeroの無効化に失敗しました',
    description: 'ウィンドウの動きや影を止めます。画面が軽くなり、配信が安定しやすくなります。',
    batch: true,
  },
  {
    command: 'restore_aero',
    label: 'Windows Aeroを元に戻す',
    success: 'Windows Aeroの復元が完了しました',
    failure: 'Windows Aeroの復元に失敗しました',
    description: 'ウィンドウの動きや影を、いつもの見た目に戻します。',
  },
  {
    command: 'set_solid_wallpaper',
    label: '壁紙を単色(黒)に変更',
    success: '壁紙の変更が完了しました',
    failure: '壁紙の変更に失敗しました',
    description: '壁紙を真っ黒にします。配信に余計な画像が映り込みにくくなります。',
    batch: true,
  },
  {
    command: 'hide_desktop_icons_and_taskbar',
    label: 'デスクトップアイコン/タスクバーを非表示',
    success: 'デスクトップアイコンとタスクバーの非表示が完了しました',
    failure: 'デスクトップアイコン/タスクバーの非表示に失敗しました',
    description: 'デスクトップのアイコンと、画面下のバーを隠します。配信に映り込みにくくなります。',
    batch: true,
  },
  {
    command: 'restore_desktop_icons_and_taskbar',
    label: 'デスクトップアイコン/タスクバーを再表示',
    success: 'デスクトップアイコンとタスクバーの再表示が完了しました',
    failure: 'デスクトップアイコン/タスクバーの再表示に失敗しました',
    description: '隠していたアイコンと、画面下のバーをまた表示します。',
  },
  {
    command: 'optimize_ndi_settings',
    label: 'NDI/OMT向けに最適化',
    success: 'NDI/OMT向け最適化が完了しました',
    failure: 'NDI/OMT向け最適化に失敗しました',
    description: '映像を送る回線の省電力と優先制御を止めます。途切れにくくします。',
    batch: true,
  },
  {
    command: 'optimize_vmix_settings',
    label: 'vMix向けに最適化',
    success: 'vMix向け最適化が完了しました',
    failure: 'vMix向け最適化に失敗しました',
    description: 'vMix向けに、回線の待ち時間とゲーム機能の邪魔を減らします。',
    batch: true,
  },
];

export const SetupPage: React.FC = () => {
  const batchable = useMemo(() => actions.filter((action) => action.batch), []);
  const batchableCommands = useMemo(() => batchable.map((action) => action.command), [batchable]);
  const recommendedActions = useMemo(
    () => actions.filter((action) => action.defaultSelected),
    []
  );
  const extraActions = useMemo(
    () => actions.filter((action) => !action.defaultSelected),
    []
  );
  const [loadingCommand, setLoadingCommand] = useState<string | null>(null);
  const [selected, setSelected] = useState<string[]>(() =>
    actions.filter((action) => action.defaultSelected).map((action) => action.command)
  );
  const [adapters, setAdapters] = useState<NetworkAdapter[]>([]);
  const selectAllRef = useRef<HTMLInputElement>(null);
  const allSelected = batchableCommands.length > 0 && batchableCommands.every((command) => selected.includes(command));
  const someSelected = batchableCommands.some((command) => selected.includes(command));

  useEffect(() => {
    invoke<NetworkAdapter[]>('list_network_adapters')
      .then(setAdapters)
      .catch((error) => {
        console.error(error);
      });
  }, []);

  useEffect(() => {
    if (selectAllRef.current) {
      selectAllRef.current.indeterminate = someSelected && !allSelected;
    }
  }, [allSelected, someSelected]);

  const toggleSelected = (command: string, checked: boolean) => {
    setSelected((current) =>
      checked ? [...current, command] : current.filter((item) => item !== command)
    );
  };

  const toggleAll = (checked: boolean) => {
    setSelected(checked ? batchableCommands : []);
  };

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

  const renderRow = (key: string, control: React.ReactNode, button: React.ReactNode) => (
    <li key={key} className="grid grid-cols-[1rem_minmax(0,1fr)] items-center gap-3">
      <div className="flex size-4 items-center justify-center">{control}</div>
      {button}
    </li>
  );

  const renderCheckbox = (
    checked: boolean,
    onChange: (checked: boolean) => void,
    label: string,
    ref?: React.Ref<HTMLInputElement>
  ) => (
    <input
      ref={ref}
      type="checkbox"
      className="action-check"
      checked={checked}
      onChange={(event) => {
        onChange(event.target.checked);
      }}
      aria-label={label}
      title={label}
    />
  );

  const renderAction = (action: Action) => {
    const loading = loadingCommand === action.command || loadingCommand === 'batch';
    return renderRow(
      action.command,
      action.batch
        ? renderCheckbox(
            selected.includes(action.command),
            (checked) => {
              toggleSelected(action.command, checked);
            },
            `${action.label}をまとめて実行に含める`
          )
        : null,
      <ActionTooltip text={action.description}>
        <Button
          type="button"
          className="w-full min-w-0"
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
    );
  };

  return (
    <>
      <h2 className="text-2xl font-bold mb-4">Setup</h2>
      <ul className="space-y-3">
        {renderRow(
          'batch',
          renderCheckbox(allSelected, toggleAll, '全選択', selectAllRef),
          <ActionTooltip text="チェックを付けた項目を、上から順にまとめて実行します。">
            <Button
              type="button"
              className="w-full min-w-0"
              variant="default"
              disabled={loadingCommand !== null}
              onClick={() => {
                void runSelected();
              }}
            >
              {loadingCommand === 'batch' && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              選択した項目をまとめて実行
            </Button>
          </ActionTooltip>
        )}
        {recommendedActions.map(renderAction)}
        {renderRow(
          'obs-init',
          null,
          <ActionTooltip text="準備中です。いまはまだ使えません。">
            <Button type="button" className="w-full min-w-0" variant="default" disabled>
              OBSの初期設定を実施
            </Button>
          </ActionTooltip>
        )}
        {extraActions.map(renderAction)}
        {renderRow(
          'dhcp-all',
          null,
          <ActionTooltip text="いまつながっている回線を、会場のネットワークから自動で設定を受け取るようにします。">
            <Button
              type="button"
              className="w-full min-w-0"
              variant="default"
              disabled={loadingCommand !== null}
              onClick={() => {
                void enableConnectedDhcp();
              }}
            >
              {loadingCommand === 'dhcp-all' && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
              接続中アダプターをDHCPにする
            </Button>
          </ActionTooltip>
        )}
        {adapters.map((adapter) =>
          renderRow(
            adapter.id,
            null,
            <ActionTooltip
              text={
                adapter.dhcp_enabled
                  ? `「${adapter.name}」は、すでに会場のネットワークから自動で設定を受け取る状態です。`
                  : `「${adapter.name}」を、会場のネットワークから自動で設定を受け取るようにします。`
              }
            >
              <Button
                type="button"
                className="w-full min-w-0"
                variant="default"
                disabled={loadingCommand !== null || adapter.dhcp_enabled}
                onClick={() => {
                  void enableAdapterDhcp(adapter);
                }}
              >
                {loadingCommand === adapter.id && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                {adapter.name} をDHCPにする
                {adapter.dhcp_enabled ? ' (設定済み)' : ''}
              </Button>
            </ActionTooltip>
          )
        )}
      </ul>
    </>
  );
};
