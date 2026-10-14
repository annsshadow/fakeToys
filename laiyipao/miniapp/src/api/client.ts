/**
 * 小程序端 API 客户端。
 *
 * 目标平台是微信小程序，但需要同时能在 H5 跑通（用于本机验证）。
 * uni.request 在两端 API 同构，因此直接用它，不引入额外 HTTP 库。
 */

/**
 * 解析后端地址。
 *
 * ⚠️ 它曾经是**写死的字面量** `'http://127.0.0.1:8080/api/v1'` ——
 * 也就是说打出来的包只能连用户自己设备的回环地址，生产环境根本不可用，
 * 而「唯一的修法」是回去改这个源码常量。
 *
 * 那条修改路径上没有任何东西阻止人把 `http://` 换成真实域名保留下来，
 * 于是最自然的修补就是「照抄，只把地址换掉」——
 * 而那会让 `access_token` / `refresh_token`（有效期默认 30 天）
 * **以明文过网**。微信小程序对非 HTTPS 域名有白名单限制，
 * 但 H5 端没有任何东西挡着。
 *
 * 所以这里做两件事，缺一不可：
 *  1. 从环境变量取地址（与 admin 的 `VITE_API_BASE` 同一套机制）；
 *  2. **非回环主机必须以 https:// 开头，否则启动即抛**。
 *
 * 判据为什么是「回环」而不是「localhost」字面量：
 * 回环地址（127.0.0.1 / localhost / ::1）在真机上指向手机自己，
 * 抓包也只能看到本机流量，明文无所谓。
 * 而**局域网 IP**（192.168.x.x / 10.x.x.x）虽然用于真机调试，
 * 它同样是明文 —— 但它是开发场景，且微信开发者工具本身就在
 * 「不校验合法域名」模式下运行，硬拦会挡住正常开发。
 * 因此对局域网 IP 只警告不抛，对**公网域名**才硬拦。
 */
function resolveBaseURL(): string {
  const configured =
    (typeof import.meta !== 'undefined' &&
      (import.meta as { env?: Record<string, string | undefined> }).env?.VITE_API_BASE) ||
    ''
  const raw = (configured || 'http://127.0.0.1:8080/api/v1').trim()

  const isLoopback = /^http:\/\/(127\.0\.0\.1|localhost|\[::1\])(:|\/|$)/i.test(raw)
  const isPrivateLAN = /^http:\/\/(192\.168\.|10\.|172\.(1[6-9]|2\d|3[01])\.)/i.test(raw)
  const isPlainHTTP = /^http:\/\//i.test(raw)

  if (isPlainHTTP && !isLoopback) {
    if (isPrivateLAN) {
      // 开发场景：真机连本机局域网。只警告。
      console.warn(
        '[api] 后端地址走明文 HTTP 的局域网地址：' +
          '真机调试可用，但令牌会明文过网，仅限开发环境。',
      )
    } else {
      // 公网域名 + 明文 = 长期有效的令牌裸奔。硬拦。
      throw new Error(
        `api: 生产后端地址必须是 HTTPS（当前 ${raw}）。` +
          '请设置 VITE_API_BASE=https://your.domain/api/v1。',
      )
    }
  }
  return raw.replace(/\/+$/, '')
}

const BASE_URL = resolveBaseURL()

