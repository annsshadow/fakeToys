// Package httpapi 提供 Fiber 路由、中间件与处理器。
package httpapi

import (
	"errors"
	"fmt"
	"log"
	"runtime/debug"
	"strconv"
	"strings"
	"time"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/limiter"

	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/service"
)

// devMode 决定 CORS 是否允许 `*`。
//
// ⚠️ 由 `SetDevMode` 在 `Register` 开头设置。默认值 false 是**故意的**：
// 任何没走 `Register` 的调用方（例如直接 `buildApp` 的测试）都会拿到
// 「不发 CORS 头」的保守行为，而不是悄悄拿到 `*`。
var devMode bool

// SetDevMode 告诉中间件层当前是不是开发环境。
func SetDevMode(dev bool) { devMode = dev }

// APIError 是统一的错误响应体。
type APIError struct {
	Error ErrorBody `json:"error"`
}

// ErrorBody 是错误详情。
type ErrorBody struct {
	Code    string `json:"code"`
	Message string `json:"message"`
}

// fail 写出错误响应。
func fail(c *fiber.Ctx, status int, code, message string) error {
	return c.Status(status).JSON(APIError{Error: ErrorBody{Code: code, Message: message}})
}

// failErr 把服务层错误映射为 HTTP 响应。
//
// 关键：不对外暴露内部错误细节（数据库报错、SQL 片段）。
// 500 只给一句通用文案，真实原因进日志。
func failErr(c *fiber.Ctx, err error) error {
	switch {
	case errors.Is(err, service.ErrUnauthorized):
		return fail(c, fiber.StatusUnauthorized, "unauthorized", err.Error())
	case errors.Is(err, service.ErrForbidden):
		return fail(c, fiber.StatusForbidden, "forbidden", err.Error())
	case errors.Is(err, service.ErrNotFound):
		return fail(c, fiber.StatusNotFound, "not_found", err.Error())
	case errors.Is(err, service.ErrBadInput):
		return fail(c, fiber.StatusBadRequest, "bad_input", err.Error())
	}
	// 结算拒绝类错误统一 422：请求格式对，但内容不可信
	if isSettleRejection(err) {
		return fail(c, fiber.StatusUnprocessableEntity, "settle_rejected", err.Error())
	}
	// 其余一律 500 + 通用文案。
	// 真实原因必须落到服务端日志，出问题时线上才能定位。
	log.Printf("[http] %s %s -> 500: %v", c.Method(), c.OriginalURL(), err)
	return fail(c, fiber.StatusInternalServerError, "internal_error", "服务内部错误")
}

// isSettleRejection 判断错误是否属于「结算被拒」类。
//
// ⚠️ 这份清单必须与 domain 包的结算错误**同步**，否则新错误会掉进 500。
// 后果有两处，都很糟：
//  1. 客户端收到「服务内部错误」，无法区分"你作弊了"和"我们坏了"；
//  2. 服务端日志被大量假 500 刷满，真正的故障被淹没。
//
// 已经漂移过一次：给 reactions / shots / leaked / heat 上界时新增了四个
// domain 错误，都没进这个清单，于是这些**用户输入不合理**的请求
// 全部返回 500。
//
// 新增 domain 结算错误时记得同步这里 —— TestSettleRejectionsAllMapTo422
// 会把两边的差集直接打出来。
func isSettleRejection(err error) bool {
	for _, target := range []error{
		// 凭证类
		domain.ErrTokenNotFound, domain.ErrTokenUsed, domain.ErrTokenExpired,
		domain.ErrLevelMismatch,
		// 数值越界类
		domain.ErrTooManyKills, domain.ErrTooShort, domain.ErrScoreRateExceeded,
		domain.ErrInvalidHitRate, domain.ErrWaveExceeded,
		// 上界校验类（2026-09-26 补齐，此前全部误返回 500）
		domain.ErrTooManyReactions, domain.ErrTooManyShots,
		domain.ErrTooManyLeaked, domain.ErrInvalidField,
		// HPLeft / slot-budget / replay_skills（第 58 轮补）。
		//
		// 这三个都是「用户输入被理解后拒绝」，此前一律落到兜底的 500：
		//   - ErrHPLeftExceedsBase      结算时上报的剩余血量超过关卡初始血量
		//   - ErrSlotBudgetExceeded     装备件数超过槽位预算（loadout 路径）
		//   - ErrReplaySkillsMismatch   回放 S 段与构筑不符（第 56 轮加的校验）
		//
		// 500 的代价是具体的：客户端只能显示「服务内部错误」，
		// 排查会滑向「服务端坏了」而不是「这个上报被拒了」，
		// 而且它们会混进服务故障告警，把真正的故障淹掉。
		//
		// `TestEverySentinelErrorIsClassified` 保证这个名单不会再漏 ——
		// 之前那份名单是硬编码的，漏一个测试照样绿。
		domain.ErrHPLeftExceedsBase,
		service.ErrSlotBudgetExceeded,
		service.ErrReplaySkillsMismatch,
	} {
		if errors.Is(err, target) {
			return true
		}
	}
	return false
}

