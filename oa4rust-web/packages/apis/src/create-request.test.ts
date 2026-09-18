/**
 * createRequest 分支覆盖测试
 * 
 * 策略：mock SDK api 对象（它是真实的 I/O 外部依赖，不在本层覆盖范围内），
 * 但让 createRequest 的函数体真正执行，从而获得 V8 语句和分支覆盖。
 */

import { describe, expect, it, vi, beforeEach } from 'vitest'

// 在 import index.ts 之前注入 mock，避免触发真实 fetch
const mockApi = {
  get: vi.fn(() => Promise.resolve({ data: 'get-result' })),
  post: vi.fn(() => Promise.resolve({ data: 'post-result' })),
  put: vi.fn(() => Promise.resolve({ data: 'put-result' })),
  delete: vi.fn(() => Promise.resolve({ data: 'delete-result' })),
}

// 注入到 sdk
vi.mock('@oa4rust/sdk', () => ({
  api: mockApi,
}))

// 延迟导入，让 mock 先生效
let createRequest: (prefix: string) => (method: string, path: string, body?: unknown) => Promise<unknown>

beforeAll(async () => {
  const mod = await import('./index.ts')
  createRequest = mod.createRequest
})

beforeEach(() => {
  vi.clearAllMocks()
})

describe('createRequest() 分支覆盖', () => {
  it('GET 分支：转大写后匹配 case "GET"', async () => {
    const req = createRequest('/api/test')
    const result = await req('get', '/endpoint')
    expect(mockApi.get).toHaveBeenCalledOnce()
    expect((mockApi.get.mock.calls[0] as string[])[0]).toBe('/api/test/endpoint')
    expect(result).toEqual({ data: 'get-result' })
  })

  it('POST 分支：带 body 转发到 api.post', async () => {
    const req = createRequest('/api/test')
    const result = await req('post', '/endpoint', { key: 'value' })
    expect(mockApi.post).toHaveBeenCalledOnce()
    const args = mockApi.post.mock.calls[0] as [string, unknown]
    expect(args[0]).toBe('/api/test/endpoint')
    expect(args[1]).toEqual({ key: 'value' })
    expect(result).toEqual({ data: 'post-result' })
  })

  it('POST 分支：body 为 undefined 时传 undefined', async () => {
    const req = createRequest('/api/test')
    const result = await req('post', '/endpoint')
    expect(mockApi.post).toHaveBeenCalledOnce()
    const args = mockApi.post.mock.calls[0] as [string, unknown]
    expect(args[1]).toBeUndefined()
    expect(result).toEqual({ data: 'post-result' })
  })

  it('PUT 分支：带 body 转发到 api.put', async () => {
    const req = createRequest('/api/test')
    const result = await req('put', '/endpoint', { update: true })
    expect(mockApi.put).toHaveBeenCalledOnce()
    const args = mockApi.put.mock.calls[0] as [string, unknown]
    expect(args[0]).toBe('/api/test/endpoint')
    expect(args[1]).toEqual({ update: true })
    expect(result).toEqual({ data: 'put-result' })
  })

  it('DELETE 分支：无 body 转发到 api.delete', async () => {
    const req = createRequest('/api/test')
    const result = await req('delete', '/endpoint')
    expect(mockApi.delete).toHaveBeenCalledOnce()
    expect((mockApi.delete.mock.calls[0] as string[])[0]).toBe('/api/test/endpoint')
    expect(result).toEqual({ data: 'delete-result' })
  })

  it('default 分支：小写方法 fallback 到 api.get', async () => {
    const req = createRequest('/api/test')
    const result = await req('patch', '/endpoint', { op: 'update' })
    // 非标准方法走 default -> api.get
    expect(mockApi.get).toHaveBeenCalledOnce()
    expect((mockApi.get.mock.calls[0] as string[])[0]).toBe('/api/test/endpoint')
    expect(result).toEqual({ data: 'get-result' })
  })

  it('混合分支：方法大小写不敏感，方法名被 toUpperCase', async () => {
    const req = createRequest('/api/test')
    // 'PoSt' -> toUpperCase -> 'POST'
    const result = await req('PoSt', '/endpoint', { mixed: true })
    expect(mockApi.post).toHaveBeenCalledOnce()
    expect((mockApi.post.mock.calls[0] as string[])[0]).toBe('/api/test/endpoint')
    expect(result).toEqual({ data: 'post-result' })
  })

  it('mixed case GET: GeT also hits GET branch', async () => {
    const req = createRequest('/api/test')
    const result = await req('GeT', '/endpoint')
    expect(mockApi.get).toHaveBeenCalledOnce()
    expect((mockApi.get.mock.calls[0] as string[])[0]).toBe('/api/test/endpoint')
    expect(result).toEqual({ data: 'get-result' })
  })

  it('body 为 null 时 hasBody 为 false', async () => {
    const req = createRequest('/api/test')
    const result = await req('post', '/endpoint', null)
    expect(mockApi.post).toHaveBeenCalledOnce()
    const args = mockApi.post.mock.calls[0] as [string, unknown]
    expect(args[1]).toBeUndefined() // null 被过滤为 undefined
    expect(result).toEqual({ data: 'post-result' })
  })

  it('url 拼接：空路径只返回 prefix', async () => {
    const req = createRequest('/api/test')
    const result = await req('get', '')
    expect(mockApi.get).toHaveBeenCalledOnce()
    expect((mockApi.get.mock.calls[0] as string[])[0]).toBe('/api/test')
    expect(result).toEqual({ data: 'get-result' })
  })
})
