/**
 * 移动端传输层（services/http.ts）单测。
 *
 * mock 掉全局 uni.request / uni.uploadFile（H5/小程序/原生平台注入的全局对象），
 * 把 401 自动 refresh、refreshPromise 去重、403 权限错误、upload 字符串 JSON 解析等
 * 语义钉死——这些正是原生 App / 小程序目标（无浏览器 Cookie 域）上会话互通
 * 能不能用的关键路径。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import {
  ApiError,
  AuthenticationError,
  getApiBase,
  mapi,
  PermissionError,
  setApiBase,
  setAuthenticationFailureHandler,
} from './http'

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

interface UploadStubOptions {
  url: string
  filePath: string
  name: string
  formData?: Record<string, string>
  header?: Record<string, string>
  success: (res: { statusCode: number; data: unknown }) => void
  fail: (err: { errMsg?: string }) => void
}

interface UniStub {
  requests: RequestStubOptions[]
  uploads: UploadStubOptions[]
  respondRequest: (fn: (opts: RequestStubOptions, callNo: number) => void) => void
  respondUpload: (fn: (opts: UploadStubOptions, callNo: number) => void) => void
  /** 便捷：对指定 opts 直接回 2xx 成功。 */
  ok: (opts: RequestStubOptions | UploadStubOptions, data: unknown, status?: number) => void
}

function installUni(): UniStub {
  const requests: RequestStubOptions[] = []
  const uploads: UploadStubOptions[] = []
  let reqResponder: ((opts: RequestStubOptions, n: number) => void) | null = null
  let uplResponder: ((opts: UploadStubOptions, n: number) => void) | null = null
  vi.stubGlobal('uni', {
    request: (opts: RequestStubOptions) => {
      requests.push(opts)
      reqResponder?.(opts, requests.length)
    },
    uploadFile: (opts: UploadStubOptions) => {
      uploads.push(opts)
      uplResponder?.(opts, uploads.length)
    },
  })
  return {
    requests,
    uploads,
    respondRequest: (fn) => (reqResponder = fn),
    respondUpload: (fn) => (uplResponder = fn),
    ok: (opts, data, status = 200) =>
      (opts as RequestStubOptions & UploadStubOptions).success({ statusCode: status, data }),
  }
}

let uni: UniStub

beforeEach(() => {
  // 模块级状态隔离：apiBase 与认证失败回调都是 http.ts 里的模块单例。
  setApiBase('')
  setAuthenticationFailureHandler(null)
  uni = installUni()
})

afterEach(() => {
  vi.unstubAllGlobals()
})

describe('error classes (transport 语义与 @oa4rust/sdk 对齐)', () => {
  it('ApiError carries name and status', () => {
    const err = new ApiError('HTTP 500', 500)
    expect(err).toBeInstanceOf(Error)
    expect(err.name).toBe('ApiError')
    expect(err.status).toBe(500)
    expect(err.message).toBe('HTTP 500')
  })

  it('AuthenticationError is a 401 ApiError and PermissionError is a 403 ApiError', () => {
    expect(new AuthenticationError('x')).toBeInstanceOf(ApiError)
    expect(new AuthenticationError('x').status).toBe(401)
    expect(new PermissionError('x')).toBeInstanceOf(ApiError)
    expect(new PermissionError('x').status).toBe(403)
  })
})

describe('apiBase management', () => {
  it('setApiBase strips trailing slashes (single and repeated)', () => {
    setApiBase('http://host:8080/api/')
    expect(getApiBase()).toBe('http://host:8080/api')
    setApiBase('http://host:8080///')
    expect(getApiBase()).toBe('http://host:8080')
  })

  it('prefixed URLs carry the absolute base while relative paths get a leading slash', async () => {
    setApiBase('http://host:8080')
    uni.respondRequest((o) => uni.ok(o, { v: 1 }))
    await mapi.get('foo/bar')
    expect(uni.requests[0].url).toBe('http://host:8080/foo/bar')
    await mapi.get('/baz')
    expect(uni.requests[1].url).toBe('http://host:8080/baz')
  })
})

