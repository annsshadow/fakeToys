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
  AdminDefenseChallengeView,
  AdminDefenseChallengesResp,
  AdminEconomyFlowView,
  AdminEconomyResp,
  AdminOrderView,
  AdminOrdersResp,
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
  AdminRedeemUsageView,
  AdminRedeemUsagesResp,
  AdminRegenerateResp,
  AdminShopItemResp,
  AdminShopItemView,
  AdminSkillResp,
  AdminSkillView,
  AdminSkillsResp,
  AdminUserDetailResp,
  AdminUserResp,
  AdminUserView,
  AdminVerificationView,
  AdminUsersResp,
  AdminWavesResp,
  Dashboard,
  RecomputeLevelStat,
  RecomputeModeState,
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
  AdminDefenseChallengeView,
  AdminDefenseChallengesResp,
  AdminEconomyFlowView,
  AdminEconomyResp,
  AdminOrderView,
  AdminOrdersResp,
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
  AdminRedeemUsageView,
  AdminRedeemUsagesResp,
  AdminRegenerateResp,
  AdminShopItemResp,
  AdminShopItemView,
  AdminSkillResp,
  AdminSkillView,
  AdminSkillsResp,
  AdminUserDetailResp,
  AdminUserResp,
  AdminUserView,
  AdminVerificationView,
  AdminUsersResp,
  AdminWavesResp,
  Dashboard,
  RecomputeLevelStat,
  RecomputeModeState,
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

export function fetchUsers(params: {
  keyword?: string
  limit?: number
  offset?: number
  date_from?: string
  date_to?: string
  is_guest?: boolean
  last_login_from?: string
  last_login_to?: string
  suspicious?: boolean
} = {}) {
  const q = new URLSearchParams()
  if (params.keyword) q.set('keyword', params.keyword)
  q.set('limit', String(params.limit ?? 50))
  q.set('offset', String(params.offset ?? 0))
  // 注册日期范围（2026-10-31 新一轮第 15 轮）：查「今天/某时间段」注册的用户
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  // 游客/账号筛选（2026-11-01 新一轮第 37 轮）
  if (params.is_guest !== undefined) q.set('is_guest', params.is_guest ? '1' : '0')
  // 最后登录时间范围（2026-11-01 新一轮第 46 轮）：查「最近活跃」的用户
  if (params.last_login_from) q.set('last_login_from', params.last_login_from)
  if (params.last_login_to) q.set('last_login_to', params.last_login_to)
  // 需核查筛选（2026-11-01 新一轮第 60 轮）
  if (params.suspicious !== undefined) q.set('suspicious', params.suspicious ? '1' : '0')
  return api.get<AdminUsersResp>(`/admin/users?${q}`)
}

/**
 * 导出用户列表 CSV（2026-10-30 实质优化第 12 轮）。
 *
 * 过滤条件与 `fetchUsers` 一致（keyword 模糊搜昵称 + 注册日期范围）。
 * 返回带 UTF-8 BOM 的 CSV Blob（Excel 中文兼容）。readonly 可导出。
 */
export function exportUsersCsv(params: {
  keyword?: string
  date_from?: string
  date_to?: string
  last_login_from?: string
  last_login_to?: string
  suspicious?: boolean
} = {}) {
  const q = new URLSearchParams()
  if (params.keyword) q.set('keyword', params.keyword)
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  if (params.last_login_from) q.set('last_login_from', params.last_login_from)
  if (params.last_login_to) q.set('last_login_to', params.last_login_to)
  if (params.suspicious !== undefined) q.set('suspicious', params.suspicious ? '1' : '0')
  return api.getBlob(`/admin/users/export?${q}`)
}

/**
 * 导出单个用户钱包流水 CSV（2026-11-01 新一轮第 82 轮）。
 * readonly 可访问（读操作）。
 */
export function exportUserFlowsCsv(userId: number) {
  return api.getBlob(`/admin/users/${userId}/flows/export`)
}

/**
 * 玩家详情（2026-10-21 P5-7）。
 *
 * 此前后台**只能看到用户列表的一行**，运营要回答「他最近打了什么 /
 * 钱去哪了 / 构筑是什么 / 被验真验出不匹配了吗」只能去别处拼 URL。
 */
