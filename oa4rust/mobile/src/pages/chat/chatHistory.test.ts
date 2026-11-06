// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { describeChatError, loadChatHistoryPage, mergeOlderMessages } from './chatHistory'

interface Msg {
  id: string
  conversationId: string
  content: string
}

/** 450 条消息，后端顺序为最新在前（m-450 … m-001）。 */
function fakeHistoryBackend(total = 450) {
  const all: Msg[] = Array.from({ length: total }, (_, i) => {
    const seq = total - i
    return { id: `m-${String(seq).padStart(3, '0')}`, conversationId: 'c-1', content: `消息 ${seq}` }
  })
  const calls: Array<{ conv: string; page: number; size: number }> = []
  const load = async (conv: string, page: number, size: number) => {
    calls.push({ conv, page, size })
    const start = (page - 1) * size
    return { data: all.slice(start, start + size) }
  }
  return { load, calls }
}

describe('移动端会话历史分页：页码进请求、更早消息可加载且到底可见（R108）', () => {
  it('450 条消息连续加载 3 页：页码递增、按时间正序拼接、无重复、末页到底', async () => {
    const { load, calls } = fakeHistoryBackend(450)

    const first = await loadChatHistoryPage<Msg>(load, 'c-1', 1)
    expect(first.messages[0]?.id).toBe('m-450')
    expect(first.messages).toHaveLength(200)
    expect(first.hasMore).toBe(true)

    let ascending = mergeOlderMessages<Msg>([], first.messages)
    expect(ascending[0]?.id).toBe('m-251')
    expect(ascending[ascending.length - 1]?.id).toBe('m-450')

    const second = await loadChatHistoryPage<Msg>(load, 'c-1', 2)
    ascending = mergeOlderMessages(ascending, second.messages)
    expect(ascending[0]?.id).toBe('m-051')
    expect(ascending).toHaveLength(400)
    expect(second.hasMore).toBe(true)

    const third = await loadChatHistoryPage<Msg>(load, 'c-1', 3)
    ascending = mergeOlderMessages(ascending, third.messages)
    expect(ascending).toHaveLength(450)
    expect(new Set(ascending.map((m) => m.id)).size).toBe(450)
    expect(ascending[0]?.id).toBe('m-001')
    expect(ascending[ascending.length - 1]?.id).toBe('m-450')
    expect(third.hasMore).toBe(false)

    // 页码真的进了请求：旧实现固定 (conv, 1, 200)。
    expect(calls.map((c) => c.page)).toEqual([1, 2, 3])
    expect(calls.every((c) => c.size === 200)).toBe(true)
    expect(calls.every((c) => c.conv === 'c-1')).toBe(true)
  })

  it('分页重叠（新消息插入导致 OFFSET 位移）按 id 去重，不产生重复行', () => {
    const { load } = fakeHistoryBackend(450)
    void load
    const current = Array.from({ length: 200 }, (_, i) => ({
      id: `m-${String(251 + i).padStart(3, '0')}`,
      conversationId: 'c-1',
      content: '',
    }))
    // 期间新到 5 条消息，OFFSET 位移后第 2 页为 m-255 … m-051，
    // 其中 m-255 … m-251 与当前页重叠。
    const overlapping = Array.from({ length: 205 }, (_, i) => ({
      id: `m-${String(255 - i).padStart(3, '0')}`,
      conversationId: 'c-1',
      content: '',
    }))
    const merged = mergeOlderMessages(current, overlapping)
    expect(merged).toHaveLength(400)
    expect(new Set(merged.map((m) => m.id)).size).toBe(400)
    expect(merged[0]?.id).toBe('m-051')
  })

  it('响应体缺失或形状异常时按空页处理，不把 undefined 当消息列表', async () => {
    const missing = await loadChatHistoryPage<Msg>(async () => undefined, 'c-1', 1)
    expect(missing.messages).toEqual([])
    expect(missing.hasMore).toBe(false)

    const malformed = await loadChatHistoryPage<Msg>(async () => ({ data: { total: 0 } }), 'c-1', 1)
    expect(malformed.messages).toEqual([])
    expect(malformed.hasMore).toBe(false)
  })

  it('缺 id 的消息不参与去重判定，也不会挡住真正带 id 的更早消息', () => {
    // 历史数据可能含 id 为空的消息：它无法判重，既不能塞进 seen，
    // 也不能让同一条更早消息因为「已被占位」而丢失。
    const current: Msg[] = [
      { id: '', conversationId: 'c-1', content: '占位' },
      { id: 'm-002', conversationId: 'c-1', content: '' },
    ]
    const older: Msg[] = [
      { id: 'm-002', conversationId: 'c-1', content: '' },
      { id: 'm-001', conversationId: 'c-1', content: '' },
    ]
    const merged = mergeOlderMessages(current, older)
    expect(merged).toHaveLength(3)
    expect(merged[0]?.id).toBe('m-001')
    expect(merged.filter((m) => m.id === 'm-002')).toHaveLength(1)
  })

  it('页码夹取与错误文案：非法页码按 1 请求，非 Error 也有可见提示', async () => {
    const { load, calls } = fakeHistoryBackend(200)
    await loadChatHistoryPage<Msg>(load, 'c-1', 0)
    await loadChatHistoryPage<Msg>(load, 'c-1', -5)
    expect(calls.map((c) => c.page)).toEqual([1, 1])
    expect(describeChatError('消息加载失败', new Error('Network error'))).toBe('消息加载失败：Network error')
    expect(describeChatError('历史消息加载失败', 'boom')).toBe('历史消息加载失败：未知错误')
  })

  it('页面提供加载更早入口、到底提示与失败重试', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'chat.vue'), 'utf8')
    // 规模下限：防止文件被截断后断言空转。
    expect(source.split('\n').length).toBeGreaterThan(200)
    expect(source).toContain("from './chatHistory'")
    expect(source).toContain('loadChatHistoryPage')
    expect(source).toContain('mergeOlderMessages')
    expect(source).toContain('加载更早消息')
    expect(source).toContain('没有更早的消息了')
    expect(source).toContain('historyError')
    expect(source).toContain('重试')
    // 旧实现：固定第 1 页 200 条，超出部分静默丢失。
    expect(source).not.toContain('msgHistory(conversationId.value, 1, 200)')
  })

  it('自动 markRead 失败保持静默，但必须留可诊断痕迹（R139）', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'chat.vue'), 'utf8')
    const start = source.indexOf('await messageApi.markRead')
    expect(start).toBeGreaterThan(-1)
    const block = source.slice(start, start + 220)
    // 不能是空 catch：静默到没有任何痕迹会让线上无法排查。
    expect(block).toContain('catch (err)')
    expect(block).toContain('console.warn(')
    expect(block).toContain('markRead failed')
    expect(block).not.toContain('/* noop */')
  })
})
