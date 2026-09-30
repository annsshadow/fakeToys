package service

// 账号域（service.go）的服务层测试：登录链路、令牌刷新、钱包。
//
// 关键行为锚点：
//   - 封禁必须同时挡住「再登录」与「旧 access token」两条路；
//   - refresh token 一次性：用过即吊销；
//   - grantWallet 扣减必须带余额条件，绝不允许负余额。
//
// initNewUser 的逐步错误分支用 scratch 库 + 表改名覆盖：
// closed pool 只能覆盖第一步（钱包 INSERT），后面的分支需要
// "前一步成功、当前表坏"的组合。

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/jackc/pgx/v5"
)

func TestGuestLoginLifecycle(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	guest := fmt.Sprintf("svc_guest_%d", time.Now().UnixNano())

	// 首登：建号 + 初始数据
	tp, err := ts.GuestLogin(ctx, guest, "svc玩家")
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	if tp.User.ID == 0 || tp.AccessToken == "" || tp.RefreshToken == "" {
		t.Fatalf("令牌对不完整：%+v", tp)
	}
	// 初始钱包必须就位（新号白屏比报错更糟）
	w := ts.wallet(t, ctx, tp.User.ID)
	if w.Coin != 2000 || w.Gem != 50 || w.Energy != 30 {
		t.Errorf("新号初始钱包 = %+v", w)
	}
	// 默认出战槽位：新号必须有技能可用，否则进游戏是死局
	slots, err := ts.Loadout(ctx, tp.User.ID)
	if err != nil || len(slots) == 0 || slots[0] == 0 {
		t.Errorf("新号默认出战槽位异常：%v, %v", slots, err)
	}

	// 二次登录：恢复同一账号，不重置进度
	tp2, err := ts.GuestLogin(ctx, guest, "svc玩家")
	if err != nil {
		t.Fatalf("二次登录失败：%v", err)
	}
	if tp2.User.ID != tp.User.ID {
		t.Errorf("同凭证登录应恢复同一账号：%d vs %d", tp2.User.ID, tp.User.ID)
	}

	// 空昵称自动生成
	tp3, err := ts.GuestLogin(ctx, "", "")
	if err != nil {
		t.Fatalf("空凭证登录失败：%v", err)
	}
	if tp3.User.Nickname == "" {
		t.Error("空昵称应自动生成")
	}
}

func TestGuestLoginBanned(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	guest := fmt.Sprintf("svc_ban_%d", time.Now().UnixNano())
	tp, err := ts.GuestLogin(ctx, guest, "b")
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	ts.exec(t, `UPDATE users SET status = 2 WHERE id = $1`, tp.User.ID)

	// 封禁后同凭证再登录 → ErrForbidden（签发口检查）
	if _, err := ts.GuestLogin(ctx, guest, "b"); !errors.Is(err, ErrForbidden) {
		t.Errorf("封禁后登录应 ErrForbidden，实际 %v", err)
	}
	// 旧 access token → ErrForbidden（每请求检查）
	if _, err := ts.ResolveUser(ctx, tp.AccessToken); !errors.Is(err, ErrForbidden) {
		t.Errorf("封禁后旧 token 应 ErrForbidden，实际 %v", err)
	}
}

func TestResolveUserTokenKinds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)
	tp, err := ts.GuestLogin(ctx, "", "rk")
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	_ = uid

	got, err := ts.ResolveUser(ctx, tp.AccessToken)
	if err != nil || got == 0 {
		t.Fatalf("有效 token 应通过：%d, %v", got, err)
	}

	// 管理员令牌打玩家端 → kind 校验拒绝
	_, adminUser, adminPass := ts.newAdminFixture(t, "rk", 1)
	adminTP, err := ts.AdminLogin(ctx, adminUser, adminPass)
	if err != nil {
		t.Fatalf("管理员登录失败：%v", err)
	}
	if _, err := ts.ResolveUser(ctx, adminTP.AccessToken); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("管理员令牌应被玩家端拒绝，实际 %v", err)
	}

	// 垃圾令牌
	if _, err := ts.ResolveUser(ctx, "junk"); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("垃圾令牌应 ErrUnauthorized，实际 %v", err)
	}

	// 有效签名但不存在的用户（id 99999999）
	tok, _, _ := ts.JWT.Issue(99999999, "user")
	if _, err := ts.ResolveUser(ctx, tok); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("不存在用户应 ErrUnauthorized，实际 %v", err)
	}
}

