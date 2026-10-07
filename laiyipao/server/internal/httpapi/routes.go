package httpapi

import (
	"strconv"
	"time"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/compress"

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
	// ⚠️ 压缩必须放在**所有产生响应的中间件之前**，否则拿不到它们的输出。
	//
	// 加它的理由是实测数字，不是「JSON 该压缩」这种常识：
	//
	//   /api/v1/config 未压缩 **167,124 字节**
	//   gzip(BestSpeed)      31,322 字节（18.7%）
	//   gzip(Default)        23,964 字节（14.3%）   ← 省掉 85.7%
	//
	// 逐字段看，`levels` 一个字段就占 123,717 字节（74%），
	// 而且它是 100 关高度重复的结构 —— 压缩率天然就高。
	//
	// 客户端是**每次冷启动都拉一次**（`loadConfig` 只有内存内缓存，
	// 见 store/game.ts），所以这 163KB 是**每次启动**的固定流量。
	// 在移动网络下这是实打实的钱。
	//
	// 之前注释里写「压缩后约 60KB」—— 那个数字是错的（实测 163KB），
	// 而且「压缩后」指的是压缩前就已存在的 JSON 体积，不是 gzip 之后。
	// 已按实测数字改写。
	app.Use(compress.New())
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
	// 各关历史最好星级（level_stars 的 GREATEST）。选关页的星数/「已通关」
	// 以此为准 —— 服务端没这个读端点的话，客户端的星图永远空白。
	me.Get("/stars", s.myStars)
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
	// 第 135 轮：七日签到奖励表的权威来源。客户端预览改读它，不再本地硬编码。
	signin.Get("/calendar", s.signinCalendar)

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
	// 第 127 轮：写操作挂 requireWritable —— readonly 账号 403，
	// 读端点（上方 GET）对 readonly 保持放行。
	adm.Put("/levels/:id", requireWritable(), s.adminUpdateLevel)
	adm.Post("/levels/regenerate", requireWritable(), s.adminRegenerateLevels)
	adm.Get("/skills", s.adminSkills)
	adm.Put("/skills/:id", requireWritable(), s.adminUpdateSkill)
	adm.Get("/equipment", s.adminEquipment)
	adm.Get("/reactions", s.adminReactions)
	adm.Get("/users", s.adminUsers)
	adm.Post("/users/:id/ban", requireWritable(), s.adminBanUser)
	adm.Post("/users/:id/unban", requireWritable(), s.adminUnbanUser)
	adm.Post("/users/:id/grant", requireWritable(), s.adminGrantUser)
	adm.Get("/battles", s.adminBattles)
	adm.Get("/battles/:id", s.adminBattleDetail)
	adm.Post("/battles/:id/verify", requireWritable(), s.adminVerifyBattle)
	adm.Get("/defenses", s.adminDefenses)
	adm.Get("/economy", s.adminEconomy)
	adm.Put("/shop/:id", requireWritable(), s.adminUpdateShop)
	adm.Get("/announcements", s.adminAnnouncements)
	adm.Post("/announcements", requireWritable(), s.adminCreateAnnouncement)
	adm.Get("/redeem-codes", s.adminRedeemCodes)
	adm.Post("/redeem-codes", requireWritable(), s.adminCreateRedeemCode)
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
		"power":        s.Svc.ComputePowerFor(build),
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

// myStars 返回玩家各关历史最好星级，形状 { "stars": { "<关卡ID>": 星数 } }。
// 从未结算的关卡不出现在 map 里 —— 客户端按缺省 0 处理，而不是服务端补 0。
func (s *Server) myStars(c *fiber.Ctx) error {
	stars, err := s.Svc.LevelStars(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"stars": stars})
}
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
	// LoadGameConfig 纯内存构造、恒不失败，签名没有 error 返回值
	return c.JSON(s.Svc.LoadGameConfig())
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
	// ⚠️ 第 75 轮：把 caller 传进去做归属校验。
	//
	// 原来只有 battleID，于是任何注册用户都能遍历连续 BIGSERIAL
	// 读到别人的完整构筑快照。详见 service.GetReplay 的注释。
	info, err := s.Svc.GetReplay(c.Context(), userIDFrom(c), id)
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

