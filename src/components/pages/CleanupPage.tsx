import type React from 'react';
import { Button } from '../ui/button';
import { invoke } from '@tauri-apps/api/core';

export const CleanupPage: React.FC = () => {
  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Cleanup</h2>
      
      <ul className="space-y-3">
        <li>
          <Button type="button" className="w-full" variant="default" onClick={() => {
            invoke('logout_discord')
              .then(() => {
                console.log('Discordのログアウトが完了しました');
              })
              .catch((error: Error) => {
                console.error('Discordのログアウトに失敗しました:', error);
              });
          }}>
            Discordログアウト
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default" onClick={() => {
            invoke('logout_chrome')
              .then(() => {
                console.log('Chromeのログアウトが完了しました');
              })
              .catch((error: Error) => {
                console.error('Chromeのログアウトに失敗しました:', error);
              });
          }}>
            Chromeログアウト
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default" onClick={() => {
            invoke('delete_download_directory')
              .then(() => {
                console.log('ダウンロードディレクトリの削除が完了しました');
              })
              .catch((error: Error) => {
                console.error('ダウンロードディレクトリの削除に失敗しました:', error);
              });
          }}>
            ダウンロードディレクトリを削除
          </Button>
        </li>
      </ul>
    </div>
  );
}; 