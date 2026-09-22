// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
/**
 * O2WebSocketClient 单测：用 FakeWebSocket 替代全局 WebSocket，钉死
 * P5 IM 协议语义——下行信封（sender/room）合并、$all 广播、非 JSON 回退、
 * 房间/ presence 上行帧形状，以及 send 仅在 OPEN 时发出的门控。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { addImListener, O2WebSocketClient, useWebSocket } from './websocket'

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

  // test helpers
  simulateOpen(): void {
    this.readyState = FakeWebSocket.OPEN
    this.onopen?.()
  }

  simulateMessage(raw: string): void {
    this.onmessage?.({ data: raw })
  }
}

let client: O2WebSocketClient

beforeEach(() => {
  FakeWebSocket.instances = []
  vi.stubGlobal('WebSocket', FakeWebSocket)
  vi.stubGlobal('window', { location: { protocol: 'https:', host: 'oa.example' } })
  client = new O2WebSocketClient()
})

afterEach(async () => {
  client.close()
  vi.unstubAllGlobals()
})

async function openClient(): Promise<FakeWebSocket> {
  const p = client.connect()
  const ws = FakeWebSocket.instances[0]
  ws.simulateOpen()
  await p
  return ws
}

describe('connect', () => {
  it('builds the wss URL from the page origin and resolves on open', async () => {
    const p = client.connect()
    const ws = FakeWebSocket.instances[0]
    expect(ws.url).toBe('wss://oa.example/ws/realtime')
    ws.simulateOpen()
    await p
    expect(client.connected).toBe(true)
  })

  it('an already-open client resolves immediately without a new socket', async () => {
    await openClient()
    client.connect()
    expect(FakeWebSocket.instances).toHaveLength(1)
  })

  it('close() drops the connection state', async () => {
    await openClient()
    client.close()
    expect(client.connected).toBe(false)
  })
})

describe('downstream message dispatch', () => {
  it('merges the envelope sender/room into the dispatched data (IM 消息可归因)', async () => {
    await openClient()
    const seen: unknown[] = []
    client.on('im_create', (d) => seen.push(d))
    FakeWebSocket.instances[0].simulateMessage(
      JSON.stringify({ type: 'im_create', data: { body: 'hi' }, sender: 'u-1', room: 'conv-9', timestamp: 1700000000 }),
    )
    expect(seen).toEqual([{ body: 'hi', sender: 'u-1', room: 'conv-9' }])
  })

  it('a $all listener is notified for every event with its event name', async () => {
    await openClient()
    const seen: unknown[] = []
    client.on('$all', (d) => seen.push(d))
    FakeWebSocket.instances[0].simulateMessage(JSON.stringify({ type: 'notification', data: { x: 1 } }))
    expect(seen[0]).toEqual({ event: 'notification', data: { x: 1 } })
  })

  it('non-JSON payloads fall back to the raw channel', async () => {
    await openClient()
    const seen: unknown[] = []
    client.on('raw', (d) => seen.push(d))
    FakeWebSocket.instances[0].simulateMessage('not-json')
    expect(seen).toEqual(['not-json'])
  })

  it('off() unsubscribes a handler', async () => {
    await openClient()
    const seen: unknown[] = []
    const handler = (d: unknown) => seen.push(d)
    client.on('presence', handler)
    client.off('presence', handler)
    FakeWebSocket.instances[0].simulateMessage(JSON.stringify({ type: 'presence', data: { online: [] } }))
    expect(seen).toEqual([])
  })
})

describe('upstream frames', () => {
  it('send() emits the o2server-compatible {channel,data,type:"message"} frame', async () => {
    const ws = await openClient()
    client.send('im_msg', { text: 'x' })
    expect(ws.sent).toHaveLength(1)
    expect(JSON.parse(ws.sent[0])).toEqual({
      channel: 'im_msg',
      data: { text: 'x' },
      type: 'message',
    })
  })

  it('joinRoom / leaveRoom emit room control frames', async () => {
    const ws = await openClient()
    client.joinRoom('conv-1')
    client.leaveRoom('conv-1')
    expect(JSON.parse(ws.sent[0])).toEqual({ channel: 'join_room', data: { room_id: 'conv-1' }, type: 'message' })
    expect(JSON.parse(ws.sent[1])).toEqual({ channel: 'leave_room', data: { room_id: 'conv-1' }, type: 'message' })
  })

  it('declarePresence emits the P5 presence frame with the sender identity', async () => {
    const ws = await openClient()
    client.declarePresence('u-1')
    expect(JSON.parse(ws.sent[0])).toEqual({ type: 'presence', data: { sender: 'u-1' } })
  })

  it('send() is a no-op while the socket is not OPEN', () => {
    client.connect() // 创建套接字但不模拟 open
    const ws = FakeWebSocket.instances[0]
    expect(ws.readyState).not.toBe(FakeWebSocket.OPEN)
    client.send('im_msg', { a: 1 })
    expect(ws.sent).toEqual([])
  })
})

describe('useWebSocket', () => {
  it('returns the shared client singleton', () => {
    expect(useWebSocket()).toBe(useWebSocket())
  })

  it('addImListener attaches the handler to the shared client (桌面 IMChat 的监听入口)', async () => {
    // addImListener 走 useWebSocket() 单例（非本文件的 client），需对单例单独连接。
    const shared = useWebSocket()
    const p = shared.connect()
    const ws = FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
    ws.simulateOpen()
    await p
    const seen: unknown[] = []
    addImListener('im_create', (d) => seen.push(d))
    ws.simulateMessage(JSON.stringify({ type: 'im_create', data: { body: 'hi' }, sender: 'u-9' }))
    expect(seen).toEqual([{ body: 'hi', sender: 'u-9' }])
    shared.close()
  })
})

describe('socket lifecycle (error / close / heartbeat / reconnect)', () => {
  it('connect() rejects with the socket error when the socket fails before opening', async () => {
    const p = client.connect()
    const ws = FakeWebSocket.instances[0]
    const err = new Error('boom')
    ws.onerror?.(err)
    await expect(p).rejects.toBe(err)
  })

  it('reopens a new socket 3s after an unexpected close', async () => {
    vi.useFakeTimers()
    const p = client.connect()
    const ws = FakeWebSocket.instances[0]
    ws.simulateOpen()
    await p

    ws.readyState = 3
    ws.onclose?.()
    expect(client.connected).toBe(false)

    vi.advanceTimersByTime(3000)
    expect(FakeWebSocket.instances).toHaveLength(2) // 重连开了新 socket
    const ws2 = FakeWebSocket.instances[1]
    ws2.simulateOpen()
    expect(client.connected).toBe(true)
    vi.useRealTimers()
  })

  it('close() cancels a pending reconnect (no new socket after 3s)', async () => {
    vi.useFakeTimers()
    const p = client.connect()
    const ws = FakeWebSocket.instances[0]
    ws.simulateOpen()
    await p

    ws.readyState = 3
    ws.onclose?.()
    client.close() // 用户主动断开：必须取消自动重连

    vi.advanceTimersByTime(3000)
    expect(FakeWebSocket.instances).toHaveLength(1)
    vi.useRealTimers()
  })

  it('sends a ping heartbeat every 30s while the socket stays open', async () => {
    vi.useFakeTimers()
    const p = client.connect()
    const ws = FakeWebSocket.instances[0]
    ws.simulateOpen()
    await p

    vi.advanceTimersByTime(30_000)
    expect(ws.sent[ws.sent.length - 1]).toBe(JSON.stringify({ type: 'ping' }))
    vi.useRealTimers()
  })

  it('keeps retrying when a reconnection attempt fails (error → reschedule loop)', async () => {
    vi.useFakeTimers()
    const p = client.connect()
    const ws = FakeWebSocket.instances[0]
    ws.simulateOpen()
    await p

    ws.readyState = 3
    ws.onclose?.() // 意外断开 → 3s 后重连

    await vi.advanceTimersByTimeAsync(3000) // 第一次重连开 ws2
    const ws2 = FakeWebSocket.instances[1]
    // ws2 的 connect() 是 Promise，onerror 触发 reject → .catch 是微任务，
    // 必须用 async 版 advance 冲掉微任务，第二次重连定时器才会被排上。
    ws2.onerror?.(new Error('still down'))
    await vi.advanceTimersByTimeAsync(3000)

    expect(FakeWebSocket.instances).toHaveLength(3)
    vi.useRealTimers()
  })

  it('a duplicate close event does not schedule a second reconnect (timer guard)', async () => {
    vi.useFakeTimers()
    const p = client.connect()
    const ws = FakeWebSocket.instances[0]
    ws.simulateOpen()
    await p

    ws.readyState = 3
    ws.onclose?.()
    ws.onclose?.() // 同一 socket 重复 close 事件：必须不产生双重重连

    vi.advanceTimersByTime(3000)
    expect(FakeWebSocket.instances).toHaveLength(2)
    vi.useRealTimers()
  })
})
