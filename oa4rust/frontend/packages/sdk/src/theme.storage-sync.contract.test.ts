// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R229：主题只写 localStorage、不监听 storage 事件时，多标签页会互相失步：
 *   - A 页切到 dark 后，B 页内存仍是 light，界面颜色不跟着变；
 *   - 更严重的是 B 页随后再切一次，会把内存里的旧值 light 写回 localStorage，
 *     直接覆盖 A 页刚选的 dark。
 * 本契约钉住「别的标签页改主题 → 本页状态与 <html data-theme> 同步」以及
 * 「同步之后本页再切换不会用旧值回写」两条行为。
 */
// @vitest-environment node
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

type Listener = (event: { key: string | null; newValue: string | null }) => void

function installDom() {
  const attrs: Record<string, string> = {}
  const storage = new Map<string, string>()
  const listeners = new Map<string, Listener[]>()
  vi.stubGlobal('document', {
    // Vue runtime-dom 在导入期读取 document.createElement，桩掉避免模块加载报错。
    createElement: () => ({ style: {}, setAttribute: () => {} }),
    documentElement: {
      setAttribute: (k: string, v: string) => {
        attrs[k] = v
      },
      getAttribute: (k: string) => attrs[k] ?? null,
    },
  })
  vi.stubGlobal('localStorage', {
    getItem: (k: string) => (storage.has(k) ? storage.get(k)! : null),
    setItem: (k: string, v: string) => storage.set(k, String(v)),
    removeItem: (k: string) => storage.delete(k),
  })
  vi.stubGlobal('addEventListener', (type: string, cb: Listener) => {
    const list = listeners.get(type) ?? []
    list.push(cb)
    listeners.set(type, list)
  })
  return {
    attrs,
    storage,
    /** 模拟另一个标签页改了 localStorage，浏览器向本页派发 storage 事件。 */
    fire: (key: string, newValue: string | null) => {
      for (const cb of listeners.get('storage') ?? []) cb({ key, newValue })
    },
  }
}

beforeEach(() => {
  vi.resetModules()
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('主题跨标签页同步（R229 防回归）', () => {
  it('另一个标签页改主题后，本页共享状态与 <html data-theme> 一起更新', async () => {
    const { attrs, storage, fire } = installDom()
    const { createThemeProvider } = await import('./theme')
    const provider = createThemeProvider()
    storage.set('oa4rust_theme', 'light')
    provider.init()
    expect(provider.theme.value).toBe('light')

    fire('oa4rust_theme', 'dark')

    expect(provider.theme.value, '另一个标签页把主题改成 dark 后，本页仍停留在 light').toBe('dark')
    expect(attrs['data-theme'], '另一个标签页改主题后，本页 <html data-theme> 没有跟着更新').toBe('dark')
  })

  it('跨标签页同步后本页再切换，不会把同步前的旧主题写回 localStorage', async () => {
    const { storage, fire } = installDom()
    const { createThemeProvider, useTheme } = await import('./theme')
    storage.set('oa4rust_theme', 'light')
    createThemeProvider().init()
    expect(useTheme().theme.value).toBe('light')

    fire('oa4rust_theme', 'dark')
    expect(useTheme().theme.value, '没有同步另一个标签页的选择，本页仍是旧的 light').toBe('dark')

    useTheme().toggleTheme()
    expect(storage.get('oa4rust_theme'), '本页用同步前的旧主题回写，覆盖了另一个标签页的选择').toBe('light')
  })
})
