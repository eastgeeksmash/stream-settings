import type React from 'react';
import { Button } from '../ui/button';

export const CleanupPage: React.FC = () => {
  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Cleanup</h2>
      
      <ul className="space-y-3">
        <li>
          <Button type="button" className="w-full" variant="default">
            Discordログアウト
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default">
            Chromeログアウト
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default">
            ダウンロードディレクトリを削除
          </Button>
        </li>
      </ul>
    </div>
  );
}; 