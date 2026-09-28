package httpapi

import (
	"context"
	"strconv"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/service"
)

// --- 防线（I-5） ---

func (s *Server) listDefenses(c *fiber.Ctx) error {
	mine, candidates, err := s.Svc.ListDefenses(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{
		"mine":          mine,
		"candidates":    candidates,
		"attempt_limit": service.DefenseAttemptLimit,
		"stolen_limit":  service.DefenseStolenLimit,
	})
}

func (s *Server) saveDefense(c *fiber.Ctx) error {
	var in service.SaveDefenseInput
	if err := c.BodyParser(&in); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	view, err := s.Svc.SaveDefense(c.Context(), userIDFrom(c), in)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"defense": view})
}

func (s *Server) challengeDefense(c *fiber.Ctx) error {
	id, err := strconv.ParseInt(c.Params("id"), 10, 64)
	if err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_input", "defense id 非法")
	}
	var in service.ChallengeInput
	if err := c.BodyParser(&in); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	res, err := s.Svc.ChallengeDefense(c.Context(), userIDFrom(c), id, in)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(res)
}

// --- 管理端 ---

func (s *Server) adminLogin(c *fiber.Ctx) error {
	var body struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	tp, err := s.Svc.AdminLogin(c.Context(), body.Username, body.Password)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(tp)
}

func (s *Server) adminMe(c *fiber.Ctx) error {
	a, err := s.Svc.AdminMe(c.Context(), adminIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"admin": a})
}

func (s *Server) adminDashboard(c *fiber.Ctx) error {
	d, err := s.Svc.AdminDashboard(c.Context())
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(d)
}

func (s *Server) adminLevels(c *fiber.Ctx) error {
	items, total, err := s.Svc.AdminListLevels(c.Context())
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items, "total": total})
}

func (s *Server) adminLevelWaves(c *fiber.Ctx) error {
	id, err := strconv.Atoi(c.Params("id"))
	if err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_input", "level id 非法")
	}
	waves, err := s.Svc.AdminLevelWaves(c.Context(), id)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"waves": waves})
}

func (s *Server) adminUpdateLevel(c *fiber.Ctx) error {
	id, err := strconv.Atoi(c.Params("id"))
	if err != nil || id < 1 || id > 100 {
		return fail(c, fiber.StatusBadRequest, "bad_input", "level id 非法")
	}
	var patch map[string]any
	if err := c.BodyParser(&patch); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	lvl, err := s.Svc.AdminUpdateLevel(c.Context(), id, patch)
	if err != nil {
		return failErr(c, err)
	}
	s.Svc.Audit(c.Context(), adminIDFrom(c), "update_level", strconv.Itoa(id), patch)
	return c.JSON(fiber.Map{"level": lvl})
}

func (s *Server) adminRegenerateLevels(c *fiber.Ctx) error {
	n, err := s.Svc.AdminRegenerateLevels(c.Context())
	if err != nil {
		return failErr(c, err)
	}
	s.Svc.Audit(c.Context(), adminIDFrom(c), "regenerate_levels", "", fiber.Map{"count": n})
	return c.JSON(fiber.Map{"generated": n})
}

func (s *Server) adminSkills(c *fiber.Ctx) error {
	data, err := s.Svc.AdminSkills(c.Context())
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(data)
}

func (s *Server) adminUpdateSkill(c *fiber.Ctx) error {
	id, err := strconv.Atoi(c.Params("id"))
	if err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_input", "skill id 非法")
	}
	var patch map[string]any
	if err := c.BodyParser(&patch); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	if err := s.Svc.AdminUpdateSkill(c.Context(), id, patch); err != nil {
		return failErr(c, err)
	}
	s.Svc.Audit(c.Context(), adminIDFrom(c), "update_skill", strconv.Itoa(id), patch)
	return s.adminSkills(c)
}

func (s *Server) adminEquipment(c *fiber.Ctx) error {
	data, err := s.Svc.AdminEquipment(c.Context())
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(data)
}