// --- 中间件 ---

// requestLogger 记录访问日志。
func requestLogger() fiber.Handler {
	return func(c *fiber.Ctx) error {
		start := time.Now()
		err := c.Next()
		c.Set("X-Request-Duration", time.Since(start).String())
		return err
	}
}

// requireAdmin 校验管理员 Bearer 令牌并把 admin_id、admin_role 写入 Locals。
func requireAdmin(s *service.Service) fiber.Handler {
	return func(c *fiber.Ctx) error {
		token := bearerToken(c)
		if token == "" {
			return fail(c, fiber.StatusUnauthorized, "unauthorized", "缺少 Authorization 头")
		}
		adminID, err := s.ResolveAdmin(c.Context(), token)
		if err != nil {
			return failErr(c, err)
		}
		// 第 127 轮：连同角色一起解析，供 requireWritable 使用。
		role, err := s.AdminRole(c.Context(), adminID)
		if err != nil {
			return failErr(c, err)
		}
		c.Locals("admin_id", adminID)
		c.Locals("admin_role", role)
		return c.Next()
	}
}

// requireWritable 拒绝 readonly 账号的写操作。
//
// 必须挂在 requireAdmin **之后**（requireAdmin 负责写 admin_role）。
//
// ⚠️ 第 127 轮：admin_users.role 自 00007 迁移就标了 admin/ops/readonly
// 三个角色，但**没有任何端点消费它** —— readonly 账号可以和 admin 一样
// 封禁玩家、发币、改关卡/商城。
//
// ⚠️ 第 155 轮改成 **fail-closed 白名单**：只有 `admin` / `ops` 可写，
// 其余（含空串、未知值）一律 403。
// 此前的语义是「只拒 readonly，未知放行」—— 那意味着把 `admin_users.role`
// 手改成一个拼错的值（如 `readonly ` 尾随空格 / `readonl`）的账号
// 会变成**可写**的：一个数据录入错误直接升级成越权。
// 白名单方向上「未知值」只能朝安全的那一侧倒（拒绝），
// 而列本身 `DEFAULT 'ops'` 覆盖了正常行，误伤的只有**本来就该 403 的行**。
func requireWritable() fiber.Handler {
	return func(c *fiber.Ctx) error {
		role, _ := c.Locals("admin_role").(string)
		if role != "admin" && role != "ops" {
			return fail(c, fiber.StatusForbidden, "role_not_writable",
				fmt.Sprintf("当前角色 %q 不可执行写操作（仅 admin/ops）", role))
		}
		return c.Next()
	}
}

// adminIDFrom 读取中间件写入的 admin_id。
func adminIDFrom(c *fiber.Ctx) int64 {
	v, ok := c.Locals("admin_id").(int64)
	if !ok {
		return 0
	}
	return v
}

