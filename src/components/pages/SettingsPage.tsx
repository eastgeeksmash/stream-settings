import type React from 'react';
import { Switch } from '../ui/switch';
import { Label } from '../ui/label';
import { Button } from '../ui/button';
import { useTheme } from '../theme-provider';
import Moon from "lucide-react/dist/esm/icons/moon";
import Sun from "lucide-react/dist/esm/icons/sun";
import Loader2 from "lucide-react/dist/esm/icons/loader-2";
import { useEffect, useState } from 'react';
import { checkAndApplyUpdate } from '@/lib/updater';

const themeMoon = <Moon className="h-4 w-4" />;
const themeSun = <Sun className="h-4 w-4" />;
const actionSpinner = <Loader2 className="mr-2 h-4 w-4 animate-spin" />;

export const SettingsPage: React.FC = () => {
  const { resolvedTheme, setTheme } = useTheme();
  const [checkingUpdate, setCheckingUpdate] = useState(false);
  const [themeReady, setThemeReady] = useState(false);

  useEffect(() => {
    setThemeReady(true);
  }, []);

  const isDark = themeReady && resolvedTheme === "dark";

  const checkForUpdates = async () => {
    setCheckingUpdate(true);
    try {
      await checkAndApplyUpdate({
        notifyWhenLatest: true,
        notifyOnError: true,
      });
    } finally {
      setCheckingUpdate(false);
    }
  };

  return (
    <>
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
              checked={isDark}
              onCheckedChange={(checked) => setTheme(checked ? "dark" : "light")}
            />
            <Label htmlFor="theme-mode" className="flex items-center gap-2">
              {isDark ? (
                <>
                  {themeMoon}
                  ダークモード
                </>
              ) : (
                <>
                  {themeSun}
                  ライトモード
                </>
              )}
            </Label>
          </div>
        </div>

        <Button
          type="button"
          className="w-full"
          variant="default"
          disabled={checkingUpdate}
          onClick={() => {
            void checkForUpdates();
          }}
        >
          {checkingUpdate && actionSpinner}
          更新を確認
        </Button>
      </div>
    </>
  );
};
