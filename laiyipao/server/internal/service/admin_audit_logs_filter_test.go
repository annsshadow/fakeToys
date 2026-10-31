package service

// 后台审计日志按操作/管理员/日期范围过滤的测试（2026-10-31 新一轮第 18 轮）。
//
// 审计日志持续增长（每次后台操作都记），运营查「谁在什么时候改了
// 兑换码」此前只能翻页。新增可选 action / admin_id / date_from / date_to。

import (
	"context"
	"testing"
	"time"
)

func TestAdminListAuditLogsFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	// 三个管理员 + 三条不同 action 的审计
	adminA := newAdminUser(t, ts)
	adminB := newAdminUser(t, ts)
	now := time.Now()
	seedAuditLog(t, ts, adminA, "update_redeem", now.Add(-3*time.Hour))
	seedAuditLog(t, ts, adminA, "ban_user", now.Add(-2*time.Hour))
	seedAuditLog(t, ts, adminB, "update_redeem", now.Add(-1*time.Hour))

	// 按 action 过滤
	items, total, err := ts.AdminListAuditLogs(ctx, 50, 0, "update_redeem", 0, nil, nil)
	if err != nil {
		t.Fatalf("action 过滤失败：%v", err)
	}
	if total != 2 || len(items) != 2 {
		t.Errorf("update_redeem 应 2 条，实际 total=%d len=%d", total, len(items))
	}
	for _, it := range items {
		if it.Action != "update_redeem" {
			t.Errorf("返回了非 update_redeem 日志：%+v", it)
		}
	}

	// 按管理员过滤
	items, total, err = ts.AdminListAuditLogs(ctx, 50, 0, "", adminB, nil, nil)
	if err != nil {
		t.Fatalf("管理员过滤失败：%v", err)
	}
	if total != 1 || len(items) != 1 {
		t.Errorf("adminB 应 1 条，实际 total=%d", total)
	}

	// 按日期范围 [now-2.5h, now-1.5h) → 只含 ban_user（-2h）
	from := now.Add(-150 * time.Minute)
	to := now.Add(-90 * time.Minute)
	items, total, err = ts.AdminListAuditLogs(ctx, 50, 0, "", 0, &from, &to)
	if err != nil {
		t.Fatalf("日期过滤失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].Action != "ban_user" {
		t.Errorf("双端日期应只含 ban_user，实际 total=%d", total)
	}

	// 组合：action + 管理员
	items, total, err = ts.AdminListAuditLogs(ctx, 50, 0, "update_redeem", adminA, nil, nil)
	if err != nil {
		t.Fatalf("组合过滤失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].Action != "update_redeem" {
		t.Errorf("update_redeem+adminA 应 1 条，实际 total=%d", total)
	}
}

func seedAuditLog(t *testing.T, ts *testService, adminID int64, action string, at time.Time) {
	t.Helper()
	ctx := context.Background()
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO admin_audit_logs (admin_id, username, action, target, detail, created_at)
		 VALUES ($1, 'admin', $2, 't', '{}'::jsonb, $3)`, adminID, action, at); err != nil {
		t.Fatalf("插入审计日志失败：%v", err)
	}
}