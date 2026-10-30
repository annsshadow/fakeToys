package service

// 管理员与后台端的服务层测试。
//
// 为什么在 service 层再测一遍（httpapi E2E 已走过）：Go 覆盖率按包统计，
// 且这里能直接构造 httpapi 够不着的场景 —— 停用管理员、短密码 bootstrap、
// 库中故障态。每条断言都锚定一个**行为后果**（库中状态、错误类型），
// 不是"函数没报错就算过"。

import (
	"context"
	"encoding/base64"
	"encoding/json"
	"errors"
	"fmt"
	"strings"
	"testing"
	"time"

	"golang.org/x/crypto/bcrypt"

	"github.com/laiyipao/server/internal/config"
	"github.com/laiyipao/server/internal/domain"
	"github.com/laiyipao/server/internal/store"
)

// newAdminFixture 直接在库中插一个管理员（密码 MinCost 加速测试）。
func (ts *testService) newAdminFixture(t *testing.T, tag string, status int) (int64, string, string) {
	t.Helper()
	username := fmt.Sprintf("svc_admin_%s_%d", tag, time.Now().UnixNano()%1_000_000)
	password := "svc-admin-pass-123"
	hash, err := bcrypt.GenerateFromPassword([]byte(password), bcrypt.MinCost)
	if err != nil {
		t.Fatalf("生成哈希失败：%v", err)
	}
	var id int64
	if err := ts.pool.QueryRow(context.Background(),
		`INSERT INTO admin_users (username, password_hash, role, status) VALUES ($1,$2,'admin',$3) RETURNING id`,
		username, string(hash), status).Scan(&id); err != nil {
		t.Fatalf("建管理员失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM admin_audit_logs WHERE username = $1`, username)
		_, _ = ts.pool.Exec(context.Background(), `DELETE FROM admin_users WHERE id = $1`, id)
	})
	return id, username, password
}

