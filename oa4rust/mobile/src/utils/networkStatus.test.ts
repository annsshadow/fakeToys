// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { afterEach, describe, expect, it, vi } from 'vitest'
import {
  isNetworkError,
  NETWORK_OFFLINE_TEXT,
  networkOffline,
  OFFLINE_FAILURE_THRESHOLD,
  setNetworkOnline,
  shouldMarkOffline,
  watchNetworkStatus,
} from './networkStatus'

afterEach(() => {
  setNetworkOnline(true)
  vi.restoreAllMocks()
})

describe('网络状态可见化（R144）', () => {
  it('系统上报 none 立即点亮，恢复 wifi 立即熄灭，清理函数解绑监听', () => {
    const listeners: Array<(res: { isConnected: boolean; networkType: string }) => void> = []
    const off = vi.fn()
    const stop = watchNetworkStatus({
      getNetworkType: ({ success }) => success?.({ networkType: 'none' }),
      onNetworkStatusChange: (listener) => listeners.push(listener),
      offNetworkStatusChange: off,
    })

    expect(networkOffline.value).toBe(true)
    expect(NETWORK_OFFLINE_TEXT).toContain('网络已断开')

    listeners[0]({ isConnected: true, networkType: 'wifi' })
    expect(networkOffline.value).toBe(false)
    listeners[0]({ isConnected: false, networkType: 'none' })
    expect(networkOffline.value).toBe(true)

    stop()
    expect(off).toHaveBeenCalledTimes(1)
  })

  it('平台缺 API 时不伪造「在线」，stop 也不抛', () => {
    setNetworkOnline(false)
    const stop = watchNetworkStatus({})
    expect(networkOffline.value).toBe(true)
    expect(() => stop()).not.toThrow()
  })

  it('连续失败阈值：1 次不点亮，2 次仅网络失败点亮，HTTP 500 不点亮', () => {
    const networkError = { status: 0, message: '网络连接失败' }
    expect(shouldMarkOffline(1, networkError)).toBe(false)
    expect(shouldMarkOffline(OFFLINE_FAILURE_THRESHOLD, networkError)).toBe(true)
    expect(shouldMarkOffline(5, { status: 500 })).toBe(false)
    expect(shouldMarkOffline(5, new Error('boom'))).toBe(false)

    expect(isNetworkError(networkError)).toBe(true)
    expect(isNetworkError({ status: 403 })).toBe(false)
    expect(isNetworkError(null)).toBe(false)
  })

  it('会话页与会话列表都渲染断网提示并订阅系统网络状态（源码契约）', () => {
    const dir = import.meta.dirname
    const chat = readFileSync(resolve(dir, '../pages/chat/chat.vue'), 'utf8')
    const message = readFileSync(resolve(dir, '../pages/message/message.vue'), 'utf8')

    for (const source of [chat, message]) {
      expect(source.split('\n').length).toBeGreaterThan(80)
      expect(source).toContain('watchNetworkStatus(uni as unknown as NetworkStatusApi)')
      expect(source).toContain('v-if="networkOffline"')
      expect(source).toContain('NETWORK_OFFLINE_TEXT')
      expect(source).toContain('.offline-banner')
      expect(source).toContain('unwatchNetwork?.()')
      expect(source).toContain('unwatchNetwork = null')
    }

    // 仅靠系统事件不够：服务器连不上但系统显示在线时，会话页必须由连续失败点亮。
    expect(chat).toContain('shouldMarkOffline(failures, err)')
    expect(chat).toContain('networkOffline.value = false')
    expect(chat).toContain('if (isNetworkError(e)) networkOffline.value = true')
  })
})
