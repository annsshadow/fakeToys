// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// Package httpapi 提供 Fiber 路由、中间件与处理器。
package httpapi

import (
	"context"
	"errors"
	"fmt"
	"io"
	"log"
	"net"
	"runtime/debug"
	"strconv"
	"strings"
	"sync/atomic"
	"syscall"
	"time"

	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/limiter"
	"github.com/jackc/pgx/v5/pgconn"
	"github.com/jackc/puddle/v2"

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
	case errors.Is(err, service.ErrExportTooLarge):
		// 413 而不是 400：参数没写错，是筛选范围太宽导致结果超上限。
		// 文案带上了实际行数与上限，运营能据此缩小范围。
		return fail(c, fiber.StatusRequestEntityTooLarge, "export_too_large", err.Error())
	case errors.Is(err, errExportBusy):
		// 503 而不是 429/500：这是「资源暂时被占用」，可重试，且与限流
		// （按 IP/分钟）区分开 —— 运营排队 3 秒后重试即可成功。
		return fail(c, fiber.StatusServiceUnavailable, "export_busy", "导出任务繁忙，请稍后重试")
	}
	// 结算拒绝类错误统一 422：请求格式对，但内容不可信
	if isSettleRejection(err) {
		return fail(c, fiber.StatusUnprocessableEntity, "settle_rejected", err.Error())
	}
	// 数据库/上游依赖的**瞬时**故障 → 503 + Retry-After。
	//
	// 与 500 的区别是给客户端的语义：503 表示「现在不行，稍后重试可能成功」，
	// 客户端/网关可以按 Retry-After 退避重试；500 表示「请求本身有问题或
	// 服务有 bug」，重试只会放大故障。此前断连/超时全部落到 500，
	// 客户端既不能退避也不能区分，只能把「数据库抖一下」显示成「服务坏了」。
	if transientDependencyError(err) {
		// 5 秒：大于一次 pgx 连接重建/语句重试的成本，又短于用户耐心。
		c.Set(fiber.HeaderRetryAfter, "5")
		logAt(logWarn, "[http] %s %s -> 503: %v", c.Method(), c.OriginalURL(), err)
		return fail(c, fiber.StatusServiceUnavailable, "service_unavailable", "依赖服务暂时不可用，请稍后重试")
	}
	// 其余一律 500 + 通用文案。
	// 真实原因必须落到服务端日志（error 级），出问题时线上才能定位。
	logAt(logError, "[http] %s %s -> 500: %v", c.Method(), c.OriginalURL(), err)
	return fail(c, fiber.StatusInternalServerError, "internal_error", "服务内部错误")
}