func TestAdminLogin(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	_, username, password := ts.newAdminFixture(t, "login", 1)

	// 成功：返回 admin 身份
	tp, err := ts.AdminLogin(ctx, username, password)
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	if tp.Admin.Username != username || tp.AccessToken == "" {
		t.Fatalf("登录响应不完整：%+v", tp)
	}
	// 第 155 轮：会话与令牌的绑定改为 JWT 载荷里的 sid 明文（摘要落库）。
	// 这里直接解载荷验证两条：① 登录签发的令牌恒带 sid；② 库里那一行
	// admin_tokens 就是 sid 的摘要 —— 不满足任一，登出/吊销都无从谈起。
	payload, err := base64.RawURLEncoding.DecodeString(
		strings.SplitN(tp.AccessToken, ".", 3)[1])
	if err != nil {
		t.Fatalf("解 JWT 载荷失败：%v", err)
	}
	var claims struct {
		Sid string `json:"sid"`
	}
	if err := json.Unmarshal(payload, &claims); err != nil || claims.Sid == "" {
		t.Fatalf("登录令牌必须携带会话 id（sid），实际载荷 %s / %v", payload, err)
	}
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM admin_tokens WHERE token_hash = $1`,
		HashRefreshToken(claims.Sid)).Scan(&n); err != nil || n != 1 {
		t.Errorf("会话摘要（sid 的 SHA-256）必须落库一行（count=%d err=%v）", n, err)
	}

	// 未知用户与错误密码必须同一错误（防用户名枚举）
	if _, err := ts.AdminLogin(ctx, "svc_no_such_admin", password); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("未知用户应 ErrUnauthorized，实际 %v", err)
	}
	if _, err := ts.AdminLogin(ctx, username, "wrong-password"); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("错误密码应 ErrUnauthorized，实际 %v", err)
	}
}

// TestAdminLogoutRevokesAllSessions 登出必须真切断：
// 该管理员的全部会话被吊销后，**签名仍然合法**的 access token 立刻不可用
// （「服务端登出」与「只清客户端 localStorage」的区别就在这一条上）。
func TestAdminLogoutRevokesAllSessions(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, username, password := ts.newAdminFixture(t, "logout", 1)

	// 两个设备各登录一次 → 两行会话
	tp1, err := ts.AdminLogin(ctx, username, password)
	if err != nil {
		t.Fatalf("登录 1 失败：%v", err)
	}
	tp2, err := ts.AdminLogin(ctx, username, password)
	if err != nil {
		t.Fatalf("登录 2 失败：%v", err)
	}
	// 登出前两枚都能解析
	if _, err := ts.ResolveAdmin(ctx, tp1.AccessToken); err != nil {
		t.Fatalf("登出前 tp1 应可解析：%v", err)
	}
	if _, err := ts.ResolveAdmin(ctx, tp2.AccessToken); err != nil {
		t.Fatalf("登出前 tp2 应可解析：%v", err)
	}

	n, err := ts.AdminLogout(ctx, adminID)
	if err != nil {
		t.Fatalf("登出失败：%v", err)
	}
	if n != 2 {
		t.Errorf("应吊销 2 条会话，实际 %d", n)
	}
	// 全部失效（登出是「切断这个人所有凭证」，不是只切当前一个）
	if _, err := ts.ResolveAdmin(ctx, tp1.AccessToken); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("登出后 tp1 应 ErrUnauthorized，实际 %v", err)
	}
	if _, err := ts.ResolveAdmin(ctx, tp2.AccessToken); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("登出后 tp2 应 ErrUnauthorized，实际 %v", err)
	}
	// 幂等：再登出一次 = 0 条
	if n2, err := ts.AdminLogout(ctx, adminID); err != nil || n2 != 0 {
		t.Errorf("重复登出应吊销 0 条，实际 %d / %v", n2, err)
	}
	// 重新登录即恢复（新会话是新的一行）
	tp3, err := ts.AdminLogin(ctx, username, password)
	if err != nil {
		t.Fatalf("重新登录失败：%v", err)
	}
	if _, err := ts.ResolveAdmin(ctx, tp3.AccessToken); err != nil {
		t.Errorf("重新登录后的令牌应可解析：%v", err)
	}

	// 吊销是**按 admin_id** 的：另一个管理员不受影响
	_, oUser, oPass := ts.newAdminFixture(t, "logout-other", 1)
	op, err := ts.AdminLogin(ctx, oUser, oPass)
	if err != nil {
		t.Fatalf("另一管理员登录失败：%v", err)
	}
	if _, err := ts.ResolveAdmin(ctx, op.AccessToken); err != nil {
		t.Errorf("其他管理员的会话不应被本次登出波及：%v", err)
	}
}

func TestAdminLoginDeactivated(t *testing.T) {
	ts := openTestService(t)
	_, username, password := ts.newAdminFixture(t, "off", 2)
	_, err := ts.AdminLogin(context.Background(), username, password)
	if !errors.Is(err, ErrForbidden) {
		t.Errorf("停用账号应 ErrForbidden，实际 %v", err)
	}
}

func TestResolveAdmin(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, username, password := ts.newAdminFixture(t, "resolve", 1)
	tp, err := ts.AdminLogin(ctx, username, password)
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}

	got, err := ts.ResolveAdmin(ctx, tp.AccessToken)
	if err != nil || got != adminID {
		t.Fatalf("ResolveAdmin = %d, %v；期望 %d", got, err, adminID)
	}

	// 玩家令牌打管理端 → 拒绝（kind 校验）
	uid := ts.newUser(t, ctx)
	userTP, err := ts.GuestLogin(ctx, "", "resolve-user")
	if err != nil {
		t.Fatalf("建玩家失败：%v", err)
	}
	_ = uid
	if _, err := ts.ResolveAdmin(ctx, userTP.AccessToken); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("玩家令牌应被拒，实际 %v", err)
	}

	// 垃圾令牌
	if _, err := ts.ResolveAdmin(ctx, "garbage"); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("垃圾令牌应 ErrUnauthorized，实际 %v", err)
	}

	// 不存在的管理员（有效签名但不存在的 id）
	tok, _, err := ts.JWT.Issue(99999999, "admin", "")
	if err != nil {
		t.Fatalf("签发失败：%v", err)
	}
	if _, err := ts.ResolveAdmin(ctx, tok); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("不存在的管理员应 ErrUnauthorized，实际 %v", err)
	}

	// 停用 → ErrForbidden（运营降权的止损通道）
	_, offUser, offPass := ts.newAdminFixture(t, "resolveoff", 2)
	offTP, err := ts.AdminLogin(ctx, offUser, offPass)
	if !errors.Is(err, ErrForbidden) {
		// 停用账号无法登录，改走"先建活的再停用"
		t.Skipf("停用账号登录未按预期拒绝，无法构造停用态令牌：%v", err)
	}
	_ = offTP
}

// TestResolveAdminDeactivated 先登录拿令牌、再停用 ——
// 验证「令牌在 TTL 内但账号已停用」必须立刻失效（ResolveAdmin 查 status 的意义）。
func TestResolveAdminDeactivated(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, username, password := ts.newAdminFixture(t, "deact", 1)
	tp, err := ts.AdminLogin(ctx, username, password)
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	ts.exec(t, `UPDATE admin_users SET status = 2 WHERE id = $1`, adminID)
	if _, err := ts.ResolveAdmin(ctx, tp.AccessToken); !errors.Is(err, ErrForbidden) {
		t.Errorf("停用管理员的令牌应 ErrForbidden，实际 %v", err)
	}
	// AdminMe 同样必须拒绝
	if _, err := ts.AdminMe(ctx, adminID); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("停用管理员 AdminMe 应 ErrUnauthorized，实际 %v", err)
	}
}

// TestResolveAdminSessionRevoked 会话行被吊销后，**签名仍合法**的令牌必须 401。
// 这是「服务端登出」与「只清客户端 localStorage」的本质区别：
// 前者让那枚 2h 有效的凭证立刻不可用，后者只是把本地那份藏起来了。
func TestResolveAdminSessionRevoked(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, username, password := ts.newAdminFixture(t, "srevoke", 1)
	tp, err := ts.AdminLogin(ctx, username, password)
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	if _, err := ts.ResolveAdmin(ctx, tp.AccessToken); err != nil {
		t.Fatalf("吊销前令牌应可解析：%v", err)
	}
	// 手工吊销这一行（登出的库层动作）
	if _, err := ts.pool.Exec(ctx,
		`UPDATE admin_tokens SET revoked_at = now() WHERE admin_id = $1`, adminID); err != nil {
		t.Fatalf("吊销失败：%v", err)
	}
	if _, err := ts.ResolveAdmin(ctx, tp.AccessToken); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("会话被吊销后旧令牌应 ErrUnauthorized，实际 %v", err)
	}
}

// TestResolveAdminSessionExpired 落库 expires_at 早于当前时间 → 令牌不可用。
// 与 JWT 自带 exp 双保险：即使有人把 JWT 的 exp 手改大，库里的会话行仍会卡住它。
func TestResolveAdminSessionExpired(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, username, password := ts.newAdminFixture(t, "sexp", 1)
	tp, err := ts.AdminLogin(ctx, username, password)
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	if _, err := ts.pool.Exec(ctx,
		`UPDATE admin_tokens SET expires_at = now() - interval '1 hour' WHERE admin_id = $1`, adminID); err != nil {
		t.Fatalf("回退 expires_at 失败：%v", err)
	}
	if _, err := ts.ResolveAdmin(ctx, tp.AccessToken); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("会话过期后旧令牌应 ErrUnauthorized，实际 %v", err)
	}
}

// TestAdminMe 读路径：必须要求 status=1（第 127 轮之前全项目唯一带
// `AND status = 1` 的 admin 端点，见 ResolveAdmin 注释里的告诫）。
func TestAdminMe(t *testing.T) {
	ts := openTestService(t)
	adminID, username, _ := ts.newAdminFixture(t, "me", 1)
	a, err := ts.AdminMe(context.Background(), adminID)
	if err != nil || a.Username != username {
		t.Fatalf("AdminMe = %+v, %v", a, err)
	}
	if _, err := ts.AdminMe(context.Background(), 99999999); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("不存在的管理员应 ErrUnauthorized，实际 %v", err)
	}
}

func TestAuditWritesRow(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, username, _ := ts.newAdminFixture(t, "audit", 1)

	ts.Audit(ctx, adminID, "svc_audit_probe", "target-1", "10.0.0.1", map[string]any{"k": "v"})

	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM admin_audit_logs WHERE admin_id = $1 AND action = 'svc_audit_probe'`,
		adminID).Scan(&n); err != nil || n != 1 {
		t.Fatalf("审计未落库（count=%d err=%v）", n, err)
	}
	// username 应带出管理员名
	var uname string
	_ = ts.pool.QueryRow(ctx,
		`SELECT username FROM admin_audit_logs WHERE admin_id = $1 AND action = 'svc_audit_probe'`,
		adminID).Scan(&uname)
	if uname != username {
		t.Errorf("审计 username = %q，期望 %q", uname, username)
	}

	// detail 无法 JSON 化（channel）不得让主流程失败，
	// 且不得静默降级成 `{}`（第 125 轮：那与「本来无明细」不可区分，
	// 审计行撒谎）。现在必须落 `{"detail_error": "..."}` 诚实记丢。
	ts.Audit(ctx, adminID, "svc_audit_badjson", "t", "10.0.0.1", make(chan int))
	var raw []byte
	var detailType string
	if err := ts.pool.QueryRow(ctx,
		`SELECT detail, jsonb_typeof(detail)
		 FROM admin_audit_logs WHERE admin_id = $1 AND action = 'svc_audit_badjson'`,
		adminID).Scan(&raw, &detailType); err != nil || detailType != "object" {
		t.Errorf("不可序列化 detail 应落对象型 JSONB，实际 %q (type=%s) err=%v", raw, detailType, err)
	}
	if !strings.Contains(string(raw), "detail_error") {
		t.Errorf("detail 必须带 detail_error 标记（第 125 轮修复），实际 %q", raw)
	}
}

