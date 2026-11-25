// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R533: IMChat 会话项键盘可达契约 (WCAG 2.1 SC 2.1.1)
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'
import { describe, it, expect } from 'vitest'

const here = dirname(fileURLToPath(import.meta.url))
const source = readFileSync(join(here, 'IMChat.vue'), 'utf-8')

function convItemOpenTag(src: string): string {
  const start = src.indexOf('class="conv-item"')
  if (start < 0) return ''
  const lt = src.lastIndexOf('<', start)
  const gt = src.indexOf('>', start)
  if (lt < 0 || gt < 0) return ''
  return src.slice(lt, gt + 1)
}

function selectConversationBody(src: string): string {
  const start = src.indexOf('function selectConversation(conv: Conversation): void {')
  if (start < 0) return ''
  const end = src.indexOf('\n}', start)
  if (end < 0) return ''
  return src.slice(start, end + 2)
}

const tag = convItemOpenTag(source)
const body = selectConversationBody(source)

describe('R533 IMChat 会话项键盘可达 (SC 2.1.1)', () => {
  // ── 核心：真父 RED 必失败 ──────────────────────────────
  it('核心1 .conv-item 开标签具备 tabindex=0 可 Tab 聚焦', () => {
    expect(tag).toContain('tabindex="0"')
  })
  it('核心2 .conv-item 开标签 Enter 复用 selectConversation', () => {
    expect(tag).toContain('@keydown.enter="selectConversation(conv)"')
  })
  it('核心3 .conv-item 开标签 Space 复用 selectConversation 且阻止滚动', () => {
    expect(tag).toContain('@keydown.space.prevent="selectConversation(conv)"')
  })

  // ── 守卫：真父与修复后都应通过 ─────────────────────────
  it('守卫1 鼠标 @click=selectConversation(conv) 零改', () => {
    expect(tag).toContain('@click="selectConversation(conv)"')
  })
  it('守卫2 role=list 容器与 role=listitem 子项语义保留', () => {
    expect(source).toContain('<div class="conversation-list" role="list">')
    expect(tag).toContain('role="listitem"')
  })
  it('守卫3 selectConversation 函数体业务零改', () => {
    expect(body).toContain("if (callState.value !== 'idle') endCall(false)")
    expect(body).toContain('roomWs?.close()')
    expect(body).toContain('selectedChat.value = conv')
    expect(body).toContain('if (conv.unread > 0) markConversationRead(conv.id)')
    expect(body).toContain('wsClient.value?.joinRoom(conv.id)')
    expect(body).toContain('void loadConversationDetail(conv.id)')
    expect(body).not.toContain('keydown')
  })
  it('守卫4 会话列表业务 API 零改', () => {
    expect(source).toContain("api.get('/api/message/assemble/communicate/im/conversation/list/my')")
    expect(source).toContain('staleTime: 30 * 1000')
    expect(source).toContain('refetchInterval: 30_000')
  })
  it('守卫5 搜索框仅过滤行为零改', () => {
    expect(source).toContain('v-model="searchMsg"')
    expect(source).toContain('data-testid="im-conversation-search"')
  })
  it('守卫6 textarea 消息输入行为零改', () => {
    expect(source).toContain('@keydown.enter.ctrl="sendMessage"')
    expect(source).toContain('@keydown.enter.shift.exact=')
    expect(source).toContain('data-testid="im-message-input"')
  })
})