func TestRefreshTokenRotation(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	tp, err := ts.GuestLogin(ctx, "", "rt")
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}

	if _, err := ts.Refresh(ctx, ""); !errors.Is(err, ErrBadInput) {
		t.Errorf("空 refresh_token 应 ErrBadInput，实际 %v", err)
	}
	if _, err := ts.Refresh(ctx, "bogus"); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("无效 refresh_token 应 ErrUnauthorized，实际 %v", err)
	}

	// 正常刷新：下发新对
	tp2, err := ts.Refresh(ctx, tp.RefreshToken)
	if err != nil {
		t.Fatalf("刷新失败：%v", err)
	}
	if tp2.RefreshToken == tp.RefreshToken {
		t.Error("刷新必须轮换 refresh_token")
	}
	// 旧的已吊销：再用 → ErrUnauthorized
	if _, err := ts.Refresh(ctx, tp.RefreshToken); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("旧 refresh_token 应已吊销，实际 %v", err)
	}
	// 新的可用
	if _, err := ts.Refresh(ctx, tp2.RefreshToken); err != nil {
		t.Errorf("新 refresh_token 应可用：%v", err)
	}
}

// TestRefreshBannedUser 封禁玩家的 refresh 链路必须断 ——
// 否则封禁后靠长期令牌续命，封禁形同虚设（issueTokens 状态检查的意义）。
func TestRefreshBannedUser(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	tp, err := ts.GuestLogin(ctx, "", "rtb")
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	ts.exec(t, `UPDATE users SET status = 2 WHERE id = $1`, tp.User.ID)
	if _, err := ts.Refresh(ctx, tp.RefreshToken); !errors.Is(err, ErrForbidden) {
		t.Errorf("封禁用户刷新应 ErrForbidden，实际 %v", err)
	}
}

func TestWechatLoginService(t *testing.T) {
	ts := openTestService(t)

	// 未启用 → fail loud
	if _, err := ts.WechatLogin(context.Background(), "code"); !errors.Is(err, ErrForbidden) {
		t.Fatalf("未启用微信登录应 ErrForbidden，实际 %v", err)
	}

	var wxResp string
	wx := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		_, _ = w.Write([]byte(wxResp))
	}))
	defer wx.Close()
	ts.Cfg.WechatAppID = "wx"
	ts.Cfg.WechatAppSecret = "sec"
	ts.Cfg.WechatEndpoint = wx.URL
	ctx := context.Background()

	if _, err := ts.WechatLogin(ctx, "  "); !errors.Is(err, ErrBadInput) {
		t.Errorf("空 code 应 ErrBadInput，实际 %v", err)
	}

	// 成功路径
	wxResp = `{"openid":"oSVC"}`
	tp, err := ts.WechatLogin(ctx, "good")
	if err != nil {
		t.Fatalf("微信登录失败：%v", err)
	}
	if tp.User.IsGuest {
		t.Error("微信账号不是游客")
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(ctx, `DELETE FROM users WHERE guest_token = $1`, "wx_oSVC")
	})

	// errcode / 缺 openid / 坏 JSON
	for _, body := range []string{
		`{"errcode":40013,"errmsg":"bad"}`,
		`{"session_key":"k"}`,
		`<html/>`,
	} {
		wxResp = body
		if _, err := ts.WechatLogin(ctx, "x"); err == nil {
			t.Errorf("响应 %q 应报错", body)
		}
	}

	// 微信不可达
	ts.Cfg.WechatEndpoint = "http://127.0.0.1:1/wx"
	if _, err := ts.WechatLogin(ctx, "dead"); err == nil {
		t.Error("微信不可达应报错")
	}
}

