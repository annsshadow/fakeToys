// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R542 RED→GREEN 契约：H5 消息页伪按钮键盘可达（WCAG 2.1 SC2.1.1 / 4.1.2）。
 * 四处 <view @tap> 补 role=button + tabindex=0 + aria-label + @keydown.enter/@keydown.space.prevent，
 * 复用 openConversation(row)/load 业务 handler；真父 39d8fd194 无属性 → 核心 RED，守卫真父亦通过。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')

const msg = read('pages/message/message.vue')
const main = read('main.ts')
const cal = read('pages/calendar/calendar.vue')

function linesWith(source: string, needle: string): string[] {
  return source.split('\n').filter((l: string) => l.includes(needle))
}
function lineWith(source: string, needle: string): string {
  const line = source.split('\n').find((l: string) => l.includes(needle))
  expect(line, '缺少锚点: ' + needle).toBeDefined()
  return line ?? ''
}

describe('R542 H5 消息页伪按钮键盘可达契约', () => {
  // ===== 核心：四处伪按钮 role/tabindex/aria-label/keydown（真父无 → RED）=====
  it('重试按钮×2 role=button/tabindex/aria-label 并复用 load 键盘事件', () => {
    const retries = linesWith(msg, 'class="retry-btn"')
    expect(retries.length).toBe(2)
    retries.forEach((l) => {
      expect(l).toContain('role="button"')
      expect(l).toContain('tabindex="0"')
      expect(l).toContain('aria-label="重试"')
      expect(l).toContain('@tap="load"')
      expect(l).toContain('@keydown.enter="load"')
      expect(l).toContain('@keydown.space.prevent="load"')
    })
  })

  it('空态刷新 role=button/tabindex/aria-label 并复用 load 键盘事件', () => {
    const refresh = lineWith(msg, 'class="empty-guide-link"')
    expect(refresh).toContain('role="button"')
    expect(refresh).toContain('tabindex="0"')
    expect(refresh).toContain('aria-label="刷新会话"')
    expect(refresh).toContain('@tap="load"')
    expect(refresh).toContain('@keydown.enter="load"')
    expect(refresh).toContain('@keydown.space.prevent="load"')
  })

  it('会话列表项 role=button/tabindex/动态 aria-label 并复用 openConversation(row)', () => {
    const item = lineWith(msg, 'class="item o2-list-item"')
    expect(item).toContain('role="button"')
    expect(item).toContain('tabindex="0"')
    expect(item).toContain(':aria-label="conversationTitle(row)"')
    expect(item).toContain('@tap="openConversation(row)"')
    expect(item).toContain('@keydown.enter="openConversation(row)"')
    expect(item).toContain('@keydown.space.prevent="openConversation(row)"')
    expect(item).toContain('v-for="(row, i) in rows"')
    expect(item).toContain(':key="i"')
  })

  // ===== 守卫：业务零改（真父亦须通过）=====
  it('守卫：openConversation / load 业务函数体逐字零改', () => {
    expect(msg).toContain('function openConversation(row: Record<string, unknown>) {')
    expect(msg).toContain('const id = conversationId(row)')
    expect(msg).toContain('if (!id) return')
    expect(msg).toContain('const name = conversationTitle(row)')
    expect(msg).toContain('uni.navigateTo')
    expect(msg).toContain('/pages/chat/chat?id=')
    expect(msg).toContain('encodeURIComponent(id)')
    expect(msg).toContain('const result = await loadConversations(() => messageApi.conversationList(), rows.value)')
    expect(msg).toContain('if (seq !== loadSeq) return')
    expect(msg).toContain('networkOffline.value = false')
  })

  it('守卫：conversationId/conversationTitle/isRawConversationName 显示层逻辑零改', () => {
    expect(msg).toContain('function conversationId(row: Record<string, unknown>): string {')
    expect(msg).toContain("for (const k of ['id', 'conversationId', 'uuid', 'flag'])")
    expect(msg).toContain('function conversationTitle(row: Record<string, unknown>): string {')
    expect(msg).toContain("for (const k of ['name', 'title', 'subject'])")
    expect(msg).toContain('单聊 · ')
    expect(msg).toContain("return '会话'")
  })

  it('守卫：原 @tap 鼠标行为与三态模板结构保留', () => {
    expect(linesWith(msg, '@tap=').length).toBeGreaterThanOrEqual(4)
    expect(msg).toContain('v-if="networkOffline"')
    expect(msg).toContain('v-if="loading && !loaded"')
    expect(msg).toContain('v-else-if="rows.length === 0 && loadError"')
    expect(msg).toContain('v-else-if="rows.length === 0"')
    expect(msg).toContain('v-else class="list"')
    expect(msg).toContain('v-if="loadError" class="tip-error error-banner"')
    expect(msg).toContain('class="title">{{ conversationTitle(row) }}</view>')
  })

  it('守卫：main.ts R537 增强器零改且未引入伪按钮全局增强器', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain("HOST_SELECTOR = 'uni-input, uni-textarea, uni-picker'")
    expect(main).toContain('installH5NativeInputNameA11y()')
    expect(main).not.toContain('o2-list-item')
    expect(main).not.toContain('keydown.enter')
  })

  it('守卫：calendar.vue:295 正面对照与 message 样式段零改', () => {
    expect(cal).toContain('<text class="calendar-create-btn ai-brand-fill" role="button" aria-label="新建日历" @tap="toggleCalendarForm">')
    const style = msg.slice(msg.indexOf('<style'))
    expect(style).not.toContain('role="button"')
    expect(style).not.toContain('tabindex=')
    expect(style).not.toContain('@keydown')
    expect(style).toContain('.retry-btn {')
    expect(style).toContain('.skeleton-row {')
  })
})