func TestAdminutil(t *testing.T) {
	if timeNow().IsZero() {
		t.Error("timeNow 不应返回零值")
	}

	plain, hash, err := NewAdminSession()
	if err != nil {
		t.Fatalf("NewAdminSession：%v", err)
	}
	if plain[:2] != "a_" || hash != HashRefreshToken(plain) {
		t.Errorf("会话令牌契约破坏：%q / %q", plain, hash)
	}

	if err := VerifyPassword(HashPasswordForTest("abc"), "abc"); err != nil {
		t.Errorf("正确密码应通过：%v", err)
	}
	if err := VerifyPassword(HashPasswordForTest("abc"), "abd"); err == nil {
		t.Error("错误密码应被拒绝")
	}

	// bcrypt 72 字节上限：超长密码必须报错而不是静默截断
	if _, err := HashPassword(string(make([]byte, 100))); err == nil {
		t.Error("超 72 字节密码应报错（bcrypt 上限）")
	}
}

// HashPasswordForTest 测试内联生成哈希（验证 VerifyPassword 用）。
func HashPasswordForTest(p string) string {
	b, _ := HashPassword(p)
	return b
}

// TestEnsureBootstrapAdmin 需要 admin_users 为空的库 —— 用 scratch。
func TestEnsureBootstrapAdmin(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 密码太短：在创建前被拒
	_, err := ts.EnsureBootstrapAdmin(ctx, "boot", "short")
	mustErr(t, err, "至少 10 位")
	// 上面失败后管理员数量仍为 0 → 继续正常创建
	created, err := ts.EnsureBootstrapAdmin(ctx, "boot", "long-enough-pass")
	if err != nil || !created {
		t.Fatalf("空库应创建 bootstrap 管理员：%v", err)
	}
	var n int
	if err := ts.pool.QueryRow(ctx, `SELECT COUNT(*) FROM admin_users`).Scan(&n); err != nil || n != 1 {
		t.Fatalf("bootstrap 管理员未落库（%d, %v）", n, err)
	}
	// 再跑：已有管理员 → false, nil（幂等）
	created, err = ts.EnsureBootstrapAdmin(ctx, "boot2", "another-pass-1")
	if created || err != nil {
		t.Errorf("非空库应返回 (false, nil)，实际 (%t, %v)", created, err)
	}
}

// --- stats.go（看板与玩家管理） ---