/** GameConfig 与服务端 /api/v1/config 的响应结构对应。 */
export interface GameConfig {
  /**
   * 配置**内容哈希**（服务端 SHA-256 前 16 位十六进制）。
   *
   * 语义：只在这一包内容**真的变了**时才变。
   * 客户端据此判断本地持久化副本还能不能用。
   *
   * ⚠️ 它曾经是 `time.Now() % 100000`（每秒都变），于是「版本没变就不用刷新」
   * 这个判断从来没被实现过。现在两端都按它做真判断。
   */
  version: string
  levels: import('@/game/types').LevelSummary[]
  enemies: import('@/game/types').EnemyDef[]
  skills: import('@/game/types').SkillDef[]
  composite_skills: import('@/game/types').SkillDef[]
  // ⚠️ 以下三组的字段名必须与 Go 侧 json tag 逐字一致（snake_case）。
  // 曾经这里写成 PascalCase（Output/A/B/Key/Name/StartLevel...），
  // 而服务端一直下发 snake_case —— 声明与实际不符，且**不报任何错**，
  // 访问结果全是 undefined。chapter 那组已经被 stage.vue 消费：
  // 读 ch.Name 显示空白、读 ch.ID 让点击与高亮彻底失效。
  recipes: Array<{ output: number; a: number; b: number; out_tier: number }>
  equipment: Array<Record<string, unknown>>
  gems: Array<Record<string, unknown>>
  gem_qualities: Array<{ name: string; affix_count: number; mult: number }>
  skins: Array<Record<string, unknown>>
  mastery_families: import('@/game/types').MasteryFamily[]
  reactions: Array<{
    key: string
    name: string
    base_coef: number
    attack_weight_pct: number
    status_duration_ms: number
    aoe_radius: number
    dispel_shield: boolean
    amplify_pct: number
  }>
  chapters: Array<{
    id: number
    name: string
    start_level: number
    end_level: number
    terrain_kind: string
    boss_enemy_id: number
  }>
  rating_weights: {
    element_coverage: number
    reaction_coverage: number
    mastery_done: number
    equipment_synergy: number
    mechanic_depth: number
  }
  /**
   * score_rules 是分数规则，**必须由服务端下发**。
   *
   * ⚠️ 客户端不得自己写死 500/5000/100 或星级比例：
   * `star_targets` 是服务端用这份规则算出来的，
   * 客户端用自己的一份就可能与门槛算法漂移，
   * 而两端各自都"自洽"，没有任何行为测试会发现。
   * 转成 ScoreRules 用 `scoreRulesFromServer()`。
   */
  score_rules: {
    per_damage_unit: number
    on_kill_normal: number
    on_kill_boss: number
    star_target_ratio: number[]
    score_full_at_sec: number
  }
  /**
   * skill_rules 是**技能升级规则**（等级上限 / 每级伤害系数 / 费用基数）。
   *
   * 唯一定义在 Go 侧 `domain.DefaultSkillRules()`，本字段是它的下发副本。
   * 客户端的 `DEFAULT_SKILL_RULES` 只是离线兜底，
   * 由 `skill.test.ts` 的 TestSkillRulesMatchServerContract 断言不漂移。
   *
   * 转换见 SkillRules 的 `skillRulesFromServer()`。
   */
  skill_rules: {
    max_level: number
    coef_permille: number
    base_cost: number
  }
  server_time: string
}

/** 本地存储的键。微信小程序与 H5 都用 uni.getStorageSync 抽象。 */
const TOKEN_KEY = 'lyp_token'
const REFRESH_KEY = 'lyp_refresh'
/** access token 的过期时刻（RFC3339 原文）。第 150 轮新增。 */
const TOKEN_EXP_KEY = 'lyp_token_exp'
const GUEST_KEY = 'lyp_guest'

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

function get(key: string, def = ''): string {
  try {
    const v = uni.getStorageSync(key)
    return typeof v === 'string' ? v : def
  } catch {
    return def
  }
}

function set(key: string, value: string): void {
  try {
    uni.setStorageSync(key, value)
  } catch {
    /* 存储不可用时退化为内存态（本次会话有效） */
  }
}

function del(key: string): void {
  try {
    uni.removeStorageSync(key)
  } catch {
    /* 同上 */
  }
}

export function getToken(): string {
  return get(TOKEN_KEY)
}
export function getGuestToken(): string {
  return get(GUEST_KEY)
}
export function setGuestToken(t: string): void {
  set(GUEST_KEY, t)
}

/**
 * 写入登录后拿到的令牌对。
 *
 * 放在 api 层而不是 store 里，是为了避免 store 反向依赖 api 的内部状态 ——
 * 令牌存取是 api 层的事，store 只调 login() 并读结果。
 *
 * `expiresAt` 是 `TokenPair.expires_at`（RFC3339）。此前它下发到了客户端
 * 却**从未被读过**：令牌到期只能等服务端回 401 才发现，
 * 于是每次「息屏很久后回来看一眼」都必然浪费一次往返（请求 → 401 → 刷新 → 重发）。
 * 记下过期时刻就能在**发请求之前**先换好，省掉那一跳。
 */
