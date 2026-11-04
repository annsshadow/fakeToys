// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 玩家修改密码的服务层测试。
//
// 关键行为锚点：
//   - 原密码正确 → 更新 password_hash，且**同一事务**内吊销全部 refresh token；
//   - 原密码错误 / 游客号 / 新密码不合规 → ErrBadInput，不泄漏「有没有密码」之外的细节；
//   - 封禁账号不允许改密（ErrForbidden）；
//   - 不存在的用户 → ErrNotFound。

import (
	"context"
	"errors"
	"fmt"
	"testing"
	"time"
)

// newPasswordAccount 引导创建一个带密码的账号，返回 id 与密码。
func newPasswordAccount(t *testing.T, ts *testService, tag string) (int64, string) {
	t.Helper()
	ctx := context.Background()
	username := fmt.Sprintf("chpwd_%s_%d", tag, time.Now().UnixNano())
	const password = "test-password-12345"
	created, err := ts.EnsureBootstrapPlayer(ctx, username, password)
	if err != nil {
		t.Fatalf("引导玩家账号失败：%v", err)
	}
	if !created {
		t.Fatal("首次引导应报告「新建」")
	}
	tp, err := ts.Login(ctx, username, password)
	if err != nil {
		t.Fatalf("登录失败：%v", err)
	}
	return tp.User.ID, password
}

func TestChangePasswordSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, oldPassword := newPasswordAccount(t, ts, "ok")

	const newPassword = "brand-new-password-987"
	revoked, err := ts.ChangePassword(ctx, id, oldPassword, newPassword)
	if err != nil {
		t.Fatalf("改密失败：%v", err)
	}
	if revoked < 1 {
		t.Errorf("改密应吊销至少 1 条 refresh token，实际 %d", revoked)
	}

	// 旧密码必须失效
	username := usernameOf(t, ts, id)
	if _, err := ts.Login(ctx, username, oldPassword); !errors.Is(err, ErrUnauthorized) {
		t.Errorf("旧密码登录应失败（ErrUnauthorized），实际 %v", err)
	}

	// 新密码必须能登录，且身份一致
	tp2, err := ts.Login(ctx, username, newPassword)
	if err != nil {
		t.Fatalf("新密码登录失败：%v", err)
	}
	if tp2.User.ID != id {
		t.Errorf("新令牌对 user id = %d，期望 %d", tp2.User.ID, id)
	}
}

// usernameOf 反查账号用户名（测试辅助）。
func usernameOf(t *testing.T, ts *testService, id int64) string {
	t.Helper()
	var username string
	if err := ts.db.Pool.QueryRow(context.Background(),
		`SELECT username FROM users WHERE id = $1`, id).Scan(&username); err != nil {
		t.Fatalf("查询用户名失败：%v", err)
	}
	return username
}

func TestChangePasswordRevokesRefreshTokens(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, oldPassword := newPasswordAccount(t, ts, "revoke")
	// 登录两次，拿到两条 refresh token
	tp1, err := ts.Login(ctx, usernameOf(t, ts, id), oldPassword)
	if err != nil {
		t.Fatalf("第一次登录失败：%v", err)
	}
	tp2, err := ts.Login(ctx, usernameOf(t, ts, id), oldPassword)
	if err != nil {
		t.Fatalf("第二次登录失败：%v", err)
	}

	if _, err := ts.ChangePassword(ctx, id, oldPassword, "another-new-password-42"); err != nil {
		t.Fatalf("改密失败：%v", err)
	}

	// 改密前签发的两条 refresh token 必须全部失效
	for i, rtk := range []string{tp1.RefreshToken, tp2.RefreshToken} {
		if _, err := ts.Refresh(ctx, rtk); !errors.Is(err, ErrUnauthorized) {
			t.Errorf("第 %d 条旧 refresh token 应已失效（ErrUnauthorized），实际 %v", i+1, err)
		}
	}

	// 库里不应残留未吊销的 refresh token
	var active int
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1 AND revoked_at IS NULL`, id).
		Scan(&active); err != nil {
		t.Fatalf("查询 refresh token 失败：%v", err)
	}
	if active != 0 {
		t.Errorf("改密后应无未吊销 refresh token，实际 %d 条", active)
	}
}

func TestChangePasswordWrongOldPassword(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newPasswordAccount(t, ts, "wrongold")
	_, err := ts.ChangePassword(ctx, id, "wrong-old-password", "brand-new-password-987")
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("旧密码错误应是 ErrBadInput，实际 %v", err)
	}
	if err == nil || err.Error() != "输入非法: 原密码不正确" {
		t.Errorf("错误信息应明确指向原密码，实际 %v", err)
	}
}

func TestChangePasswordValidation(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, oldPassword := newPasswordAccount(t, ts, "validation")
	cases := []struct {
		name    string
		old, pw string
	}{
		{"空原密码", "", "brand-new-password-987"},
		{"空新密码", oldPassword, ""},
		{"新密码太短", oldPassword, "short"},
		{"新旧密码相同", oldPassword, oldPassword},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			_, err := ts.ChangePassword(ctx, id, tc.old, tc.pw)
			if !errors.Is(err, ErrBadInput) {
				t.Errorf("应返回 ErrBadInput，实际 %v", err)
			}
		})
	}
}

func TestChangePasswordGuestAccountRejected(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 游客号：password_hash 为 NULL
	var id int64
	err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO users (guest_token, nickname, is_guest, last_login_at)
		 VALUES ($1, $2, TRUE, now())
		 RETURNING id`,
		fmt.Sprintf("guest_chpwd_%d", time.Now().UnixNano()), "游客").Scan(&id)
	if err != nil {
		t.Fatalf("创建游客号失败：%v", err)
	}

	_, err = ts.ChangePassword(ctx, id, "whatever", "brand-new-password-987")
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("游客号改密应返回 ErrBadInput，实际 %v", err)
	}
	if err == nil || err.Error() != "输入非法: 该账号没有设置密码，无法修改" {
		t.Errorf("错误信息应说明账号无密码，实际 %v", err)
	}
}

