package service

// 后台用户列表/导出按「需核查」筛选的测试（2026-11-01 新一轮第 60 轮）。
//
// 判据与前端列表的 suspicious 标记一致：战力（level_exp）> 5000
// 且最高关（max_stage）<= 1 —— 高战力却停在第一关是刷分嫌疑。

import (
	"context"
	"strings"
	"testing"
	"time"
)

// newUserWithProgress 造用户并直接设置 user_progress。
func newUserWithProgress(t *testing.T, ts *testService, nickname string, levelExp int64, maxStage int) int64 {
	t.Helper()
	ctx := context.Background()
	uid := newUserWithCreatedAt(t, ts, nickname, time.Now())
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO user_progress (user_id, max_stage, level_exp) VALUES ($1, $2, $3)`,
		uid, maxStage, levelExp); err != nil {
		t.Fatalf("写入 user_progress 失败：%v", err)
	}
	return uid
}

func TestAdminListUsersSuspiciousFilter(t *testing.T) {
	ts := openScratchService(t) // 一次性库，避免共享库数据污染
	ctx := context.Background()

	// 需核查：战力 6000 但最高关 1
	suspID := newUserWithProgress(t, ts, "susp_a", 6000, 1)
	// 正常：战力 100 最高关 10
	_ = newUserWithProgress(t, ts, "norm_b", 100, 10)
	// 高战力但已过第一关：不标
	_ = newUserWithProgress(t, ts, "high_c", 6000, 5)
	// 低战力但停在第一关：不标
	_ = newUserWithProgress(t, ts, "low_d", 1000, 1)

	// suspicious=true：只含需核查用户
	items, total, err := ts.AdminListUsers(ctx, "", 50, 0, nil, nil, nil, nil, nil, boolPtr(true))
	if err != nil {
		t.Fatalf("suspicious=true 查询失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].ID != suspID {
		t.Errorf("suspicious=true 应只含需核查用户，实际 total=%d items=%+v", total, items)
	}

	// suspicious=false：排除需核查用户（其余 3 个都在）
	items, total, err = ts.AdminListUsers(ctx, "", 50, 0, nil, nil, nil, nil, nil, boolPtr(false))
	if err != nil {
		t.Fatalf("suspicious=false 查询失败：%v", err)
	}
	if total != 3 {
		t.Errorf("suspicious=false 应排除需核查用户（剩 3 个），实际 total=%d", total)
	}
	for _, u := range items {
		if u.ID == suspID {
			t.Error("suspicious=false 不应包含需核查用户")
		}
	}

	// suspicious=nil：不过滤
	items, total, err = ts.AdminListUsers(ctx, "", 50, 0, nil, nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatalf("suspicious=nil 查询失败：%v", err)
	}
	if total != 4 {
		t.Errorf("suspicious=nil 应返回全部用户，实际 total=%d", total)
	}
}

func TestAdminExportUsersSuspiciousFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	_ = newUserWithProgress(t, ts, "exp_susp", 6000, 1)
	_ = newUserWithProgress(t, ts, "exp_norm", 100, 10)

	// 导出 suspicious=true：只含需核查用户（1 表头 + 1 数据）
	data, err := ts.AdminExportUsers(ctx, "", nil, nil, nil, nil, boolPtr(true))
	if err != nil {
		t.Fatalf("suspicious=true 导出失败：%v", err)
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 2 {
		t.Errorf("suspicious=true 导出行数应为 2（1 表头 + 1 数据），实际 %d", len(lines))
	}

	// 导出 suspicious=nil：全部（1 表头 + 2 数据）
	data, err = ts.AdminExportUsers(ctx, "", nil, nil, nil, nil, nil)
	if err != nil {
		t.Fatalf("suspicious=nil 导出失败：%v", err)
	}
	lines = strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 3 {
		t.Errorf("suspicious=nil 导出行数应为 3（1 表头 + 2 数据），实际 %d", len(lines))
	}
}