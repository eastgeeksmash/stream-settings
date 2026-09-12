import { useEffect } from 'react';
import { checkAndApplyUpdate } from '@/lib/updater';

let startupCheckStarted = false;

export function StartupUpdateCheck() {
  useEffect(() => {
    if (startupCheckStarted) {
      return;
    }

    startupCheckStarted = true;
    void checkAndApplyUpdate({
      notifyWhenLatest: false,
      notifyOnError: false,
    });
  }, []);

  return null;
}
