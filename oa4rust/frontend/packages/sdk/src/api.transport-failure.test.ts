// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
/**
 * R155 / B1：桌面 SDK 传输层的网络失败文案归一化。
 *
 * 与移动 services/http.ts（R136）和 laiyipao api/client.ts（B1）同口径：
 * fetch / XHR 的底层英文技术串（Failed to fetch / Network error / Timeout /
 * Request aborted）不能透到用户可见的「加载失败：{message}」里。
 *
 * 判据同时钉住「归一化」与「不改语义」：
 * - 网络失败 → 中文文案 + status 0（shouldRetryQuery 仍不重试 status 0）；
 * - 用户取消仍是 RequestCancelledError、HTTP 状态错误仍是 HTTP xxx（不过度归一化）。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiClient, ApiError, RequestCancelledError } from './api'

/** 最小 XHR 桩：可手动触发 onerror / ontimeout。 */
class FakeXHR {
  static instances: FakeXHR[] = []
  upload = { onprogress: null as ((e: unknown) => void) | null }
  onload: (() => void) | null = null
  onerror: (() => void) | null = null
  ontimeout: (() => void) | null = null
  withCredentials = false
  status = 200
  responseText = ''
  timeout = 0

  open() {
    FakeXHR.instances.push(this)
  }
  setRequestHeader() {}
  send() {}
}

const NETWORK_TEXT = '网络连接失败，请检查网络后重试'
const UPLOAD_TEXT = '上传失败，请检查网络后重试'

describe('桌面 SDK 传输层网络失败文案归一化（R155/B1）', () => {
  beforeEach(() => {
    FakeXHR.instances = []
    vi.stubGlobal('window', { location: { origin: 'https://web.example.test' } })
    vi.stubGlobal('XMLHttpRequest', FakeXHR as unknown as typeof XMLHttpRequest)
  })

  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  const fetchFails = () => vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new TypeError('Failed to fetch')))

  it('request：网络失败 → 中文文案 + status 0，不再透出 Failed to fetch', async () => {
    fetchFails()
    const err = await new ApiClient().get('/api/list').catch((e) => e)
    expect(err).toBeInstanceOf(ApiError)
    expect(err.status).toBe(0)
    expect(err.message).toBe(NETWORK_TEXT)
    expect(err.message).not.toContain('Failed to fetch')
  })

  it('upload：网络失败 → 上传失败文案 + status 0', async () => {
    fetchFails()
    const err = await new ApiClient().upload('/api/file', new FormData()).catch((e) => e)
    expect(err).toBeInstanceOf(ApiError)
    expect(err.status).toBe(0)
    expect(err.message).toBe(UPLOAD_TEXT)
  })

  it('uploadBytes：网络失败 → 上传失败文案 + status 0', async () => {
    fetchFails()
    const err = await new ApiClient().uploadBytes('/api/file', new Uint8Array([1])).catch((e) => e)
    expect(err).toBeInstanceOf(ApiError)
    expect(err.status).toBe(0)
    expect(err.message).toBe(UPLOAD_TEXT)
  })

  it('stream：网络失败 → 中文文案 + status 0', async () => {
    fetchFails()
    const err = await new ApiClient().stream('/api/ai/stream', {}, () => {}).catch((e) => e)
    expect(err).toBeInstanceOf(ApiError)
    expect(err.status).toBe(0)
    expect(err.message).toBe(NETWORK_TEXT)
  })

  it('uploadWithProgress：XHR onerror → 上传失败文案，不透出 Network error', async () => {
    const p = new ApiClient().uploadWithProgress('/api/file', new FormData())
    FakeXHR.instances[0].onerror?.()
    const err = await p.catch((e) => e)
    expect(err).toBeInstanceOf(ApiError)
    expect(err.status).toBe(0)
    expect(err.message).toBe(UPLOAD_TEXT)
    expect(err.message).not.toContain('Network error')
  })

  it('uploadWithProgress：XHR ontimeout → 上传超时文案，不透出 Timeout', async () => {
    const p = new ApiClient().uploadWithProgress('/api/file', new FormData())
    FakeXHR.instances[0].ontimeout?.()
    const err = await p.catch((e) => e)
    expect(err).toBeInstanceOf(ApiError)
    expect(err.status).toBe(0)
    expect(err.message).toBe('上传超时，请检查网络后重试')
    expect(err.message).not.toContain('Timeout')
  })

  it('语义守卫：用户取消仍是 RequestCancelledError（不被归一化吞掉）', async () => {
    const external = new AbortController()
    const p = new ApiClient().get('/api/list', { signal: external.signal })
    external.abort()
    await expect(p).rejects.toBeInstanceOf(RequestCancelledError)
  })

  it('语义守卫：HTTP 500 仍是 HTTP 500（不过度归一化）', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(new Response(null, { status: 500 })))
    const err = await new ApiClient().get('/api/list').catch((e) => e)
    expect(err.status).toBe(500)
    expect(err.message).toContain('500')
  })
})
