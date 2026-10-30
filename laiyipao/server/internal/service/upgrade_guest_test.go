package service

// 游客账号转正（设置用户名 + 密码）的服务层测试。
//
// 关键行为锚点：
//   - 游客号升级后 `is_guest=false`、`username`/`password_hash` 落库，
//     且 **guest_token 保持不变**（老客户端冷启动仍能按旧凭证找回同一账号）；
//   - 升级后可以用新用户名密码登录回**同一个号**；
//   - 用户名已被占用 → ErrBadInput（先查后写，不暴露唯一约束错误）；
//   - 已经是账号登录的行走这里没有意义 → ErrBadInput；
//   - 封禁 / 不存在 / 库故障都有明确分支。

import (
	"context"
	"errors"
	"fmt"
	"testing"
	"time"
)

// newGuestUser 直接插入一个纯游客号（password_hash 为 NULL），返回 id 与 guest_token。
func newGuestUser(t *testing.T, ts *testService, tag string) (int64, string) {
	t.Helper()
	ctx := context.Background()
	guestToken := fmt.Sprintf("guest_upgrade_%s_%d", tag, time.Now().UnixNano())
	var id int64
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, nickname, is_guest, last_login_at)
		 VALUES ($1, $2, TRUE, now())
		 RETURNING id`, guestToken, "游客"+tag).Scan(&id); err != nil {
		t.Fatalf("创建游客号失败：%v", err)
	}
	return id, guestToken
}

func TestUpgradeGuestAccountSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, guestToken := newGuestUser(t, ts, "ok")
	username := fmt.Sprintf("upgraded_%d", time.Now().UnixNano())
	const password = "new-password-12345"

	if err := ts.UpgradeGuestAccount(ctx, id, username, password); err != nil {
		t.Fatalf("游客转正失败：%v", err)
	}

	// 库中状态：is_guest=false、username/password_hash 已写、guest_token 不变
	var (
		gotGuest      bool
		gotUsername   string
		gotHash       *string
		gotGuestToken string
	)
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT is_guest, username, password_hash, guest_token FROM users WHERE id = $1`, id).
		Scan(&gotGuest, &gotUsername, &gotHash, &gotGuestToken); err != nil {
		t.Fatalf("查询升级结果失败：%v", err)
	}
	if gotGuest {
		t.Error("升级后 is_guest 应为 false")
	}
	if gotUsername != username {
		t.Errorf("用户名 = %q，期望 %q", gotUsername, username)
	}
	if gotHash == nil || *gotHash == "" {
		t.Error("升级后 password_hash 不应为空")
	}
	if gotGuestToken != guestToken {
		t.Errorf("guest_token 不应被轮换：%q → %q", guestToken, gotGuestToken)
	}

	// 升级后可以用新用户名密码登录回同一个号
	tp, err := ts.Login(ctx, username, password)
	if err != nil {
		t.Fatalf("升级后新密码登录失败：%v", err)
	}
	if tp.User.ID != id {
		t.Errorf("登录身份应为 %d，实际 %d", id, tp.User.ID)
	}
	if tp.User.IsGuest {
		t.Error("登录返回的身份不应再是游客")
	}
}

func TestUpgradeGuestAccountUsernameTaken(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 先引导一个共享账号占住用户名
	takenUsername := fmt.Sprintf("taken_%d", time.Now().UnixNano())
	if _, err := ts.EnsureBootstrapPlayer(ctx, takenUsername, "test-password-12345"); err != nil {
		t.Fatalf("引导账号失败：%v", err)
	}

	id, _ := newGuestUser(t, ts, "taken")
	err := ts.UpgradeGuestAccount(ctx, id, takenUsername, "new-password-12345")
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("用户名被占用应返回 ErrBadInput，实际 %v", err)
	}
	if err == nil || err.Error() != "输入非法: 用户名已被使用" {
		t.Errorf("错误信息应指向用户名占用，实际 %v", err)
	}
}

func TestUpgradeGuestAccountAlreadyAccount(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	username := fmt.Sprintf("already_%d", time.Now().UnixNano())
	if _, err := ts.EnsureBootstrapPlayer(ctx, username, "test-password-12345"); err != nil {
		t.Fatalf("引导账号失败：%v", err)
	}
	var id int64
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT id FROM users WHERE username = $1`, username).Scan(&id); err != nil {
		t.Fatalf("查询账号失败：%v", err)
	}

	err := ts.UpgradeGuestAccount(ctx, id, "another-name", "new-password-12345")
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("已是账号的行走这里应返回 ErrBadInput，实际 %v", err)
	}
}

func TestUpgradeGuestAccountValidation(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "validation")
	longName := ""
	for i := 0; i < 33; i++ {
		longName += "名"
	}

	cases := []struct {
		name               string
		username, password string
	}{
		{"空用户名", "", "new-password-12345"},
		{"空密码", "valid_name", ""},
		{"密码太短", "valid_name", "short"},
		{"用户名含空格", "bad name", "new-password-12345"},
		{"用户名超长", longName, "new-password-12345"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			err := ts.UpgradeGuestAccount(ctx, id, tc.username, tc.password)
			if !errors.Is(err, ErrBadInput) {
				t.Errorf("应返回 ErrBadInput，实际 %v", err)
			}
		})
	}
}

func TestUpgradeGuestAccountBanned(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "banned")
	if _, err := ts.db.Pool.Exec(ctx,
		`UPDATE users SET status = 2 WHERE id = $1`, id); err != nil {
		t.Fatalf("封禁账号失败：%v", err)
	}

	err := ts.UpgradeGuestAccount(ctx, id, "valid_name", "new-password-12345")
	if !errors.Is(err, ErrForbidden) {
		t.Fatalf("封禁游客转正应返回 ErrForbidden，实际 %v", err)
	}
}

func TestUpgradeGuestAccountUnknownUser(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	err := ts.UpgradeGuestAccount(ctx, 999_999_999, "valid_name", "new-password-12345")
	if !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的用户应返回 ErrNotFound，实际 %v", err)
	}
}

func TestUpgradeGuestAccountBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	err := ts.UpgradeGuestAccount(ctx, 1, "valid_name", "new-password-12345")
	if err == nil {
		t.Fatal("连接池已关闭时转正必须报错，不能静默成功")
	}
}
