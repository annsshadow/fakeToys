/**
 * api/client.ts 单元测试。
 *
 * 为什么测这么细：client 是全后台唯一的数据通道，错误分类（network_error / bad_json /
 * http_error / 后端 error.code）直接决定每个视图的报错文案；令牌注入与否决定鉴权是否生效。
 * fetch 全程用受控替身，不发出真实网络请求。
 */
import { beforeEach, describe, expect, it, vi } from 'vitest'

/** 每个用例重新导入被测模块：API_BASE 在模块加载期求值，需要随环境变量重算 */
async function freshClient() {
  vi.resetModules()
  return await import('@/api/client')
}

type Client = Awaited<ReturnType<typeof freshClient>>

/** 构造一个最小的 fetch Response 替身 */
function mockResp(opts: { ok: boolean; status: number; text: string }) {
  return { ok: opts.ok, status: opts.status, text: async () => opts.text }
}

beforeEach(() => {
  // 先解除替身再清理：上一用例可能 stub 过 localStorage（无 clear 方法）
  vi.unstubAllGlobals()
  vi.unstubAllEnvs()
  localStorage.clear()
})

describe('api/client 令牌存取', () => {
  it('无令牌时 getToken 返回空串', async () => {
    const { getToken } = await freshClient()
    expect(getToken()).toBe('')
  })

  it('setToken 写入、getToken 读出，键名固定', async () => {
    const { getToken, setToken } = await freshClient()
    setToken('tok-1')
    expect(localStorage.getItem('lyp_admin_token')).toBe('tok-1')
    expect(getToken()).toBe('tok-1')
  })

  it('setToken 传空串时移除令牌（登出语义）', async () => {
    const { getToken, setToken } = await freshClient()
    setToken('tok-1')
    setToken('')
    expect(localStorage.getItem('lyp_admin_token')).toBeNull()
    expect(getToken()).toBe('')
  })

  it('隐私模式下 localStorage 抛异常：getToken 返回空串、setToken 静默降级不抛错', async () => {
    // jsdom 的 localStorage 不会抛错；用抛错替身模拟 Safari 隐私模式
    vi.stubGlobal(
      'localStorage',
      Object.defineProperties(
        {},
        {
          getItem: { value: () => { throw new Error('SecurityError') } },
          setItem: { value: () => { throw new Error('SecurityError') } },
          removeItem: { value: () => { throw new Error('SecurityError') } },
        },
      ),
    )
    const { getToken, setToken } = await freshClient()
    expect(getToken()).toBe('')
    expect(() => setToken('tok')).not.toThrow()
    expect(() => setToken('')).not.toThrow()
  })
})

