import type { O2User } from './types.js';

/** 统一 API 响应结构（与 oa4rust ActionResult<T> 对齐） */
export interface ApiResponse<T = unknown> {
  success: boolean;
  data: T;
  message?: string;
  code?: number;
}

/** 分页响应 */
export interface PagedResponse<T> {
  data: T[];
  total: number;
  page: number;
  size: number;
}

/** 查询选项（传给 TanStack Query） */
export interface QueryOptions<TData = unknown> {
  enabled?: boolean;
  staleTime?: number;
  cacheTime?: number;
  retry?: number | boolean;
  retryDelay?: number | ((attempt: number) => number);
  meta?: Record<string, unknown>;
}

/** mutation 选项 */
export interface MutationOptions<TData = unknown, TVariables = unknown> {
  onSuccess?: (data: TData, variables: TVariables, context: unknown) => void;
  onError?: (error: unknown, variables: TVariables, context: unknown) => void;
  onSettled?: (data: TData | undefined, error: unknown, variables: TVariables, context: unknown) => void;
}

/**
 * 带自动认证头的 fetch 封装
 * 等价于 o2web 的 MWF.ajax，但更现代
 */
class ApiClient {
  private base: string;

  constructor(base: string = '/jaxrs') {
    this.base = base;
  }

  private getAuthHeader(): Record<string, string> {
    const token = localStorage.getItem('oa4rust_session');
    if (token) {
      try {
        const { token: t } = JSON.parse(token) as { token: string };
        return { 'Authorization': `Bearer ${t}` };
      } catch {
        // ignore
      }
    }
    return {};
  }

  private async request<T>(
    method: string,
    path: string,
    options?: {
      body?: unknown;
      params?: Record<string, string>;
      requireAuth?: boolean;
      headers?: Record<string, string>;
    },
  ): Promise<ApiResponse<T>> {
    const url = new URL(`${this.base}${path}`, window.location.origin);
    if (options?.params) {
      for (const [k, v] of Object.entries(options.params)) {
        url.searchParams.set(k, v);
      }
    }

    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...this.getAuthHeader(),
      ...options?.headers,
    };

    const init: RequestInit = {
      method,
      headers,
      credentials: 'include',
    };

    if (options?.body !== null && options?.body !== undefined && method !== 'GET') {
      init.body = JSON.stringify(options.body);
    }

    const resp = await fetch(url.toString(), init);

    if (!resp.ok) {
      if (resp.status === 401) {
        // 清除无效 session
        localStorage.removeItem('oa4rust_session');
        throw new AuthenticationError('Session expired, please login again');
      }
      if (resp.status === 403) {
        throw new PermissionError('Permission denied');
      }
      throw new ApiError(`HTTP ${resp.status}: ${resp.statusText}`, resp.status);
    }

    return resp.json() as Promise<ApiResponse<T>>;
  }

  get<T>(path: string, options?: { params?: Record<string, string>; requireAuth?: boolean }): Promise<ApiResponse<T>> {
    return this.request<T>('GET', path, options);
  }

  post<T>(
    path: string,
    body?: unknown,
    options?: { params?: Record<string, string>; requireAuth?: boolean; headers?: Record<string, string> },
  ): Promise<ApiResponse<T>> {
    return this.request<T>('POST', path, { ...options, body });
  }

  put<T>(
    path: string,
    body?: unknown,
    options?: { params?: Record<string, string>; requireAuth?: boolean },
  ): Promise<ApiResponse<T>> {
    return this.request<T>('PUT', path, { ...options, body });
  }

  delete<T>(path: string, options?: { params?: Record<string, string>; requireAuth?: boolean }): Promise<ApiResponse<T>> {
    return this.request<T>('DELETE', path, options);
  }

  /** 文件上传（multipart/form-data） */
  async upload<T>(path: string, formData: FormData): Promise<ApiResponse<T>> {
    const url = new URL(`${this.base}${path}`, window.location.origin);
    const resp = await fetch(url.toString(), {
      method: 'POST',
      headers: this.getAuthHeader(),
      body: formData,
      credentials: 'include',
    });
    if (!resp.ok) throw new ApiError(`HTTP ${resp.status}`, resp.status);
    return resp.json() as Promise<ApiResponse<T>>;
  }
}

export class ApiError extends Error {
  constructor(public message: string, public status: number) {
    super(message);
    this.name = 'ApiError';
  }
}

export class AuthenticationError extends ApiError {
  constructor(message: string) {
    super(message, 401);
    this.name = 'AuthenticationError';
  }
}

export class PermissionError extends ApiError {
  constructor(message: string) {
    super(message, 403);
    this.name = 'PermissionError';
  }
}

export const api = new ApiClient();
