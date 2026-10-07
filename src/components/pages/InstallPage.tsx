import type React from 'react';
import { useEffect, useMemo, useRef, useState } from 'react';
import Loader2 from 'lucide-react/dist/esm/icons/loader-2';
import { Button } from '../ui/button';
import { ActionTooltip } from '@/components/action-tooltip';
import { invokeAction, showErrorToast, showSuccessToast } from '@/lib/invoke';

const actionSpinner = <Loader2 className="mr-2 h-4 w-4 animate-spin" />;

type Action = {
  id: string;
  command: string;
  args?: Record<string, unknown>;
  label: string;
  success: string;
  failure: string;
  description: string;
  batch?: boolean;
  defaultSelected?: boolean;
};

const packageActions: Action[] = [
  {
    id: 'pkg-localsend',
    command: 'install_winget_package',
    args: { packageId: 'LocalSend.LocalSend' },
    label: 'LocalSendをインストール',
    success: 'LocalSendのインストールが完了しました',
    failure: 'パッケージのインストールに失敗しました',
    description: '近くのPCへファイルを送る LocalSend を入れます。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'pkg-chrome',
    command: 'install_winget_package',
    args: { packageId: 'Google.Chrome' },
    label: 'Google Chromeをインストール',
    success: 'Google Chromeのインストールが完了しました',
    failure: 'パッケージのインストールに失敗しました',
    description: '配信用のブラウザとして Google Chrome を入れます。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'pkg-coreutils',
    command: 'install_winget_package',
    args: { packageId: 'Microsoft.Coreutils' },
    label: 'Microsoft Coreutilsをインストール',
    success: 'Microsoft Coreutilsのインストールが完了しました',
    failure: 'パッケージのインストールに失敗しました',
    description: 'よく使うコマンドラインの道具を入れます。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'pkg-ffmpeg',
    command: 'install_winget_package',
    args: { packageId: 'Gyan.FFmpeg' },
    label: 'FFmpegをインストール',
    success: 'FFmpegのインストールが完了しました',
    failure: 'パッケージのインストールに失敗しました',
    description: '映像と音声を変換する FFmpeg を入れます。',
    batch: true,
    defaultSelected: true,
  },
];

export const InstallPage: React.FC = () => {
  return (
    <>
      <h2 className="text-2xl font-bold mb-4">Install</h2>
      <ActionSection actions={packageActions} />
    </>
  );
};

const ActionSection: React.FC<{ actions: Action[] }> = ({ actions }) => {
  const batchable = useMemo(() => actions.filter((action) => action.batch), [actions]);
  const batchableIds = useMemo(() => batchable.map((action) => action.id), [batchable]);
  const [loadingId, setLoadingId] = useState<string | null>(null);
  const [selected, setSelected] = useState<string[]>(() =>
    actions.filter((action) => action.defaultSelected).map((action) => action.id)
  );
  const selectedSet = useMemo(() => new Set(selected), [selected]);
  const selectAllRef = useRef<HTMLInputElement>(null);
  const allSelected = batchableIds.length > 0 && batchableIds.every((id) => selectedSet.has(id));
  const someSelected = batchableIds.some((id) => selectedSet.has(id));

  useEffect(() => {
    if (selectAllRef.current) {
      selectAllRef.current.indeterminate = someSelected && !allSelected;
    }
  }, [allSelected, someSelected]);

  const toggleSelected = (id: string, checked: boolean) => {
    setSelected((current) => (checked ? [...current, id] : current.filter((item) => item !== id)));
  };

  const toggleAll = (checked: boolean) => {
    setSelected(checked ? batchableIds : []);
  };

  const run = async (action: Action) => {
    setLoadingId(action.id);
    const result = await invokeAction(action.command, action.failure, action.args);
    if (result.ok) {
      showSuccessToast(action.success);
    } else {
      showErrorToast(result.message);
    }
    setLoadingId(null);
  };

  const runSelected = async () => {
    const targets = actions.filter((action) => selectedSet.has(action.id));
    if (targets.length === 0) {
      showErrorToast('まとめて実行する項目を選択してください。');
      return;
    }

    setLoadingId('batch');
    const failures: string[] = [];
    for (const action of targets) {
      const result = await invokeAction(action.command, action.failure, action.args);
      if (!result.ok) {
        failures.push(result.message);
      }
    }
    if (failures.length === 0) {
      showSuccessToast('選択した項目の実行が完了しました');
    } else {
      showErrorToast(failures.join('\n'));
    }
    setLoadingId(null);
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
    <ul className="space-y-3">
        {renderRow(
          'batch',
          renderCheckbox(allSelected, toggleAll, '全選択', selectAllRef),
          <ActionTooltip text="チェックを付けた項目を、上から順にまとめて実行します。">
            <Button
              type="button"
              className="w-full min-w-0"
              variant="default"
              disabled={loadingId !== null}
              onClick={() => {
                void runSelected();
              }}
            >
              {loadingId === 'batch' && actionSpinner}
              選択した項目をまとめて実行
            </Button>
          </ActionTooltip>
        )}
        {actions.map((action) => {
          const loading = loadingId === action.id || loadingId === 'batch';
          return renderRow(
            action.id,
            action.batch
              ? renderCheckbox(
                  selectedSet.has(action.id),
                  (checked) => {
                    toggleSelected(action.id, checked);
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
    </ul>
  );
};
