/** 管理员端 API 路径与类型定义（与 server/internal/httpapi/routes.go 一一对应） */

import { api } from './client'

// ---------- 认证 ----------

export interface AdminLoginResp {
  access_token: string
  admin: { id: number; username: string; role: string }
}

export function adminLogin(username: string, password: string) {
  return api.post<AdminLoginResp>('/admin/login', { username, password }, { auth: false })
}

export function adminMe() {
  return api.get<{ admin: { id: number; username: string; role: string } }>('/admin/me')
}

/**
 * 第 155 轮：后台登出。
 *
 * 服务端吊销该管理员的**全部**会话（与玩家侧 /me/logout 同口径），
 * 不是只清本地 localStorage。响应回带被吊销的会话条数（幂等：
 * 重复调用 = 0）。
 */
export function adminLogout() {
  return api.post<{ revoked_sessions: number }>('/admin/logout')
}

// ---------- 看板 ----------

export interface DashboardResp {
  users: { total: number; guests: number; wechat: number; banned: number; new_today: number }
  battles: { total: number; today: number; wins: number; avg_duration_ms: number }
  progression: { avg_max_stage: number; avg_power: number; stage_100_clears: number }
  economy: {
    coin_in: number
    coin_out: number
    gem_in: number
    gem_out: number
    orders_paid: number
    orders_pending: number
  }
  // I-6：验真健康度
  verification: { checked: number; matched: number; mismatched: number }
  // I-1：反应使用分布（验证元素机制是否真的在被使用）
  reaction_usage: Array<{ reaction: string; count: number }>
  daily_active: Array<{ date: string; dau: number }>
  stage_funnel: Array<{ level_id: number; attempts: number; clears: number }>
}

export function fetchDashboard() {
  return api.get<DashboardResp>('/admin/dashboard')
}

// ---------- 关卡 ----------

export interface AdminLevel {
  id: number
  chapter: number
  name: string
  seed: string
  base_hp: number
  wave_count: number
  difficulty: number
  energy_cost: number
  is_boss: boolean
  enabled: boolean
  terrain_config: unknown
  star_targets: number[]
  /**
   * 关卡配置的权威侧（第 23 条产品决策，2026-10-20）。
   * `generator` = 玩家侧由生成器产出，本页编辑只是规划记录（不影响线上）；
   * 若将来改走「表权威」路线，这里会变成 `table`。
   */
  player_authority?: 'generator' | 'table'
  /** 本次写入是否对玩家生效。`generator` 路线下恒为 false。 */
  applied_to_players?: boolean
  /**
   * 通关率，**已是百分数**（服务端算的是 clears*100/attempts）。
   * 视图层直接展示，不要再乘 100 —— 否则 100% 会被显示成 10000%。
   */
  clear_rate: number
  avg_wave: number
}

export function fetchLevels(params: { chapter?: number; keyword?: string } = {}) {
  const q = new URLSearchParams()
  if (params.chapter) q.set('chapter', String(params.chapter))
  if (params.keyword) q.set('keyword', params.keyword)
  const qs = q.toString()
  return api.get<{ items: AdminLevel[]; total: number }>(`/admin/levels${qs ? `?${qs}` : ''}`)
}

export function updateLevel(id: number, patch: Partial<AdminLevel>) {
  return api.put<{ level: AdminLevel }>(`/admin/levels/${id}`, patch)
}

export function regenerateLevels() {
  return api.post<{ generated: number }>('/admin/levels/regenerate')
}

export function fetchLevelWaves(levelId: number) {
  return api.get<{ waves: Array<{ wave_index: number; spawns: unknown }> }>(
    `/admin/levels/${levelId}/waves`,
  )
}

// ---------- 技能 / 装备 / 皮肤 ----------

export interface AdminSkill {
  id: number
  code: string
  name: string
  family: string
  element: string
  kind: string
  base_damage: number
  heat_cost: number
  cooldown_ms: number
  pierce: number
  aoe_radius: number
  apply_element: string
  apply_stacks: number
  unlock_level: number
  descr: string
}