// TestInitNewUserErrorBranches 覆盖 initNewUser 的六个顺序错误分支。
//
// 手法：scratch 库上逐个改名目标表 → initNewUser 必然在**那一步**失败 →
// 错误信息里带出对应的前缀（init wallet / init progress / ...）。
// 每轮用一个全新的 users 行：initNewUser 的每条 INSERT 都带指向 users 的
// 外键，用户不存在会先在第一步就以 FK 错误失败，覆盖不到后面的分支。
func TestInitNewUserErrorBranches(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	steps := []struct{ table, wantErr string }{
		{"user_wallets", "init wallet"},
		{"user_progress", "init progress"},
		{"offline_rewards", "init offline"},
		{"user_equipment", "init equipment"},
		{"user_skills", "init skills"},
		{"user_skill_slots", "init skill slots"},
	}
	for i, s := range steps {
		var uid int64
		if err := ts.pool.QueryRow(ctx,
			`INSERT INTO users (guest_token, nickname, is_guest) VALUES ($1,'init',TRUE) RETURNING id`,
			fmt.Sprintf("svc_init_%d_%d", i, time.Now().UnixNano())).Scan(&uid); err != nil {
			t.Fatalf("造用户失败：%v", err)
		}
		ts.renameTable(t, s.table, s.table+"_bak")
		err := ts.initNewUser(ctx, uid)
		// 恢复表名，保证下一轮的前置步骤成功
		ts.renameTable(t, s.table+"_bak", s.table)
		mustErr(t, err, s.wantErr)
	}

	// 恢复后对普通用户一切正常
	tp, err := ts.GuestLogin(ctx, "init-after", "x")
	if err != nil {
		t.Errorf("恢复后登录应成功：%v", err)
	}
	_ = tp
}

// TestIssueTokensStoreError 覆盖 issueTokens 落库 refresh token 失败的分支。
// 该分支在「loadUser 成功之后」—— 只能靠 scratch 库把 refresh_tokens 拿走。
func TestIssueTokensStoreError(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	tp, err := ts.GuestLogin(ctx, "tok-user", "x")
	if err != nil {
		t.Fatalf("建号失败：%v", err)
	}
	ts.renameTable(t, "refresh_tokens", "refresh_tokens_bak")
	// 已存在用户再次登录：upsert/初始化/读用户全部成功，
	// 卡在 issueTokens 的 INSERT refresh_tokens
	_, err = ts.GuestLogin(ctx, "tok-user", "x")
	ts.renameTable(t, "refresh_tokens_bak", "refresh_tokens")
	mustErr(t, err, "store refresh token")
	_ = tp
}

