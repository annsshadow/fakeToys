// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { type Ref, ref } from 'vue'

export type ThemeMode = 'dark' | 'light'

const _theme = ref<ThemeMode>('dark')

/**
 * R196：所有 createThemeProvider() 实例共享同一份 _theme 状态，系统明暗跟随的
 * 销毁闭包也必须共享。真实链路里 init() 跑在 main.ts 的临时实例上，而用户选色走
 * useTheme() 单例；若把 stopFollowing 放在实例闭包里，单例 setTheme() 看不到它，
 * 就拆不掉临时实例安装的 matchMedia change 监听器，OS 切换会覆盖用户的显式主题。
 */
let stopFollowing: (() => void) | null = null

const DARK_QUERY = '(prefers-color-scheme: dark)'
const THEME_KEY = 'oa4rust_theme'

/**
 * 主题管理
 * o2web 不支持主题切换，这是新栈的增强能力
 */
export function createThemeProvider() {
  /** 只更新内存与 <html> 属性，不落盘（系统跟随是隐式的）。 */
  function applyTheme(mode: ThemeMode): void {
    _theme.value = mode
    document.documentElement.setAttribute('data-theme', mode)
  }

  function setTheme(mode: ThemeMode): void {
    stopFollowing?.()
    stopFollowing = null
    applyTheme(mode)
    localStorage.setItem(THEME_KEY, mode)
  }

  function toggleTheme(): void {
    setTheme(_theme.value === 'dark' ? 'light' : 'dark')
  }

  function init(): void {
    // 重新初始化先拆掉旧的系统跟随监听，避免 stopFollowing 被覆盖后旧监听残留。
    stopFollowing?.()
    stopFollowing = null
    const stored = localStorage.getItem(THEME_KEY) as ThemeMode | null
    if (stored) {
      applyTheme(stored)
      return
    }

    // SSR/node 无 matchMedia：安全兜底 dark，不注册任何监听器。
    if (typeof matchMedia !== 'function') {
      applyTheme('dark')
      return
    }

    const mql = matchMedia(DARK_QUERY)
    applyTheme(mql.matches ? 'dark' : 'light')

    const listener = (event: MediaQueryListEvent): void => {
      applyTheme(event.matches ? 'dark' : 'light')
    }
    mql.addEventListener('change', listener)
    stopFollowing = () => mql.removeEventListener('change', listener)
  }

  return {
    theme: _theme as Ref<ThemeMode>,
    setTheme,
    toggleTheme,
    init,
  }
}

export const themeProvider = createThemeProvider()

/** Composable 入口 */
export function useTheme() {
  return themeProvider
}
