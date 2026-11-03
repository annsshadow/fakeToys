// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httperr

import "github.com/gofiber/fiber/v2"

// 对外错误码的**唯一清单**（R68 建立，R69 随出口一起搬进本包）。
//
// # 为什么需要它
//
// R67 把「依赖瞬时故障」从 500 改成了 503 + Retry-After，但错误码本身
// 仍是散落在各 handler 里的字符串字面量：同一个语义可能写成两个码
// （`bad_id` / `bad_body` 与 `bad_input` 描述的是同一件事），
// 新增一个码时也没人检查它该配哪个 HTTP 状态、该不该带 Retry-After。
//
// 后果不是「代码不整齐」，而是**契约漂移**：
//   - 客户端按 `code` 做分支（例如 401 触发重新登录、503 触发退避），
//     一个码换了写法，客户端就静默走进兜底分支；
//   - 同一类错误两个码，意味着客户端要为同一种情况写两条分支，
//     漏写一条的症状是「偶发地显示未知错误」。
//
// 所以把所有码收进这一张表：码 → 状态码 → 是否带 Retry-After。
// Render 按表自动写 Retry-After；`error_entry_test.go` 用 AST 扫描
// **全仓**所有 `httperr.New(...)` 调用点，断言码是常量、状态与表一致。
const (
	// 400：请求体/参数不合法。
	//
	// ⚠️ `bad_id` / `bad_body` 在 R68 并入 `bad_input`。
	// 它们与 `bad_input` 是同一语义（输入能被解析，但值不可用），
	// 拆成三个码只会让客户端多写两条永远不会命中的分支。
	CodeBadJSON  = "bad_json"
	CodeBadInput = "bad_input"

	// 401 / 403：身份缺失或权限不足。
	//
	// ⚠️ `login_required` 是 403 而不是 401：请求本身合法，
	// 只是该环境关闭了游客建号 —— 客户端据此跳登录页，
	// 而不是当成「令牌失效」去刷新。
	CodeUnauthorized    = "unauthorized"
	CodeForbidden       = "forbidden"
	CodeRoleNotWritable = "role_not_writable"
	CodeLoginRequired   = "login_required"

	// 404：资源不存在或不属于调用方（两者刻意同码，避免 id 预言机）。
	CodeNotFound = "not_found"

	// 413：筛选范围太宽导致导出超上限（参数没写错，不该报 400）。
	CodeExportTooLarge = "export_too_large"

	// 422：结算内容不可信（格式对，但数值被拒）。
	CodeSettleRejected = "settle_rejected"

	// 429：限流。必须带 Retry-After，客户端据此退避。
	CodeRateLimited = "rate_limited"

	// 5xx：内部错误与依赖故障。
	//
	// ⚠️ `service_unavailable` / `export_busy` / `db_unavailable`
	// 都是 503，都必须带 Retry-After —— 503 的语义就是「稍后重试」，
	// 不带这个头等于让客户端猜等多久。
	CodeInternalError      = "internal_error"
	CodeServiceUnavailable = "service_unavailable"
	CodeExportBusy         = "export_busy"
	CodeDBUnavailable      = "db_unavailable"

	// CodeHTTPError 由框架错误（未知路由/方法不允许/body 超限）使用。
	//
	// ⚠️ 它**不在**契约表里：状态码取自 `*fiber.Error`（404/405/413…），
	// 不固定。这是唯一一个「码不绑定单一状态」的对外错误，
	// 由 Render 的 `*fiber.Error` 分支直接写出。
	CodeHTTPError = "http_error"
)

// errorContract 是一个错误码的对外契约。
type errorContract struct {
	Status int
	// RetryAfter 非空表示响应**必须**带 `Retry-After` 头，值即此处的秒数。
	RetryAfter string
}

// errorContracts 是「码 → 状态 → Retry-After」的权威表。
//
// ⚠️ 新增错误码时**必须**在这里登记：`error_entry_test.go` 会扫描
// 所有 `httperr.New(...)` 调用点，未登记的码直接红。
var errorContracts = map[string]errorContract{
	CodeBadJSON:  {Status: fiber.StatusBadRequest},
	CodeBadInput: {Status: fiber.StatusBadRequest},

	CodeUnauthorized:    {Status: fiber.StatusUnauthorized},
	CodeForbidden:       {Status: fiber.StatusForbidden},
	CodeRoleNotWritable: {Status: fiber.StatusForbidden},
	CodeLoginRequired:   {Status: fiber.StatusForbidden},

	CodeNotFound: {Status: fiber.StatusNotFound},

	CodeExportTooLarge: {Status: fiber.StatusRequestEntityTooLarge},

	CodeSettleRejected: {Status: fiber.StatusUnprocessableEntity},

	// 限流窗口是 1 分钟（limiter.Expiration），所以 Retry-After 取 60。
	// 取 60 而不是「平均剩余时间」：客户端拿到确定的退避上界即可。
	CodeRateLimited: {Status: fiber.StatusTooManyRequests, RetryAfter: "60"},

	CodeInternalError: {Status: fiber.StatusInternalServerError},

	// 5 秒 > 一次 pgx 连接重建/语句重试的成本，又短于用户耐心。
	CodeServiceUnavailable: {Status: fiber.StatusServiceUnavailable, RetryAfter: "5"},
	// 导出闸门是**排队**语义：持锁者通常几秒内完成，3 秒后重试即可。
	CodeExportBusy: {Status: fiber.StatusServiceUnavailable, RetryAfter: "3"},
	// 就绪探针被编排器调用，同样给一个可重试的间隔。
	CodeDBUnavailable: {Status: fiber.StatusServiceUnavailable, RetryAfter: "5"},
}

// nonContractCodes 是「状态码不固定」的对外码。
//
// 只有 http_error：它来自框架错误，状态随框架给出（404/405/413…）。
// 其余每个码都必须绑定唯一状态 —— 否则客户端无法按码分支。
var nonContractCodes = map[string]string{
	CodeHTTPError: "框架错误：状态由 *fiber.Error 给出（404/405/413…），码本身不绑定单一状态",
}

// contractOf 返回错误码的契约；未登记时 ok=false。
func contractOf(code string) (errorContract, bool) {
	c, ok := errorContracts[code]
	return c, ok
}

// Contract 供契约测试读取（只读视图）。
func Contract(code string) (status int, retryAfter string, ok bool) {
	c, ok := contractOf(code)
	return c.Status, c.RetryAfter, ok
}

// Contracts 返回契约表的副本，供测试遍历。
func Contracts() map[string]errorContractView {
	out := make(map[string]errorContractView, len(errorContracts))
	for k, v := range errorContracts {
		out[k] = errorContractView{Status: v.Status, RetryAfter: v.RetryAfter}
	}
	return out
}

// errorContractView 是契约的只读投影。
type errorContractView struct {
	Status     int
	RetryAfter string
}

// NonContractCodes 返回不绑定单一状态的码及理由（供测试做完整性核对）。
func NonContractCodes() map[string]string {
	out := make(map[string]string, len(nonContractCodes))
	for k, v := range nonContractCodes {
		out[k] = v
	}
	return out
}
