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

/**
 * BUG-B：认证失败（401）有两类完全不同的语义，不能共用一条文案：
 *   - 登录接口的 401 = 凭据校验失败（用户名/密码错误或账户锁定）。用户仍在登录页，
 *     没有会话可「过期」，提示必须以凭据错误为导向，否则会误导用户去刷新会话。
 *   - 其它业务请求的 401 = 会话/令牌失效，需要重新登录。
 * 后端 POST /api/authentication/login 对「用户不存在」直接返回 401（AppError::Unauthorized），
 * 且登录请求 requireAuth:false 不会触发 refresh，因此这条 401 必然是凭据错误。
 */
const LOGIN_FAILED_MESSAGE = '用户名或密码错误，请重新输入'
const SESSION_EXPIRED_MESSAGE = 'Session expired, please login again'

/**
 * R155 / B1：传输层网络失败文案归一化。
 * fetch/XHR 的底层错误是英文技术串（Failed to fetch / Network error / Timeout /
 * Request aborted），直接透到页面会变成「加载失败：Failed to fetch」。
 * 与移动 services/http.ts（R136）同口径，在这里换成中文可读文案；
 * status 保持 0，使 shouldRetryQuery 的不重试判定不变。
 */
const NETWORK_FAILURE_MESSAGE = '网络连接失败，请检查网络后重试'
const UPLOAD_FAILURE_MESSAGE = '上传失败，请检查网络后重试'
const UPLOAD_TIMEOUT_MESSAGE = '上传超时，请检查网络后重试'

/** fetch/XHR 网络失败 → 归一化的 ApiError(status 0)。 */
function networkFailure(message: string): ApiError {
  return new ApiError(message, 0)
}

function httpStatusError(status: number, statusText?: string): ApiError {
  if (status === 404) return new ApiError(NOT_FOUND_MESSAGE, 404)
  return new ApiError(statusText ? `HTTP ${status}: ${statusText}` : `HTTP ${status}`, status)
}

/**
 * R282：2xx 响应体非法 JSON 的错误信封归一化。
 * 网关误插 HTML 错误页 / 代理截断 / 空 body 边缘都会让 2xx body 不是合法 JSON；
 * 原生 SyntaxError 会绕过所有 ApiError 判别分支，并把英文技术串突到页面/toast。
 * 与 XHR 通道 uploadWithProgress 的 `new ApiError('malformed JSON response', status)` 同口径。
 */
async function parseJsonEnvelope<T>(resp: Response): Promise<ApiResponse<T>> {
  try {
    return (await resp.json()) as ApiResponse<T>
  } catch {
    throw new ApiError('malformed JSON response', resp.status)
  }
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
  /**
   * 调用方取消句柄（R143）。`abort()` 后请求/流会立刻断开，并以
   * `RequestCancelledError` 结束——这是「用户不再需要这个结果」，不是失败，
   * UI 不应提示错误。与内部超时（`ApiError('请求超时', 0)`）语义分开。
   */
  signal?: AbortSignal
}

interface AbortBridge {
  cancelled(): boolean
  dispose(): void
}

/**
 * 把调用方的 signal 桥接到内部 controller：
 * 内部超时只 abort 内部 controller；外部取消会置 `cancelled` 标志，
 * 让 catch 能区分「用户取消」与「超时」。
 */
function bridgeAbort(signal: AbortSignal | undefined, controller: AbortController): AbortBridge {
  if (!signal) return { cancelled: () => false, dispose: () => {} }
  let cancelled = false
  const onAbort = () => {
    cancelled = true
    controller.abort()
  }
  if (signal.aborted) {
    onAbort()
    return { cancelled: () => true, dispose: () => {} }
  }
  signal.addEventListener('abort', onAbort, { once: true })
  return {
    cancelled: () => cancelled,
    dispose: () => signal.removeEventListener('abort', onAbort),
  }
}

export class ApiClient {
  private base: string
  private refreshPromise: Promise<void> | null = null
  private authenticationFailureHandler: (() => void) | null = null

  constructor(base = '') {
    this.base = base
  }

