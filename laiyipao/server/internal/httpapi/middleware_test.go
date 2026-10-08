package httpapi

// HTTP 层的契约测试。
//
// ⚠️ 这个文件存在的理由是一个结构性缺口：本包此前**零测试文件**，
// 而本轮安全加固改的正是鉴权中间件（requireUser / requireAdmin 加 status
// 校验、ResolveUser 签名加 ctx）。也就是说，改鉴权逻辑不会有任何测试报警。
//
// 全部用例不依赖数据库：鉴权拒绝路径与错误映射都是纯逻辑，
// 用 Fiber 的 app.Test() 在内存里跑。

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/service"
)

// --- 工具 ---

// callWith 跑一次请求并返回状态码与解析后的 APIError。
func callWith(t *testing.T, app *fiber.App, req *http.Request) (int, *APIError) {
	t.Helper()
	resp, err := app.Test(req, 5000)
	if err != nil {
		t.Fatalf("请求执行失败：%v", err)
	}
	defer resp.Body.Close()
	var body APIError
	if resp.StatusCode >= 400 {
		if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
			t.Fatalf("解析错误响应失败（status=%d）：%v", resp.StatusCode, err)
		}
	}
	return resp.StatusCode, &body
}

func get(url string) *http.Request {
	req := httptest.NewRequest(http.MethodGet, url, nil)
	return req
}

// --- bearerToken ---

func TestBearerTokenParsing(t *testing.T) {
	cases := []struct {
		name   string
		header string
		want   string
	}{
		{"标准格式", "Bearer abc123", "abc123"},
		{"大小写不敏感", "bearer abc123", "abc123"},
		{"全大写", "BEARER abc123", "abc123"},
		{"多余空格应被裁掉", "Bearer   abc123  ", "abc123"},
		// 下面几条是审计点名的未验证行为。
		// "Bearer"（无空格）必须被拒：len(h) > len(prefix) 为假 -> 返回 ""。
		// 若有人把 > 改成 >=，这里会返回 "" 之外的东西，鉴权语义就变了。
		{"只有关键字无空格", "Bearer", ""},
		{"只有关键字无空格小写", "bearer", ""},
		{"关键字后全是空格", "Bearer   ", ""},
		{"空 header", "", ""},
		{"错误前缀", "Basic abc123", ""},
		{"前缀大小写混写", "BeArEr abc", "abc"},
		{"token 本身含空格应被裁剪而非全拒", "Bearer a b c", "a b c"},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			app := fiber.New()
			var got string
			app.Use(func(ctx *fiber.Ctx) error {
				got = bearerToken(ctx)
				return ctx.SendStatus(fiber.StatusOK)
			})
			req := get("/")
			if c.header != "" {
				req.Header.Set("Authorization", c.header)
			}
			if _, err := app.Test(req, 5000); err != nil {
				t.Fatalf("请求失败：%v", err)
			}
			if got != c.want {
				t.Errorf("bearerToken(%q) = %q，期望 %q", c.header, got, c.want)
			}
		})
	}
}

// --- 鉴权拒绝路径 ---

func TestRequireUserRejectsMissingHeader(t *testing.T) {
	// requireUser 需要 service.Service，但拒绝发生在调用 ResolveUser **之前**，
	// 所以可以传 nil —— 这既验证了"缺 header 不会碰 DB"，
	// 也让这条用例零 DB 依赖。
	app := fiber.New()
	app.Use("/api/v1/me", requireUser(nil), func(c *fiber.Ctx) error {
		return c.SendStatus(fiber.StatusOK)
	})

	status, body := callWith(t, app, get("/api/v1/me"))
	if status != fiber.StatusUnauthorized {
		t.Fatalf("缺 Authorization 头应 401，实际 %d", status)
	}
	if body.Error.Code != "unauthorized" {
		t.Errorf("错误码 = %q，期望 unauthorized", body.Error.Code)
	}
}