export function fetchSkills() {
  return api.get<{ items: AdminSkill[]; recipes: Array<Record<string, number>> }>('/admin/skills')
}

export function updateSkill(id: number, patch: Partial<AdminSkill>) {
  return api.put<{ skill: AdminSkill }>(`/admin/skills/${id}`, patch)
}

export function fetchEquipment() {
  // Record<string, unknown> 而非 unknown[]：这三个目录表字段随版本增长，
  // 视图层用 str()/num() 安全取值即可，写死具体字段反而会在改表时报一片红。
  return api.get<{
    equipment: Array<Record<string, unknown>>
    gems: Array<Record<string, unknown>>
    skins: Array<Record<string, unknown>>
  }>('/admin/equipment')
}

// ---------- 用户 ----------

export interface AdminUser {
  id: number
  nickname: string
  is_guest: boolean
  status: number
  max_stage: number
  power: number
  coin: number
  gem: number
  last_login_at: string
  created_at: string
  /**
   * 验真统计。**两个字段必须一起看**：
   *
   *   0 / 0 → 从没被验真过 = **未知**（不是「干净」）
   *   n / 0 → 验过且都一致
   *   n / m → m > 0 = **可疑**
   *
   * 只看 `verify_mismatched` 会把「没验过」误判成「验过且没问题」。
   */
  verify_checked: number
  verify_mismatched: number
}

export function fetchUsers(params: { keyword?: string; limit?: number; offset?: number } = {}) {
  const q = new URLSearchParams()
  if (params.keyword) q.set('keyword', params.keyword)
  q.set('limit', String(params.limit ?? 50))
  q.set('offset', String(params.offset ?? 0))
  return api.get<{ items: AdminUser[]; total: number }>(`/admin/users?${q}`)
}

export function banUser(id: number, reason: string) {
  return api.post<{ user: AdminUser }>(`/admin/users/${id}/ban`, { reason })
}

export function unbanUser(id: number) {
  return api.post<{ user: AdminUser }>(`/admin/users/${id}/unban`)
}

export function grantUser(id: number, currency: string, amount: number) {
  return api.post<{ wallet: Record<string, number> }>(`/admin/users/${id}/grant`, {
    currency,
    amount,
  })
}

// ---------- 战斗记录与验真 ----------

export interface AdminBattle {
  id: number
  user_id: number
  level_id: number
  result: string
  stars: number
  score: number
  kills: number
  leaked: number
  wave_reached: number
  duration_ms: number
  reactions: number
  heat_max: number
  replay_hash: string
  created_at: string
  /** 第 123 轮：本关总怪数（生成器权威值），供「守恒」列做伪造判定 */
  total_enemies: number
  /**
   * 本场战报的验真状态。**两个字段必须一起看**：
   * `0 / 0` = 从没被验真过（**未知**，不是「干净」）。
   *
   * 上一轮 `fetchUsers` 那边已有同样两个字段，语义完全一致。
   */
  verify_checked: number
  verify_mismatched: number
}

export function fetchBattles(
  params: { user_id?: number; level_id?: number; limit?: number; only_mismatched?: boolean } = {},
) {
  const q = new URLSearchParams()
  if (params.user_id) q.set('user_id', String(params.user_id))
  if (params.level_id) q.set('level_id', String(params.level_id))
  // 只看验真不匹配的战报：从「这个用户有 3 次不匹配」直接跳到「是哪 3 场」
  if (params.only_mismatched) q.set('only_mismatched', '1')
  q.set('limit', String(params.limit ?? 50))
  return api.get<{ items: AdminBattle[]; total: number }>(`/admin/battles?${q}`)
}

export function fetchBattleDetail(id: number) {
  return api.get<{ battle: AdminBattle & Record<string, unknown> }>(`/admin/battles/${id}`)
}

