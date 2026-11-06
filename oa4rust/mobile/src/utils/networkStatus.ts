// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 移动端网络状态可见化（R144）。
 *
 * 旧实现里轮询失败只留 console.warn：断网时用户看到的是一个「永远不变」的会话页，
 * 不知道是没人说话还是网络断了。这里维护一个全局只读状态 `networkOffline`，
 * 由系统网络事件**和**连续失败共同驱动，任意一次成功请求都会复位。
 */

import { ref } from 'vue'

export const NETWORK_OFFLINE_TEXT = '网络已断开，正在等待恢复…'

/** 连续失败达到该次数才点亮断网提示：单次抖动不值得打扰用户。 */
export const OFFLINE_FAILURE_THRESHOLD = 2

export const networkOffline = ref(false)

export interface NetworkStatusApi {
  getNetworkType?: (options: { success?: (res: { networkType: string }) => void }) => void
  onNetworkStatusChange?: (listener: (res: { isConnected: boolean; networkType: string }) => void) => void
  offNetworkStatusChange?: (listener: (res: { isConnected: boolean; networkType: string }) => void) => void
}

export function setNetworkOnline(online: boolean): void {
  networkOffline.value = !online
}

export function applyNetworkType(networkType: string | undefined): void {
  setNetworkOnline(networkType !== 'none')
}

/** 传输层网络失败统一为 status 0（见 services/http.ts 的 fail 分支）。 */
export function isNetworkError(error: unknown): boolean {
  return (error as { status?: unknown } | null | undefined)?.status === 0
}

/** 是否应该点亮「网络已断开」：连续失败达到阈值且这次确实是网络失败。 */
export function shouldMarkOffline(consecutiveFailures: number, error: unknown): boolean {
  return consecutiveFailures >= OFFLINE_FAILURE_THRESHOLD && isNetworkError(error)
}

/**
 * 订阅系统网络状态；返回清理函数。平台缺 API 时保持当前值，不伪造「在线」。
 */
export function watchNetworkStatus(api: NetworkStatusApi): () => void {
  const onChange = (res: { isConnected: boolean; networkType: string }) => {
    setNetworkOnline(res.isConnected && res.networkType !== 'none')
  }
  api.getNetworkType?.({ success: (res) => applyNetworkType(res.networkType) })
  api.onNetworkStatusChange?.(onChange)
  return () => api.offNetworkStatusChange?.(onChange)
}
