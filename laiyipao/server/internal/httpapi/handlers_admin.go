package httpapi

import (
	"bytes"
	"context"
	"strconv"
	"strings"
	"time"

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
	// 第 111 轮：筛选参数必须传到 service（修前这里一个都不收，UI 的
	// 章节下拉与关键字搜索发了也白发，永远回全量）。
	chapter, err := queryInt(c, "chapter", 0)
	if err != nil {
		return failErr(c, err)
	}
	keyword := c.Query("keyword")
	items, total, err := s.Svc.AdminListLevels(c.Context(), chapter, keyword)
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
	limit, err := queryInt(c, "limit", 50)
	if err != nil {
		return failErr(c, err)
	}
	offset, err := queryInt(c, "offset", 0)
	if err != nil {
		return failErr(c, err)
	}
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
	//
	// ⚠️ 第 105 轮：原来这里是 `_ = c.BodyParser(&body)` —— 解析错误被丢弃。
	//
	// 实测：`POST /admin/users/5/ban` 带一个**截断的** JSON（`{bad`）
	// → 返回 **200**，用户**真的被封禁**，
	// 而且 `ban_reason` 落库成「违反用户协议」—— 一个运营从未提交过的理由。
	//
	// 为什么这条比第 104 轮那两条查询参数更重：
	//
	//  1. **它是破坏性动作。** 前者是返回错数据，这里是改用户状态。
	//     （好在有 unban，但那需要有人意识到出错了。）
	//  2. **审计轨迹被污染。** 事后看 `admin_audit_logs`，
	//     「违反用户协议」看起来像是运营深思熟虑后的判断，
	//     而实际上那只是一个解析失败的默认值。
	//  3. **触发条件极易达到** —— 运营脚本 / 代理截断 / 复制粘贴漏字符，
	//     都会让一个手滑变成一次误封。
	//
	// # 为什么先判 `len(c.Body())`
	//
	// 空 body 是**合法**调用（「就封他，理由按默认的」），
	// 而 `BodyParser` 对空 body 返回 EOF 类错误。
	// 所以「没 body」与「body 坏了」必须分开 ——
	// 这与第 104 轮 `queryInt` 的「键不存在」vs「值为空」同源。
	if len(bytes.TrimSpace(c.Body())) > 0 {
		if err := c.BodyParser(&body); err != nil {
			return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
		}
	}
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
	// 第 109 轮：符号进审计 —— 负数是回收，事件名必须与正数发放可区分，
	// 否则审计里「发放」和「回收」长得一模一样。
	evt := "grant_currency"
	if body.Amount < 0 {
		evt = "revoke_currency"
	}
	s.Svc.Audit(c.Context(), adminIDFrom(c), evt,
		strconv.FormatInt(id, 10), fiber.Map{"currency": body.Currency, "amount": body.Amount})
	return c.JSON(fiber.Map{"wallet": wallet})
}

func (s *Server) adminBattles(c *fiber.Ctx) error {
	limit, err := queryInt(c, "limit", 50)
	if err != nil {
		return failErr(c, err)
	}
	//
	// ⚠️ 第 104 轮：user_id / level_id 的默认值 0 是「**不过滤**」的哨兵值
	// （`AdminListBattles` 的 `($1 = 0 OR br.user_id = $1)`）。
	// 原来这两行是 `strconv.ParseInt(..., 10, 64)` 并**丢弃错误**，
	// 于是 `?user_id=abc` → 0 → 过滤恒真 → **返回所有用户的战报 + 200**。
	//
	// 运营查疑似作弊玩家时打错 id，看到的是别人的战报，
	// 于是得出「这个人没有异常战报」的结论。
	userID, err := queryInt(c, "user_id", 0)
	if err != nil {
		return failErr(c, err)
	}
	levelID, err := queryInt(c, "level_id", 0)
	if err != nil {
		return failErr(c, err)
	}
	// 只看验真不匹配的战报。
	//
	// 上一轮把「每用户验真统计」接进了 /admin/users，运营知道**谁**可疑；
	// 这一步是为了能直接回答「**哪一场**对局对不上」——
	// 否则还得手工按 user_id 查战报再交叉比对 replay_verifications。
	onlyMismatched := c.Query("only_mismatched") == "1" || c.Query("only_mismatched") == "true"
	items, total, err := s.Svc.AdminListBattles(c.Context(), int64(userID), levelID, limit, onlyMismatched)
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
	// 运营侧**不加**归属过滤 —— 查任意玩家的战报本来就是它的职责
	// （见 service.AdminGetReplay 的注释）。
	info, err := s.Svc.AdminGetReplay(c.Context(), id)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"battle": info})
}

