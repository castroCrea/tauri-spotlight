'use client'

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import { isRegistered, register, unregisterAll } from '@tauri-apps/api/globalShortcut';
import { appWindow } from '@tauri-apps/api/window';

const SHORTCUT = 'Alt+Space'

export default function Greet() {
  const [greeting, setGreeting] = useState('');

  useEffect(() => {
    (async () => {
      const isOn = await isRegistered(SHORTCUT);
      console.log({ isOn })
      if (isOn) return;

      await register(SHORTCUT, async () => {
        const isVisible = await appWindow.isVisible()
        isVisible ? appWindow.hide() : appWindow.show();
        console.log('Shortcut triggered', { isVisible });
      });
    })()
    invoke<string>('greet', { name: 'Next.js' })
      .then(result => setGreeting(result))
      .catch(console.error)

    return () => {
      unregisterAll()
    }
  }, [])

  // Necessary because we will have to use Greet as a component later.
  return <div>{greeting}</div>;
}