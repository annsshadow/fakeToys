import { api, type ApiResponse } from './api.js';
import type { O2User } from './types.js';

/**
 * Vue Router 封装
 * 替代 o2web 的 MWF.xDesktop.open() / layout.desktop.navigate()
 */

// 延迟导入，避免循环依赖
let _router: ReturnType<typeof import('vue-router').createRouter> | null = null;

export function useRouter() {
  if (!_router) {
    throw new Error('useRouter() called before Vue Router is installed. Call createO2App() first.');
  }
  return _router;
}

export function setRouter(router: ReturnType<typeof import('vue-router').createRouter>) {
  _router = router;
}

/** 导航到指定应用窗口 */
export async function openApp(
  appId: string,
  params?: Record<string, unknown>,
  options?: { newWindow?: boolean; replace?: boolean },
): Promise<void> {
  const query = params ? '?' + new URLSearchParams(params as Record<string, string>).toString() : '';
  const path = `/app/${appId}${query}`;

  if (options?.newWindow) {
    window.open(path, '_blank');
    return;
  }

  if (options?.replace) {
    useRouter().replace(path);
  } else {
    useRouter().push(path);
  }
}

/** 获取当前应用参数 */
export function getAppParams<T = Record<string, string>>(): T {
  const params = useRouter().currentRoute.value.query;
  return params as unknown as T;
}

/** 登录相关路由导航 */
export const authRoutes = {
  login: '/login',
  callback: '/oauth/callback/:platform',
  error: '/oauth/error',
  sso: '/sso',
};

/** 检查是否需要登录 */
export async function checkAuth(): Promise<boolean> {
  try {
    const resp = await api.get<{ data: O2User }>('/jaxrs/authentication/who');
    return !!resp.data;
  } catch {
    return false;
  }
}
