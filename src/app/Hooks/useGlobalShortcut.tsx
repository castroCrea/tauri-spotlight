'use client'

import { useEffect } from 'react';
import { isRegistered, register, unregisterAll } from '@tauri-apps/api/globalShortcut';
import { appWindow } from '@tauri-apps/api/window';

const SHORTCUT = 'Alt+Space'

export default function useGlobalShortcut() {
  useEffect(() => {
    (async () => {
      const isOn = await isRegistered(SHORTCUT);
      console.log({ isOn })
      if (isOn) return;

      await register(SHORTCUT, async () => {
        const isVisible = await appWindow.isVisible()
        isVisible ? appWindow.hide() : appWindow.show().then(() => {
          appWindow.setFocus()
        });
        console.log('Shortcut triggered', { isVisible });
      });
    })()
    return () => {
      unregisterAll()
    }
  }, [])
}