export function setTokenInternal(access: string, refresh: string, expiresAt = ''): void {
  set(TOKEN_KEY, access)
  set(REFRESH_KEY, refresh)
  if (expiresAt) set(TOKEN_EXP_KEY, expiresAt)
  else del(TOKEN_EXP_KEY)
}

export function clearTokens(): void {
  del(TOKEN_KEY)
  del(REFRESH_KEY)
  del(TOKEN_EXP_KEY)
}

/**
 * 登出：吊销服务端该账号的全部 refresh token，再清本地令牌。
 *
 * # 为什么「登出」必须调服务端，不能只清本地
 *
 * 本地 `clearTokens()` 只是让**这台设备**不再用它。
 * 而 refresh token 的有效期是 30 天（服务端 `REFRESH_TTL`），
 * 在这期间它**仍然是有效的凭证** —— 手机丢了 / 令牌被读走之后，
 * 玩家在设置里点「退出登录」，攻击者手里的那份照样能换出新令牌。
 *
 * 这是服务端「唯一能批量吊销 refresh token 的路径此前只有封禁」那个
 * 缺陷的客户端那一半。
 *
 * ⚠️ **失败也必须清本地**：网络不通时服务端那一份没吊销，
 *   但用户的意图是「我现在不信任这台设备」，此时把有效令牌留在
 *   localStorage 是违背意图的。
 *   代价是「服务端那份可能还活着」—— 所以 UI 要如实告知用户
 *   「已在本机退出，但未能通知服务端」，而不是谎称已完全登出。
 */
export async function logout(): Promise<{ serverNotified: boolean }> {
  try {
    await request<{ ok: boolean }>('/me/logout', { method: 'POST' })
    clearTokens()
    return { serverNotified: true }
  } catch {
    clearTokens()
    return { serverNotified: false }
  }
}

interface RequestOptions {
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE'
  body?: unknown
  auth?: boolean
}

/** 统一错误响应体，与服务端 httpapi.APIError 对应。 */
interface ApiPayload {
  error?: { code: string; message: string }
  [k: string]: unknown
}

/**
 * 发起请求。
 *
 * 401 时自动用 refresh token 换新 access token 并重试一次；
 * 刷新也失败则清除令牌，让上层重新登录（而不是无限重试）。
 *
 * 第 150 轮：发请求**之前**先看 `TokenPair.expires_at`，
 * 已知快过期就先换新（`ensureFreshToken`），省掉一次必然 401 的往返。
 */
export function request<T>(path: string, opts: RequestOptions = {}): Promise<T> {
  const method = opts.method ?? 'GET'
  const auth = opts.auth !== false
  return ensureFreshToken(auth).then(
    () =>
      new Promise<T>((resolve, reject) => {
        const header: Record<string, string> = { 'Content-Type': 'application/json' }
        if (auth) {
          const t = getToken()
          if (t) header.Authorization = `Bearer ${t}`
        }
        uni.request({
          url: `${BASE_URL}${path}`,
          method,
          data: opts.body as Record<string, unknown> | undefined,
          header,
          timeout: 20000,
          success: async (res) => {
            const status = res.statusCode
            // uni.request 的 data 类型被声明为 string | AnyObject | ArrayBuffer，
            // 直接访问 .error 会报类型错误。统一收敛成结构化响应。
            const data = (res.data ?? {}) as ApiPayload
            if (status >= 200 && status < 300) {
              resolve(data as T)
              return
            }
            // 401：尝试刷新一次
            if (status === 401 && auth) {
              try {
                await refreshAccessToken()
                // 重试
                const retryHeader: Record<string, string> = { 'Content-Type': 'application/json' }
                const t = getToken()
                if (t) retryHeader.Authorization = `Bearer ${t}`
                uni.request({
                  url: `${BASE_URL}${path}`,
                  method,
                  data: opts.body as Record<string, unknown> | undefined,
                  header: retryHeader,
                  timeout: 20000,
                  success: (r2) => {
                    const d2 = (r2.data ?? {}) as ApiPayload
                    if (r2.statusCode >= 200 && r2.statusCode < 300) {
                      resolve(d2 as T)
                    } else {
                      reject(
                        new ApiError(
                          r2.statusCode,
                          d2.error?.code ?? 'http_error',
                          d2.error?.message ?? `HTTP ${r2.statusCode}`,
                        ),
                      )
                    }
                  },
                  fail: (e) => reject(new ApiError(0, 'network_error', e.errMsg)),
                })
                return
              } catch {
                clearTokens()
                reject(new ApiError(401, 'unauthorized', '登录已过期，请重新进入'))
                return
              }
            }
            reject(
              new ApiError(
                status,
                data.error?.code ?? 'http_error',
                data.error?.message ?? `请求失败 HTTP ${status}`,
              ),
            )
          },
          fail: (e) => reject(new ApiError(0, 'network_error', `无法连接服务器：${e.errMsg}`)),
        })
      }),
  )
}

