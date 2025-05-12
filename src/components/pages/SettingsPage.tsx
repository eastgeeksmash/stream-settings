import React from 'react';

export const SettingsPage: React.FC = () => {
  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Settings</h2>
      <div className="mb-4">
        <label className="flex items-center cursor-pointer">
          <input type="checkbox" className="form-checkbox h-5 w-5 text-blue-600" />
          <span className="ml-2 text-gray-700">起動時に自動的に開始する</span>
        </label>
      </div>
    </div>
  );
}; 