export function fetchUserDetail(id: number) {
  return api.get<AdminUserDetailResp>(`/admin/users/${id}`)
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

/**
 * 后台重置玩家密码（2026-10-30 实质优化第 11 轮）。
 *
 * 玩家侧已有改密/转正，但运营缺少兜底：账号遗忘 / 被盗需要强制轮换时
 * 只能封禁或改环境变量重启。重置会更新哈希并吊销该账号全部会话。
 */
export function resetPlayerPassword(id: number, newPassword: string) {
  return api.post(`/admin/users/${id}/reset-password`, { new_password: newPassword })
}

// ---------- 战斗记录与验真 ----------

export function fetchBattles(
  params: {
    user_id?: number
    level_id?: number
    limit?: number
    offset?: number
    only_mismatched?: boolean
    date_from?: string
    date_to?: string
    result?: string
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
  // 日期范围（2026-10-31 新一轮第 14 轮）：查「今天/某时间段」的战报。
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  // 结果筛选（2026-11-01 新一轮第 44 轮）：win / lose
  if (params.result) q.set('result', params.result)
  return api.get<AdminBattlesResp>(`/admin/battles?${q}`)
}

/**
 * 导出战报列表 CSV（2026-10-30 实质优化第 13 轮）。
 *
 * 过滤条件与 `fetchBattles` 一致（user_id / level_id / only_mismatched
 * / date_from / date_to）。返回带 UTF-8 BOM 的 CSV Blob（Excel 中文兼容）。
 * readonly 可导出。
 */
export function exportBattlesCsv(params: {
  user_id?: number
  level_id?: number
  only_mismatched?: boolean
  date_from?: string
  date_to?: string
  result?: string
} = {}) {
  const q = new URLSearchParams()
  if (params.user_id) q.set('user_id', String(params.user_id))
  if (params.level_id) q.set('level_id', String(params.level_id))
  if (params.only_mismatched) q.set('only_mismatched', '1')
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  if (params.result) q.set('result', params.result)
  return api.getBlob(`/admin/battles/export?${q}`)
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
 * 重算台账 CSV 导出（2026-11-01 新一轮第 45 轮）。
 * 与列表同口径的 only_mismatch / user_id / level_id 筛选。
 */
export function exportRecomputesCsv(params: {
  user_id?: number
  level_id?: number
  only_mismatch?: boolean
} = {}) {
  const q = new URLSearchParams()
  if (params.user_id) q.set('user_id', String(params.user_id))
  if (params.level_id) q.set('level_id', String(params.level_id))
  if (params.only_mismatch) q.set('only_mismatch', '1')
  return api.getBlob(`/admin/recomputes/export?${q}`)
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
 * 重算闸门状态 —— **类型来自生成物**（`./types.gen`）。
 *
 * ⚠️ 2026-10-21（二轮审计 P1）：此前这里**手写**了一份同名类型，
 * 而它没有进服务端的类型注册表 —— 于是「唯一事实源」当场破功：
 * 改 Go 侧的 json tag，前端**不会红**。
 *
 * 现在它由 `cmd/tygen` 生成，改一处两端同时变。
 *
 * 语义（服务端 `RecomputeModeState` 的注释是权威）：
 *   - `mode` / `skills_mode` —— **权威档位**。S 段（replay_skills）
 *     才是真正「拒发奖励」的那一段，且它**永不**被熔断降级。
 *   - `engine_mode` —— 引擎侧（整局 `replay_hash`）实际生效的档，
 *     它**可以被熔断降级**。
 *   - `enforcing` —— S 段此刻是否会拒发奖励。
 */

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

/**
 * 防线列表（2026-10-21 P2-6）：带 offset ——
 * 此前只能看前 100 条，而 `total` 是全量 COUNT，
 * 于是「共 N 条防线」却只看得到 100 条（漏掉的还是**弱**防线，按战力倒序）。
 */
export function fetchDefenses(
  limit = 50,
  offset = 0,
  params: { keyword?: string; owner_id?: number } = {},
) {
  const q = new URLSearchParams()
  q.set('limit', String(limit))
  q.set('offset', String(offset))
  // 防线名/玩家昵称搜索 + owner_id 过滤（2026-10-31 新一轮第 19 轮）
  if (params.keyword) q.set('keyword', params.keyword)
  if (params.owner_id) q.set('owner_id', String(params.owner_id))
  return api.get<AdminDefensesResp>(`/admin/defenses?${q}`)
}

/**
 * 防线挑战记录（2026-10-31 新一轮第 30 轮）。
 * 运营此前只能看到防线胜负计数，无法查看挑战明细。
 */
export function fetchAdminDefenseChallenges(
  limit = 50,
  offset = 0,
  params: { defense_id?: number; challenger_id?: number; date_from?: string; date_to?: string } = {},
) {
  const q = new URLSearchParams()
  q.set('limit', String(limit))
  q.set('offset', String(offset))
  if (params.defense_id) q.set('defense_id', String(params.defense_id))
  if (params.challenger_id) q.set('challenger_id', String(params.challenger_id))
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  return api.get<AdminDefenseChallengesResp>(`/admin/defense-challenges?${q}`)
}

/**
 * 导出防线挑战记录 CSV（2026-10-31 新一轮第 32 轮）。
 * 过滤条件与列表一致。返回带 UTF-8 BOM 的 CSV Blob。readonly 可导出。
 */
export function exportDefenseChallengesCsv(params: {
  defense_id?: number
  challenger_id?: number
  date_from?: string
  date_to?: string
} = {}) {
  const q = new URLSearchParams()
  if (params.defense_id) q.set('defense_id', String(params.defense_id))
  if (params.challenger_id) q.set('challenger_id', String(params.challenger_id))
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  return api.getBlob(`/admin/defense-challenges/export?${q}`)
}

/**
 * 删除玩家防线（2026-10-31 实质优化第 26 轮）。
 * 违规防线名（侮辱/广告等）运营此前只能看到、无法处理。
 */
export function deleteDefense(id: number) {
  return api.del(`/admin/defenses/${id}`)
}

// ---------- 经济 / 商城 / 公告 / 兑换码 ----------

export function fetchEconomy() {
  return api.get<AdminEconomyResp>('/admin/economy')
}

/**
 * 订单列表（2026-10-31 实质优化第 33 轮）。
 * 后台此前只有订单计数，没有明细可查。
 */
export function fetchAdminOrders(
  limit = 50,
  offset = 0,
  params: { status?: string; date_from?: string; date_to?: string; provider?: string } = {},
) {
  const q = new URLSearchParams()
  q.set('limit', String(limit))
  q.set('offset', String(offset))
  // 状态 + 日期范围（2026-10-31 新一轮第 16 轮）
  if (params.status) q.set('status', params.status)
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  // 渠道筛选（2026-11-01 新一轮第 67 轮）
  if (params.provider) q.set('provider', params.provider)
  return api.get<AdminOrdersResp>(`/admin/orders?${q}`)
}

/**
 * 订单 CSV 导出（2026-11-01 新一轮第 40 轮）。
 * 与列表同口径的状态 + 创建日期筛选，运营对账需要全量快照。
 */
export function exportOrdersCsv(params: {
  status?: string
  date_from?: string
  date_to?: string
  provider?: string
} = {}) {
  const q = new URLSearchParams()
  if (params.status) q.set('status', params.status)
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  if (params.provider) q.set('provider', params.provider)
  return api.getBlob(`/admin/orders/export?${q}`)
}

export function updateShopItem(id: number, patch: AdminShopPatch) {
  return api.put<AdminShopItemResp>(`/admin/shop/${id}`, patch)
}

export function fetchAnnouncements(
  limit = 50,
  offset = 0,
  params: { published?: boolean; date_from?: string; date_to?: string } = {},
) {
  const q = new URLSearchParams()
  q.set('limit', String(limit))
  q.set('offset', String(offset))
  // 发布状态 + 创建日期筛选（2026-10-31 新一轮第 21 轮）
  if (params.published !== undefined) q.set('published', params.published ? '1' : '0')
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  return api.get<AdminAnnouncementsResp>(`/admin/announcements?${q}`)
}

export function createAnnouncement(payload: { title: string; body: string; published: boolean }) {
  return api.post<AdminAnnouncementResp>('/admin/announcements', payload)
}

/**
 * 更新公告（2026-10-31 实质优化第 23 轮）。
 * 此前只有创建/列表，发错内容无法修改、发错发布状态无法撤回。
 */
export function updateAnnouncement(id: number, payload: { title: string; body: string; published: boolean }) {
  return api.put<AdminAnnouncementResp>(`/admin/announcements/${id}`, payload)
}

/** 删除公告（2026-10-31 实质优化第 23 轮）。 */
export function deleteAnnouncement(id: number) {
  return api.del(`/admin/announcements/${id}`)
}

/**
 * 导出公告 CSV（2026-10-31 实质优化第 38 轮）。
 * 运营需要把公告清单带走做归档。readonly 可导出。
 */
export function exportAnnouncementsCsv() {
  return api.getBlob('/admin/announcements/export')
}

export function fetchRedeemCodes(
  limit = 50,
  offset = 0,
  params: { keyword?: string; enabled?: boolean; expired?: boolean } = {},
) {
  const q = new URLSearchParams()
  q.set('limit', String(limit))
  q.set('offset', String(offset))
  // 码搜索 + 启用/过期筛选（2026-10-31 新一轮第 20 轮）
  if (params.keyword) q.set('keyword', params.keyword)
  if (params.enabled !== undefined) q.set('enabled', params.enabled ? '1' : '0')
  if (params.expired !== undefined) q.set('expired', params.expired ? '1' : '0')
  return api.get<AdminRedeemCodesResp>(`/admin/redeem-codes?${q}`)
}

/**
 * 兑换码使用记录（2026-10-31 新一轮第 17 轮）。
 * 运营发码后此前无法查看「谁用了 / 奖励是什么」。
 */
export function fetchAdminRedeemUsages(
  limit = 50,
  offset = 0,
  params: { code_keyword?: string; user_id?: number } = {},
) {
  const q = new URLSearchParams()
  q.set('limit', String(limit))
  q.set('offset', String(offset))
  // 按码搜索 / 用户过滤（2026-10-31 新一轮第 23 轮）
  if (params.code_keyword) q.set('code_keyword', params.code_keyword)
  if (params.user_id) q.set('user_id', String(params.user_id))
  return api.get<AdminRedeemUsagesResp>(`/admin/redeem-usages?${q}`)
}

/**
 * 导出兑换码使用记录 CSV（2026-10-31 新一轮第 34 轮）。
 * 过滤条件与列表一致。返回带 UTF-8 BOM 的 CSV Blob。readonly 可导出。
 */
export function exportRedeemUsagesCsv(params: {
  code_keyword?: string
  user_id?: number
} = {}) {
  const q = new URLSearchParams()
  if (params.code_keyword) q.set('code_keyword', params.code_keyword)
  if (params.user_id) q.set('user_id', String(params.user_id))
  return api.getBlob(`/admin/redeem-usages/export?${q}`)
}

export function createRedeemCode(payload: {
  code: string
  reward: Record<string, number>
  max_uses: number
  expires_at?: string
}) {
  return api.post<AdminRedeemCodeResp>('/admin/redeem-codes', payload)
}

/**
 * 批量生成兑换码（2026-11-01 新一轮第 57 轮）。
 * 一次生成 count 个随机码（服务端 crypto/rand 生成）。
 */
export function createRedeemCodesBatch(payload: {
  count: number
  reward: Record<string, number>
  max_uses: number
  expires_at?: string
}) {
  return api.post<{ items: AdminRedeemCodeView[] }>('/admin/redeem-codes/batch', payload)
}

/**
 * 启用/禁用兑换码（2026-10-31 实质优化第 24 轮）。
 * 运营创建错误码后此前无法停用（只能等过期）。
 */
export function setRedeemCodeEnabled(id: number, enabled: boolean) {
  return api.put<AdminRedeemCodeResp>(`/admin/redeem-codes/${id}/enabled`, { enabled })
}

/**
 * 更新兑换码（2026-10-31 实质优化第 25 轮）。
 * 改奖励/次数/过期时间此前只能停用后重建（码会变，已分享的全作废）。
 */
export function updateRedeemCode(id: number, payload: {
  reward: Record<string, number>
  max_uses: number
  expires_at?: string
}) {
  return api.put<AdminRedeemCodeResp>(`/admin/redeem-codes/${id}`, payload)
}

/**
 * 导出兑换码 CSV（2026-10-31 实质优化第 37 轮）。
 * 运营需要把已生成码的清单带走做线下核对/客服分发。readonly 可导出。
 */
export function exportRedeemCodesCsv() {
  return api.getBlob('/admin/redeem-codes/export')
}

export function fetchAuditLogs(
  limit = 100,
  offset = 0,
  params: { action?: string; admin_id?: number; date_from?: string; date_to?: string } = {},
) {
  // 2026-10-21（P5-1）：审计日志也要能翻页 —— 固定 100 条时，
  // 更早的操作**根本看不到**。
  const q = new URLSearchParams()
  q.set('limit', String(limit))
  q.set('offset', String(offset))
  // 操作/管理员/日期筛选（2026-10-31 新一轮第 18 轮）
  if (params.action) q.set('action', params.action)
  if (params.admin_id) q.set('admin_id', String(params.admin_id))
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  return api.get<AdminAuditLogsResp>(`/admin/audit-logs?${q}`)
}

/**
 * 导出审计日志 CSV（2026-10-30 实质优化第 15 轮）。
 *
 * 合规归档需要全量快照。返回带 UTF-8 BOM 的 CSV Blob。readonly 可导出。
 * 过滤条件与列表一致（2026-10-31 新一轮第 18 轮）。
 */
export function exportAuditLogsCsv(params: {
  action?: string
  admin_id?: number
  date_from?: string
  date_to?: string
} = {}) {
  const q = new URLSearchParams()
  if (params.action) q.set('action', params.action)
  if (params.admin_id) q.set('admin_id', String(params.admin_id))
  if (params.date_from) q.set('date_from', params.date_from)
  if (params.date_to) q.set('date_to', params.date_to)
  return api.getBlob(`/admin/audit-logs/export?${q}`)
}