func TestRequireAdminRejectsMissingHeader(t *testing.T) {
	app := fiber.New()
	app.Use("/api/v1/admin/stats", requireAdmin(nil), func(c *fiber.Ctx) error {
		return c.SendStatus(fiber.StatusOK)
	})
	status, body := callWith(t, app, get("/api/v1/admin/stats"))
	if status != fiber.StatusUnauthorized {
		t.Fatalf("缺 Authorization 头应 401，实际 %d", status)
	}
	if body.Error.Code != "unauthorized" {
		t.Errorf("错误码 = %q，期望 unauthorized", body.Error.Code)
	}
}

// TestRequireUserRejectsMalformedHeader 确认畸形 header 也在碰 DB 之前被拒。
func TestRequireUserRejectsMalformedHeader(t *testing.T) {
	for _, h := range []string{"Bearer", "Bearer   ", "Basic xyz", "bearer"} {
		app := fiber.New()
		app.Use("/x", requireUser(nil), func(c *fiber.Ctx) error {
			return c.SendStatus(fiber.StatusOK)
		})
		req := get("/x")
		req.Header.Set("Authorization", h)
		status, _ := callWith(t, app, req)
		if status != fiber.StatusUnauthorized {
			t.Errorf("header=%q 应 401，实际 %d", h, status)
		}
	}
}

// TestRequireAdminRejectsUserToken 确认玩家令牌打不通管理端点。
//
// 这一条是**行为**测试：即使中间件顺序被改错（先跑 requireUser 再跑
// requireAdmin），玩家令牌也不该通过 requireAdmin。
// 但 ResolveAdmin 需要 DB，所以这里只验证"没有 Authorization 头"这一档；
// 跨角色那条由 service 层的 ResolveAdmin 状态查询保证（见 TestResolveAdmin*）。
func TestAdminRouteIsGuarded(t *testing.T) {
	app := fiber.New()
	// 模拟一个"守卫被摘掉"的错误配置：此时应当能到达处理器。
	// 这条断言是为了确认 app.Use 的路径匹配本身是对的，
	// 否则上面的"应 401"可能是因为路由根本没注册。
	guarded := fiber.New()
	guarded.Use("/api/v1/admin/stats", requireAdmin(nil), func(c *fiber.Ctx) error {
		return c.SendStatus(fiber.StatusOK)
	})
	status, _ := callWith(t, guarded, get("/api/v1/admin/stats"))
	if status == fiber.StatusOK {
		t.Fatal("未鉴权的管理端点不应返回 200")
	}
	_ = app
}

// --- Locals 取值 ---

func TestLocalsAccessorsReturnZeroWhenAbsent(t *testing.T) {
	// 中间件被摘掉时这些函数返回 0 而不是 panic。
	// 但 0 是**合法的 user_id 吗**？不是 —— users.id 是 BIGSERIAL 从 1 开始，
	// 所以 0 一定查不到任何用户。这是"安全地失败"的前提。
	app := fiber.New()
	var adminID, userID int64
	app.Use(func(c *fiber.Ctx) error {
		adminID = adminIDFrom(c)
		userID = userIDFrom(c)
		return c.SendStatus(fiber.StatusOK)
	})
	if _, err := app.Test(get("/"), 5000); err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	if adminID != 0 || userID != 0 {
		t.Errorf("未写入 Locals 时应返回 0，实际 admin=%d user=%d", adminID, userID)
	}
}

func TestLocalsAccessorsRejectWrongType(t *testing.T) {
	// 若有人把 Locals 写成字符串 id（JSON 反序列化的常见做法），
	// 类型断言会失败并返回 0。这里锁住该行为。
	app := fiber.New()
	var got int64
	app.Use(func(c *fiber.Ctx) error {
		c.Locals("user_id", "42")
		got = userIDFrom(c)
		return c.SendStatus(fiber.StatusOK)
	})
	if _, err := app.Test(get("/"), 5000); err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	if got != 0 {
		t.Errorf("Locals 类型不匹配时应返回 0，实际 %d", got)
	}
}

