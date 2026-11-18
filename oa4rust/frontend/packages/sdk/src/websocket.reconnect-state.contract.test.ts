// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
/**
 * WebSocket 意外断线自动重连后应用层状态重放契约（R260）：
 * IMChat 首次 connect().then() 里一次性声明 presence / 加入房间，
 * 而 SDK 的 scheduleReconnect() 直接调 connect()，绕过组件回调。
 * 一次网络闪断后客户端静默掉出所有房间（roster/回执/在线数全失效），
 * 但 connected=true 让 UI 误报成功。本契约钉死：
 * registerReconnectState 注册的状态在每次连接成功后重放，getter 读当前值；
 * clearReconnectState 后不再重放。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { O2WebSocketClient } from './websocket'

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

  simulateUnexpectedClose(): void {
    this.readyState = 3
    this.onclose?.()
  }
}

let client: O2WebSocketClient

beforeEach(() => {
  FakeWebSocket.instances = []
  vi.stubGlobal('WebSocket', FakeWebSocket)
  vi.stubGlobal('window', { location: { protocol: 'https:', host: 'oa.example' } })
  vi.spyOn(Math, 'random').mockReturnValue(1)
  client = new O2WebSocketClient()
})

afterEach(() => {
  client.close()
  vi.useRealTimers()
  vi.unstubAllGlobals()
  vi.restoreAllMocks()
})

async function openLatest(target: O2WebSocketClient = client): Promise<FakeWebSocket> {
  const p = target.connect()
  const ws = FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
  ws.simulateOpen()
  await p
  return ws
}

type SentFrame = { type?: string; channel?: string; data?: unknown }

function presenceFrames(ws: FakeWebSocket): unknown[] {
  return ws.sent.map((raw) => JSON.parse(raw) as SentFrame).filter((frame) => frame.type === 'presence')
}

function joinFrames(ws: FakeWebSocket): unknown[] {
  return ws.sent.map((raw) => JSON.parse(raw) as SentFrame).filter((frame) => frame.channel === 'join_room')
}

describe('意外断线重连重放应用层状态', () => {
  it('重连后自动重新声明身份并重新加入房间', async () => {
    // R260 缺口：registerReconnectState 尚不存在时，本行在旧代码上直接抛错 → RED。
    const t = await import('./websocket')
    const proto = t.O2WebSocketClient.prototype as unknown as { registerReconnectState?: unknown }
    expect(typeof proto.registerReconnectState).toBe('function')

    let currentSender: string | undefined = 'alice'
    let currentRoom: string | undefined = 'room-1'
    client.registerReconnectState({
      getSender: () => currentSender,
      getRoomId: () => currentRoom,
    })

    const ws1 = await openLatest()
    expect(presenceFrames(ws1)).toHaveLength(1)
    expect(joinFrames(ws1)).toHaveLength(1)

    // 运行中切换了会话：重连必须读 getter 当前值，不能回到 room-1。
    currentSender = 'bob'
    currentRoom = 'room-2'

    vi.useFakeTimers()
    ws1.simulateUnexpectedClose()
    // 退避 3s（random=1 取上限 3s），推进到重连并打开新连接。
    await vi.advanceTimersByTimeAsync(3000)
    const ws2 = FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
    expect(ws2).not.toBe(ws1)
    ws2.simulateOpen()
    await vi.advanceTimersByTimeAsync(0)

    // 旧代码：新连接 sent 为空 → 这两条断言失败（业务上 = 静默掉线、roster 失效）。
    expect(presenceFrames(ws2)).toEqual([{ type: 'presence', data: { sender: 'bob' } }])
    expect(joinFrames(ws2)).toEqual([{ channel: 'join_room', data: { room_id: 'room-2' }, type: 'message' }])
  })

  it('clearReconnectState 后重连不再重放', async () => {
    client.registerReconnectState({ getSender: () => 'alice', getRoomId: () => 'room-1' })
    const ws1 = await openLatest()
    expect(presenceFrames(ws1)).toHaveLength(1)

    client.clearReconnectState()
    vi.useFakeTimers()
    ws1.simulateUnexpectedClose()
    await vi.advanceTimersByTimeAsync(3000)
    const ws2 = FakeWebSocket.instances[FakeWebSocket.instances.length - 1]
    ws2.simulateOpen()
    await vi.advanceTimersByTimeAsync(0)

    expect(presenceFrames(ws2)).toHaveLength(0)
    expect(joinFrames(ws2)).toHaveLength(0)
  })
})
