import type React from 'react';
import { Button } from '../ui/button';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'sonner';
import { useState } from 'react';
import { Loader2 } from 'lucide-react';

export const CleanupPage: React.FC = () => {
  const [isDiscordLogoutLoading, setIsDiscordLogoutLoading] = useState(false);
  const [isChromeLogoutLoading, setIsChromeLogoutLoading] = useState(false);
  const [isDownloadDirectoryDeleteLoading, setIsDownloadDirectoryDeleteLoading] = useState(false);

  const logoutDiscord = () => {
    setIsDiscordLogoutLoading(true);
    invoke('logout_discord')
      .then(() => {
        toast.success('Discordのログアウトが完了しました');
      })
      .catch((error: Error) => {
        console.error(error);
        toast.error(`Discordのログアウトに失敗しました: ${error.message}`);
      })
      .finally(() => {
        setIsDiscordLogoutLoading(false);
      });
  }

  const logoutChrome = () => {
    setIsChromeLogoutLoading(true);
    invoke('logout_chrome')
      .then(() => {
        toast.success('Chromeのログアウトが完了しました');
      })
      .catch((error: Error) => {
        console.error(error);
        toast.error(`Chromeのログアウトに失敗しました: ${error.message}`);
      })
      .finally(() => {
        setIsChromeLogoutLoading(false);
      });
  }

  const deleteDownloadDirectory = () => {
    setIsDownloadDirectoryDeleteLoading(true);
    invoke('delete_download_directory')
      .then(() => {
        toast.success('ダウンロードディレクトリの削除が完了しました');
      })
      .catch((error: Error) => {
        console.error(error);
        toast.error(`ダウンロードディレクトリの削除に失敗しました: ${error.message}`);
      })
      .finally(() => {
        setIsDownloadDirectoryDeleteLoading(false);
      });
  }

  return (
    <>
      <h2 className="text-2xl font-bold mb-4">Cleanup</h2>
      
      <ul className="space-y-3">
        <li>
          <Button type="button" className="w-full" variant="default" disabled={isDiscordLogoutLoading} onClick={(event) => {
            logoutDiscord();
          }}>
            {isDiscordLogoutLoading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            Discordログアウト
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default" disabled={isChromeLogoutLoading} onClick={(event) => {
            logoutChrome();
          }}>
            {isChromeLogoutLoading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            Chromeログアウト
          </Button>
        </li>
        <li>
          <Button type="button" className="w-full" variant="default" disabled={isDownloadDirectoryDeleteLoading} onClick={(event) => {
            deleteDownloadDirectory();
          }}>
            {isDownloadDirectoryDeleteLoading && <Loader2 className="mr-2 h-4 w-4 animate-spin" />}
            ダウンロードディレクトリを削除
          </Button>
        </li>
      </ul>
    </>
  );
}; 