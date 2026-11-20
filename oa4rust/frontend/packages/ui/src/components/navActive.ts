// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 侧边栏导航项高亮判定（从 AppShell.vue 抽取，便于契约测试）。
 *
 * 判定当前路由路径是否落在某个导航项下，用于 `:class="{ active: isActive(path) }"`。
 */
export function isNavActive(currentPath: string, navPath: string): boolean {
  if (!navPath) return false
  // 归一化导航项尾部斜杠：'/app/process/' 与 '/app/process' 视为同一路由。
  const base = navPath.replace(/\/+$/, '')
  if (!base) return false
  // 只允许两种命中：完全相等，或当前路径位于该导航项的「子段」下。
  // 关键是不能再用朴素 startsWith：/app/fileinfo 与 /app/file 前缀相同却是兄弟路由。
  return currentPath === base || currentPath.startsWith(`${base}/`)
}
