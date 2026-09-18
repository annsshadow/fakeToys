import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiClient, ApiError, AuthenticationError, PermissionError } from './api'

function jsonResponse(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { 'Content-Type': 'application/json' },
  })
}

describe('ApiClient', () => {
  beforeEach(() => {
    vi.stubGlobal('window', { location: { origin: 'https://web.example.test' } })
    vi.stubGlobal('localStorage', {
      getItem: () => {
        throw new Error('must not read localStorage')
      },
    })
    vi.stubGlobal('sessionStorage', {
      getItem: () => {
        throw new Error('must not read sessionStorage')
      },
    })
  })

  afterEach(() => {
    vi.unstubAllGlobals()
    vi.restoreAllMocks()
  })

  it.each([
    ['', '/api/authentication/who', 'https://web.example.test/api/authentication/who'],
    ['/api', '/api/authentication/who', 'https://web.example.test/api/authentication/who'],
    ['https://api.example.test/root/api', '/api/x', 'https://api.example.test/root/api/x'],
  ])('resolves base %s and path %s without duplicating prefixes', async (base, path, expected) => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }))
    vi.stubGlobal('fetch', fetchMock)

    await new ApiClient(base).get(path)

    expect(fetchMock).toHaveBeenCalledWith(expected, expect.any(Object))
  })

  it('encodes query parameters', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }))
    vi.stubGlobal('fetch', fetchMock)

    await new ApiClient().get('/api/search', { params: { keyword: 'a b&c' } })

    expect(fetchMock.mock.calls[0][0]).toBe('https://web.example.test/api/search?keyword=a+b%26c')
  })

  it('uses cookies without reading storage or adding Authorization', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }))
    vi.stubGlobal('fetch', fetchMock)

    await new ApiClient().get('/api/private')

    expect(fetchMock.mock.calls[0][1]).toMatchObject({ credentials: 'include' })
    expect(fetchMock.mock.calls[0][1].headers).not.toHaveProperty('Authorization')
  })

  it('returns the complete API response envelope', async () => {
    const body = { success: true, data: { value: 'ok' } }
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(jsonResponse(body)))

    await expect(new ApiClient().get('/api/x')).resolves.toEqual(body)
  })

  it('can discard a legacy token response without parsing it', async () => {
    const response = new Response('not-json')
    const jsonSpy = vi.spyOn(response, 'json')
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(response))

    await expect(
      new ApiClient().post(
        '/api/authentication/login',
        {},
        {
          requireAuth: false,
          discardResponse: true,
        },
      ),
    ).resolves.toEqual({ success: true, data: undefined })
    expect(jsonSpy).not.toHaveBeenCalled()
  })

  it('single-flights refresh and replays each concurrent request once', async () => {
    const attempts = new Map<string, number>()
    const fetchMock = vi.fn().mockImplementation(async (input: string, init: RequestInit) => {
      const path = new URL(input).pathname
      if (path === '/api/authentication/refresh') {
        expect(init).toMatchObject({ method: 'POST', credentials: 'include' })
        expect(init).not.toHaveProperty('body')
        return new Response('legacy-token-json')
      }
      const count = (attempts.get(path) ?? 0) + 1
      attempts.set(path, count)
      return count === 1 ? jsonResponse({}, 401) : jsonResponse({ success: true, data: path })
    })
    vi.stubGlobal('fetch', fetchMock)

    const client = new ApiClient()
    await expect(Promise.all([client.get('/api/one'), client.get('/api/two')])).resolves.toHaveLength(2)

    const paths = fetchMock.mock.calls.map(([input]) => new URL(input).pathname)
    expect(paths.filter((path) => path === '/api/authentication/refresh')).toHaveLength(1)
    expect(paths.filter((path) => path === '/api/one')).toHaveLength(2)
    expect(paths.filter((path) => path === '/api/two')).toHaveLength(2)
  })

  it('does not refresh again after the single replay is unauthorized', async () => {
    const fetchMock = vi
      .fn()
      .mockImplementation(async (input: string) =>
        new URL(input).pathname.endsWith('/refresh') ? new Response('ok') : jsonResponse({}, 401),
      )
    vi.stubGlobal('fetch', fetchMock)

    await expect(new ApiClient().get('/api/private')).rejects.toBeInstanceOf(AuthenticationError)
    expect(fetchMock).toHaveBeenCalledTimes(3)
  })

  it('does not replay or recurse when refresh fails', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({}, 401))
    vi.stubGlobal('fetch', fetchMock)

    await expect(new ApiClient().get('/api/private')).rejects.toBeInstanceOf(AuthenticationError)
    expect(fetchMock).toHaveBeenCalledTimes(2)
  })

  it('does not refresh public requests and preserves forbidden errors', async () => {
    const fetchMock = vi.fn().mockResolvedValueOnce(jsonResponse({}, 401)).mockResolvedValueOnce(jsonResponse({}, 403))
    vi.stubGlobal('fetch', fetchMock)
    await expect(new ApiClient().get('/api/public', { requireAuth: false })).rejects.toBeInstanceOf(AuthenticationError)
    expect(fetchMock).toHaveBeenCalledTimes(1)

    await expect(new ApiClient().get('/api/private', { requireAuth: false })).rejects.toBeInstanceOf(PermissionError)
  })

  it('uses cookie credentials and no Authorization for uploads', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }))
    vi.stubGlobal('fetch', fetchMock)

    await new ApiClient('/api').upload('/api/file', new FormData())

    expect(fetchMock).toHaveBeenCalledWith(
      'https://web.example.test/api/file',
      expect.objectContaining({
        credentials: 'include',
        headers: {},
      }),
    )
  })

  it('passes an absolute URL path through without base joining', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }))
    vi.stubGlobal('fetch', fetchMock)

    await new ApiClient('/api').get('https://remote.example.test/data')

    expect(fetchMock).toHaveBeenCalledWith('https://remote.example.test/data', expect.any(Object))
  })

  it('aborts a slow request once timeoutMs elapses', async () => {
    const fetchMock = vi.fn(
      (_url: string | URL, init: RequestInit) =>
        new Promise<Response>((resolve) => {
          init.signal?.addEventListener('abort', () => resolve(new Response(null, { status: 204 })))
        }),
    )
    vi.stubGlobal('fetch', fetchMock)

    const resp = await new ApiClient().get('/slow', { timeoutMs: 30 })

    expect(fetchMock).toHaveBeenCalledTimes(1)
    expect(resp.success).toBe(true)
  })

  it('retries an upload exactly once after a 401 refresh, replaying the same URL', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(jsonResponse({}, 401))
      .mockResolvedValueOnce(new Response(null, { status: 204 }))
      .mockResolvedValueOnce(jsonResponse({ success: true, data: { sent: true } }))
    vi.stubGlobal('fetch', fetchMock)

    const resp = await new ApiClient().upload('/api/attachment/upload/folder/1', new FormData())

    expect(resp.success).toBe(true)
    const urls = fetchMock.mock.calls.map((c) => String(c[0]))
    expect(urls).toEqual([
      'https://web.example.test/api/attachment/upload/folder/1',
      'https://web.example.test/api/authentication/refresh',
      'https://web.example.test/api/attachment/upload/folder/1',
    ])
  })

  it('surfaces AuthenticationError without a refresh attempt when the upload is public', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({}, 401))
    vi.stubGlobal('fetch', fetchMock)

    await expect(
      new ApiClient().upload('/api/pub-upload', new FormData(), { requireAuth: false }),
    ).rejects.toBeInstanceOf(AuthenticationError)
    expect(fetchMock).toHaveBeenCalledTimes(1)
  })

  it('maps upload 403 to PermissionError and other statuses to ApiError', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(jsonResponse({}, 403)))
    await expect(new ApiClient().upload('/api/u', new FormData())).rejects.toBeInstanceOf(PermissionError)
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(jsonResponse({}, 500)))
    await expect(new ApiClient().upload('/api/u', new FormData())).rejects.toBeInstanceOf(ApiError)
  })

  it('put() sends a JSON body and delete() sends none, on the correct verbs', async () => {
    // 每次调用返回全新 Response：同一 Response 的 body 只能被读一次。
    const fetchMock = vi.fn().mockImplementation(() => Promise.resolve(jsonResponse({ success: true, data: {} })))
    vi.stubGlobal('fetch', fetchMock)
    const client = new ApiClient()
    await client.put('/api/item/1', { name: 'x' })
    await client.delete('/api/item/1')
    const inits = fetchMock.mock.calls.map((c) => c[1] as RequestInit)
    expect(inits[0].method).toBe('PUT')
    expect(inits[0].body).toBe(JSON.stringify({ name: 'x' }))
    expect(inits[1].method).toBe('DELETE')
    expect(inits[1].body).toBeUndefined()
  })

  it('aborts an upload once its own timeoutMs elapses', async () => {
    const fetchMock = vi.fn(
      (_url: string | URL, init: RequestInit) =>
        new Promise<Response>((resolve) => {
          // upload() 成功路径总走 resp.json()（不像 request() 特判 204），
          // 故 abort 后必须用带 JSON 体的响应收尾，否则 body 为空会抛错。
          init.signal?.addEventListener('abort', () => resolve(jsonResponse({ success: true, data: { sent: true } })))
        }),
    )
    vi.stubGlobal('fetch', fetchMock)
    const resp = await new ApiClient().upload('/api/attachment/upload/folder/1', new FormData(), {
      timeoutMs: 30,
    })
    expect(fetchMock).toHaveBeenCalledTimes(1)
    expect(resp.success).toBe(true)
  })

  it('rejects the upload with AuthenticationError when the 401-triggered refresh fails', async () => {
    const fetchMock = vi
      .fn()
      .mockResolvedValueOnce(jsonResponse({}, 401)) // upload 首次 401
      .mockResolvedValue(jsonResponse({}, 500)) // refresh 失败
    vi.stubGlobal('fetch', fetchMock)
    await expect(new ApiClient().upload('/api/u', new FormData())).rejects.toBeInstanceOf(AuthenticationError)
    expect(fetchMock).toHaveBeenCalledTimes(2)
  })
})