// transientDependencyError 判断错误是否属于「稍后重试可能成功」的依赖故障。
//
// 只收**已知的瞬时类别**，不做「所有非 nil 错误都当 503」的兜底：
// 后者会把 SQL 语法错误、约束冲突、扫描类型不符这些真 bug 伪装成
// 「服务暂时不可用」，客户端会无限重试，排障也会被误导。
//
// 分类依据：
//   - 连接池已关闭（优雅停机/重建）→ puddle.ErrClosedPool；
//   - 连接级故障 → PostgreSQL class 08（connection exception）、
//     57P01/57P02/57P03（后台终止/崩溃/暂不可连）、53300（连接数满）、
//     53200（内存不足）；
//   - 语句被取消 → 57014（statement_timeout / 主动 cancel），
//     这也是本仓库 `DB_STATEMENT_TIMEOUT` 超时的实际错误码；
//   - 网络层 → *net.OpError、ECONNREFUSED/ECONNRESET/EPIPE、EOF
//     （数据库断连与微信等上游不可达都会落到这里）；
//   - 请求/加载超时 → context.DeadlineExceeded。
//
// 故意**不**包含 context.Canceled：那是客户端主动断开，响应已经无人接收，
// 归为 503 只会制造一条误导性的告警。
func transientDependencyError(err error) bool {
	if err == nil {
		return false
	}
	if errors.Is(err, puddle.ErrClosedPool) ||
		errors.Is(err, context.DeadlineExceeded) ||
		errors.Is(err, io.EOF) || errors.Is(err, io.ErrUnexpectedEOF) ||
		errors.Is(err, syscall.ECONNREFUSED) ||
		errors.Is(err, syscall.ECONNRESET) ||
		errors.Is(err, syscall.EPIPE) {
		return true
	}
	var pgErr *pgconn.PgError
	if errors.As(err, &pgErr) {
		switch {
		case strings.HasPrefix(pgErr.Code, "08"):
			return true
		case pgErr.Code == "57014", pgErr.Code == "57P01", pgErr.Code == "57P02",
			pgErr.Code == "57P03", pgErr.Code == "53300", pgErr.Code == "53200":
			return true
		}
		return false
	}
	var netErr *net.OpError
	return errors.As(err, &netErr)
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

// --- 访问日志分级（LOG_LEVEL 的消费方，第 4 轮）---
//
// LOG_LEVEL 此前只在 config 解析、全仓零消费（死配置）。这里给它最小的
// 分级消费：stdlib log，不引第三方日志库 —— 本服务要记的量级
// （500 原因 + 4xx + 慢请求）用不上结构化日志的复杂度。
//
// 分级阶梯（默认 info）：
//   - error：500 的真实原因（failErr）、直接写出的 5xx 响应
//   - warn ：4xx 响应
//   - info ：慢请求（≥ slowRequestThreshold）
//   - debug：其余全部访问 + handler 返回错误但未写 500 的路径

type logLevel int32

const (
	logDebug logLevel = iota
	logInfo
	logWarn
	logError
)

// accessLevel 是当前访问日志级别。原子读：日志判定在每个请求上，
// 不为读档引入锁。
var accessLevel atomic.Int32

// slowRequestThreshold 是「慢请求」的判定线（info 级）。测试会调小它。
var slowRequestThreshold = time.Second

// parseLogLevel 解析 LOG_LEVEL。未知值回退 info 并打警告：
// 配置写错不该静默（运维以为记了其实没记），也不该因此打死进程。
func parseLogLevel(s string) logLevel {
	switch strings.ToLower(strings.TrimSpace(s)) {
	case "debug":
		return logDebug
	case "info", "":
		return logInfo
	case "warn", "warning":
		return logWarn
	case "error":
		return logError
	default:
		log.Printf("[http] 未知 LOG_LEVEL %q，按 info 处理（可选 debug/info/warn/error）", s)
		return logInfo
	}
}

// SetLogLevel 设置访问日志级别（Register 启动时从 config.LOG_LEVEL 调一次；
// 测试直接调用）。与 SetDevMode 同款包级形状。
func SetLogLevel(s string) { accessLevel.Store(int32(parseLogLevel(s))) }

// logAt 按级别打日志，级别低于当前档的调用直接丢弃。
func logAt(level logLevel, format string, args ...any) {
	if int32(level) < accessLevel.Load() {
		return
	}
	log.Printf(format, args...)
}

// requestLogger 记录访问日志并写 X-Request-Duration（分级见 logLevel）。
func requestLogger() fiber.Handler {
	return func(c *fiber.Ctx) error {
		start := time.Now()
		err := c.Next()
		dur := time.Since(start)
		c.Set("X-Request-Duration", dur.String())
		status := c.Response().StatusCode()
		switch {
		case err != nil:
			// handler 返回错误：最终状态码由 ErrorHandler 写出，这里还看不到。
			// 500 的真实原因 failErr 已按 error 级落日志，不重复；
			// 本行在 debug 级补「哪个端点、哪个错误」的上下文。
			logAt(logDebug, "[http] %s %s -> handler error (%s): %v",
				c.Method(), c.OriginalURL(), dur, err)
		case status >= 500:
			logAt(logError, "[http] %s %s -> %d (%s)",
				c.Method(), c.OriginalURL(), status, dur)
		case status >= 400:
			logAt(logWarn, "[http] %s %s -> %d (%s)",
				c.Method(), c.OriginalURL(), status, dur)
		case dur >= slowRequestThreshold:
			logAt(logInfo, "[http] %s %s -> %d (%s，慢请求)",
				c.Method(), c.OriginalURL(), status, dur)
		default:
			logAt(logDebug, "[http] %s %s -> %d (%s)",
				c.Method(), c.OriginalURL(), status, dur)
		}
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
		adminID, role, err := s.ResolveAdminSession(c.Context(), token)
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

// queryTime 解析可选的日期/时间查询参数（2026-10-31 新一轮第 14 轮）。
//
// 键不在 → nil（不过滤）。值支持两种格式：
//   - `YYYY-MM-DD`：当天 00:00:00 UTC（后台日期选择器的自然格式）；
//   - RFC3339：精确到秒的时间点。
//
// 解析失败返回 ErrBadInput（与 queryInt 同一个 fail-loud 口径，
// 运营手滑打错格式看到的是 400 而不是「静默不过滤」）。
func queryTime(c *fiber.Ctx, name string) (*time.Time, error) {
	if !c.Context().QueryArgs().Has(name) {
		return nil, nil
	}
	raw := strings.TrimSpace(c.Query(name))
	if raw == "" {
		return nil, nil
	}
	t, _, err := parseQueryTime(name, raw)
	if err != nil {
		return nil, err
	}
	return &t, nil
}

// parseQueryTime 解析单个时间值。dateOnly 表示输入形如 `YYYY-MM-DD`
// （此时 t 是当天 00:00 UTC），调用方据此决定左端/右端的语义。
func parseQueryTime(name, raw string) (t time.Time, dateOnly bool, err error) {
	if t, err := time.Parse("2006-01-02", raw); err == nil {
		return t, true, nil
	}
	if t, err := time.Parse(time.RFC3339, raw); err == nil {
		return t, false, nil
	}
	return time.Time{}, false, fmt.Errorf("%w: 查询参数 %s=%q 不是合法日期（期望 YYYY-MM-DD 或 RFC3339）",
		service.ErrBadInput, name, raw)
}

// queryTimeEnd 解析范围右端。与 queryTime 的唯一区别：
// `YYYY-MM-DD` 解释为**次日 00:00（不含）**。
//
// # 为什么右端必须 +1 天
//
// 服务层的日期条件一律是半开区间 `created_at < $to`（见 adminapi.go /
// stats.go 的各条 SQL）。若 `date_to=2026-11-02` 按当天 00:00 传入，
// 11-02 全天的数据都会被 `< 00:00` 排除 —— 运营在 UI 上明明选了
// 「到 11-02」，看到的是「11-02 没数据」。这不是少一两条，是**整天缺失**，
// 而且状态码是 200、响应形状完全正常，没有人会去查。
func queryTimeEnd(c *fiber.Ctx, name string) (*time.Time, error) {
	if !c.Context().QueryArgs().Has(name) {
		return nil, nil
	}
	raw := strings.TrimSpace(c.Query(name))
	if raw == "" {
		return nil, nil
	}
	t, dateOnly, err := parseQueryTime(name, raw)
	if err != nil {
		return nil, err
	}
	if dateOnly {
		t = t.Add(24 * time.Hour)
	}
	return &t, nil
}

// queryTimeRange 成对解析时间范围（左闭右开），并拒绝反转/空范围。
//
// # 为什么反转要报错而不是返回空集
//
// `date_from=2026-11-05&date_to=2026-11-01` 在 SQL 里是
// `>= 11-05 AND < 11-01`，必然为空，且状态码 200。
// 运营看到「无数据」会得出「这段时间没有异常/没有订单」的结论，
// 而真实情况是两端选反了。与 keyword 拼错、user_id 打错同族：
// **输入无效却得到合法的空结果** 是本项目最反复修的一类缺陷。
//
// 右端用 queryTimeEnd：日期格式的 `date_to` 包含当天整天。
func queryTimeRange(c *fiber.Ctx, fromName, toName string) (from, to *time.Time, err error) {
	from, err = queryTime(c, fromName)
	if err != nil {
		return nil, nil, err
	}
	to, err = queryTimeEnd(c, toName)
	if err != nil {
		return nil, nil, err
	}
	if from != nil && to != nil && !from.Before(*to) {
		return nil, nil, fmt.Errorf("%w: 时间范围为空：%s=%q 不早于 %s=%q 的右端（结果必然为空）",
			service.ErrBadInput, fromName, strings.TrimSpace(c.Query(fromName)),
			toName, strings.TrimSpace(c.Query(toName)))
	}
	return from, to, nil
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

	// playerLoginLimit 是玩家账号登录的每 IP 每分钟上限。
	//
	// # 为什么不是游客建号那样宽松
	//
	// 两个原因叠在一起：
	//
	//  1. 它做 bcrypt（cost 10，单次 50~100ms **纯 CPU**）——
	//     和后台登录一样贵，是未认证请求里最贵的一档。
	//  2. ⚠️ **测试环境的共享账号口令与运营超管相同**，
	//     所以这个端点同时是**超管口令的爆破面**。
	//     给 60 次/分钟等于给爆破器 4 倍于后台登录的额度。
	//
	// # 为什么不是后台那样严（5）
	//
	// 共享测试账号是**整个测试组**在用，同一出口 IP 下可能同时有十几个人
	// 在登录（NAT 场景）。5 次/分钟会误伤正常测试 ——
	// 一个把正常人挡在门外的限额，比没有限额更难排查。
	//
	// 10 次/分钟：正常人（一次冷启动一次登录）余量极大，
	// 爆破器每天 14400 次上限，且 bcrypt 让每次都很贵。
	playerLoginLimit = 10

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
// 判据是**写放大**：每次调用都落库、且没有幂等键挡住重复的端点才限流。
// 查询类端点不放大存储，不在此列。最初只覆盖下面三条，后续按同一判据
// 扩到其它写端点（R52 补齐 `economyWriteLimit` 注释里列出、但实现漏掉的经济写路由）。
//
//   - `/battle/verify`：实测同一场战报重复 200 次就写 200 行
//     （`replay_verifications` 上既无唯一约束也无去重）。
//   - `/battle/settle`：每次一张 `battle_records`。
//   - `/redeem`：已由 IP 额度覆盖，但同一个账号在 NAT 后换 IP 也该被限。
const (
	settleLimit   = 60
	verifyLimit   = 20
	perUserRedeem = 20

	// battleTokenLimit 是开战铸造 token 的每用户每分钟上限。
	//
	// 每次 `POST /battle/token` 都**插一行 battle_tokens + 一行钱包流水**
	//（见 service.StartBattle 的事务）。体力会随时间回复，所以「反复开战
	// 不结算」能在体力允许的范围内无限造 token 行 —— 这是**写放大**面。
	// 正常玩家一分钟打不了几关（还得真的打完），120 给足余量；
	// 对刷量脚本则是硬顶。按用户而非 IP：开战在登录态，且同一 NAT 下
	// 多个真实玩家不该互相挤占（见 perIPLimiter 的 NAT 说明）。
	battleTokenLimit = 120

	// economyWriteLimit 是「花自己资源」的写端点每用户每分钟上限：
	// 买 / 领 / 签 / 洗 / 升 / 熔 / 分配 / 重铸 / 挑战 / 放弃 / 存配装。
	//
	// 这些端点花的是**玩家自己的**钱或道具，经济上无法靠刷量获利
	//（不可能花超过自己拥有的），所以限流防的不是套利，而是**写放大**：
	// 每个动作都落库，无限制时一个账号能把库写爆（自我 DoS）并拖慢全站。
	// 60/分钟对任何真实操作都绰绰有余；NAT 安全，因为按 user_id 计。
	economyWriteLimit = 60

	// passwordChangeLimit 是改密码的每用户每分钟上限。
	//
	// 改密码是口令爆破面：拿到 access token 后攻击者可以对着旧密码无限试。
	// 正常玩家一天改不了一次，10 次/分钟足够宽，同时把暴力试探压住。
	// 按 user_id 计（authRateLimit），NAT 下多个真实玩家互不挤占。
	passwordChangeLimit = 10

	// accountUpgradeLimit 是「游客转正」的每用户每分钟上限。
	//
	// 转正是注册面：脚本可以批量占用户名 / 枚举「哪个用户名已被占用」。
	// 正常玩家一生只转正一次，10 次/分钟足够宽，同时把脚本批量注册压住。
	// 按 user_id 计（authRateLimit），NAT 下多个真实玩家互不挤占。
	accountUpgradeLimit = 10

	// nicknameChangeLimit 是改昵称的每用户每分钟上限。
	//
	// 昵称会进排行榜 / 战报 / 防线 / 后台列表等所有列表响应，频繁改名
	// 可用来刷存在感或干扰后台审计。正常玩家一天改不了一次，
	// 10 次/分钟足够宽。按 user_id 计（authRateLimit），NAT 安全。
	nicknameChangeLimit = 10
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
