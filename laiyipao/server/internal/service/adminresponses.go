package service

import (
	"github.com/laiyipao/server/internal/domain"
)

// ─────────────────────────────────────────────────────────────────────────────
// 后台 API 的**响应信封**（2026-10-21 新增）
//
// # 为什么信封也要有类型
//
// 视图行（AdminLevelView 等）有类型还不够：handler 还要在外面包一层
// `{"items": ..., "total": ...}`。那层包装此前是 `fiber.Map{...}` ——
// 同样是手写的，同样没有事实源。
//
// 前端那层包装此前也是手写的：
//
//	api.get<{ items: AdminBattle[]; total: number }>(...)
//
// 于是「行」的字段名有事实源了，「信封」的字段名还是没有 ——
// 而前端真正写的是**信封**。
//
// 现在信封也是 Go 类型、由 handler 直接返回、也进生成表：
// 前端写 `api.get<AdminBattlesResp>(...)` 时，它描述的就是服务端**真的**返回的东西。
//
// # 命名
//
//	Admin<资源>Resp  = 列表型（items + total）
//	Admin<资源>Resp  = 单体型（<资源> 字段）
//
// ─────────────────────────────────────────────────────────────────────────────

// AdminLevelsResp 是 `GET /admin/levels` 的响应。
type AdminLevelsResp struct {
	Items []AdminLevelView `json:"items"`
	Total int64            `json:"total"`
}

// AdminLevelResp 是 `PUT /admin/levels/:id` 的响应。
type AdminLevelResp struct {
	Level AdminLevelView `json:"level"`
}

// AdminWavesResp 是 `GET /admin/levels/:id/waves` 的响应。
type AdminWavesResp struct {
	Waves []AdminLevelWaveView `json:"waves"`
}

// AdminRegenerateResp 是 `POST /admin/levels/regenerate` 的响应。
type AdminRegenerateResp struct {
	Generated int `json:"generated"`
}

// AdminSkillResp 是 `PUT /admin/skills/:id` 的响应。
type AdminSkillResp struct {
	Skill AdminSkillView `json:"skill"`
}

// AdminUsersResp 是 `GET /admin/users` 的响应。
type AdminUsersResp struct {
	Items []AdminUserView `json:"items"`
	Total int64           `json:"total"`
}

// AdminUserResp 是 `POST /admin/users/:id/ban|unban` 的响应。
type AdminUserResp struct {
	User AdminUserView `json:"user"`
}

// AdminGrantResp 是 `POST /admin/users/:id/grant` 的响应。
//
// Wallet 是「货币 → 变更后的余额」。
type AdminGrantResp struct {
	Wallet map[string]int64 `json:"wallet"`
}

// AdminBattlesResp 是 `GET /admin/battles` 的响应。
type AdminBattlesResp struct {
	Items []AdminBattleView `json:"items"`
	Total int64             `json:"total"`
}

// AdminRecomputesResp 是 `GET /admin/recomputes` 的响应。
type AdminRecomputesResp struct {
	Items []RecomputeView `json:"items"`
	Total int64           `json:"total"`
}

// AdminDefensesResp 是 `GET /admin/defenses` 的响应。
type AdminDefensesResp struct {
	Items []AdminDefenseView `json:"items"`
	Total int64              `json:"total"`
}

// AdminShopItemResp 是 `PUT /admin/shop/:id` 的响应。
type AdminShopItemResp struct {
	Item AdminShopUpdateResp `json:"item"`
}

// AdminAnnouncementsResp 是 `GET /admin/announcements` 的响应。
type AdminAnnouncementsResp struct {
	Items []AdminAnnouncementView `json:"items"`
	// Total 是**全量行数**（不是当前页条数）—— 前端拿它算页数。
	Total int64 `json:"total"`
}

// AdminAnnouncementResp 是 `POST /admin/announcements` 的响应。
type AdminAnnouncementResp struct {
	Item AdminAnnouncementView `json:"item"`
}

// AdminRedeemCodesResp 是 `GET /admin/redeem-codes` 的响应。
type AdminRedeemCodesResp struct {
	Items []AdminRedeemCodeView `json:"items"`
	// Total 是**全量行数**（不是当前页条数）—— 前端拿它算页数。
	Total int64 `json:"total"`
}

// AdminRedeemCodeResp 是 `POST /admin/redeem-codes` 的响应。
type AdminRedeemCodeResp struct {
	Item AdminRedeemCodeView `json:"item"`
}

// AdminAuditLogsResp 是 `GET /admin/audit-logs` 的响应。
type AdminAuditLogsResp struct {
	Items []AdminAuditLogView `json:"items"`
	// Total 是**全量行数**（不是当前页条数）—— 前端拿它算页数。
	Total int64 `json:"total"`
}

// AdminReactionsResp 是 `GET /admin/reactions` 的响应。
type AdminReactionsResp struct {
	Reactions []domain.ReactionSpec `json:"reactions"`
}

// AdminMeResp 是 `GET /admin/me` 的响应。
type AdminMeResp struct {
	Admin AdminUser `json:"admin"`
}

// AdminLogoutResp 是 `POST /admin/logout` 的响应。
type AdminLogoutResp struct {
	RevokedSessions int `json:"revoked_sessions"`
}

// AdminLoginResp 是 `POST /admin/login` 的响应。
type AdminLoginResp struct {
	AccessToken string    `json:"access_token"`
	ExpiresAt   string    `json:"expires_at"`
	Admin       AdminUser `json:"admin"`
}
