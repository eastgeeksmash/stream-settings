import React from 'react';

export const SetupPage: React.FC = () => {
  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Setup</h2>
      <ul className="space-y-3">
        <li>
          <button className="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded transition-colors">
            ネットワークをプライベート化
          </button>
        </li>
        <li>
          <button className="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded transition-colors">
            OBSの初期設定実施
          </button>
        </li>
        <li>
          <button 
            className="w-full bg-blue-500 hover:bg-blue-600 text-white py-2 px-4 rounded transition-colors"
            title="高パフォーマンスモードに変更、スリープモード無効化、電源ボタンのロックを行います"
          >
            電源設定の変更
          </button>
        </li>
      </ul>
    </div>
  );
}; 