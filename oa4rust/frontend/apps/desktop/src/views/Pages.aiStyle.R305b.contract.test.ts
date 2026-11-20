// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R305 P0B 契约：搜索/文件/消息三页深度消费 t1 的 --ai-* 令牌。
 * 三页都要有真实 ai-page 语义容器、玻璃/渐变/栅格/骨架/空态引导，
 * 同时保留既有业务 testid、搜索接口与 IMChat 的会话名/索引脱敏成果。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string): string => readFileSync(resolve(import.meta.dirname, name), 'utf8')
const search = read('SearchApp.vue')
const file = read('FileManager.vue')
const im = read('IMChat.vue')
const templateOf = (s: string): string => s.slice(s.indexOf('<template>'), s.indexOf('</template>'))
const styleOf = (s: string): string => s.slice(s.indexOf('<style'))
const aiVarCount = (s: string): number => (s.match(/var\(--ai-[a-z0-9-]+\)/g) || []).length
const hasAiRule = (style: string, selector: string, token: string): boolean => {
  let from = 0
  const needle = 'var(--ai-' + token + ')'
  while (from < style.length) {
    const at = style.indexOf(selector, from)
    if (at < 0) return false
    const open = style.indexOf('{', at)
    const close = style.indexOf('}', open)
    if (open > at && close > open && style.slice(open, close).includes(needle)) return true
    from = close > open ? close + 1 : at + selector.length
  }
  return false
}

describe('R305 P0B 搜索/文件/消息 AI 令牌深度消费契约', () => {
  it('全局搜索页采用真实 ai-page 容器、玻璃面板与品牌渐变，骨架和空态有 AI 引导', () => {
    const tpl = templateOf(search)
    const css = styleOf(search)
    expect(tpl).toContain('class="search-view modern-page ai-page"')
    expect(tpl).toContain('class="view-header glass-card search-header ai-page ai-panel"')
    expect(tpl).toContain('data-testid="search-loading-skeleton"')
    expect(tpl).toContain('ai-skeleton-line')
    expect(tpl).toContain('输入关键词开始搜索')
    expect(tpl).toContain('试试“请假”、“文件”或同事姓名，快速定位业务上下文')
    expect(hasAiRule(css, '.search-view', 'grid-2')).toBe(true)
    expect(hasAiRule(css, '.view-header', 'gradient-surface')).toBe(true)
    expect(hasAiRule(css, '.search-panel', 'glass-bg')).toBe(true)
    expect(hasAiRule(css, '.btn-search', 'gradient-brand')).toBe(true)
    expect(hasAiRule(css, '.result-item', 'glass-border')).toBe(true)
    expect(search).toContain('var(--ai-motion-fast)')
    expect(search).toContain('var(--ai-ease-spring)')
    expect(aiVarCount(search)).toBeGreaterThanOrEqual(5)
    for (const id of ['search-business-view', 'search-header', 'search-panel', 'search-form', 'search-results-tabs']) {
      expect(tpl).toContain('data-testid="' + id + '"')
    }
    expect(search).toContain("api.post('/api/queryview/search'")
  })

  it('文件管理页把既有现代化外壳补成双 ai-page 容器并令牌化骨架/栅格', () => {
    const tpl = templateOf(file)
    const css = styleOf(file)
    expect(tpl).toContain('class="file-view modern-page ai-page ai-panel"')
    expect(tpl).toContain('class="modern-hero ai-panel ai-page"')
    expect(tpl).toContain('class="skeleton-row ai-skeleton-line"')
    expect(tpl).toContain('📤 上传文件，开始协作')
    expect(hasAiRule(css, '.file-view', 'grid-2')).toBe(true)
    expect(hasAiRule(css, '.modern-hero', 'gradient-surface')).toBe(true)
    expect(hasAiRule(css, '.skeleton-row', 'skeleton-base')).toBe(true)
    expect(file).toContain('var(--ai-ease-spring)')
    expect(aiVarCount(file)).toBeGreaterThanOrEqual(5)
    for (const id of ['file-create-json', 'files-mine-list', 'files-recyclable-list', 'files-shared-list', 'files-recycle-list']) {
      expect(tpl).toContain('data-testid="' + id + '"')
    }
  })

  it('消息中心保留会话脱敏成果，并补齐双 ai-page 容器与真实令牌消费', () => {
    const tpl = templateOf(im)
    const css = styleOf(im)
    expect(tpl).toContain('class="im-view ai-grid-2"')
    expect(tpl).toContain('class="im-sidebar glass-card ai-page"')
    expect(tpl).toContain('class="im-main glass-card ai-page"')
    expect(im).toContain(":data-testid=\"'im-conv-' + (convIndex + 1)\"")
    expect(tpl).toContain('displayConversationName(conv)')
    expect(tpl).toContain('displayConversationName(selectedChat)')
    expect(tpl).toContain('当前会话还没有消息，先从常用语开始')
    expect(hasAiRule(css, '.im-sidebar', 'glass-bg')).toBe(true)
    expect(hasAiRule(css, '.im-main', 'gradient-surface')).toBe(true)
    expect(hasAiRule(css, '.message-list', 'skeleton-base')).toBe(true)
    expect(hasAiRule(css, '.no-chat-placeholder', 'grid-2')).toBe(true)
    expect(im).toContain('var(--ai-motion-fast)')
    expect(im).toContain('var(--ai-ease-spring)')
    expect(aiVarCount(im)).toBeGreaterThanOrEqual(5)
    for (const id of ['im-root', 'im-conversation-list', 'im-message-composer', 'im-message-pane']) {
      expect(tpl).toContain('data-testid="' + id + '"')
    }
  })
})
