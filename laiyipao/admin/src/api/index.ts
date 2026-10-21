/**
 * 管理员端 API 路径与类型（与 server/internal/httpapi/routes.go 一一对应）。
 *
 * ## 类型从哪来
 *
 * 本文件里的**响应类型全部来自 `./types.gen.ts`** —— 那是**生成物**，
 * 由 `laiyipao/server/cmd/tygen` 从 Go 的响应 struct 派生。
 *
 * 改一个字段名会同时改到两端：Go 侧改了 → 重新生成 → 前端 `type-check` 立刻红。
 * 这正是本轮要消灭的缺陷的根治点：此前响应是手写的 `map[string]any`，
 * 前端用 `Record<string, unknown>` 承接，字段名拼错**不报错**，
 * 于是「战报详情抽屉 14 个字段恒空」这类问题能一路活到线上，
 * 而测试 fixture 又按前端形状写 —— 每一条都测得绿。
 *
 * **不要在这里手写响应类型。** 需要新形状时：改 Go 侧 → `go run ./cmd/tygen`。
 */

import { api } from './client'
import type {
  AdminAnnouncementResp,
  AdminAnnouncementView,
  AdminAnnouncementsResp,
  AdminAuditLogView,
  AdminAuditLogsResp,
  AdminBattleDetailResp,
  AdminBattleDetailView,
  AdminBattleView,
  AdminBattlesResp,
  AdminDefenseView,
  AdminDefensesResp,
  AdminEconomyFlowView,
  AdminEconomyResp,
  AdminEquipmentResp,
  AdminEquipmentView,
  AdminGrantResp,
  AdminLevelResp,
  AdminLevelView,
  AdminLevelsResp,
  AdminLoginResp,
  AdminLogoutResp,
  AdminMeResp,
  AdminReactionsResp,
  AdminRecomputesResp,
  AdminRedeemCodeResp,
  AdminRedeemCodeView,
  AdminRedeemCodesResp,
  AdminRegenerateResp,
  AdminShopItemResp,
  AdminShopItemView,
  AdminSkillResp,
  AdminSkillView,
  AdminSkillsResp,
  AdminUserResp,
  AdminUserView,
  AdminUsersResp,
  AdminWavesResp,
  Dashboard,
  RecomputeLevelStat,
  RecomputeSummary,
  RecomputeView,
  SeedGem,
  SeedRecipe,
  SeedSkin,
} from './types.gen'

export type {
  AdminAnnouncementResp,
  AdminAnnouncementView,
  AdminAnnouncementsResp,
  AdminAuditLogView,
  AdminAuditLogsResp,
  AdminBattleDetailResp,
  AdminBattleDetailView,
  AdminBattleView,
  AdminBattlesResp,
  AdminDefenseView,
  AdminDefensesResp,
  AdminEconomyFlowView,
  AdminEconomyResp,
  AdminEquipmentResp,
  AdminEquipmentView,
  AdminGrantResp,
  AdminLevelResp,
  AdminLevelView,
  AdminLevelsResp,
  AdminLoginResp,
  AdminLogoutResp,
  AdminMeResp,
  AdminReactionsResp,
  AdminRecomputesResp,
  AdminRedeemCodeResp,
  AdminRedeemCodeView,
  AdminRedeemCodesResp,
  AdminRegenerateResp,
  AdminShopItemResp,
  AdminShopItemView,
  AdminSkillResp,
  AdminSkillView,
  AdminSkillsResp,
  AdminUserResp,
  AdminUserView,
  AdminUsersResp,
  AdminWavesResp,
  Dashboard,
  RecomputeLevelStat,
  RecomputeSummary,
  RecomputeView,
  SeedGem,
  SeedRecipe,
  SeedSkin,
}

/** 后台可写字段白名单（服务端 `PUT /admin/levels/:id` 的 9 个键）。 */
export type AdminLevelPatch = Partial<
  Pick<
    AdminLevelView,
    | 'name'
    | 'base_hp'
    | 'wave_count'
    | 'difficulty'
    | 'energy_cost'
    | 'star_targets'
    | 'terrain_config'
    | 'is_boss'
    | 'enabled'
  >
>

