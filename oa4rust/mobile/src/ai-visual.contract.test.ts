// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/** R300 B6: H5 端首轮视觉语言对齐契约。
 * 断言 AI 令牌从桌面 packages/ui 引入移动端全局样式，20 个页面（19 路由 + App.vue）
 * 均出现一处 ai-* 类锚点，且关键锚点从纯色升级为渐变/玻璃，但既有语义色合同不变。 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = (relative: string): string => readFileSync(resolve(import.meta.dirname, relative), 'utf8')
const app = src('App.vue')
const pagesJson = JSON.parse(src('pages.json'))

const PAGES: string[] = [
  'pages/index/index.vue',
  'pages/message/message.vue',
  'pages/chat/chat.vue',
  'pages/process/process.vue',
  'pages/process/start.vue',
  'pages/attendance/attendance.vue',
  'pages/doc/doc.vue',
  'pages/contacts/contacts.vue',
  'pages/meeting/meeting.vue',
  'pages/calendar/calendar.vue',
  'pages/bbs/bbs.vue',
  'pages/bbs/topic.vue',
  'pages/bbs/new.vue',
  'pages/collect/collect.vue',
  'pages/browse/browse.vue',
  'pages/search/search.vue',
  'pages/recycle/recycle.vue',
  'pages/mine/mine.vue',
  'pages/login/login.vue',
]

describe('R300 H5 视觉语言首轮对齐（20 页面覆盖）', () => {
  it('App.vue 消费桌面 R300 AI 令牌：8px 栅格/渐变/玻璃/弹簧缓动/shimmer', () => {
    for (const token of [
      '--ai-grid-unit', '--ai-grid-8', '--ai-indigo', '--ai-cyan',
      '--ai-gradient-brand', '--ai-gradient-text', '--ai-glass-bg',
      '--ai-glass-border', '--ai-glass-blur', '--ai-shadow-2',
      '--ai-ease-spring', '--ai-motion-fast', '--ai-skeleton-shine',
    ]) {
      expect(app, token).toMatch(new RegExp(token.replace(/[-]/g, '\\-') + '\\s*:'))
    }
  })

  it('App.vue 提供全局 ai-* 视觉类：渐变品牌面、玻璃面板、渐变字、骨架屏', () => {
    for (const cls of ['ai-brand-fill', 'ai-panel', 'ai-gradient-text', 'ai-skeleton-line']) {
      expect(app, cls).toContain('.' + cls)
    }
    expect(app).toContain('@keyframes ai-shimmer')
  })

  it('pages.json 的 19 个路由都拥有对应页面文件（第 20 个为 App.vue）', () => {
    const routed = pagesJson.pages.map((p: { path: string }) => p.path + '.vue')
    expect(routed.sort()).toEqual([...PAGES].sort())
    for (const page of PAGES) expect(() => src(page), page).not.toThrow()
  })

  it('全部 19 个路由页面都带有 R300 H5 视觉锚点 class="ai-*"', () => {
    for (const page of PAGES) {
      expect(src(page), page).toMatch(/class="[^"]*\bai-[a-z-]+\b[^"]*"/)
    }
  })

  it('首页统计卡从纯色升级为 AI 渐变锚点，同时保留原 .stat-card 语义', () => {
    const index = src('pages/index/index.vue')
    expect(index).toMatch(/class="stat-card[^"]*ai-brand-fill/)
  })

  it('登录页主按钮带 ai-brand-fill 渐变锚点，原 color/background token 合同不变', () => {
    const login = src('pages/login/login.vue')
    expect(login).toMatch(/class="submit[^"]*ai-brand-fill/)
    expect(login).toContain('background: var(--color-primary)')
  })

  it('全局渐变锚点拥有高于 uni-app scoped 页面规则的优先级（实测覆盖回归）', () => {
    // uni-app 的 data-v scoped 规则特异性 (0,2,0) 会压过普通 .ai-brand-fill (0,1,0)，
    // 导致 H5 真机渲染仍为纯色；需用容器前缀+重复类或 !important 明确让位。
    const brandRule = app.match(/\.ai-brand-fill\s*\{[^}]*background:\s*var\(--ai-gradient-brand\)[^}]*\}/)?.[0] ?? ''
    expect(brandRule, '全局 .ai-brand-fill 必须声明渐变').not.toBe('')
    // scoped 规则 .stat-card[data-v] 特异性为 (0,2,0)；容器前缀+重复类 (0,2,1) 才能压过。
    const qualified = app.match(/(?:uni-page-body|body)\s+\.ai-brand-fill\.ai-brand-fill\s*\{[^}]*background:\s*var\(--ai-gradient-brand\)[^}]*\}/)?.[0] ?? ''
    expect(qualified, 'H5 实测需容器前缀+重复类达到 (0,2,1) 压过 scoped').not.toBe('')

    const indexScoped = src('pages/index/index.vue').match(/\.stat-card\s*\{[^}]*background:\s*var\(--color-primary\)[^}]*\}/)?.[0] ?? ''
    if (indexScoped) expect(indexScoped, '首页 stat-card 纯色基础值不得用 !important 压过渐变').not.toMatch(/background[^;]*color-primary[^;]*!important/)
    const loginScoped = src('pages/login/login.vue').match(/\.submit\s*\{[^}]*background:\s*var\(--color-primary\)[^}]*\}/)?.[0] ?? ''
    if (loginScoped) expect(loginScoped, '登录 submit 纯色基础值不得用 !important 压过渐变').not.toMatch(/background[^;]*color-primary[^;]*!important/)
  })
});
