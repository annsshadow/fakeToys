// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * X 批 R31/R32 微交互守卫（2026-11-17）。
 *
 * 按压反馈与骨架屏都是「看不见也能编译通过」的纯样式改动 ——
 * 用源码扫描把它变成结构约束：删除后测试立刻失败。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const read = (rel: string) => readFileSync(join(__dirname, rel), 'utf-8')
const CSS = () => read('../styles/common.css')

describe('微交互与加载反馈守卫', () => {
  it('R31 按钮/链接按压反馈 + 减少动效无障碍分支', () => {
    const css = CSS()
    expect(css).toContain('.btn:active:not(.btn-disabled)')
    expect(css).toContain('.link:active')
    expect(css).toContain('@media (prefers-reduced-motion: reduce)')
  })

  it('R32 共享骨架屏样式落地', () => {
    const css = CSS()
    expect(css).toContain('.skeleton-row')
    expect(css).toContain('.skeleton-bar')
    expect(css).toContain('@keyframes skeleton-shimmer')
  })

  it('R32 四个列表页加载态使用骨架占位且保留加载文案', () => {
    for (const p of ['rank', 'battles', 'shop', 'announcements']) {
      const src = read(`../pages/${p}/${p}.vue`)
      expect(src, p).toContain('skeleton-row')
      expect(src, p).toContain('加载中…')
    }
  })
})
