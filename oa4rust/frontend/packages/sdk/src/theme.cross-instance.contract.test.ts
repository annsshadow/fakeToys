// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R196 主题系统跨实例生命周期契约。
 *
 * 缺陷证据：_theme 是模块级共享 ref（theme.ts:8），所有 createThemeProvider()
 * 实例共用同一份状态；但系统明暗跟随监听的销毁闭包 stopFollowing 是每个实例各自的
 * 局部状态（theme.ts:19/28/57-58）。真实启动链路里：
 *   - main.ts:534-535 用 createThemeProvider() 建「临时」实例并 init()，监听器装在这个
 *     临时实例上，stopFollowing 也记在它身上；
 *   - AppShell.vue:126 通过 useTheme() 拿到「单例」themeProvider，用户点主题切换时
 *     走单例的 setTheme()，而单例从未 init()，它的 stopFollowing 永远是 null。
 * 结果：用户显式选色后，系统偏好 change 监听器并未被移除，仍持续改写共享 _theme，
 * OS 明暗一变就覆盖用户的显式选择。
 *
 * 本测试钉住真实用户路径（init 在临时实例、选色在单例），防止跨实例状态污染回归。
 */
// @vitest-environment node
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
// 静态导入先于 document 桩：Vue runtime-dom 在导入期读取 document，与 theme.test.ts 一致。
import { createThemeProvider, useTheme } from './theme'

function installDom(): void {
  vi.stubGlobal('document', {
    documentElement: {
      setAttribute: () => {},
      getAttribute: () => null,
    },
  })
  const storage = new Map<string, string>()
  vi.stubGlobal('localStorage', {
    getItem: (k: string) => (storage.has(k) ? storage.get(k)! : null),
    setItem: (k: string, v: string) => storage.set(k, String(v)),
  })
}

/** 可控 matchMedia：count 暴露当前 system-following 监听器数量。 */
function installMatchMedia(initialMatches: boolean) {
  const listeners = new Set<(e: { matches: boolean }) => void>()
  const mql = {
    matches: initialMatches,
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
    get count() {
      return listeners.size
    },
  }
  vi.stubGlobal('matchMedia', () => mql)
  return mql
}

beforeEach(() => {
  installDom()
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('主题系统跨实例生命周期（R196 防回归）', () => {
  it('临时实例 init 安装的系统跟随，必须被单例的显式选色一并停止', () => {
    // 系统初始为亮色：临时实例 init 后进入「跟随系统」并安装监听器。
    const mql = installMatchMedia(false)
    const bootstrap = createThemeProvider()
    bootstrap.init()
    expect(mql.count).toBe(1)
    expect(useTheme().theme.value).toBe('light')

    // 真实用户路径：AppShell 通过单例显式选「暗色」。
    const shell = useTheme()
    shell.setTheme('dark')
    expect(useTheme().theme.value).toBe('dark')

    // 契约：显式选色后系统跟随必须被拆除，否则 OS 切换会覆盖用户选择。
    expect(
      mql.count,
      '用户在单例上显式选色后，临时实例安装的 matchMedia change 监听器仍在（stopFollowing 未跨实例共享）',
    ).toBe(0)

    // 行为层面：此后即便系统明暗真的变化，也不得改写用户的显式选择。
    mql.emit(true)
    mql.emit(false)
    expect(useTheme().theme.value, 'OS 明暗切换覆盖了用户的显式主题选择').toBe('dark')
  })
})