/**
 * 运营侧发起验真（R41 新增）。
 *
 * ⚠️ **本端点不重算哈希** —— 服务端没有引擎。
 * 调用方需要自己在能跑引擎的地方用种子重放，把算出的 hash 贴进来。
 * 比对的是「提交的」与「结算时记录的」，所以它防的是
 * 「改了数据却没改凭证」，**不是**「从一开始就伪造」。
 */
export interface VerifyResult {
  battle_id: number
  expected_hash: string
  actual_hash: string
  matched: boolean
  level_id: number
  /** 字符串下发：int64 可能超过 2^53，JSON number 会被 JS 解析成错的 */
  seed: string
  recorded_at: string
}

export function verifyBattle(id: number, replayHash: string) {
  return api.post<VerifyResult>(`/admin/battles/${id}/verify`, { replay_hash: replayHash })
}

// ---------- 结算重算审计（R1）----------

/**
 * 上报值与重算值的一边。
 *
 * ⚠️ `recomputed` 侧允许整体缺失（`RecomputedRecomputeSide | null`）：
 *   引擎未接入时服务端没有重算值，这**不是**「重算结果是 0」。
 *   合法战斗里分数不可能算成 0，所以两者必须能区分 ——
 *   用同一个 `RecomputeSide` 类型会让界面把「没算」画成「0 分」，
 *   而后者是最强的可疑信号。
 */
export interface RecomputeSide {
  score: number
  kills: number
  duration_ms: number
  hash: string
}

export interface AdminRecompute {
  id: number
  /**
   * 关联的战报 id。
   *
   * ⚠️ **`0` 不是「战报 #0」，而是「没有战报」**：
   * `replay_skills` 不符发生在写战报**之前**，所以那批拒绝没有战报 id。
   * 界面上必须显示成「—」而不是 `0` ——
   * 显示 0 会让人去查一条不存在的战报。
   */
  battle_id: number
  user_id: number
  nickname: string
  level_id: number
  reported: RecomputeSide
  recomputed: RecomputeSide | null
  /** 不符的具体字段：`score` / `kills` / `duration` / `hash` / `replay_skills` */
  mismatched_fields: string[]
  /** 记录时的**实际生效**档位，可能已被熔断降级 */
  mode: string
  /** 这一次到底有没有真的拒发奖励 */
  rejected: boolean
  engine_version: string
  created_at: string
}

export function fetchRecomputes(
  params: {
    user_id?: number
    level_id?: number
    limit?: number
    offset?: number
    only_mismatch?: boolean
  } = {},
) {
  const q = new URLSearchParams()
  if (params.user_id) q.set('user_id', String(params.user_id))
  if (params.level_id) q.set('level_id', String(params.level_id))
  if (params.only_mismatch) q.set('only_mismatch', '1')
  q.set('limit', String(params.limit ?? 50))
  if (params.offset) q.set('offset', String(params.offset))
  return api.get<{ items: AdminRecompute[]; total: number }>(`/admin/recomputes?${q}`)
}

export interface RecomputeLevelStat {
  level_id: number
  total: number
  mismatched: number
  rate_permille: number
  /** 单个玩家在这一关的不符次数上限 —— 区分漂移与作弊的关键列 */
  max_user_mismatches: number
}

