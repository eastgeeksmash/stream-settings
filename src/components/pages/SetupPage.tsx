import type React from 'react';
import { Button } from '../ui/button';
import { toast } from "sonner"
import { invoke } from '@tauri-apps/api/core';
import { useState } from 'react';


export const SetupPage: React.FC = () => {
  const [isMakeNetworkPrivateLoading, setIsMakeNetworkPrivateLoading] = useState(false);
  const [isChangePowerSettingsLoading, setIsChangePowerSettingsLoading] = useState(false);

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

  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Setup</h2>
      <ul className="space-y-3">
        <li>
          <Button type="button" className="w-full" variant="default" disabled={isMakeNetworkPrivateLoading} onClick={() => {
            makeNetworkPrivate();
          }}>
            ネットワークをプライベート化
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default" disabled>
            OBSの初期設定実施
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
            電源設定の変更
          </Button>
        </li>
      </ul>
    </div>
  );
}; 