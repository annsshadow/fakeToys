// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R234：PWA / 移动浏览器外壳的 theme-color 在所有主题下都必须跟随应用主题。
 *
 * 缺陷证据：应用支持 dark/light 切换（theme.ts 与 dark.css 已落地），但
 * index.html 的 <meta name="theme-color"> 与 manifest.json 的 theme_color
 * 都写死为深色 #060913。安装到桌面或在 Android Chrome 任务切换器里，用户切到
 * light 主题后系统外壳仍是深色，出现刺眼的深色标题栏/任务卡——真实的现代一致性
 * 缺陷。静态 meta 无法跟随，必须在主题落定 / 切换 / 跨标签页同步时同步更新。
 * 本契约钉住两件事：静态外壳声明与默认主题一致；运行时同步覆盖全部主题入口。
 */
// @vitest-environment node
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../..')
const html = readFileSync(path.join(frontendRoot, 'apps/desktop/index.html'), 'utf8')
const manifest = JSON.parse(readFileSync(path.join(frontendRoot, 'apps/desktop/manifest.json'), 'utf8')) as {
  theme_color?: string
  background_color?: string
}
const themeSource = readFileSync(path.join(frontendRoot, 'packages/sdk/src/theme.ts'), 'utf8')

type ThemeMode = 'dark' | 'light'

const DARK = '#060913'
const LIGHT = '#f0f4f8' // 与 dark.css [data-theme="light"] 的 --bg-base 一致

/** 与 theme.test.ts 同款 DOM 桩，另补 meta[name=theme-color] 与 head.querySelector。 */
function installDom() {
  const attrs: Record<string, string> = {}
  const storage = new Map<string, string>()
  const meta: Record<string, string> = { 'theme-color': DARK }
  vi.stubGlobal('document', {
    createElement: () => ({ style: {}, setAttribute: () => {} }),
    documentElement: {
      setAttribute: (k: string, v: string) => (attrs[k] = v),
      getAttribute: (k: string) => attrs[k] ?? null,
    },
    head: {
      querySelector: (selector: string) => {
        const match = /name=["']([^"']+)["']/.exec(selector)
        const name = match?.[1]
        if (!name || !(name in meta)) return null
        // 真实 DOM 里 setAttribute('content', color) 改的就是该 meta 的 content，
        // 这里按 name 回填，便于断言 meta['theme-color']。
        return { setAttribute: (_k: string, v: string) => (meta[name] = v) }
      },
    },
  })
  vi.stubGlobal('localStorage', {
    getItem: (k: string) => (storage.has(k) ? storage.get(k)! : null),
    setItem: (k: string, v: string) => storage.set(k, String(v)),
    removeItem: (k: string) => storage.delete(k),
  })
  return { attrs, storage, meta }
}

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
  }
  const query = vi.fn(() => mql)
  vi.stubGlobal('matchMedia', query)
  return { mql, query }
}

beforeEach(() => {
  vi.resetModules()
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('PWA 外壳 theme-color 静态声明（R234）', () => {
  it('index.html 的默认 theme-color 与 dark.css 深色背景一致', () => {
    expect(html).toMatch(/<meta\s+name=["']theme-color["']\s+content=["']#060913["']\s*\/?>/)
  })

  it('manifest theme_color 与静态 meta 一致，且 background_color 不变', () => {
    expect(manifest.theme_color).toBe(DARK)
    expect(manifest.background_color).toBe(DARK)
  })
})

describe('主题切换时同步浏览器外壳（R234 行为契约）', () => {
  it('setTheme 显式选亮/暗时 meta[name=theme-color] 同步切换', async () => {
    const { meta } = installDom()
    const { createThemeProvider } = await import('./theme')
    const provider = createThemeProvider()

    provider.setTheme('light')
    expect(meta['theme-color'], '切到 light 后外壳仍是深色').toBe(LIGHT)
    provider.setTheme('dark')
    expect(meta['theme-color'], '切回 dark 后外壳未还原').toBe(DARK)
  })

  it('init 恢复持久化主题与跟随系统时，meta 也同步为对应外壳色', async () => {
    const { storage, meta } = installDom()
    const { mql } = installMatchMedia(true)
    const { createThemeProvider } = await import('./theme')

    createThemeProvider().init()
    expect(meta['theme-color']).toBe(DARK)

    storage.set('oa4rust_theme', 'light')
    createThemeProvider().init()
    expect(meta['theme-color'], 'init 恢复 light 后外壳仍是深色').toBe(LIGHT)

    storage.clear()
    createThemeProvider().init()
    mql.emit(false)
    expect(meta['theme-color'], '系统切到 light 后外壳未同步').toBe(LIGHT)
  })

  it('跨标签页同步主题时，meta 同样跟随', async () => {
    type Listener = (event: { key: string | null; newValue: string | null }) => void
    const { meta } = installDom()
    const listeners = new Map<string, Listener[]>()
    vi.stubGlobal('addEventListener', (type: string, cb: Listener) => {
      const list = listeners.get(type) ?? []
      list.push(cb)
      listeners.set(type, list)
    })
    const { createThemeProvider } = await import('./theme')
    createThemeProvider().init()
    expect(meta['theme-color']).toBe(DARK)

    for (const cb of listeners.get('storage') ?? []) {
      cb({ key: 'oa4rust_theme', newValue: 'light' })
    }
    expect(meta['theme-color'], '另一标签页切到 light 后外壳未同步').toBe(LIGHT)
  })

  it('无 meta 标签或非浏览器环境不抛错（SSR 安全）', async () => {
    const { meta } = installDom()
    delete meta['theme-color']
    const { createThemeProvider } = await import('./theme')
    expect(() => createThemeProvider().setTheme('light')).not.toThrow()
  })

  it('theme.ts 用共享同步函数覆盖全部入口，而非只在个别路径打补丁', () => {
    expect(themeSource, '必须存在共享的 syncBrowserThemeColor 同步函数').toMatch(/function\s+syncBrowserThemeColor/)
    expect(themeSource, '共享函数内必须更新 meta[name=theme-color] 的 content').toMatch(
      /querySelector\(['"]meta\[name=["']theme-color/, // 见下方等价断言
    )
    const applyStart = themeSource.indexOf('function applyTheme')
    expect(applyStart).toBeGreaterThan(-1)
    const applyFn = themeSource.slice(applyStart, themeSource.indexOf('\n  }', applyStart))
    expect(applyFn, 'applyTheme 必须调用共享同步函数，覆盖 setTheme/init/system 路径').toMatch(
      /syncBrowserThemeColor\(/, // init/system 都经 applyTheme
    )
    const storageStart = themeSource.indexOf("addEventListener('storage'")
    expect(storageStart).toBeGreaterThan(-1)
    const storageFn = themeSource.slice(storageStart, storageStart + 600)
    expect(storageFn, '跨标签页 storage 监听器也必须同步外壳').toMatch(/syncBrowserThemeColor\(/)
  })
})
