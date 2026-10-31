package service

// 后台防线挑战记录 CSV 导出的服务层测试（2026-10-31 新一轮第 32 轮）。

import (
	"context"
	"strings"
	"testing"
	"time"
)

func TestAdminExportDefenseChallenges(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	owner, _ := newGuestUser(t, ts, "dcexp_owner")
	challenger, _ := newGuestUser(t, ts, "dcexp_ch")
	defID := seedDefense(t, ts, owner, "铁壁")
	seedChallenge(t, ts, defID, challenger, true, time.Now().Add(-time.Hour))

	data, err := ts.AdminExportDefenseChallenges(ctx, 0, 0, nil, nil)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	// 表头 + 1 行数据
	if len(lines) != 2 {
		t.Fatalf("应有 2 行（1 表头 + 1 数据），实际 %d", len(lines))
	}
	if !strings.HasPrefix(string(data), "\xEF\xBB\xBF") {
		t.Error("CSV 应带 UTF-8 BOM")
	}
	header := lines[0]
	for _, col := range []string{"id", "defense_id", "defense_name", "challenger_id", "challenger_nickname", "won", "loot", "created_at"} {
		if !strings.Contains(header, col) {
			t.Errorf("表头缺少列 %s：%s", col, header)
		}
	}
	row := lines[1]
	if !strings.Contains(row, "铁壁") {
		t.Errorf("数据行应含防线名：%s", row)
	}
	if !strings.Contains(row, "coin:100") {
		t.Errorf("数据行应含 loot 可读文本 coin:100：%s", row)
	}
}

func TestAdminExportDefenseChallengesFiltered(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	owner, _ := newGuestUser(t, ts, "dcexp_f_owner")
	challengerA, _ := newGuestUser(t, ts, "dcexp_f_a")
	challengerB, _ := newGuestUser(t, ts, "dcexp_f_b")
	defID := seedDefense(t, ts, owner, "铁壁")
	seedChallenge(t, ts, defID, challengerA, true, time.Now().Add(-2*time.Hour))
	seedChallenge(t, ts, defID, challengerB, false, time.Now().Add(-1*time.Hour))

	// 按挑战者过滤 → 1 行数据
	data, err := ts.AdminExportDefenseChallenges(ctx, 0, challengerA, nil, nil)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 2 {
		t.Errorf("按挑战者过滤应 2 行（1 表头 + 1 数据），实际 %d", len(lines))
	}

	// 按日期过滤（-1.5h 起）→ 1 行数据
	from := time.Now().Add(-90 * time.Minute)
	data, err = ts.AdminExportDefenseChallenges(ctx, 0, 0, &from, nil)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	lines = strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 2 {
		t.Errorf("日期过滤应 2 行，实际 %d", len(lines))
	}
}

func TestAdminExportDefenseChallengesBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminExportDefenseChallenges(ctx, 0, 0, nil, nil); err == nil {
		t.Fatal("连接池已关闭时必须报错")
	}
}