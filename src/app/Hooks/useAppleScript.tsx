'use client'

import { useEffect, useState } from 'react';
import type Tauri from '@tauri-apps/api/tauri';
import { script } from './script.jxa'

export default function useAppleScript() {
  const [tauri, setTauri] = useState<typeof Tauri>()

  // Import tauri and save it inside the state for later usage
  async function setupTauri() {
    const tauri = (await import('@tauri-apps/api/tauri'))
    setTauri(tauri)
  }

  useEffect(() => {
    setupTauri()
  }, [])

  return async () => {
    console.log({ tauri })
    if (!tauri) return;
    const result = await tauri.invoke('execute_jxa_script', {
      script: `JSON.stringify((${script.toString()})())`
    }) as string;
    const div = document.createElement('div');
    div.append(result)
    document.body.appendChild(div)
    console.log(JSON.parse(result))
  }
}
