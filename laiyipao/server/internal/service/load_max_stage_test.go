package service

// 第 142 轮：LoadMaxStage 是玩家进度的服务端权威读取。
// 判据：写入 42 后读回 42；从未有进度记录的用户得 0（不是错误、不是 -1）。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func TestLoadMaxStage(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	newUser := func(tag string) int64 {
		var uid int64
		if err := ts.db.Pool.QueryRow(ctx,
			`INSERT INTO users (guest_token, nickname, is_guest) VALUES ($1,$2,TRUE) RETURNING id`,
			fmt.Sprintf("maxstage_%s_%d", tag, time.Now().UnixNano()), tag).Scan(&uid); err != nil {
			t.Fatalf("造用户失败：%v", err)
		}
		t.Cleanup(func() {
			cctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
			defer cancel()
			_, _ = ts.db.Pool.Exec(cctx, `DELETE FROM user_progress WHERE user_id = $1`, uid)
			_, _ = ts.db.Pool.Exec(cctx, `DELETE FROM users WHERE id = $1`, uid)
		})
		return uid
	}

	// 从未有进度记录 → 0
	uid0 := newUser("fresh")
	if n, err := ts.LoadMaxStage(ctx, uid0); err != nil || n != 0 {
		t.Errorf("无进度用户应得 0，实际 %d err=%v", n, err)
	}

	// 通关 42 关 → 42
	uid1 := newUser("clear")
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO user_progress (user_id, max_stage) VALUES ($1, 42)
		 ON CONFLICT (user_id) DO UPDATE SET
		   max_stage = GREATEST(user_progress.max_stage, EXCLUDED.max_stage)`, uid1); err != nil {
		t.Fatalf("写进度失败：%v", err)
	}
	if n, err := ts.LoadMaxStage(ctx, uid1); err != nil || n != 42 {
		t.Errorf("应为 42，实际 %d err=%v", n, err)
	}

	// 属主隔离：uid1 的 42 不得泄漏给 uid0
	if n, _ := ts.LoadMaxStage(ctx, uid0); n != 0 {
		t.Errorf("uid0 应仍是 0（进度属主隔离），实际 %d", n)
	}
}