describe('api/client request', () => {
  let client: Client
  const fetchMock = vi.fn()

  beforeEach(async () => {
    client = await freshClient()
    fetchMock.mockReset()
    vi.stubGlobal('fetch', fetchMock)
  })

  it('GET 成功：URL 拼接 API_BASE，带 Accept 头，无令牌不带 Authorization，返回解析后的 payload', async () => {
    const sentinel = { items: [1, 2] }
    fetchMock.mockResolvedValueOnce(mockResp({ ok: true, status: 200, text: JSON.stringify(sentinel) }))

    await expect(client.api.get('/admin/users')).resolves.toEqual(sentinel)

    const [url, init] = fetchMock.mock.calls[0]
    expect(url).toBe('/api/v1/admin/users')
    expect(init.method).toBe('GET')
    expect(init.headers.Accept).toBe('application/json')
    expect(init.headers['Content-Type']).toBeUndefined()
    expect(init.headers.Authorization).toBeUndefined()
    expect(init.body).toBeUndefined()
  })

  it('有令牌时注入 Authorization: Bearer', async () => {
    client.setToken('tok-9')
    fetchMock.mockResolvedValueOnce(mockResp({ ok: true, status: 200, text: '{}' }))
    await client.api.get('/admin/me')
    expect(fetchMock.mock.calls[0][1].headers.Authorization).toBe('Bearer tok-9')
  })

  it('POST 带 body：加 Content-Type 并序列化 JSON', async () => {
    fetchMock.mockResolvedValueOnce(mockResp({ ok: true, status: 200, text: 'null' }))
    await client.api.post('/admin/users/1/ban', { reason: '刷分' })
    const [, init] = fetchMock.mock.calls[0]
    expect(init.method).toBe('POST')
    expect(init.headers['Content-Type']).toBe('application/json')
    expect(init.body).toBe(JSON.stringify({ reason: '刷分' }))
  })

  it('opts.auth = false 时不注入令牌（登录接口本身的语义）', async () => {
    client.setToken('stale-token')
    fetchMock.mockResolvedValueOnce(mockResp({ ok: true, status: 200, text: '{}' }))
    await client.api.post('/admin/login', { username: 'a', password: 'b' }, { auth: false })
    expect(fetchMock.mock.calls[0][1].headers.Authorization).toBeUndefined()
  })

  it('网络错误（fetch reject）归类为 network_error，status 0', async () => {
    fetchMock.mockRejectedValueOnce(new TypeError('Failed to fetch'))
    const err = await client.api.get('/admin/dashboard').catch((e) => e)
    expect(err).toMatchObject({ name: 'ApiError', status: 0, code: 'network_error' })
    expect(err.message).toContain('无法连接后端服务')
    expect(err.message).toContain('Failed to fetch')
  })

  it('非 2xx 且后端返回标准错误体：透传 status / code / message', async () => {
    fetchMock.mockResolvedValueOnce(
      mockResp({
        ok: false,
        status: 403,
        text: JSON.stringify({ error: { code: 'forbidden', message: '没有权限' } }),
      }),
    )
    const err = await client.api.get('/admin/users').catch((e) => e)
    expect(err).toMatchObject({ status: 403, code: 'forbidden', message: '没有权限' })
  })

  it('非 2xx 但错误体不是约定结构：回退到 http_error 与默认文案', async () => {
    fetchMock.mockResolvedValueOnce(
      mockResp({ ok: false, status: 500, text: JSON.stringify({ foo: 1 }) }),
    )
    const err = await client.api.get('/admin/users').catch((e) => e)
    expect(err).toMatchObject({ status: 500, code: 'http_error', message: '请求失败 HTTP 500' })
  })

  it('响应体不是合法 JSON：抛 bad_json，消息里保留原文片段', async () => {
    const raw = `<html>${'x'.repeat(300)}</html>`
    fetchMock.mockResolvedValueOnce(mockResp({ ok: true, status: 200, text: raw }))
    const err = await client.api.get('/admin/dashboard').catch((e) => e)
    expect(err).toMatchObject({ status: 200, code: 'bad_json' })
    expect(err.message).toContain('响应不是合法 JSON')
    // slice(0, 200) 截断：消息不应包含整段原文
    expect(err.message.length).toBeLessThan(raw.length)
    expect(err.message).toContain('<html>')
  })

  it('2xx 空 body（204 风格）：返回 null 而不是抛错', async () => {
    fetchMock.mockResolvedValueOnce(mockResp({ ok: true, status: 204, text: '' }))
    await expect(client.api.del('/admin/cache')).resolves.toBeNull()
  })

  it('api.put / api.del 方法与路径映射正确', async () => {
    fetchMock.mockResolvedValue(mockResp({ ok: true, status: 200, text: 'null' }))
    await client.api.put('/admin/levels/3', { base_hp: 500 })
    expect(fetchMock.mock.calls[0][0]).toBe('/api/v1/admin/levels/3')
    expect(fetchMock.mock.calls[0][1].method).toBe('PUT')
    await client.api.del('/admin/announcements/2')
    expect(fetchMock.mock.calls[1][1].method).toBe('DELETE')
  })

  it('VITE_API_BASE 未设置时默认 /api/v1', async () => {
    const { API_BASE } = await freshClient()
    expect(API_BASE).toBe('/api/v1')
  })

  it('VITE_API_BASE 设置时覆盖默认值（生产同域反代场景）', async () => {
    vi.stubEnv('VITE_API_BASE', 'https://ops.example.com/api/v1')
    const { API_BASE } = await freshClient()
    expect(API_BASE).toBe('https://ops.example.com/api/v1')
  })

  it('ApiError 是 Error 的子类，便于上游按 Error 统一捕获', async () => {
    const { ApiError } = await freshClient()
    const e = new ApiError(418, 'teapot', 'short and stout')
    expect(e).toBeInstanceOf(Error)
    expect(e.message).toBe('short and stout')
  })
})
