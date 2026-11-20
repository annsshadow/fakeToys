// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R293 桌面壳层偏移几何契约：侧栏占位必须有且只有一条链路。
 *
 * 缺陷证据：AppShell.vue 中 .main-area 已用 margin-left: var(--sidebar-width)
 * 为固定定位的侧栏让位，但 .topbar 与 .content 又各自叠加内联
 * :style="{ marginLeft: ... }"（展开 240px / 折叠 64px），内容区被双重右移，
 * 侧栏旁留下一条 --sidebar-width 宽的永久空带；且 .main-area 的 margin 不随
 * 折叠状态变化，折叠后空带不回收，视觉上比展开态更空。
 *
 * 契约：
 *   1) 模板不得再用内联 marginLeft 为 topbar/content 让位；
 *   2) .main-area 是唯一的桌面偏移来源，折叠态由
 *      .app-shell.collapsed .main-area 收窄到 --sidebar-collapsed；
 *   3) 移动端媒体查询需为 .topbar/.content 提供与桌面同源的偏移令牌；
 *   4) 顶层 .content 不再自带 margin-left（避免第三条偏移链路）。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AppShell.vue'), 'utf8')

describe('AppShell 侧栏偏移几何（R293）', () => {
  it('topbar/content 不再携带内联 marginLeft 让位样式', () => {
    expect(source, 'topbar/content 上的内联 marginLeft 与 .main-area 的 margin 叠加，造成内容区双重右移').not.toMatch(
      /:style="\{\s*marginLeft:/,
    )
  })

  it('.main-area 是唯一桌面偏移来源，且随折叠态收窄', () => {
    expect(source, '.main-area 必须用 --sidebar-width 让位').toMatch(
      /\.main-area\s*\{[^}]*margin-left:\s*var\(--sidebar-width\)/,
    )
    expect(source, '折叠态必须由 .app-shell.collapsed .main-area 收窄到 --sidebar-collapsed').toMatch(
      /\.app-shell\.collapsed \.main-area\s*\{[^}]*margin-left:\s*var\(--sidebar-collapsed\)/,
    )
  })

  it('移动端媒体查询为 topbar/content 提供侧栏偏移补偿', () => {
    expect(source).toMatch(
      /@media \(max-width: 768px\)\s*\{[\s\S]*?\.topbar,\s*\.content\s*\{[^}]*margin-left:\s*var\(--sidebar-width\)/,
    )
  })

  it('顶层 .content 不自带第三条 margin 偏移链路', () => {
    const topLevelContent = source.match(/\.content\s*\{[^}]*\}/)?.[0] ?? ''
    expect(topLevelContent, '.content 自身不得再声明 margin-left').not.toContain('margin-left')
  })
})
