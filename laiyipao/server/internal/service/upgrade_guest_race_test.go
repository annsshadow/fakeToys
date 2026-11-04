// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"testing"
	"time"
)

// R35：游客转正撞上唯一约束时，必须映射成 ErrBadInput 而不是 500。
//
// 「先查后写」在并发下必然有窗口：两个游客同时用同一个用户名转正，
// 两次预查都看到「没人占用」，最后由 `idx_users_username` 决定胜负。
// 输掉的那次此前被包成普通 error → HTTP 500「服务内部错误」，
// 玩家会以为系统坏了并反复重试，而不是换一个用户名。
//
// 这里用故障注入把「输掉竞争的那一刻」变成确定性场景：BEFORE UPDATE
// 触发器按 23505 抛出与真实唯一索引同码的错误。
// 同时断言用户行没有任何半成品（用户名/密码/`is_guest` 都没变）。
func TestUpgradeGuestAccountUniqueViolationIsBadInput(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	tp, err := ts.GuestLogin(ctx, "", "并发转正")
	if err != nil {
		t.Fatalf("建号失败：%v", err)
	}
	uid := tp.User.ID
	username := fmt.Sprintf("race_%d", time.Now().UnixNano())

	ts.exec(t, `CREATE OR REPLACE FUNCTION simulate_username_taken() RETURNS trigger AS $$
		BEGIN
			RAISE EXCEPTION 'duplicate key value violates unique constraint "idx_users_username"'
				USING ERRCODE = '23505';
		END;
	$$ LANGUAGE plpgsql`)
	ts.exec(t, fmt.Sprintf(`CREATE TRIGGER simulate_username_taken_trg BEFORE UPDATE OF username ON users
		FOR EACH ROW WHEN (NEW.id = %d) EXECUTE FUNCTION simulate_username_taken()`, uid))
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(),
			`DROP TRIGGER IF EXISTS simulate_username_taken_trg ON users`)
	})

	err = ts.UpgradeGuestAccount(ctx, uid, username, "long-enough-pass")
	if err == nil {
		t.Fatal("用户名冲突时必须失败")
	}
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("用户名冲突必须映射为 ErrBadInput（HTTP 400），实际：%v", err)
	}

	// 没有半成品：仍然是游客、无用户名、无密码哈希。
	var (
		gotUsername *string
		gotHash     *string
		isGuest     bool
	)
	if err := ts.pool.QueryRow(ctx,
		`SELECT username, password_hash, is_guest FROM users WHERE id = $1`, uid).
		Scan(&gotUsername, &gotHash, &isGuest); err != nil {
		t.Fatalf("回读用户失败：%v", err)
	}
	if gotUsername != nil || gotHash != nil || !isGuest {
		t.Fatalf("转正失败后用户行必须保持游客态：username=%v hash=%v is_guest=%v",
			gotUsername, gotHash, isGuest)
	}
}

