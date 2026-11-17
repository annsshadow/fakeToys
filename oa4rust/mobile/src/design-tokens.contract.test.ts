// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// 契约测试：设计 token 在每个目标页面上的语义落地，而非简单字符串扫描。
// RED 阶段：App.vue / 目标页面尚未改造，以下断言应失败。
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = (relative: string): string => readFileSync(resolve(import.meta.dirname, relative), 'utf8')

const app = src('App.vue')
const bbs = src('pages/bbs/bbs.vue')
const login = src('pages/login/login.vue')
const mine = src('pages/mine/mine.vue')
const start = src('pages/process/start.vue')
const process = src('pages/process/process.vue')
const meeting = src('pages/meeting/meeting.vue')
const calendar = src('pages/calendar/calendar.vue')
const contacts = src('pages/contacts/contacts.vue')
const attendance = src('pages/attendance/attendance.vue')
const index = src('pages/index/index.vue')
const chat = src('pages/chat/chat.vue')

/** 从页面 <style scoped> 中按选择器+属性取值，确保断言绑定在实际渲染规则上。 */
function styleValue(page: string, selector: string, property: string): string {
  const block = page.match(new RegExp('\\' + selector + '\\s*\\{([^}]*)\\}'))
  expect(block, '缺少选择器 ' + selector).not.toBeNull()
  const match = block![1].match(new RegExp('(?:^|;)\\s*' + property + '\\s*:\\s*([^;]+)'))
  expect(match, selector + ' 缺少属性 ' + property).not.toBeNull()
  return match![1].trim()
}

const tokens: Record<string, string> = {
  '--color-bg-page': '#f5f7fa',
  '--color-primary': '#2d8cf0',
  '--color-text-primary': '#263238',
  '--color-text-secondary': '#90979f',
  '--color-text-muted': '#8a94a6',
  '--color-error': '#c0392b',
  '--color-error-bg': '#fdecea',
  '--color-surface': '#ffffff',
  '--color-border': '#e3e6eb',
  '--color-divider': '#f0f2f5',
  '--color-primary-soft': '#f0f7ff',
  '--color-error-tint': '#fdf2f0',
  '--color-text-body': '#5b6572',
  '--color-placeholder': '#c0c4cc',
  '--color-border-strong': '#dcdfe6',
  '--color-warning': '#ff9900',
  '--color-success': '#67c23a',
  '--color-warning-bg': '#fff4e5',
  '--color-warning-text': '#b26a00',
}

