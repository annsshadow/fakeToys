// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R302 t29 DR-04/DR-06: H5 消息空态引导与会话标题摘要显示契约（RED first）。
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const message = readFileSync(resolve(__dirname, './message.vue'), 'utf8')

describe('R302 t29 H5 消息空态与会话可读性', () => {
  it('空态含 empty-guide 文案与刷新入口', () => {
    expect(message).toMatch(/class="[^"]*empty-guide[^"]*"/)
    expect(message).toMatch(/暂无会话/)
    expect(message).toMatch(/class="empty-guide-link"[^>]*@tap="load"/)
  })
  it('会话列表显示姓名标题与摘要，不显示原始 id', () => {
    expect(message).toMatch(/class="title"[^>]*>\{\{\s*conversationTitle\(row\)\s*\}\}/)
    expect(message).toMatch(/class="meta"[^>]*>\{\{\s*\(row\.lastMessage as string\)\s*\|\|/)
    expect(message).toMatch(/function conversationTitle[\s\S]{0,300}return '会话'/)
  })
})
