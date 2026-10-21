// Package apitypes is the registry of API response types that are mirrored
// into TypeScript.
//
// # 为什么要有注册表
//
// `internal/tygen` 需要知道「哪些 Go 类型要变成 TS interface、TS 里叫什么」。
// 把这份清单放在**类型旁边**（而不是藏在生成器里）有两个好处：
//
//  1. 加一个响应类型时，**必须**在这里登记 —— 忘了登记，它就不会出现在
//     TS 侧，于是前端会缺一个类型（但不会报错，因为前端可以自己编）；
//  2. 守卫测试（tygen_test.go）能核对「注册表里的每个类型都真的出现在
//     生成物里」，两侧不会悄悄漂移。
//
// # 命名约定
//
// TS 名 = Go 类型名（`AdminLevelView` → `AdminLevelView`）。
// 刻意**不加前缀**（比如 `Admin`）：Go 侧的类型名已经带了语义前缀，
// 再加一层会让 TS 侧出现 `AdminAdminLevelView` 这种噪音。
package apitypes

import (
	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/service"
	"github.com/laiyipao/server/internal/tygen"
)

// AdminRegistry 是后台 API 的响应类型注册表。
var AdminRegistry = tygen.Registry{
	// ── 关卡 ──
	{Name: "AdminLevelView", Go: service.AdminLevelView{}},
	{Name: "AdminLevelWaveView", Go: service.AdminLevelWaveView{}},
	// ── 技能 ──
	{Name: "AdminSkillView", Go: service.AdminSkillView{}},
	{Name: "AdminSkillsResp", Go: service.AdminSkillsResp{}},
	// ── 装备 / 宝石 / 皮肤 / 内容表 ──
	{Name: "AdminEquipmentView", Go: service.AdminEquipmentView{}},
	{Name: "AdminEquipmentResp", Go: service.AdminEquipmentResp{}},
	// ── 防线 ──
	{Name: "AdminDefenseView", Go: service.AdminDefenseView{}},
	// ── 经济 / 商城 ──
	{Name: "AdminEconomyFlowView", Go: service.AdminEconomyFlowView{}},
	{Name: "AdminShopItemView", Go: service.AdminShopItemView{}},
	{Name: "AdminEconomyResp", Go: service.AdminEconomyResp{}},
	{Name: "AdminShopUpdateResp", Go: service.AdminShopUpdateResp{}},
	// ── 公告 / 兑换码 / 审计 ──
	{Name: "AdminAnnouncementView", Go: service.AdminAnnouncementView{}},
	{Name: "AdminRedeemCodeView", Go: service.AdminRedeemCodeView{}},
	{Name: "AdminAuditLogView", Go: service.AdminAuditLogView{}},
	// ── 响应信封（handler 直接返回它们，前端写的就是这层）──
	{Name: "AdminLoginResp", Go: service.AdminLoginResp{}},
	{Name: "AdminMeResp", Go: service.AdminMeResp{}},
	{Name: "AdminLogoutResp", Go: service.AdminLogoutResp{}},
	{Name: "AdminLevelsResp", Go: service.AdminLevelsResp{}},
	{Name: "AdminLevelResp", Go: service.AdminLevelResp{}},
	{Name: "AdminWavesResp", Go: service.AdminWavesResp{}},
	{Name: "AdminRegenerateResp", Go: service.AdminRegenerateResp{}},
	{Name: "AdminSkillResp", Go: service.AdminSkillResp{}},
	{Name: "AdminUsersResp", Go: service.AdminUsersResp{}},
	{Name: "AdminUserResp", Go: service.AdminUserResp{}},
	{Name: "AdminUserDetailResp", Go: service.AdminUserDetailResp{}},
	{Name: "AdminVerificationView", Go: service.AdminVerificationView{}},
	{Name: "AdminGrantResp", Go: service.AdminGrantResp{}},
	{Name: "AdminBattlesResp", Go: service.AdminBattlesResp{}},
	{Name: "AdminRecomputesResp", Go: service.AdminRecomputesResp{}},
	{Name: "AdminDefensesResp", Go: service.AdminDefensesResp{}},
	{Name: "AdminShopItemResp", Go: service.AdminShopItemResp{}},
	{Name: "AdminAnnouncementsResp", Go: service.AdminAnnouncementsResp{}},
	{Name: "AdminAnnouncementResp", Go: service.AdminAnnouncementResp{}},
	{Name: "AdminRedeemCodesResp", Go: service.AdminRedeemCodesResp{}},
	{Name: "AdminRedeemCodeResp", Go: service.AdminRedeemCodeResp{}},
	{Name: "AdminAuditLogsResp", Go: service.AdminAuditLogsResp{}},
	{Name: "AdminReactionsResp", Go: service.AdminReactionsResp{}},
	// ── 共享的内容表类型（`/config` 与后台用的是同一份）──
	{Name: "SeedEnemy", Go: domain.SeedEnemy{}},
	{Name: "SeedSkill", Go: domain.SeedSkill{}},
	{Name: "SeedRecipe", Go: domain.SeedRecipe{}},
	{Name: "SeedEquipment", Go: domain.SeedEquipment{}},
	{Name: "SeedGem", Go: domain.SeedGem{}},
	{Name: "SeedSkin", Go: domain.SeedSkin{}},
	{Name: "QualityInfo", Go: service.QualityInfo{}},
	{Name: "ReactionSpec", Go: domain.ReactionSpec{}},
	{Name: "MasteryFamily", Go: domain.MasteryFamily{}},
	{Name: "RatingWeights", Go: domain.RatingWeights{}},
	{Name: "TerrainPlacement", Go: domain.TerrainPlacement{}},
	{Name: "Spawn", Go: domain.Spawn{}},
	{Name: "LevelSummary", Go: domain.LevelSummary{}},
	{Name: "ChapterInfo", Go: service.ChapterInfo{}},
	{Name: "ScoreRules", Go: domain.ScoreRules{}},
	{Name: "SkillRules", Go: domain.SkillRules{}},
	{Name: "GameConfig", Go: service.GameConfig{}},
	{Name: "Dashboard", Go: service.Dashboard{}},
	{Name: "ReactionCount", Go: service.ReactionCount{}},
	{Name: "DauPoint", Go: service.DauPoint{}},
	{Name: "StageFunnel", Go: service.StageFunnel{}},
	{Name: "AdminUserView", Go: service.AdminUserView{}},
	{Name: "AdminBattleView", Go: service.AdminBattleView{}},
	{Name: "AdminBattleDetailView", Go: service.AdminBattleDetailView{}},
	{Name: "AdminBattleDetailResp", Go: service.AdminBattleDetailResp{}},
	{Name: "RecomputeSide", Go: service.RecomputeSide{}},
	{Name: "RecomputeView", Go: service.RecomputeView{}},
	{Name: "RecomputeLevelStat", Go: service.RecomputeLevelStat{}},
	{Name: "RecomputeSummary", Go: service.RecomputeSummary{}},
	{Name: "RecomputeTotals", Go: service.RecomputeTotals{}},
	{Name: "RecomputeEngineCounters", Go: service.RecomputeEngineCounters{}},
}

// Header 是生成物顶部的说明。
const Header = `// 本文件是**生成物**，请勿手工编辑。
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
`
