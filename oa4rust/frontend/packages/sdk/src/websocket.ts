// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * WebSocket 封装
 * 替代 o2web 的 layout.desktop.socket
 */

/** 子路径部署时的 WebSocket 前缀（如 '/oa4rust-ws'）；默认空串保持根路径兼容。 */
let wsBasePath = ''

/** 设置 WebSocket URL 前缀，须在首次 connect() 前调用。 */
export function setWebSocketBase(base: string): void {
  wsBasePath = base.replace(/\/+$/, '')
}

type MessageHandler = (data: unknown) => void

/**
 * 自动重连成功后要重放的应用层状态。
 * 用 getter 而非快照值：会话可在运行时切换（selectConversation），重连时必须读取当前值，
 * 否则会把 presence / 房间恢复到过期会话。返回空串表示当前无该状态、跳过发送。
 */
export interface O2ReconnectState {
  getSender?: () => string | undefined
  getRoomId?: () => string | undefined
}
type EventMap = {
  im_create: MessageHandler
  im_revoke: MessageHandler
  im_conversation: MessageHandler
  notification: MessageHandler
  process_task: MessageHandler
  /** P5：房间在线快照下行 `{room, online: string[], count}` */
  presence: MessageHandler
  /** P5：IM 消息回执下行 `{delivered: number}` */
  receipt: MessageHandler
  [event: string]: MessageHandler
}

export interface WebSocketMessage {
  type: string
  data: unknown
  timestamp: number
  /** 下行信封（RealtimeMessage）：发送者身份（presence 声明后为声明值） */
  sender?: string
  /** 下行信封（RealtimeMessage）：所在房间 */
  room?: string
}

export class O2WebSocketClient {
  /** 重连退避起点（R135）：第一次意外断开后 3s 重连。 */
  private static readonly RECONNECT_BASE_MS = 3_000
  /** 重连退避上限：长断网时不再拉长等待，保证恢复后能及时重连。 */
  private static readonly RECONNECT_MAX_MS = 30_000

  private ws: WebSocket | null = null
  private url: string
  private handlers = new Map<string, Set<MessageHandler>>()
  /**
   * 应用层需要在每次连接（含意外断线后自动重连）成功后重放的状态。
   * 典型场景：IMChat 的在线身份声明（presence）与房间加入（join_room）。
   * 仅在 onopen 成功时重放，失败/未连接时不会发送，避免半连接状态。
   */
  private reconnectStates: O2ReconnectState[] = []
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null
  private heartbeatTimer: ReturnType<typeof setInterval> | null = null
  private _connected = false
  /** 主动关闭标记：close() 后 onclose 异步触发，不得再排重连（否则按退避间隔永远重连）。 */
  private intentionalClose = false
  /** 连续重连次数：成功打开即清零（R135）。 */
  private reconnectAttempts = 0

  constructor(url: string = '/ws/realtime') {
    this.url = url
  }

  get connected(): boolean {
    return this._connected
  }

  /** 连接 WebSocket */
  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      if (this.ws?.readyState === WebSocket.OPEN) {
        resolve()
        return
      }
      this.intentionalClose = false

      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:'
      const wsUrl = `${protocol}//${window.location.host}${wsBasePath}${this.url}`

      this.ws = new WebSocket(wsUrl)

      this.ws.onopen = () => {
        this._connected = true
        this.reconnectAttempts = 0
        this.startHeartbeat()
        this.replayReconnectState()
        resolve()
      }

      this.ws.onmessage = (event) => {
        try {
          const msg: WebSocketMessage = JSON.parse(event.data)
          // P5：合并下行信封字段（sender/room），使 IM 消息可归因到已声明身份。
          const dataObj = (msg.data ?? {}) as Record<string, unknown>
          const envelope = {
            sender: msg.sender,
            room: msg.room,
            ...dataObj,
          }
          this.dispatch(msg.type, envelope)
          // dispatch() 内部已向 $all 广播统一 {event,data} 信封；这里再显式发一次
          // 会让全局监听一帧收 3 次（正确信封 / 裸 data / 误导的 $all 信封）。
        } catch {
          // raw message, dispatch as-is
          this.dispatch('raw', event.data)
        }
      }

      this.ws.onerror = (event) => {
        reject(event)
      }

