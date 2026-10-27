// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * ApiClient 的**错误映射与重试语义**（uploadBytes / stream 两条通道）。
 *
 * 为什么单独立一个文件：`api.test.ts` 覆盖的是 `request()` 的主干，
 * `api.stream.test.ts` 覆盖的是 SSE 解析；而 `uploadBytes` 与 `stream` 各自
 * **重复实现了一遍** 401→refresh→retry / 403→PermissionError / 其它→ApiError
 * 这段状态机（见 api.ts 中两处同构代码）。重复实现 = 两份独立出错的机会，
 * 而此前这两处 401/403 分支在覆盖率报表上是空的。
 *
 * 断言重点不是「抛了个错」，而是**抛的是哪一类错、refresh 被调了几次**：
 * 把 403 误判成 401 会触发一次无意义的 refresh 并把用户踢下线；
 * 把 401 误判成普通错误则会让「自动续期」永久失效。
 */

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiClient, ApiError, AuthenticationError, PermissionError } from './api'

function jsonResponse(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { 'Content-Type': 'application/json' },
  })
}

function emptyResponse(status: number): Response {
  return new Response(null, { status })
}

function sseResponse(chunks: string[], status = 200): Response {
  const encoder = new TextEncoder()
  const stream = new ReadableStream<Uint8Array>({
    start(controller) {
      for (const c of chunks) controller.enqueue(encoder.encode(c))
      controller.close()
    },
  })
  return new Response(stream, { status, headers: { 'Content-Type': 'text/event-stream' } })
}

