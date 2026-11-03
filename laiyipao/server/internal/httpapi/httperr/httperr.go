// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// Package httperr 是**唯一的错误出口**（R69）。
//
// # 为什么要有这个包
//
// R67 把「依赖瞬时故障」从 500 改成 503 + Retry-After，R68 把错误码
// 收进一张契约表 —— 但错误**出口**本身仍是分散的：每个 handler 都可以
//
//	return failErr(c, err)
//
// 或者自己判断 `errors.Is(err, service.ErrBadInput)` 再 `fail(...)`。
// 分散的代价不是「代码不整齐」，而是**同一个错误在不同 handler 里
// 可能得到不同状态码**，而没有任何测试在看这一点：
//
//   - handler 自己判断漏一个类型 → 落到 500，客户端被告知「服务坏了」；
//   - 两个 handler 对同一类错误给出 400 与 422 → 客户端要为同一种情况
//     写两条分支，漏一条就显示「未知错误」；
//   - 新增错误类型时没人知道该在哪个文件登记。
//
// 所以把「错误 → 状态码/码/Retry-After」的判定收进这个包：
//
//	handler / middleware 只 **return err**
//	Fiber 的 ErrorHandler 调一次 Render(c, err)
//
// 包外不允许出现任何写错误响应的调用（`fail(`、`failErr(`）。这条由
// `error_entry_test.go` 做全仓 AST 扫描断言 —— 人工检查会漏，而扫描不会。
package httperr

