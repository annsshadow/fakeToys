// Package httpapi 提供 Fiber 路由、中间件与处理器。
package httpapi

import (
	"errors"
	"log"
	"runtime/debug"
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
