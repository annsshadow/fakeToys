// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"errors"
	"fmt"
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
