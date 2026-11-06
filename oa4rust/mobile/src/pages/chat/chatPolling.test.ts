// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { afterEach, describe, expect, it, vi } from 'vitest'
import {
  activeChatLifecycleCounts,
  CHAT_POLL_INTERVAL_MS,
  CHAT_POLL_MAX_INTERVAL_MS,
  mergeNewerMessages,
  nextPollDelayMs,
  startChatPolling,
} from './chatPolling'

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

  it('20 次进入/离开会话后，活跃轮询与定时器回到基线（R142）', () => {
    vi.useFakeTimers()
    const base = activeChatLifecycleCounts()
    for (let i = 0; i < 20; i++) {
      const handle = startChatPolling<Msg>({
        loadLatest: async () => [],
        onLatest: () => {},
      })
      expect(activeChatLifecycleCounts()).toEqual({ pollers: base.pollers + 1, timers: base.timers + 1 })
      handle.stop()
      // 重复 stop（例如 onHide 与 onUnload 同时触发）不得把计数减成负数。
      handle.stop()
      expect(activeChatLifecycleCounts()).toEqual(base)
    }
    expect(activeChatLifecycleCounts()).toEqual({ pollers: 0, timers: 0 })
  })

  it('停止时请求仍在途：定时器已归零，活跃轮询数只减一次', async () => {
    vi.useFakeTimers()
    const pending: { resolve: ((rows: Msg[]) => void) | null } = { resolve: null }
    const handle = startChatPolling<Msg>({
      intervalMs: CHAT_POLL_INTERVAL_MS,
      loadLatest: () =>
        new Promise<Msg[]>((resolve) => {
          pending.resolve = resolve
        }),
      onLatest: () => {},
    })
    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS)
    expect(activeChatLifecycleCounts()).toEqual({ pollers: 1, timers: 0 })

    handle.stop()
    expect(activeChatLifecycleCounts()).toEqual({ pollers: 0, timers: 0 })

    // 在途结果返回后不得重新排下一次，也不得改变计数。
    pending.resolve?.([])
    await vi.advanceTimersByTimeAsync(0)
    expect(activeChatLifecycleCounts()).toEqual({ pollers: 0, timers: 0 })
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

describe('弱网/断网降级：连续失败退避，成功即复位（R144）', () => {
  it('退避表 5→10→20→40→60 封顶，非法基数走 1s 下限', () => {
    expect([0, 1, 2, 3, 4, 5, 6].map((n) => nextPollDelayMs(n))).toEqual([
      5000,
      5000,
      10000,
      20000,
      40000,
      CHAT_POLL_MAX_INTERVAL_MS,
      CHAT_POLL_MAX_INTERVAL_MS,
    ])
    expect(nextPollDelayMs(2, 0)).toBe(2000)
    expect(nextPollDelayMs(9, 0)).toBe(CHAT_POLL_MAX_INTERVAL_MS)
  })

  it('连续失败按指数退避，封顶后不再增长；成功一次立即回到 5s', async () => {
    vi.useFakeTimers()
    const calls: number[] = []
    let failing = true
    const handle = startChatPolling<Msg>({
      intervalMs: CHAT_POLL_INTERVAL_MS,
      loadLatest: async () => {
        calls.push(Date.now())
        if (failing) throw new Error('Network error')
        return []
      },
      onLatest: () => {},
      onError: () => {},
    })

    // t=5s 首轮失败 → 5s；t=10s 二轮失败 → 10s；再到 20s → 40s → 60s（封顶）
    await vi.advanceTimersByTimeAsync(5000)
    await vi.advanceTimersByTimeAsync(5000)
    await vi.advanceTimersByTimeAsync(10000)
    await vi.advanceTimersByTimeAsync(20000)
    await vi.advanceTimersByTimeAsync(40000)
    await vi.advanceTimersByTimeAsync(60000)
    expect(calls.map((t) => (t - calls[0]) / 1000)).toEqual([0, 5, 15, 35, 75, 135])

    // 第 7 轮成功：复位后下一次只等 5s，而不是继续 60s。
    failing = false
    await vi.advanceTimersByTimeAsync(60000)
    expect(calls).toHaveLength(7)
    await vi.advanceTimersByTimeAsync(5000 - 1)
    expect(calls).toHaveLength(7)
    await vi.advanceTimersByTimeAsync(1)
    expect(calls).toHaveLength(8)
    handle.stop()
  })

  it('失败时请求仍在途也不叠加：任何时刻最多 1 个在途请求', async () => {
    vi.useFakeTimers()
    let attempts = 0
    let inFlight = 0
    let maxInFlight = 0
    const pending: { reject?: (error: unknown) => void } = {}
    const handle = startChatPolling<Msg>({
      intervalMs: CHAT_POLL_INTERVAL_MS,
      loadLatest: () => {
        attempts += 1
        inFlight += 1
        maxInFlight = Math.max(maxInFlight, inFlight)
        return new Promise<Msg[]>((_resolve, reject) => {
          pending.reject = (error) => {
            inFlight -= 1
            reject(error)
          }
        })
      },
      onLatest: () => {},
    })

    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS)
    expect(attempts).toBe(1)
    // 断网下请求一直挂着：等 30s 也不得再发一个。
    await vi.advanceTimersByTimeAsync(30000)
    expect(attempts).toBe(1)

    pending.reject?.(new Error('offline'))
    await vi.advanceTimersByTimeAsync(0)
    await vi.advanceTimersByTimeAsync(CHAT_POLL_INTERVAL_MS)
    expect(attempts).toBe(2)
    await vi.advanceTimersByTimeAsync(30000)
    expect(attempts).toBe(2)
    expect(maxInFlight).toBe(1)

    pending.reject?.(new Error('offline'))
    await vi.advanceTimersByTimeAsync(0)
    handle.stop()
  })

  it('onError 携带连续失败次数，供页面判定是否点亮断网提示', async () => {
    vi.useFakeTimers()
    const failures: number[] = []
    const handle = startChatPolling<Msg>({
      intervalMs: CHAT_POLL_INTERVAL_MS,
      loadLatest: async () => {
        throw new Error('offline')
      },
      onLatest: () => {},
      onError: (_error, consecutive) => failures.push(consecutive),
    })
    await vi.advanceTimersByTimeAsync(5000)
    await vi.advanceTimersByTimeAsync(5000)
    await vi.advanceTimersByTimeAsync(10000)
    expect(failures).toEqual([1, 2, 3])
    handle.stop()
  })
})
