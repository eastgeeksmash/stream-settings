import type React from 'react';

export const AboutPage: React.FC = () => {
  return (
    <>
      <h2 className="text-2xl font-bold mb-4">About</h2>
      <div className="mt-6 border-t pt-4">
        <h3 className="text-xl font-semibold mb-2">Copyright</h3>
        <p className="text-slate-600">
          このソフトウェアは <a href="https://github.com/eastgeeksmash/stream-settings" className="text-blue-600 hover:underline" target="_blank" rel="noopener noreferrer">EastGeekSmash</a> によって開発・保守されているフリーソフトウェアです。
        </p>
        <p className="text-slate-600 mt-2">
          MIT Licenseの下で提供されており、無償で使用、複製、変更、配布することができます。詳細については<a href="https://github.com/eastgeeksmash/stream-settings/blob/release/LICENSE" className="text-blue-600 hover:underline" target="_blank" rel="noopener noreferrer">ライセンス</a>をご確認ください。
        </p>
        <p className="text-slate-600 mt-2 text-sm">
          Copyright (c) 2025 EastGeekSmash
        </p>
      </div>
    </>
  );
}; 