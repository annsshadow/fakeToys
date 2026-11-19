// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * BUG-B 回归：登录失败的错误语义。
 *
 * 后端 POST /api/authentication/login 对「用户不存在」返回 401（AppError::Unauthorized）。
 * 这是一次**凭据认证失败**，用户仍停留在 /login 页，并没有任何会话过期。
 * 但 ApiClient 此前把 401 无条件映射成 "Session expired, please login again"，
 * 误导用户以为会话失效。本文件钉住两类 401 语义必须区分：
 *   - 登录接口的 401 → 凭据错误（用户名或密码错误）
 *   - 普通业务请求的 401（refresh 失败）→ 会话过期（保持原语义）
 */

import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiClient, AuthenticationError } from './api'

function unauthorized(): Response {
  return new Response(null, { status: 401 })
}

describe('login 401 must mean invalid credentials, not session expiry', () => {
  beforeEach(() => {
    vi.stubGlobal('window', { location: { origin: 'https://web.example.test' } })
  })

  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it('reports 用户名或密码错误 when the login endpoint returns 401', async () => {
    const fetchMock = vi.fn().mockResolvedValue(unauthorized())
    vi.stubGlobal('fetch', fetchMock)

    const error = await new ApiClient()
      .post(
        '/api/authentication/login',
        { credential: 'no-such-user', password: 'wrong' },
        { requireAuth: false, discardResponse: true },
      )
      .then(
        () => null,
        (e: unknown) => e,
      )

    expect(error).toBeInstanceOf(AuthenticationError)
    const message = (error as Error).message
    expect(message).not.toContain('Session expired')
    expect(message).toContain('用户名或密码错误')
    // 登录请求 requireAuth:false，401 不该触发 refresh
    expect(fetchMock).toHaveBeenCalledTimes(1)
  })

  it('keeps 会话过期 semantics for a normal authenticated request whose refresh fails', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(unauthorized()) // 业务请求 401
      .mockResolvedValueOnce(unauthorized()) // refresh 也 401
    vi.stubGlobal('fetch', fetchMock)

    const error = await new ApiClient().get('/api/some/protected/resource').then(
      () => null,
      (e: unknown) => e,
    )

    expect(error).toBeInstanceOf(AuthenticationError)
    expect((error as Error).message).toContain('Session expired')
    expect(fetchMock).toHaveBeenCalledTimes(2)
    expect(fetchMock.mock.calls[1][0]).toContain('/api/authentication/refresh')
  })
})