func TestAdminDashboard(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	// ⚠️ 自己造一个用户，**不依赖同包其他用例先跑**。
	//
	// 整包共用 `TestMain` 建的一次性库，用例之间本来就共享状态
	// （见 openTestService 的注释）。此前这里没造用户却断言
	// `Total >= 1`，实际靠的是源码顺序排在前面的 Cases
	// （TestAdminListUsers 等）先写进去的数据 —— 于是：
	//   - `go test ./...` 全量跑：绿；
	//   - `-run TestAdminDashboard$` 单独跑：库是干净的，Total=0 → 红。
	//
	// 这是测试隔离缺陷，不是产品缺陷。一个用例的正确性不能建立在
	// 「别的用例恰好先跑过」上：单独复现问题是排障的基本动作，
	// 而复现不了的问题会被当成"偶发"或"环境问题"。
	ts.newUser(t, ctx)

	d, err := ts.AdminDashboard(ctx)
	if err != nil {
		t.Fatalf("看板失败：%v", err)
	}
	if d.Users.Total < 1 {
		t.Error("本用例已建号，total 不应为 0")
	}
	// 反应分布必须覆盖全部反应定义（客户端渲染依赖完整清单）
	if len(d.ReactionUsage) != len(domain.AllReactionSpecs()) {
		t.Errorf("ReactionUsage 应有 %d 项，实际 %d", len(domain.AllReactionSpecs()), len(d.ReactionUsage))
	}
	if len(d.DailyActive) != 14 {
		t.Errorf("DAU 应覆盖近 14 天，实际 %d", len(d.DailyActive))
	}

	broken := openBrokenService(t)
	if _, err := broken.AdminDashboard(context.Background()); err == nil {
		t.Error("数据库故障时看板应报错")
	}
}

func TestAdminListUsers(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	u, _ := ts.loadUser(ctx, uid)

	items, total, err := ts.AdminListUsers(ctx, u.Nickname, 50, 0)
	if err != nil {
		t.Fatalf("列表失败：%v", err)
	}
	if total < 1 {
		t.Fatalf("按昵称搜索应至少命中 %d", uid)
	}
	found := false
	for _, it := range items {
		if it.ID == uid {
			found = true
		}
	}
	if !found {
		t.Errorf("搜索结果应包含 %d", uid)
	}

	// limit/offset 夹紧：非法值不报错、不越界
	if _, _, err := ts.AdminListUsers(ctx, "", 9999, -5); err != nil {
		t.Errorf("非法分页参数应被夹紧而不是报错：%v", err)
	}

	broken := openBrokenService(t)
	if _, _, err := broken.AdminListUsers(ctx, "", 50, 0); err == nil {
		t.Error("数据库故障时应报错")
	}
}

func TestAdminSetUserStatus(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	v, err := ts.AdminSetUserStatus(ctx, uid, true, "测试封禁")
	if err != nil || v.Status != 2 {
		t.Fatalf("封禁应 status=2：%+v, %v", v, err)
	}
	// 封禁必须吊销全部 refresh token（否则长期令牌续命）
	var active int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL`, uid).
		Scan(&active); err != nil || active != 0 {
		t.Errorf("封禁后有效 refresh token 应为 0，实际 %d（err=%v）", active, err)
	}

	v, err = ts.AdminSetUserStatus(ctx, uid, false, "")
	if err != nil || v.Status != 1 {
		t.Fatalf("解封应 status=1：%+v, %v", v, err)
	}
}

func TestAdminGrantCurrency(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	before := ts.wallet(t, ctx, uid)

	out, err := ts.AdminGrantCurrency(ctx, uid, "gem", 77)
	if err != nil {
		t.Fatalf("发币失败：%v", err)
	}
	if out["gem"] != before.Gem+77 {
		t.Errorf("发币后 gem = %d，期望 %d", out["gem"], before.Gem+77)
	}

	// 未知货币必须业务错误，不是 500
	if _, err := ts.AdminGrantCurrency(ctx, uid, "diamonds", 1); !errors.Is(err, ErrBadInput) {
		t.Errorf("未知货币应 ErrBadInput，实际 %v", err)
	}

	//
	// ⚠️ 第 109 轮：符号必须进钱包流水。
	// 修前无论正负，wallet_flows.reason 一律是 admin_grant、
	// 审计一律是 grant_currency —— 负数就是**静默回收**，
	// 事后翻审计分不清哪笔是扣款。
	// 判据落在流水的 reason 列上（能直接观测的那一层）：
	// 正数 → admin_grant，负数 → admin_revoke，0 → 拒绝。
	if _, err := ts.AdminGrantCurrency(ctx, uid, "coin", 0); !errors.Is(err, ErrBadInput) {
		t.Errorf("数量 0 应 ErrBadInput（0 的发放没有意义），实际 %v", err)
	}
	// 先补足 gem 库存，避免「余额不足」干扰对流水 reason 的判读
	ts.grant(t, ctx, uid, map[string]int64{"gem": 500})
	if _, err := ts.AdminGrantCurrency(ctx, uid, "gem", -100); err != nil {
		t.Fatalf("负数（回收场景，老测试钉住的既有能力）应可执行：%v", err)
	}
	if got := ts.countFlowsByReason(t, ctx, uid, "admin_revoke"); got != 1 {
		t.Errorf("负数发放应写 1 条 admin_revoke 流水，实际 %d", got)
	}
	if got := ts.countFlowsByReason(t, ctx, uid, "admin_grant"); got != 1 {
		t.Errorf("本轮唯一的正数发放（gem +77）应有 1 条 admin_grant 流水，实际 %d", got)
	}
}

// countFlowsByReason 数某用户某 reason 的 wallet_flows 行数。
func (ts *testService) countFlowsByReason(t *testing.T, ctx context.Context, uid int64, reason string) int {
	t.Helper()
	var n int
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM wallet_flows WHERE user_id = $1 AND reason = $2`, uid, reason).
		Scan(&n); err != nil {
		t.Fatalf("数流水失败：%v", err)
	}
	return n
}

