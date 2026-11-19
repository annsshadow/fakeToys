// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 2xx 响应体非法 JSON 的错误信封归一化（移动端 services/http.ts）。
 *
 * 网关误插 HTML 错误页、反向代理截断都会让 2xx 响应体不是合法 JSON。
 * 桌面端 @oa4rust/sdk 的 ApiClient 已在 R282 把三通道归一化为
 * ApiError('malformed JSON response', status)，但移动端 performRequest 此前
 * 直接 resolve(data as ApiResponse<T>)，把 HTML 字符串当成功信封返回。
 * 页面（如 search.vue doSearch）拿到字符串后 g.data 为 undefined，
 * 把"服务端失败"静默吞成"搜索成功但无结果"，覆盖上一次结果。
 * 本测试锁定 performRequest 与桌面端同口径。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiError, mapi, setApiBase, setAuthenticationFailureHandler } from './http'

interface RequestStubOptions {
  url: string
  method: string
  data?: unknown
  header: Record<string, string>
  timeout: number
  withCredentials: boolean
  success: (res: { statusCode: number; data: unknown }) => void
  fail: (err: { errMsg?: string }) => void
}

function installUni(responder: (opts: RequestStubOptions) => void): void {
  vi.stubGlobal('uni', {
    request: (opts: RequestStubOptions) => responder(opts),
    uploadFile: () => {},
  })
}

let html200: (opts: RequestStubOptions) => void

beforeEach(() => {
  setApiBase('')
  setAuthenticationFailureHandler(null)
  // 2xx 但响应体是网关误插的 HTML 错误页（非法 JSON 信封）
  html200 = (opts) => opts.success({ statusCode: 200, data: '<!DOCTYPE html><html>Bad Gateway</html>' })
  installUni(html200)
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('performRequest 2xx 非法 JSON 信封归一化', () => {
  it('get() 把 2xx HTML 响应归一化为 ApiError 并保留 status', async () => {
    const err = await mapi.get('/api/data').catch((e) => e)

    expect(err).toBeInstanceOf(ApiError)
    expect(err.name).toBe('ApiError')
    expect(err.status).toBe(200)
    expect(err.message).toBe('malformed JSON response')
  })

  it('post() 同样归一化（searchApi.global 走的就是 post 通道）', async () => {
    const err = await mapi.post('/api/queryview/search', { key: 'x' }).catch((e) => e)

    expect(err).toBeInstanceOf(ApiError)
    expect((err as ApiError).status).toBe(200)
    expect((err as ApiError).message).toBe('malformed JSON response')
  })

  it('合法 envelope 仍原样返回，不被误判', async () => {
    installUni((opts) => opts.success({ statusCode: 200, data: { success: true, data: { id: 7 } } }))
    const resp = await mapi.get<{ id: number }>('/api/thing')
    expect(resp).toEqual({ success: true, data: { id: 7 } })
  })

  it('undefined/204 空体仍归一化为成功空信封（兼容既有 no-content 语义）', async () => {
    installUni((opts) => opts.success({ statusCode: 204, data: undefined }))
    await expect(mapi.post('/api/thing', {})).resolves.toEqual({ success: true, data: undefined })
  })
})
