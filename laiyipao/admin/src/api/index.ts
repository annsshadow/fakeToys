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