// leaderboardKinds 是全部合法榜单类型 —— 与 `service.Leaderboard` 的
// switch 分支、以及小程序 `pages/rank/rank.vue` 的三个 tab **三方一致**。
//
// ⚠️ 第 104 轮新增。原来 `type` 走 `default:` 分支，
// 于是任何未知 type 都会**静默返回战力榜**，
// 而响应里 `"type"` 仍然**回显那个未知的值**。
//
// 后果：客户端按 type 决定标题与说明
// （`rank.vue` 的 `curDesc` + `formatScore`），
// 于是会在「日榜」这种标题下显示战力榜的数据，
// 且客户端**无法察觉**（响应确认了它请求的类型）。
//
// 这与 `user_id=abc` 是同一类：**把非法输入静默重解释成另一个合法值**。
//
// 三方一致由 `TestLeaderboardKindIsValidated` 的对拍守住。
var leaderboardKinds = map[string]bool{"power": true, "stage": true, "efficiency": true}

func (s *Server) leaderboard(c *fiber.Ctx) error {
	//
	// ⚠️⚠️ **不能**写成 `c.Query("type", "power")` 再校验。
	//
	// 实测（探针）：Fiber 的 `c.Query(key, default)` 在**值为空**时
	// 也返回 default —— 它看的是值，不是键在不在：
	//
	//	?type=        → c.Query("type","power") == "power"   ← 空串被吞成默认值
	//	（不传）      → c.Query("type","power") == "power"
	//	?other=1      → c.Query("type","power") == "power"
	//
	// 三种输入三种含义（没传 / 显式传空 / 传了别的键），
	// 而这个 API **把它们压成同一个值**。
	//
	// 我第一版就是这么写的，于是 `?type=` 绕过了校验返回 200 + 战力榜。
	// 只有 `QueryArgs().Has(key)` 能区分「键在不在」——
	// 实测 `?type=` → Has=true，`(不传)` → Has=false。
	//
	// 所以顺序必须是：**先 Has，再取不带默认值的 raw**，最后校验。
	// 这与 `queryInt` 是同一条道理，两处必须一致。
	kind := "power"
	if c.Context().QueryArgs().Has("type") {
		kind = c.Query("type") // 显式传了 —— 哪怕是空串，也要走下面的校验
	}
	if !leaderboardKinds[kind] {
		return fail(c, fiber.StatusBadRequest, "bad_input",
			"type 只能是 power / stage / efficiency")
	}
	limit, err := queryInt(c, "limit", 50)
	if err != nil {
		return failErr(c, err)
	}
	items, err := s.Svc.Leaderboard(c.Context(), kind, limit)
	if err != nil {
		return failErr(c, err)
	}
	// 回显**规范化之后**的 kind，而不是原样回显请求值 ——
	// 响应里的 type 必须与实际返回的数据是同一件事。
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
	//
	// ⚠️ 第 106 轮：原来这里不校验 scope，
	// 于是 `LoadTasks` 的 `WHERE t.scope = $3` 匹配不到任何行 →
	// **200 + 空列表**，而响应里 `"scope"` 仍**回显那个未知的值**。
	//
	// 实测：`?scope=daliy`（拼错）与 `?scope=Daily`（大小写）→ 都是
	// 200 + 0 条 + 回显原值。
	//
	// 为什么这比「返回了错数据」更糟：玩家看到的是
	// 「**今天没有任务**」—— 一个**看起来完全合理**的答案。
	// 而且它按玩家、按天出现，很可能永远不会被发现。
	// （leaderboard 的 type 是同一个毛病，第 104 轮已修。）
	//
	// 顺序与 `leaderboard` 一致：**先 Has，再取不带默认值的 raw，最后校验**。
	// 理由见那里的注释 —— `c.Query(key, default)` 会把空值吞成默认值。
	scope := "daily"
	if c.Context().QueryArgs().Has("scope") {
		scope = c.Query("scope")
	}
	if !service.ValidTaskScope(scope) {
		return fail(c, fiber.StatusBadRequest, "bad_input",
			"scope 只能是 daily / weekly / achievement")
	}
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

// signinCalendar 下发服务端权威的七日签到奖励表（第 135 轮）。
func (s *Server) signinCalendar(c *fiber.Ctx) error {
	days, err := s.Svc.SignInCalendar(c.Context())
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"days": days})
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
	levelID, err := queryInt(c, "level_id", 1)
	if err != nil {
		return failErr(c, err)
	}
	failed, err := queryInt(c, "failed_times", 1)
	if err != nil {
		return failErr(c, err)
	}
	res, err := s.Svc.Diagnose(c.Context(), int64(userIDFrom(c)), levelID, failed)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(res)
}
