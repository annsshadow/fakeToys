package httpapi

import (
	"strconv"
	"time"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/service"
)

// Server 持有路由所需的依赖。
type Server struct {
	Svc *service.Service
}

// New 构造 Server。
func New(svc *service.Service) *Server { return &Server{Svc: svc} }

// Register 挂载全部路由。
func (s *Server) Register(app *fiber.App) {
	app.Use(recoverPanic())
	app.Use(requestLogger())
	app.Use(cors())

	app.Get("/healthz", s.healthz)
	app.Get("/readyz", s.readyz)

	v1 := app.Group("/api/v1")

	// ---- 玩家端 ----
	//
	// 路由组织约定：一律用**显式前缀**的 Group，不要用空前缀 Group("")。
	// Fiber v2 默认非严格路由下，空前缀分组会与兄弟分组产生前缀歧义 ——
	// 曾导致 /admin/login 被误套上 requireUser 而无法登录。
	// 每个需要认证的路径前缀挂一次 requireUser，一一对应，不做嵌套。

	authAPI := v1.Group("/auth")
	authAPI.Post("/guest", s.guestLogin)
	authAPI.Post("/wechat", s.wechatLogin)
	authAPI.Post("/refresh", s.refresh)

	v1.Get("/config", s.getConfig)
	v1.Get("/leaderboard", s.leaderboard)

	me := v1.Group("/me", requireUser(s.Svc))
	me.Get("/", s.me)
	// 出战技能配置。必须在服务端持久化 —— 槽位参与回放哈希计算，
	// 放在前端存的话重放方拿不到同一份槽位，I-6 直接失效。
	me.Get("/loadout", s.getLoadout)
	me.Put("/loadout", s.saveLoadout)
	// 技能升级：消耗金币把一个已拥有技能升一级。
	//
	// ⚠️ 挂在 /me 组下（带 requireUser），因为它要花**玩家自己的**钱。
	// 若挂在 v1 根下，任何人都能替别人花钱/给别人升级 ——
	// 鉴权不是这个端点的细节，是它的全部意义。
	me.Post("/skills/:id/upgrade", s.upgradeSkill)

	wallet := v1.Group("/wallet", requireUser(s.Svc))
	wallet.Get("/", s.wallet)

	levels := v1.Group("/levels", requireUser(s.Svc))
	levels.Get("/:id", s.getLevel)

	battle := v1.Group("/battle", requireUser(s.Svc))
	battle.Post("/token", s.startBattle)
	battle.Post("/settle", s.settleBattle)
	battle.Post("/verify", s.verifyReplay)
	battle.Get("/:id/replay", s.getReplay)

	mastery := v1.Group("/mastery", requireUser(s.Svc))
	mastery.Get("/", s.getMastery)
	mastery.Post("/allocate", s.allocateMastery)

	tasks := v1.Group("/tasks", requireUser(s.Svc))
	tasks.Get("/", s.tasks)
	tasks.Post("/:id/claim", s.claimTask)

	shop := v1.Group("/shop", requireUser(s.Svc))
	shop.Get("/", s.shop)
	shop.Post("/:id/buy", s.buy)

	signin := v1.Group("/signin", requireUser(s.Svc))
	signin.Post("/", s.signIn)

	redeem := v1.Group("/redeem", requireUser(s.Svc))
	redeem.Post("/", s.redeem)

	diagnose := v1.Group("/diagnose", requireUser(s.Svc))
	diagnose.Get("/", s.diagnose)

	defenses := v1.Group("/defenses", requireUser(s.Svc))
	defenses.Get("/", s.listDefenses)
	defenses.Post("/save", s.saveDefense)
	defenses.Post("/:id/challenge", s.challengeDefense)

	// ---- 管理端 ----
	adminLogin := v1.Group("/admin")
	adminLogin.Post("/login", s.adminLogin) // 公开

	adm := v1.Group("/admin", requireAdmin(s.Svc))
	adm.Get("/me", s.adminMe)
	adm.Get("/dashboard", s.adminDashboard)
	adm.Get("/levels", s.adminLevels)
	adm.Get("/levels/:id/waves", s.adminLevelWaves)
	adm.Put("/levels/:id", s.adminUpdateLevel)
	adm.Post("/levels/regenerate", s.adminRegenerateLevels)
	adm.Get("/skills", s.adminSkills)
	adm.Put("/skills/:id", s.adminUpdateSkill)
	adm.Get("/equipment", s.adminEquipment)
	adm.Get("/users", s.adminUsers)
	adm.Post("/users/:id/ban", s.adminBanUser)
	adm.Post("/users/:id/unban", s.adminUnbanUser)
	adm.Post("/users/:id/grant", s.adminGrantUser)
	adm.Get("/battles", s.adminBattles)
	adm.Get("/battles/:id", s.adminBattleDetail)
	adm.Get("/defenses", s.adminDefenses)
	adm.Get("/economy", s.adminEconomy)
	adm.Put("/shop/:id", s.adminUpdateShop)
	adm.Get("/announcements", s.adminAnnouncements)
	adm.Post("/announcements", s.adminCreateAnnouncement)
	adm.Get("/redeem-codes", s.adminRedeemCodes)
	adm.Post("/redeem-codes", s.adminCreateRedeemCode)
	adm.Get("/audit-logs", s.adminAuditLogs)
}

