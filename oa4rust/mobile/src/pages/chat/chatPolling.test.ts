// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { afterEach, describe, expect, it, vi } from 'vitest'
import { CHAT_POLL_INTERVAL_MS, mergeNewerMessages, startChatPolling } from './chatPolling'

interface Msg {
  id: string
  content?: string
  sender?: string
}

afterEach(() => {
  vi.useRealTimers()
})

describe('会话内新消息自动刷新（R140）', () => {
  it('最新一页按时间正序追加到末尾，服务器 id 去重', () => {
    const current: Msg[] = [
      { id: 'm-001', content: 'a', sender: 'u1' },
      { id: 'm-002', content: 'b', sender: 'u1' },
    ]
    // 后端顺序：新→旧；m-002 与当前页重叠。
    const latest: Msg[] = [
      { id: 'm-005', content: 'e', sender: 'u2' },
      { id: 'm-004', content: 'd', sender: 'u2' },
      { id: 'm-003', content: 'c', sender: 'u2' },
      { id: 'm-002', content: 'b', sender: 'u1' },
    ]
    const merged = mergeNewerMessages(current, latest)
    expect(merged.map((m) => m.id)).toEqual(['m-001', 'm-002', 'm-003', 'm-004', 'm-005'])
  })

  it('本地乐观占位行被服务器回显一对一替换，同一条消息不显示两次', () => {
    const current: Msg[] = [
      { id: 'm-001', content: 'old', sender: 'u1' },
      { id: 'local-1', content: 'hi', sender: 'me' },
    ]
    const latest: Msg[] = [
      { id: 'm-010', content: 'hi', sender: 'me' },
      { id: 'm-001', content: 'old', sender: 'u1' },
    ]
    const merged = mergeNewerMessages(current, latest)
    expect(merged.map((m) => m.id)).toEqual(['m-001', 'm-010'])
    expect(merged.filter((m) => m.content === 'hi')).toHaveLength(1)
  })

  it('同一句连发两次也按次数一对一替换，占位行不会残留', () => {
    const current: Msg[] = [
      { id: 'local-1', content: 'ok', sender: 'me' },
      { id: 'local-2', content: 'ok', sender: 'me' },
    ]
    const latest: Msg[] = [
      { id: 'm-011', content: 'ok', sender: 'me' },
      { id: 'm-010', content: 'ok', sender: 'me' },
    ]
    const merged = mergeNewerMessages(current, latest)
    expect(merged.map((m) => m.id)).toEqual(['m-010', 'm-011'])
    expect(merged).toHaveLength(2)
  })

  it('本地占位行缺发送者/内容时按空值匹配，不崩', () => {
    const current: Msg[] = [{ id: 'local-1' }]
    const latest: Msg[] = [{ id: 'm-001' }]
    const merged = mergeNewerMessages(current, latest)
    expect(merged.map((m) => m.id)).toEqual(['m-001'])
  })

  it('缺 id 的服务器行忽略，缺 id 的本地行原样保留', () => {
    const current: Msg[] = [
      { id: '', content: 'placeholder' },
      { id: 'm-001', content: 'a' },
    ]
    const latest: Msg[] = [
      { id: '', content: 'ghost' },
      { id: 'm-002', content: 'b' },
    ]
    const merged = mergeNewerMessages(current, latest)
    expect(merged.map((m) => m.id)).toEqual(['', 'm-001', 'm-002'])
  })

  it('轮询串行：上一次未完成不叠加请求，停止后在途结果不再写状态', async () => {
    vi.useFakeTimers()
    const calls: number[] = []
    const onLatest = vi.fn()
    const pending: { resolve: ((rows: Msg[]) => void) | null } = { resolve: null }
    const handle = startChatPolling<Msg>({
      intervalMs: CHAT_POLL_INTERVAL_MS,
      loadLatest: () => {
        calls.push(Date.now())
        return new Promise<Msg[]>((resolve) => {
          pending.resolve = resolve
        })
      },
      onLatest,
      onError: vi.fn(),
    })

    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS - 1)
    expect(calls).toHaveLength(0)
    await vi.advanceTimersByTimeAsync(1)
    expect(calls).toHaveLength(1)

    // 慢网：第一次请求 20s 未返回，期间不应再发请求。
    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS * 4)
    expect(calls).toHaveLength(1)

    pending.resolve?.([{ id: 'm-1' }])
    await vi.advanceTimersByTimeAsync(0)
    expect(onLatest).toHaveBeenCalledTimes(1)

    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS)
    expect(calls).toHaveLength(2)

    handle.stop()
    pending.resolve?.([{ id: 'm-2' }])
    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS * 5)
    expect(calls).toHaveLength(2)
    expect(onLatest).toHaveBeenCalledTimes(1)
  })

  it('行为闭环：会话内对方新消息在下一轮出现在列表末尾，并触发一次已读', async () => {
    vi.useFakeTimers()
    // 后端顺序：新→旧。进入会话时只有两条。
    const server: Msg[] = [
      { id: 'm-002', content: 'b', sender: 'u1' },
      { id: 'm-001', content: 'a', sender: 'u1' },
    ]
    let shown: Msg[] = mergeNewerMessages<Msg>([], server)
    const readCalls: string[] = []
    const handle = startChatPolling<Msg>({
      intervalMs: CHAT_POLL_INTERVAL_MS,
      loadLatest: async () => server.slice(0, 50),
      onLatest: (rows) => {
        const before = shown.length
        shown = mergeNewerMessages(shown, rows)
        if (shown.length !== before) readCalls.push('markRead')
      },
    })

    expect(shown.map((m) => m.id)).toEqual(['m-001', 'm-002'])
    expect(readCalls).toHaveLength(0)

    // 对方发来新消息（后端最新在前）。
    server.unshift({ id: 'm-003', content: 'from peer', sender: 'u2' })
    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS)
    expect(shown.map((m) => m.id)).toEqual(['m-001', 'm-002', 'm-003'])
    expect(shown[shown.length - 1]?.content).toBe('from peer')
    expect(readCalls).toHaveLength(1)

    // 没有新消息的轮次不应重复调用已读接口。
    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS)
    expect(readCalls).toHaveLength(1)
    handle.stop()
  })

  it('轮询失败只上报错误并继续下一轮，不中断也不打扰用户', async () => {
    vi.useFakeTimers()
    const errors: unknown[] = []
    let attempt = 0
    const handle = startChatPolling<Msg>({
      intervalMs: CHAT_POLL_INTERVAL_MS,
      loadLatest: async () => {
        attempt += 1
        if (attempt === 1) throw new Error('Network error')
        return [{ id: 'm-1' }]
      },
      onLatest: () => {},
      onError: (error) => errors.push(error),
    })

    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS)
    expect(errors).toHaveLength(1)
    expect((errors[0] as Error).message).toBe('Network error')

    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS)
    expect(attempt).toBe(2)
    handle.stop()
  })

  it('没有 onError 时失败同样被吞下（不给用户弹错），轮询继续', async () => {
    vi.useFakeTimers()
    let attempt = 0
    // 不传 intervalMs：走默认间隔。
    const handle = startChatPolling<Msg>({
      loadLatest: async () => {
        attempt += 1
        throw new Error('boom')
      },
      onLatest: () => {},
    })
    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS)
    expect(attempt).toBe(1)
    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS)
    expect(attempt).toBe(2)
    handle.stop()
  })

  it('停止后在途失败不再上报（页面已隐藏）', async () => {
    vi.useFakeTimers()
    const onError = vi.fn()
    const pending: { reject: ((error: unknown) => void) | null } = { reject: null }
    const handle = startChatPolling<Msg>({
      intervalMs: CHAT_POLL_INTERVAL_MS,
      loadLatest: () =>
        new Promise<Msg[]>((_resolve, reject) => {
          pending.reject = reject
        }),
      onLatest: () => {},
      onError,
    })
    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS)
    expect(onError).toHaveBeenCalledTimes(0)
    handle.stop()
    pending.reject?.(new Error('late'))
    await vi.advanceTimersByTimeAsync(0)
    expect(onError).toHaveBeenCalledTimes(0)
  })

  it('间隔下限 1s：误传 0/负数不会变成请求风暴', async () => {
    vi.useFakeTimers()
    const calls: number[] = []
    const handle = startChatPolling<Msg>({
      intervalMs: -5,
      loadLatest: async () => {
        calls.push(Date.now())
        return []
      },
      onLatest: () => {},
    })
    await vi.advanceTimersByTimeAsync(999)
    expect(calls).toHaveLength(0)
    await vi.advanceTimersByTimeAsync(1)
    expect(calls).toHaveLength(1)
    handle.stop()
  })
})
