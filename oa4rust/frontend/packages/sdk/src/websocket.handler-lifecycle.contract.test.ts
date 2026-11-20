// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
/**
 * O2WebSocketClient 订阅生命周期契约（R198）：
 * IMChat 是路由页，每次挂载都往 useWebSocket() 单例塞入新的匿名 handler，
 * 卸载时只调用单例 close()。close() 若不清空 handlers，则旧闭包（捕获已卸载
 * 组件的 ref 与 queryClient）永久残留并在后续每条消息上重复触发。
 *
 * 本契约钉死两条互补语义：
 * 1. 主动 close()（组件卸载 / 登出）= 彻底拆除该客户端全部订阅；
 * 2. 意外断线自动重连（onclose → scheduleReconnect）= 保留订阅，消息照收。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { O2WebSocketClient, useWebSocket } from './websocket'

class FakeWebSocket {
  static readonly OPEN = 1
  static instances: FakeWebSocket[] = []
  url: string
  readyState = 3
  sent: string[] = []
  onopen: (() => void) | null = null
  onmessage: ((ev: { data: string }) => void) | null = null
  onclose: (() => void) | null = null
  onerror: ((ev: unknown) => void) | null = null

  constructor(url: string) {
    this.url = url
    FakeWebSocket.instances.push(this)
  }

  send(data: string): void {
    this.sent.push(data)
  }

  close(): void {
    this.readyState = 3
  }

  simulateOpen(): void {
    this.readyState = FakeWebSocket.OPEN
    this.onopen?.()
  }

  simulateMessage(raw: string): void {
    this.onmessage?.({ data: raw })
  }

  simulateUnexpectedClose(): void {
    this.readyState = 3
    this.onclose?.()
  }
}

let client: O2WebSocketClient
const imFrame = JSON.stringify({ type: 'im_create', data: { body: 'hi' }, sender: 'u-1' })

beforeEach(() => {
  FakeWebSocket.instances = []
  vi.stubGlobal('WebSocket', FakeWebSocket)
  vi.stubGlobal('window', { location: { protocol: 'https:', host: 'oa.example' } })
  vi.spyOn(Math, 'random').mockReturnValue(1)
  client = new O2WebSocketClient()
})

afterEach(async () => {
  client.close()
  useWebSocket().close()
  vi.unstubAllGlobals()
  vi.restoreAllMocks()
  vi.useRealTimers()
})

async function openClient(target: O2WebSocketClient = client): Promise<FakeWebSocket> {
  const p = target.connect()
  const ws = FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
  ws.simulateOpen()
  await p
  return ws
}

describe('close() tears down message subscriptions (R198)', () => {
  it('旧 handler 在 close() 后重新连接、再次收到消息时不再触发', async () => {
    const ws1 = await openClient()
    const stale: unknown[] = []
    client.on('im_create', (d) => stale.push(d))

    ws1.simulateMessage(imFrame)
    expect(stale).toHaveLength(1) // 前置 sanity：订阅确实生效

    client.close() // 模拟 IMChat 路由卸载

    const ws2 = await openClient()
    ws2.simulateMessage(imFrame)
    // 缺陷下 close() 不清 handlers，旧闭包仍被 dispatch → 这里会变成 2。
    expect(stale).toHaveLength(1)
  })

  it('close() 清空所有事件类型与 $all 广播的订阅', async () => {
    const ws1 = await openClient()
    const counts = { im: 0, presence: 0, receipt: 0, all: 0 }
    client.on('im_create', () => counts.im++)
    client.on('presence', () => counts.presence++)
    client.on('receipt', () => counts.receipt++)
    client.on('$all', () => counts.all++)

    ws1.simulateMessage(imFrame)
    expect(counts).toEqual({ im: 1, presence: 0, receipt: 0, all: 1 })

    client.close()
    const ws2 = await openClient()
    ws2.simulateMessage(imFrame)
    ws2.simulateMessage(JSON.stringify({ type: 'presence', data: { online: [] } }))
    ws2.simulateMessage(JSON.stringify({ type: 'receipt', data: { delivered: 1 } }))
    // 重新连接后如果不重新 on()，所有回调都应为 0；残留则证明泄漏。
    expect(counts).toEqual({ im: 1, presence: 0, receipt: 0, all: 1 })
  })

  it('单例跨路由重新挂载：旧挂载的 handler 不残留，新挂载的 handler 正常工作', async () => {
    const shared = useWebSocket()
    const ws1 = await openClient(shared)
    const stale: unknown[] = []
    shared.on('im_create', (d) => stale.push(d)) // 第一次进入 IMChat
    ws1.simulateMessage(imFrame)
    expect(stale).toHaveLength(1)

    shared.close() // 离开 IMChat 路由

    const ws2 = await openClient(shared)
    const fresh: unknown[] = []
    shared.on('im_create', (d) => fresh.push(d)) // 再次进入 IMChat
    ws2.simulateMessage(imFrame)

    expect(stale).toHaveLength(1) // 旧闭包不再触发（缺陷下会变成 2）
    expect(fresh).toHaveLength(1) // 新订阅照常工作
  })
})

describe('意外断线自动重连保留订阅（防止过度清理）', () => {
  it('onclose 触发的自动重连后，原有 handler 仍能收到消息', async () => {
    vi.useFakeTimers()
    try {
      const ws1 = await openClient()
      const seen: unknown[] = []
      client.on('im_create', (d) => seen.push(d))

      ws1.simulateMessage(imFrame)
      expect(seen).toHaveLength(1)

      // 意外断线（非主动 close）→ 自动重连，订阅应跨 socket 存活。
      ws1.simulateUnexpectedClose()
      await vi.advanceTimersByTimeAsync(3000)
      const ws2 = FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
      ws2.simulateOpen()

      ws2.simulateMessage(imFrame)
      expect(seen).toHaveLength(2) // 重连后仍收到消息
    } finally {
      vi.useRealTimers()
    }
  })
})
