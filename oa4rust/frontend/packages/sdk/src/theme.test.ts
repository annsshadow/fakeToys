// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { createThemeProvider, themeProvider, useTheme } from './theme'

function installDom() {
  const attrs: Record<string, string> = {}
  const storage = new Map<string, string>()
  vi.stubGlobal('document', {
    documentElement: {
      setAttribute: (k: string, v: string) => (attrs[k] = v),
      getAttribute: (k: string) => attrs[k] ?? null,
    },
  })
  vi.stubGlobal('localStorage', {
    getItem: (k: string) => (storage.has(k) ? storage.get(k)! : null),
    setItem: (k: string, v: string) => storage.set(k, String(v)),
    removeItem: (k: string) => storage.delete(k),
  })
  return { attrs, storage }
}

/** 可控的 prefers-color-scheme 桩：matches=true 表示系统偏好暗色，emit 模拟 OS 切换。 */
function installMatchMedia(initialMatches: boolean) {
  const listeners = new Set<(e: { matches: boolean }) => void>()
  const mql = {
    matches: initialMatches,
    media: '(prefers-color-scheme: dark)',
    addEventListener(_type: string, cb: (e: { matches: boolean }) => void) {
      listeners.add(cb)
    },
    removeEventListener(_type: string, cb: (e: { matches: boolean }) => void) {
      listeners.delete(cb)
    },
    emit(matches: boolean) {
      mql.matches = matches
      for (const cb of [...listeners]) cb({ matches })
    },
    get listenerCount() {
      return listeners.size
    },
  }
  const query = vi.fn(() => mql)
  vi.stubGlobal('matchMedia', query)
  return { mql, query }
}

let attrs: Record<string, string>
let storage: Map<string, string>

beforeEach(() => {
  ;({ attrs, storage } = installDom())
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('theme provider', () => {
  it('init without a stored theme keeps the default dark and applies it to <html>', () => {
    createThemeProvider().init()
    expect(attrs['data-theme']).toBe('dark')
  })

  it('setTheme applies the attribute and persists the choice', () => {
    const provider = createThemeProvider()
    provider.setTheme('light')
    expect(provider.theme.value).toBe('light')
    expect(attrs['data-theme']).toBe('light')
    expect(storage.get('oa4rust_theme')).toBe('light')
  })

  it('toggleTheme flips between dark and light', () => {
    const provider = createThemeProvider()
    provider.setTheme('light')
    provider.toggleTheme()
    expect(provider.theme.value).toBe('dark')
    provider.toggleTheme()
    expect(provider.theme.value).toBe('light')
  })

  it('init restores the persisted theme over the default', () => {
    storage.set('oa4rust_theme', 'light')
    createThemeProvider().init()
    expect(attrs['data-theme']).toBe('light')
  })

  it('useTheme returns the shared singleton provider', () => {
    expect(useTheme()).toBe(themeProvider)
  })

  describe('system preference following', () => {
    it('init without a stored theme follows a dark system preference', () => {
      const { query, mql } = installMatchMedia(true)
      const provider = createThemeProvider()
      provider.init()
      expect(query).toHaveBeenCalledWith('(prefers-color-scheme: dark)')
      expect(provider.theme.value).toBe('dark')
      expect(attrs['data-theme']).toBe('dark')
      expect(mql.listenerCount).toBe(1)
    })

    it('init without a stored theme follows a light system preference', () => {
      const { mql } = installMatchMedia(false)
      const provider = createThemeProvider()
      provider.init()
      expect(provider.theme.value).toBe('light')
      expect(attrs['data-theme']).toBe('light')
      expect(mql.listenerCount).toBe(1)
    })

    it('system theme changes update the applied theme without persisting', () => {
      const { mql } = installMatchMedia(true)
      const provider = createThemeProvider()
      provider.init()
      mql.emit(false)
      expect(provider.theme.value).toBe('light')
      expect(attrs['data-theme']).toBe('light')
      expect(storage.has('oa4rust_theme')).toBe(false)
      mql.emit(true)
      expect(provider.theme.value).toBe('dark')
      expect(storage.has('oa4rust_theme')).toBe(false)
    })

    it('explicit setTheme stops following later system changes', () => {
      const { mql } = installMatchMedia(true)
      const provider = createThemeProvider()
      provider.init()
      provider.setTheme('dark')
      expect(storage.get('oa4rust_theme')).toBe('dark')
      expect(mql.listenerCount).toBe(0)
      mql.emit(false)
      expect(provider.theme.value).toBe('dark')
      expect(attrs['data-theme']).toBe('dark')
    })

    it('explicit toggleTheme stops following later system changes', () => {
      const { mql } = installMatchMedia(false)
      const provider = createThemeProvider()
      provider.init()
      provider.toggleTheme()
      expect(provider.theme.value).toBe('dark')
      expect(storage.get('oa4rust_theme')).toBe('dark')
      expect(mql.listenerCount).toBe(0)
      mql.emit(false)
      expect(provider.theme.value).toBe('dark')
    })

    it('a stored preference wins over the system and installs no listener', () => {
      const { mql } = installMatchMedia(true)
      storage.set('oa4rust_theme', 'light')
      const provider = createThemeProvider()
      provider.init()
      expect(provider.theme.value).toBe('light')
      expect(mql.listenerCount).toBe(0)
      mql.emit(false)
      expect(provider.theme.value).toBe('light')
    })

    it('falls back to dark safely when matchMedia is unavailable (SSR/node)', () => {
      expect(() => createThemeProvider().init()).not.toThrow()
      expect(attrs['data-theme']).toBe('dark')
    })
  })
})
