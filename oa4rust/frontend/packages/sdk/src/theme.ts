// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { type Ref, ref } from 'vue'

export type ThemeMode = 'dark' | 'light'

const _theme = ref<ThemeMode>('dark')

const DARK_QUERY = '(prefers-color-scheme: dark)'
const THEME_KEY = 'oa4rust_theme'

/**
 * 主题管理
 * o2web 不支持主题切换，这是新栈的增强能力
 */
export function createThemeProvider() {
  /** 取消跟随系统偏好的闭包；用户显式选择后调用，之后不再自动联动。 */
  let stopFollowing: (() => void) | null = null

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
