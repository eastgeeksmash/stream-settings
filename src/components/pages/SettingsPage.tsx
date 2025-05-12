import type React from 'react';
import { Switch } from '../ui/switch';
import { Label } from '../ui/label';

export const SettingsPage: React.FC = () => {
  return (
    <div className="bg-white p-6 rounded-lg shadow-md">
      <h2 className="text-2xl font-bold mb-4">Settings</h2>
      <div className="flex items-center space-x-2">
        <Switch id="airplane-mode" />
        <Label htmlFor="airplane-mode">起動時に自動的に開始する</Label>
      </div>

    </div>
  );
}; 