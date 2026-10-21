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
 * 会话内缓存的「令牌已验过真」结论。
 *
 * ⚠️ 为什么必须缓存，以及为什么它只在**内存里**：
 *
 * 路由守卫每次导航都会问一次「令牌还有效吗」。若每次都打
 * `/admin/me`，那么每切换一个页面就是一次额外请求 ——
 * 而后台的 11 个视图切换是运营最频繁的操作。
 *
 * 但不能把它持久化到 localStorage：那等于把「已验过」写进磁盘，
 * 一次验证永久有效，令牌过期后缓存反而成了绕过校验的后门。
 * 所以存内存，刷新页面重新验 —— 刷新本来就会重新走一遍守卫，
 * 那一次验真是必要成本。
 */
let tokenVerified = false

/** 令牌被换过（登录 / 登出 / 401 清理）时，验真结论必须作废。 */
export function invalidateTokenVerification(): void {
  tokenVerified = false
}

/**
 * 验一次令牌是否真的有效（而不是「localStorage 里有字符串」）。
 *
 * 语义：
 *   - 没令牌            → false（不必打请求）
 *   - 已验过真          → true（内存缓存）
 *   - 请求成功          → true 并缓存
 *   - 请求失败          → false（401 的清理由 request 内部完成）
 *
 * ⚠️ 这里的 catch 是**宽**的：连网络错误（后端没起来、断网）也会被
 * 当成「验真失败」。这是有意的 —— 那种情况下放行进入后台壳，
 * 运营看到的是一整片加载失败/401 的空壳；跳登录页反而能明确说出
 * 「现在连不上后端」。代价是「后端重启期间运营会被弹回登录页」，
 * 重新登录即可恢复，比在一个全是错误的壳里找原因便宜得多。
 */
export async function confirmTokenValid(): Promise<boolean> {
  if (!getToken()) return false
  if (tokenVerified) return true
  try {
    // 直接走本模块的 request（函数声明提升，定义在下方也可用）：
    // 复用它的 401 清理逻辑（handleUnauthorized 会 setToken('') + 跳登录页）。
    // 这里刻意不 import api/index —— client 与 index 互相 import 会成环。
    await request<{ admin: { id: number } }>('GET', '/admin/me')
    tokenVerified = true
    return true
  } catch {
    invalidateTokenVerification()
    return false
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
  // 令牌失效 = 验真结论作废。否则下次导航会拿着缓存里的 true 放行，
  // 而那时 localStorage 里已经没令牌了 —— 守卫直接进入后台壳。
  invalidateTokenVerification()
  // 直接改 location.hash 而不是走 router 导航：
  // 这里可能正处在一次请求的回调里，router 的导航是异步的；
  // 直接改 hash 立即生效，不依赖任何未决的导航。
  if (typeof window !== 'undefined' && !window.location.hash.startsWith('#/admin/login')) {
    window.location.hash = '#/admin/login'
  }
}

/**
 * `soft` 的语义：这次请求的 401 是**预期答案**，不是「会话过期」。
 *
 * 2026-10-21（P2-5）：玩家向的文档站（`/wiki`）会拉一个**管理员**端点
 * （`/admin/dashboard`）来做「实战分布」。匿名访客拿到 401 →
 * `handleUnauthorized()` 把 `location.hash` 改成 `#/admin/login`
 * —— 一个公开页面把访客踢去了运营后台。
 *
 * `soft: true` 时：401 **不**跳转、**不**清令牌（那是「有效登录被误清」，
 * 比不跳转更糟），但仍然抛错，让调用方自己决定怎么显示。
 */
async function request<T>(
  method: string,
  path: string,
  body?: unknown,
  opts: { auth?: boolean; soft?: boolean } = {},
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
  //
  // 第 154 轮补上 `opts.auth !== false`：公开请求（`{ auth: false }`，
  // 见 WikiView 对 `/config` 的调用）的 401 与「我的令牌失效」无关，
  // 却会把运营**有效**的令牌清掉并踢去登录页 —— 一个不该登出的地方
  // 被登出了，而运营只会看到「莫名其妙要重新登录」。
  // 判据收紧为「这次请求确实带了令牌 + 不是登录接口本身」。
  if (
    resp.status === 401 &&
    opts.auth !== false &&
    !opts.soft &&
    !path.endsWith('/admin/login')
  ) {
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
  get: <T>(path: string, opts?: { auth?: boolean; soft?: boolean }) =>
    request<T>('GET', path, undefined, opts),
  post: <T>(path: string, body?: unknown, opts?: { auth?: boolean; soft?: boolean }) =>
    request<T>('POST', path, body, opts),
  put: <T>(path: string, body?: unknown, opts?: { auth?: boolean; soft?: boolean }) =>
    request<T>('PUT', path, body, opts),
  del: <T>(path: string, opts?: { auth?: boolean; soft?: boolean }) =>
    request<T>('DELETE', path, undefined, opts),
}
