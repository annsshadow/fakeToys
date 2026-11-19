// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 2xx 响应体非法 JSON 的错误信封归一化：R282。
 *
 * 网关误插 HTML 错误页、反向代理截断或空 body 边缘都会让 2xx 响应体不是合法 JSON。
 * 三条 fetch 通道（request/upload/uploadBytes）此前直接返回 resp.json()，原生
 * SyntaxError 会冒泡到页面并绕过所有 ApiError 判别分支；而 XHR 通道
 * uploadWithProgress 早已把同类情况归一化为 ApiError('malformed JSON response', status)。
 * 本测试锁定三条通道与 XHR 同口径：错误必须是 ApiError、保留 HTTP status。
 */

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiClient, ApiError } from './api'

function htmlResponse(status = 200): Response {
  return new Response('<!DOCTYPE html><html><body>Bad Gateway</body></html>', {
    status,
    headers: { 'Content-Type': 'text/html' },
  })
}

describe('ApiClient 2xx malformed JSON envelope', () => {
  beforeEach(() => {
    vi.stubGlobal('window', { location: { origin: 'https://web.example.test' } })
  })

  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('request(): get() 把 2xx 非法 JSON 归一化为 ApiError 并保留 status', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(htmlResponse(200)))

    const err = await new ApiClient().get('/api/data').catch((e) => e)

    expect(err).toBeInstanceOf(ApiError)
    expect(err.name).toBe('ApiError')
    expect(err.status).toBe(200)
    expect(err.message).toBe('malformed JSON response')
  })

  it('upload(): multipart 上传把 2xx 非法 JSON 归一化为 ApiError 并保留 status', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(htmlResponse(201)))

    const err = await new ApiClient().upload('/api/upload', new FormData()).catch((e) => e)

    expect(err).toBeInstanceOf(ApiError)
    expect(err.name).toBe('ApiError')
    expect(err.status).toBe(201)
    expect(err.message).toBe('malformed JSON response')
  })

  it('uploadBytes(): 字节流上传把 2xx 非法 JSON 归一化为 ApiError 并保留 status', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(htmlResponse(202)))

    const err = await new ApiClient().uploadBytes('/api/upload/bytes', new Uint8Array([1])).catch((e) => e)

    expect(err).toBeInstanceOf(ApiError)
    expect(err.name).toBe('ApiError')
    expect(err.status).toBe(202)
    expect(err.message).toBe('malformed JSON response')
  })
})