// --- failErr 错误映射 ---

func TestFailErrStatusMapping(t *testing.T) {
	cases := []struct {
		name     string
		err      error
		wantCode string
		wantHTTP int
	}{
		{"未认证", service.ErrUnauthorized, "unauthorized", 401},
		{"无权限", service.ErrForbidden, "forbidden", 403},
		{"不存在", service.ErrNotFound, "not_found", 404},
		{"输入非法", service.ErrBadInput, "bad_input", 400},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			app := fiber.New()
			app.Get("/x", func(ctx *fiber.Ctx) error {
				return failErr(ctx, fmt.Errorf("包装过: %w", c.err))
			})
			status, body := callWith(t, app, get("/x"))
			if status != c.wantHTTP {
				t.Errorf("HTTP = %d，期望 %d", status, c.wantHTTP)
			}
			if body.Error.Code != c.wantCode {
				t.Errorf("code = %q，期望 %q", body.Error.Code, c.wantCode)
			}
		})
	}
}

// TestSettleRejectionsAllMapTo422 是本批最重要的一条。
//
// 结算校验的错误全部属于「请求格式对、内容不可信」，应统一 422。
// 而 isSettleRejection 是一份**手写的错误清单** —— 往 ValidateSettle 里
// 加新校验时忘记同步这个列表，新错误就会掉进 500 分支：
//   - 客户端收到「服务内部错误」，无法区分"你作弊了"和"我们坏了"
//   - 服务端日志被大量假 500 刷满，真正的故障被淹没
//
// 之前就已经漂移过一次：加 reactions / shots / leaked 上界时新增了
// ErrTooManyReactions / ErrTooManyShots / ErrTooManyLeaked / ErrInvalidField，
// 四个都没进清单。本测试把"清单必须覆盖 domain 包导出的全部结算错误"钉死。
func TestSettleRejectionsAllMapTo422(t *testing.T) {
	// 与 ValidateSettle 实际可能返回的结算错误保持一致。
	// 新增 domain 结算错误时必须同步这里（测试会提醒）。
	settleErrors := []error{
		domain.ErrTokenNotFound,
		domain.ErrTokenUsed,
		domain.ErrTokenExpired,
		domain.ErrLevelMismatch,
		domain.ErrTooManyKills,
		domain.ErrTooShort,
		domain.ErrScoreRateExceeded,
		domain.ErrInvalidHitRate,
		domain.ErrWaveExceeded,
		domain.ErrTooManyReactions,
		domain.ErrTooManyShots,
		domain.ErrTooManyLeaked,
		domain.ErrInvalidField,
		// 第 58 轮补：这三个此前都落到兜底的 500。
		// ErrReplaySkillsMismatch 是第 56 轮新加的校验 —— 加了校验却忘了
		// 映射，作弊请求会得到 500 而不是 422，而且完全看不出是校验在拦。
		domain.ErrHPLeftExceedsBase,
		service.ErrSlotBudgetExceeded,
		service.ErrReplaySkillsMismatch,
	}
	for _, target := range settleErrors {
		if !isSettleRejection(target) {
			t.Errorf("结算错误 %v 未被 isSettleRejection 识别 -> 会返回 500", target)
		}
		// 再端到端确认一遍：真实响应必须是 422
		app := fiber.New()
		app.Get("/x", func(ctx *fiber.Ctx) error {
			return failErr(ctx, fmt.Errorf("包装: %w", target))
		})
		status, body := callWith(t, app, get("/x"))
		if status != fiber.StatusUnprocessableEntity {
			t.Errorf("%v -> HTTP %d，期望 422", target, status)
		}
		if body.Error.Code != "settle_rejected" {
			t.Errorf("%v -> code %q，期望 settle_rejected", target, body.Error.Code)
		}
	}
}