// R101：游客转正的「先读后写」丢失更新（同 R100 的形态，后果更重）。
//
// 缺陷形态：UpgradeGuestAccount 先 `SELECT password_hash, status` 判定
// 「仍是纯游客」，再查用户名占用，然后 bcrypt 生成新哈希（约 60ms），
// 最后**无条件** `UPDATE users SET username=$2, password_hash=$3,
// is_guest=FALSE WHERE id=$1`。同一游客账号的两笔并发转正都会通过
// 「仍是游客」的检查，各自写回 —— 后提交者把先提交者刚绑定的身份
// （用户名 + 密码）整体覆盖，而**两条都返回成功**。
//
// 这条是**丢失更新**，不是「重复绑定」：users 里只有一行，只可能有一个
// username；跨账号抢同一用户名的重复绑定由 `idx_users_username`
// 部分唯一索引挡住（R35 已覆盖）。顺序调用两次也不会出问题——第二次
// 的预查会看到 password_hash 非 NULL 并拒绝；错误只存在于「读」与「写」
// 之间的并发窗口。
//
// 后果（比 R100 的密码问题更难解释，因为涉及数据归属）：
//   - 先提交者拿到「转正成功」，但库里绑定的不是他填的用户名/密码；
//     他用自己填的凭据登录 → 用户名或密码错误；游客 token 还在，所以他
//     还能进同一个号，但会坚信自己已经转正、账号已安全；
//   - 后提交者也可能被更晚的请求覆盖 —— 两条成功回执里只有最后一条为真，
//     客服查库只能看到一个合法身份，无法判断哪条回执是假的。
//
// 注入方式：BEFORE UPDATE OF username 触发器在拿到行锁后 pg_sleep(1.2s)
// （压在同一套 `DB_LOCK_TIMEOUT` 2s 之下，否则复现成锁超时）。判据：
// 恰好 1 条成功，另一条 ErrBadInput；落库身份必须是成功那条的。
func TestUpgradeGuestAccountConcurrentLostUpdate(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	tp, err := ts.GuestLogin(ctx, "", "R101 并发转正")
	if err != nil {
		t.Fatalf("建号失败：%v", err)
	}
	uid := tp.User.ID

	ts.exec(t, `CREATE OR REPLACE FUNCTION r101_slow_guest_upgrade() RETURNS trigger AS $$
		BEGIN
			PERFORM pg_sleep(1.2);
			RETURN NEW;
		END;
	$$ LANGUAGE plpgsql`)
	ts.exec(t, fmt.Sprintf(`CREATE TRIGGER r101_slow_guest_upgrade_trg
		BEFORE UPDATE OF username ON users
		FOR EACH ROW WHEN (NEW.id = %d) EXECUTE FUNCTION r101_slow_guest_upgrade()`, uid))
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(),
			`DROP TRIGGER IF EXISTS r101_slow_guest_upgrade_trg ON users`)
		_, _ = ts.pool.Exec(context.Background(),
			`DROP FUNCTION IF EXISTS r101_slow_guest_upgrade()`)
	})

	stamp := time.Now().UnixNano()
	firstUser := fmt.Sprintf("r101_first_%d", stamp)
	secondUser := fmt.Sprintf("r101_second_%d", stamp)
	const (
		firstPass  = "r101-first-password"
		secondPass = "r101-second-password"
	)

	type outcome struct {
		name string
		err  error
	}
	results := make(chan outcome, 2)

	go func() {
		results <- outcome{"first", ts.UpgradeGuestAccount(ctx, uid, firstUser, firstPass)}
	}()
	// 等第一条读完账号状态并进入触发器的 sleep（此时尚未提交）。
	time.Sleep(400 * time.Millisecond)
	go func() {
		results <- outcome{"second", ts.UpgradeGuestAccount(ctx, uid, secondUser, secondPass)}
	}()

	got := map[string]error{}
	for i := 0; i < 2; i++ {
		res := <-results
		got[res.name] = res.err
	}

	var (
		gotUsername *string
		gotHash     *string
		isGuest     bool
	)
	if err := ts.pool.QueryRow(ctx,
		`SELECT username, password_hash, is_guest FROM users WHERE id = $1`, uid).
		Scan(&gotUsername, &gotHash, &isGuest); err != nil {
		t.Fatalf("回读用户失败：%v", err)
	}
	firstOK := gotHash != nil && VerifyPassword(*gotHash, firstPass) == nil
	secondOK := gotHash != nil && VerifyPassword(*gotHash, secondPass) == nil
	boundTo := "nil"
	if gotUsername != nil {
		boundTo = *gotUsername
	}

	if got["first"] != nil {
		t.Fatalf("先提交的转正必须成功，实际 %v（落库 username=%q is_guest=%v first=%v second=%v）",
			got["first"], boundTo, isGuest, firstOK, secondOK)
	}
	if got["second"] == nil {
		t.Fatalf("并发转正的第二条返回成功，但落库 username=%q is_guest=%v first=%v second=%v —— 第二条的成功回执是假的",
			boundTo, isGuest, firstOK, secondOK)
	}
	if !errors.Is(got["second"], ErrBadInput) {
		t.Fatalf("后到的转正应返回 ErrBadInput，实际 %v", got["second"])
	}
	if !strings.Contains(got["second"].Error(), "已完成转正") {
		t.Fatalf("后到的转正文案应明确「已完成转正」，实际 %v", got["second"])
	}
	if gotUsername == nil || *gotUsername != firstUser {
		t.Fatalf("落库身份必须属于成功那条：期望 %q，实际 %q（first=%v second=%v）",
			firstUser, boundTo, firstOK, secondOK)
	}
	if !firstOK || secondOK {
		t.Fatalf("落库密码必须是成功那条的（first=%v second=%v）", firstOK, secondOK)
	}
	if isGuest {
		t.Fatal("转正后 is_guest 必须为 false")
	}

	// 顺序重复提交（同一游客账号再次转正）也必须被拒绝 —— 这条不靠 CAS，
	// 靠预查看到 password_hash 非 NULL；CAS 只是把同一判据挪到写入时刻。
	err = ts.UpgradeGuestAccount(ctx, uid, secondUser, secondPass)
	if !errors.Is(err, ErrBadInput) {
		t.Fatalf("顺序重复转正必须返回 ErrBadInput，实际 %v", err)
	}
}
