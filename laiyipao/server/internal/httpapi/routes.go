package httpapi

import (
	"strconv"
	"strings"
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
	// CORS 的宽松程度取决于环境：dev 放行 `*`（前后端不同端口），
	// prod 默认一条 CORS 头都不发（等价只允许同源）。
	SetDevMode(s.Svc.Cfg.Env != "prod")
	app.Use(recoverPanic())
	// ⚠️ 安全响应头必须在**任何**会写响应体的中间件之前，
	// 否则拿不到它们的输出（与 compress 同理）。
	app.Use(securityHeaders())
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
	app.Use(cors(s.Svc.Cfg.CORSAllowOrigins))

	app.Get("/healthz", s.healthz)
	app.Get("/readyz", s.readyz)
	// 指标端点：仅在 METRICS_ENABLED=true 时注册。默认不挂，理由见
	// config.MetricsEnabled —— 缓存命中率对运维有意义、对公网无意义，
	// 多一个公开端点就多一个面，所以默认朝安全的一侧倒。
	if s.Svc.Cfg.MetricsEnabled {
		app.Get("/metrics", s.metrics)
	}

	v1 := app.Group("/api/v1")

	// ---- 玩家端 ----
	//
	// 路由组织约定：一律用**显式前缀**的 Group，不要用空前缀 Group("")。
	// Fiber v2 默认非严格路由下，空前缀分组会与兄弟分组产生前缀歧义 ——
	// 曾导致 /admin/login 被误套上 requireUser 而无法登录。
	// 每个需要认证的路径前缀挂一次 requireUser，一一对应，不做嵌套。

	// ---- 认证入口 ----
	//
	// 限流挂在**未认证**这一侧：这三个端点都不需要令牌，
	// 所以只能按来源 IP 计。理由与额度见 middleware.go 的 perIPLimiter。
	authAPI := v1.Group("/auth")
	authAPI.Post("/guest", perIPLimiter("guest", guestLoginLimit), s.guestLogin)
	authAPI.Post("/wechat", perIPLimiter("wechat", guestLoginLimit), s.wechatLogin)
	authAPI.Post("/refresh", perIPLimiter("refresh", refreshLimit), s.refresh)
	// 玩家账号登录（用户名 + 密码）。
	//
	// ⚠️ 限流比游客建号严得多：这个端点做 bcrypt（cost 10，单次 50~100ms
	// 纯 CPU），而且**测试环境的共享账号口令与运营超管相同** ——
	// 它同时是超管口令的爆破面。理由与额度见 middleware.go 的 playerLoginLimit。
	authAPI.Post("/login", perIPLimiter("playerlogin", playerLoginLimit), s.playerLogin)

	v1.Get("/config", s.getConfig)
	v1.Get("/leaderboard", s.leaderboard)
	// 公告（玩家侧，2026-10-23）。
	//
	// ⚠️ 这条路由此前**不存在** —— `announcements` 表、后台的增删查全都有，
	// 唯独玩家端没有任何读入口。也就是说运营发的公告**只有运营自己看得见**。
	//
	// 不挂 `requireUser`：公告是公开内容，且首页首屏在登录完成前就要能显示它。
	// 与 `/config`、`/leaderboard` 同级。
	v1.Get("/announcements", s.announcements)

	me := v1.Group("/me", requireUser(s.Svc))
	me.Get("/", s.me)
	// 出战技能配置。必须在服务端持久化 —— 槽位参与回放哈希计算，
	// 放在前端存的话重放方拿不到同一份槽位，I-6 直接失效。
	me.Get("/loadout", s.getLoadout)
	me.Put("/loadout", authRateLimit("loadout", economyWriteLimit), s.saveLoadout)
	//
	// 2026-10-21（P4-1）：装备的**获取与装配**入口。
	//
	// 此前装备的数值有效果（base_armor / base_bonus_pct 会被读进攻方），
	// 但既没有获取途径、也没有装配入口 —— 18 件目录里 16 件玩家拿不到，
	// 而且拿到的 2 件也不能换。于是「装备」在玩法上等于不存在，
	// 而界面上还摆着 18 件装备的目录。
	me.Get("/equipment", s.getEquipment)
	// 整组替换（不是「装一件/卸一件」两个增量端点）——
	// 增量端点会留下「同部位两件都 equipped」的中间态。
	me.Put("/equipment", authRateLimit("equip", economyWriteLimit), s.saveEquipment)
	// 各关历史最好星级（level_stars 的 GREATEST）。选关页的星数/「已通关」
	// 以此为准 —— 服务端没这个读端点的话，客户端的星图永远空白。
	me.Get("/stars", s.myStars)
	// 我的排名（2026-10-23）。
	//
	// `GET /leaderboard` 只回前 N 条，而客户端只能在**这 N 条里**高亮自己。
	// 未进前 N 的玩家在榜上找不到自己，也不知道总共有多少人。
	// 独立端点而不是给公开榜单加可选鉴权 —— 理由见 service.MyRank 的注释。
	me.Get("/rank", s.myRank)
	// 技能升级：消耗金币把一个已拥有技能升一级。
	//
	// ⚠️ 挂在 /me 组下（带 requireUser），因为它要花**玩家自己的**钱。
	// 若挂在 v1 根下，任何人都能替别人花钱/给别人升级 ——
	// 鉴权不是这个端点的细节，是它的全部意义。
	me.Post("/skills/:id/upgrade", s.upgradeSkill)
	// 技能熔炼：把两个已拥有的材料技能熔炼成高阶技能（发放 18 个合成技能）。
	//
	// ⚠️ 与上面的 `/skills/:id/upgrade` 不冲突：那段是 3 段路径且固定带
	// `/upgrade` 后缀，这段是 2 段 —— 纤维按段数匹配，不会被 `:id` 吃掉。
	//
	// 挂在 /me 组下（带 requireUser）：熔炼消耗的是**玩家自己的**材料。
	me.Post("/skills/synthesize", s.synthesizeSkill)

	// 第 150 轮：宝石。第 133 轮的权益扫描要求「宝石要有装备/卸下入口」，
	// 那条只覆盖了 equipped_slot；这里补的是**看得见**与**洗得动**。
	me.Get("/gems", s.listGems)
	me.Post("/gems/:id/wash", s.washGem)

	wallet := v1.Group("/wallet", requireUser(s.Svc))
	wallet.Get("/", s.wallet)

	levels := v1.Group("/levels", requireUser(s.Svc))
	levels.Get("/:id", s.getLevel)

	battle := v1.Group("/battle", requireUser(s.Svc))
	battle.Post("/token", authRateLimit("battletoken", battleTokenLimit), s.startBattle)
	// 结算与验真是**写放大**路径：每次调用都无条件落库一行，
	// 且没有幂等键挡住重复（实测同一场战报重复验真 200 次 → 200 行）。
	// 额度与理由见 middleware.go 的 authRateLimit。
	battle.Post("/settle", authRateLimit("settle", settleLimit), s.settleBattle)
	battle.Post("/verify", authRateLimit("verify", verifyLimit), s.verifyReplay)
	// 2026-10-21（P4-5）：放弃战斗并退回开战时扣掉的资源。
	battle.Post("/abandon", authRateLimit("battleabandon", economyWriteLimit), s.abandonBattle)
	battle.Get("/:id/replay", s.getReplay)

	mastery := v1.Group("/mastery", requireUser(s.Svc))
	mastery.Get("/", s.getMastery)
	mastery.Post("/allocate", s.allocateMastery)
	// 重铸专精：消耗 1 件 mastery_reset 道具，清空已点亮节点。
	// 挂在 /mastery 组（带 requireUser），因为要消耗**玩家自己的**道具。
	mastery.Post("/reset", s.resetMastery)

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
	// 第 141 轮：玩家签到状态（已签天数 / 今日可否签）——签到页初始展示。
	signin.Get("/status", s.signInStatus)

	// 登出：吊销**该用户全部** refresh token。
	//
	// ⚠️ 本轮（审计轮）之前**没有登出端点**，后果是具体的：
	//
	//   - 玩家手机丢了 / 令牌被 XSS 读走 → **没有任何办法**切断会话。
	//     唯一能批量吊销 refresh token 的路径是「封禁」（AdminSetUserStatus），
	//     而那会把正常玩家也一起踢下线。
	//   - access token（默认 2h）在 TTL 内本来就没法单条吊销，
	//     但 refresh token 是**长期**的（默认 30 天）—— 那才是真正要切断的。
	//
	// 为什么吊销**全部**而不是「只吊销本次这一条」：
	// 客户端把令牌存在本地，登出的语义是「这个设备/这个人我不信任了」。
	// 若只吊销一条，被盗的那条依然有效 30 天。
	// 全部吊销的代价最坏是「重登一次」，而那正是登出的含义。
	//
	// 不限流：登出的作用恰恰是「出事时立刻切断」，限流会让受害者
	// 在最需要它的时候被拒。
	me.Post("/logout", s.logout)

	// 兑换码既按 IP 也按身份限：IP 额度挡单点爆破，
	// 身份额度挡「同一个号换 IP 继续试」。
	redeem := v1.Group("/redeem", requireUser(s.Svc))
	redeem.Post("/", perIPLimiter("redeem", redeemLimit), authRateLimit("redeem", perUserRedeem), s.redeem)

	diagnose := v1.Group("/diagnose", requireUser(s.Svc))
	diagnose.Get("/", s.diagnose)

	defenses := v1.Group("/defenses", requireUser(s.Svc))
	defenses.Get("/", s.listDefenses)
	defenses.Post("/save", authRateLimit("savedefense", economyWriteLimit), s.saveDefense)
	defenses.Post("/:id/challenge", authRateLimit("challenge", economyWriteLimit), s.challengeDefense)

	// ---- 管理端 ----
	adminLogin := v1.Group("/admin")
	// ⚠️ 这是全站**最贵的一次未认证请求**：bcrypt cost 10，
	// 单次比对约 50~100ms 纯 CPU。无限流时，未认证攻击者打满并发
	// 就能让正常运营完全登不进后台，同时顺便无限次口令喷洒。
	// 额度 5/分钟对个位数运营者有极大余量，对喷洒器是 7200 次/天的天花板。
	adminLogin.Post("/login", perIPLimiter("adminlogin", adminLoginLimit), s.adminLogin) // 公开

	adm := v1.Group("/admin", requireAdmin(s.Svc))
	// 第 155 轮：后台登出（吊销全部会话）。刻意不挂 requireWritable ——
	// 会话生命周期不是数据写；readonly_guard_test.go 里的 AST 守卫
	// 有且仅有这一条带注释的豁免。
	adm.Post("/logout", s.adminLogout)
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
	// 2026-10-21（P5-7）：玩家详情（只读）。
	adm.Get("/users/:id", s.adminUserDetail)
	adm.Post("/users/:id/unban", requireWritable(), s.adminUnbanUser)
	adm.Post("/users/:id/grant", requireWritable(), s.adminGrantUser)
	adm.Get("/battles", s.adminBattles)
	adm.Get("/battles/:id", s.adminBattleDetail)
	adm.Post("/battles/:id/verify", requireWritable(), s.adminVerifyBattle)
	// 结算重算审计（R1）。两个端点成对：列表看单条，stats 看分布与闸门状态。
	// ⚠️ /stats 必须挂在 /recomputes 之后且**不是** `/recomputes/:id` ——
	//   后者一旦存在就会把 /stats 当成 id 匹配掉。
	adm.Get("/recomputes", s.adminRecomputes)
	adm.Get("/recomputes/stats", s.adminRecomputeStats)
	// 2026-10-21（P3-3）：档位可读可改。
	adm.Get("/recomputes/mode", s.adminRecomputeMode)
	// 切档是**写操作**，而且是「关掉反作弊校验」那种写 ——
	// readonly 账号绝不能做（它会 403，而不是被静默地切成功）。
	adm.Put("/recomputes/mode", requireWritable(), s.adminSetRecomputeMode)
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
	// ⚠️ 开关打开时**连游客建号都不许**。
	//
	// 只在客户端挡（弹登录页）是不够的：浏览器里缓存着旧 bundle 的
	// 测试者仍会调到这个端点，于是「每个浏览器一个号、测试数据散在各处」
	// 这个老问题会在**看起来已经修好**之后继续存在。
	//
	// 403 + 明确的 code=login_required：客户端据此跳登录页，
	// 而不是把「登录失败」显示成一句看不懂的报错。
	if s.Svc.Cfg.PlayerLoginRequired {
		return fail(c, fiber.StatusForbidden, "login_required",
			"本环境要求账号登录，请使用账号密码登录")
	}
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

// playerLogin 玩家账号登录（用户名 + 密码）。
//
// 与游客登录的差别只在凭据：这里走 `users.username` + `users.password_hash`，
// 而游客登录走 `users.guest_token`（一枚客户端持有的随机串）。
// 两者拿到的是同一种令牌（kind=user），权限面完全一样。
func (s *Server) playerLogin(c *fiber.Ctx) error {
	var body struct {
		Username string `json:"username"`
		Password string `json:"password"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	tp, err := s.Svc.Login(c.Context(), body.Username, body.Password)
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
	// 第 142 轮：max_stage 是「玩家进度」的服务端权威来源。客户端此前只在
	// 本会话结算后才知道进度，老玩家重进 App 恒从 0 开始。/me 带上它，
	// 客户端 refreshProfile 即可恢复真实解锁进度。
	maxStage, err := s.Svc.LoadMaxStage(c.Context(), userID)
	if err != nil {
		return failErr(c, err)
	}
	// 2026-10-21（P3-1）：评分**可能失败**（读不到专精点数）。
	// 此前它把错误吞成 0 点，于是返回 200 + 一个少了专精加成的评分 ——
	// 一个看起来合理但**错的**数字。失败就报失败。
	rating, err := s.Svc.ComputeRatingFor(c.Context(), userID, build)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{
		"user_id":      userID,
		"build":        build,
		"build_rating": rating,
		"power":        s.Svc.ComputePowerFor(build),
		"max_stage":    maxStage,
	})
}

// logout 吊销该账号的全部 refresh token，并**轮换**游客凭证。
//
// ⚠️ 客户端必须把响应里的 `guest_token` 存下来：旧凭证已作废，
//
//	不存的话下一次冷启动会建出一个**新号**，玩家的进度看起来凭空消失。
func (s *Server) logout(c *fiber.Ctx) error {
	res, err := s.Svc.Logout(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(res)
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

// synthesizeSkill 把两个已拥有的材料技能熔炼成高阶技能（发放 18 个合成技能）。
//
// 请求体：{"a": <技能id>, "b": <技能id>}。顺序无所谓（service 用 LEAST/GREATEST
// 抹平，(a,b) 与 (b,a) 命中同一配方）。
// 失败一律走 `failErr`（未配方/未达等级/材料不全/已拥有产物 → 400，不泄漏配方细节）。
func (s *Server) synthesizeSkill(c *fiber.Ctx) error {
	var in struct {
		A int64 `json:"a"`
		B int64 `json:"b"`
	}
	if err := c.BodyParser(&in); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_body", "熔炼请求体需为 {\"a\":int,\"b\":int}")
	}
	output, err := s.Svc.SynthesizeSkills(c.Context(), userIDFrom(c), in.A, in.B)
	if err != nil {
		return failErr(c, err)
	}
	// 熔炼会删材料并改槽位，build 快照立即失效 —— 一并回给前端，
	// 避免它拿着旧 build 又发一次 /me 看到中间态。
	build, err := s.Svc.LoadBuildSnapshot(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"output": output, "build": build})
}

func (s *Server) wallet(c *fiber.Ctx) error {
	w, err := s.Svc.LoadWallet(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(w)
}

// getConfig 下发全量配置，支持按 version 做条件请求。
//
// `?if_version=<客户端已有的 version>`：内容没变时只回 304（零载荷）。
//
// ## 为什么用查询参数而不是 `If-None-Match` / ETag
//
// 两条都是真实原因，不是风格偏好：
//
//  1. **自定义请求头会触发 CORS 预检**。H5 端的 cors 中间件目前只放行
//     `Content-Type,Authorization`；加一个头就得同步改预检白名单，
//     而忘改的症状是「整批请求发不出去」，很难和网络故障区分。
//  2. **ETag 会让浏览器自作主张缓存**。响应一旦带上 ETag，
//     H5 的 XHR 会拿到浏览器缓存里的旧响应体 ——
//     里面的 `server_time` 是上一次拉取时的时间，
//     而 `estimateServerNowMs()` 会把它当成「服务端当前时间」。
//     不设 ETag 则两端行为一致：每次都真的走到服务端，
//     由 `if_version` 显式决定要不要载荷。
//
// ⚠️ 用 304 而不是 204：304 在 HTTP 语义里就是「条件 GET 未修改」，
// 客户端一眼能看懂，也和浏览器/中间件的既有约定相容。
func (s *Server) getConfig(c *fiber.Ctx) error {
	cfg := s.Svc.LoadGameConfig()
	if v := strings.TrimSpace(c.Query("if_version")); v != "" && v == cfg.Version {
		// 不写响应体。fiber 的 304 会自带 Content-Length: 0。
		return c.Status(fiber.StatusNotModified).Send(nil)
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

// listGems 列出玩家持有的全部宝石实例。
//
// 这是宝石从「库里躺着没人看得见」变成「玩家能看到、能洗」的那一步 ——
// 此前 `user_gems` 只有两处内部读取（计入攻击者），没有任何面向玩家的读取面。
func (s *Server) listGems(c *fiber.Ctx) error {
	gems, err := s.Svc.ListGems(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"gems": gems})
}

// washGem 消耗 1 枚洗练券，重 roll 这颗宝石的词条（品质不变）。
func (s *Server) washGem(c *fiber.Ctx) error {
	id, err := strconv.ParseInt(c.Params("id"), 10, 64)
	if err != nil || id < 1 {
		return fail(c, fiber.StatusBadRequest, "bad_input", "宝石 ID 非法")
	}
	g, err := s.Svc.WashGem(c.Context(), userIDFrom(c), id)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"gem": g})
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

// abandonBattle 放弃一场未结算的战斗，退回体力（与复活币）。
//
// ⚠️ 2026-10-21（P4-5）：此前**没有放弃路径** ——
// 玩家中途退出，体力已经扣了，却什么都没有发生。
func (s *Server) abandonBattle(c *fiber.Ctx) error {
	var body struct {
		TokenID int64 `json:"token_id"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	if body.TokenID <= 0 {
		return fail(c, fiber.StatusBadRequest, "bad_input", "token_id 非法")
	}
	refunded, err := s.Svc.AbandonBattle(c.Context(), userIDFrom(c), body.TokenID)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"refunded_energy": refunded, "abandoned": true})
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

// announcements 下发**已发布**的公告（玩家侧）。
//
// limit 走 `queryInt` 而不是 `c.Query("limit", "5")`：
// 后者在 `?limit=` 时会吞掉空串返回默认值，而 `?limit=abc` 会静默变成 0
// —— 与 `leaderboard` 里那段注释是同一条道理，两处必须一致。
// 越界由 service 层夹住（它是展示型端点，传错一个数不该让首屏整块消失）。
func (s *Server) announcements(c *fiber.Ctx) error {
	limit, err := queryInt(c, "limit", 0)
	if err != nil {
		return failErr(c, err)
	}
	items, err := s.Svc.ListAnnouncements(c.Context(), limit)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items})
}