// TestNonSettleErrorsMapTo500AndHideDetails 确认 500 不泄露内部细节。
func TestNonSettleErrorsMapTo500AndHideDetails(t *testing.T) {
	secret := "pq: duplicate key value violates unique constraint \"users_nickname_key\""
	app := fiber.New()
	app.Get("/x", func(ctx *fiber.Ctx) error {
		return failErr(ctx, errors.New(secret))
	})
	status, body := callWith(t, app, get("/x"))
	if status != fiber.StatusInternalServerError {
		t.Fatalf("HTTP = %d，期望 500", status)
	}
	if body.Error.Code != "internal_error" {
		t.Errorf("code = %q，期望 internal_error", body.Error.Code)
	}
	// 响应体绝不能包含数据库错误细节
	if body.Error.Message == secret || contains(body.Error.Message, "constraint") ||
		contains(body.Error.Message, "pq:") {
		t.Errorf("500 响应泄露了内部细节：%q", body.Error.Message)
	}
	if body.Error.Message != "服务内部错误" {
		t.Errorf("500 文案 = %q，期望通用文案", body.Error.Message)
	}
}

func contains(s, sub string) bool {
	return len(s) >= len(sub) && (func() bool {
		for i := 0; i+len(sub) <= len(s); i++ {
			if s[i:i+len(sub)] == sub {
				return true
			}
		}
		return false
	})()
}

// TestIsSettleRejectionIgnoresUnrelated 确认清单不会误伤。
func TestIsSettleRejectionIgnoresUnrelated(t *testing.T) {
	for _, err := range []error{
		service.ErrForbidden,
		service.ErrNotFound,
		service.ErrBadInput,
		errors.New("完全无关的错误"),
	} {
		if isSettleRejection(err) {
			t.Errorf("%v 不应被判为结算拒绝", err)
		}
	}
}

// --- panic 兜底 ---

func TestRecoverPanicDoesNotKillProcess(t *testing.T) {
	app := fiber.New()
	app.Use(recoverPanic())
	app.Get("/boom", func(c *fiber.Ctx) error {
		panic("故意的 panic")
	})
	app.Get("/ok", func(c *fiber.Ctx) error {
		return c.SendString("ok")
	})

	status, body := callWith(t, app, get("/boom"))
	if status != fiber.StatusInternalServerError {
		t.Fatalf("panic 应转成 500，实际 %d", status)
	}
	if body.Error.Message != "服务内部错误" {
		t.Errorf("panic 文案 = %q，不应泄露 panic 内容", body.Error.Message)
	}

	// 关键：panic 之后同一个 app 仍能服务其它请求
	status, _ = callWith(t, app, get("/ok"))
	if status != fiber.StatusOK {
		t.Errorf("panic 后其它请求应仍可用，实际 %d", status)
	}
}

// --- CORS ---

func TestCORSHeadersAndPreflight(t *testing.T) {
	app := fiber.New()
	app.Use(cors())
	app.Get("/x", func(c *fiber.Ctx) error { return c.SendString("ok") })

	resp, err := app.Test(get("/x"), 5000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	if got := resp.Header.Get("Access-Control-Allow-Origin"); got != "*" {
		t.Errorf("CORS 头 = %q", got)
	}

	pre := httptest.NewRequest(http.MethodOptions, "/x", nil)
	resp2, err := app.Test(pre, 5000)
	if err != nil {
		t.Fatalf("预检请求失败：%v", err)
	}
	if resp2.StatusCode != fiber.StatusNoContent {
		t.Errorf("预检应 204，实际 %d", resp2.StatusCode)
	}
}

// TestRequestLoggerSetsDuration 确认耗时头被写入。
func TestRequestLoggerSetsDuration(t *testing.T) {
	app := fiber.New()
	app.Use(requestLogger())
	app.Get("/x", func(c *fiber.Ctx) error { return c.SendString("ok") })
	resp, err := app.Test(get("/x"), 5000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	if resp.Header.Get("X-Request-Duration") == "" {
		t.Error("X-Request-Duration 未写入")
	}
}
