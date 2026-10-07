/**
 * 小程序端 API 客户端。
 *
 * 目标平台是微信小程序，但需要同时能在 H5 跑通（用于本机验证）。
 * uni.request 在两端 API 同构，因此直接用它，不引入额外 HTTP 库。
 */

const BASE_URL = 'http://127.0.0.1:8080/api/v1'

/** GameConfig 与服务端 /api/v1/config 的响应结构对应。 */
export interface GameConfig {
  version: number
  levels: import('@/game/types').GeneratedLevel[]
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
 */
export function setTokenInternal(access: string, refresh: string): void {
  set(TOKEN_KEY, access)
  set(REFRESH_KEY, refresh)
}

export function clearTokens(): void {
  del(TOKEN_KEY)
  del(REFRESH_KEY)
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
 */
export function request<T>(path: string, opts: RequestOptions = {}): Promise<T> {
  const method = opts.method ?? 'GET'
  const auth = opts.auth !== false
  return new Promise<T>((resolve, reject) => {
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
  })
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
  return request<{ coin: number; gem: number; energy: number; keys: number }>('/wallet')
}

export function fetchConfig() {
  return request<GameConfig>('/config', { auth: false })
}

/** 服务端下发的构筑快照（结构与 game/replay.ts 的 BuildSnapshot 一致） */
export type BuildSnapshotView = import('@/game/replay').BuildSnapshot

export function fetchMe() {
  return request<{
    user_id: number
    build: BuildSnapshotView
    build_rating: any
    power: number
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
