/**
 * 移动端统一 API 传输层。
 *
 * 与桌面端 @oa4rust/sdk 的 ApiClient 语义对齐（401 自动 refresh、403 权限、统一错误类、
 * ApiResponse 结构），但底层使用 uni.request 以跨 H5 / App / 小程序多平台运行。
 * 认证模型与 oa4rust 后端一致：H5 目标携带同源 Cookie（withCredentials）；
 * 原生 / 小程序目标需经 setApiBase 指向后端绝对地址。
 */

import type { ApiResponse } from '@oa4rust/sdk'

export interface MobileRequestOptions {
  params?: Record<string, string>
  /** 设为 false 可跳过 401 时的自动 refresh（登录 / 刷新 / who 等端点）。 */
  requireAuth?: boolean
  headers?: Record<string, string>
  /** 请求超时（毫秒）。默认 30000。 */
  timeoutMs?: number
  /** 成功时不解析响应体（可能包含遗留 token 字段的认证端点）。 */
  discardResponse?: boolean
}

export class ApiError extends Error {
  readonly status: number
  constructor(message: string, status: number) {
    super(message)
    this.name = 'ApiError'
    this.status = status
  }
}

export class AuthenticationError extends ApiError {
  constructor(message: string) {
    super(message, 401)
    this.name = 'AuthenticationError'
  }
}

export class PermissionError extends ApiError {
  constructor(message: string) {
    super(message, 403)
    this.name = 'PermissionError'
  }
}

let apiBase = ''
let authFailureHandler: (() => void) | null = null
let refreshPromise: Promise<void> | null = null

/** 设置后端绝对地址（原生 App / 小程序目标）。H5 目标走同源代理，保持默认空串即可。 */
export function setApiBase(base: string): void {
  apiBase = base.replace(/\/+$/, '')
}

export function getApiBase(): string {
  return apiBase
}

/** 认证失败（refresh 后仍 401）回调，会话存储据此清除本地用户。 */
export function setAuthenticationFailureHandler(handler: (() => void) | null): void {
  authFailureHandler = handler
}

function buildUrl(path: string, params?: Record<string, string>): string {
  const endpoint = path.startsWith('/') ? path : `/${path}`
  const url = apiBase ? `${apiBase}${endpoint}` : endpoint
  if (!params) return url
  const entries = Object.entries(params).filter(([, v]) => v !== undefined && v !== null)
  if (entries.length === 0) return url
  const qs = entries.map(([k, v]) => `${encodeURIComponent(k)}=${encodeURIComponent(String(v))}`).join('&')
  return url + (url.includes('?') ? '&' : '?') + qs
}

function isRefreshRequest(path: string): boolean {
  return path === '/jaxrs/authentication/refresh'
}

function authenticationFailed(): AuthenticationError {
  authFailureHandler?.()
  return new AuthenticationError('Session expired, please login again')
}

async function refreshSession(): Promise<void> {
  if (!refreshPromise) {
    refreshPromise = performRequest<never>('POST', '/jaxrs/authentication/refresh', null, {
      requireAuth: false,
      discardResponse: true,
    })
      .then(() => undefined)
      .finally(() => {
        refreshPromise = null
      })
  }
  return refreshPromise
}

type HttpMethod = 'GET' | 'POST' | 'PUT' | 'DELETE'