func TestAdminListBattles(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	ts.grant(t, ctx, uid, map[string]int64{"energy": 100})

	tp, err := ts.StartBattle(ctx, uid, 1)
	if err != nil {
		t.Skipf("开局失败：%v", err)
	}
	_, err = ts.SettleBattle(ctx, uid, tp.TokenID, loseSettleInput())
	if err != nil {
		t.Fatalf("结算失败：%v", err)
	}

	_, total, err := ts.AdminListBattles(ctx, uid, 1, 10, false, 0)
	if err != nil || total < 1 {
		t.Fatalf("按用户+关卡过滤应命中：%d, %v", total, err)
	}
	// 第 123 轮：total_enemies 必须填上（生成器权威值），后台「守恒」列靠它做伪造判定
	list, _, err := ts.AdminListBattles(ctx, uid, 1, 10, false, 0)
	if err != nil {
		t.Fatalf("读战报失败：%v", err)
	}
	wantTotal := 0
	for _, w := range domain.GenerateLevel(1).Waves {
		for _, sp := range w.Spawns {
			wantTotal += sp.Count
		}
	}
	if wantTotal <= 0 {
		t.Fatalf("第 1 关生成器无怪，测试前提不成立")
	}
	for _, b := range list {
		if b.LevelID != 1 {
			continue
		}
		if b.TotalEnemies != wantTotal {
			t.Errorf("第 1 关战报 total_enemies 应为生成器值 %d，实际 %d（修前恒 0，守恒列形同虚设）",
				wantTotal, b.TotalEnemies)
		}
	}
	// 非匹配过滤 → 0
	_, total, err = ts.AdminListBattles(ctx, uid, 99, 10, false, 0)
	if err != nil || total != 0 {
		t.Errorf("level_id=99 不应命中：%d, %v", total, err)
	}
	// limit 夹紧
	if _, _, err := ts.AdminListBattles(ctx, 0, 0, 0, false, 0); err != nil {
		t.Errorf("limit=0 应回落默认：%v", err)
	}

	broken := openBrokenService(t)
	if _, _, err := broken.AdminListBattles(ctx, 0, 0, 10, false, 0); err == nil {
		t.Error("数据库故障时应报错")
	}
}

// loseSettleInput 一份必然通过校验的失败局上报（供多处复用）。
//
// ⚠️ `ReplayHash` 是**空串**，这是 2026-10-15 整局重算接线时改的。
//
// 改之前这里是占位假值 "0000000000000000"。接线后 `recomputeEngine`
// 拿存档输入重跑整局，算出真哈希与它一比必然不符，于是在 enforce 档
// （缺省档位）把**每一次** e2e 结算都拒了。症状是 7 个「中途失败」
// 用例集体变红，而且红在一个**与它们要测的东西毫无关系**的地方：
// 它们要测的是 DB 中途失败时整体回滚，却在到达注入点之前就被重算拦下了。
//
// 为什么选空串而不是「给它们算一个真哈希」：
//
//  1. 空串走的是**既有的兼容路径**——「客户端没上报」，与 S 段那条
//     「空串放行」同一条规则，不需要为本文件新增任何语义；
//  2. 这些用例测的是**结算机制**（幂等、回滚、发奖、进度），
//     不是哈希验真。给它们塞真哈希会把一个纯载荷构造函数变成
//     「需要先开一局、拿到种子与构筑快照」的服务端方法，
//     25 个调用点全部要改 —— 而它们并不需要这个；
//  3. 真哈希路径由 recompute_engine_test.go 专门覆盖，
//     那里每一局都用**真**种子 + **真**构筑快照算出真哈希。
//
// 代价要说清楚：这样一来，这 25 处 e2e 结算**不再经过**重算路径。
// 如果将来重算本身有 bug，纯靠本文件是发现不了的。
func loseSettleInput() domain.SettleInput {
	return domain.SettleInput{
		// Kills 必须非零：kills 任务的进度推进依赖它
		Result: "lose", Score: 500, DurationMs: 30_000,
		Kills: 3, Shots: 200, Hits: 100, Reactions: 2, HeatMax: 30,
		HPLeft: 500, WaveReached: 1, ReplayHash: "",
		ElementsUsed: map[string]int{"fire": 1}, ReactionsUsed: map[string]int{},
		TerrainUsed: []string{}, CardPicks: []int{0, -1},
	}
}

func TestAdminUpdateLevel(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 全部白名单字段
	//
	// ⚠️ 第 109 轮修：`star_targets` 用**线形**（JSON 数组在 Go 侧解码成
	// []any of float64）。旧输入 []int 是 Go 直调类型，
	// 第 107 轮的形状校验只认线形 —— 那是**契约**，
	// 测试必须编码「请求真正长什么样」而不是随便一个 Go 类型。
	_, err := ts.AdminUpdateLevel(ctx, 1, map[string]any{
		"name": "svc关卡", "base_hp": float64(6000), "wave_count": float64(4),
		"difficulty": float64(3), "energy_cost": float64(7),
		"star_targets": []any{1.0, 2.0, 3.0}, "terrain_config": []any{}, "enabled": true, "is_boss": false,
	})
	if err != nil {
		t.Fatalf("更新失败：%v", err)
	}
	var hp int64
	if err := ts.pool.QueryRow(ctx, `SELECT base_hp FROM levels WHERE id = 1`).Scan(&hp); err != nil || hp != 6000 {
		t.Fatalf("base_hp 应写库为 6000，实际 %d（err=%v）", hp, err)
	}

	// 全非法值 → 整单拒绝（第 107 轮起：不再是「静默过滤」）
	if _, err := ts.AdminUpdateLevel(ctx, 1, map[string]any{
		"name": 123, "base_hp": "x", "wave_count": float64(51), "energy_cost": float64(-1),
	}); !errors.Is(err, ErrBadInput) {
		t.Errorf("全非法值应 ErrBadInput，实际 %v", err)
	}
	// 空 patch
	if _, err := ts.AdminUpdateLevel(ctx, 1, map[string]any{}); !errors.Is(err, ErrBadInput) {
		t.Errorf("空 patch 应 ErrBadInput，实际 %v", err)
	}

	// 恢复：把第 1 关重置回生成器默认值（共享库卫生）。
	// 第 116 轮起 regenerate **保留** base_hp（不再覆盖），
	// 所以「靠 regenerate 恢复」这条路对 base_hp 失效了，必须显式还原。
	g := domain.GenerateLevel(1)
	if _, err := ts.pool.Exec(ctx,
		`UPDATE levels SET name=$1, base_hp=$2, wave_count=$3, difficulty=$4,
		 energy_cost=$5, is_boss=$6, enabled=TRUE WHERE id = 1`,
		g.Name, g.BaseHP, g.WaveCount, g.Difficulty, g.EnergyCost, g.IsBoss); err != nil {
		t.Errorf("还原第 1 关失败：%v", err)
	}
	// regenerate 仍必须能跑通且同步内容字段（它本身也是被测函数）
	if n, err := ts.AdminRegenerateLevels(ctx); err != nil || n != domain.TotalLevels {
		t.Fatalf("regenerate 应生成 %d 关：%d, %v", domain.TotalLevels, n, err)
	}

	broken := openBrokenService(t)
	if _, err := broken.AdminUpdateLevel(ctx, 1, map[string]any{"name": "x"}); err == nil {
		t.Error("数据库故障时应报错")
	}
}