func TestGrantWallet(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 发放
	err := ts.DB.Tx(ctx, func(tx pgx.Tx) error {
		return ts.grantWallet(ctx, tx, uid, map[string]int64{"coin": 100, "gem": 0}, "test", 0)
	})
	if err != nil {
		t.Fatalf("发放失败：%v", err)
	}
	if got := ts.wallet(t, ctx, uid).Coin; got < 100 {
		t.Errorf("coin 未到账：%d", got)
	}

	// delta=0 的币种必须跳过（不发流水）
	var flows int
	_ = ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM wallet_flows WHERE user_id = $1 AND currency = 'gem'`, uid).Scan(&flows)
	if flows != 0 {
		t.Errorf("delta=0 不应写流水，实际 %d 条", flows)
	}

	// 未知货币
	err = ts.DB.Tx(ctx, func(tx pgx.Tx) error {
		return ts.grantWallet(ctx, tx, uid, map[string]int64{"diamonds": 1}, "test", 0)
	})
	if !errors.Is(err, ErrBadInput) {
		t.Errorf("未知货币应 ErrBadInput，实际 %v", err)
	}

	// 余额不足：扣减必须带 WHERE 余额条件，返回业务错误而非数据库错误
	err = ts.DB.Tx(ctx, func(tx pgx.Tx) error {
		return ts.grantWallet(ctx, tx, uid, map[string]int64{"gem": -999999}, "test", 0)
	})
	if !errors.Is(err, ErrBadInput) {
		t.Errorf("余额不足应 ErrBadInput，实际 %v", err)
	}
	if w := ts.wallet(t, ctx, uid); w.Gem < 0 {
		t.Errorf("绝不允许负余额：%d", w.Gem)
	}

	// 故障态：grantWallet 的通用查询错误
	broken := openBrokenService(t)
	err = broken.DB.Tx(ctx, func(tx pgx.Tx) error {
		return broken.grantWallet(ctx, tx, uid, map[string]int64{"coin": 1}, "test", 0)
	})
	if err == nil {
		t.Error("故障态应报错")
	}
}

// TestGrantWalletFlowInsertError 覆盖「扣款成功、写流水失败」分支。
// 流水是审计与对账的依据，漏写等于资金变动不可追溯。
func TestGrantWalletFlowInsertError(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	tp, err := ts.GuestLogin(ctx, "flow-user", "x")
	if err != nil {
		t.Fatalf("建号失败：%v", err)
	}

	ts.renameTable(t, "wallet_flows", "wallet_flows_bak")
	err = ts.DB.Tx(ctx, func(tx pgx.Tx) error {
		return ts.grantWallet(ctx, tx, tp.User.ID, map[string]int64{"coin": 5}, "test", 0)
	})
	ts.renameTable(t, "wallet_flows_bak", "wallet_flows")
	mustErr(t, err, "insert flow")
}

func TestLoadWalletRegenAndErrors(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 体力恢复：把 updated_at 拨回 1 小时前 → 应恢复 10 点（6 分钟 1 点），封顶 120
	ts.exec(t, `UPDATE user_wallets SET energy = 50, energy_updated_at = now() - interval '1 hour' WHERE user_id = $1`, uid)
	w, err := ts.LoadWallet(ctx, uid)
	if err != nil {
		t.Fatalf("LoadWallet 失败：%v", err)
	}
	if w.Energy != 60 {
		t.Errorf("1 小时应恢复 10 点体力（50→60），实际 %d", w.Energy)
	}

	// 顶格：不超上限
	ts.exec(t, `UPDATE user_wallets SET energy = 118, energy_updated_at = now() - interval '1 hour' WHERE user_id = $1`, uid)
	w, _ = ts.LoadWallet(ctx, uid)
	if w.Energy != 120 {
		t.Errorf("体力应封顶 120，实际 %d", w.Energy)
	}

	// 不存在用户
	if _, err := ts.LoadWallet(ctx, 99999999); !errors.Is(err, ErrNotFound) {
		t.Errorf("无钱包应 ErrNotFound，实际 %v", err)
	}

	// 故障态
	broken := openBrokenService(t)
	if _, err := broken.LoadWallet(ctx, uid); err == nil {
		t.Error("故障态应报错")
	}
}

func TestLoadUserAndMarshalJSON(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	u, err := ts.loadUser(ctx, uid)
	if err != nil || u.ID != uid {
		t.Fatalf("loadUser = %+v, %v", u, err)
	}
	if _, err := ts.loadUser(ctx, 99999999); !errors.Is(err, ErrNotFound) {
		t.Errorf("不存在用户应 ErrNotFound，实际 %v", err)
	}

	broken := openBrokenService(t)
	if _, err := broken.loadUser(ctx, uid); err == nil {
		t.Error("故障态应报错")
	}

	// marshalJSON：合法值与不可序列化值各一条
	if b, err := marshalJSON(map[string]int{"a": 1}); err != nil || len(b) == 0 {
		t.Errorf("marshalJSON 合法路径异常：%v", err)
	}
	if _, err := marshalJSON(make(chan int)); err == nil {
		t.Error("channel 应不可序列化")
	}
}
