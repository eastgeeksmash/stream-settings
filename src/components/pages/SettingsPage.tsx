import type React from 'react';
import { Switch } from '../ui/switch';
import { Label } from '../ui/label';
import { useTheme } from '../theme-provider';
import { Moon, Sun } from "lucide-react";

export const SettingsPage: React.FC = () => {
  const { theme, setTheme } = useTheme();

  return (
    <div className="dark:bg-slate-800 p-6 rounded-lg">
      <h2 className="text-2xl font-bold mb-4">Settings</h2>
      
      <div className="space-y-6">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <Switch id="airplane-mode" disabled />
            <Label htmlFor="airplane-mode">起動時に自動的に開始する</Label>
          </div>
        </div>

        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-2">
            <Switch
              id="theme-mode"
              checked={theme === "dark"}
              onCheckedChange={(checked) => setTheme(checked ? "dark" : "light")}
            />
            <Label htmlFor="theme-mode" className="flex items-center gap-2">
              {theme === "dark" ? (
                <>
                  <Moon className="h-4 w-4" />
                  ダークモード
                </>
              ) : (
                <>
                  <Sun className="h-4 w-4" />
                  ライトモード
                </>
              )}
            </Label>
          </div>
        </div>
      </div>
    </div>
  );
}; 