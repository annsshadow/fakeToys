// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/** 统一 API 响应结构（与 oa4rust ActionResult<T> 对齐） */
export interface ApiResponse<T = unknown> {
  success: boolean
  data: T
  message?: string
  code?: number
}

/** 分页响应 */
export interface PagedResponse<T> {
  data: T[]
  total: number
  page: number
  size: number
}

/**
 * 默认请求超时（R133）。与移动端 `uni.request` 的 30s 对齐：
 * 服务端/网络一直不响应时，前端不能无限等（否则列表永远停在 loading，表现为白屏）。
 * 上传接口仍按需显式传 `timeoutMs`：大文件上传不该被 30s 误杀。
 */
const DEFAULT_TIMEOUT_MS = 30_000

/**
 * 流式响应默认空闲超时（R134）。SSE 是长连接，不能用「总时长」超时，
 * 但「连续一段时间一个字节都没收到」说明连接已死（半开 TCP / 代理掐断 /
 * 上游 LLM 不再吐数据），继续等只会让页面永远停在「生成中」。
 * 每收到一个分块重新计时；首字节之前同样生效（服务端连响应头都不发也算死连接）。
 * 后端 `chat_completion_stream` 未配置 SSE keep-alive，因此取一个宽于正常首字节延迟的值。
 */
const DEFAULT_STREAM_IDLE_TIMEOUT_MS = 60_000

/**
 * R138：后端安全修复后，写端点会把「无权限 / 目标已不存在」明确成 403/404。
 * 这两类不是异常网络状态，而是用户需要理解并据此行动的业务结果，因此给中文可读文案；
 * 其它状态码保留 `HTTP xxx` 形状，避免影响既有调用方的错误判定。
 */
const FORBIDDEN_MESSAGE = '没有权限执行该操作'
const NOT_FOUND_MESSAGE = '请求的资源不存在或已被删除'

function httpStatusError(status: number, statusText?: string): ApiError {
  if (status === 404) return new ApiError(NOT_FOUND_MESSAGE, 404)
  return new ApiError(statusText ? `HTTP ${status}: ${statusText}` : `HTTP ${status}`, status)
}

/** 查询选项（传给 TanStack Query） */
export interface QueryOptions {
  enabled?: boolean
  staleTime?: number
  cacheTime?: number
  retry?: number | boolean
  retryDelay?: number | ((attempt: number) => number)
  meta?: Record<string, unknown>
}

/** mutation 选项 */
export interface MutationOptions<TData = unknown, TVariables = unknown> {
  onSuccess?: (data: TData, variables: TVariables, context: unknown) => void
  onError?: (error: unknown, variables: TVariables, context: unknown) => void
  onSettled?: (data: TData | undefined, error: unknown, variables: TVariables, context: unknown) => void
}

/**
 * 带自动认证头的 fetch 封装
 * 等价于 o2web 的 MWF.ajax，但更现代
 */
export interface ApiRequestOptions {
  params?: Record<string, string>
  requireAuth?: boolean
  headers?: Record<string, string>
  /** 请求超时（毫秒）。普通请求默认 30s；`stream()` 里表示「空闲超时」（默认 60s，收到数据即重置）。 */
  timeoutMs?: number
  /** 成功时不读取响应体，用于可能包含遗留 token 字段的认证端点。 */
  discardResponse?: boolean
}

export class ApiClient {
  private base: string
  private refreshPromise: Promise<void> | null = null
  private authenticationFailureHandler: (() => void) | null = null

  constructor(base = '') {
    this.base = base
  }

  setAuthenticationFailureHandler(handler: (() => void) | null): void {
    this.authenticationFailureHandler = handler
  }

