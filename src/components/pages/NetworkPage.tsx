import type React from 'react';
import { Button } from '../ui/button';
import { invoke } from '@tauri-apps/api/core';
import { useEffect, useMemo, useRef, useState } from 'react';
import Loader2 from 'lucide-react/dist/esm/icons/loader-2';
import { ActionTooltip } from '@/components/action-tooltip';
import { invokeAction, showErrorToast, showSuccessToast } from '@/lib/invoke';

const actionSpinner = <Loader2 className="mr-2 h-4 w-4 animate-spin" />;

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
    command: 'make_network_private',
    label: '全ネットワークをプライベート化',
    success: 'ネットワークのプライベート化が完了しました',
    failure: 'ネットワークのプライベート化に失敗しました',
    description: 'このPCのネットワークを、会場で機材とつなぎやすい設定にします。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'optimize_ndi_settings',
    label: 'NDI/OMT向けに最適化',
    success: 'NDI/OMT向け最適化が完了しました',
    failure: 'NDI/OMT向け最適化に失敗しました',
    description: '映像を送る回線の省電力と優先制御を止めます。途切れにくくします。',
    batch: true,
  },
];

export const NetworkPage: React.FC = () => {
  const batchable = useMemo(() => actions.filter((action) => action.batch), []);
  const batchableCommands = useMemo(() => batchable.map((action) => action.command), [batchable]);
  const [loadingCommand, setLoadingCommand] = useState<string | null>(null);
  const [selected, setSelected] = useState<string[]>(() =>
    actions.filter((action) => action.defaultSelected).map((action) => action.command)
  );
  const selectedSet = useMemo(() => new Set(selected), [selected]);
  const [adapters, setAdapters] = useState<NetworkAdapter[]>([]);
  const selectAllRef = useRef<HTMLInputElement>(null);
  const allSelected = batchableCommands.length > 0 && batchableCommands.every((command) => selectedSet.has(command));
  const someSelected = batchableCommands.some((command) => selectedSet.has(command));

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
    const targets = actions.filter((action) => selectedSet.has(action.command));
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
      showSuccessToast('選択したネットワーク項目の実行が完了しました');
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

  return (
    <>
      <h2 className="text-2xl font-bold mb-4">Network</h2>
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
              {loadingCommand === 'batch' && actionSpinner}
              選択した項目をまとめて実行
            </Button>
          </ActionTooltip>
        )}
        {actions.map((action) => {
          const loading = loadingCommand === action.command || loadingCommand === 'batch';
          return renderRow(
            action.command,
            action.batch
              ? renderCheckbox(
                  selectedSet.has(action.command),
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
                {loading && actionSpinner}
                {action.label}
              </Button>
            </ActionTooltip>
          );
        })}
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
              {loadingCommand === 'dhcp-all' && actionSpinner}
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
                {loadingCommand === adapter.id && actionSpinner}
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