// adminVerifyBattle 运营侧发起验真。
//
// ## 补的是哪一段
//
// 验真机制此前**只有玩家入口**。而这产生了一个结构性后果：
// **能采取行动的人（运营），恰恰是唯一不能发起验真的人。**
// 于是「验真一致率」这个数字的分母，永远只由玩家的自愿行为决定。
//
// 现在运营可以拿着重算出来的 hash 发起比对，结果与玩家侧**落在同一张表**，
// 前两轮加的「用户列表 / 战报列表的验真列」才会真的亮起来。
//
// ⚠️ 本端点**不重算**哈希。它比对的是「运营提交的」与「结算时记录的」。
// 一个**原始作弊客户端**可以报一个相同的 hash 让结果「一致」——
// 这个机制防的是「改数据不改凭证」，不是「从一开始就伪造」。
// 详见 `internal/service/verification.go` 的文件头。
func (s *Server) adminVerifyBattle(c *fiber.Ctx) error {
	id, err := strconv.ParseInt(c.Params("id"), 10, 64)
	if err != nil || id < 1 {
		return fail(c, fiber.StatusBadRequest, "bad_input", "battle id 非法")
	}
	var body struct {
		ReplayHash string `json:"replay_hash"`
	}
	//
	// ⚠️ 第 105 轮：原来也是 `_ = c.BodyParser(&body)`。
	//
	// 与 `adminBanUser` 不同，**这里当时是安全的** ——
	// 因为紧接着的 `if body.ReplayHash == ""` 把后果挡住了
	// （解析失败 → hash 为空 → 被判 400）。
	//
	// 但那是**运气**：安全来自下游的一个巧合，
	// 而不是来自这里检查了解析错误。
	// 任何人日后放宽那个空值检查（例如为了支持「空 = 跳过」），缺陷立刻回来。
	//
	// 18 个处理器检查 BodyParser 的错误，这两处不检查 ——
	// 现在两处都检查，理由写在各自注释里。
	if len(bytes.TrimSpace(c.Body())) > 0 {
		if err := c.BodyParser(&body); err != nil {
			return fail(c, fiber.StatusBadRequest, "bad_json", "请求体不是合法 JSON")
		}
	}
	// 空的 actual hash 会被判成「不匹配」，把一条**没验过**的记录
	// 写成「验过且不符」—— 那是凭空制造一条指控。
	// 所以这里直接拒掉，与空白的处理方式保持一致。
	if body.ReplayHash == "" {
		return fail(c, fiber.StatusBadRequest, "bad_input", "缺少 replay_hash")
	}
	// adminIDFrom 在取不到时返回 0，而 0 不是合法的 admin_users.id
	// （`admin_verifier_id` 是外键）—— 插进去会撞 CHECK 或外键约束。
	// 与其让那条约束报错（500，信息还很难读），不如明确拒绝。
	adminID := adminIDFrom(c)
	if adminID == 0 {
		return fail(c, fiber.StatusUnauthorized, "unauthorized", "缺少管理员身份")
	}
	res, err := s.Svc.AdminVerifyReplay(c.Context(), adminID, id, body.ReplayHash)
	if err != nil {
		return failErr(c, err)
	}
	// 验真是一次**管理动作**，审计轨迹要独立于统计表存在。
	s.Svc.Audit(c.Context(), adminID, "battle_verify", strconv.FormatInt(id, 10),
		service.AuditVerificationDetail(res))
	return c.JSON(res)
}

func (s *Server) adminDefenses(c *fiber.Ctx) error {
	limit, err := queryInt(c, "limit", 50)
	if err != nil {
		return failErr(c, err)
	}
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
	var expiresAt *time.Time
	if s := strings.TrimSpace(body.ExpiresAt); s != "" {
		//
		// ⚠️ 第 110 轮：handler 此前解析了 ExpiresAt 却从不传给 service，
		// UI 有「过期时间」输入框、运营设的值被**静默丢弃**，兑换码永久有效。
		// 解析必须带时区（RFC3339 或显式偏移），不接受裸本地时间——
		// 兑换判定的 `expires_at > now()` 是 UTC 语义，裸本地时间会让
		// 「过期」在部署机时区漂移 8 小时。
		parsed, err := time.Parse(time.RFC3339, s)
		if err != nil {
			return fail(c, fiber.StatusBadRequest, "bad_input",
				"expires_at 必须是 RFC3339 格式（如 2026-12-31T23:59:59Z），不能是裸本地时间")
		}
		expiresAt = &parsed
	}
	item, err := s.Svc.AdminCreateRedeemCode(c.Context(), body.Code, body.Reward, body.MaxUses, expiresAt)
	if err != nil {
		return failErr(c, err)
	}
	s.Svc.Audit(c.Context(), adminIDFrom(c), "create_redeem_code", body.Code, body)
	return c.JSON(fiber.Map{"item": item})
}

func (s *Server) adminAuditLogs(c *fiber.Ctx) error {
	limit, err := queryInt(c, "limit", 100)
	if err != nil {
		return failErr(c, err)
	}
	items, err := s.Svc.AdminListAuditLogs(c.Context(), limit)
	if err != nil {
		return failErr(c, err)
	}
	return c.JSON(fiber.Map{"items": items})
}

// ctxUnused 保留下划线避免未使用告警（中间件已封装 Context 传递）。
var _ = context.Background