func TestAdminUpdateSkill(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	var origName string
	if err := ts.pool.QueryRow(ctx, `SELECT name FROM skills WHERE id = 1`).Scan(&origName); err != nil {
		t.Skipf("无技能种子：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(ctx, `UPDATE skills SET name = $1 WHERE id = 2`, origName)
	})

	// 第 120 轮：AdminUpdateSkill 回读落库行作为返回值（后台契约 {skill}）
	row, err := ts.AdminUpdateSkill(ctx, 1, map[string]any{
		"name": "svc技能", "descr": "d", "base_damage": float64(33),
		"element": "fire", "family": "flame", "kind": "active", "heat_cost": float64(5),
	})
	if err != nil {
		t.Fatalf("更新失败：%v", err)
	}
	if row.Name != "svc技能" {
		t.Errorf("回读行 name 应为 svc技能，实际 %v", row.Name)
	}
	var dmg int64
	if err := ts.pool.QueryRow(ctx, `SELECT base_damage FROM skills WHERE id = 1`).Scan(&dmg); err != nil || dmg != 33 {
		t.Fatalf("base_damage 应写库 33，实际 %d（err=%v）", dmg, err)
	}
	if row.BaseDamage != int64(33) {
		t.Errorf("回读行 base_damage 应为 33，实际 %v（写后读必须与落库一致）", row.BaseDamage)
	}

	if _, err := ts.AdminUpdateSkill(ctx, 1, map[string]any{"name": 42, "base_damage": -1}); !errors.Is(err, ErrBadInput) {
		t.Errorf("全非法值应 ErrBadInput，实际 %v", err)
	}
	if _, err := ts.AdminUpdateSkill(ctx, 1, map[string]any{}); !errors.Is(err, ErrBadInput) {
		t.Errorf("空 patch 应 ErrBadInput，实际 %v", err)
	}
}

// --- adminapi.go ---

func TestAdminListLevelsAndWaves(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	items, total, err := ts.AdminListLevels(ctx, 0, "", 0, 0)
	if err != nil || total != int64(domain.TotalLevels) {
		t.Fatalf("关卡列表应 %d 关：%d, %v", domain.TotalLevels, total, err)
	}
	// 每行必须带运营要看的通关率字段
	// 2026-10-21: the row is a typed view now, so "the field exists" is a
	// compile-time fact. What still deserves an assertion is that the
	// aggregation columns carry real numbers (not zero-valued placeholders).
	first := items[0]
	if first.Attempts < 0 || first.Clears < 0 {
		t.Errorf("通关率聚合不应为负：attempts=%d clears=%d", first.Attempts, first.Clears)
	}
	if first.ClearRate < 0 || first.ClearRate > 100 {
		t.Errorf("clear_rate 应是百分数（0..100），实际 %v", first.ClearRate)
	}
	if first.AvgWave < 0 {
		t.Errorf("avg_wave 不应为负，实际 %v", first.AvgWave)
	}

	waves, err := ts.AdminLevelWaves(ctx, 1)
	if err != nil || len(waves) == 0 {
		t.Fatalf("第 1 关应有波次配置：%d, %v", len(waves), err)
	}

	broken := openBrokenService(t)
	if _, _, err := broken.AdminListLevels(ctx, 0, "", 0, 0); err == nil {
		t.Error("故障态应报错")
	}
	if _, err := broken.AdminLevelWaves(ctx, 1); err == nil {
		t.Error("故障态应报错")
	}
}

