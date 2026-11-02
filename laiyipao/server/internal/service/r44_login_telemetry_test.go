// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R44：`last_login_at` 是统计字段，与令牌签发有意不同事务，且不得阻断登录。
//
// 修前 `Login` 里这条 UPDATE 失败会直接返回错误：账号密码都正确、账号状态
// 正常，玩家却拿不到任何令牌 —— 这就是 B2 要消灭的「既不能用新也不能用旧」
// 的锁死形态。修后失败只打日志，登录与签发照常。
//
// 证据方式：用触发器让「改 last_login_at」这条 UPDATE 必失败，
// 然后断言登录成功、refresh token +1、时间戳保持原值。
// 触发器必须能真实生效 —— 删掉触发器后同一次登录能把时间戳推进。

import (
	"context"
	"testing"
	"time"
)

func TestLoginSurvivesLastLoginAtWriteFailure(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	const username = "r44_login_telemetry"
	created, err := ts.EnsureBootstrapPlayer(ctx, username, "test-password-12345")
	if err != nil {
		t.Fatalf("引导账号失败：%v", err)
	}
	if !created {
		t.Fatal("首次引导应新建账号")
	}

	var uid int64
	var beforeLoginAt time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT id, last_login_at FROM users WHERE username = $1`, username).
		Scan(&uid, &beforeLoginAt); err != nil {
		t.Fatalf("读取基线失败：%v", err)
	}

	// 先证明触发器真的能拦住这条 UPDATE（否则后面的「登录仍成功」没有意义）。
	ts.exec(t, `CREATE OR REPLACE FUNCTION r44_block_last_login() RETURNS trigger AS $$
		BEGIN
			IF NEW.last_login_at IS DISTINCT FROM OLD.last_login_at THEN
				RAISE EXCEPTION 'r44 injected last_login_at failure';
			END IF;
			RETURN NEW;
		END
	$$ LANGUAGE plpgsql`)
	ts.exec(t, `CREATE TRIGGER r44_block_last_login BEFORE UPDATE ON users
		FOR EACH ROW EXECUTE FUNCTION r44_block_last_login()`)
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(context.Background(),
			`DROP TRIGGER IF EXISTS r44_block_last_login ON users`)
		_, _ = ts.pool.Exec(context.Background(),
			`DROP FUNCTION IF EXISTS r44_block_last_login()`)
	})

	if _, err := ts.pool.Exec(ctx,
		`UPDATE users SET last_login_at = now() WHERE id = $1`, uid); err == nil {
		t.Fatal("触发器未生效：直接更新 last_login_at 竟然成功")
	}

	var tokensBefore int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1`, uid).
		Scan(&tokensBefore); err != nil {
		t.Fatalf("统计 refresh token 失败：%v", err)
	}

	// 关键断言：统计写入失败不得阻断登录。
	tp, err := ts.Login(ctx, username, "test-password-12345")
	if err != nil {
		t.Fatalf("last_login_at 更新失败不应阻断登录，实际报错：%v", err)
	}
	if tp.AccessToken == "" || tp.RefreshToken == "" {
		t.Error("登录成功必须签发完整令牌对")
	}

	var tokensAfter int64
	if err := ts.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM refresh_tokens WHERE user_id = $1`, uid).
		Scan(&tokensAfter); err != nil {
		t.Fatalf("统计 refresh token 失败：%v", err)
	}
	if tokensAfter != tokensBefore+1 {
		t.Errorf("登录成功应新增 1 张 refresh token：改前 %d 改后 %d", tokensBefore, tokensAfter)
	}

	var stillOld time.Time
	if err := ts.pool.QueryRow(ctx,
		`SELECT last_login_at FROM users WHERE id = $1`, uid).Scan(&stillOld); err != nil {
		t.Fatalf("回读 last_login_at 失败：%v", err)
	}
	if !stillOld.Equal(beforeLoginAt) {
		t.Errorf("注入失败时 last_login_at 应保持原值：改前 %v 现在 %v", beforeLoginAt, stillOld)
	}

	// 故障解除后，同一次登录必须能把时间戳推进 —— 证明上面「保持不变」
	// 是注入造成的，而不是代码根本不再更新它。
	ts.exec(t, `DROP TRIGGER r44_block_last_login ON users`)
	if _, err := ts.Login(ctx, username, "test-password-12345"); err != nil {
		t.Fatalf("故障解除后登录失败：%v", err)
	}
	var advanced bool
	if err := ts.pool.QueryRow(ctx,
		`SELECT last_login_at > $2 FROM users WHERE id = $1`,
		uid, beforeLoginAt).Scan(&advanced); err != nil {
		t.Fatalf("比较 last_login_at 失败：%v", err)
	}
	if !advanced {
		t.Error("故障解除后 last_login_at 应被推进")
	}
}
