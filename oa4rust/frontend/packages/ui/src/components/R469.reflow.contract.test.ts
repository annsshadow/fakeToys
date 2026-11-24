// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R469 PC 窄视口 Reflow 契约：移动端侧栏默认抽屉隐藏、内容回流占满视口。
 *
 * 缺陷证据（R468 甄别）：320px 视口下 .app-shell.mobile .sidebar 无条件
 * transform:translateX(0) 让 280px 侧栏常驻；.topbar/.content 仍叠加
 * margin-left:var(--sidebar-width) 240px，主内容仅剩 80px 文字碎片。
 * 本契约以源码静态真断言锁定抽屉态修复，父版必 RED、修复版必 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AppShell.vue'), 'utf8')

/** 提取 @media (max-width: 768px) 整块（含花括号配平） */
function mediaBlock(src: string): string {
  const m = src.match(/@media \(max-width: 768px\)\s*\{/)
  if (!m || m.index === undefined) return ''
  const start = m.index + m[0].length - 1
  let depth = 0
  for (let i = start; i < src.length; i++) {
    if (src[i] === '{') depth++
    else if (src[i] === '}') {
      depth--
      if (depth === 0) return src.slice(start, i + 1)
    }
  }
  return ''
}

describe('AppShell 移动端 Reflow 抽屉态（R469）', () => {
  it('根容器绑定 drawer-open 且存在 mobileDrawerOpen ref', () => {
    expect(source, '根 div 必须绑定 drawer-open 状态类').toMatch(/'drawer-open'\s*:\s*mobileDrawerOpen/)
    expect(source, '必须新增 mobileDrawerOpen 响应式状态').toMatch(/const\s+mobileDrawerOpen\s*=\s*ref\(false\)/)
  })

  it('toggleSidebar 移动端切抽屉、桌面端维持原折叠', () => {
    const fn = source.match(/function toggleSidebar\(\): void \{[\s\S]*?\n\}/)?.[0] ?? ''
    expect(fn, '必须按 isMobile.value 分支').toMatch(/if\s*\(isMobile\.value\)/)
    expect(fn, '移动端必须切换抽屉').toMatch(/mobileDrawerOpen\.value\s*=\s*!mobileDrawerOpen\.value/)
    expect(fn, '移动端分支必须提前 return').toMatch(/return/)
    expect(fn, '桌面端必须保留原折叠逻辑').toMatch(/sidebarCollapsed\.value\s*=\s*!sidebarCollapsed\.value/)
  })

  it('移动端侧栏基线 translateX(-100%) 隐藏，仅 drawer-open 态滑出', () => {
    const media = mediaBlock(source)
    expect(media, '基线必须 translateX(-100%) 隐藏').toMatch(/\.sidebar\s*\{[^}]*transform:\s*translateX\(-100%\)/)
    expect(media, '只有 drawer-open 态才滑出').toMatch(/\.app-shell\.mobile\.drawer-open\s+\.sidebar\s*\{[^}]*transform:\s*translateX\(0\)/)
    expect(media, '禁止无条件的 .app-shell.mobile .sidebar 常驻').not.toMatch(/\.app-shell\.mobile\s+\.sidebar\s*\{[^}]*transform:\s*translateX\(0\)/)
  })

  it('移动端 topbar/content margin-left 归零，不再有侧栏偏移补偿', () => {
    const media = mediaBlock(source)
    expect(media, '移动端 topbar/content 必须 margin-left:0').toMatch(/\.topbar,\s*\.content\s*\{[^}]*margin-left:\s*0/)
    expect(media, '移动端禁止 --sidebar-width 偏移').not.toMatch(/\.topbar,\s*\.content\s*\{[^}]*margin-left:\s*var\(--sidebar-width\)/)
  })

  it('R293 过时移动端偏移补偿已移除', () => {
    expect(source, '不得再保留移动端 topbar/content 的 --sidebar-width 补偿').not.toMatch(
      /@media \(max-width: 768px\)\s*\{[\s\S]*?\.topbar,\s*\.content\s*\{[^}]*margin-left:\s*var\(--sidebar-width\)/,
    )
  })
})
