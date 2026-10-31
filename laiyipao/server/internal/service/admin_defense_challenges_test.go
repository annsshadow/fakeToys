package service

// 后台防线挑战记录列表的服务层测试（2026-10-31 新一轮第 30 轮）。

import (
	"context"
	"testing"
	"time"
)

func TestAdminListDefenseChallenges(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	owner, _ := newGuestUser(t, ts, "dc_list_owner")
	challengerA, _ := newGuestUser(t, ts, "dc_list_a")
	challengerB, _ := newGuestUser(t, ts, "dc_list_b")
	defID := seedDefense(t, ts, owner, "铁壁")
	now := time.Now()
	seedChallenge(t, ts, defID, challengerA, true, now.Add(-2*time.Hour))
	seedChallenge(t, ts, defID, challengerB, false, now.Add(-1*time.Hour))

	// 无筛选 → 2 条
	items, total, err := ts.AdminListDefenseChallenges(ctx, 50, 0, 0, 0, nil, nil)
	if err != nil {
		t.Fatalf("查询失败：%v", err)
	}
	if total != 2 || len(items) != 2 {
		t.Fatalf("应有 2 条记录，实际 total=%d len=%d", total, len(items))
	}
	// 新记录在前（-1h 的 challengerB）
	if items[0].ChallengerID != challengerB {
		t.Errorf("第一条应是最新的 challengerB，实际 %+v", items[0])
	}
	if items[0].DefenseName != "铁壁" || items[0].ChallengerNickname == "" {
		t.Errorf("字段不完整：%+v", items[0])
	}
	if items[0].Loot["coin"] != 100 {
		t.Errorf("loot 解析错误：%+v", items[0].Loot)
	}

	// 按挑战者过滤 → 1 条
	items, total, err = ts.AdminListDefenseChallenges(ctx, 50, 0, 0, challengerA, nil, nil)
	if err != nil {
		t.Fatalf("挑战者过滤失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].ChallengerID != challengerA {
		t.Errorf("challengerA 应 1 条，实际 total=%d", total)
	}

	// 按防线过滤 → 2 条
	items, total, err = ts.AdminListDefenseChallenges(ctx, 50, 0, defID, 0, nil, nil)
	if err != nil {
		t.Fatalf("防线过滤失败：%v", err)
	}
	if total != 2 {
		t.Errorf("防线过滤应 2 条，实际 %d", total)
	}

	// 按日期范围 [now-1.5h, now) → 只含 -1h 的 challengerB
	from := now.Add(-90 * time.Minute)
	items, total, err = ts.AdminListDefenseChallenges(ctx, 50, 0, 0, 0, &from, nil)
	if err != nil {
		t.Fatalf("日期过滤失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].ChallengerID != challengerB {
		t.Errorf("date_from=-1.5h 应只含 challengerB，实际 total=%d", total)
	}
}

func TestAdminListDefenseChallengesBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, _, err := ts.AdminListDefenseChallenges(ctx, 50, 0, 0, 0, nil, nil); err == nil {
		t.Fatal("连接池已关闭时必须报错，不能静默返回空列表")
	}
}