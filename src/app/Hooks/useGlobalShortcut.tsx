'use client'

import { useEffect, useState } from 'react';
import { isRegistered, register, unregisterAll } from '@tauri-apps/api/globalShortcut';
import type { WebviewWindow } from "@tauri-apps/api/window"

const SHORTCUT = 'Alt+Space'

export default function useGlobalShortcut() {
  const [appWindow, setAppWindow] = useState<WebviewWindow>()

  // Import appWindow and save it inside the state for later usage
  async function setupAppWindow() {
    const appWindow = (await import('@tauri-apps/api/window')).appWindow
    setAppWindow(appWindow)
  }

  useEffect(() => {
    setupAppWindow()
  }, [])

  useEffect(() => {
    if (!appWindow) return;

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
  }, [appWindow])
} 