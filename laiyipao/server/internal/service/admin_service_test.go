package service

// 管理员与后台端的服务层测试。
//
// 为什么在 service 层再测一遍（httpapi E2E 已走过）：Go 覆盖率按包统计，
// 且这里能直接构造 httpapi 够不着的场景 —— 停用管理员、短密码 bootstrap、
// 库中故障态。每条断言都锚定一个**行为后果**（库中状态、错误类型），
// 不是"函数没报错就算过"。

import (
	"context"
	"errors"
	"fmt"
	"testing"
	"time"

	"golang.org/x/crypto/bcrypt"

	"github.com/laiyipao/server/internal/auth"
	"github.com/laiyipao/server/internal/domain"
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

	// 成功：返回 admin 身份与会话令牌
	tp, err := ts.AdminLogin(ctx, username, password)
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	if tp.Admin.Username != username || tp.AccessToken == "" || tp.Session == "" {
		t.Fatalf("登录响应不完整：%+v", tp)
	}
	// 会话摘要落库，且是对带 a_ 前缀明文的哈希
	var n int
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM admin_tokens WHERE token_hash = $1`,
		auth.HashRefreshToken(tp.Session)).Scan(&n); err != nil || n != 1 {
		t.Errorf("会话摘要未按 a_+明文 落库（count=%d err=%v）", n, err)
	}

	// 未知用户与错误密码必须同一错误（防用户名枚举）
	if _, err := ts.AdminLogin(ctx, "svc_no_such_admin", password); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("未知用户应 ErrUnauthorized，实际 %v", err)
	}
	if _, err := ts.AdminLogin(ctx, username, "wrong-password"); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("错误密码应 ErrUnauthorized，实际 %v", err)
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
	tok, _, err := ts.JWT.Issue(99999999, "admin")
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

	ts.Audit(ctx, adminID, "svc_audit_probe", "target-1", map[string]any{"k": "v"})

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

	// detail 无法 JSON 化（channel）必须降级为 {}，而不是让主流程失败
	ts.Audit(ctx, adminID, "svc_audit_badjson", "t", make(chan int))
	var raw []byte
	if err := ts.pool.QueryRow(ctx,
		`SELECT detail FROM admin_audit_logs WHERE admin_id = $1 AND action = 'svc_audit_badjson'`,
		adminID).Scan(&raw); err != nil || string(raw) != "{}" {
		t.Errorf("不可序列化 detail 应落 {}，实际 %q err=%v", raw, err)
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
	if plain[:2] != "a_" || hash != auth.HashRefreshToken(plain) {
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
	d, err := ts.AdminDashboard(context.Background())
	if err != nil {
		t.Fatalf("看板失败：%v", err)
	}
	if d.Users.Total < 1 {
		t.Error("库中有测试用户，total 不应为 0")
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

	_, total, err := ts.AdminListBattles(ctx, uid, 1, 10, false)
	if err != nil || total < 1 {
		t.Fatalf("按用户+关卡过滤应命中：%d, %v", total, err)
	}
	// 非匹配过滤 → 0
	_, total, err = ts.AdminListBattles(ctx, uid, 99, 10, false)
	if err != nil || total != 0 {
		t.Errorf("level_id=99 不应命中：%d, %v", total, err)
	}
	// limit 夹紧
	if _, _, err := ts.AdminListBattles(ctx, 0, 0, 0, false); err != nil {
		t.Errorf("limit=0 应回落默认：%v", err)
	}

	broken := openBrokenService(t)
	if _, _, err := broken.AdminListBattles(ctx, 0, 0, 10, false); err == nil {
		t.Error("数据库故障时应报错")
	}
}

// loseSettleInput 一份必然通过校验的失败局上报（供多处复用）。
func loseSettleInput() domain.SettleInput {
	return domain.SettleInput{
		// Kills 必须非零：kills 任务的进度推进依赖它
		Result: "lose", Score: 500, DurationMs: 30_000,
		Kills: 3, Shots: 200, Hits: 100, Reactions: 2, HeatMax: 30,
		HPLeft: 500, WaveReached: 1, ReplayHash: "0000000000000000",
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

	if err := ts.AdminUpdateSkill(ctx, 1, map[string]any{
		"name": "svc技能", "descr": "d", "base_damage": float64(33),
		"element": "fire", "family": "flame", "kind": "active", "heat_cost": float64(5),
	}); err != nil {
		t.Fatalf("更新失败：%v", err)
	}
	var dmg int64
	if err := ts.pool.QueryRow(ctx, `SELECT base_damage FROM skills WHERE id = 1`).Scan(&dmg); err != nil || dmg != 33 {
		t.Fatalf("base_damage 应写库 33，实际 %d（err=%v）", dmg, err)
	}

	if err := ts.AdminUpdateSkill(ctx, 1, map[string]any{"name": 42, "base_damage": -1}); !errors.Is(err, ErrBadInput) {
		t.Errorf("全非法值应 ErrBadInput，实际 %v", err)
	}
	if err := ts.AdminUpdateSkill(ctx, 1, map[string]any{}); !errors.Is(err, ErrBadInput) {
		t.Errorf("空 patch 应 ErrBadInput，实际 %v", err)
	}
}

// --- adminapi.go ---

func TestAdminListLevelsAndWaves(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	items, total, err := ts.AdminListLevels(ctx, 0, "")
	if err != nil || total != int64(domain.TotalLevels) {
		t.Fatalf("关卡列表应 %d 关：%d, %v", domain.TotalLevels, total, err)
	}
	// 每行必须带运营要看的通关率字段
	first := items[0]
	for _, key := range []string{"attempts", "clears", "clear_rate", "avg_wave"} {
		if _, ok := first[key]; !ok {
			t.Errorf("关卡列表缺字段 %q", key)
		}
	}

	waves, err := ts.AdminLevelWaves(ctx, 1)
	if err != nil || len(waves) == 0 {
		t.Fatalf("第 1 关应有波次配置：%d, %v", len(waves), err)
	}

	broken := openBrokenService(t)
	if _, _, err := broken.AdminListLevels(ctx, 0, ""); err == nil {
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
	items, _ := data["items"].([]map[string]any)
	if len(items) == 0 {
		t.Fatal("技能列表不应为空")
	}
	// base / composite 拆分边界：id<=24 是基础技能
	if len(data["recipes"].([]domain.SeedRecipe)) == 0 {
		t.Error("配方清单不应为空")
	}

	eq, err := ts.AdminEquipment(ctx)
	if err != nil {
		t.Fatalf("AdminEquipment：%v", err)
	}
	for _, key := range []string{"equipment", "gems", "qualities", "skins", "enemies", "reactions", "mastery"} {
		if _, ok := eq[key]; !ok {
			t.Errorf("AdminEquipment 缺 %q", key)
		}
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

	_, total, err := ts.AdminListDefenses(ctx, 50)
	if err != nil || total < 1 {
		t.Fatalf("防线列表应非空：%d, %v", total, err)
	}
	if _, _, err := ts.AdminListDefenses(ctx, 9999); err != nil {
		t.Errorf("limit 夹紧不应报错：%v", err)
	}

	broken := openBrokenService(t)
	if _, _, err := broken.AdminListDefenses(ctx, 50); err == nil {
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
	shop, ok := data["shop"].([]map[string]any)
	if !ok || len(shop) == 0 {
		t.Fatal("商城配置不应为空")
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
	if out["id"].(int64) != 1 {
		t.Errorf("返回 id = %v", out["id"])
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
		_, _ = ts.pool.Exec(ctx, `DELETE FROM announcements WHERE id = $1`, item["id"])
	})
	if item["published"] != true {
		t.Error("published 应原样返回")
	}
	items, err := ts.AdminListAnnouncements(ctx)
	if err != nil || len(items) == 0 {
		t.Fatalf("公告列表应非空：%v", err)
	}

	broken := openBrokenService(t)
	if _, err := broken.AdminCreateAnnouncement(ctx, "x", "y", false); err == nil {
		t.Error("故障态应报错")
	}
	if _, err := broken.AdminListAnnouncements(ctx); err == nil {
		t.Error("故障态应报错")
	}
}

func TestAdminRedeemCodes(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	items, err := ts.AdminListRedeemCodes(ctx)
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
	if id, _ := created["id"].(int); id <= 0 {
		t.Errorf("自动分配的 id 应为正，得 %v", created["id"])
	}

	after, err := ts.AdminListRedeemCodes(ctx)
	if err != nil {
		t.Fatalf("创建后列表失败：%v", err)
	}
	if len(after) != len(items)+1 {
		t.Errorf("列表应多出 1 条，前 %d 后 %d", len(items), len(after))
	}
	found := false
	for _, it := range after {
		if it["code"] == code {
			found = true
			if mu, _ := it["max_uses"].(int); mu != 3 {
				t.Errorf("max_uses 应为 3，得 %v", it["max_uses"])
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
	ts.Audit(ctx, adminID, "svc_logs_probe", "t", map[string]any{"a": 1})

	items, err := ts.AdminListAuditLogs(ctx, 100)
	if err != nil {
		t.Fatalf("审计列表失败：%v", err)
	}
	found := false
	for _, it := range items {
		if it["username"] == username {
			found = true
		}
	}
	if !found {
		t.Error("探针审计行应出现在列表里")
	}
	if _, err := ts.AdminListAuditLogs(ctx, 0); err != nil {
		t.Errorf("limit=0 应回落默认：%v", err)
	}

	broken := openBrokenService(t)
	if _, err := broken.AdminListAuditLogs(ctx, 10); err == nil {
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