// --- 基础 ---

func (s *Server) healthz(c *fiber.Ctx) error {
	return c.JSON(fiber.Map{"ok": true, "time": time.Now()})
}

func (s *Server) readyz(c *fiber.Ctx) error {
	if err := s.Svc.DB.Ping(c.Context()); err != nil {
		return fail(c, fiber.StatusServiceUnavailable, "db_unavailable", "数据库不可用")
	}
	return c.JSON(fiber.Map{"ok": true, "db": "up"})
}

// --- 玩家端处理器 ---

func (s *Server) guestLogin(c *fiber.Ctx) error {
	var body struct {
		GuestToken string `json:"guest_token"`
		Nickname   string `json:"nickname"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	tp, err := s.Svc.GuestLogin(c.Context(), body.GuestToken, body.Nickname)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(tp)
}

func (s *Server) wechatLogin(c *fiber.Ctx) error {
	var body struct {
		Code string `json:"code"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	tp, err := s.Svc.WechatLogin(c.Context(), body.Code)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(tp)
}

func (s *Server) refresh(c *fiber.Ctx) error {
	var body struct {
		RefreshToken string `json:"refresh_token"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	tp, err := s.Svc.Refresh(c.Context(), body.RefreshToken)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(tp)
}

func (s *Server) me(c *fiber.Ctx) error {
	userID := userIDFrom(c)
	build, err := s.Svc.LoadBuildSnapshot(c.Context(), userID)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{
		"user_id":      userID,
		"build":        build,
		"build_rating": s.Svc.ComputeRatingFor(userID, build),
		"power":        s.Svc.ComputePowerFor(userID, build),
	})
}

// getLoadout 返回当前出战槽位。
func (s *Server) getLoadout(c *fiber.Ctx) error {
	ids, err := s.Svc.Loadout(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"skill_ids": ids})
}

// saveLoadout 保存出战技能配置。
func (s *Server) saveLoadout(c *fiber.Ctx) error {
	var in service.SaveLoadoutInput
	if err := c.BodyParser(&in); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	ids, err := s.Svc.SaveLoadout(c.Context(), userIDFrom(c), in)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"skill_ids": ids})
}

// upgradeSkill 把 :id 对应的技能升一级，返回新等级。
//
// 失败一律走 `failErr`，由它把 service 的哨兵错误翻成合适的 HTTP 码
// （余额不足 / 未拥有 / 已满级 → 400，不泄漏「该技能存在但你没拥有」）。
func (s *Server) upgradeSkill(c *fiber.Ctx) error {
	id, err := strconv.Atoi(c.Params("id"))
	if err != nil || id <= 0 {
		return fail(c, fiber.StatusBadRequest, "bad_id", "技能 id 必须是正整数")
	}
	level, err := s.Svc.UpgradeSkill(c.Context(), userIDFrom(c), int64(id))
	if err != nil {
		return failErr(c, err)
	}
	// 余额一并回给客户端：升级必然花钱，让前端再发一次 /wallet 才能刷新余额
	// 是个可避免的竞态（玩家连点两次会看到中间态）。
	w, err := s.Svc.LoadWallet(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"skill_id": id, "level": level, "wallet": w})
}

func (s *Server) wallet(c *fiber.Ctx) error {
	w, err := s.Svc.LoadWallet(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(w)
}

func (s *Server) getConfig(c *fiber.Ctx) error {
	cfg, err := s.Svc.LoadGameConfig(c.Context())
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(cfg)
}

func (s *Server) getLevel(c *fiber.Ctx) error {
	id, err := strconv.Atoi(c.Params("id"))
	if err != nil || id < 1 || id > domain.TotalLevels {
		return fail(c, fiber.StatusBadRequest, "bad_input", "关卡 ID 非法")
	}
	return c.JSON(domain.GenerateLevel(id))
}

func (s *Server) startBattle(c *fiber.Ctx) error {
	var body struct {
		LevelID int `json:"level_id"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	resp, err := s.Svc.StartBattle(c.Context(), userIDFrom(c), body.LevelID)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(resp)
}

func (s *Server) settleBattle(c *fiber.Ctx) error {
	var body struct {
		TokenID int64 `json:"token_id"`
		domain.SettleInput
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	if body.TokenID == 0 {
		return fail(c, fiber.StatusBadRequest, "bad_input", "缺少 token_id")
	}
	resp, err := s.Svc.SettleBattle(c.Context(), userIDFrom(c), body.TokenID, body.SettleInput)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(resp)
}

func (s *Server) getReplay(c *fiber.Ctx) error {
	id, err := strconv.ParseInt(c.Params("id"), 10, 64)
	if err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_input", "battle id 非法")
	}
	info, err := s.Svc.GetReplay(c.Context(), id)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(info)
}

func (s *Server) verifyReplay(c *fiber.Ctx) error {
	var body struct {
		BattleID   int64  `json:"battle_id"`
		ReplayHash string `json:"replay_hash"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	res, err := s.Svc.VerifyReplay(c.Context(), userIDFrom(c), body.BattleID, body.ReplayHash)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(res)
}

func (s *Server) leaderboard(c *fiber.Ctx) error {
	kind := c.Query("type", "power")
	limit, _ := strconv.Atoi(c.Query("limit", "50"))
	items, err := s.Svc.Leaderboard(c.Context(), kind, limit)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"type": kind, "items": items})
}

func (s *Server) getMastery(c *fiber.Ctx) error {
	pts, nodes, err := s.Svc.LoadMastery(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	// ⚠️ `total_slots` 必须下发，而不只是 `base_slots`。
	//
	// 专精树第 3 层槽 0 是「额外插槽」，每点一个 +1 槽（`MasteryEffect.ExtraSlots`）。
	// 那 8 个节点此前**完全惰性** —— 客户端根本不读 `base_slots`，
	// 而是用编译期常量 `ACTIVE_SLOTS = 4`，
	// 于是玩家花点数点出来的槽位在战斗里不存在。
	//
	// 同时这里补上服务端侧的**槽位校验入口**（loadSkillsAndSlots 用它）：
	// 「客户端能装几个技能」必须由服务端说了算。
	extra, err := s.Svc.ExtraSlots(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{
		"points":      pts,
		"nodes":       nodes,
		"layer_limit": domain.PerLayerPickLimit,
		"base_slots":  domain.BaseSkillSlots,
		"total_slots": domain.BaseSkillSlots + extra,
		"families":    domain.AllMasteryFamilies(),
	})
}

func (s *Server) allocateMastery(c *fiber.Ctx) error {
	var body struct {
		NodeID int `json:"node_id"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	if err := s.Svc.AllocateMastery(c.Context(), userIDFrom(c), body.NodeID); err != nil {
		return failErr(c, err)
	}
	return s.getMastery(c)
}

func (s *Server) tasks(c *fiber.Ctx) error {
	scope := c.Query("scope", "daily")
	items, err := s.Svc.LoadTasks(c.Context(), userIDFrom(c), scope)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"scope": scope, "items": items})
}

func (s *Server) claimTask(c *fiber.Ctx) error {
	id, err := strconv.Atoi(c.Params("id"))
	if err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_input", "task id 非法")
	}
	reward, err := s.Svc.ClaimTask(c.Context(), userIDFrom(c), id)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"reward": reward})
}

func (s *Server) signIn(c *fiber.Ctx) error {
	res, err := s.Svc.SignIn(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(res)
}

func (s *Server) shop(c *fiber.Ctx) error {
	items, err := s.Svc.LoadShop(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items})
}

func (s *Server) buy(c *fiber.Ctx) error {
	id, err := strconv.ParseInt(c.Params("id"), 10, 64)
	if err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_input", "item id 非法")
	}
	granted, err := s.Svc.Buy(c.Context(), userIDFrom(c), id)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"granted": granted})
}

func (s *Server) redeem(c *fiber.Ctx) error {
	var body struct {
		Code string `json:"code"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	reward, err := s.Svc.Redeem(c.Context(), userIDFrom(c), body.Code)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"reward": reward})
}

func (s *Server) diagnose(c *fiber.Ctx) error {
	levelID, _ := strconv.Atoi(c.Query("level_id", "1"))
	failed, _ := strconv.Atoi(c.Query("failed_times", "1"))
	res, err := s.Svc.Diagnose(c.Context(), int64(userIDFrom(c)), levelID, failed)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(res)
}
