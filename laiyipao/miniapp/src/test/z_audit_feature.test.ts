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

  it('R81–R85 表单校验与失败反馈', () => {
    expect(readPages('login')).toContain('form-hint')
    expect(readPages('signin')).toContain('input-error')
    expect(readPages('verify')).toContain('input-error')
    expect(readPages('shop')).toContain('buy-failed')
    expect(readPages('bag')).toContain('preset-saved-hint')
  })

  it('R86–R90 收尾视觉打磨', () => {
    const css = CSS()
    expect(css).toContain('border-left: 5rpx solid var(--fire)')
    expect(css).toContain('/* R87 chip-row 间距一致 */')
    expect(css).toContain('@keyframes btn-sheen')
    expect(css).toContain('R90 tag-outline')
    expect(css).toContain('R88 link-underline-active')
  })

  it('R91–R95 各页最终打磨', () => {
    expect(readPages('stage')).toContain('R91 星级进度条淡入光泽')
    expect(readPages('rank')).toContain('@keyframes medal-float')
    expect(readPages('battles')).toContain('@keyframes badge-pop')
    expect(readPages('me')).toContain('.back-btn:active')
    expect(readPages('verify')).toContain('R95 裁定文字加字距')
  })

  it('R96 减少动效补全', () => {
    const css = CSS()
    expect(css).toContain('R96 减少动效补全')
    expect(css).toContain('.btn-primary::after')
  })

  it('R97 键盘焦点可见', () => {
    expect(CSS()).toContain(':focus-visible')
  })

  it('R98 桌面卡片悬停', () => {
    expect(CSS()).toContain('.card:hover')
  })

  it('R99 平滑滚动', () => {
    expect(CSS()).toContain('scroll-behavior: smooth')
  })

  it('R100 锚点避让 + 全端审计关键标记齐全', () => {
    const css = CSS()
    expect(css).toContain('scroll-margin-top: 120rpx')
    // 全端 100 轮的关键动效/反馈标记任何一处被删都会红
    for (const marker of [
      'skeleton-shimmer', 'pop-in', 'claim-pop', 'stagger-in', 'shake',
      'ok-pulse', 'spin', 'page-in', 'chip-pop', 'stars-in', 'empty-float',
      'btn-sheen', 'prefers-reduced-motion', ':focus-visible',
    ]) {
      expect(css, marker).toContain(marker)
    }
    // 页面级动效标记（不在 common.css 中）
    expect(readPages('rank')).toContain('medal-float')
    expect(readPages('battles')).toContain('badge-pop')
  })
})

function readPages(name: string): string {
  return readFileSync(join(__dirname, `../pages/${name}/${name}.vue`), 'utf-8')
}
