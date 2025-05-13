import type React from 'react';
import { Button } from '../ui/button';
import { toast } from "sonner"
import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';
import { Loader2 } from 'lucide-react';


export const SetupPage: React.FC = () => {
  const [isMakeNetworkPrivateLoading, setIsMakeNetworkPrivateLoading] = useState(false);
  const [isChangePowerSettingsLoading, setIsChangePowerSettingsLoading] = useState(false);
  const [isDisableWindowsNotificationsLoading, setIsDisableWindowsNotificationsLoading] = useState(false);


  const makeNetworkPrivate = () => {
    setIsMakeNetworkPrivateLoading(true);
    invoke('make_network_private')
      .then(() => {
        toast.success("ネットワークのプライベート化が完了しました");
      })
      .catch((error: Error) => {
        console.error(error);
        toast.error(`ネットワークのプライベート化に失敗しました: ${error.message}`);
      })
      .finally(() => {
        setIsMakeNetworkPrivateLoading(false);
      });
  }

  const changePowerSettings = () => {
    setIsChangePowerSettingsLoading(true);
    invoke('change_power_settings')
      .then(() => {
        toast.success("電源設定の変更が完了しました");
      })
      .catch((error: Error) => {
        console.error(error);
        toast.error(`電源設定の変更に失敗しました: ${error.message}`);
      })
      .finally(() => {
        setIsChangePowerSettingsLoading(false);
      });
  }

  const disableWindowsNotifications = () => {
    setIsDisableWindowsNotificationsLoading(true);
    invoke('disable_windows_notifications')
      .then(() => {
        toast.success("通知センターの無効化が完了しました");
      })
      .catch((error: Error) => {
        console.error(error);
        toast.error(`通知センターの無効化に失敗しました: ${error.message}`);
      })
      .finally(() => {
        setIsDisableWindowsNotificationsLoading(false);
      });
  }

  return (
    <>
      <h2 className="text-2xl font-bold mb-4">Setup</h2>
      <ul className="space-y-3">
        <li>
          <Button type="button" className="w-full" variant="default" disabled={isMakeNetworkPrivateLoading} onClick={() => {
            makeNetworkPrivate();
          }}>
            {isMakeNetworkPrivateLoading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            全ネットワークをプライベート化
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default" disabled>
            OBSの初期設定を実施
          </Button>
        </li>
        <li>
          <Button 
            type="button"
            className="w-full"
            variant="default"
            title="高パフォーマンスモードに変更、スリープモード無効化、電源ボタンのロックを行います"
            disabled={isChangePowerSettingsLoading}
            onClick={() => {
              changePowerSettings();
            }}
          >
            {isChangePowerSettingsLoading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            電源設定を変更(高パフォーマンス)
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default" disabled={isDisableWindowsNotificationsLoading} onClick={() => {
            disableWindowsNotifications();
          }}>
            {isDisableWindowsNotificationsLoading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            通知を無効化
          </Button>
        </li>
      </ul>
    </>
  );
}; 