/** 提前多少毫秒认为「快过期了」。留出网络往返 + 服务端时钟偏差的余量。 */
const TOKEN_EXPIRY_SKEW_MS = 30_000

/**
 * 令牌是否已知「马上要过期」。
 *
 * ⚠️ 用的是**本地时钟**（`Date.now`），不是服务端校准过的时钟 ——
 * api 层不该反向依赖 store 的 `estimateServerNowMs`。
 * 设备时钟错了会怎样？两个方向都安全：
 *
 *   - 钟走快了 → 误判为「快过期」→ 多刷一次。浪费一次 refresh，不影响正确性。
 *   - 钟走慢了 → 误判为「还早」→ 不主动刷 → 退回原来的 401 兜底路径。
 *
 * 所以这里刻意**不做**时钟校正 —— 那会让一个纯优化引入「钟错导致拒绝登录」的新风险。
 */
function tokenExpiringSoon(): boolean {
  const raw = get(TOKEN_EXP_KEY)
  if (!raw) return false // 不知道过期时刻（旧版本登录/被清过）→ 保持旧行为
  const at = Date.parse(raw)
  if (Number.isNaN(at)) return false // 解析不了也不猜，照旧走 401 兜底
  return at - Date.now() <= TOKEN_EXPIRY_SKEW_MS
}

/**
 * 发请求前的主动续期。
 *
 * 失败**不阻断**：清掉令牌后继续发，下面照常收到 401 并走既有兜底。
 * 这里抢跑只是为了省一次往返，不该把错误处理的责任从原路径搬过来。
 */
function ensureFreshToken(auth: boolean): Promise<void> {
  if (!auth) return Promise.resolve()
  if (!tokenExpiringSoon()) return Promise.resolve()
  return refreshAccessToken().catch(() => clearTokens())
}

/**
 * 正在进行中的刷新 Promise。
 *
 * ⚠️ 单飞（single-flight）是必需的，不是优化。服务端轮换 refresh token
 * 是原子的（条件 UPDATE + RETURNING），所以 N 个并发刷新里
 * **恰好 1 次成功、N-1 次 401**。若每个 401 handler 各自处理：
 * 成功的写入新令牌 → 随后 N-1 个失败分支执行 clearTokens()
 * → 把刚写入的新令牌一起删掉 → 用户被静默登出。
 *
 * 实际触发场景不需要攻击者：access token 2 小时过期后回到小程序，
 * 某页面并发拉 wallet + me + tasks + mastery 四个接口就是 4 个 401。
 * 所以这是**必然复现**的可用性缺陷。
 */
let refreshInflight: Promise<void> | null = null

function refreshAccessToken(): Promise<void> {
  if (refreshInflight) return refreshInflight
  refreshInflight = doRefresh().finally(() => {
    refreshInflight = null
  })
  return refreshInflight
}

function doRefresh(): Promise<void> {
  const refresh = get(REFRESH_KEY)
  if (!refresh) return Promise.reject(new Error('无 refresh token'))
  return new Promise((resolve, reject) => {
    uni.request({
      url: `${BASE_URL}/auth/refresh`,
      method: 'POST',
      data: { refresh_token: refresh },
      header: { 'Content-Type': 'application/json' },
      timeout: 20000,
      success: (res) => {
        if (res.statusCode >= 200 && res.statusCode < 300) {
          const d = res.data as { access_token: string; refresh_token: string }
          set(TOKEN_KEY, d.access_token)
          set(REFRESH_KEY, d.refresh_token)
          resolve()
        } else {
          reject(new Error('刷新失败'))
        }
      },
      fail: () => reject(new Error('刷新请求失败')),
    })
  })
}

// ---- 接口封装 ----