function performRequest<T>(
  method: HttpMethod,
  path: string,
  body: unknown,
  options: MobileRequestOptions & { retried?: boolean },
): Promise<ApiResponse<T>> {
  return new Promise((resolve, reject) => {
    const url = buildUrl(path, options.params)
    const header: Record<string, string> = {
      'Content-Type': 'application/json',
      ...options.headers,
    }

    const reqOptions = {
      url,
      method,
      data: method !== 'GET' && body !== null && body !== undefined ? body : undefined,
      header,
      timeout: options.timeoutMs ?? 30000,
      withCredentials: true,
    } as UniApp.RequestOptions

    uni.request({
      ...reqOptions,
      success: (res: UniApp.RequestSuccessCallbackResult) => {
        const status = res.statusCode
        const data = res.data as ApiResponse<T> | unknown
        if (status >= 200 && status < 300) {
          if (options.discardResponse || status === 204) {
            resolve({ success: true, data: undefined as T })
            return
          }
          resolve(data as ApiResponse<T>)
          return
        }
        if (status === 401) {
          if (options.requireAuth !== false && !options.retried && !isRefreshRequest(path)) {
            refreshSession()
              .then(() => performRequest<T>(method, path, body, { ...options, retried: true }).then(resolve, reject))
              .catch(() => reject(authenticationFailed()))
            return
          }
          reject(authenticationFailed())
          return
        }
        if (status === 403) {
          reject(new PermissionError('Permission denied'))
          return
        }
        reject(new ApiError(`HTTP ${status}`, status))
      },
      fail: (err: UniApp.GeneralCallbackResult) => {
        reject(new ApiError(err.errMsg || 'Network error', 0))
      },
    })
  })
}

/** 文件上传（multipart）。移动端经 uni.uploadFile；filePath 为本地文件路径。 */
function performUpload<T>(
  path: string,
  filePath: string,
  options: MobileRequestOptions & { name?: string; formData?: Record<string, unknown> },
  retried = false,
): Promise<ApiResponse<T>> {
  return new Promise((resolve, reject) => {
    const url = buildUrl(path, options.params)
    uni.uploadFile({
      url,
      filePath,
      name: options.name ?? 'file',
      formData: options.formData as Record<string, string> | undefined,
      header: { ...options.headers },
      success: (res: UniApp.UploadFileSuccessCallbackResult) => {
        const status = res.statusCode
        let parsed: ApiResponse<T> | unknown = res.data
        if (typeof parsed === 'string') {
          try {
            parsed = JSON.parse(parsed)
          } catch {
            /* keep string */
          }
        }
        if (status >= 200 && status < 300) {
          resolve(parsed as ApiResponse<T>)
          return
        }
        if (status === 401 && options.requireAuth !== false && !retried) {
          refreshSession()
            .then(() => performUpload<T>(path, filePath, options, true).then(resolve, reject))
            .catch(() => reject(authenticationFailed()))
          return
        }
        if (status === 403) {
          reject(new PermissionError('Permission denied'))
          return
        }
        reject(new ApiError(`HTTP ${status}`, status))
      },
      fail: (err: UniApp.GeneralCallbackResult) => {
        reject(new ApiError(err.errMsg || 'Upload failed', 0))
      },
    })
  })
}

export interface MobileApiClient {
  get: <T>(path: string, options?: MobileRequestOptions) => Promise<ApiResponse<T>>
  post: <T>(path: string, body?: unknown, options?: MobileRequestOptions) => Promise<ApiResponse<T>>
  put: <T>(path: string, body?: unknown, options?: MobileRequestOptions) => Promise<ApiResponse<T>>
  delete: <T>(path: string, options?: MobileRequestOptions) => Promise<ApiResponse<T>>
  upload: <T>(
    path: string,
    filePath: string,
    options?: MobileRequestOptions & { name?: string; formData?: Record<string, unknown> },
  ) => Promise<ApiResponse<T>>
  refresh: () => Promise<void>
}

export const mapi: MobileApiClient = {
  get: <T>(path: string, options?: MobileRequestOptions) => performRequest<T>('GET', path, undefined, options ?? {}),
  post: <T>(path: string, body?: unknown, options?: MobileRequestOptions) =>
    performRequest<T>('POST', path, body, options ?? {}),
  put: <T>(path: string, body?: unknown, options?: MobileRequestOptions) =>
    performRequest<T>('PUT', path, body, options ?? {}),
  delete: <T>(path: string, options?: MobileRequestOptions) =>
    performRequest<T>('DELETE', path, undefined, options ?? {}),
  upload: <T>(
    path: string,
    filePath: string,
    options?: MobileRequestOptions & { name?: string; formData?: Record<string, unknown> },
  ) => performUpload<T>(path, filePath, options ?? {}),
  refresh: () => refreshSession(),
}