// adminReactions 返回反应表（key + 中文名 + 各档数值）。
//
// ## 为什么要单独开一个端点，而不是让后台去拉 /api/v1/config
//
// `ReactionSpec.Name` 的注释本来就写着「反应的中文名，后台系统/运营看板要用」——
// 也就是说**服务端本来就打算把名字下发给运营**。
//
// 而后台的看板（DashboardView）之前是自己硬编码了一张
// `REACTION_LABEL: Record<string,string>`。
// 那张表当前**恰好是全的**（7 个反应常量对 7 个条目），所以看不出问题；
// 但它是**纯重复**：名字的真源在 `domain/elements.go` 的常量旁边，
// 复制一份就多一个「两处可能不同步」的面。
//
// 有人加第 8 个反应而忘了改那张表时，后果不是报错，而是
// 图表上悄悄出现一个英文 key（`REACTION_LABEL[x] || x` 的兜底）。
//
// 为什么不复用 `/api/v1/config`：那个响应实测 **167,124 字节**
// （其中 `levels` 占 74%），而且每次请求都要 `GenerateAllLevels()` 重算 100 关。
// 为了拿 7 个名字去生成 100 关关卡数据，不划算。
//
// 返回**对象**而不是裸数组，理由有两个：
//  1. 符合 admin 其余端点的约定（`{items,total}` / `{equipment,gems,skins}`），
//     以后要加分页或元数据也不必改响应形状；
//  2. 裸数组在多数 HTTP 客户端的解码助手（`map[string]any`）里会直接失败 ——
//     这是实测踩到的，不是理论顾虑。
func (s *Server) adminReactions(c *fiber.Ctx) error {
	return c.JSON(fiber.Map{"reactions": domain.AllReactionSpecs()})
}

func (s *Server) adminUsers(c *fiber.Ctx) error {
	limit, _ := strconv.Atoi(c.Query("limit", "50"))
	offset, _ := strconv.Atoi(c.Query("offset", "0"))
	items, total, err := s.Svc.AdminListUsers(c.Context(), c.Query("keyword", ""), limit, offset)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items, "total": total})
}

func (s *Server) adminBanUser(c *fiber.Ctx) error {
	id, err := strconv.ParseInt(c.Params("id"), 10, 64)
	if err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_input", "user id 非法")
	}
	var body struct {
		Reason string `json:"reason"`
	}
	_ = c.BodyParser(&body)
	if body.Reason == "" {
		body.Reason = "违反用户协议"
	}
	u, err := s.Svc.AdminSetUserStatus(c.Context(), id, true, body.Reason)
	if err != nil {
		return failErr(c, err)
	}
	s.Svc.Audit(c.Context(), adminIDFrom(c), "ban_user", strconv.FormatInt(id, 10), body)
	return c.JSON(fiber.Map{"user": u})
}

func (s *Server) adminUnbanUser(c *fiber.Ctx) error {
	id, err := strconv.ParseInt(c.Params("id"), 10, 64)
	if err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_input", "user id 非法")
	}
	u, err := s.Svc.AdminSetUserStatus(c.Context(), id, false, "")
	if err != nil {
		return failErr(c, err)
	}
	s.Svc.Audit(c.Context(), adminIDFrom(c), "unban_user", strconv.FormatInt(id, 10), nil)
	return c.JSON(fiber.Map{"user": u})
}

