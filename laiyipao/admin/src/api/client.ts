/**
 * 统一 API 客户端。
 * 约定：后端错误统一为 { error: { code, message } }，非 2xx 一律抛 ApiError。
 */

export const API_BASE = import.meta.env.VITE_API_BASE || '/api/v1'

export class ApiError extends Error {
  constructor(
    readonly status: number,
    readonly code: string,
    message: string,
  ) {
    super(message)
    this.name = 'ApiError'
  }
}

const TOKEN_KEY = 'lyp_admin_token'

export function getToken(): string {
  try {
    return localStorage.getItem(TOKEN_KEY) || ''
  } catch {
    return ''
  }
}

export function setToken(token: string): void {
  try {
    if (token) localStorage.setItem(TOKEN_KEY, token)
    else localStorage.removeItem(TOKEN_KEY)
  } catch {
    /* 隐私模式下 localStorage 不可用，退化为内存态 */
  }
}

async function request<T>(
  method: string,
  path: string,
  body?: unknown,
  opts: { auth?: boolean } = {},
): Promise<T> {
  const headers: Record<string, string> = { Accept: 'application/json' }
  if (body !== undefined) headers['Content-Type'] = 'application/json'
  if (opts.auth !== false) {
    const token = getToken()
    if (token) headers.Authorization = `Bearer ${token}`
  }

  let resp: Response
  try {
    resp = await fetch(`${API_BASE}${path}`, {
      method,
      headers,
      body: body === undefined ? undefined : JSON.stringify(body),
    })
  } catch (e) {
    throw new ApiError(0, 'network_error', `无法连接后端服务：${(e as Error).message}`)
  }

  const text = await resp.text()
  let payload: any = null
  if (text) {
    try {
      payload = JSON.parse(text)
    } catch {
      throw new ApiError(resp.status, 'bad_json', `响应不是合法 JSON：${text.slice(0, 200)}`)
    }
  }

  if (!resp.ok) {
    const err = payload?.error
    throw new ApiError(
      resp.status,
      err?.code || 'http_error',
      err?.message || `请求失败 HTTP ${resp.status}`,
    )
  }
  return payload as T
}

export const api = {
  get: <T>(path: string, opts?: { auth?: boolean }) => request<T>('GET', path, undefined, opts),
  post: <T>(path: string, body?: unknown, opts?: { auth?: boolean }) =>
    request<T>('POST', path, body, opts),
  put: <T>(path: string, body?: unknown, opts?: { auth?: boolean }) =>
    request<T>('PUT', path, body, opts),
  del: <T>(path: string, opts?: { auth?: boolean }) => request<T>('DELETE', path, undefined, opts),
}
