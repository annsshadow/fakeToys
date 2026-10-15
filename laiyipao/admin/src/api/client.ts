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

/**
 * 401 的集中处理：清令牌 + 送回登录页。
 *
 * ⚠️ 这里曾经**根本不存在**，而 `router/index.ts` 的守卫注释写着
 * 「令牌失效兜底：api 层抛 401 时把令牌清掉并送回登录页……
 * 真正的 401 检测在 api/client 抛出」。
 *
 * 那条注释描述的检测从来没被实现过 —— `client.ts` 的 `!resp.ok` 分支
 * 无条件抛 ApiError 就结束了。于是：
 *
 *   - 令牌过期（后台 access token 默认 2h）或管理员被停用
 *   - 路由守卫看到 `getToken()` 仍为真 → 放行
 *   - 每个视图 catch 到 401 → 弹一句 `unauthorized`
 *   - 运营被永久卡在「已登录的壳」里反复看到报错，
 *     **永远不会被提示重新认证**，localStorage 里还长期留着一枚失效凭证
 *
 * 与后端同族的缺陷形状：**注释是最容易让所有人以为已经做了的形态**。
 * `tests/router.test.ts` 的注释也建立在同一个虚构前提上。
 *
 * 为什么必须**清 localStorage**而不只是跳登录页：
 * 失效令牌是别人设备上的凭证，写在磁盘上直到被显式删除。
 * 只跳页不删，等于把这枚凭证留在原地。
 */
function handleUnauthorized(): void {
  setToken('')
  // 直接改 location.hash 而不是走 router 导航：
  // 这里可能正处在一次请求的回调里，router 的导航是异步的；
  // 直接改 hash 立即生效，不依赖任何未决的导航。
  if (typeof window !== 'undefined' && !window.location.hash.startsWith('#/admin/login')) {
    window.location.hash = '#/admin/login'
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

  // ⚠️ 必须在读 body **之前**处理：一旦被后面的普通错误分支抛出去，
  // 就再也不会有代码回到这里。判据必须落在「抛错之前」。
  //
  // 登录接口自己的 401（用户名或密码错）**不能**触发跳转 ——
  // 那会把「密码打错」变成「被踢回登录页」，看不出是哪里错了。
  if (resp.status === 401 && !path.endsWith('/admin/login')) {
    handleUnauthorized()
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