export interface TokenPair {
  access_token: string
  refresh_token: string
  expires_at: string
  user: { id: number; nickname: string; avatar_url: string; is_guest: boolean; status: number }
}

/** 游客登录。首次进入时自动建立账号。 */
export function guestLogin(nickname = ''): Promise<TokenPair> {
  return request<TokenPair>('/auth/guest', {
    method: 'POST',
    auth: false,
    body: { guest_token: getGuestToken(), nickname },
  })
}

export function fetchWallet() {
  // tokens 是道具余额（mastery_reset / revive_token / gem_wash_token）。
  // 消费端用它判断「重铸按钮该不该亮」。
  return request<{
    coin: number
    gem: number
    energy: number
    keys: number
    tokens: Record<string, number>
  }>('/wallet')
}

/**
 * 带条件请求的配置拉取。
 *
 * ## 为什么不用通用的 `request()`
 *
 * 这里要处理 **304 Not Modified**：通用包装把一切非 2xx 都当失败，
 * 而 304 在本场景里是**成功**（「你已经是最新的了」）。
 *
 * ## 这个端点的存在意义
 *
 * `version` 是内容哈希，客户端把它存在本地。带上它去问服务端：
 * 内容没变 → 服务端只回一个 304（**零字节载荷**），
 * 内容变了 → 正常 200 带全量。
 *
 * 省下的是每次冷启动的 24KB gzip 传输，加上客户端 168KB 的 JSON 解析。
 *
 * ## 304 时 `server_time` 从哪来
 *
 * 304 没有响应体，但客户端仍要校准本地时钟（`estimateServerNowMs`）。
 * 用 HTTP 标准 `Date` 响应头 —— 它每一跳都会刷新，且粒度 1 秒，
 * 对「护盾还剩几小时」这种判断完全够。
 *
 * ⚠️ 不能「304 就不校准」：那会让时钟偏移随着缓存存在时间一起漂移，
 * 而症状是**过期的护盾显示为生效中**（或反过来），
 * 且只在长时间未更新配置的老玩家身上复现。
 */
export type ConfigFetchResult =
  | { status: 'modified'; config: GameConfig; serverTimeMs: number }
  | { status: 'unmodified'; version: string; serverTimeMs: number }

export function fetchConfigIfVersion(haveVersion = ''): Promise<ConfigFetchResult> {
  return new Promise<ConfigFetchResult>((resolve, reject) => {
    const url = haveVersion
      ? `${BASE_URL}/config?if_version=${encodeURIComponent(haveVersion)}`
      : `${BASE_URL}/config`
    uni.request({
      url,
      method: 'GET',
      header: { 'Content-Type': 'application/json' },
      timeout: 20000,
      success: (res) => {
        const headers = (res.header ?? {}) as Record<string, unknown>
        const serverTimeMs = headerTimeMs(
          headers,
          (res.data as GameConfig | undefined)?.server_time,
        )
        if (res.statusCode === 304) {
          resolve({ status: 'unmodified', version: haveVersion, serverTimeMs })
          return
        }
        if (res.statusCode >= 200 && res.statusCode < 300) {
          resolve({ status: 'modified', config: res.data as GameConfig, serverTimeMs })
          return
        }
        reject(
          new ApiError(res.statusCode, 'http_error', `配置拉取失败 HTTP ${res.statusCode}`),
        )
      },
      fail: (e) => reject(new ApiError(0, 'network_error', `无法连接服务器：${e.errMsg}`)),
    })
  })
}

/**
 * 从响应头取服务端时间（毫秒时间戳）。
 *
 * `Date` 是 HTTP 标准头，代理/CDN 每一跳都会重写它，
 * 所以它反映的是「离客户端最近那一跳的服务端时钟」。
 * 解析失败时回退到响应体里的 `server_time`，再失败回退 0
 * —— 由调用方决定怎么处理 0（store 里会用本地时钟诚实兜底）。
 */
function headerTimeMs(headers: Record<string, unknown>, bodyServerTime?: string): number {
  // 大小写两路都试：uni-app 在 H5 与小程序两端对 header 键的处理不一致。
  const raw = headers['Date'] ?? headers['date']
  if (typeof raw === 'string') {
    const t = Date.parse(raw)
    if (Number.isFinite(t)) return t
  }
  if (typeof bodyServerTime === 'string') {
    const t = Date.parse(bodyServerTime)
    if (Number.isFinite(t)) return t
  }
  return 0
}

