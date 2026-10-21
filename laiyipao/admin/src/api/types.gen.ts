// 本文件是**生成物**，请勿手工编辑。
//
// 来源：laiyipao/server/internal/service/adminviews.go 与 internal/domain 的响应类型，
// 由 laiyipao/server/cmd/tygen 生成。
//
// 重新生成：
//     cd laiyipao/server && go run ./cmd/tygen
//
// 守卫：server/internal/tygen/tygen_test.go 会把生成结果与本文件逐字节比对，
// 改了 Go 侧的类型而不重新生成 → 测试变红。
//
// 为什么要有它：后台此前 15 个端点的响应是手写的 map[string]any，
// 形状由裸 SQL 决定，前后端两侧都没有事实源 —— 于是前端字段名拼错
// 不报错（"列恒空"），测试 fixture 又按前端形状写（每条都测得绿）。
// 现在两端从同一份定义派生：改一个字段名，两端同时红。

export interface AdminAnnouncementResp {
  item: AdminAnnouncementView
}

export interface AdminAnnouncementView {
  id: number
  title: string
  body: string
  published: boolean
  published_at: string | null
  created_at: string | null
}

export interface AdminAnnouncementsResp {
  items: AdminAnnouncementView[]
}

export interface AdminAuditLogView {
  id: number | null
  admin_id: number | null
  username: string
  action: string
  target: string
  detail: unknown
  created_at: string
}

export interface AdminAuditLogsResp {
  items: AdminAuditLogView[]
}

export interface AdminBattleDetailResp {
  battle: AdminBattleDetailView
}

export interface AdminBattleDetailView {
  id: number
  user_id: number
  nickname: string
  level_id: number
  result: string
  stars: number
  score: number
  kills: number
  leaked: number
  hp_left: number
  wave_reached: number
  duration_ms: number
  shots: number
  hits: number
  reactions: number
  heat_max: number
  elements_used: { [key: string]: number }
  reactions_used: { [key: string]: number }
  terrain_used: string[]
  replay_hash: string
  seed: string
  power_after: number
  created_at: string
}

export interface AdminBattleView {
  id: number
  user_id: number
  nickname: string
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
  total_enemies: number
  verify_checked: number
  verify_mismatched: number
}

export interface AdminBattlesResp {
  items: AdminBattleView[]
  total: number
}

export interface AdminDefenseView {
  id: number
  owner_id: number
  owner_name: string
  name: string
  power: number
  element_coverage: number
  mastery_done: number
  wins: number
  losses: number
  shielded_until: string | null
  updated_at: string
  snapshot_hash: string
}

export interface AdminDefensesResp {
  items: AdminDefenseView[]
  total: number
}

export interface AdminEconomyFlowView {
  reason: string
  currency: string
  count: number
  delta: number
}

export interface AdminEconomyResp {
  flows: AdminEconomyFlowView[]
  shop: AdminShopItemView[]
}

export interface AdminEquipmentResp {
  equipment: AdminEquipmentView[]
  gems: SeedGem[]
  qualifications: QualityInfo[]
  skins: SeedSkin[]
  enemies: SeedEnemy[]
  reactions: ReactionSpec[]
  mastery: MasteryFamily[]
  rating_weights: RatingWeights
}

export interface AdminEquipmentView {
  id: number
  code: string
  name: string
  slot: string
  tier: number
  element: string
  descr: string
  base_armor: number
  base_bonus_pct: number
  unlock_level: number
}

export interface AdminGrantResp {
  wallet: { [key: string]: number }
}

export interface AdminLevelResp {
  level: AdminLevelView
}

export interface AdminLevelView {
  id: number
  chapter: number
  name: string
  seed: string
  base_hp: number
  wave_count: number
  difficulty: number
  energy_cost: number
  star_targets: number[]
  terrain_config: TerrainPlacement[]
  is_boss: boolean
  enabled: boolean
  attempts: number
  clears: number
  clear_rate: number
  avg_wave: number
  player_authority: string
  applied_to_players: boolean
}

