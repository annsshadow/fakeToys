package service

// 玩家挑战历史记录的服务层测试（2026-10-30 实质优化第 18 轮）。
//
// 关键行为锚点：
//   - 返回按时间倒序的挑战记录，含对手昵称/防线名/胜负/偷取明细；
//   - 无记录返回空列表（不是 nil）；
//   - limit 有上界（>50 夹到 20）；
//   - 库故障必须报错而不是静默返回空列表。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

// seedDefense 直接插入一条防线，返回 id。
func seedDefense(t *testing.T, ts *testService, ownerID int64, name string) int64 {
	t.Helper()
	ctx := context.Background()
	var id int64
	if err := ts.db.Pool.QueryRow(ctx,
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, $2, 1000, '{}'::jsonb, $3)
		 RETURNING id`, ownerID, name, fmt.Sprintf("snap_%d", time.Now().UnixNano())).Scan(&id); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}
	return id
}

// seedChallenge 直接插入一条挑战记录。
func seedChallenge(t *testing.T, ts *testService, defenseID, challengerID int64, won bool, createdAt time.Time) {
	t.Helper()
	ctx := context.Background()
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO defense_challenges (defense_id, challenger_id, seed, won, duration_ms, hp_left_pct, loot, settled, created_at)
		 VALUES ($1, $2, 1, $3, 1000, 80, '{"coin":100}'::jsonb, TRUE, $4)`,
		defenseID, challengerID, won, createdAt); err != nil {
		t.Fatalf("插入挑战记录失败：%v", err)
	}
}

func TestMyChallengeHistorySucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	ownerID, _ := newGuestUser(t, ts, "chist_owner")
	meID, _ := newGuestUser(t, ts, "chist_me")
	defID := seedDefense(t, ts, ownerID, "铁壁防线")

	// 两条记录：一条赢（较新）、一条输（较旧）
	seedChallenge(t, ts, defID, meID, false, time.Now().Add(-2*time.Hour))
	seedChallenge(t, ts, defID, meID, true, time.Now().Add(-1*time.Hour))

	items, err := ts.MyChallengeHistory(ctx, meID, 20)
	if err != nil {
		t.Fatalf("查询挑战历史失败：%v", err)
	}
	if len(items) != 2 {
		t.Fatalf("应有 2 条记录，实际 %d", len(items))
	}
	// 倒序：新的在前
	if !items[0].Won {
		t.Error("第一条应是最新（赢）的记录")
	}
	if items[1].Won {
		t.Error("第二条应是较旧（输）的记录")
	}
	// 字段完整性
	first := items[0]
	if first.DefenseName != "铁壁防线" {
		t.Errorf("防线名 = %q，期望 铁壁防线", first.DefenseName)
	}
	if first.OwnerNickname == "" {
		t.Error("对手昵称不应为空")
	}
	if first.OwnerPower != 1000 {
		t.Errorf("对手战力 = %d，期望 1000", first.OwnerPower)
	}
	if first.Loot["coin"] != 100 {
		t.Errorf("偷取明细 coin = %d，期望 100", first.Loot["coin"])
	}
}

func TestMyChallengeHistoryEmpty(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	id, _ := newGuestUser(t, ts, "chist_empty")
	items, err := ts.MyChallengeHistory(ctx, id, 20)
	if err != nil {
		t.Fatalf("查询挑战历史失败：%v", err)
	}
	if items == nil || len(items) != 0 {
		t.Errorf("无记录应返回空列表（非 nil），实际 %v", items)
	}
}

func TestMyChallengeHistoryLimitCap(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	ownerID, _ := newGuestUser(t, ts, "chist_cap_owner")
	meID, _ := newGuestUser(t, ts, "chist_cap_me")
	defID := seedDefense(t, ts, ownerID, "防线")
	for i := 0; i < 25; i++ {
		seedChallenge(t, ts, defID, meID, i%2 == 0, time.Now().Add(-time.Duration(i)*time.Minute))
	}

	// limit=1000 应被夹到 20
	items, err := ts.MyChallengeHistory(ctx, meID, 1000)
	if err != nil {
		t.Fatalf("查询挑战历史失败：%v", err)
	}
	if len(items) != 20 {
		t.Errorf("limit 超上限应夹到 20 条，实际 %d", len(items))
	}
}

func TestMyChallengeHistoryBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	_, err := ts.MyChallengeHistory(ctx, 1, 20)
	if err == nil {
		t.Fatal("连接池已关闭时查询必须报错，不能静默返回空列表")
	}
}