describe('URL building and param filtering', () => {
  async function lastUrl(): Promise<string> {
    uni.respondRequest((o) => uni.ok(o, undefined))
    await mapi.get('/api/thing')
    return uni.requests[0].url
  }

  it('serializes params after "?" in fixed key order', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined))
    await mapi.get('/api/thing', { params: { a: '1', b: '2' } })
    expect(uni.requests[0].url).toBe('/api/thing?a=1&b=2')
  })

  it('filters undefined/null param values out of the query string', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined))
    // 传输层对 null/undefined 值做运行时过滤（类型上 params 是 Record<string, string>）。
    await mapi.get('/api/thing', {
      params: { a: '1', b: undefined, c: null, d: '2' } as unknown as Record<string, string>,
    })
    expect(uni.requests[0].url).toBe('/api/thing?a=1&d=2')
  })

  it('omits the query string entirely when every param value is empty', async () => {
    expect(await lastUrl()).toBe('/api/thing')
  })

  it('omits the query string when params exist but every value is undefined/null', async () => {
    // 运行时过滤后 entries 为空 → 不能留下裸 "?" 尾巴。
    uni.respondRequest((o) => uni.ok(o, undefined))
    await mapi.get('/api/thing', {
      params: { a: undefined, b: null } as unknown as Record<string, string>,
    })
    expect(uni.requests[0].url).toBe('/api/thing')
  })

  it('URL-encodes keys and values', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined))
    await mapi.get('/api/thing', { params: { q: 'a b&c' } })
    expect(uni.requests[0].url).toBe('/api/thing?q=a%20b%26c')
  })

  it('appends with "&" when the path already contains a query string', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined))
    await mapi.get('/api/thing?x=1', { params: { y: '2' } })
    expect(uni.requests[0].url).toBe('/api/thing?x=1&y=2')
  })
})

describe('method, body, header, and timeout semantics', () => {
  it('GET omits the body while POST/PUT send it and DELETE omits it', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined))
    await mapi.get('/api/x')
    await mapi.post('/api/x', { a: 1 })
    await mapi.put('/api/x', { a: 2 })
    await mapi.delete('/api/x')
    expect(uni.requests[0].data).toBeUndefined()
    expect(uni.requests[1].data).toEqual({ a: 1 })
    expect(uni.requests[2].data).toEqual({ a: 2 })
    expect(uni.requests[3].data).toBeUndefined()
    expect(uni.requests[1].method).toBe('POST')
    expect(uni.requests[2].method).toBe('PUT')
  })

  it('null body is sent as no data (POST logout 等端点依赖此行为)', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined))
    await mapi.post('/api/x', null)
    expect(uni.requests[0].data).toBeUndefined()
  })

  it('defaults Content-Type to application/json and lets headers override it', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined))
    await mapi.get('/api/x', { headers: { 'X-Token': 't' } })
    expect(uni.requests[0].header['Content-Type']).toBe('application/json')
    expect(uni.requests[0].header['X-Token']).toBe('t')
  })

  it('defaults the timeout to 30000ms and honors an explicit timeoutMs', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined))
    await mapi.get('/api/x')
    await mapi.get('/api/x', { timeoutMs: 5000 })
    expect(uni.requests[0].timeout).toBe(30000)
    expect(uni.requests[1].timeout).toBe(5000)
  })

  it('always sends withCredentials (H5 同源 Cookie 鉴权的前提)', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined))
    await mapi.get('/api/x')
    expect(uni.requests[0].withCredentials).toBe(true)
  })
})

describe('success responses', () => {
  it('200 resolves with the ApiResponse payload untouched', async () => {
    uni.respondRequest((o) => uni.ok(o, { success: true, data: { id: 7 } }))
    const resp = await mapi.get<{ id: number }>('/api/thing')
    expect(resp).toEqual({ success: true, data: { id: 7 } })
  })

  it('204 resolves with an empty success envelope', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined, 204))
    await expect(mapi.post('/api/thing', {})).resolves.toEqual({ success: true, data: undefined })
  })

  it('discardResponse swallows the body (login/refresh 遗留 token 字段不得外泄)', async () => {
    uni.respondRequest((o) => uni.ok(o, { success: true, data: { legacy: 'token' } }))
    const resp = await mapi.post('/api/thing', {}, { discardResponse: true })
    expect(resp).toEqual({ success: true, data: undefined })
  })

  it('non-2xx other than 401/403 rejects with a generic ApiError carrying the status', async () => {
    uni.respondRequest((o) => uni.ok(o, { success: false }, 500))
    const err = await mapi.get('/api/thing').catch((e) => e)
    expect(err).toBeInstanceOf(ApiError)
    expect((err as ApiError).status).toBe(500)
    expect((err as ApiError).message).toBe('HTTP 500')
  })
})