export interface AdminLevelWaveView {
  wave_index: number
  spawns: Spawn[]
}

export interface AdminLevelsResp {
  items: AdminLevelView[]
  total: number
}

export interface AdminLoginResp {
  access_token: string
  expires_at: string
  admin: { id: number; username: string; role: string }
}

export interface AdminLogoutResp {
  revoked_sessions: number
}

export interface AdminMeResp {
  admin: { id: number; username: string; role: string }
}

export interface AdminReactionsResp {
  reactions: ReactionSpec[]
}

export interface AdminRecomputesResp {
  items: RecomputeView[]
  total: number
}

export interface AdminRedeemCodeResp {
  item: AdminRedeemCodeView
}

export interface AdminRedeemCodeView {
  id: number
  code: string
  reward: { [key: string]: number }
  max_uses: number
  used_count: number
  expires_at: string | null
  enabled: boolean
}

export interface AdminRedeemCodesResp {
  items: AdminRedeemCodeView[]
}

export interface AdminRegenerateResp {
  generated: number
}

export interface AdminShopItemResp {
  item: AdminShopUpdateResp
}

export interface AdminShopItemView {
  id: number
  code: string
  name: string
  category: string
  price: { [key: string]: number }
  payload: { [key: string]: number }
  limit_per_day: number
  sort_order: number
  enabled: boolean
}

export interface AdminShopUpdateResp {
  id: number
  updated: string[]
}

export interface AdminSkillResp {
  skill: AdminSkillView
}

export interface AdminSkillView {
  id: number
  code: string
  name: string
  family: string
  element: string
  kind: string
  descr: string
  base_damage: number
  heat_cost: number
  cooldown_ms: number
  pierce: number
  aoe_radius: number
  apply_element: string
  apply_stacks: number
  projectile_speed: number
  chain: number
  unlock_level: number
}

export interface AdminSkillsResp {
  items: AdminSkillView[]
  recipes: SeedRecipe[]
}

export interface AdminUserResp {
  user: AdminUserView
}

export interface AdminUserView {
  id: number
  nickname: string
  is_guest: boolean
  status: number
  max_stage: number
  power: number
  coin: number
  gem: number
  created_at: string
  last_login_at: string
  verify_checked: number
  verify_mismatched: number
}

export interface AdminUsersResp {
  items: AdminUserView[]
  total: number
}

export interface AdminWavesResp {
  waves: AdminLevelWaveView[]
}

export interface ChapterInfo {
  id: number
  name: string
  start_level: number
  end_level: number
  terrain_kind: string
  boss_enemy_id: number
}

export interface Dashboard {
  users: { total: number; guests: number; wechat: number; banned: number; new_today: number }
  battles: { total: number; today: number; wins: number; avg_duration_ms: number }
  progression: { avg_max_stage: number; avg_power: number; stage_100_clears: number }
  economy: { coin_in: number; coin_out: number; gem_in: number; gem_out: number; orders_paid: number; orders_pending: number }
  verification: { checked: number; matched: number; mismatched: number }
  reaction_usage: ReactionCount[]
  daily_active: DauPoint[]
  stage_funnel: StageFunnel[]
}

export interface DauPoint {
  date: string
  dau: number
}

export interface GameConfig {
  version: string
  require_login: boolean
  levels: LevelSummary[]
  enemies: SeedEnemy[]
  skills: SeedSkill[]
  composite_skills: SeedSkill[]
  recipes: SeedRecipe[]
  equipment: SeedEquipment[]
  gems: SeedGem[]
  gem_qualities: QualityInfo[]
  skins: SeedSkin[]
  mastery_families: MasteryFamily[]
  reactions: ReactionSpec[]
  chapters: ChapterInfo[]
  rating_weights: RatingWeights
  score_rules: ScoreRules
  skill_rules: SkillRules
  server_time: string
}

