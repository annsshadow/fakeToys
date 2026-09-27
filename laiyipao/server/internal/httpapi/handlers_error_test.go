package httpapi

// 「连接池已关闭」环境下的全 handler 错误分支覆盖。
//
// 为什么用关闭的池：handler 的 failErr 分支只在 service 层出错时走到。
// 真实环境里这些分支对应"数据库挂了"——不可能在共享开发库上人为制造，
// 但把 pool 关掉就能**确定性地**让每个 handler 走到自己的错误出口。
// 这不是 mock：整条真实代码路径都执行，只是数据库处于故障态。
//
// 鉴权中间件被跳过（Locals 直接注入身份），因为 requireUser/requireAdmin
// 的拒绝与放行路径已由 middleware_test.go 与 E2E 用例覆盖；
// 这里只负责让 handler 体执行到 failErr。

import (
	"context"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/service"
	"github.com/laiyipao/server/internal/store"
)

// newBrokenService 开一个真库连接后立刻关闭池，返回"故障态" Service。
func newBrokenService(t *testing.T) *service.Service {
	t.Helper()
	dsn := os.Getenv("TEST_DATABASE_URL")
	if dsn == "" {
		dsn = "postgres://postgres@127.0.0.1:5432/laiyipao?sslmode=disable"
	}
	cfg := config.Load()
	cfg.DatabaseURL = dsn
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()
	db, err := store.Open(ctx, cfg)
	if err != nil {
		t.Skipf("无可用测试数据库，跳过：%v", err)
	}
	svc := service.New(db, cfg)
	db.Close() // 此后所有查询确定性失败
	return svc
}

// callAs 向 app 发请求，返回状态码。
func callAs(t *testing.T, app *fiber.App, method, path, body string) int {
	t.Helper()
	var req *http.Request
	if body != "" {
		req = httptest.NewRequest(method, path, strings.NewReader(body))
		req.Header.Set("Content-Type", "application/json")
	} else {
		req = httptest.NewRequest(method, path, nil)
	}
	resp, err := app.Test(req, 5000)
	if err != nil {
		t.Fatalf("%s %s 执行失败：%v", method, path, err)
	}
	resp.Body.Close()
	return resp.StatusCode
}

