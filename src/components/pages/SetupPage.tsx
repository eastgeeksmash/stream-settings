import type React from 'react';
import { Button } from '../ui/button';
import { useEffect, useMemo, useRef, useState } from 'react';
import Loader2 from 'lucide-react/dist/esm/icons/loader-2';
import { ActionTooltip } from '@/components/action-tooltip';
import { invokeAction, showErrorToast, showSuccessToast } from '@/lib/invoke';

const actionSpinner = <Loader2 className="mr-2 h-4 w-4 animate-spin" />;

type Action = {
  id?: string;
  command: string;
  args?: Record<string, unknown>;
  label: string;
  success: string;
  failure: string;
  description: string;
  batch?: boolean;
  defaultSelected?: boolean;
};

const actionKey = (action: Action) => action.id ?? action.command;

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
    command: 'disable_call_ducking',
    label: '通話時の音量下げを無効化',
    success: '通話時の音量下げ無効化が完了しました',
    failure: '通話時の音量下げ無効化に失敗しました',
    description: '通話を始めたときに、ほかの音が小さくなるのを止めます。配信の音量を保ちます。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'set_no_sounds_scheme',
    label: 'Windowsサウンドをサウンドなしにする',
    success: 'Windowsサウンドをサウンドなしに変更しました',
    failure: 'Windowsサウンドの変更に失敗しました',
    description: 'Windowsの効果音を止めます。配信に通知音が入りにくくなります。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_telemetry',
    label: 'テレメトリと診断データを無効化',
    success: 'テレメトリと診断データの無効化が完了しました',
    failure: 'テレメトリの無効化に失敗しました',
    description: '利用状況の送信と、広告向けの診断データを止めます。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_suggestions',
    label: 'ヒント・提案・広告を無効化',
    success: 'ヒント・提案・広告の無効化が完了しました',
    failure: 'ヒントと提案の無効化に失敗しました',
    description: 'スタートや設定に出る提案と広告を止めます。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_copilot',
    label: 'Copilotを無効化',
    success: 'Copilotの無効化が完了しました',
    failure: 'Copilotの無効化に失敗しました',
    description: 'Copilot のボタンと機能を止めます。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_recall_and_click_to_do',
    label: 'RecallとClick to Doを無効化',
    success: 'RecallとClick to Doの無効化が完了しました',
    failure: 'RecallとClick to Doの無効化に失敗しました',
    description: '画面の記録と、選択内容のAI分析を止めます。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_widgets',
    label: 'ウィジェットを無効化',
    success: 'ウィジェットの無効化が完了しました',
    failure: 'ウィジェットの無効化に失敗しました',
    description: 'タスクバーとロック画面のウィジェットを、関連アプリを削除して止めます。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_bing_search',
    label: 'Bing検索を無効化',
    success: 'Bing検索の無効化が完了しました',
    failure: 'Bing検索の無効化に失敗しました',
    description: 'スタートメニューのウェブ検索とCortanaを外します。',
    batch: true,
    defaultSelected: true,
  },
  {
    command: 'disable_fast_startup',
    label: '高速スタートアップを無効化',
    success: '高速スタートアップの無効化が完了しました',
    failure: '高速スタートアップの無効化に失敗しました',
    description: 'シャットダウンを完全な終了にします。次回起動が安定しやすくなります。',
    batch: true,
    defaultSelected: true,
  },
  {
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
    id: 'apps-hp',
    command: 'remove_debloat_group',
    args: { group: 'hp' },
    label: 'HPのプリインストールアプリを削除',
    success: 'HPのプリインストールアプリの削除が完了しました',
    failure: '不要アプリの削除に失敗しました',
    description: 'HP製PCに入っている追加アプリを削除します。HP以外では何も消えません。',
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
  {
    command: 'optimize_nvidia_settings',
    label: 'NVIDIA向けに最適化',
    success: 'NVIDIA向け最適化が完了しました',
    failure: 'NVIDIA向け最適化に失敗しました',
    description: 'NVIDIAの電源を最大性能にし、画質設定をアプリ任せにします。モニターの色形式も配信向けに整えます。NVIDIA搭載PCのみ使えます。',
    batch: true,
  },
];

export const SetupPage: React.FC = () => {
  const batchable = useMemo(() => actions.filter((action) => action.batch), []);
  const batchableKeys = useMemo(() => batchable.map(actionKey), [batchable]);
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
    actions.filter((action) => action.defaultSelected).map(actionKey)
  );
  const selectedSet = useMemo(() => new Set(selected), [selected]);
  const selectAllRef = useRef<HTMLInputElement>(null);
  const allSelected = batchableKeys.length > 0 && batchableKeys.every((key) => selectedSet.has(key));
  const someSelected = batchableKeys.some((key) => selectedSet.has(key));

  useEffect(() => {
    if (selectAllRef.current) {
      selectAllRef.current.indeterminate = someSelected && !allSelected;
    }
  }, [allSelected, someSelected]);

  const toggleSelected = (key: string, checked: boolean) => {
    setSelected((current) =>
      checked ? [...current, key] : current.filter((item) => item !== key)
    );
  };

  const toggleAll = (checked: boolean) => {
    setSelected(checked ? batchableKeys : []);
  };

  const run = async (action: Action) => {
    setLoadingCommand(actionKey(action));
    const result = await invokeAction(action.command, action.failure, action.args);
    if (result.ok) {
      showSuccessToast(action.success);
    } else {
      showErrorToast(result.message);
    }
    setLoadingCommand(null);
  };

  const runSelected = async () => {
    const targets = actions.filter((action) => selectedSet.has(actionKey(action)));
    if (targets.length === 0) {
      showErrorToast('まとめて実行する項目を選択してください。');
      return;
    }

    setLoadingCommand('batch');
    const failures: string[] = [];
    for (const action of targets) {
      const result = await invokeAction(action.command, action.failure, action.args);
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
    const key = actionKey(action);
    const loading = loadingCommand === key || loadingCommand === 'batch';
    return renderRow(
      key,
      action.batch
        ? renderCheckbox(
            selectedSet.has(key),
            (checked) => {
              toggleSelected(key, checked);
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
              {loadingCommand === 'batch' && actionSpinner}
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
      </ul>
    </>
  );
};