describe('401 handling: auto refresh + single retry', () => {
  function first401Then200(url: string, payload: unknown, firstN401 = 1) {
    const seen = new Map<string, number>()
    uni.respondRequest((o) => {
      if (o.url === '/api/authentication/refresh') {
        uni.ok(o, undefined)
        return
      }
      seen.set(o.url, (seen.get(o.url) ?? 0) + 1)
      if ((seen.get(o.url) ?? 0) <= firstN401 && o.url === url) uni.ok(o, undefined, 401)
      else uni.ok(o, payload, 200)
    })
  }

  it('refreshes the session then retries the original request exactly once', async () => {
    first401Then200('/api/secret', { success: true, data: 'ok' })
    const resp = await mapi.get('/api/secret')
    expect(resp).toEqual({ success: true, data: 'ok' })
    expect(uni.requests.map((r) => r.url)).toEqual(['/api/secret', '/api/authentication/refresh', '/api/secret'])
  })

  it('does not refresh for requireAuth:false calls (login/who 自身 401 即失败)', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined, 401))
    const err = await mapi.get('/api/who', { requireAuth: false }).catch((e) => e)
    expect(err).toBeInstanceOf(AuthenticationError)
    expect(uni.requests).toHaveLength(1)
  })

  it('deduplicates concurrent 401s into a single refresh request', async () => {
    first401Then200('/api/concurrent', { success: true, data: 'ok' }, 2)
    const [a, b] = await Promise.all([mapi.get('/api/concurrent'), mapi.get('/api/concurrent')])
    expect(a).toEqual({ success: true, data: 'ok' })
    expect(b).toEqual({ success: true, data: 'ok' })
    const refreshCalls = uni.requests.filter((r) => r.url === '/api/authentication/refresh')
    expect(refreshCalls).toHaveLength(1)
    // 2 次首发 + 1 次 refresh + 2 次重试
    expect(uni.requests).toHaveLength(5)
  })

  it('a second 401 after refresh fails without a further retry loop', async () => {
    uni.respondRequest((o) => {
      if (o.url === '/api/authentication/refresh') {
        uni.ok(o, undefined)
        return
      }
      uni.ok(o, undefined, 401)
    })
    const err = await mapi.get('/api/secret').catch((e) => e)
    expect(err).toBeInstanceOf(AuthenticationError)
    // 首发 + refresh + 重试（仍 401，retried=true 不再刷新）
    expect(uni.requests.map((r) => r.url)).toEqual(['/api/secret', '/api/authentication/refresh', '/api/secret'])
  })

  it('a failed refresh rejects with AuthenticationError and fires the failure handler', async () => {
    const handler = vi.fn()
    setAuthenticationFailureHandler(handler)
    uni.respondRequest((o) => uni.ok(o, undefined, 401))
    const err = await mapi.get('/api/secret').catch((e) => e)
    expect(err).toBeInstanceOf(AuthenticationError)
    expect(err.message).toBe('Session expired, please login again')
    // 会话不可恢复时故障回调必须触发（会话存储据此清本地用户）；次数是实现细节，钉"至少一次"。
    expect(handler.mock.calls.length).toBeGreaterThan(0)
  })

  it('the refresh endpoint itself failing 401 never triggers a second refresh', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined, 401))
    const err = await mapi.refresh().catch((e) => e)
    expect(err).toBeInstanceOf(AuthenticationError)
    expect(uni.requests.filter((r) => r.url === '/api/authentication/refresh')).toHaveLength(1)
  })
})

describe('403 handling', () => {
  it('rejects with a PermissionError and performs no refresh', async () => {
    uni.respondRequest((o) => uni.ok(o, undefined, 403))
    const err = await mapi.get('/api/forbidden').catch((e) => e)
    expect(err).toBeInstanceOf(PermissionError)
    expect((err as PermissionError).status).toBe(403)
    expect(uni.requests).toHaveLength(1)
  })
})

describe('network failures', () => {
  it('fail callback rejects with the uni error message and status 0', async () => {
    uni.respondRequest((o) => o.fail({ errMsg: 'request:fail timeout' }))
    const err = await mapi.get('/api/thing').catch((e) => e)
    expect(err).toBeInstanceOf(ApiError)
    expect((err as ApiError).message).toBe('request:fail timeout')
    expect((err as ApiError).status).toBe(0)
  })

  it('a missing errMsg falls back to a generic message', async () => {
    uni.respondRequest((o) => o.fail({}))
    const err = await mapi.get('/api/thing').catch((e) => e)
    expect((err as ApiError).message).toBe('Network error')
  })
})