func TestHandlerErrorBranchesOnBrokenPool(t *testing.T) {
	svc := newBrokenService(t)
	s := New(svc)

	// 每条 = 一个 handler。请求形状合法（参数解析通过），
	// service 调用必然失败 → 断言期望状态码。
	// 默认 500；个别 service 把任意错误包装成哨兵错误（failErr 据此映射）：
	//   refresh / adminMe → ErrUnauthorized → 401
	//   getReplay / adminBattleDetail → ErrNotFound → 404
	// want=0 表示取默认 500。
	// hasID 为 true 的 handler 读 c.Params("id")，路由须注册 ":id" 参数。
	// ⚠️ getConfig 不在清单里：LoadGameConfig 纯内存计算永不返回错误，
	// 它的 failErr 分支不可达（豁免，见报告）。
	cases := []struct {
		name   string
		method string
		path   string
		body   string
		hasID  bool
		want   int
		run    func(c *fiber.Ctx) error
	}{
		{"guestLogin", "POST", "/x", `{"guest_token":"g","nickname":"n"}`, false, 0, s.guestLogin},
		{"refresh", "POST", "/x", `{"refresh_token":"r"}`, false, fiber.StatusUnauthorized, s.refresh},
		{"me", "GET", "/x", "", false, 0, s.me},
		{"getLoadout", "GET", "/x", "", false, 0, s.getLoadout},
		{"saveLoadout", "PUT", "/x", `{"skill_ids":[1,2]}`, false, 0, s.saveLoadout},
		{"upgradeSkill", "POST", "/x/1", "", true, 0, s.upgradeSkill},
		{"wallet", "GET", "/x", "", false, 0, s.wallet},
		{"startBattle", "POST", "/x", `{"level_id":1}`, false, 0, s.startBattle},
		{"settleBattle", "POST", "/x", `{"token_id":1,"result":"lose","duration_ms":30000}`, false, 0, s.settleBattle},
		{"getReplay", "GET", "/x/1", "", true, fiber.StatusNotFound, s.getReplay},
		{"verifyReplay", "POST", "/x", `{"battle_id":1,"replay_hash":"x"}`, false, 0, s.verifyReplay},
		{"leaderboard", "GET", "/x", "", false, 0, s.leaderboard},
		{"getMastery", "GET", "/x", "", false, 0, s.getMastery},
		{"allocateMastery", "POST", "/x", `{"node_id":1}`, false, 0, s.allocateMastery},
		{"tasks", "GET", "/x", "", false, 0, s.tasks},
		{"claimTask", "POST", "/x/1", "", true, 0, s.claimTask},
		{"signIn", "POST", "/x", "", false, 0, s.signIn},
		{"shop", "GET", "/x", "", false, 0, s.shop},
		{"buy", "POST", "/x/1", "", true, 0, s.buy},
		{"redeem", "POST", "/x", `{"code":"X"}`, false, 0, s.redeem},
		{"diagnose", "GET", "/x", "", false, 0, s.diagnose},
		{"listDefenses", "GET", "/x", "", false, 0, s.listDefenses},
		{"saveDefense", "POST", "/x", `{"name":"x","skills":[1]}`, false, 0, s.saveDefense},
		{"challengeDefense", "POST", "/x/1", `{"won":true}`, true, 0, s.challengeDefense},
		// 管理端
		{"adminLogin", "POST", "/x", `{"username":"a","password":"b"}`, false, 0, s.adminLogin},
		{"adminMe", "GET", "/x", "", false, fiber.StatusUnauthorized, s.adminMe},
		{"adminDashboard", "GET", "/x", "", false, 0, s.adminDashboard},
		{"adminLevels", "GET", "/x", "", false, 0, s.adminLevels},
		{"adminLevelWaves", "GET", "/x/1", "", true, 0, s.adminLevelWaves},
		{"adminUpdateLevel", "PUT", "/x/1", `{"name":"x"}`, true, 0, s.adminUpdateLevel},
		{"adminRegenerateLevels", "POST", "/x", "", false, 0, s.adminRegenerateLevels},
		{"adminSkills", "GET", "/x", "", false, 0, s.adminSkills},
		{"adminUpdateSkill", "PUT", "/x/1", `{"name":"x"}`, true, 0, s.adminUpdateSkill},
		{"adminEquipment", "GET", "/x", "", false, 0, s.adminEquipment},
		{"adminUsers", "GET", "/x", "", false, 0, s.adminUsers},
		{"adminBanUser", "POST", "/x/1", "", true, 0, s.adminBanUser},
		{"adminUnbanUser", "POST", "/x/1", "", true, 0, s.adminUnbanUser},
		{"adminGrantUser", "POST", "/x/1", `{"currency":"coin","amount":1}`, true, 0, s.adminGrantUser},
		{"adminBattles", "GET", "/x", "", false, 0, s.adminBattles},
		{"adminBattleDetail", "GET", "/x/1", "", true, fiber.StatusNotFound, s.adminBattleDetail},
		{"adminDefenses", "GET", "/x", "", false, 0, s.adminDefenses},
		{"adminEconomy", "GET", "/x", "", false, 0, s.adminEconomy},
		{"adminUpdateShop", "PUT", "/x/1", `{"limit_per_day":1}`, true, 0, s.adminUpdateShop},
		{"adminAnnouncements", "GET", "/x", "", false, 0, s.adminAnnouncements},
		{"adminCreateAnnouncement", "POST", "/x", `{"title":"t","body":"b"}`, false, 0, s.adminCreateAnnouncement},
		{"adminRedeemCodes", "GET", "/x", "", false, 0, s.adminRedeemCodes},
		{"adminCreateRedeemCode", "POST", "/x", `{"code":"C","reward":{"coin":1}}`, false, 0, s.adminCreateRedeemCode},
		{"adminAuditLogs", "GET", "/x", "", false, 0, s.adminAuditLogs},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			sub := fiber.New(fiber.Config{DisableStartupMessage: true})
			sub.Use(func(c *fiber.Ctx) error {
				// 身份注入：值不必真实（池已关，查询必失败），
				// 但必须是 int64 以通过 userIDFrom/adminIDFrom 的类型断言。
				c.Locals("user_id", int64(1))
				c.Locals("admin_id", int64(1))
				return c.Next()
			})
			route := tc.path
			if tc.hasID {
				// handler 读 c.Params("id")：路由必须是参数段，
				// 注册成字面量 /x/1 会让 Params 取到空串 → 400 而不是目标分支
				route = "/x/:id"
			}
			switch tc.method {
			case "POST":
				sub.Post(route, tc.run)
			case "PUT":
				sub.Put(route, tc.run)
			default:
				sub.Get(route, tc.run)
			}
			want := tc.want
			if want == 0 {
				want = fiber.StatusInternalServerError
			}
			status := callAs(t, sub, tc.method, tc.path, tc.body)
			if status != want {
				t.Errorf("%s 在数据库故障态应 %d，实际 %d", tc.name, want, status)
			}
		})
	}

	// readyz 在真实 app 上的故障分支（healthz 不碰库，无需故障态）
	app := fiber.New(fiber.Config{DisableStartupMessage: true})
	New(svc).Register(app)
	if got := callAs(t, app, "GET", "/readyz", ""); got != fiber.StatusServiceUnavailable {
		t.Errorf("池关闭时 readyz 应 503，实际 %d", got)
	}
}