  private resolveUrl(path: string): URL {
    if (/^https?:\/\//i.test(path)) return new URL(path)

    const base = this.base.replace(/\/+$/, '')
    let endpoint = path.startsWith('/') ? path : `/${path}`
    if (base.endsWith('/api') && endpoint.startsWith('/api/')) {
      endpoint = endpoint.slice('/api'.length)
    }

    return new URL(`${base}${endpoint}`, window.location.origin)
  }

  private isRefreshRequest(path: string): boolean {
    return this.resolveUrl(path).pathname === '/api/authentication/refresh'
  }

  async refreshSession(): Promise<void> {
    if (!this.refreshPromise) {
      this.refreshPromise = this.request<never>('POST', '/api/authentication/refresh', {
        body: null,
        requireAuth: false,
        discardResponse: true,
      })
        .then(() => undefined)
        .finally(() => {
          this.refreshPromise = null
        })
    }
    return this.refreshPromise
  }

  /**
   * 认证失败的唯一出口。
   *
   * `refreshSession()` 内部走的是 `request()`，refresh 本身 401 时已经调用过
   * 本方法并抛出了 `AuthenticationError`。外层 catch 若再调一次，同一次失败
   * 会通知上层两次（handler 弹 toast 就会弹两条）。传入 `cause` 后，已经通知
   * 过的错误原样透传，只有「refresh 因非认证原因失败」才补一次通知。
   */
  private authenticationFailed(cause?: unknown): AuthenticationError {
    if (cause instanceof AuthenticationError) return cause
    this.authenticationFailureHandler?.()
    return new AuthenticationError('Session expired, please login again')
  }

  private async request<T>(
    method: string,
    path: string,
    options?: ApiRequestOptions & { body?: unknown; retried?: boolean },
  ): Promise<ApiResponse<T>> {
    const url = this.resolveUrl(path)
    if (options?.params) {
      for (const [k, v] of Object.entries(options.params)) {
        url.searchParams.set(k, v)
      }
    }

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...options?.headers,
    }

    const init: RequestInit = {
      method,
      headers,
      credentials: 'include',
    }

    if (options?.body !== null && options?.body !== undefined && method !== 'GET') {
      init.body = JSON.stringify(options.body)
    }

    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), options?.timeoutMs ?? DEFAULT_TIMEOUT_MS)

    let resp: Response
    try {
      resp = await fetch(url.toString(), { ...init, signal: controller.signal })
    } catch (err) {
      // 超时映射成 status 0：上层（vue-query 重试策略）据此识别「不会自己好」的失败并停止重试。
      if ((err as { name?: string } | null)?.name === 'AbortError') throw new ApiError('请求超时', 0)
      throw err
    } finally {
      clearTimeout(timeoutId)
    }

    if (!resp.ok) {
      if (resp.status === 401) {
        if (options?.requireAuth !== false && !options?.retried && !this.isRefreshRequest(path)) {
          try {
            await this.refreshSession()
          } catch (err) {
            throw this.authenticationFailed(err)
          }
          return this.request<T>(method, path, { ...options, retried: true })
        }
        throw this.authenticationFailed()
      }
      if (resp.status === 403) {
        throw new PermissionError(FORBIDDEN_MESSAGE)
      }
      throw httpStatusError(resp.status, resp.statusText)
    }

    if (options?.discardResponse || resp.status === 204) {
      return { success: true, data: undefined as T }
    }
    return resp.json() as Promise<ApiResponse<T>>
  }

  get<T>(path: string, options?: ApiRequestOptions): Promise<ApiResponse<T>> {
    return this.request<T>('GET', path, options)
  }

  post<T>(path: string, body?: unknown, options?: ApiRequestOptions): Promise<ApiResponse<T>> {
    return this.request<T>('POST', path, { ...options, body })
  }

  put<T>(path: string, body?: unknown, options?: ApiRequestOptions): Promise<ApiResponse<T>> {
    return this.request<T>('PUT', path, { ...options, body })
  }

  delete<T>(path: string, options?: ApiRequestOptions): Promise<ApiResponse<T>> {
    return this.request<T>('DELETE', path, options)
  }

  /** 文件上传（multipart/form-data） */
  async upload<T>(
    path: string,
    formData: FormData,
    options?: ApiRequestOptions & { timeoutMs?: number; method?: 'POST' | 'PUT' },
    retried = false,
  ): Promise<ApiResponse<T>> {
    const url = this.resolveUrl(path)
    const controller = new AbortController()
    if (options?.timeoutMs) {
      setTimeout(() => controller.abort(), options.timeoutMs)
    }
    const resp = await fetch(url.toString(), {
      method: options?.method ?? 'POST',
      headers: {
        ...options?.headers,
      },
      body: formData,
      credentials: 'include',
      signal: controller.signal,
    })
    if (!resp.ok) {
      if (resp.status === 401 && options?.requireAuth !== false && !retried) {
        try {
          await this.refreshSession()
        } catch (err) {
          throw this.authenticationFailed(err)
        }
        return this.upload<T>(path, formData, options, true)
      }
      if (resp.status === 401) throw this.authenticationFailed()
      if (resp.status === 403) throw new PermissionError(FORBIDDEN_MESSAGE)
      throw httpStatusError(resp.status)
    }
    return resp.json() as Promise<ApiResponse<T>>
  }

  /** 文件上传（带真实进度回调；fetch 拿不到上传字节流，走 XHR）。 */
  uploadWithProgress<T>(
    path: string,
    formData: FormData,
    onProgress?: (percent: number) => void,
    options?: ApiRequestOptions & { timeoutMs?: number },
  ): Promise<ApiResponse<T>> {
    return this.uploadWithProgressOnce<T>(path, formData, onProgress, options, false)
  }

  private uploadWithProgressOnce<T>(
    path: string,
    formData: FormData,
    onProgress: ((percent: number) => void) | undefined,
    options: (ApiRequestOptions & { timeoutMs?: number }) | undefined,
    retried: boolean,
  ): Promise<ApiResponse<T>> {
    return new Promise((resolve, reject) => {
      const url = this.resolveUrl(path)
      const xhr = new XMLHttpRequest()
      xhr.open('POST', url.toString())
      xhr.withCredentials = true
      for (const [k, v] of Object.entries(options?.headers ?? {})) {
        xhr.setRequestHeader(k, String(v))
      }
      if (options?.timeoutMs) xhr.timeout = options.timeoutMs
      xhr.upload.onprogress = (e) => {
        if (onProgress && e.lengthComputable) {
          onProgress(Math.round((e.loaded / e.total) * 100))
        }
      }
      xhr.onload = () => {
        if (xhr.status === 401 && options?.requireAuth !== false && !retried) {
          this.refreshSession()
            .then(() => this.uploadWithProgressOnce<T>(path, formData, onProgress, options, true).then(resolve, reject))
            .catch((err) => reject(this.authenticationFailed(err)))
          return
        }
        if (xhr.status === 401) {
          reject(this.authenticationFailed())
          return
        }
        if (xhr.status === 403) {
          reject(new PermissionError(FORBIDDEN_MESSAGE))
          return
        }
        if (xhr.status < 200 || xhr.status >= 300) {
          reject(httpStatusError(xhr.status))
          return
        }
        try {
          resolve(JSON.parse(xhr.responseText) as ApiResponse<T>)
        } catch {
          reject(new ApiError('malformed JSON response', xhr.status))
        }
      }
      xhr.onerror = () => reject(new ApiError('Network error', 0))
      xhr.ontimeout = () => reject(new ApiError('Timeout', 0))
      xhr.send(formData)
    })
  }

  /**
   * 原始字节流上传（application/octet-stream；o2 文件 API 的 octet 形态，
   * 文件名经 ?fileName= 查询参数传递）。
   */
  async uploadBytes<T>(
    path: string,
    bytes: Blob | ArrayBuffer | Uint8Array,
    options?: ApiRequestOptions & { timeoutMs?: number; fileName?: string },
    retried = false,
  ): Promise<ApiResponse<T>> {
    const url = this.resolveUrl(path)
    if (options?.fileName) url.searchParams.set('fileName', options.fileName)
    const controller = new AbortController()
    if (options?.timeoutMs) {
      setTimeout(() => controller.abort(), options.timeoutMs)
    }
    const resp = await fetch(url.toString(), {
      method: 'POST',
      headers: { 'Content-Type': 'application/octet-stream', ...options?.headers },
      body: bytes as Blob,
      credentials: 'include',
      signal: controller.signal,
    })
    if (!resp.ok) {
      if (resp.status === 401 && options?.requireAuth !== false && !retried) {
        try {
          await this.refreshSession()
        } catch (err) {
          throw this.authenticationFailed(err)
        }
        return this.uploadBytes<T>(path, bytes, options, true)
      }
      if (resp.status === 401) throw this.authenticationFailed()
      if (resp.status === 403) throw new PermissionError(FORBIDDEN_MESSAGE)
      throw httpStatusError(resp.status)
    }
    return resp.json() as Promise<ApiResponse<T>>
  }

  /**
   * SSE 流式 POST（POST + text/event-stream 响应）。
   * 每收到一个完整 SSE 事件回调 onEvent(event, data)；流结束 resolve。
   * 401 时的会话刷新语义与 request() 一致。
   *
   * R134：空闲超时。`options.timeoutMs` 在这里表示「多久没有任何字节就判定连接已死」
   * （默认 60s，收到分块即重置），超时以 `ApiError('流式响应超时', 0)` 结束，
   * 由调用方展示明确文案，而不是让 `await stream()` 永远不返回。
   */
  async stream(
    path: string,
    body: unknown,
    onEvent: (event: string, data: string) => void,
    options?: ApiRequestOptions,
    retried = false,
  ): Promise<void> {
    const url = this.resolveUrl(path)
    const idleMs = options?.timeoutMs ?? DEFAULT_STREAM_IDLE_TIMEOUT_MS
    const controller = new AbortController()
    let idleTimer: ReturnType<typeof setTimeout> | null = null
    let idleTimedOut = false
    const clearIdle = () => {
      if (idleTimer) {
        clearTimeout(idleTimer)
        idleTimer = null
      }
    }
    const armIdle = () => {
      clearIdle()
      idleTimer = setTimeout(() => {
        idleTimedOut = true
        controller.abort()
      }, idleMs)
    }

    try {
      armIdle()
      const resp = await fetch(url.toString(), {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...options?.headers },
        credentials: 'include',
        body: JSON.stringify(body ?? {}),
        signal: controller.signal,
      })
      if (!resp.ok) {
        if (resp.status === 401 && options?.requireAuth !== false && !retried) {
          try {
            await this.refreshSession()
          } catch (err) {
            throw this.authenticationFailed(err)
          }
          return this.stream(path, body, onEvent, options, true)
        }
        if (resp.status === 401) throw this.authenticationFailed()
        if (resp.status === 403) throw new PermissionError(FORBIDDEN_MESSAGE)
        throw httpStatusError(resp.status, resp.statusText)
      }
      if (!resp.body) throw new ApiError('no response body stream', resp.status)
      const reader = resp.body.getReader()
      const decoder = new TextDecoder()
      let buf = ''
      for (;;) {
        const { done, value } = await reader.read()
        if (done) break
        armIdle()
        buf += decoder.decode(value, { stream: true })
        // SSE 事件以空行分隔（容忍 \r\n）
        for (;;) {
          const m = buf.match(/(?:\r?\n){2}/)
          if (!m || m.index === undefined) break
          const raw = buf.slice(0, m.index)
          buf = buf.slice(m.index + m[0].length)
          let event = 'message'
          const dataLines: string[] = []
          for (const line of raw.split(/\r?\n/)) {
            if (line.startsWith('event:')) event = line.slice(6).trim()
            else if (line.startsWith('data:')) dataLines.push(line.slice(5).trimStart())
          }
          if (dataLines.length) onEvent(event, dataLines.join('\n'))
        }
      }
    } catch (err) {
      // 只有「我们自己的空闲看门狗」超时才改写错误；其余（401/403/网络/解析）原样抛。
      if (idleTimedOut) throw new ApiError('流式响应超时', 0)
      throw err
    } finally {
      clearIdle()
    }
  }
}

export class ApiError extends Error {
  constructor(
    public message: string,
    public status: number,
  ) {
    super(message)
    this.name = 'ApiError'
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

export const api = new ApiClient()