func TestChangePasswordBannedAccount(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, oldPassword := newPasswordAccount(t, ts, "banned")
	if _, err := ts.db.Pool.Exec(ctx,
		`UPDATE users SET status = 2 WHERE id = $1`, id); err != nil {
		t.Fatalf("封禁账号失败：%v", err)
	}

	_, err := ts.ChangePassword(ctx, id, oldPassword, "brand-new-password-987")
	if !errors.Is(err, ErrForbidden) {
		t.Fatalf("封禁账号改密应返回 ErrForbidden，实际 %v", err)
	}
}

func TestChangePasswordUnknownUser(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	_, err := ts.ChangePassword(ctx, 999_999_999, "whatever", "brand-new-password-987")
	if !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的用户应返回 ErrNotFound，实际 %v", err)
	}
}

func TestChangePasswordBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	_, err := ts.ChangePassword(ctx, 1, "whatever", "brand-new-password-987")
	if err == nil {
		t.Fatal("连接池已关闭时改密必须报错，不能静默成功")
	}
}

// R100：并发改密的「先读后写」丢失更新。
//
// 缺陷形态：ChangePassword 先 `SELECT password_hash`，然后 bcrypt 校验 +
// 生成新哈希（两步各约 60ms），最后**无条件** `UPDATE users SET
// password_hash=$2 WHERE id=$1`。两个并发请求都能通过「旧密码正确」
// 的检查，再各自写回 —— 后提交者覆盖先提交者，但**两条都返回成功**。
// 先返回成功的那枚新密码从未落库。
//
// 后果不是「谁赢谁输无所谓」：
//   - 玩家拿到成功回执、以为密码已改，下次登录却失败，而旧密码也已失效
//     → 账号临时锁死，服务端没有任何冲突记录（投诉无法复现）；
//   - 运营强制重置密码（账号被盗应急）时，在途的玩家改密会把运营刚写的
//     新哈希静默覆盖掉 —— 强制轮换等于没做。
//
// 注入方式：BEFORE UPDATE OF password_hash 触发器在拿到行锁后 pg_sleep，
// 把「读到旧哈希」与「写回」之间的窗口拉成必现。sleep 必须小于
// `DB_LOCK_TIMEOUT`（默认 2s），否则后一条 UPDATE 会先被 55P03 取消，
// 复现的就不是丢失更新而是锁超时。
//
// 判据：并发改密的成功回执必须恰好 1 条；落库的必须是成功那条的新密码。
func TestChangePasswordConcurrentLostUpdate(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, oldPassword := newPasswordAccount(t, ts, "lostupdate")

	ts.exec(t, `CREATE OR REPLACE FUNCTION r100_slow_password_update() RETURNS trigger AS $$
		BEGIN
			PERFORM pg_sleep(1.2);
			RETURN NEW;
		END;
	$$ LANGUAGE plpgsql`)
	ts.exec(t, fmt.Sprintf(`CREATE TRIGGER r100_slow_password_update_trg
		BEFORE UPDATE OF password_hash ON users
		FOR EACH ROW WHEN (NEW.id = %d) EXECUTE FUNCTION r100_slow_password_update()`, id))
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(),
			`DROP TRIGGER IF EXISTS r100_slow_password_update_trg ON users`)
		_, _ = ts.pool.Exec(context.Background(),
			`DROP FUNCTION IF EXISTS r100_slow_password_update()`)
	})

	const (
		firstNew  = "r100-first-new-password"
		secondNew = "r100-second-new-password"
	)
	type outcome struct {
		name string
		err  error
	}
	results := make(chan outcome, 2)

	go func() {
		_, err := ts.ChangePassword(ctx, id, oldPassword, firstNew)
		results <- outcome{"first", err}
	}()
	// 等第一条读完哈希并进入触发器的 sleep（此时尚未提交）。
	time.Sleep(400 * time.Millisecond)
	go func() {
		_, err := ts.ChangePassword(ctx, id, oldPassword, secondNew)
		results <- outcome{"second", err}
	}()

	got := map[string]error{}
	for i := 0; i < 2; i++ {
		res := <-results
		got[res.name] = res.err
	}

	var hash string
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT password_hash FROM users WHERE id = $1`, id).Scan(&hash); err != nil {
		t.Fatalf("回读密码哈希失败：%v", err)
	}
	firstOK := VerifyPassword(hash, firstNew) == nil
	secondOK := VerifyPassword(hash, secondNew) == nil

	if got["first"] != nil {
		t.Fatalf("先提交的改密必须成功，实际 %v（落库 first=%v second=%v）",
			got["first"], firstOK, secondOK)
	}
	if got["second"] == nil {
		t.Fatalf("并发改密的第二条请求返回成功，但落库 first=%v second=%v —— 第二条的成功回执是假的",
			firstOK, secondOK)
	}
	if !errors.Is(got["second"], ErrBadInput) {
		t.Fatalf("后到的改密应返回 ErrBadInput（密码已被改过），实际 %v", got["second"])
	}
	if !firstOK {
		t.Fatalf("最终密码必须是成功那条的新密码，实际 first=%v second=%v", firstOK, secondOK)
	}
	if secondOK {
		t.Fatalf("失败那条的新密码不应落库，实际 first=%v second=%v", firstOK, secondOK)
	}
}
