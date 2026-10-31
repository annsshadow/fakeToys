package service

// 玩家「我的防线被挑战」历史记录的服务层测试（2026-10-31 新一轮第 3 轮）。
//
// 关键行为锚点：
//   - 返回按时间倒序的被挑战记录，含挑战者昵称/防线名/胜负/偷取明细；
//   - 无防线或无可挑战记录返回空列表（不是 nil）；
//   - 只能看到**自己防线**的挑战（按 defenses.owner_id 过滤）；
//   - limit 有上界（>50 夹到 20）；
//   - 库故障必须报错而不是静默返回空列表。

import (
	"context"
	"testing"
	"time"
)

func TestMyDefenseChallengesSucceeds(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	ownerID, _ := newGuestUser(t, ts, "defch_owner")
	challengerID, _ := newGuestUser(t, ts, "defch_me")
	defID := seedDefense(t, ts, ownerID, "我的铁壁")

	// 两条记录：一条输（较新，挑战者获胜）、一条赢（较旧，挑战者落败）
	seedChallenge(t, ts, defID, challengerID, true, time.Now().Add(-1*time.Hour))
	seedChallenge(t, ts, defID, challengerID, false, time.Now().Add(-2*time.Hour))

	items, err := ts.MyDefenseChallenges(ctx, ownerID, 20)
	if err != nil {
		t.Fatalf("查询被挑战历史失败：%v", err)
	}
	if len(items) != 2 {
		t.Fatalf("应有 2 条记录，实际 %d", len(items))
	}
	// 倒序：新的在前
	if !items[0].Won {
		t.Error("第一条应是最新（挑战者获胜）的记录")
	}
	if items[1].Won {
		t.Error("第二条应是较旧（挑战者落败）的记录")
	}
	// 字段完整性
	first := items[0]
	if first.DefenseName != "我的铁壁" {
		t.Errorf("防线名 = %q，期望 我的铁壁", first.DefenseName)
	}
	if first.ChallengerNickname == "" {
		t.Error("挑战者昵称不应为空")
	}
	if first.Loot["coin"] != 100 {
		t.Errorf("偷取明细 coin = %d，期望 100", first.Loot["coin"])
	}
}

func TestMyDefenseChallengesNoDefense(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 没布置防线的玩家：返回空列表，不是错误也不是 nil
	id, _ := newGuestUser(t, ts, "defch_nodef")
	items, err := ts.MyDefenseChallenges(ctx, id, 20)
	if err != nil {
		t.Fatalf("查询被挑战历史失败：%v", err)
	}
	if items == nil || len(items) != 0 {
		t.Errorf("无防线应返回空列表（非 nil），实际 %v", items)
	}
}

func TestMyDefenseChallengesEmpty(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	ownerID, _ := newGuestUser(t, ts, "defch_empty")
	_ = seedDefense(t, ts, ownerID, "防线")
	items, err := ts.MyDefenseChallenges(ctx, ownerID, 20)
	if err != nil {
		t.Fatalf("查询被挑战历史失败：%v", err)
	}
	if items == nil || len(items) != 0 {
		t.Errorf("无记录应返回空列表（非 nil），实际 %v", items)
	}
}

func TestMyDefenseChallengesOwnershipOnly(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	ownerA, _ := newGuestUser(t, ts, "defch_own_a")
	ownerB, _ := newGuestUser(t, ts, "defch_own_b")
	challengerID, _ := newGuestUser(t, ts, "defch_own_c")
	defA := seedDefense(t, ts, ownerA, "A 的防线")
	defB := seedDefense(t, ts, ownerB, "B 的防线")

	// 两人的防线都被挑战，但 ownerA 只能看到自己防线的记录
	seedChallenge(t, ts, defA, challengerID, true, time.Now())
	seedChallenge(t, ts, defB, challengerID, false, time.Now())

	itemsA, err := ts.MyDefenseChallenges(ctx, ownerA, 20)
	if err != nil {
		t.Fatalf("查询 A 的被挑战历史失败：%v", err)
	}
	if len(itemsA) != 1 {
		t.Fatalf("A 应只有 1 条记录，实际 %d", len(itemsA))
	}
	if itemsA[0].DefenseName != "A 的防线" {
		t.Errorf("A 的记录防线名 = %q，期望 A 的防线", itemsA[0].DefenseName)
	}
}

func TestMyDefenseChallengesLimitCap(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	ownerID, _ := newGuestUser(t, ts, "defch_cap_owner")
	challengerID, _ := newGuestUser(t, ts, "defch_cap_me")
	defID := seedDefense(t, ts, ownerID, "防线")
	for i := 0; i < 25; i++ {
		seedChallenge(t, ts, defID, challengerID, i%2 == 0, time.Now().Add(-time.Duration(i)*time.Minute))
	}

	items, err := ts.MyDefenseChallenges(ctx, ownerID, 1000)
	if err != nil {
		t.Fatalf("查询被挑战历史失败：%v", err)
	}
	if len(items) != 20 {
		t.Errorf("limit 超上限应夹到 20 条，实际 %d", len(items))
	}
}

func TestMyDefenseChallengesBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	_, err := ts.MyDefenseChallenges(ctx, 1, 20)
	if err == nil {
		t.Fatal("连接池已关闭时查询必须报错，不能静默返回空列表")
	}
}