func (s *Server) adminGrantUser(c *fiber.Ctx) error {
	id, err := strconv.ParseInt(c.Params("id"), 10, 64)
	if err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_input", "user id 非法")
	}
	var body struct {
		Currency string `json:"currency"`
		Amount   int64  `json:"amount"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	wallet, err := s.Svc.AdminGrantCurrency(c.Context(), id, body.Currency, body.Amount)
	if err != nil {
		return failErr(c, err)
	}
	s.Svc.Audit(c.Context(), adminIDFrom(c), "grant_currency",
		strconv.FormatInt(id, 10), fiber.Map{"currency": body.Currency, "amount": body.Amount})
	return c.JSON(fiber.Map{"wallet": wallet})
}

func (s *Server) adminBattles(c *fiber.Ctx) error {
	limit, _ := strconv.Atoi(c.Query("limit", "50"))
	userID, _ := strconv.ParseInt(c.Query("user_id", "0"), 10, 64)
	levelID, _ := strconv.Atoi(c.Query("level_id", "0"))
	items, total, err := s.Svc.AdminListBattles(c.Context(), userID, levelID, limit)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items, "total": total})
}

func (s *Server) adminBattleDetail(c *fiber.Ctx) error {
	id, err := strconv.ParseInt(c.Params("id"), 10, 64)
	if err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_input", "battle id 非法")
	}
	info, err := s.Svc.GetReplay(c.Context(), id)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"battle": info})
}

func (s *Server) adminDefenses(c *fiber.Ctx) error {
	limit, _ := strconv.Atoi(c.Query("limit", "50"))
	items, total, err := s.Svc.AdminListDefenses(c.Context(), limit)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items, "total": total})
}

func (s *Server) adminEconomy(c *fiber.Ctx) error {
	data, err := s.Svc.AdminEconomy(c.Context())
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(data)
}

func (s *Server) adminUpdateShop(c *fiber.Ctx) error {
	id, err := strconv.ParseInt(c.Params("id"), 10, 64)
	if err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_input", "item id 非法")
	}
	var patch map[string]any
	if err := c.BodyParser(&patch); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	item, err := s.Svc.AdminUpdateShopItem(c.Context(), id, patch)
	if err != nil {
		return failErr(c, err)
	}
	s.Svc.Audit(c.Context(), adminIDFrom(c), "update_shop_item",
		strconv.FormatInt(id, 10), patch)
	return c.JSON(fiber.Map{"item": item})
}

func (s *Server) adminAnnouncements(c *fiber.Ctx) error {
	items, err := s.Svc.AdminListAnnouncements(c.Context())
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items})
}

func (s *Server) adminCreateAnnouncement(c *fiber.Ctx) error {
	var body struct {
		Title     string `json:"title"`
		Body      string `json:"body"`
		Published bool   `json:"published"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	if body.Title == "" || body.Body == "" {
		return fail(c, fiber.StatusBadRequest, "bad_input", "标题与正文不能为空")
	}
	item, err := s.Svc.AdminCreateAnnouncement(c.Context(), body.Title, body.Body, body.Published)
	if err != nil {
		return failErr(c, err)
	}
	s.Svc.Audit(c.Context(), adminIDFrom(c), "create_announcement", body.Title, nil)
	return c.JSON(fiber.Map{"item": item})
}

func (s *Server) adminRedeemCodes(c *fiber.Ctx) error {
	items, err := s.Svc.AdminListRedeemCodes(c.Context())
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items})
}

func (s *Server) adminCreateRedeemCode(c *fiber.Ctx) error {
	var body struct {
		Code      string         `json:"code"`
		Reward    map[string]int `json:"reward"`
		MaxUses   int            `json:"max_uses"`
		ExpiresAt string         `json:"expires_at"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	if body.Code == "" || len(body.Reward) == 0 {
		return fail(c, fiber.StatusBadRequest, "bad_input", "兑换码与奖励不能为空")
	}
	if body.MaxUses <= 0 {
		body.MaxUses = 1
	}
	item, err := s.Svc.AdminCreateRedeemCode(c.Context(), body.Code, body.Reward, body.MaxUses)
	if err != nil {
		return failErr(c, err)
	}
	s.Svc.Audit(c.Context(), adminIDFrom(c), "create_redeem_code", body.Code, body)
	return c.JSON(fiber.Map{"item": item})
}

func (s *Server) adminAuditLogs(c *fiber.Ctx) error {
	limit, _ := strconv.Atoi(c.Query("limit", "100"))
	items, err := s.Svc.AdminListAuditLogs(c.Context(), limit)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items})
}

// ctxUnused 保留下划线避免未使用告警（中间件已封装 Context 传递）。
var _ = context.Background