export interface LevelSummary {
  id: number
  chapter: number
  name: string
  seed: number
  base_hp: number
  wave_count: number
  difficulty: number
  energy_cost: number
  star_targets: number[]
  terrain: TerrainPlacement[]
  is_boss: boolean
  element_cap: number
  armor_permille: number
  max_reaction_tier: number
  max_score: number
}

export interface MasteryFamily {
  Family: string
  Name: string
  Nodes: []
}

export interface QualityInfo {
  name: string
  affix_count: number
  mult: number
}

export interface RatingWeights {
  element_coverage: number
  reaction_coverage: number
  mastery_done: number
  equipment_synergy: number
  mechanic_depth: number
}

export interface ReactionCount {
  reaction: string
  count: number
}

export interface ReactionSpec {
  key: string
  name: string
  base_coef: number
  attack_weight_pct: number
  status_duration_ms: number
  aoe_radius: number
  spread_stacks: number
  dispel_shield: boolean
  amplify_pct: number
  armor_shred_permille: number
  knockback: number
  descr: string
}

export interface RecomputeEngineCounters {
  verified: number
  mismatch: number
  cannot_run: number
  skipped: number
}

export interface RecomputeLevelStat {
  level_id: number
  total: number
  mismatched: number
  rate_permille: number
  max_user_mismatches: number
}

export interface RecomputeSide {
  score: number
  kills: number
  duration_ms: number
  hash: string
}

export interface RecomputeSummary {
  mode: string
  configured_mode: string
  demoted: boolean
  demoted_reason: string
  since: string
  breaker_rate_permille: number
  breaker_min_samples: number
  totals: RecomputeTotals
  levels: RecomputeLevelStat[]
  engine: { verified: number; mismatch: number; cannot_run: number; skipped: number; settled_total: number; verified_permille: number; status: string }
}

export interface RecomputeTotals {
  total: number
  mismatched: number
  rejected: number
  rate_permille: number
}

export interface RecomputeView {
  id: number
  battle_id: number
  user_id: number
  nickname: string
  level_id: number
  reported: RecomputeSide
  recomputed: RecomputeSide | null
  mismatched_fields: string[]
  mode: string
  rejected: boolean
  engine_version: string
  created_at: string
}

export interface ScoreRules {
  per_damage_unit: number
  on_kill_normal: number
  on_kill_boss: number
  star_target_ratio: number[]
  score_full_at_sec: number
}

export interface SeedEnemy {
  id: number
  code: string
  name: string
  category: string
  hp: number
  speed: number
  armor: number
  shield_hp: number
  attack: number
  attack_range: number
  attack_interval: number
  fly_height: number
  burrow: boolean
  is_boss: boolean
  resist: { [key: string]: number }
  descr: string
}

export interface SeedEquipment {
  id: number
  code: string
  name: string
  slot: string
  tier: number
  element: string
  descr: string
  base_armor: number
  base_bonus_pct: number
  unlock_level: number
}

export interface SeedGem {
  id: number
  code: string
  name: string
  attr: string
  descr: string
}

export interface SeedRecipe {
  output: number
  a: number
  b: number
  out_tier: number
}

export interface SeedSkill {
  id: number
  code: string
  name: string
  family: string
  element: string
  kind: string
  descr: string
  base_damage: number
  heat_cost: number
  cooldown_ms: number
  pierce: number
  aoe_radius: number
  apply_element: string
  apply_stacks: number
  projectile_speed: number
  chain: number
  unlock_level: number
}

export interface SeedSkin {
  id: number
  code: string
  name: string
  rarity: string
  passive: string
  descr: string
}

export interface SkillRules {
  max_level: number
  coef_permille: number
  base_cost: number
}

export interface Spawn {
  enemy_id: number
  count: number
  interval: number
  delay: number
}

export interface StageFunnel {
  level_id: number
  attempts: number
  clears: number
}

export interface TerrainPlacement {
  kind: string
  x: number
  y: number
  param: number
}

