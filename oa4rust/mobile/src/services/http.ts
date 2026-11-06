// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

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

/**
 * R138：后端安全修复后 403/404 是写端点的正常业务结果（无权限 / 目标已不存在），
 * 文案必须可读；其它状态码保留 `HTTP xxx` 形状，避免影响页面既有错误判定。
 */
const FORBIDDEN_MESSAGE = '没有权限执行该操作'
const NOT_FOUND_MESSAGE = '请求的资源不存在或已被删除'

function httpStatusError(status: number): ApiError {
  return status === 404 ? new ApiError(NOT_FOUND_MESSAGE, 404) : new ApiError(`HTTP ${status}`, status)
}

let apiBase = ''
let authFailureHandler: (() => void) | null = null
let refreshPromise: Promise<void> | null = null

/** 设置后端绝对地址（原生 App / 小程序目标）。H5 目标走同源代理，保持默认空串即可。 */
export function setApiBase(base: string): void {
  apiBase = base.replace(/\/+$/, '')
  // 安全审计（2026-10）：明文 http:// 绝对地址会在生产环境传输登录凭据/会话 cookie。
  // 仅允许 localhost/127.0.0.1 形式的 dev 地址走 http；其余一律告警提示改用 https。
  if (apiBase.startsWith('http://') && !/^http:\/\/(localhost|127\.0\.0\.1)(:|$)/.test(apiBase)) {
    console.warn(
      '[oa4rust-mobile] insecure http:// API base detected:',
      apiBase,
      '— use https:// in production to protect credentials and session cookies',
    )
  }
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
  return path === '/api/authentication/refresh'
}

/**
 * 网络失败文案归一化（R136）。uni.request / uni.uploadFile 的 `errMsg` 是平台英文串
 *（实测形态如 `request:fail timeout`），旧实现直接塞进 `ApiError.message`，
 * 页面再拼成「加载失败：request:fail timeout」给用户看。传输层是唯一出口，在这里归一化。
 */
function transportFailureMessage(raw: string | undefined, timeoutMessage: string, fallbackMessage: string): string {
  return raw && /timeout/i.test(raw) ? timeoutMessage : fallbackMessage
}

function authenticationFailed(): AuthenticationError {
  authFailureHandler?.()
  return new AuthenticationError('Session expired, please login again')
}

async function refreshSession(): Promise<void> {
  if (!refreshPromise) {
    refreshPromise = performRequest<never>('POST', '/api/authentication/refresh', null, {
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
          reject(new PermissionError(FORBIDDEN_MESSAGE))
          return
        }
        reject(httpStatusError(status))
      },
      fail: (err: UniApp.GeneralCallbackResult) => {
        reject(
          new ApiError(
            transportFailureMessage(err.errMsg, '请求超时，请检查网络后重试', '网络连接失败，请检查网络后重试'),
            0,
          ),
        )
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
        if (status === 401) {
          if (options.requireAuth !== false && !retried) {
            refreshSession()
              .then(() => performUpload<T>(path, filePath, options, true).then(resolve, reject))
              .catch(() => reject(authenticationFailed()))
            return
          }
          reject(authenticationFailed())
          return
        }
        if (status === 403) {
          reject(new PermissionError(FORBIDDEN_MESSAGE))
          return
        }
        reject(httpStatusError(status))
      },
      fail: (err: UniApp.GeneralCallbackResult) => {
        reject(
          new ApiError(
            transportFailureMessage(err.errMsg, '上传超时，请检查网络后重试', '上传失败，请检查网络后重试'),
            0,
          ),
        )
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