/**
 * 单关完整数据（含 `waves`）。
 *
 * ⚠️ `/config` 里的 `levels` 是**摘要**，没有波次 ——
 * 因为 100 关的刷怪排布占了整个配置包 56% 的体积，
 * 而客户端只有「防线挑战的标准战场」一处需要它。
 * 那一处走这里按需取；战斗页则用 `/battle/token` 下发的关卡（本来就带波次）。
 */
/** 玩家持有的一颗宝石实例（第 150 轮新增）。 */
export interface OwnedGem {
  id: number
  gem_id: number
  code: string
  name: string
  descr: string
  /** 品质名：gray / blue / purple / red / gold，与 `/config` 的 gem_qualities 同源。 */
  quality: string
  /** 词条，形如 `[{affix:"crit", value:42}]`。 */
  affixes: { affix: string; value: number }[]
  /** 已装备的槽位；空串表示未装备。 */
  equipped_slot: string
}

/**
 * 列出玩家持有的宝石。
 *
 * 此前 `user_gems` 只有服务端内部读取（计入攻击者派生属性），
 * **没有任何面向玩家的读取面** —— 玩家买了也看不见自己有什么。
 */
export function fetchGems() {
  return request<{ gems: OwnedGem[] }>('/me/gems')
}

/**
 * 洗练一颗宝石：消耗 1 枚洗练券，重新 roll 词条（品质不变）。
 *
 * 为什么要玩家点一下而不是自动洗：词条是**构筑选择**，
 * 自动洗等于替玩家做决定。
 */
export function washGem(gemId: number) {
  return request<{ gem: OwnedGem }>(`/me/gems/${gemId}/wash`, { method: 'POST' })
}

export function fetchLevel(id: number) {
  return request<import('@/game/types').GeneratedLevel>(`/levels/${id}`)
}

/** 服务端下发的构筑快照（结构与 game/replay.ts 的 BuildSnapshot 一致） */
export type BuildSnapshotView = import('@/game/replay').BuildSnapshot

export function fetchMe() {
  return request<{
    user_id: number
    build: BuildSnapshotView
    build_rating: any
    power: number
    /** 已通关最高关卡（第 142 轮：服务端权威进度，客户端刷新档案时恢复解锁态） */
    max_stage: number
  }>('/me')
}

/** /me/loadout 的响应。skill_ids 按槽位排列，0 表示空槽 */
export function fetchLoadout() {
  return request<{ skill_ids: number[] }>('/me/loadout')
}

/**
 * /me/stars 的响应。key 是关卡 ID（JSON 键为字符串），value 是该关历史最好星级（0–3）。
 * 从未结算的关卡不在 map 里，消费侧按 0 处理。
 */
export function fetchMyStars() {
  return request<{ stars: Record<string, number> }>('/me/stars')
}

/**
 * 保存出战技能。
 *
 * ⚠️ 必须落服务端，不能只存本地。槽位参与回放哈希计算 ——
 * 存在前端的话，验真方拿不到同一份槽位，I-6 从设计上就失效了。
 */
export function saveLoadout(skillIds: number[]) {
  return request<{ skill_ids: number[] }>('/me/loadout', {
    method: 'PUT',
    body: { skill_ids: skillIds },
  })
}

/**
 * 把一个已拥有的技能升一级。
 *
 * 费用与上限由服务端裁定（`service.UpgradeSkill` 在事务里读等级、
 * 条件扣费、升一级），客户端**不预判**能不能升 ——
 * 余额、满级、并发推满都由服务端说了算。
 * 客户端只负责把「点了升级」这个意图发出去。
 *
 * 响应里带 `wallet` 是刻意的：升级必然花钱，
 * 让前端再发一次 `/wallet` 才能刷新余额是个可避免的竞态
 * （玩家连点两次会看到中间态余额）。
 */
export function upgradeSkill(skillId: number) {
  return request<{
    skill_id: number
    level: number
    wallet: { coin: number; gem: number; energy: number; keys: number }
  }>(`/me/skills/${skillId}/upgrade`, { method: 'POST' })
}