// recoverPanic 兜底 panic，避免单个请求打挂整个进程。
//
// ⚠️ panic 值必须直接落日志：曾经的实现是写进 Locals("internal_error")
// 等着 failErr 来读 —— 但 failErr 只在 handler 正常返回 error 时被调用，
// panic 展开（unwind）后根本不会走到它，那份"详情"永远无人读取，
// 线上只能看到"服务内部错误"五个字，panic 现场完全丢失。
func recoverPanic() fiber.Handler {
	return func(c *fiber.Ctx) (err error) {
		defer func() {
			if r := recover(); r != nil {
				log.Printf("[panic] %s %s: %v\n%s", c.Method(), c.OriginalURL(), r, debug.Stack())
				err = fail(c, fiber.StatusInternalServerError, "internal_error", "服务内部错误")
			}
		}()
		return c.Next()
	}
}

// requireUser 校验 Bearer 令牌并把 user_id 写入 Locals。
func requireUser(s *service.Service) fiber.Handler {
	return func(c *fiber.Ctx) error {
		token := bearerToken(c)
		if token == "" {
			return fail(c, fiber.StatusUnauthorized, "unauthorized", "缺少 Authorization 头")
		}
		userID, err := s.ResolveUser(c.Context(), token)
		if err != nil {
			return failErr(c, err)
		}
		c.Locals("user_id", userID)
		return c.Next()
	}
}

// bearerToken 从请求头提取令牌。
func bearerToken(c *fiber.Ctx) string {
	h := c.Get("Authorization")
	if h == "" {
		return ""
	}
	const prefix = "Bearer "
	if len(h) > len(prefix) && strings.EqualFold(h[:len(prefix)], prefix) {
		return strings.TrimSpace(h[len(prefix):])
	}
	return ""
}

// queryInt 解析查询参数里的整数，并把「存在但不是合法整数」变成 ErrBadInput。
//
// # 为什么不能用 strconv 然后丢错误
//
// 第 104 轮：`adminBattles` 写的是
//
//	userID, _ := strconv.ParseInt(c.Query("user_id", "0"), 10, 64)
//
// 而 service 层的过滤条件是
//
//	($1 = 0 OR br.user_id = $1) AND ($2 = 0 OR br.level_id = $2)
//
// —— **0 是「不过滤」的哨兵值**（`stats.go`）。
// 于是 `?user_id=abc` 解析失败 → 0 → 过滤条件恒真
// → **返回所有用户的战报**，而且返回 **200**。实测确认。
//
// 后果不是「参数没生效」那么轻：运营在查一个疑似作弊的玩家时，
// 若 user_id 打错，看到的是**别人的**战报列表，
// 于是得出「这个人没有异常战报」的结论。
//
// 这是一个**会误导调查方向的静默错误答案**，比 400 坏得多。
//
// # 三种情形必须分开
//
//	（键不存在）  → 返回 def（「不过滤」/「默认页大小」）
//	?limit=      → ErrBadInput（400）
//	?limit=%20   → ErrBadInput（400）
//	?limit=abc   → ErrBadInput（400）
//
// # 为什么「显式空串」也算非法，而不是宽容地当缺失
//
// 我第一版把空串当缺失（理由是「前端拼 URL 可能传空」）。
// **实测推翻了它**：`?user_id=` 走默认值 0 → 不过滤 →
// 200 + **所有人的**战报 —— 与 `?user_id=abc` 危害完全相同。
//
// 宽容在这里不是仁慈，是**开一个同样的洞**。
//
// 而「前端会不会真发空参数」是可以查的，不必猜：
// `admin/src/api/index.ts` 用 `URLSearchParams` 且只在
// `if (params.x)` 为真时 `q.set(...)` —— **从不发空参数**。
//
// 所以判据必须是「键在不在」（`QueryArgs().Has`），
// 而不是「值空不空」（`c.Query(name) != ""`）。
// 前者区分「没传」与「传了但是空的」，后者分不开 ——
// 而这两个的**后果完全不同**。
func queryInt(c *fiber.Ctx, name string, def int) (int, error) {
	if !c.Context().QueryArgs().Has(name) {
		return def, nil
	}
	raw := strings.TrimSpace(c.Query(name))
	v, err := strconv.Atoi(raw)
	if err != nil {
		return 0, fmt.Errorf("%w: 查询参数 %s=%q 不是合法整数", service.ErrBadInput, name, raw)
	}
	return v, nil
}

