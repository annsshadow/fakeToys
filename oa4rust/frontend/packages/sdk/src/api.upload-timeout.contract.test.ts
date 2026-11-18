// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// @vitest-environment node
/**
 * 上传超时文案契约：fetch 版 upload/uploadBytes 的 timeoutMs 触发的 AbortError
 * 必须归一化为「上传超时」而非「上传失败」。XHR 版 uploadWithProgress 的
 * ontimeout 早已使用 UPLOAD_TIMEOUT_MESSAGE，三条上传入口的用户可见文案必须一致。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiClient, ApiError } from './api'

const TIMEOUT_TEXT = '上传超时，请检查网络后重试'
const FAILURE_TEXT = '上传失败，请检查网络后重试'

/** 真实 fetch 语义：AbortController 超时后 reject AbortError。 */
function fetchThatTimesOut(): void {
  vi.stubGlobal(
    'fetch',
    vi.fn(
      (_url: string | URL, init: RequestInit) =>
        new Promise<Response>((_resolve, reject) => {
          init.signal?.addEventListener('abort', () => {
            const err = new Error('The user aborted a request.')
            err.name = 'AbortError'
            reject(err)
          })
        }),
    ),
  )
}

describe('fetch 上传超时的用户可见文案（与 XHR 上传一致）', () => {
  beforeEach(() => {
    vi.stubGlobal('window', { location: { origin: 'https://web.example.test' } })
  })

  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('upload：timeoutMs 到期后报「上传超时」而不是「上传失败」', async () => {
    fetchThatTimesOut()
    const err = await new ApiClient()
      .upload('/api/file/upload', new FormData(), { timeoutMs: 5 })
      .catch((e: unknown) => e)

    expect(err).toBeInstanceOf(ApiError)
    expect((err as ApiError).status).toBe(0)
    expect((err as ApiError).message).toBe(TIMEOUT_TEXT)
    expect((err as ApiError).message).not.toBe(FAILURE_TEXT)
  })

  it('uploadBytes：timeoutMs 到期后报「上传超时」而不是「上传失败」', async () => {
    fetchThatTimesOut()
    const err = await new ApiClient()
      .uploadBytes('/api/file/upload', new Uint8Array([1]), { timeoutMs: 5 })
      .catch((e: unknown) => e)

    expect(err).toBeInstanceOf(ApiError)
    expect((err as ApiError).status).toBe(0)
    expect((err as ApiError).message).toBe(TIMEOUT_TEXT)
    expect((err as ApiError).message).not.toBe(FAILURE_TEXT)
  })
})
