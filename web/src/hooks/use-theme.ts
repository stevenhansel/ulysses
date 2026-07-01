import { useCallback, useSyncExternalStore } from 'react'

type Theme = 'light' | 'dark'

function getSystemTheme(): Theme {
  if (typeof window === 'undefined') return 'light'
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
}

function getStoredTheme(): Theme | null {
  try {
    const stored = localStorage.getItem('ulysses-theme')
    if (stored === 'light' || stored === 'dark') return stored
    return null
  } catch {
    return null
  }
}

function getTheme(): Theme {
  return getStoredTheme() ?? getSystemTheme()
}

function applyTheme(theme: Theme) {
  const root = document.documentElement
  root.classList.toggle('dark', theme === 'dark')
}

const listeners = new Set<() => void>()

function subscribe(callback: () => void) {
  listeners.add(callback)
  return () => listeners.delete(callback)
}

function emitChange() {
  for (const listener of listeners) {
    listener()
  }
}

// Initialize theme on load
applyTheme(getTheme())

export function useTheme() {
  const theme = useSyncExternalStore(subscribe, getTheme)

  const setTheme = useCallback((newTheme: Theme) => {
    localStorage.setItem('ulysses-theme', newTheme)
    applyTheme(newTheme)
    emitChange()
  }, [])

  const toggleTheme = useCallback(() => {
    setTheme(theme === 'light' ? 'dark' : 'light')
  }, [theme, setTheme])

  return { theme, setTheme, toggleTheme }
}
