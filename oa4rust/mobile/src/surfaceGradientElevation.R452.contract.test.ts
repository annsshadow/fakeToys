// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/** R452: H5 根容器 .ai-surface-gradient 渐变被 uni-app scoped .page[data-v] (0,2,0)
 *  覆盖导致 computedStyle bgImage=none 的可达真缺陷。修复仿 .ai-brand-fill 模式，
 *  在 App.vue 为 .ai-surface-gradient 增加容器前缀+重复类提权 (0,2,1)。
 *  本契约断言 CSS 提权存在、令牌词序正确、19 页锚点不回退、业务样式不删。 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = (relative: string): string => readFileSync(resolve(import.meta.dirname, relative), 'utf8')
const app = src('App.vue')

/** R451 甄别坐实的 13 页失效页（scoped .page[data-v] 覆盖全局类） */
const BROKEN_PAGES = [
  'pages/message/message.vue',
  'pages/chat/chat.vue',
  'pages/process/process.vue',
  'pages/process/start.vue',
  'pages/attendance/attendance.vue',
  'pages/doc/doc.vue',
  'pages/contacts/contacts.vue',
  'pages/meeting/meeting.vue',
  'pages/calendar/calendar.vue',
  'pages/browse/browse.vue',
  'pages/search/search.vue',
  'pages/recycle/recycle.vue',
  'pages/mine/mine.vue',
]

/** R451 实测已生效 6 页，回归不得破坏 */
const WORKING_PAGES = [
  'pages/login/login.vue',
  'pages/index/index.vue',
  'pages/bbs/bbs.vue',
  'pages/bbs/topic.vue',
  'pages/bbs/new.vue',
  'pages/collect/collect.vue',
]

describe('R452 H5 .ai-surface-gradient 根容器渐变提权（scoped 覆盖修复）', () => {
  it('App.vue 保留全局 .ai-surface-gradient 单点定义且令牌词序正确', () => {
    const single = app.match(/\.ai-surface-gradient\s*\{[^}]*background:\s*var\(--ai-gradient-surface\)[^}]*\}/)?.[0] ?? ''
    expect(single, '全局单点定义必须保留且用正确令牌 --ai-gradient-surface').not.toBe('')
    expect(app).not.toContain('--ai-surface-gradient:')
  })

  it('App.vue 新增容器前缀+重复类提权规则达 (0,2,1) 压过 scoped .page', () => {
    const qualified = app.match(/(?:uni-page-body|body)\s+\.ai-surface-gradient\.ai-surface-gradient\s*\{[^}]*background:\s*var\(--ai-gradient-surface\)[^}]*\}/)?.[0] ?? ''
    expect(qualified, '需 uni-page-body/body .ai-surface-gradient.ai-surface-gradient 提权规则达 (0,2,1)').not.toBe('')
    expect(app, 'uni-page-body 前缀').toMatch(/uni-page-body\s+\.ai-surface-gradient\.ai-surface-gradient/)
    expect(app, 'body 前缀').toMatch(/body\s+\.ai-surface-gradient\.ai-surface-gradient/)
  })

  it('提权规则使用正确令牌 --ai-gradient-surface，不引入错误令牌 --ai-surface-gradient', () => {
    // uni-app 编译时 uni-page-body/body 为逗号合并选择器，共享一个规则块。
    const rule = app.match(/uni-page-body\s+\.ai-surface-gradient\.ai-surface-gradient\s*,\s*body\s+\.ai-surface-gradient\.ai-surface-gradient\s*\{[^}]*\}/)?.[0] ?? ''
    expect(rule, '需 uni-page-body 与 body 双前缀逗号合并选择器').not.toBe('')
    expect(rule).toContain('var(--ai-gradient-surface)')
    expect(rule).not.toContain('--ai-surface-gradient')
  })

  it('13 失效页 + 6 生效页根容器均保留 ai-surface-gradient 锚点', () => {
    for (const page of [...BROKEN_PAGES, ...WORKING_PAGES]) {
      expect(src(page), page).toMatch(/class="[^"]*\bai-surface-gradient\b[^"]*"/)
    }
  })

  it('13 失效页 scoped 背景声明仍在（CSS 提权不删业务样式）', () => {
    for (const page of BROKEN_PAGES) {
      expect(src(page), page + ' 必须保留 scoped 背景声明').toContain('background: var(--color-bg-page)')
    }
  })

  it('ai-brand-fill 提权契约零回退（既有品牌渐变规则不动）', () => {
    const brand = app.match(/(?:uni-page-body|body)\s+\.ai-brand-fill\.ai-brand-fill\s*\{[^}]*background:\s*var\(--ai-gradient-brand\)[^}]*\}/)?.[0] ?? ''
    expect(brand, 'brand-fill 提权规则必须仍在').not.toBe('')
  })
})
