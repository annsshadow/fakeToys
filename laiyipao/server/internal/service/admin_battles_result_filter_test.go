package service

// 后台战报列表/导出按结果（胜/负）过滤的测试（2026-11-01 新一轮第 44 轮）。
//
// 运营此前只能按 user_id / level_id / only_mismatched / 日期过滤，
// 想查「这个用户赢过哪些局 / 哪些关卡的胜率低」只能翻页或导出后
// 在 Excel 里筛。新增可选的 result（win / lose，空串不过滤）。

import (
	"context"
	"strings"
	"testing"
	"time"
)

func TestAdminListBattlesResultFilter(t *testing.T) {
	ts := openScratchService(t) // 一次性库，避免共享库的跨测试数据污染（userID=0 会导出所有用户）
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "bresult")
	now := time.Now()
	winID := seedBattleFor(t, ts, uid, 1, "win", 3, now.Add(-time.Hour))
	loseID := seedBattleFor(t, ts, uid, 2, "lose", 0, now.Add(-time.Hour))

	// result=win：只含胜利战报
	list, total, err := ts.AdminListBattles(ctx, 0, 0, 50, false, 0, nil, nil, "win")
	if err != nil {
		t.Fatalf("result=win 过滤失败：%v", err)
	}
	if total != 1 || len(list) != 1 || list[0].ID != winID {
		t.Errorf("result=win 应只含胜利战报，实际 total=%d list=%+v", total, list)
	}

	// result=lose：只含失败战报
	list, total, err = ts.AdminListBattles(ctx, 0, 0, 50, false, 0, nil, nil, "lose")
	if err != nil {
		t.Fatalf("result=lose 过滤失败：%v", err)
	}
	if total != 1 || len(list) != 1 || list[0].ID != loseID {
		t.Errorf("result=lose 应只含失败战报，实际 total=%d list=%+v", total, list)
	}

	// result 空串：不过滤，两条都在
	list, total, err = ts.AdminListBattles(ctx, 0, 0, 50, false, 0, nil, nil, "")
	if err != nil {
		t.Fatalf("result 空串查询失败：%v", err)
	}
	if total != 2 {
		t.Errorf("result 空串应返回全部战报，实际 total=%d", total)
	}
}

func TestAdminExportBattlesResultFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "bresult_exp")
	now := time.Now()
	seedBattleFor(t, ts, uid, 1, "win", 3, now.Add(-time.Hour))
	seedBattleFor(t, ts, uid, 2, "lose", 0, now.Add(-time.Hour))

	// 导出 result=win：CSV 数据行只含 win
	data, err := ts.AdminExportBattles(ctx, 0, 0, false, nil, nil, "win")
	if err != nil {
		t.Fatalf("result=win 导出失败：%v", err)
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 2 {
		t.Fatalf("result=win 导出行数应为 2（1 表头 + 1 数据），实际 %d", len(lines))
	}
	// 数据行第 5 列是 result（表头 id,user_id,nickname,level_id,result,...）
	cols := strings.Split(lines[1], ",")
	if len(cols) < 5 || cols[4] != "win" {
		t.Errorf("导出行应为 win，实际 %v", cols)
	}

	// 导出 result 空串：两条都在
	data, err = ts.AdminExportBattles(ctx, 0, 0, false, nil, nil, "")
	if err != nil {
		t.Fatalf("result 空串导出失败：%v", err)
	}
	lines = strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 3 {
		t.Errorf("result 空串导出行数应为 3（1 表头 + 2 数据），实际 %d", len(lines))
	}
}