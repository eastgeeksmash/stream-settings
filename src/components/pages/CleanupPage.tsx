import type React from 'react';
import { Button } from '../ui/button';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'sonner';

export const CleanupPage: React.FC = () => {
  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Cleanup</h2>
      
      <ul className="space-y-3">
        <li>
          <Button type="button" className="w-full" variant="default" onClick={(event) => {
            const button = event.currentTarget as HTMLButtonElement;
            button.disabled = true;

            invoke('logout_discord')
              .then(() => {
                toast.success('Discordのログアウトが完了しました');
              })
              .catch((error: Error) => {
                console.error(error);
                toast.error(`Discordのログアウトに失敗しました: ${error.message}`);
              })
              .finally(() => {
                button.disabled = false;
              });
          }}>
            Discordログアウト
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default" onClick={(event) => {
            const button = event.currentTarget as HTMLButtonElement;
            button.disabled = true;

            invoke('logout_chrome')
              .then(() => {
                toast.success('Chromeのログアウトが完了しました');
              })
              .catch((error: Error) => {
                console.error(error);
                toast.error(`Chromeのログアウトに失敗しました: ${error.message}`);
              })
              .finally(() => {
                button.disabled = false;
              });
          }}>
            Chromeログアウト
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default" onClick={(event) => {
            const button = event.currentTarget as HTMLButtonElement;
            button.disabled = true;

            invoke('delete_download_directory')
              .then(() => {
                toast.success('ダウンロードディレクトリの削除が完了しました');
              })
              .catch((error: Error) => {
                console.error(error);
                toast.error(`ダウンロードディレクトリの削除に失敗しました: ${error.message}`);
              })
              .finally(() => {
                button.disabled = false;
              });
          }}>
            ダウンロードディレクトリを削除
          </Button>
        </li>
      </ul>
    </div>
  );
}; 