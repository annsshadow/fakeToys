// Package httpapi 提供 Fiber 路由、中间件与处理器。
package httpapi

import (
	"errors"
	"log"
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
	// 真实原因必须落到服务端日志 —— 只写进 Locals 等于没写，
	// 出问题时线上只能看到"服务内部错误"，无从定位。
	if detail, ok := c.Locals("internal_error").(string); ok {
		log.Printf("[http] %s %s -> 500: %s", c.Method(), c.OriginalURL(), detail)
	} else {
		log.Printf("[http] %s %s -> 500: %v", c.Method(), c.OriginalURL(), err)
	}
	return fail(c, fiber.StatusInternalServerError, "internal_error", "服务内部错误")
}

func isSettleRejection(err error) bool {
	for _, target := range []error{
		domain.ErrTokenNotFound, domain.ErrTokenUsed, domain.ErrTokenExpired,
		domain.ErrLevelMismatch, domain.ErrTooManyKills, domain.ErrTooShort,
		domain.ErrScoreRateExceeded, domain.ErrInvalidHitRate, domain.ErrWaveExceeded,
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
func recoverPanic() fiber.Handler {
	return func(c *fiber.Ctx) (err error) {
		defer func() {
			if r := recover(); r != nil {
				c.Locals("internal_error", "panic")
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
