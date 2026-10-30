package service

// 玩家兑换记录的服务层测试（2026-10-30 实质优化第 21 轮）。
//
// 关键行为锚点：
//   - 返回按时间倒序的兑换记录，含兑换码与奖励快照；
//   - 无记录返回空列表（不是 nil）；
//   - limit 有上界（>50 夹到 20）；
//   - 库故障必须报错而不是静默返回空列表。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func seedRedeemCode(t *testing.T, ts *testService, code string, reward string) int {
	t.Helper()
	ctx := context.Background()
	var id int
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO redeem_codes (code, reward, max_uses)
		 VALUES ($1, $2::jsonb, 100)
		 RETURNING id`, code, reward).Scan(&id); err != nil {
		t.Fatalf("插入兑换码失败：%v", err)
	}
	return id
}

func seedRedemption(t *testing.T, ts *testService, codeID int, userID int64, daysAgo int) {
	t.Helper()
	ctx := context.Background()
	createdAt := time.Now().AddDate(0, 0, -daysAgo)
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO redeem_usages (code_id, user_id, created_at)
		 VALUES ($1, $2, $3)`, codeID, userID, createdAt); err != nil {
		t.Fatalf("插入兑换记录失败：%v", err)
	}
}

func TestMyRedemptionsSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	codeA := fmt.Sprintf("REWARD_%d", time.Now().UnixNano())
	codeB := fmt.Sprintf("KEYS_%d", time.Now().UnixNano())
	idA := seedRedeemCode(t, ts, codeA, `{"coin":1000}`)
	idB := seedRedeemCode(t, ts, codeB, `{"keys":5}`)

	uid, _ := newGuestUser(t, ts, "redemptions")
	seedRedemption(t, ts, idA, uid, 2)
	seedRedemption(t, ts, idB, uid, 1)

	items, err := ts.MyRedemptions(ctx, uid, 20)
	if err != nil {
		t.Fatalf("查询兑换记录失败：%v", err)
	}
	if len(items) != 2 {
		t.Fatalf("应有 2 条记录，实际 %d", len(items))
	}
	// 倒序：最近的（codeB）在前
	if items[0].Code != codeB {
		t.Errorf("第一条应是最新兑换（%s），实际 %+v", codeB, items[0])
	}
	if items[0].Reward["keys"] != 5 {
		t.Errorf("奖励快照 keys = %d，期望 5", items[0].Reward["keys"])
	}
	if items[1].Code != codeA || items[1].Reward["coin"] != 1000 {
		t.Errorf("第二条记录不正确：%+v", items[1])
	}
}

func TestMyRedemptionsEmpty(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "redemptions_empty")
	items, err := ts.MyRedemptions(ctx, uid, 20)
	if err != nil {
		t.Fatalf("查询兑换记录失败：%v", err)
	}
	if items == nil || len(items) != 0 {
		t.Errorf("无记录应返回空列表（非 nil），实际 %v", items)
	}
}

func TestMyRedemptionsLimitCap(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "redemptions_cap")
	// 同一用户对同一码只有一条记录（UNIQUE(code_id, user_id)），
	// 所以用 25 个不同的码造 25 条记录。
	for i := 0; i < 25; i++ {
		code := fmt.Sprintf("CAP_%d_%d", time.Now().UnixNano(), i)
		id := seedRedeemCode(t, ts, code, `{"coin":1}`)
		seedRedemption(t, ts, id, uid, i)
	}

	items, err := ts.MyRedemptions(ctx, uid, 1000)
	if err != nil {
		t.Fatalf("查询兑换记录失败：%v", err)
	}
	if len(items) != 20 {
		t.Errorf("limit 超上限应夹到 20 条，实际 %d", len(items))
	}
}

func TestMyRedemptionsBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	_, err := ts.MyRedemptions(ctx, 1, 20)
	if err == nil {
		t.Fatal("连接池已关闭时查询必须报错，不能静默返回空列表")
	}
}