describe('移动端设计 token 语义落地契约', () => {
  it('App.vue 全局样式定义全部 token，且值取当前最高频色', () => {
    expect(app).toContain('<style>')
    expect(app).not.toContain('<style scoped>')
    for (const [name, value] of Object.entries(tokens)) {
      expect(app).toMatch(new RegExp(name.replace(/[-]/g, '\\-') + '\\s*:\\s*' + value.replace('#', '\\#')))
    }
  })

  it('App.vue 的 page 背景也走 token，避免全局色与 token 双源分叉', () => {
    expect(styleValue(app, 'page', 'background-color')).toBe('var(--color-bg-page)')
  })

  it('bbs 页面：正文标题归一到主文字色 token，且不再硬编码旧灰阶', () => {
    expect(styleValue(bbs, '.toolbar-title', 'color')).toBe('var(--color-text-primary)')
    expect(styleValue(bbs, '.topic-title', 'color')).toBe('var(--color-text-primary)')
    expect(bbs).not.toContain('color: #1f2733')
  })

  it('bbs 页面：错误态颜色与背景走语义 token，不再硬编码', () => {
    expect(styleValue(bbs, '.tip-error', 'color')).toBe('var(--color-error)')
    expect(styleValue(bbs, '.error-banner', 'color')).toBe('var(--color-error)')
    expect(styleValue(bbs, '.error-banner', 'background')).toBe('var(--color-error-bg)')
    expect(styleValue(bbs, '.retry-btn', 'border')).toBe('1px solid var(--color-error)')
    expect(styleValue(bbs, '.retry-btn', 'color')).toBe('var(--color-error)')
    expect(bbs).not.toContain('color: #c0392b')
    expect(bbs).not.toContain('background: #fdecea')
  })

  it('bbs 页面：主蓝按钮与卡片表面归一到 token，无遗留次高频蓝', () => {
    expect(styleValue(bbs, '.new-btn', 'background')).toBe('var(--color-primary)')
    expect(styleValue(bbs, '.new-btn', 'color')).toBe('var(--color-surface)')
    expect(styleValue(bbs, '.topic', 'background')).toBe('var(--color-surface)')
    expect(bbs).not.toContain('background: #2f6bff')
    expect(bbs).not.toContain('background: #ffffff')
  })

  it('bbs 页面：提示与摘要灰阶归一到对应 token', () => {
    expect(styleValue(bbs, '.tip', 'color')).toBe('var(--color-text-muted)')
    expect(styleValue(bbs, '.topic-meta', 'color')).toBe('var(--color-text-muted)')
    expect(styleValue(bbs, '.topic-excerpt', 'color')).toBe('var(--color-text-secondary)')
    expect(bbs).not.toContain('color: #8a94a6')
    expect(bbs).not.toContain('color: #5a6478')
  })

  it('login 页面：品牌与提交按钮主蓝归一到 token', () => {
    expect(styleValue(login, '.brand', 'color')).toBe('var(--color-primary)')
    expect(styleValue(login, '.submit', 'background')).toBe('var(--color-primary)')
    expect(login).not.toContain('color: #2d8cf0')
    expect(login).not.toContain('background: #2d8cf0')
  })

  it('login 页面：错误红归一到 token，不再使用次高频红 #f56c6c', () => {
    expect(styleValue(login, '.error', 'color')).toBe('var(--color-error)')
    expect(login).not.toContain('#f56c6c')
  })

  it('login 页面：表单表面与边框走 token，输入背景仍保持视觉等价', () => {
    expect(styleValue(login, '.field input', 'background')).toBe('var(--color-surface)')
    expect(styleValue(login, '.field input', 'border')).toBe('1px solid var(--color-border)')
    expect(styleValue(login, '.label', 'color')).toBe('var(--color-text-secondary)')
    expect(login).not.toContain('color: #666')
    expect(styleValue(login, '.submit', 'color')).toBe('var(--color-surface)')
    expect(login).not.toContain('background: #fff')
    expect(login).not.toContain('border: 1px solid #e3e6eb')
  })

  it('mine 页面：22 处硬编码色全部归一到语义 token', () => {
    expect(styleValue(mine, '.page', 'background')).toBe('var(--color-bg-page)')
    expect(styleValue(mine, '.card', 'background')).toBe('var(--color-surface)')
    expect(styleValue(mine, '.avatar', 'background')).toBe('var(--color-primary)')
    expect(styleValue(mine, '.avatar', 'color')).toBe('var(--color-surface)')
    expect(styleValue(mine, '.name', 'color')).toBe('var(--color-text-primary)')
    expect(styleValue(mine, '.sub', 'color')).toBe('var(--color-text-secondary)')
    expect(styleValue(mine, '.row', 'border-bottom')).toBe('1px solid var(--color-divider)')
    expect(styleValue(mine, '.k', 'color')).toBe('var(--color-text-secondary)')
    expect(styleValue(mine, '.v', 'color')).toBe('var(--color-text-primary)')
    expect(styleValue(mine, '.push-desc', 'color')).toBe('var(--color-text-secondary)')
    expect(styleValue(mine, '.logout', 'color')).toBe('var(--color-error)')
    expect(styleValue(mine, '.logout', 'border')).toBe('1px solid var(--color-error)')
    expect(styleValue(mine, '.pwd-input', 'background')).toBe('var(--color-bg-page)')
    expect(styleValue(mine, '.pwd-btn', 'background')).toBe('var(--color-primary)')
    expect(mine).not.toMatch(/color: #[0-9a-fA-F]{3,6}/)
    expect(mine).not.toMatch(/background: #[0-9a-fA-F]{3,6}/)
    expect(mine).not.toMatch(/border: 1px solid #[0-9a-fA-F]{3,6}/)
  })

  it('process/start 页面：35 处硬编码色全部归一到语义 token', () => {
    expect(styleValue(start, '.page', 'background')).toBe('var(--color-bg-page)')
    expect(styleValue(start, '.card', 'background')).toBe('var(--color-surface)')
    expect(styleValue(start, '.card-title', 'color')).toBe('var(--color-text-primary)')
    expect(styleValue(start, '.group-title', 'color')).toBe('var(--color-primary)')
    expect(styleValue(start, '.group-title', 'border-left')).toBe('6rpx solid var(--color-primary)')
    expect(styleValue(start, '.proc-item', 'border')).toBe('1px solid var(--color-border)')
    expect(styleValue(start, '.proc-item.on', 'background')).toBe('var(--color-primary-soft)')
    expect(styleValue(start, '.error-banner', 'background')).toBe('var(--color-error-bg)')
    expect(styleValue(start, '.error-banner', 'color')).toBe('var(--color-error)')
    expect(styleValue(start, '.retry-btn', 'border')).toBe('1px solid var(--color-error)')
    expect(styleValue(start, '.label', 'color')).toBe('var(--color-text-secondary)')
    expect(styleValue(start, '.attach-btn', 'border')).toBe('1px dashed var(--color-primary)')
    expect(styleValue(start, '.submit', 'background')).toBe('var(--color-primary)')
    expect(start).not.toMatch(/#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})\b/)
  })

  it('process/process 页面：28 处硬编码色全部归一到语义 token', () => {
    expect(styleValue(process, '.page', 'background')).toBe('var(--color-bg-page)')
    expect(styleValue(process, '.tabs', 'background')).toBe('var(--color-surface)')
    expect(styleValue(process, '.tab.on', 'color')).toBe('var(--color-primary)')
    expect(styleValue(process, '.tab.on', 'border-bottom')).toBe('4rpx solid var(--color-primary)')
    expect(styleValue(process, '.tip-error', 'color')).toBe('var(--color-error)')
    expect(styleValue(process, '.error-banner', 'background')).toBe('var(--color-error-tint)')
    expect(styleValue(process, '.item', 'border-bottom')).toBe('1px solid var(--color-divider)')
    expect(styleValue(process, '.fab', 'background')).toBe('var(--color-primary)')
    expect(process).not.toMatch(/#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})\b/)
  })

  it('contacts 页面：27 处硬编码色全部归一到语义 token', () => {
    expect(styleValue(contacts, '.page', 'background')).toBe('var(--color-bg-page)')
    expect(styleValue(contacts, '.search', 'background')).toBe('var(--color-surface)')
    expect(styleValue(contacts, '.input', 'border')).toBe('1px solid var(--color-border)')
    expect(styleValue(contacts, '.error-banner', 'background')).toBe('var(--color-error-tint)')
    expect(styleValue(contacts, '.item', 'border-bottom')).toBe('1px solid var(--color-divider)')
    expect(styleValue(contacts, '.avatar', 'background')).toBe('var(--color-primary)')
    expect(styleValue(contacts, '.mode', 'color')).toBe('var(--color-text-body)')
    expect(styleValue(contacts, '.mode.on', 'background')).toBe('var(--color-primary)')
    expect(styleValue(contacts, '.detail-empty', 'color')).toBe('var(--color-placeholder)')
    expect(contacts).not.toMatch(/#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})\b/)
  })

  it('meeting 页面：27 处硬编码色全部归一到语义 token', () => {
    expect(styleValue(meeting, '.btn', 'background')).toBe('var(--color-primary)')
    expect(styleValue(meeting, '.btn.ghost', 'color')).toBe('var(--color-text-body)')
    expect(styleValue(meeting, '.btn.ghost', 'border')).toBe('1px solid var(--color-border-strong)')
    expect(meeting).toContain('.btn[disabled] {\n  background: var(--color-placeholder);')
    expect(styleValue(meeting, '.tip-error', 'color')).toBe('var(--color-error)')
    expect(styleValue(meeting, '.error-banner', 'background')).toBe('var(--color-error-tint)')
    expect(styleValue(meeting, '.error-banner', 'border-bottom')).toBe('1px solid var(--color-divider)')
    expect(styleValue(meeting, '.code-dialog', 'background')).toBe('var(--color-surface)')
    expect(styleValue(meeting, '.code-title', 'color')).toBe('var(--color-text-primary)')
    expect(styleValue(meeting, '.code-body', 'color')).toBe('var(--color-primary)')
    expect(meeting).not.toMatch(/#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})\b/)
  })

  it('calendar 页面：25 处硬编码色全部归一到语义 token', () => {
    expect(styleValue(calendar, '.page', 'background')).toBe('var(--color-bg-page)')
    expect(styleValue(calendar, '.tab.on', 'background')).toBe('var(--color-primary)')
    expect(styleValue(calendar, '.time', 'color')).toBe('var(--color-primary)')
    expect(styleValue(calendar, '.card', 'background')).toBe('var(--color-surface)')
    expect(styleValue(calendar, '.title', 'color')).toBe('var(--color-text-primary)')
    expect(styleValue(calendar, '.sub', 'color')).toBe('var(--color-text-secondary)')
    expect(styleValue(calendar, '.tip-error', 'color')).toBe('var(--color-error)')
    expect(styleValue(calendar, '.retry-btn', 'border')).toBe('1rpx solid var(--color-error)')
    expect(styleValue(calendar, '.error-banner', 'background')).toBe('var(--color-error-tint)')
    expect(styleValue(calendar, '.c-submit', 'background')).toBe('var(--color-primary)')
    expect(calendar).not.toMatch(/#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})\b/)
  })

  it('attendance 页面：21 处硬编码色全部归一到语义 token（含状态色）', () => {
    expect(styleValue(attendance, '.page', 'background')).toBe('var(--color-bg-page)')
    expect(styleValue(attendance, '.btn', 'background')).toBe('var(--color-primary)')
    expect(styleValue(attendance, '.btn-out', 'background')).toBe('var(--color-warning)')
    expect(styleValue(attendance, '.tip-error', 'color')).toBe('var(--color-error)')
    expect(styleValue(attendance, '.error-banner', 'background')).toBe('var(--color-error-tint)')
    expect(styleValue(attendance, '.section', 'background')).toBe('var(--color-surface)')
    expect(attendance).toContain('color: var(--color-success)')
    expect(attendance).not.toMatch(/#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})\b/)
  })

  it('index 首页：21 处硬编码色全部归一到既有语义 token', () => {
    expect(styleValue(index, '.hello', 'color')).toBe('var(--color-text-primary)')
    expect(styleValue(index, '.sub', 'color')).toBe('var(--color-text-secondary)')
    expect(styleValue(index, '.stat-card', 'background')).toBe('var(--color-primary)')
    expect(styleValue(index, '.cell', 'background')).toBe('var(--color-surface)')
    expect(styleValue(index, '.cell-text', 'color')).toBe('var(--color-text-secondary)')
    expect(styleValue(index, '.sec-title', 'color')).toBe('var(--color-text-primary)')
    expect(styleValue(index, '.sec-error', 'color')).toBe('var(--color-error)')
    expect(styleValue(index, '.notice', 'border-bottom')).toBe('1px solid var(--color-divider)')
    expect(index).not.toMatch(/#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})\b/)
  })

  it('chat 页面：20 处硬编码色全部归一到语义 token（含离线提示色）', () => {
    expect(styleValue(chat, '.chat', 'background')).toBe('var(--color-bg-page)')
    expect(styleValue(chat, '.new-msg', 'background')).toBe('var(--color-primary)')
    expect(styleValue(chat, '.head', 'color')).toBe('var(--color-text-primary)')
    expect(styleValue(chat, '.offline-banner', 'background')).toBe('var(--color-warning-bg)')
    expect(styleValue(chat, '.offline-banner', 'color')).toBe('var(--color-warning-text)')
    expect(styleValue(chat, '.tip-error', 'color')).toBe('var(--color-error)')
    expect(styleValue(chat, '.load-older', 'color')).toBe('var(--color-primary)')
    expect(styleValue(chat, '.input', 'border')).toBe('1px solid var(--color-border)')
    expect(chat).not.toMatch(/#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})\b/)
  })

  it('两页面 style 仍为 scoped 纯 CSS，未偷偷引入 sass/lang', () => {
    for (const page of [bbs, login]) {
      expect(page).toContain('<style scoped>')
      expect(page).not.toContain('lang="scss"')
      expect(page).not.toContain('lang="sass"')
    }
  })
})
