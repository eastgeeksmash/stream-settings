import type React from 'react';
import { Button } from '../ui/button';

export const SetupPage: React.FC = () => {
  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Setup</h2>
      <ul className="space-y-3">
        <li>
          <Button type="button" className="w-full" variant="default">
            ネットワークをプライベート化
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default">
            OBSの初期設定実施
          </Button>
        </li>
        <li>
          <Button 
            type="button"
            className="w-full"
            variant="default"
            title="高パフォーマンスモードに変更、スリープモード無効化、電源ボタンのロックを行います"
          >
            電源設定の変更
          </Button>
        </li>
      </ul>
    </div>
  );
}; 