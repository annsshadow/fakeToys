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
})