/** 后台可写字段白名单（服务端 `PUT /admin/skills/:id` 的 11 个键）。 */
export type AdminSkillPatch = Partial<
  Pick<
    import('./types.gen').AdminSkillView,
    | 'name'
    | 'descr'
    | 'base_damage'
    | 'heat_cost'
    | 'cooldown_ms'
    | 'pierce'
    | 'aoe_radius'
    | 'apply_stacks'
    | 'projectile_speed'
    | 'chain'
    | 'element'
    | 'family'
    | 'kind'
  >
>

/** 后台可写字段白名单（服务端 `PUT /admin/shop/:id` 的 6 个键）。 */
export type AdminShopPatch = Partial<
  Pick<
    import('./types.gen').AdminShopItemView,
    'name' | 'price' | 'payload' | 'limit_per_day' | 'sort_order' | 'enabled'
  >
>

// ---------- 认证 ----------

export function adminLogin(username: string, password: string) {
  return api.post<AdminLoginResp>('/admin/login', { username, password }, { auth: false })
}

export function adminMe() {
  return api.get<AdminMeResp>('/admin/me')
}

/**
 * 第 155 轮：后台登出。
 *
 * 服务端吊销该管理员的**全部**会话（与玩家侧 /me/logout 同口径），
 * 不是只清本地 localStorage。响应回带被吊销的会话条数（幂等：
 * 重复调用 = 0）。
 */
export function adminLogout() {
  return api.post<AdminLogoutResp>('/admin/logout')
}

// ---------- 看板 ----------

export function fetchDashboard() {
  return api.get<Dashboard>('/admin/dashboard')
}

/**
 * 「尽力而为」版看板：401 **不**触发跳转登录。
 *
 * 2026-10-21（P2-5）：玩家向的文档站用这个版本。
 * 匿名访客拿到 401 时，页面照常显示 —— 之前那次 401 会把
 * `location.hash` 改成 `#/admin/login`，把访客踢去了运营后台。
 */
export function fetchDashboardSoft() {
  return api.get<Dashboard>('/admin/dashboard', { soft: true })
}

// ---------- 关卡 ----------

export function fetchLevels(params: { chapter?: number; keyword?: string } = {}) {
  const q = new URLSearchParams()
  if (params.chapter) q.set('chapter', String(params.chapter))
  if (params.keyword) q.set('keyword', params.keyword)
  const qs = q.toString()
  return api.get<AdminLevelsResp>(`/admin/levels${qs ? `?${qs}` : ''}`)
}

export function updateLevel(id: number, patch: AdminLevelPatch) {
  return api.put<AdminLevelResp>(`/admin/levels/${id}`, patch)
}

/**
 * 用生成器重建全部 100 关。
 *
 * ⚠️ 2026-10-21（P3-6）：现在必须带 `confirm: 'regenerate'` ——
 * 这是**最重的写操作**（一次覆盖运营手工调过的所有关卡值，且不可回滚），
 * 服务端要求显式确认口令。
 */
export function regenerateLevels() {
  return api.post<AdminRegenerateResp>('/admin/levels/regenerate', { confirm: 'regenerate' })
}

export function fetchLevelWaves(levelId: number) {
  return api.get<AdminWavesResp>(`/admin/levels/${levelId}/waves`)
}

// ---------- 技能 / 装备 / 皮肤 ----------

export function fetchSkills() {
  return api.get<AdminSkillsResp>('/admin/skills')
}

export function updateSkill(id: number, patch: AdminSkillPatch) {
  return api.put<AdminSkillResp>(`/admin/skills/${id}`, patch)
}

export function fetchEquipment() {
  return api.get<AdminEquipmentResp>('/admin/equipment')
}

// ---------- 用户 ----------

export function fetchUsers(params: { keyword?: string; limit?: number; offset?: number } = {}) {
  const q = new URLSearchParams()
  if (params.keyword) q.set('keyword', params.keyword)
  q.set('limit', String(params.limit ?? 50))
  q.set('offset', String(params.offset ?? 0))
  return api.get<AdminUsersResp>(`/admin/users?${q}`)
}

export function banUser(id: number, reason: string) {
  return api.post<AdminUserResp>(`/admin/users/${id}/ban`, { reason })
}

export function unbanUser(id: number) {
  return api.post<AdminUserResp>(`/admin/users/${id}/unban`)
}

export function grantUser(id: number, currency: string, amount: number) {
  return api.post<AdminGrantResp>(`/admin/users/${id}/grant`, { currency, amount })
}

// ---------- 战斗记录与验真 ----------