// userIDFrom 读取中间件写入的 user_id。
func userIDFrom(c *fiber.Ctx) int64 {
	v, ok := c.Locals("user_id").(int64)
	if !ok {
		return 0
	}
	return v
}

// CORS 允许后台与 H5 跨域访问。
//
// ⚠️ 「生产应收紧」这句注释在本轮（审计轮）之前一直是**只有注释、没有实现** ——
// 而注释是最容易让所有人以为已经做了的形态。
//
// 现在按 `Env` 分两种行为：
//
//   - dev：原样保留 `*`（H5 devServer 与后台 devServer 不同端口，必须放行）
//   - prod：**不设任何 CORS 响应头**（等价于只允许同源）。
//     配了 `CORS_ALLOW_ORIGINS` 就只放行那份白名单。
//
// # 为什么 prod 的默认值是「什么都不发」而不是「继续发 *」
//
// 本项目**不用 cookie 存凭证**（纯 `Authorization: Bearer`，全仓库零 cookie），
// 所以 `Access-Control-Allow-Origin: *` 本身**不构成**「带凭证的跨域读」——
// 攻击站点读不到别origin 的 localStorage，浏览器也不会在 `*` 下带 cookie。
//
// 这一点必须诚实说明，否则会把它误报成一个并不存在的严重漏洞：
// 真正的风险是「将来有人加了 cookie 登录」——那时 `*` 立刻变成
// 全站沦陷，且没有任何东西会提醒。所以 prod 的**默认**取安全的一侧，
// 需要跨域就显式配白名单：那是一个需要人做的决定，而不是默认值。
func cors(allow []string) fiber.Handler {
	allowSet := make(map[string]bool, len(allow))
	for _, o := range allow {
		if o = strings.TrimSpace(o); o != "" {
			allowSet[o] = true
		}
	}
	wildcard := len(allowSet) == 0 && len(allow) == 0 && devMode
	return func(c *fiber.Ctx) error {
		origin := c.Get("Origin")
		switch {
		case wildcard:
			c.Set("Access-Control-Allow-Origin", "*")
		case origin != "" && allowSet[origin]:
			// 回显**具体来源**而不是 `*`：只有这样才谈得上带凭证，
			// 而本项目本来也不用 cookie，所以不必设 Allow-Credentials
			// （设了反而要求 `*` 失效，是无谓的收紧）。
			c.Set("Access-Control-Allow-Origin", origin)
			c.Set("Vary", "Origin")
		default:
			// 不在白名单：一条 CORS 头都不发。浏览器据此拒绝读取响应。
		}
		if origin != "" {
			c.Set("Access-Control-Allow-Methods", "GET,POST,PUT,DELETE,OPTIONS")
			c.Set("Access-Control-Allow-Headers", "Content-Type,Authorization")
			c.Set("Access-Control-Max-Age", "600")
		}
		if c.Method() == fiber.MethodOptions {
			return c.SendStatus(fiber.StatusNoContent)
		}
		return c.Next()
	}
}