export interface RecomputeSummary {
  /** **实际生效**的档位（可能已被熔断降级） */
  mode: string
  /** 配置值。用来区分「本来就配的 shadow」与「被熔断了」 */
  configured_mode: string
  /** 为真 = enforce 已被熔断降级，**本该拒发的奖励正在放行** */
  demoted: boolean
  demoted_reason: string
  since: string
  breaker_rate_permille: number
  breaker_min_samples: number
  /** 台账侧读数：只含不符的局，受 `since` 窗口限制 */
  totals: {
    total: number
    mismatched: number
    rejected: number
    rate_permille: number
  }
  /** 按关卡分档的台账分布（只含不符） */
  levels: RecomputeLevelStat[]
  /**
   * 引擎侧累计读数：进程内计数器，**从进程启动算起**，含**所有**走到重算的局。
   *
   * ## 与 totals 的口径不同 —— 别混用
   *
   * 单看 totals 会把「校验根本没跑」误判成「没问题」，而那正是最坏的误判。
   * 这个字段是唯一一个「校验到底有没有在跑」的读数。
   */
  engine: {
    verified: number
    mismatch: number
    cannot_run: number
    skipped: number
    /** Verified+Mismatch+CannotRun+Skipped —— 作为覆盖率分母 */
    settled_total: number
    /** Verified/SettledTotal * 1000；settled_total==0 时为 -1 */
    verified_permille: number
    /** 一眼可判的结论字符串，判据在后端 */
    status: string
  }
}

/**
 * 取总账 + 按关卡分布 + 闸门状态。
 *
 * ⚠️ `hours` 是**时间窗**，缺省 24 小时、服务端上限 30 天。
 *   熔断看的是 10 分钟窗口，而「最近是否出现一批新的集中不符」要看更长 ——
 *   这两个尺度问的是不同的问题，所以由调用方选。
 */
export function fetchRecomputeStats(hours = 24) {
  return api.get<RecomputeSummary>(`/admin/recomputes/stats?hours=${hours}`)
}

/**
 * 反应表（key + 中文名 + 各档数值），直接对应服务端的 `domain.AllReactionSpecs()`。
 *
 * 看板的「反应使用分布」图用它把 `reactions_used` 的 key 翻成中文。
 *
 * 为什么不复用 `/api/v1/config`：那个响应实测 167,124 字节（`levels` 占 74%），
 * 而且每次请求都要重算 100 关关卡数据 —— 为了 7 个名字不值得。
 */
export interface ReactionSpec {
  key: string
  name: string
  base_coef: number
  attack_weight_pct: number
  status_duration_ms: number
  aoe_radius: number
  dispel_shield: boolean
  amplify_pct: number
  descr: string
}

/**
 * 响应包了一层 `{ reactions: [...] }` 而不是裸数组 ——
 * 与 admin 其余端点（`{items,total}` / `{equipment,gems,skins}`）保持一致，
 * 以后加分页或元数据也不必改响应形状。
 */
export function fetchReactions() {
  return api.get<{ reactions: ReactionSpec[] }>('/admin/reactions').then((r) => r.reactions)
}

// ---------- 防线 ----------

export function fetchDefenses(limit = 50) {
  return api.get<{ items: Array<Record<string, unknown>>; total: number }>(
    `/admin/defenses?limit=${limit}`,
  )
}

// ---------- 经济 / 商城 / 公告 / 兑换码 ----------

export function fetchEconomy() {
  return api.get<{ flows: Array<Record<string, unknown>>; shop: Array<Record<string, unknown>> }>(
    '/admin/economy',
  )
}

export function updateShopItem(id: number, patch: Record<string, unknown>) {
  return api.put<{ item: Record<string, unknown> }>(`/admin/shop/${id}`, patch)
}

export function fetchAnnouncements() {
  return api.get<{ items: Array<Record<string, unknown>> }>('/admin/announcements')
}

export function createAnnouncement(payload: { title: string; body: string; published: boolean }) {
  return api.post<{ item: Record<string, unknown> }>('/admin/announcements', payload)
}

export function fetchRedeemCodes() {
  return api.get<{ items: Array<Record<string, unknown>> }>('/admin/redeem-codes')
}

export function createRedeemCode(payload: {
  code: string
  reward: Record<string, number>
  max_uses: number
  expires_at?: string
}) {
  return api.post<{ item: Record<string, unknown> }>('/admin/redeem-codes', payload)
}

export function fetchAuditLogs(limit = 100) {
  return api.get<{ items: Array<Record<string, unknown>> }>(`/admin/audit-logs?limit=${limit}`)
}
