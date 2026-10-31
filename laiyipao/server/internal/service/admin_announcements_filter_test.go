package service

// 后台公告列表按发布状态/创建日期过滤的测试（2026-10-31 新一轮第 21 轮）。

import (
	"context"
	"testing"
	"time"
)

func TestAdminListAnnouncementsFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	now := time.Now()
	seedAnnouncement(t, ts, "草稿A", false, now.Add(-3*time.Hour))
	seedAnnouncement(t, ts, "已发布B", true, now.Add(-2*time.Hour))
	seedAnnouncement(t, ts, "已发布C", true, now.Add(-1*time.Hour))

	// 只看已发布 → 2 条
	items, total, err := ts.AdminListAnnouncements(ctx, 50, 0, boolPtr(true), nil, nil)
	if err != nil {
		t.Fatalf("已发布筛选失败：%v", err)
	}
	if total != 2 || len(items) != 2 {
		t.Errorf("已发布应 2 条，实际 total=%d", total)
	}
	for _, it := range items {
		if !it.Published {
			t.Errorf("返回了未发布公告：%+v", it)
		}
	}

	// 只看草稿 → 1 条
	items, total, err = ts.AdminListAnnouncements(ctx, 50, 0, boolPtr(false), nil, nil)
	if err != nil {
		t.Fatalf("草稿筛选失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].Title != "草稿A" {
		t.Errorf("草稿应 1 条，实际 total=%d", total)
	}

	// 日期范围 [now-2.5h, now-1.5h) → 只含已发布B
	from := now.Add(-150 * time.Minute)
	to := now.Add(-90 * time.Minute)
	items, total, err = ts.AdminListAnnouncements(ctx, 50, 0, nil, &from, &to)
	if err != nil {
		t.Fatalf("日期过滤失败：%v", err)
	}
	if total != 1 || len(items) != 1 || items[0].Title != "已发布B" {
		t.Errorf("双端日期应只含已发布B，实际 total=%d", total)
	}

	// 组合：已发布 + date_from=-2.5h → 2 条（已发布B 与 已发布C）
	items, total, err = ts.AdminListAnnouncements(ctx, 50, 0, boolPtr(true), &from, nil)
	if err != nil {
		t.Fatalf("组合过滤失败：%v", err)
	}
	if total != 2 {
		t.Errorf("已发布+date_from=-2.5h 应 2 条（B/C），实际 total=%d", total)
	}
}

func seedAnnouncement(t *testing.T, ts *testService, title string, published bool, at time.Time) {
	t.Helper()
	ctx := context.Background()
	var publishedAt *time.Time
	if published {
		publishedAt = &at
	}
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO announcements (title, body, published, published_at, created_at)
		 VALUES ($1, 'body', $2, $3, $4)`,
		title, published, publishedAt, at); err != nil {
		t.Fatalf("插入公告失败：%v", err)
	}
}