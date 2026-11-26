// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R300 B2 工作台与个人域 AI 风格契约：四个高频页面必须消费 t1 的 --ai-* 令牌，
 * 保留全部业务 testid/表单，并把 t2 审计里的原始 ID、空态无引导、可见播报噪声修掉。
 * 仅读源码，不碰接口与业务逻辑。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string): string =>
  readFileSync(resolve(import.meta.dirname, name), 'utf8')

const dashboard = read('Dashboard.vue')
const personal = read('Personal.vue')
const settings = read('Settings.vue')
const im = read('IMChat.vue')

describe('R300 B2 工作台与个人域 AI 风格契约', () => {
  it('工作台欢迎/统计/空态消费 AI 玻璃、渐变、栅格与动效令牌', () => {
    expect(dashboard).toContain('class="welcome-bar glass-card ai-panel"')
    expect(dashboard).toMatch(/\.welcome-bar\s*\{[^}]*var\(--ai-gradient-surface\)/)
    expect(dashboard).toMatch(/\.welcome-bar\s*\{[^}]*var\(--ai-cmd-radius\)/)
    expect(dashboard).toMatch(/\.stat-card\s*\{[^}]*var\(--ai-grid-4\)/)
    expect(dashboard).toContain('var(--ai-motion-fast)')
    expect(dashboard).toContain('var(--ai-ease-spring)')
  })

  it('工作台待办不展示原始 ID/未知应用，统计加载中不当成零值', () => {
    const template = dashboard.slice(dashboard.indexOf('<template>'), dashboard.indexOf('</template>'))
    expect(template).not.toContain('未知应用')
    expect(template).toContain('item.appName || ')
    expect(template).not.toContain("s.value ?? '—'")
    expect(template).toContain('s.value ?? null')
    expect(template).toContain('stat-placeholder')
    expect(template).toContain('暂无待办，去流程中心发起')
  })

  it('个人中心标题、头像、主操作使用 AI 渐变与 spring 动效', () => {
    expect(personal).toContain('class="profile-header modern-hero ai-panel"')
    expect(personal).toMatch(/\.profile-header\s*\{[^}]*var\(--ai-gradient-surface\)/)
    expect(personal).toMatch(/\.avatar-large\s*\{[^}]*var\(--ai-gradient-brand\)/)
    expect(personal).toContain('var(--ai-ease-spring)')
    expect(personal).toContain('完善资料，让协作更顺畅')
  })

  it('设置页分组布局采用 AI 栅格间距，保存按钮用品牌渐变', () => {
    expect(settings).toContain('class="settings-layout ai-grid-2"')
    expect(settings).toMatch(/\.settings-layout\.ai-grid-2\s*\{[^}]*var\(--ai-grid-2\)/)
    expect(settings).toMatch(/\.save-btn\s*\{[^}]*var\(--ai-gradient-brand\)/)
    expect(settings).toContain('var(--ai-motion-fast)')
  })

  it('消息中心会话/气泡/发送器采用 AI 渐变玻璃，空态给出引导', () => {
    expect(im).toContain('class="im-view ai-grid-2"')
    expect(im).toMatch(/\.im-sidebar\s*\{[^}]*var\(--ai-glass-bg\)/)
    expect(im).toMatch(/\.conv-avatar\s*\{[^}]*var\(--ai-gradient-brand\)/)
    expect(im).toContain('.typing-indicator')
    expect(im).toContain('message-input ai-panel')
    expect(im).toContain('当前会话还没有消息，先从常用语开始')
  })

  it('无障碍播报在设置页只存在于 sr-only 区域，不产生可见文本', () => {
    expect(settings).toContain('data-a11y-hidden="true"')
    expect(settings).toContain('.sr-only')
    expect(settings).not.toContain('已切换到：')
  })
})