describe('file upload (uni.uploadFile)', () => {
  it('parses the JSON-string body into an ApiResponse on success', async () => {
    uni.respondUpload((o) => uni.ok(o, JSON.stringify({ success: true, data: { id: 'att-1' } })))
    const resp = await mapi.upload('/api/attachment/upload/folder/f1', '/tmp/f.png', {
      name: 'file',
      formData: { name: 'f.png' },
    })
    expect(resp).toEqual({ success: true, data: { id: 'att-1' } })
    expect(uni.uploads[0].name).toBe('file')
    expect(uni.uploads[0].formData).toEqual({ name: 'f.png' })
    expect(uni.uploads[0].filePath).toBe('/tmp/f.png')
  })

  it('a non-JSON string body resolves as the raw string (parse-failure fallback, not an envelope)', async () => {
    uni.respondUpload((o) => uni.ok(o, 'plain-bytes'))
    const resp = await mapi.upload('/api/attachment/upload/folder/f1', '/tmp/x.bin')
    expect(resp).toBe('plain-bytes')
  })

  it('defaults the multipart field name to "file"', async () => {
    uni.respondUpload((o) => uni.ok(o, '{"success":true}'))
    await mapi.upload('/api/attachment/upload/folder/f1', '/tmp/x.bin')
    expect(uni.uploads[0].name).toBe('file')
  })

  it('refreshes once on 401 then retries the upload', async () => {
    const uploads = new Map<string, number>()
    uni.respondUpload((o, n) => {
      uploads.set(o.url, (uploads.get(o.url) ?? 0) + 1)
      if (n === 1) uni.ok(o, undefined, 401)
      else uni.ok(o, JSON.stringify({ success: true, data: { id: 'att-2' } }))
    })
    uni.respondRequest((o) => uni.ok(o, undefined))
    const resp = await mapi.upload('/api/attachment/upload/folder/f1', '/tmp/f.png')
    expect(resp).toEqual({ success: true, data: { id: 'att-2' } })
    expect(uni.requests.filter((r) => r.url === '/api/authentication/refresh')).toHaveLength(1)
    expect(uni.uploads).toHaveLength(2)
  })

  it('403 rejects with PermissionError without refresh', async () => {
    uni.respondUpload((o) => uni.ok(o, undefined, 403))
    const err = await mapi.upload('/api/attachment/upload/folder/f1', '/tmp/f.png').catch((e) => e)
    expect(err).toBeInstanceOf(PermissionError)
    expect(uni.requests).toHaveLength(0)
  })

  it('rejects with AuthenticationError and does NOT retry when the 401-triggered refresh itself fails', async () => {
    // upload 收到 401 → 触发 refreshSession；若 refresh 也失败（如 500），
    // 必须走 authenticationFailed() 分支直接 reject，不能无限重试上传。
    uni.respondUpload((o) => uni.ok(o, undefined, 401))
    uni.respondRequest((o) => {
      if (o.url === '/api/authentication/refresh') uni.ok(o, undefined, 500)
      else uni.ok(o, undefined)
    })
    const err = await mapi.upload('/api/attachment/upload/folder/f1', '/tmp/f.png').catch((e) => e)
    expect(err).toBeInstanceOf(AuthenticationError)
    expect(uni.uploads).toHaveLength(1) // 上传只发生一次，refresh 失败后不再重试
    expect(uni.requests.filter((r) => r.url === '/api/authentication/refresh')).toHaveLength(1)
  })

  it('requireAuth:false suppresses the refresh-and-retry path', async () => {
    uni.respondUpload((o) => uni.ok(o, undefined, 401))
    const err = await mapi
      .upload('/api/attachment/upload/folder/f1', '/tmp/f.png', { requireAuth: false })
      .catch((e) => e)
    expect(err).toBeInstanceOf(AuthenticationError)
    expect(uni.requests).toHaveLength(0)
  })

  it('generic 5xx rejects with the status-bearing ApiError', async () => {
    uni.respondUpload((o) => uni.ok(o, undefined, 502))
    const err = await mapi.upload('/api/attachment/upload/folder/f1', '/tmp/f.png').catch((e) => e)
    expect(err).toBeInstanceOf(ApiError)
    expect((err as ApiError).status).toBe(502)
  })

  it('network fail rejects with the uni error message, else "Upload failed"', async () => {
    uni.respondUpload((o) => o.fail({ errMsg: 'upload:fail abort' }))
    const err = await mapi.upload('/api/attachment/upload/folder/f1', '/tmp/f.png').catch((e) => e)
    expect((err as ApiError).message).toBe('upload:fail abort')
    uni.respondUpload((o) => o.fail({}))
    const err2 = await mapi.upload('/api/attachment/upload/folder/f1', '/tmp/f.png').catch((e) => e)
    expect((err2 as ApiError).message).toBe('Upload failed')
  })
})
