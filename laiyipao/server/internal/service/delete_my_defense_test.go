package service

// 玩家删除自己防线的服务层测试（2026-10-31 实质优化第 32 轮）。
//
// 关键行为锚点：
//   - 删除后防线从库中消失；
//   - 未设置防线 → ErrNotFound（区别于「删除成功但什么都没发生」）；
//   - 库故障必须报错而不是静默成功。

import (
	"context"
	"errors"
	"testing"
)

func TestDeleteMyDefenseSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	ownerID, _ := newGuestUser(t, ts, "mydeldel")
	defID := seedDefense(t, ts, ownerID, "我的防线")

	if err := ts.DeleteMyDefense(ctx, ownerID); err != nil {
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

func TestDeleteMyDefenseNotFound(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	ownerID, _ := newGuestUser(t, ts, "mydel_none")
	err := ts.DeleteMyDefense(ctx, ownerID)
	if !errors.Is(err, ErrNotFound) {
		t.Fatalf("未设置防线应返回 ErrNotFound，实际 %v", err)
	}
}

func TestDeleteMyDefenseBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	err := ts.DeleteMyDefense(ctx, 1)
	if err == nil {
		t.Fatal("连接池已关闭时删除必须报错，不能静默成功")
	}
}