describe('ApiClient error mapping', () => {
  beforeEach(() => {
    vi.stubGlobal('window', { location: { origin: 'https://web.example.test' } })
  })

  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  describe('uploadBytes', () => {
    it('posts raw bytes as octet-stream and passes fileName as query', async () => {
      const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: { id: 1 } }))
      vi.stubGlobal('fetch', fetchMock)

      const bytes = new Uint8Array([1, 2, 3])
      const res = await new ApiClient().uploadBytes('/api/file/upload', bytes, { fileName: 'a b.bin' })

      expect(res).toEqual({ success: true, data: { id: 1 } })
      const [url, init] = fetchMock.mock.calls[0]
      expect(url).toBe('https://web.example.test/api/file/upload?fileName=a+b.bin')
      expect(init).toMatchObject({ method: 'POST', credentials: 'include' })
      expect(init.headers).toMatchObject({ 'Content-Type': 'application/octet-stream' })
      expect(init.body).toBe(bytes)
    })

    it('refreshes the session once on 401 and replays the same bytes', async () => {
      const bytes = new Uint8Array([9, 9])
      const fetchMock = vi
        .fn()
        .mockResolvedValueOnce(emptyResponse(401)) // 首次上传：会话过期
        .mockResolvedValueOnce(jsonResponse({ success: true, data: null })) // refresh
        .mockResolvedValueOnce(jsonResponse({ success: true, data: { ok: true } })) // 重放
      vi.stubGlobal('fetch', fetchMock)

      await expect(new ApiClient().uploadBytes('/api/file/upload', bytes)).resolves.toEqual({
        success: true,
        data: { ok: true },
      })

      expect(fetchMock).toHaveBeenCalledTimes(3)
      expect(fetchMock.mock.calls[1][0]).toContain('/api/authentication/refresh')
      // 重放必须带同一份字节，而不是被吞掉的空 body
      expect(fetchMock.mock.calls[2][1].body).toBe(bytes)
    })

    it('does not refresh when requireAuth is false', async () => {
      const fetchMock = vi.fn().mockResolvedValue(emptyResponse(401))
      vi.stubGlobal('fetch', fetchMock)

      await expect(
        new ApiClient().uploadBytes('/api/file/upload', new Uint8Array([1]), { requireAuth: false }),
      ).rejects.toBeInstanceOf(AuthenticationError)
      expect(fetchMock).toHaveBeenCalledTimes(1)
    })

    it('fails with AuthenticationError when the refresh itself fails', async () => {
      const onAuthFailure = vi.fn()
      const client = new ApiClient()
      client.setAuthenticationFailureHandler(onAuthFailure)
      const fetchMock = vi
        .fn()
        .mockResolvedValueOnce(emptyResponse(401))
        .mockResolvedValueOnce(emptyResponse(401)) // refresh 也 401
      vi.stubGlobal('fetch', fetchMock)

      await expect(client.uploadBytes('/api/file/upload', new Uint8Array([1]))).rejects.toBeInstanceOf(
        AuthenticationError,
      )
      // 认证彻底失败必须通知上层（路由跳登录页就挂在这个回调上）。
      //
      // 这里**故意不断言次数**：当前实现会通知两次 —— 一次来自 refreshSession
      // 内部那个 401（request() 里的 authenticationFailed），一次来自本方法的
      // catch。这是已知瑕疵（同一次失败重复通知，若 handler 弹 toast 会弹两条），
      // 记在 docs/plans 的待修项里。把 2 写死会让「去重」这个修复反而弄红测试，
      // 所以只钉住真正的契约：至少通知一次。
      expect(onAuthFailure).toHaveBeenCalled()
    })

    it('does not retry twice when the replay also returns 401', async () => {
      const fetchMock = vi
        .fn()
        .mockResolvedValueOnce(emptyResponse(401))
        .mockResolvedValueOnce(jsonResponse({ success: true })) // refresh 成功
        .mockResolvedValueOnce(emptyResponse(401)) // 重放仍 401
      vi.stubGlobal('fetch', fetchMock)

      await expect(new ApiClient().uploadBytes('/api/file/upload', new Uint8Array([1]))).rejects.toBeInstanceOf(
        AuthenticationError,
      )
      expect(fetchMock).toHaveBeenCalledTimes(3) // 不允许无限重试
    })

    it('maps 403 to PermissionError and other statuses to ApiError', async () => {
      vi.stubGlobal('fetch', vi.fn().mockResolvedValue(emptyResponse(403)))
      await expect(new ApiClient().uploadBytes('/api/x', new Uint8Array([1]))).rejects.toBeInstanceOf(
        PermissionError,
      )

      vi.stubGlobal('fetch', vi.fn().mockResolvedValue(emptyResponse(500)))
      const err = await new ApiClient()
        .uploadBytes('/api/x', new Uint8Array([1]))
        .catch((e) => e)
      expect(err).toBeInstanceOf(ApiError)
      expect(err.status).toBe(500)
    })

    it('honours an explicit PUT method for uploads', async () => {
      const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: null }))
      vi.stubGlobal('fetch', fetchMock)

      await new ApiClient().upload('/api/file/replace', new FormData(), { method: 'PUT' })

      expect(fetchMock.mock.calls[0][1].method).toBe('PUT')
    })
  })

  describe('stream', () => {
    it('refreshes the session on 401 and replays the request once', async () => {
      const fetchMock = vi
        .fn()
        .mockResolvedValueOnce(emptyResponse(401))
        .mockResolvedValueOnce(jsonResponse({ success: true })) // refresh
        .mockResolvedValueOnce(sseResponse(['event: token\ndata: {"token":"好"}\n\n']))
      vi.stubGlobal('fetch', fetchMock)

      const events: Array<[string, string]> = []
      await new ApiClient().stream('/api/ai/stream', { q: 1 }, (e, d) => events.push([e, d]))

      expect(events).toEqual([['token', '{"token":"好"}']])
      expect(fetchMock).toHaveBeenCalledTimes(3)
      // 重放必须保留原始 body（否则对话内容丢失，用户看到空回答）
      expect(fetchMock.mock.calls[2][1].body).toBe(JSON.stringify({ q: 1 }))
    })

    it('maps 403 to PermissionError without refreshing', async () => {
      const fetchMock = vi.fn().mockResolvedValue(emptyResponse(403))
      vi.stubGlobal('fetch', fetchMock)

      await expect(new ApiClient().stream('/api/ai/stream', {}, () => {})).rejects.toBeInstanceOf(
        PermissionError,
      )
      expect(fetchMock).toHaveBeenCalledTimes(1)
    })

    it('reports HTTP status and status text for other failures', async () => {
      vi.stubGlobal('fetch', vi.fn().mockResolvedValue(new Response(null, { status: 502, statusText: 'Bad Gateway' })))

      const err = await new ApiClient()
        .stream('/api/ai/stream', {}, () => {})
        .catch((e) => e)

      expect(err).toBeInstanceOf(ApiError)
      expect(err.message).toContain('502')
      expect(err.message).toContain('Bad Gateway')
    })

    it('rejects a 200 response that carries no readable body', async () => {
      // 没有 body 的 200 会让 reader 永远挂住 —— 必须显式失败而不是静默成功
      vi.stubGlobal('fetch', vi.fn().mockResolvedValue(emptyResponse(200)))

      await expect(new ApiClient().stream('/api/ai/stream', {}, () => {})).rejects.toBeInstanceOf(ApiError)
    })
  })

  describe('refreshSession', () => {
    it('single-flights concurrent refreshes', async () => {
      // 两个请求同时 401：只能发一次 refresh，否则会互相吊销新会话
      let refreshCalls = 0
      const fetchMock = vi.fn(async (url: string) => {
        if (url.includes('/api/authentication/refresh')) {
          refreshCalls += 1
          await new Promise((r) => setTimeout(r, 5))
          return jsonResponse({ success: true, data: null })
        }
        return jsonResponse({ success: true, data: null })
      })
      vi.stubGlobal('fetch', fetchMock)

      const client = new ApiClient()
      await Promise.all([client.refreshSession(), client.refreshSession()])

      expect(refreshCalls).toBe(1)
    })

    it('allows a new refresh after the previous one settled', async () => {
      const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: null }))
      vi.stubGlobal('fetch', fetchMock)

      const client = new ApiClient()
      await client.refreshSession()
      await client.refreshSession()

      expect(fetchMock).toHaveBeenCalledTimes(2) // refreshPromise 必须被 finally 清空
    })

    it('never refreshes the refresh endpoint itself (no infinite recursion)', async () => {
      const fetchMock = vi.fn().mockResolvedValue(emptyResponse(401))
      vi.stubGlobal('fetch', fetchMock)

      await expect(
        new ApiClient().post('/api/authentication/refresh', null, { requireAuth: false }),
      ).rejects.toBeInstanceOf(AuthenticationError)
      expect(fetchMock).toHaveBeenCalledTimes(1)
    })
  })
})
