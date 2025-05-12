import React from 'react';

export const CleanupPage: React.FC = () => {
  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Cleanup</h2>
      
      <ul className="space-y-3">
        <li>
          <button className="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded transition-colors">
            Discordログアウト
          </button>
        </li>
        <li>
          <button className="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded transition-colors">
            Chromeログアウト
          </button>
        </li>
        <li>
          <button className="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded transition-colors">
            ダウンロードディレクトリを削除
          </button>
        </li>
      </ul>
    </div>
  );
}; 