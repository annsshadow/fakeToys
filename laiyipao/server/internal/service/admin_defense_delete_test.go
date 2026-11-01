package service

// 后台删除玩家防线的服务层测试（2026-10-31 实质优化第 26 轮）。
//
// 关键行为锚点：
//   - 删除后防线从库中消失（挑战记录随外键 CASCADE 删除）；
//   - 不存在的防线 → ErrNotFound；
//   - 库故障必须报错而不是静默成功。

import (
	"context"
	"errors"
	"testing"
)

func TestAdminDeleteDefenseSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	ownerID, _ := newGuestUser(t, ts, "defdel_owner")
	defID := seedDefense(t, ts, ownerID, "违规防线")

	if err := ts.AdminDeleteDefense(ctx, defID); err != nil {
		t.Fatalf("删除防线失败：%v", err)
	}

	var n int64
	if err := ts.db.Pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM defenses WHERE id = $1`, defID).Scan(&n); err != nil {
		t.Fatalf("查询防线失败：%v", err)
	}
	if n != 0 {
		t.Error("删除后防线仍存在")
	}
}

func TestAdminDeleteDefenseNotFound(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	err := ts.AdminDeleteDefense(ctx, 999_999)
	if !errors.Is(err, ErrNotFound) {
		t.Fatalf("不存在的防线应返回 ErrNotFound，实际 %v", err)
	}
}

func TestAdminDeleteDefenseBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	err := ts.AdminDeleteDefense(ctx, 1)
	if err == nil {
		t.Fatal("连接池已关闭时删除必须报错，不能静默成功")
	}
}