export function fetchBattles(
  params: {
    user_id?: number
    level_id?: number
    limit?: number
    offset?: number
    only_mismatched?: boolean
  } = {},
) {
  const q = new URLSearchParams()
  if (params.user_id) q.set('user_id', String(params.user_id))
  if (params.level_id) q.set('level_id', String(params.level_id))
  // 只看验真不匹配的战报：从「这个用户有 3 次不匹配」直接跳到「是哪 3 场」
  if (params.only_mismatched) q.set('only_mismatched', '1')
  q.set('limit', String(params.limit ?? 50))
  // 2026-10-21（P5-1）：分页。此前只能看前 50 条 ——
  // 战报表可能有上万行，而「共 N 条」显示的其实是**这次返回了几条**。
  if (params.offset) q.set('offset', String(params.offset))
  return api.get<AdminBattlesResp>(`/admin/battles?${q}`)
}

/**
 * 战报详情。
 *
 * ⚠️ 2026-10-21：此前这个端点返回的是**复现信息**（`ReplayInfo`：
 * seed / expected_hash / build / card_picks），而后台抽屉按 `AdminBattle`
 * 的字段读 —— 于是 14/16 个字段恒空。现在返回战报详情本身。
 */
export function fetchBattleDetail(id: number) {
  return api.get<AdminBattleDetailResp>(`/admin/battles/${id}`)
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
  return api.get<AdminRecomputesResp>(`/admin/recomputes?${q}`)
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
 * 重算闸门状态（2026-10-21 P3-3）。
 *
 * `mode` 是**实际生效**的档，`configured_mode` 是配置里写的档。
 * 两者不一致 = 被熔断降级了 —— 那时 enforce 该拒发的奖励正在**放行**。
 */
export interface RecomputeModeState {
  mode: string
  configured_mode: string
  demoted: boolean
  demoted_reason: string
  breaker_rate_permille: number
  breaker_min_samples: number
}

export function fetchRecomputeMode() {
  return api.get<RecomputeModeState>('/admin/recomputes/mode')
}

/**
 * 切换重算档位。
 *
 * ⚠️ 破坏性操作：把 enforce 改成 off 等于关掉反作弊校验。
 * 所以服务端要求带 `confirm: 'confirm'` —— 一个手滑不该能关掉校验。
 */
export function setRecomputeMode(mode: string) {
  return api.put<RecomputeModeState>('/admin/recomputes/mode', { mode, confirm: 'confirm' })
}

// ---------- 反应表 ----------

/**
 * 反应表（key + 中文名 + 各档数值），直接对应服务端的 `domain.AllReactionSpecs()`。
 *
 * 看板的「反应使用分布」图用它把 `reactions_used` 的 key 翻成中文。
 *
 * 为什么不复用 `/api/v1/config`：那个响应实测 167,124 字节（`levels` 占 74%），
 * 而且每次请求都要重算 100 关关卡数据 —— 为了 7 个名字不值得。
 */
export function fetchReactions() {
  return api.get<AdminReactionsResp>('/admin/reactions').then((r) => r.reactions)
}

// ---------- 防线 ----------

export function fetchDefenses(limit = 50) {
  return api.get<AdminDefensesResp>(`/admin/defenses?limit=${limit}`)
}

// ---------- 经济 / 商城 / 公告 / 兑换码 ----------

export function fetchEconomy() {
  return api.get<AdminEconomyResp>('/admin/economy')
}

export function updateShopItem(id: number, patch: AdminShopPatch) {
  return api.put<AdminShopItemResp>(`/admin/shop/${id}`, patch)
}

export function fetchAnnouncements() {
  return api.get<AdminAnnouncementsResp>('/admin/announcements')
}

export function createAnnouncement(payload: { title: string; body: string; published: boolean }) {
  return api.post<AdminAnnouncementResp>('/admin/announcements', payload)
}

export function fetchRedeemCodes() {
  return api.get<AdminRedeemCodesResp>('/admin/redeem-codes')
}

export function createRedeemCode(payload: {
  code: string
  reward: Record<string, number>
  max_uses: number
  expires_at?: string
}) {
  return api.post<AdminRedeemCodeResp>('/admin/redeem-codes', payload)
}

export function fetchAuditLogs(limit = 100, offset = 0) {
  // 2026-10-21（P5-1）：审计日志也要能翻页 —— 固定 100 条时，
  // 更早的操作**根本看不到**。
  return api.get<AdminAuditLogsResp>(`/admin/audit-logs?limit=${limit}&offset=${offset}`)
}