import (
	"errors"
	"log"

	"github.com/gofiber/fiber/v2"

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

// Error 是 handler/middleware 主动构造的对外错误。
//
// handler 只负责「这次请求失败，原因是 X」，不再决定响应怎么写 ——
// 写响应是 Render 的事。
type Error struct {
	Status  int
	Code    string
	Message string
}

func (e *Error) Error() string { return e.Message }

// New 构造一个对外错误。状态与码必须与 codes.go 的契约一致
// （`error_entry_test.go` 会扫描所有调用点核对）。
func New(status int, code, message string) *Error {
	return &Error{Status: status, Code: code, Message: message}
}

// 日志钩子：由 httpapi.Register 接到分级日志（LOG_LEVEL）上。
// 放在这里而不是直接 log.Printf，是为了让 httperr 不反向依赖 httpapi。
var (
	logWarn  = func(format string, args ...any) { log.Printf(format, args...) }
	logError = func(format string, args ...any) { log.Printf(format, args...) }
)

// SetLoggers 注入分级日志函数（httpapi.Register 启动时调一次）。
func SetLoggers(warn, errFn func(format string, args ...any)) {
	if warn != nil {
		logWarn = warn
	}
	if errFn != nil {
		logError = errFn
	}
}

// Render 是**唯一**的错误出口：把任意错误写成对外的 JSON 响应。
//
// 映射顺序是固定的，且与 R67/R68 的对外行为逐条对齐：
//
//	*Error（handler 主动构造） → 用它自带的状态/码
//	*fiber.Error（框架错误）   → http_error + 它自带状态（404/405/413…）
//	service 哨兵               → 401/403/404/400/413
//	导出闸门占用               → 503 + export_busy + Retry-After
//	结算拒绝                   → 422 + settle_rejected
//	依赖瞬时故障               → 503 + service_unavailable + Retry-After
//	其余                       → 500 + internal_error（真实原因进日志）
func Render(c *fiber.Ctx, err error) error {
	if err == nil {
		return nil
	}
	status, code, message, logAs := classify(err)
	// R84：错误详情行带 trace id（与 httpapi 中间件的 Locals key 一致；
	// e2e 断言「错误日志含响应头同款 id」钉住这条链，两处不会悄悄分叉）。
	reqID, _ := c.Locals("trace_id").(string)
	if reqID == "" {
		reqID = "-"
	}
	switch logAs {
	case logWarnLevel:
		logWarn("[http] req=%s %s %s -> %d: %v", reqID, c.Method(), c.OriginalURL(), status, err)
	case logErrorLevel:
		logError("[http] req=%s %s %s -> %d: %v", reqID, c.Method(), c.OriginalURL(), status, err)
	}
	return write(c, status, code, message)
}

// StatusOf 返回这个错误**最终会被写成**的 HTTP 状态。
//
// 供访问日志（middleware）在 ErrorHandler 运行**之前**使用：R69 之后
// handler 只 `return err`，中间件跑完时 `c.Response().StatusCode()` 仍是 200，
// 若不先解析，4xx 访问日志会全部掉进 debug 档而消失。
//
// 判据与 Render 共用 classify，避免两处各写一份映射又悄悄分叉。
func StatusOf(err error) int {
	status, _, _, _ := classify(err)
	return status
}

// 日志级别：Render 只在「需要留原因」的分支打日志。
const (
	logNone = iota
	logWarnLevel
	logErrorLevel
)

// classify 是 Render 与 StatusOf 的**唯一**判定逻辑。
func classify(err error) (status int, code, message string, logAs int) {
	var appErr *Error
	if errors.As(err, &appErr) {
		return appErr.Status, appErr.Code, appErr.Message, logNone
	}

	var fiberErr *fiber.Error
	if errors.As(err, &fiberErr) {
		// 框架错误（未知路由 404、方法不允许 405、body 超限 413…）。
		// 状态由框架给出，码统一 http_error（R68 已存在的对外口径）。
		return fiberErr.Code, CodeHTTPError, fiberErr.Message, logNone
	}

	switch {
	case errors.Is(err, service.ErrUnauthorized):
		return fiber.StatusUnauthorized, CodeUnauthorized, err.Error(), logNone
	case errors.Is(err, service.ErrForbidden):
		return fiber.StatusForbidden, CodeForbidden, err.Error(), logNone
	case errors.Is(err, service.ErrNotFound):
		return fiber.StatusNotFound, CodeNotFound, err.Error(), logNone
	case errors.Is(err, service.ErrBadInput):
		return fiber.StatusBadRequest, CodeBadInput, err.Error(), logNone
	case errors.Is(err, service.ErrExportTooLarge):
		// 413 而不是 400：参数没写错，是筛选范围太宽导致结果超上限。
		return fiber.StatusRequestEntityTooLarge, CodeExportTooLarge, err.Error(), logNone
	case errors.Is(err, ErrExportBusy):
		// 503 而不是 429/500：资源暂时被占用，可重试，且与按 IP 的限流区分开。
		return fiber.StatusServiceUnavailable, CodeExportBusy, "导出任务繁忙，请稍后重试", logNone
	}

	// 结算拒绝类错误统一 422：请求格式对，但内容不可信。
	if IsSettleRejection(err) {
		return fiber.StatusUnprocessableEntity, CodeSettleRejected, err.Error(), logNone
	}

	// 数据库/上游依赖的**瞬时**故障 → 503 + Retry-After。
	// 与 500 的区别是给客户端的语义：503 表示「现在不行，稍后可能成功」。
	if IsTransientDependency(err) {
		return fiber.StatusServiceUnavailable, CodeServiceUnavailable,
			"依赖服务暂时不可用，请稍后重试", logWarnLevel
	}

	// 其余一律 500 + 通用文案：不对外暴露数据库报错/SQL 片段。
	// 真实原因必须落到服务端日志（error 级），出问题时线上才能定位。
	return fiber.StatusInternalServerError, CodeInternalError, "服务内部错误", logErrorLevel
}

// write 按契约写响应。Retry-After 由契约表统一决定，调用方不再手写。
func write(c *fiber.Ctx, status int, code, message string) error {
	if contract, ok := contractOf(code); ok && contract.RetryAfter != "" {
		c.Set(fiber.HeaderRetryAfter, contract.RetryAfter)
	}
	return c.Status(status).JSON(APIError{Error: ErrorBody{Code: code, Message: message}})
}

// ErrExportBusy 表示导出并发已达上限且排队超时。
var ErrExportBusy = errors.New("导出并发已达上限")
