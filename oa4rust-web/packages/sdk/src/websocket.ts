import { api, type ApiResponse } from './api.js';

/**
 * WebSocket 封装
 * 替代 o2web 的 layout.desktop.socket
 */

type MessageHandler = (data: unknown) => void;
type EventMap = {
  im_create: MessageHandler;
  im_revoke: MessageHandler;
  im_conversation: MessageHandler;
  notification: MessageHandler;
  process_task: MessageHandler;
  [event: string]: MessageHandler;
};

export interface WebSocketMessage {
  type: string;
  data: unknown;
  timestamp: number;
}

export class O2WebSocketClient {
  private ws: WebSocket | null = null;
  private url: string;
  private handlers = new Map<string, Set<MessageHandler>>();
  private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
  private heartbeatTimer: ReturnType<typeof setInterval> | null = null;
  private _connected = false;

  constructor(url: string = '/ws/realtime') {
    this.url = url;
  }

  get connected(): boolean {
    return this._connected;
  }

  /** 连接 WebSocket */
  connect(): Promise<void> {
    return new Promise((resolve, reject) => {
      if (this.ws?.readyState === WebSocket.OPEN) {
        resolve();
        return;
      }

      const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
      const wsUrl = `${protocol}//${window.location.host}${this.url}`;

      this.ws = new WebSocket(wsUrl);

      this.ws.onopen = () => {
        this._connected = true;
        this.startHeartbeat();
        resolve();
      };

      this.ws.onmessage = (event) => {
        try {
          const msg: WebSocketMessage = JSON.parse(event.data);
          this.dispatch(msg.type, msg.data);
          this.dispatch('$all', msg.data);
        } catch {
          // raw message, dispatch as-is
          this.dispatch('raw', event.data);
        }
      };

      this.ws.onerror = (event) => {
        reject(event);
      };

      this.ws.onclose = () => {
        this._connected = false;
        this.stopHeartbeat();
        this.scheduleReconnect();
      };
    });
  }

  /** 添加事件监听 */
  on<T extends keyof EventMap>(event: T, handler: EventMap[T]): void {
    if (!this.handlers.has(event as string)) {
      this.handlers.set(event as string, new Set());
    }
    this.handlers.get(event as string)!.add(handler as MessageHandler);
  }

  /** 移除事件监听 */
  off<T extends keyof EventMap>(event: T, handler: EventMap[T]): void {
    this.handlers.get(event as string)?.delete(handler as MessageHandler);
  }

  /** 发送消息 */
  send(channel: string, data: unknown): void {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ channel, data, type: 'message' }));
    }
  }

  /** 加入房间 */
  joinRoom(roomId: string): void {
    this.send('join_room', { room_id: roomId });
  }

  /** 离开房间 */
  leaveRoom(roomId: string): void {
    this.send('leave_room', { room_id: roomId });
  }

  /** 断开连接 */
  close(): void {
    this.stopHeartbeat();
    if (this.reconnectTimer) clearTimeout(this.reconnectTimer);
    this.ws?.close();
    this.ws = null;
    this._connected = false;
  }

  private dispatch(event: string, data: unknown): void {
    this.handlers.get(event)?.forEach((h) => h(data));
    this.handlers.get('$all')?.forEach((h) => h({ event, data }));
  }

  private startHeartbeat(): void {
    this.heartbeatTimer = setInterval(() => {
      if (this.ws?.readyState === WebSocket.OPEN) {
        this.ws.send(JSON.stringify({ type: 'ping' }));
      }
    }, 30000);
  }

  private stopHeartbeat(): void {
    if (this.heartbeatTimer) {
      clearInterval(this.heartbeatTimer);
      this.heartbeatTimer = null;
    }
  }

  private scheduleReconnect(): void {
    if (this.reconnectTimer) return;
    this.reconnectTimer = setTimeout(() => {
      this.reconnectTimer = null;
      this.connect().catch(() => this.scheduleReconnect());
    }, 3000);
  }
}

/** Composable 入口 */
let _wsClient: O2WebSocketClient | null = null;

export function useWebSocket(url?: string): O2WebSocketClient {
  if (!_wsClient) {
    _wsClient = new O2WebSocketClient(url);
  }
  return _wsClient;
}

/**
 * IM 专用 WebSocket 快捷方法
 * 对应 o2web 的 layout.desktop.socket.addImListener()
 */
export function addImListener(event: string, handler: MessageHandler): void {
  useWebSocket().on(event, handler);
}