// myRank 返回「我在榜上的位置」。
//
// kind 的校验与 `leaderboard` 同口径（先 `Has` 再取 raw 再校验），
// 不能写成 `c.Query("type","power")` —— 那会把 `?type=` 吞成默认值，
// 而「显式传空」与「没传」是两种输入（见 leaderboard 里那段注释）。
func (s *Server) myRank(c *fiber.Ctx) error {
	kind := "power"
	if c.Context().QueryArgs().Has("type") {
		kind = c.Query("type")
	}
	if !leaderboardKinds[kind] {
		return fail(c, fiber.StatusBadRequest, "bad_input",
			"type 只能是 power / stage / efficiency")
	}
	out, err := s.Svc.MyRank(c.Context(), userIDFrom(c), kind)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(out)
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
	return c.JSON(s.masteryState(c, pts, nodes, extra))
}

// masteryState 汇总专精状态。
//
// ⚠️ 2026-10-21（P3-7）：`GET /mastery` 与 `POST /mastery/reset` 的响应
// 形状此前**不一致** —— reset 少了 `families`（专精系定义）。
//
// 后果：客户端重铸专精后如果靠这个响应刷新专精树，就必须**再拉一次**
// `/mastery`；而更糟的是「两个端点形状一样」这个假设一旦被默认，
// 下一个字段加在一边就会静默不同步。
//
// 现在两个端点**共用这一个函数** —— 形状不一致在结构上就不可能发生。
func (s *Server) masteryState(c *fiber.Ctx, pts int, nodes []int, extra int64) fiber.Map {
	return fiber.Map{
		"points":      pts,
		"nodes":       nodes,
		"layer_limit": domain.PerLayerPickLimit,
		"base_slots":  domain.BaseSkillSlots,
		"total_slots": domain.BaseSkillSlots + int(extra),
		"families":    domain.AllMasteryFamilies(),
	}
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

// resetMastery 重铸专精：消耗 1 件 mastery_reset 道具清空已点亮节点。
//
// 回执回显**清掉后的真实状态**（复用 getMastery 的字段形状）+ 本次清掉节点数，
// 让客户端拿到的不是「操作成功」四个字，而是可直接刷 UI 的完整状态。
func (s *Server) resetMastery(c *fiber.Ctx) error {
	cleared, err := s.Svc.ResetMastery(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	pts, nodes, err := s.Svc.LoadMastery(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	extra, err := s.Svc.ExtraSlots(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	// 形状与 GET /mastery **完全一致**（共用 masteryState），只是多一个
	// reset_cleared 说明「这次清掉了几个节点」。
	state := s.masteryState(c, pts, nodes, extra)
	state["reset_cleared"] = cleared
	return c.JSON(state)
}

// getEquipment 返回装备目录 + 玩家的拥有/装配状态。
func (s *Server) getEquipment(c *fiber.Ctx) error {
	items, err := s.Svc.ListEquipment(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items})
}

// saveEquipment 整组替换玩家的装配集合。
func (s *Server) saveEquipment(c *fiber.Ctx) error {
	var body struct {
		// EquipmentIDs 是玩家**想要装备**的完整集合（不是增量）。
		EquipmentIDs []int `json:"equipment_ids"`
	}
	if err := c.BodyParser(&body); err != nil {
		return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
	}
	items, err := s.Svc.SetEquippedEquipment(c.Context(), userIDFrom(c), body.EquipmentIDs)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items})
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

// signInStatus 回读玩家签到状态（第 141 轮）：已签天数与今日是否可签。
func (s *Server) signInStatus(c *fiber.Ctx) error {
	st, err := s.Svc.SignInStatus(c.Context(), userIDFrom(c))
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(st)
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