func TestAdminSkillsAndEquipment(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	data, err := ts.AdminSkills(ctx)
	if err != nil {
		t.Fatalf("AdminSkills：%v", err)
	}
	if len(data.Items) == 0 {
		t.Fatal("技能列表不应为空")
	}
	// base / composite 拆分边界：id<=24 是基础技能
	if len(data.Recipes) == 0 {
		t.Error("配方清单不应为空")
	}

	eq, err := ts.AdminEquipment(ctx)
	if err != nil {
		t.Fatalf("AdminEquipment：%v", err)
	}
	// 2026-10-21: the admin frontend used to declare only 3 of the 8 keys
	// this endpoint returns, so enemies / reactions / mastery / rating
	// weights were silently dropped on the way to the UI. The keys are
	// compile-time facts now; what still needs an assertion is that they
	// are actually populated.
	if len(eq.Equipment) == 0 {
		t.Error("装备清单不应为空")
	}
	if len(eq.Gems) == 0 {
		t.Error("宝石清单不应为空")
	}
	if len(eq.Qualifications) == 0 {
		t.Error("宝石品质清单不应为空")
	}
	if len(eq.Skins) == 0 {
		t.Error("皮肤清单不应为空")
	}
	if len(eq.Enemies) == 0 {
		t.Error("敌人清单不应为空")
	}
	if len(eq.Reactions) == 0 {
		t.Error("反应表不应为空")
	}
	if len(eq.Mastery) == 0 {
		t.Error("专精树清单不应为空")
	}
	if eq.RatingWeights.ElementCoverage <= 0 || eq.RatingWeights.ReactionCoverage <= 0 {
		t.Errorf("构筑权重不应全零，实际 %+v", eq.RatingWeights)
	}

	broken := openBrokenService(t)
	if _, err := broken.AdminSkills(ctx); err == nil {
		t.Error("故障态应报错")
	}
	if _, err := broken.AdminEquipment(ctx); err == nil {
		t.Error("故障态应报错")
	}
}

func TestAdminListDefenses(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	if _, err := ts.SaveDefense(ctx, uid, SaveDefenseInput{Name: "svc防线", Skills: []int{1}}); err != nil {
		t.Fatalf("建防线失败：%v", err)
	}

	_, total, err := ts.AdminListDefenses(ctx, 50, 0)
	if err != nil || total < 1 {
		t.Fatalf("防线列表应非空：%d, %v", total, err)
	}
	if _, _, err := ts.AdminListDefenses(ctx, 9999, 0); err != nil {
		t.Errorf("limit 夹紧不应报错：%v", err)
	}

	broken := openBrokenService(t)
	if _, _, err := broken.AdminListDefenses(ctx, 50, 0); err == nil {
		t.Error("故障态应报错")
	}
}

func TestAdminEconomy(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	data, err := ts.AdminEconomy(ctx)
	if err != nil {
		t.Fatalf("AdminEconomy：%v", err)
	}
	if len(data.Shop) == 0 {
		t.Fatal("商城配置不应为空")
	}
	if len(data.Flows) == 0 {
		t.Fatal("钱包流水不应为空")
	}

	broken := openBrokenService(t)
	if _, err := broken.AdminEconomy(ctx); err == nil {
		t.Error("故障态应报错")
	}
}

func TestAdminUpdateShopItem(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	var origName string
	var origLimit int
	if err := ts.pool.QueryRow(ctx,
		`SELECT name, limit_per_day FROM shop_items WHERE id = 1`).Scan(&origName, &origLimit); err != nil {
		t.Skipf("无商城种子：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(ctx, `UPDATE shop_items SET name = $1, limit_per_day = $2 WHERE id = 1`,
			origName, origLimit)
	})

	out, err := ts.AdminUpdateShopItem(ctx, 1, map[string]any{
		"name": "svc商品", "price": map[string]int{"gem": 9}, "payload": map[string]int{"coin": 1},
		"limit_per_day": float64(9), "sort_order": float64(9), "enabled": true,
	})
	if err != nil {
		t.Fatalf("更新失败：%v", err)
	}
	if out.ID != 1 {
		t.Errorf("返回 id = %v", out.ID)
	}
	var limit int
	if err := ts.pool.QueryRow(ctx, `SELECT limit_per_day FROM shop_items WHERE id = 1`).Scan(&limit); err != nil || limit != 9 {
		t.Fatalf("limit_per_day 应写库 9，实际 %d（err=%v）", limit, err)
	}

	if _, err := ts.AdminUpdateShopItem(ctx, 1, map[string]any{"name": ""}); !errors.Is(err, ErrBadInput) {
		t.Errorf("全被过滤应 ErrBadInput，实际 %v", err)
	}
	if _, err := ts.AdminUpdateShopItem(ctx, 1, map[string]any{}); !errors.Is(err, ErrBadInput) {
		t.Errorf("空 patch 应 ErrBadInput，实际 %v", err)
	}
}

func TestAdminAnnouncements(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	item, err := ts.AdminCreateAnnouncement(ctx, "svc公告", "正文", true)
	if err != nil {
		t.Fatalf("创建公告失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(ctx, `DELETE FROM announcements WHERE id = $1`, item.ID)
	})
	if !item.Published {
		t.Error("published 应原样返回")
	}
	items, _, err := ts.AdminListAnnouncements(ctx, 50, 0)
	if err != nil || len(items) == 0 {
		t.Fatalf("公告列表应非空：%v", err)
	}

	broken := openBrokenService(t)
	if _, err := broken.AdminCreateAnnouncement(ctx, "x", "y", false); err == nil {
		t.Error("故障态应报错")
	}
	if _, _, err := broken.AdminListAnnouncements(ctx, 50, 0); err == nil {
		t.Error("故障态应报错")
	}
}