      this.ws.onclose = () => {
        this._connected = false
        this.stopHeartbeat()
        if (!this.intentionalClose) this.scheduleReconnect()
      }
    })
  }

  /** 添加事件监听 */
  on<T extends keyof EventMap>(event: T, handler: EventMap[T]): void {
    if (!this.handlers.has(event as string)) {
      this.handlers.set(event as string, new Set())
    }
    this.handlers.get(event as string)!.add(handler as MessageHandler)
  }

  /** 移除事件监听 */
  off<T extends keyof EventMap>(event: T, handler: EventMap[T]): void {
    this.handlers.get(event as string)?.delete(handler as MessageHandler)
  }

  /** 发送消息 */
  send(channel: string, data: unknown): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ channel, data, type: 'message' }))
    }
  }

  /** 加入房间 */
  joinRoom(roomId: string): void {
    this.send('join_room', { room_id: roomId })
  }

  /**
   * 注册连接成功后要重放的应用层状态（身份声明 + 房间加入）。
   * IMChat 首次 connect().then() 用本方法替代一次性回调，使 SDK 在
   * 每次自动重连成功后都能恢复 presence 与房间订阅。
   * 重复调用以最后一次注册为准（覆盖语义），不累积。
   */
  registerReconnectState(state: O2ReconnectState): void {
    this.reconnectStates = [state]
  }

  /** 清除已注册的重连状态；组件卸载显式调用（close 不自动清，保持订阅生命周期对称）。 */
  clearReconnectState(): void {
    this.reconnectStates = []
  }

  /** 连接成功后重放已注册状态；每次读取 getter 当前值，避免恢复到过期会话。 */
  private replayReconnectState(): void {
    for (const state of this.reconnectStates) {
      const sender = state.getSender?.()
      const roomId = state.getRoomId?.()
      if (sender) this.declarePresence(sender)
      if (roomId) this.joinRoom(roomId)
    }
  }

  /** 离开房间 */
  leaveRoom(roomId: string): void {
    this.send('leave_room', { room_id: roomId })
  }

  /** P5：声明在线身份（presence 上行）——后端据此登记房间 roster，
   *  并将该身份作为后续 IM 消息的 sender 标签。sender 一般传 session.user.unique。 */
  declarePresence(sender: string): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ type: 'presence', data: { sender } }))
    }
  }

  /** 断开连接 */
  close(): void {
    this.intentionalClose = true
    this.reconnectAttempts = 0
    this.stopHeartbeat()
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer)
      this.reconnectTimer = null
    }
    this.ws?.close()
    this.ws = null
    this._connected = false
    // 主动关闭（组件卸载 / 登出）彻底拆除订阅：IMChat 每次挂载都往单例塞入
    // 新的匿名 handler，若不清空，旧闭包（持有已卸载组件状态与 queryClient）
    // 会随消息重复触发。意外断线走 onclose → scheduleReconnect，不经过 close()，
    // 订阅照常保留。
    this.handlers.clear()
  }

  private dispatch(event: string, data: unknown): void {
    this.handlers.get(event)?.forEach((h) => {
      h(data)
    })
    this.handlers.get('$all')?.forEach((h) => {
      h({ event, data })
    })
  }

  private startHeartbeat(): void {
    this.heartbeatTimer = setInterval(() => {
      if (this.ws?.readyState === WebSocket.OPEN) {
        this.ws.send(JSON.stringify({ type: 'ping' }))
      }
    }, 30000)
  }

  private stopHeartbeat(): void {
    if (this.heartbeatTimer) {
      clearInterval(this.heartbeatTimer)
      this.heartbeatTimer = null
    }
  }

  /**
   * 下一次重连等待（R135）：3s 起步指数退避，30s 封顶；
   * 再叠加 [50%, 100%] 抖动，避免服务端重启后大量客户端在同一毫秒重连。
   * 抖动区间取半宽而非 [0, exp]：保证仍有下限进度，不会出现「几乎立刻重连」。
   */
  private nextReconnectDelay(): number {
    const backoff = Math.min(
      O2WebSocketClient.RECONNECT_BASE_MS * 2 ** this.reconnectAttempts,
      O2WebSocketClient.RECONNECT_MAX_MS,
    )
    this.reconnectAttempts += 1
    return backoff * (0.5 + Math.random() * 0.5)
  }

  private scheduleReconnect(): void {
    if (this.reconnectTimer) return
    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null
      this.connect().catch(() => this.scheduleReconnect())
    }, this.nextReconnectDelay())
  }
}

/** Composable 入口 */
let _wsClient: O2WebSocketClient | null = null

export function useWebSocket(url?: string): O2WebSocketClient {
  if (!_wsClient) {
    _wsClient = new O2WebSocketClient(url)
  }
  return _wsClient
}

/**
 * IM 专用 WebSocket 快捷方法
 * 对应 o2web 的 layout.desktop.socket.addImListener()
 */
export function addImListener(event: string, handler: MessageHandler): void {
  useWebSocket().on(event, handler)
}