  /** 子路径部署时设置 API 前缀（如 '/oa4rust-api'），其后所有请求自动带上该前缀。 */
  setBase(base: string): void {
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
    return path === '/api/authentication/refresh'
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
  private authenticationFailed(cause?: unknown, path?: string): AuthenticationError {
    if (cause instanceof AuthenticationError) return cause
    this.authenticationFailureHandler?.()
    // 登录接口的 401 是凭据错误，不是会话过期（用户还没登录，无所谓会话失效）。
    const message = path && this.isLoginRequest(path) ? LOGIN_FAILED_MESSAGE : SESSION_EXPIRED_MESSAGE
    return new AuthenticationError(message)
  }

  /** 登录/认证类端点：这些接口的 401 表示凭据无效，而非已登录会话过期。 */
  private isLoginRequest(path: string): boolean {
    return (
      path === '/api/authentication/login' ||
      path === '/api/authentication' ||
      path === '/api/authentication/captcha' ||
      path.startsWith('/api/authentication/two')
    )
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
    const bridge = bridgeAbort(options?.signal, controller)
    const timeoutId = setTimeout(() => controller.abort(), options?.timeoutMs ?? DEFAULT_TIMEOUT_MS)

    let resp: Response
    try {
      resp = await fetch(url.toString(), { ...init, signal: controller.signal })
    } catch (err) {
      // 用户取消优先判定：此时内部 controller 也被桥接 abort，不能再报「超时」。
      if (bridge.cancelled()) throw new RequestCancelledError()
      // 超时映射成 status 0：上层（vue-query 重试策略）据此识别「不会自己好」的失败并停止重试。
      if ((err as { name?: string } | null)?.name === 'AbortError') throw new ApiError('请求超时', 0)
      throw networkFailure(NETWORK_FAILURE_MESSAGE)
    } finally {
      clearTimeout(timeoutId)
      bridge.dispose()
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
        throw this.authenticationFailed(undefined, path)
      }
      if (resp.status === 403) {
        throw new PermissionError(FORBIDDEN_MESSAGE)
      }
      throw httpStatusError(resp.status, resp.statusText)
    }

    if (options?.discardResponse || resp.status === 204) {
      return { success: true, data: undefined as T }
    }
    return parseJsonEnvelope<T>(resp)
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
    const bridge = bridgeAbort(options?.signal, controller)
    let timeoutId: ReturnType<typeof setTimeout> | null = null
    let timeoutElapsed = false
    if (options?.timeoutMs) {
      timeoutId = setTimeout(() => {
        timeoutElapsed = true
        controller.abort()
      }, options.timeoutMs)
    }
    let resp: Response
    try {
      resp = await fetch(url.toString(), {
        method: options?.method ?? 'POST',
        headers: {
          ...options?.headers,
        },
        body: formData,
        credentials: 'include',
        signal: controller.signal,
      })
    } catch {
      if (bridge.cancelled()) throw new RequestCancelledError()
      if (timeoutElapsed) throw networkFailure(UPLOAD_TIMEOUT_MESSAGE)
      throw networkFailure(UPLOAD_FAILURE_MESSAGE)
    } finally {
      if (timeoutId !== null) clearTimeout(timeoutId)
      bridge.dispose()
    }
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
    return parseJsonEnvelope<T>(resp)
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
      let cancelled = false
      const onAbort = () => {
        cancelled = true
        xhr.abort()
      }
      const settleResolve = (value: ApiResponse<T>) => {
        options?.signal?.removeEventListener('abort', onAbort)
        resolve(value)
      }
      const settleReject = (error: unknown) => {
        options?.signal?.removeEventListener('abort', onAbort)
        reject(error)
      }
      if (options?.signal) {
        if (options.signal.aborted) {
          settleReject(new RequestCancelledError())
          return
        }
        options.signal.addEventListener('abort', onAbort, { once: true })
      }
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
            .then(() =>
              this.uploadWithProgressOnce<T>(path, formData, onProgress, options, true).then(
                settleResolve,
                settleReject,
              ),
            )
            .catch((err) => settleReject(this.authenticationFailed(err)))
          return
        }
        if (xhr.status === 401) {
          settleReject(this.authenticationFailed())
          return
        }
        if (xhr.status === 403) {
          settleReject(new PermissionError(FORBIDDEN_MESSAGE))
          return
        }
        if (xhr.status < 200 || xhr.status >= 300) {
          settleReject(httpStatusError(xhr.status))
          return
        }
        try {
          settleResolve(JSON.parse(xhr.responseText) as ApiResponse<T>)
        } catch {
          settleReject(new ApiError('malformed JSON response', xhr.status))
        }
      }
      xhr.onerror = () => settleReject(networkFailure(UPLOAD_FAILURE_MESSAGE))
      xhr.onabort = () => settleReject(cancelled ? new RequestCancelledError() : networkFailure(UPLOAD_FAILURE_MESSAGE))
      xhr.ontimeout = () => settleReject(networkFailure(UPLOAD_TIMEOUT_MESSAGE))
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
    const bridge = bridgeAbort(options?.signal, controller)
    let timeoutId: ReturnType<typeof setTimeout> | null = null
    let timeoutElapsed = false
    if (options?.timeoutMs) {
      timeoutId = setTimeout(() => {
        timeoutElapsed = true
        controller.abort()
      }, options.timeoutMs)
    }
    let resp: Response
    try {
      resp = await fetch(url.toString(), {
        method: 'POST',
        headers: { 'Content-Type': 'application/octet-stream', ...options?.headers },
        body: bytes as Blob,
        credentials: 'include',
        signal: controller.signal,
      })
    } catch {
      if (bridge.cancelled()) throw new RequestCancelledError()
      if (timeoutElapsed) throw networkFailure(UPLOAD_TIMEOUT_MESSAGE)
      throw networkFailure(UPLOAD_FAILURE_MESSAGE)
    } finally {
      if (timeoutId !== null) clearTimeout(timeoutId)
      bridge.dispose()
    }
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
    return parseJsonEnvelope<T>(resp)
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
    const bridge = bridgeAbort(options?.signal, controller)
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
      const emitEvent = (raw: string): void => {
        let event = 'message'
        const dataLines: string[] = []
        for (const line of raw.split(/\r?\n/)) {
          if (line.startsWith('event:')) event = line.slice(6).trim()
          else if (line.startsWith('data:')) dataLines.push(line.slice(5).trimStart())
        }
        if (dataLines.length) onEvent(event, dataLines.join('\n'))
      }
      // 解析所有以空行分隔的完整事件，返回未闭合的尾部缓冲。
      const drainEvents = (): string => {
        for (;;) {
          const m = buf.match(/(?:\r?\n){2}/)
          if (!m || m.index === undefined) return buf
          emitEvent(buf.slice(0, m.index))
          buf = buf.slice(m.index + m[0].length)
        }
      }
      for (;;) {
        const { done, value } = await reader.read()
        if (done) break
        armIdle()
        buf += decoder.decode(value, { stream: true })
        // SSE 事件以空行分隔（容忍 \r\n）
        drainEvents()
      }
      // 流结束时缓冲区可能还留着最后一条事件（服务端未补终止空行），
      // 且解码器内部可能还有未输出的多字节字符；flush 后一并处理，避免
      // 丢失结束事件 / 最后一段 token。
      buf = decoder.decode() + drainEvents()
      if (buf.trim()) emitEvent(buf)
    } catch (err) {
      // 用户取消优先：切换会话/离开页面时应静默结束，不报「流式响应超时」。
      if (bridge.cancelled()) throw new RequestCancelledError()
      // 只有「我们自己的空闲看门狗」超时才改写错误；其余（401/403/网络/解析）原样抛。
      if (idleTimedOut) throw new ApiError('流式响应超时', 0)
      // stream 的 try 块包住了状态判定：认证/权限/HTTP 状态/无 body 等语义错误
      // 都已是 ApiError（含子类），必须原样透传；这里只归一化传输层原始网络异常。
      if (err instanceof ApiError) throw err
      throw networkFailure(NETWORK_FAILURE_MESSAGE)
    } finally {
      clearIdle()
      bridge.dispose()
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

/**
 * 调用方主动取消（R143）：切换会话、离开页面等。不是失败，UI 应静默返回，
 * 不要弹「请求失败」。`status` 沿用 0 与 `ApiError` 契约一致。
 */
export class RequestCancelledError extends ApiError {
  constructor(message = '请求已取消') {
    super(message, 0)
    this.name = 'RequestCancelledError'
  }
}

/** 判断是否为调用方取消；UI 用它决定「静默返回」而不是「展示错误」。 */
export function isRequestCancelled(error: unknown): boolean {
  if (error instanceof RequestCancelledError) return true
  return (error as { name?: string } | null | undefined)?.name === 'RequestCancelledError'
}

export const api = new ApiClient()
