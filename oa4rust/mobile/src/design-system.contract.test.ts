// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/** 移动端「与桌面统一设计标准」契约：令牌、无障碍守卫与性能优化落地。 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = (relative: string): string => readFileSync(resolve(import.meta.dirname, relative), 'utf8')
const app = src('App.vue')
const scss = src('uni.scss')
const bbs = src('pages/bbs/bbs.vue')
const index = src('pages/index/index.vue')

describe('移动端设计系统令牌（与桌面 tokens.css 同名同值）', () => {
  it('App.vue 提供间距/字号/圆角/阴影/动效/层级/触控/断点增补令牌', () => {
    for (const token of [
      '--space-0',
      '--space-4',
      '--space-8',
      '--space-16',
      '--font-size-xs',
      '--font-size-base',
      '--font-size-3xl',
      '--radius-none',
      '--radius-md',
      '--radius-lg',
      '--radius-full',
      '--shadow-card-mobile',
      '--shadow-panel',
      '--transition-fast',
      '--duration-normal',
      '--ease-out',
      '--z-dropdown',
      '--z-modal',
      '--z-toast',
      '--control-height-md',
      '--touch-target-min',
      '--breakpoint-md',
    ]) {
      expect(app, token).toMatch(new RegExp(token.replace(/[-]/g, '\\-') + '\\s*:'))
    }
  })

  it('uni.scss 提供同名 Sass 编译期镜像，色彩仍由 App.vue 单点维护', () => {
    for (const token of ['$space-4', '$font-size-base', '$radius-lg', '$shadow-card-mobile', '$touch-target-min']) {
      expect(scss).toContain(token)
    }
    expect(scss).toContain('$brand-primary: #2d8cf0')
  })

  it('全局无障碍守卫：键盘焦点环、44px 触控目标、reduced-motion', () => {
    expect(app).toContain(':focus-visible')
    expect(app).toContain('min-height: var(--touch-target-min)')
    expect(app).toContain('@media (prefers-reduced-motion: reduce)')
  })

  it('提供与桌面 UnoCSS shortcut 同名的 o2-* 全局组件类', () => {
    for (const cls of ['o2-card', 'o2-btn', 'o2-input', 'o2-tag', 'o2-divider', 'o2-list-item', 'o2-skeleton']) {
      expect(app).toContain('.' + cls)
    }
  })
})

describe('长列表渲染优化', () => {
  it('论坛主题列表与首页公告使用可视区渲染类', () => {
    expect(bbs).toContain('class="topic o2-list-item"')
    expect(index).toContain('class="notice o2-list-item"')
    expect(app).toContain('content-visibility: auto')
    expect(app).toContain('contain-intrinsic-size')
  })
})

describe('R294 空状态与交互反馈（H5 美化迭代）', () => {
  const emptyPages = [
    src('pages/message/message.vue'),
    src('pages/bbs/bbs.vue'),
    src('pages/meeting/meeting.vue'),
    src('pages/contacts/contacts.vue'),
    src('pages/process/process.vue'),
  ]

  it('列表空状态统一为 o2-empty（图标 + 主文案），替代单行灰字', () => {
    expect(app).toContain('.o2-empty')
    expect(app).toContain('.o2-empty-icon')
    for (const page of emptyPages) {
      // R304：允许修饰类（如 class="o2-empty empty-guide"），但 o2-empty 必须是独立 class token。
      expect(page).toMatch(/class="[^"]*\bo2-empty\b[^"]*"/)
      expect(page).toContain('o2-empty-icon')
    }
  })

  it('列表项与按钮提供按压反馈态，动效走统一令牌', () => {
    expect(app).toContain('.o2-list-item:active')
    expect(app).toContain('.o2-btn:active')
    expect(app).toContain('transition: background-color var(--transition-fast) var(--ease-out)')
  })
})