/** 开局凭证。seed 是字符串 —— 服务端刻意不用 number，避免 JS 精度损失。 */
export interface BattleTokenResp {
  token_id: number
  seed: string
  expires_at: string
  level: import('@/game/types').GeneratedLevel & { seed_str: string }
  /** 本局是否消耗了复活币。 */
  revived?: boolean
  /**
   * 复活的额外当前血量（绝对值，未复活为 0）。
   *
   * 直接喂给引擎的 `BattleConfig.extraBaseHp`，**不要**加到 `level.base_hp` 上 ——
   * 漏怪伤害以 `baseHpMax` 为分母，一起抬高就把加成抵掉了。
   */
  extra_base_hp?: number
  build: any
}

export function startBattle(levelId: number) {
  return request<BattleTokenResp>('/battle/token', { method: 'POST', body: { level_id: levelId } })
}

export function settleBattle(payload: any) {
  return request<any>('/battle/settle', { method: 'POST', body: payload })
}

export function getReplay(battleId: number) {
  return request<any>(`/battle/${battleId}/replay`)
}

export function verifyReplay(battleId: number, replayHash: string) {
  return request<any>('/battle/verify', { method: 'POST', body: { battle_id: battleId, replay_hash: replayHash } })
}

export function fetchMastery() {
  return request<any>('/mastery')
}

export function allocateMastery(nodeId: number) {
  return request<any>('/mastery/allocate', { method: 'POST', body: { node_id: nodeId } })
}

/**
 * 重铸专精：消耗 1 件 `mastery_reset` 道具，清空全部已点亮节点。
 *
 * **点数预算不变** —— 只清「已选节点」，玩家重选一遍仍受同样的点数约束。
 *
 * ⚠️ 它存在的理由：`AllocateMastery` 是**只插不删**的，点错方向后
 * 没有任何改向途径。没有这个端点，`mastery_reset` 就只是商店里一件
 * 「买了不知道干嘛」的商品。
 *
 * 回执回显**清完之后的真实状态**（points / nodes / total_slots），
 * 外加 `reset_cleared`（本次清掉几个节点）—— 所以调用方不需要
 * 再发一次 `GET /mastery`。
 */
export function resetMastery() {
  return request<{
    reset_cleared: number
    points: number
    nodes: number[]
    layer_limit: number
    base_slots: number
    total_slots: number
  }>('/mastery/reset', { method: 'POST' })
}

export function fetchTasks(scope = 'daily') {
  return request<any>(`/tasks?scope=${scope}`)
}

export function claimTask(taskId: number) {
  return request<any>(`/tasks/${taskId}/claim`, { method: 'POST' })
}

export function signIn() {
  return request<any>('/signin', { method: 'POST' })
}

/** 七日签到奖励表（服务端权威，第 135 轮：预览不再本地硬编码）。 */
export interface SignInCalendarDay {
  day_index: number
  reward: Record<string, number>
}

export function fetchSignInCalendar() {
  return request<{ days: SignInCalendarDay[] }>('/signin/calendar')
}

/** /signin/status 响应：玩家签到状态（服务端权威，第 141 轮：初始展示不再写死）。 */
export interface SignInStatus {
  /** 本周期已签天数（MAX(day_index)，没签过为 0） */
  claimed_count: number
  /** 今天是否已签 */
  signed_today: boolean
  /** 今天还能不能签（今日未签且周期未满） */
  can_sign: boolean
}

export function fetchSignInStatus() {
  return request<SignInStatus>('/signin/status')
}

export function fetchShop() {
  return request<any>('/shop')
}

export function buyItem(itemId: number) {
  return request<any>(`/shop/${itemId}/buy`, { method: 'POST' })
}

export function redeem(code: string) {
  return request<any>('/redeem', { method: 'POST', body: { code } })
}

export function fetchLeaderboard(type = 'power') {
  return request<any>(`/leaderboard?type=${type}`)
}

export function fetchDefenses() {
  return request<any>('/defenses')
}

export function saveDefense(payload: any) {
  return request<any>('/defenses/save', { method: 'POST', body: payload })
}

export function challengeDefense(id: number, payload: any) {
  return request<any>(`/defenses/${id}/challenge`, { method: 'POST', body: payload })
}

export function diagnose(levelId: number, failedTimes: number) {
  return request<any>(`/diagnose?level_id=${levelId}&failed_times=${failedTimes}`)
}

export { BASE_URL }
