import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { ApiClient, AuthenticationError, PermissionError } from './api'

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
    ['', '/jaxrs/authentication/who', 'https://web.example.test/jaxrs/authentication/who'],
    ['/jaxrs', '/jaxrs/authentication/who', 'https://web.example.test/jaxrs/authentication/who'],
    ['https://api.example.test/root/jaxrs', '/jaxrs/x', 'https://api.example.test/root/jaxrs/x'],
  ])('resolves base %s and path %s without duplicating prefixes', async (base, path, expected) => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }))
    vi.stubGlobal('fetch', fetchMock)

    await new ApiClient(base).get(path)

    expect(fetchMock).toHaveBeenCalledWith(expected, expect.any(Object))
  })

  it('encodes query parameters', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }))
    vi.stubGlobal('fetch', fetchMock)

    await new ApiClient().get('/jaxrs/search', { params: { keyword: 'a b&c' } })

    expect(fetchMock.mock.calls[0][0]).toBe('https://web.example.test/jaxrs/search?keyword=a+b%26c')
  })

  it('uses cookies without reading storage or adding Authorization', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }))
    vi.stubGlobal('fetch', fetchMock)

    await new ApiClient().get('/jaxrs/private')

    expect(fetchMock.mock.calls[0][1]).toMatchObject({ credentials: 'include' })
    expect(fetchMock.mock.calls[0][1].headers).not.toHaveProperty('Authorization')
  })

  it('returns the complete API response envelope', async () => {
    const body = { success: true, data: { value: 'ok' } }
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(jsonResponse(body)))

    await expect(new ApiClient().get('/jaxrs/x')).resolves.toEqual(body)
  })

  it('can discard a legacy token response without parsing it', async () => {
    const response = new Response('not-json')
    const jsonSpy = vi.spyOn(response, 'json')
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(response))

    await expect(
      new ApiClient().post(
        '/jaxrs/authentication/login',
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
      if (path === '/jaxrs/authentication/refresh') {
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
    await expect(Promise.all([client.get('/jaxrs/one'), client.get('/jaxrs/two')])).resolves.toHaveLength(2)

    const paths = fetchMock.mock.calls.map(([input]) => new URL(input).pathname)
    expect(paths.filter((path) => path === '/jaxrs/authentication/refresh')).toHaveLength(1)
    expect(paths.filter((path) => path === '/jaxrs/one')).toHaveLength(2)
    expect(paths.filter((path) => path === '/jaxrs/two')).toHaveLength(2)
  })

  it('does not refresh again after the single replay is unauthorized', async () => {
    const fetchMock = vi
      .fn()
      .mockImplementation(async (input: string) =>
        new URL(input).pathname.endsWith('/refresh') ? new Response('ok') : jsonResponse({}, 401),
      )
    vi.stubGlobal('fetch', fetchMock)

    await expect(new ApiClient().get('/jaxrs/private')).rejects.toBeInstanceOf(AuthenticationError)
    expect(fetchMock).toHaveBeenCalledTimes(3)
  })

  it('does not replay or recurse when refresh fails', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({}, 401))
    vi.stubGlobal('fetch', fetchMock)

    await expect(new ApiClient().get('/jaxrs/private')).rejects.toBeInstanceOf(AuthenticationError)
    expect(fetchMock).toHaveBeenCalledTimes(2)
  })

  it('does not refresh public requests and preserves forbidden errors', async () => {
    const fetchMock = vi.fn().mockResolvedValueOnce(jsonResponse({}, 401)).mockResolvedValueOnce(jsonResponse({}, 403))
    vi.stubGlobal('fetch', fetchMock)
    await expect(new ApiClient().get('/jaxrs/public', { requireAuth: false })).rejects.toBeInstanceOf(
      AuthenticationError,
    )
    expect(fetchMock).toHaveBeenCalledTimes(1)

    await expect(new ApiClient().get('/jaxrs/private', { requireAuth: false })).rejects.toBeInstanceOf(PermissionError)
  })

  it('uses cookie credentials and no Authorization for uploads', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }))
    vi.stubGlobal('fetch', fetchMock)

    await new ApiClient('/jaxrs').upload('/jaxrs/file', new FormData())

    expect(fetchMock).toHaveBeenCalledWith(
      'https://web.example.test/jaxrs/file',
      expect.objectContaining({
        credentials: 'include',
        headers: {},
      }),
    )
  })
})