// securityHeaders 下发与内容无关的安全响应头。
//
// # 为什么必须有：后台可被点击劫持（clickjacking）
//
// `admin` 是运营后台，而它**所有写操作都是单次点击即生效**
// （发币 / 封号 / 改价 / 再生成全部 100 关），没有二次确认弹窗。
//
// 缺了 `X-Frame-Options` / `frame-ancestors` 时，攻击者可以：
//
//	evil.com 页面 → <iframe src="https://后台域名/#/admin/users">
//	            → 上面盖一层透明遮罩，把假按钮对齐到 iframe 内的真按钮
//
// 已登录运营只要点一下那个假按钮，请求就**真的**带上了他自己
// localStorage 里的 `Authorization` 头发出去 —— 跨域读不到 token
// 也不影响，**因为执行的是他自己页面里的代码**。
//
// 结果落进 `admin_audit_logs`，看起来与运营亲手点的完全一样。
//
// ⚠️ 这不是理论风险也不是「加个头更保险」：它是当前**唯一一条**
// 「不需要任何服务端漏洞、只要运营点一下就能在后台执行写操作」的路径。
//
// # 为什么 XSS 类头排在后面
//
// 本项目源码里**不存在任何 XSS sink**（已全量核查 `v-html` / `innerHTML` /
// `eval` / `new Function`，admin 与 miniapp 均为 0 命中），
// 所以 CSP 此刻**拦不住任何已知攻击**。写上它的理由是纵深防御：
// admin 令牌存在 localStorage（`admin/src/api/client.ts`），
// 将来任何一处 `v-html` 疏漏都会被 CSP 挡在「读不到 token」之前。
//
// 因此 CSP 只收 `default-src 'self'` 与 `frame-ancestors 'none'`，
// **不加** `unsafe-inline` / `unsafe-eval` —— 加了等于没加。
// ⚠️ `default-src 'self'` 会禁掉内联脚本与样式；admin 走 Vite 构建产物
// （外链 JS/CSS），H5 亦然，所以是安全的。若将来有内联脚本，
// 正确做法是加 nonce 而不是放宽 `unsafe-inline`。
func securityHeaders() fiber.Handler {
	return func(c *fiber.Ctx) error {
		// 点击劫持：两个头都给，因为 `frame-ancestors` 只在 CSP2+ 生效，
		// 而 `X-Frame-Options` 是所有老浏览器的唯一手段。两者语义一致。
		c.Set("X-Frame-Options", "DENY")
		c.Set("Content-Security-Policy", "default-src 'self'; frame-ancestors 'none'")
		// 禁止 MIME 嗅探：否则被当成 text/plain 的 JSON 可能被当脚本执行。
		c.Set("X-Content-Type-Options", "nosniff")
		// 不外泄完整 URL（里面可能带 user_id 之类的查询参数）。
		c.Set("Referrer-Policy", "no-referrer")
		// 不宣告框架版本（除 X-Powered-By 外，Go 标准库也会加 Server 头，
		// 这里覆盖成不暴露具体实现）。
		c.Set("X-Powered-By", "")
		c.Set("Server", "")
		return c.Next()
	}
}

// --- 速率限制 ---

// 限流参数。每一档都写明**为什么是这个量**，而不是随手取整。
//
// # 为什么必须有限流（本轮新增，此前全站零限制）
//
// 未认证的三条入口各自都是放大器：
//
//  1. `POST /api/v1/admin/login` —— **bcrypt cost 10**（`bcrypt.DefaultCost`）
//     单次比对约 50~100ms **纯 CPU**。无限制时，未认证攻击者只要打满并发，
//     就能让正常运营**完全登不进后台**，同时顺便无限次口令喷洒。
//     这是本项目里「最贵的一次未认证请求」。
//
//  2. `POST /api/v1/auth/guest` —— 无需任何凭证。空 `guest_token` 时
//     一次调用写 7 行（users + wallet + progress + offline + 2 装备 + 4 技能 +
//     3 槽位）。无限流 = 可批量造号把库撑爆，且没有任何东西在计数。
//
//  3. `POST /api/v1/redeem` —— 兑换码无失败次数限制，可在线爆破。
//
// # 为什么按 IP 而不是按用户
//
// 前三条里有两条发生在**还没有身份**的时候，只能按来源限。
// 已认证的高频写端点（结算 / 验真 / 挑战）另有按身份的限流，见
// `authRateLimit` 的说明。
//
// # 为什么限流量给得比「直觉上的严」宽松
//
// 小程序玩家大量走**运营商 NAT**，同一个出口 IP 后面可能有成百上千个真实用户。
// 把「每 IP 每分钟」卡到个位数会**误伤正常玩家**，而一个误伤整个运营商网段的
// 限流比不限流更糟 —— 它会变成一个谁都说不清为什么的故障。
//
// 所以分工是：**按 IP 的额度卡住「一台机器能打多猛」，
// 按身份的额度卡住「一个账号能打多猛」**。
const (
	// adminLoginLimit 是后台登录的每 IP 每分钟上限。
	//
	// 后台操作者数量在个位数，且 bcrypt 让每次尝试都很贵 ——
	// 5 次/分钟对正常人有极大余量，对喷洒器则是 7200 次/天的天花板。
	adminLoginLimit = 5

	// guestLoginLimit 是游客建号/恢复的每 IP 每分钟上限。
	//
	// 给得比登录宽松：同一 NAT 下可能有很多玩家在**冷启动**时
	// 同时打这个端点（App 启动即调用）。
	guestLoginLimit = 60

	// refreshLimit 是刷新令牌的每 IP 每分钟上限。
	//
	// 一次冷启动用一次，偶发 401 重试再一两次。60 给足了余量。
	refreshLimit = 60

	// redeemLimit 是兑换码兑换的每 IP 每分钟上限。
	//
	// 正常玩家一天兑一两个码；5 次/分钟既拦得住爆破
	// （7200 次/天对短兑换码仍不够暴力）又不影响误操作重试。
	redeemLimit = 5
)

