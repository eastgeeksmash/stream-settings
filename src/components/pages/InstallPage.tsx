import type React from 'react';
import { useEffect, useMemo, useRef, useState } from 'react';
import { Loader2 } from 'lucide-react';
import { Button } from '../ui/button';
import { ActionTooltip } from '@/components/action-tooltip';
import { invokeAction, showErrorToast, showSuccessToast } from '@/lib/invoke';

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

const debloatActions: Action[] = [
  {
    id: 'disable-telemetry',
    command: 'disable_telemetry',
    label: 'テレメトリと診断データを無効化',
    success: 'テレメトリと診断データの無効化が完了しました',
    failure: 'テレメトリの無効化に失敗しました',
    description: '利用状況の送信と、広告向けの診断データを止めます。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'disable-suggestions',
    command: 'disable_suggestions',
    label: 'ヒント・提案・広告を無効化',
    success: 'ヒント・提案・広告の無効化が完了しました',
    failure: 'ヒントと提案の無効化に失敗しました',
    description: 'スタートや設定に出る提案と広告を止めます。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'disable-copilot',
    command: 'disable_copilot',
    label: 'Copilotを無効化',
    success: 'Copilotの無効化が完了しました',
    failure: 'Copilotの無効化に失敗しました',
    description: 'Copilot のボタンと機能を止めます。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'disable-recall',
    command: 'disable_recall_and_click_to_do',
    label: 'RecallとClick to Doを無効化',
    success: 'RecallとClick to Doの無効化が完了しました',
    failure: 'RecallとClick to Doの無効化に失敗しました',
    description: '画面の記録と、選択内容のAI分析を止めます。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'disable-widgets',
    command: 'disable_widgets',
    label: 'ウィジェットを無効化',
    success: 'ウィジェットの無効化が完了しました',
    failure: 'ウィジェットの無効化に失敗しました',
    description: 'タスクバーのウィジェットと、その掲示板を止めます。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'disable-bing',
    command: 'disable_bing_search',
    label: 'Bing検索を無効化',
    success: 'Bing検索の無効化が完了しました',
    failure: 'Bing検索の無効化に失敗しました',
    description: 'スタートの検索から、ウェブ検索とCortanaを外します。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'disable-fast-startup',
    command: 'disable_fast_startup',
    label: '高速スタートアップを無効化',
    success: '高速スタートアップの無効化が完了しました',
    failure: '高速スタートアップの無効化に失敗しました',
    description: 'シャットダウンを完全な終了にします。次回起動が安定しやすくなります。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'disable-storage-sense',
    command: 'disable_storage_sense',
    label: 'Storage Senseを無効化',
    success: 'Storage Senseの無効化が完了しました',
    failure: 'Storage Senseの無効化に失敗しました',
    description: 'ダウンロードフォルダなどを自動で消さないようにします。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'apps-default',
    command: 'remove_debloat_group',
    args: { group: 'default' },
    label: '標準の不要アプリを削除',
    success: '標準の不要アプリの削除が完了しました',
    failure: '不要アプリの削除に失敗しました',
    description: 'プリインストールの不要アプリを削除します。入っていないものはそのままにします。反映にはサインアウトが必要な場合があります。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'apps-gaming',
    command: 'remove_debloat_group',
    args: { group: 'gaming' },
    label: 'XboxとGame Barを削除',
    success: 'XboxとGame Barの削除が完了しました',
    failure: '不要アプリの削除に失敗しました',
    description: 'Xbox と Game Bar を削除します。配信中のオーバーレイが出にくくなります。',
    batch: true,
    defaultSelected: true,
  },
  {
    id: 'apps-hp',
    command: 'remove_debloat_group',
    args: { group: 'hp' },
    label: 'HPのプリインストールアプリを削除',
    success: 'HPのプリインストールアプリの削除が完了しました',
    failure: '不要アプリの削除に失敗しました',
    description: 'HP製PCに入っている追加アプリを削除します。HP以外では何も消えません。',
    batch: true,
  },
];

export const InstallPage: React.FC = () => {
  return (
    <>
      <h2 className="text-2xl font-bold mb-4">Install</h2>
      <ActionSection title="パッケージのインストール" actions={packageActions} />
      <ActionSection title="不要機能の削除" actions={debloatActions} />
    </>
  );
};

const ActionSection: React.FC<{ title: string; actions: Action[] }> = ({ title, actions }) => {
  const batchable = useMemo(() => actions.filter((action) => action.batch), [actions]);
  const batchableIds = useMemo(() => batchable.map((action) => action.id), [batchable]);
  const [loadingId, setLoadingId] = useState<string | null>(null);
  const [selected, setSelected] = useState<string[]>(() =>
    actions.filter((action) => action.defaultSelected).map((action) => action.id)
  );
  const selectAllRef = useRef<HTMLInputElement>(null);
  const allSelected = batchableIds.length > 0 && batchableIds.every((id) => selected.includes(id));
  const someSelected = batchableIds.some((id) => selected.includes(id));

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
    const targets = actions.filter((action) => selected.includes(action.id));
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
    <section className="mb-8">
      <h3 className="text-xl font-semibold mb-4">{title}</h3>
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
              {loadingId === 'batch' && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
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
                  selected.includes(action.id),
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
                {loading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
                {action.label}
              </Button>
            </ActionTooltip>
          );
        })}
      </ul>
    </section>
  );
};
