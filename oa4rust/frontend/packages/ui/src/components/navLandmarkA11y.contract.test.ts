// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R223 契约：导航 landmark 可区分 + 当前位置语义（WCAG 1.3.1 / 2.4.8）。
 *
 * 缺陷证据：AppShell.vue 有两处 <nav>（侧边栏、移动端底部 Tab），均无 aria-label，
 * 读屏器 landmark 列表里读作两个无差别的「navigation」，用户无法区分；导航项仅用
 * class="active" 表达当前页面（R205 navActive 只修了视觉段边界），DOM 中没有任何
 * aria-current，读屏用户 Tab 到导航项时听不到「当前页面」。
 *
 * 契约：两处 nav 各有中文 aria-label；两类导航项用 isActive(...) 派生
 * aria-current="page"，非当前项不产生 current 语义。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function appShellSource(): string {
  return readFileSync(resolve(import.meta.dirname, 'AppShell.vue'), 'utf8')
}

describe('导航 landmark 与当前位置语义（WCAG 1.3.1 / 2.4.8）', () => {
  const src = appShellSource()

  it('侧边栏主导航 <nav> 带可区分的 aria-label', () => {
    const tag = src.match(/<nav class="sidebar-nav"[^>]*>/)?.[0] ?? ''
    expect(tag, '侧边栏导航缺少 aria-label').toMatch(/aria-label="[^"]+"/)
  })

  it('移动端底部导航 <nav> 带可区分的 aria-label', () => {
    const tag = src.match(/<nav v-if="isMobile" class="tab-bar"[^>]*>/)?.[0] ?? ''
    expect(tag, '底部 Tab 导航缺少 aria-label').toMatch(/aria-label="[^"]+"/)
  })

  it('两处导航 aria-label 都存在且互不相同', () => {
    const labels = [...src.matchAll(/<nav[^>]*aria-label="([^"]+)"/g)].map((m) => m[1])
    expect(labels.length, '两处 <nav> 都应有 aria-label').toBeGreaterThanOrEqual(2)
    expect(new Set(labels).size, '两处导航 aria-label 不得相同').toBe(labels.length)
  })

  it('侧边栏与底部 Tab 导航项都用 isActive 派生 aria-current="page"（共两处）', () => {
    const matches = src.match(/:aria-current="isActive\(item\.path\)\s*\?\s*'page'\s*:\s*undefined"/g) ?? []
    expect(matches.length, '应为侧边栏 nav-item 与底部 tab-item 各一处 aria-current').toBe(2)
  })

  it('不得把 aria-current 静态写死为 page（否则每项都宣称当前页）', () => {
    expect(src).not.toMatch(/aria-current="page"/)
  })
})
