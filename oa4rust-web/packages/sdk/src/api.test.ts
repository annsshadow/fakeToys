import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { ApiClient, AuthenticationError, PermissionError } from './api';

const storage = new Map<string, string>();

function jsonResponse(body: unknown, status = 200): Response {
  return new Response(JSON.stringify(body), {
    status,
    headers: { 'Content-Type': 'application/json' },
  });
}

describe('ApiClient', () => {
  beforeEach(() => {
    storage.clear();
    vi.stubGlobal('window', { location: { origin: 'https://web.example.test' } });
    vi.stubGlobal('localStorage', {
      getItem: (key: string) => storage.get(key) ?? null,
      setItem: (key: string, value: string) => storage.set(key, value),
      removeItem: (key: string) => storage.delete(key),
    });
  });

  afterEach(() => {
    vi.unstubAllGlobals();
    vi.restoreAllMocks();
  });

  it.each([
    ['', '/jaxrs/authentication/who', 'https://web.example.test/jaxrs/authentication/who'],
    ['/jaxrs', '/jaxrs/authentication/who', 'https://web.example.test/jaxrs/authentication/who'],
    ['https://api.example.test', '/jaxrs/x', 'https://api.example.test/jaxrs/x'],
    ['https://api.example.test/root/', '/jaxrs/x', 'https://api.example.test/root/jaxrs/x'],
    ['https://api.example.test/root/jaxrs', '/jaxrs/x', 'https://api.example.test/root/jaxrs/x'],
  ])('resolves base %s and path %s without duplicating prefixes', async (base, path, expected) => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }));
    vi.stubGlobal('fetch', fetchMock);

    await new ApiClient(base).get(path);

    expect(fetchMock).toHaveBeenCalledWith(expected, expect.any(Object));
  });

  it('encodes query parameters', async () => {
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }));
    vi.stubGlobal('fetch', fetchMock);

    await new ApiClient().get('/jaxrs/search', { params: { keyword: 'a b&c' } });

    expect(fetchMock.mock.calls[0][0]).toBe('https://web.example.test/jaxrs/search?keyword=a+b%26c');
  });

  it('sends bearer authentication by default', async () => {
    storage.set('oa4rust_session', JSON.stringify({ token: 'secret' }));
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }));
    vi.stubGlobal('fetch', fetchMock);

    await new ApiClient().get('/jaxrs/private');

    expect(fetchMock.mock.calls[0][1]).toMatchObject({
      credentials: 'include',
      headers: expect.objectContaining({ Authorization: 'Bearer secret' }),
    });
  });

  it('omits bearer authentication when requireAuth is false', async () => {
    storage.set('oa4rust_session', JSON.stringify({ token: 'secret' }));
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }));
    vi.stubGlobal('fetch', fetchMock);

    await new ApiClient().get('/jaxrs/public', { requireAuth: false });

    expect(fetchMock.mock.calls[0][1]).toMatchObject({ credentials: 'include' });
    expect(fetchMock.mock.calls[0][1].headers).not.toHaveProperty('Authorization');
  });

  it('returns the complete API response envelope', async () => {
    const body = { success: true, data: { token: 'value' } };
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(jsonResponse(body)));

    await expect(new ApiClient().post<{ token: string }>('/jaxrs/login')).resolves.toEqual(body);
  });

  it('clears an invalid session on 401', async () => {
    storage.set('oa4rust_session', JSON.stringify({ token: 'expired' }));
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(jsonResponse({}, 401)));

    await expect(new ApiClient().get('/jaxrs/private')).rejects.toBeInstanceOf(AuthenticationError);
    expect(storage.has('oa4rust_session')).toBe(false);
  });

  it('uses a dedicated error for forbidden responses', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(jsonResponse({}, 403)));

    await expect(new ApiClient().get('/jaxrs/private')).rejects.toBeInstanceOf(PermissionError);
  });

  it('uses the same URL and authentication rules for uploads', async () => {
    storage.set('oa4rust_session', JSON.stringify({ token: 'secret' }));
    const fetchMock = vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} }));
    vi.stubGlobal('fetch', fetchMock);

    await new ApiClient('/jaxrs').upload('/jaxrs/file', new FormData(), { requireAuth: false });

    expect(fetchMock).toHaveBeenCalledWith(
      'https://web.example.test/jaxrs/file',
      expect.objectContaining({ headers: {} }),
    );
  });

  it('accepts timeoutMs option without error', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue(jsonResponse({ success: true, data: {} })));
    await expect(new ApiClient().get('/jaxrs/x', { timeoutMs: 5000 })).resolves.toBeDefined();
  });
});