// perIPLimiter 返回一个「每 IP 每分钟 N 次」的限流中间件。
//
// ⚠️ **进程内计数**：多副本部署时实际额度是 N × 副本数。
// 本项目当前是单实例部署（`cmd/api` 监听 :8080，无水平扩展），
// 所以这是准确的；将来上多副本时必须换成 Redis 之类的共享计数器，
// 否则限流形同虚设。这一点写在这里而不是等出事时再想起来。
func perIPLimiter(name string, limitPerMinute int) fiber.Handler {
	return limiter.New(limiter.Config{
		Max:        limitPerMinute,
		Expiration: time.Minute,
		KeyGenerator: func(c *fiber.Ctx) string {
			return name + ":" + c.IP()
		},
		LimitReached: func(c *fiber.Ctx) error {
			return fail(c, fiber.StatusTooManyRequests, "rate_limited",
				"请求过于频繁，请稍后再试")
		},
	})
}

// authRateLimit 按**已认证身份**限流，覆盖「一个账号能打多猛」。
//
// 与 `perIPLimiter` 的分工：IP 额度挡住单点爆发，身份额度挡住
// 「注册 1000 个号、每个号慢慢打」的绕行 —— 那是对付 IP 限流最直接的手段。
//
// 它挂在 `requireUser` **之后**，所以 `c.Locals("user_id")` 已经有值；
// 限流键用 `user_id` 而不是 token —— 否则换发新令牌即可重置计数。
//
// 为什么只覆盖这三条：它们是**写放大**路径（每次调用都落库一行，
// 且没有任何幂等键挡住重复）。查询类端点不放大存储，不在此列。
//
//   - `/battle/verify`：实测同一场战报重复 200 次就写 200 行
//     （`replay_verifications` 上既无唯一约束也无去重）。
//   - `/battle/settle`：每次一张 `battle_records`。
//   - `/redeem`：已由 IP 额度覆盖，但同一个账号在 NAT 后换 IP 也该被限。
const (
	settleLimit   = 60
	verifyLimit   = 20
	perUserRedeem = 20
)

// authRateLimit 返回按登录身份计数的限流中间件。
func authRateLimit(name string, limitPerMinute int) fiber.Handler {
	return limiter.New(limiter.Config{
		Max:        limitPerMinute,
		Expiration: time.Minute,
		KeyGenerator: func(c *fiber.Ctx) string {
			return fmt.Sprintf("%s:user%d", name, userIDFrom(c))
		},
		LimitReached: func(c *fiber.Ctx) error {
			return fail(c, fiber.StatusTooManyRequests, "rate_limited",
				"操作过于频繁，请稍后再试")
		},
	})
}
