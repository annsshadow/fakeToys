// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * Z 批（收尾审计与回归）结构守卫（2026-11-17）。
 *
 * 收尾阶段以「全局一致性」为主：任何一处共享样式被删掉，
 * 全端都会退化，但现有行为测试全绿 —— 用源码扫描固化。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const CSS = () => readFileSync(join(__dirname, '../styles/common.css'), 'utf-8')

describe('Z 批收尾审计守卫', () => {
  it('R71 chip/tag 按压反馈', () => {
    const css = CSS()
    expect(css).toContain('.chip:active')
    expect(css).toContain('.tag:active')
  })

  it('R72 H5 滚动条细化', () => {
    const css = CSS()
    expect(css).toContain('::-webkit-scrollbar')
    expect(css).toContain('::-webkit-scrollbar-thumb')
  })

  it('R73 次级文字对比度', () => {
    const css = CSS()
    expect(css).toContain('.muted { color: var(--text-2); }')
    expect(css).toContain('.dim { color: var(--muted); }')
  })

  it('R74 哈希字距与选中高亮', () => {
    const css = CSS()
    expect(css).toContain('::selection')
    expect(css).toContain('letter-spacing: 0.5rpx')
  })

  it('R75 按钮最小点击区', () => {
    expect(CSS()).toContain('min-height: 72rpx')
  })

  it('R76–R80 各页数量徽章', () => {
    expect(readPages('bag')).toContain('tab-count')
    expect(readPages('shop')).toContain('group-count')
    expect(readPages('battles')).toContain('filter-count')
    expect(readPages('rank')).toContain('rank-count')
    expect(readPages('me')).toContain('cand-count')
  })
})

function readPages(name: string): string {
  return readFileSync(join(__dirname, `../pages/${name}/${name}.vue`), 'utf-8')
}
