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

	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/service"
)

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

// requireAdmin 校验管理员 Bearer 令牌并把 admin_id 写入 Locals。
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
		c.Locals("admin_id", adminID)
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

// CORS 允许后台与 H5 跨域访问（开发期直接放行，生产应收紧）。
func cors() fiber.Handler {
	return func(c *fiber.Ctx) error {
		c.Set("Access-Control-Allow-Origin", "*")
		c.Set("Access-Control-Allow-Methods", "GET,POST,PUT,DELETE,OPTIONS")
		c.Set("Access-Control-Allow-Headers", "Content-Type,Authorization")
		if c.Method() == fiber.MethodOptions {
			return c.SendStatus(fiber.StatusNoContent)
		}
		return c.Next()
	}
}
