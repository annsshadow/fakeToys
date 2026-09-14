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
  /** 请求超时（毫秒）。默认无超时，由 fetch 运行时决定。 */
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
    if (base.endsWith('/jaxrs') && endpoint.startsWith('/jaxrs/')) {
      endpoint = endpoint.slice('/jaxrs'.length)
    }

    return new URL(`${base}${endpoint}`, window.location.origin)
  }

  private isRefreshRequest(path: string): boolean {
    return this.resolveUrl(path).pathname === '/jaxrs/authentication/refresh'
  }

  async refreshSession(): Promise<void> {
    if (!this.refreshPromise) {
      this.refreshPromise = this.request<never>('POST', '/jaxrs/authentication/refresh', {
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

  private authenticationFailed(): AuthenticationError {
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
    if (options?.timeoutMs) {
      setTimeout(() => controller.abort(), options.timeoutMs)
    }

    const resp = await fetch(url.toString(), { ...init, signal: controller.signal })

    if (!resp.ok) {
      if (resp.status === 401) {
        if (options?.requireAuth !== false && !options?.retried && !this.isRefreshRequest(path)) {
          try {
            await this.refreshSession()
          } catch {
            throw this.authenticationFailed()
          }
          return this.request<T>(method, path, { ...options, retried: true })
        }
        throw this.authenticationFailed()
      }
      if (resp.status === 403) {
        throw new PermissionError('Permission denied')
      }
      throw new ApiError(`HTTP ${resp.status}: ${resp.statusText}`, resp.status)
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
    options?: ApiRequestOptions & { timeoutMs?: number },
    retried = false,
  ): Promise<ApiResponse<T>> {
    const url = this.resolveUrl(path)
    const controller = new AbortController()
    if (options?.timeoutMs) {
      setTimeout(() => controller.abort(), options.timeoutMs)
    }
    const resp = await fetch(url.toString(), {
      method: 'POST',
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
        } catch {
          throw this.authenticationFailed()
        }
        return this.upload<T>(path, formData, options, true)
      }
      if (resp.status === 401) throw this.authenticationFailed()
      if (resp.status === 403) throw new PermissionError('Permission denied')
      throw new ApiError(`HTTP ${resp.status}`, resp.status)
    }
    return resp.json() as Promise<ApiResponse<T>>
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
