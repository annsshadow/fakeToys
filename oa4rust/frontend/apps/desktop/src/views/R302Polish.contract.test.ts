// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R302 t29 DR-04/DR-06: Dashboard 标题语义与会话 testid 脱敏的源码契约（RED first）。
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const dashboard = readFileSync(resolve(__dirname, './Dashboard.vue'), 'utf8')
const imchat = readFileSync(resolve(__dirname, './IMChat.vue'), 'utf8')

describe('R302 t29 Dashboard 语义与会话标识脱敏', () => {
  it('DR-04 工作台必须只有一个 h1 并显式标记 aiPage', () => {
    expect(dashboard.match(/<h1\b/g) ?? []).toHaveLength(1)
    expect(dashboard).toContain('data-testid="dashboard-h1"')
    expect(dashboard).toContain('ai-page-dashboard')
    expect(dashboard).toContain('ai-gradient-text')
  })

  it('DR-06 会话列表 data-testid 不得含原始会话 id', () => {
    expect(imchat).toContain('v-for="(conv, convIndex) in filteredConversations"')
    expect(imchat).toContain(`:data-testid="'im-conv-' + (convIndex + 1)"`)
    expect(imchat).not.toContain(`:data-testid="'im-conv-' + conv.id"`)
  })

  it('DR-06 会话行显示可读名称与摘要而非原始 id', () => {
    expect(imchat).toContain('{{ displayConversationName(conv) }}')
    expect(imchat).toContain('{{ displayConversationPreview(conv) }}')
  })
})
