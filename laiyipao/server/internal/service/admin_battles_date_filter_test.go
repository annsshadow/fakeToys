package service

// 后台战报列表/导出按日期范围过滤的测试（2026-10-31 新一轮第 14 轮）。
//
// 运营此前只能按 user_id / level_id / only_mismatched 过滤，
// 想查「今天/某时间段的战报」只能翻页。新增可选的
// date_from / date_to（RFC3339 或 YYYY-MM-DD，开区间 < date_to）。

import (
	"context"
	"strings"
	"testing"
	"time"
)

func TestAdminListBattlesDateFilter(t *testing.T) {
	ts := openScratchService(t) // 一次性库，避免共享库的跨测试数据污染（userID=0 会导出所有用户）
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "bdate")
	now := time.Now()
	oldID := seedBattleFor(t, ts, uid, 1, "win", 1, now.Add(-3*time.Hour))
	midID := seedBattleFor(t, ts, uid, 2, "win", 2, now.Add(-2*time.Hour))
	_ = seedBattleFor(t, ts, uid, 3, "win", 3, now.Add(-1*time.Hour))

	from := now.Add(-150 * time.Minute) // -2.5h
	to := now.Add(-90 * time.Minute)    // -1.5h

	// 只传 date_from：包含 mid（-2h）与 new（-1h），不含 old（-3h）
	list, total, err := ts.AdminListBattles(ctx, 0, 0, 50, false, 0, &from, nil)
	if err != nil {
		t.Fatalf("date_from 过滤失败：%v", err)
	}
	if total != 2 {
		t.Errorf("date_from=-2.5h 应剩 2 条（-2h/-1h），实际 %d", total)
	}
	for _, b := range list {
		if b.ID == oldID {
			t.Errorf("date_from 过滤不应包含 -3h 的战报")
		}
	}

	// 只传 date_to（开区间 <）：应只含 old（-3h）
	list, total, err = ts.AdminListBattles(ctx, 0, 0, 50, false, 0, nil, &from)
	if err != nil {
		t.Fatalf("date_to 过滤失败：%v", err)
	}
	if total != 1 || len(list) != 1 || list[0].ID != oldID {
		t.Errorf("date_to=-2.5h 应只含 -3h 战报，实际 total=%d list=%+v", total, list)
	}

	// 双端 [from, to)：应只含 mid（-2h）
	list, total, err = ts.AdminListBattles(ctx, 0, 0, 50, false, 0, &from, &to)
	if err != nil {
		t.Fatalf("双端过滤失败：%v", err)
	}
	if total != 1 || len(list) != 1 || list[0].ID != midID {
		t.Errorf("双端 [-2.5h,-1.5h) 应只含 -2h 战报，实际 total=%d list=%+v", total, list)
	}
}

func TestAdminExportBattlesDateFilter(t *testing.T) {
	ts := openScratchService(t) // 一次性库，避免共享库的跨测试数据污染
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "bdate_exp")
	now := time.Now()
	_ = seedBattleFor(t, ts, uid, 1, "win", 1, now.Add(-3*time.Hour))
	seedBattleFor(t, ts, uid, 2, "win", 2, now.Add(-2*time.Hour))
	seedBattleFor(t, ts, uid, 3, "win", 3, now.Add(-1*time.Hour))

	from := now.Add(-150 * time.Minute)
	data, err := ts.AdminExportBattles(ctx, 0, 0, false, &from, nil)
	if err != nil {
		t.Fatalf("date_from 导出失败：%v", err)
	}
	// CSV 首行是表头；范围内应有 2 条战报
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 3 {
		t.Errorf("date_from=-2.5h 导出行数应为 3（1 表头 + 2 数据），实际 %d", len(lines))
	}
}