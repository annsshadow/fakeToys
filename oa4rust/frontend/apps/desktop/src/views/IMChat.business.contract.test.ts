// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/** R296 P2：IM 默认视图必须是任务导向的三栏业务界面（会话列表 / 消息区 / 消息输入框），而非 /api/* 端点调试面板。 */
describe('IMChat 业务视图契约（R296 P2）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'IMChat.vue'), 'utf8')
  const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))

  it('提供会话列表 / 消息区 / 消息输入框三个业务区块', () => {
    expect(template).toContain('会话列表')
    expect(template).toContain('消息区')
    expect(template).toContain('消息输入框')
    for (const s of ['im-conversation-list', 'im-message-pane', 'im-message-composer']) {
      expect(template, `缺少业务区块: ${s}`).toContain(`data-testid="${s}"`)
    }
  })

  it('会话列表有搜索框和列表控件，可作为日常沟通入口', () => {
    expect(template).toContain('data-testid="im-conversation-search"')
    expect(template).toContain('data-testid="im-conversation-list"')
    expect(template).toContain('role="list"')
    expect(template).toMatch(/v-for="\(conv,\s*convIndex\) in filteredConversations"/)
    expect(template).toContain('data-testid="im-mark-all-read"')
  })

  it('消息区有消息记录容器和聊天对象名称', () => {
    expect(template).toContain('data-testid="im-message-pane"')
    expect(template).toContain('data-testid="im-chat-name"')
    expect(template).toContain('data-testid="im-message-list"')
    expect(template).toMatch(/v-for="msg in messages"/)
  })

  it('消息输入框是真实表单（文本域 + 提交按钮）', () => {
    const composer = template.slice(template.indexOf('<form class="message-input ai-panel"'))
    expect(composer).toContain('<form')
    expect(composer).toContain('<textarea')
    expect(composer).toContain('data-testid="im-message-input"')
    expect(composer).toMatch(/<button[^>]*type="submit"[^>]*data-testid="im-message-send"/)
  })

  it('默认视图不渲染 /api/* 端点调试按钮，副标题不暴露 API 路径', () => {
    expect(template).not.toContain('/api/')
    expect(template).not.toMatch(
      /@click="[^"]*(?:loadImStats|loadImMeta|loadImInstant|loadImArchive|loadMsgByType|loadMassMessages|imConvAction|imMsgAction|imMore)/,
    )
  })
})