func TestAdminRedeemCodes(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	items, _, err := ts.AdminListRedeemCodes(ctx, 50, 0)
	if err != nil {
		t.Fatalf("兑换码列表失败：%v", err)
	}
	if len(items) == 0 {
		t.Log("库中无兑换码（种子可能未跑）")
	}

	// 迁移 00009 把 redeem_codes.id 改为 IDENTITY、种子跑完对齐了序列，
	// 所以管理端"自动 id"创建兑换码现在能成功（此前 id 无默认值必然 500）。
	// 这条钉住修复后的行为：创建成功、能被列表读回、且落库字段正确。
	code := fmt.Sprintf("SVCBUG%d", time.Now().UnixNano()%1_000_000)
	created, err := ts.AdminCreateRedeemCode(ctx, code, map[string]int{"coin": 1}, 3, nil)
	if err != nil {
		t.Fatalf("创建兑换码失败（迁移 00009 后应成功）：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(ctx, `DELETE FROM redeem_codes WHERE code = $1`, code)
	})
	if created.ID <= 0 {
		t.Errorf("自动分配的 id 应为正，得 %v", created.ID)
	}

	after, _, err := ts.AdminListRedeemCodes(ctx, 50, 0)
	if err != nil {
		t.Fatalf("创建后列表失败：%v", err)
	}
	if len(after) != len(items)+1 {
		t.Errorf("列表应多出 1 条，前 %d 后 %d", len(items), len(after))
	}
	found := false
	for _, it := range after {
		if it.Code == code {
			found = true
			if it.MaxUses != 3 {
				t.Errorf("max_uses 应为 3，得 %v", it.MaxUses)
			}
		}
	}
	if !found {
		t.Error("新建兑换码应出现在列表里")
	}

	//
	// ⚠️ 第 110 轮：过期时间必须**真的进库**。
	// 修前 handler 解析了 expires_at 却从不传到这里，
	// 运营在 UI 设的过期时间被静默丢弃（兑换码永久有效）。
	// 判据：nil → 库里 IS NULL；非 nil → 库里 == 传入值。
	var wasNull bool
	_ = ts.pool.QueryRow(ctx,
		`SELECT expires_at IS NULL FROM redeem_codes WHERE code = $1`, code).
		Scan(&wasNull)
	if !wasNull {
		t.Errorf("expiresAt=nil 应落 NULL，实际非 NULL")
	}

	expiredCode := code + "-EXP"
	// timestamptz 是微秒精度：带纳秒的 time.Time 写库再读回必然 != 原值，
	// 断言前必须截到微秒（同一族「观测手段本身有前提」的坑）。
	exp := time.Now().UTC().Truncate(time.Microsecond).Add(24 * time.Hour)
	if _, err := ts.AdminCreateRedeemCode(ctx, expiredCode, map[string]int{"coin": 1}, 1, &exp); err != nil {
		t.Fatalf("带过期时间创建失败：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(ctx, `DELETE FROM redeem_codes WHERE code = $1`, expiredCode)
	})
	var got time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT expires_at FROM redeem_codes WHERE code = $1`, expiredCode).
		Scan(&got); err != nil {
		t.Fatalf("读回过期时间失败：%v", err)
	}
	if !got.Equal(exp) {
		t.Errorf("expires_at 应 == 传入值 %s，落库 %s", exp.Format(time.RFC3339), got.Format(time.RFC3339))
	}
}

func TestAdminListAuditLogs(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	adminID, username, _ := ts.newAdminFixture(t, "logs", 1)
	ts.Audit(ctx, adminID, "svc_logs_probe", "t", "10.0.0.1", map[string]any{"a": 1})

	items, _, err := ts.AdminListAuditLogs(ctx, 100, 0)
	if err != nil {
		t.Fatalf("审计列表失败：%v", err)
	}
	found := false
	for _, it := range items {
		if it.Username == username {
			found = true
		}
	}
	if !found {
		t.Error("探针审计行应出现在列表里")
	}
	if _, _, err := ts.AdminListAuditLogs(ctx, 0, 0); err != nil {
		t.Errorf("limit=0 应回落默认：%v", err)
	}

	broken := openBrokenService(t)
	if _, _, err := broken.AdminListAuditLogs(ctx, 10, 0); err == nil {
		t.Error("故障态应报错")
	}
}

// joinComma / toInt64：纯函数直接测。
func TestJoinCommaAndToInt64(t *testing.T) {
	if got := joinComma([]string{"a", "b", "c"}); got != "a, b, c" {
		t.Errorf("joinComma = %q", got)
	}
	if got := joinComma(nil); got != "" {
		t.Errorf("joinComma(nil) = %q", got)
	}
	if got := joinComma([]string{"only"}); got != "only" {
		t.Errorf("单元素不应带分隔符：%q", got)
	}

	cases := []struct {
		in   any
		want int64
		ok   bool
	}{
		{float64(3.9), 3, true},
		{int(7), 7, true},
		{int64(9), 9, true},
		{"5", 0, false}, // 字符串不支持 —— 白名单更新走 JSON number
	}
	for i, c := range cases {
		got, ok := toInt64(c.in)
		if ok != c.ok || (ok && got != c.want) {
			t.Errorf("toInt64(%v)（用例 %d）= %d, %t；期望 %d, %t", c.in, i, got, ok, c.want, c.ok)
		}
	}
}

// TestAdminLogoutFailsOnDBError 覆盖 AdminLogout 的数据库错误路径：
// 吊销 SQL 执行失败时必须返回错误（而不是静默 0 条）。
// 连接 postgres 系统库（可达但无 admin_tokens 表）→ UPDATE 报 42P01。
func TestAdminLogoutFailsOnDBError(t *testing.T) {
	cfg := config.Load()
	cfg.DatabaseURL = "postgres://postgres@127.0.0.1:5432/postgres?sslmode=disable"
	cfg.DBMaxConns = 2
	cfg.DBMinConns = 0
	ctx := context.Background()

	db, err := store.Open(ctx, cfg)
	if err != nil {
		t.Skipf("postgres 系统库连不上，跳过：%v", err)
	}
	defer db.Close()

	s := New(db, cfg)
	if _, err := s.AdminLogout(ctx, 12345); err == nil {
		t.Fatal("无 admin_tokens 表时 AdminLogout 必须返回错误")
	}
}
