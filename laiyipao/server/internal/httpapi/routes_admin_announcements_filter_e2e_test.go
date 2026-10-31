package httpapi

// 后台公告列表按发布状态/日期筛选的端到端测试（2026-10-31 新一轮第 21 轮）。

import (
	"context"
	"net/url"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminAnnouncementsFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "ann_f")
	now := time.Now()

	// 三条公告：草稿 A、已发布 B（-2h）、已发布 C（-1h）
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO announcements (title, body, published, published_at, created_at)
		 VALUES ('草稿A', 'b', FALSE, NULL, $1),
		        ('已发布B', 'b', TRUE, $2, $2),
		        ('已发布C', 'b', TRUE, $3, $3)`,
		now.Add(-3*time.Hour), now.Add(-2*time.Hour), now.Add(-1*time.Hour)); err != nil {
		t.Fatalf("插入公告失败：%v", err)
	}

	// 无筛选 → total=3
	status, body := e.get(t, "/api/v1/admin/announcements?limit=50", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("列表应 200，实际 %d %v", status, body)
	}
	totalAll, ok := body["total"].(float64)
	if !ok || totalAll != 3 {
		t.Fatalf("无筛选应 3 条，实际 %v", body["total"])
	}

	// 只看已发布 → 2 条
	status, body = e.get(t, "/api/v1/admin/announcements?limit=50&published=1", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("已发布筛选应 200，实际 %d %v", status, body)
	}
	totalPub, ok := body["total"].(float64)
	if !ok || totalPub != 2 {
		t.Errorf("已发布应 2 条，实际 %v", body["total"])
	}

	// 只看草稿 → 1 条
	status, body = e.get(t, "/api/v1/admin/announcements?limit=50&published=0", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("草稿筛选应 200，实际 %d %v", status, body)
	}
	totalDraft, ok := body["total"].(float64)
	if !ok || totalDraft != 1 {
		t.Errorf("草稿应 1 条，实际 %v", body["total"])
	}

	// 日期筛选 date_from=-2.5h → 2 条（B/C）
	from := now.Add(-150 * time.Minute).Format(time.RFC3339)
	status, body = e.get(t, "/api/v1/admin/announcements?limit=50&date_from="+url.QueryEscape(from), adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("日期筛选应 200，实际 %d %v", status, body)
	}
	totalDate, ok := body["total"].(float64)
	if !ok || totalDate != 2 {
		t.Errorf("date_from=-2.5h 应 2 条，实际 %v", body["total"])
	}

	// 非法日期 → 400
	status, body = e.get(t, "/api/v1/admin/announcements?date_from=not-a-date", adminTok)
	if status != fiber.StatusBadRequest {
		t.Errorf("非法 date_from 应 400，实际 %d %v", status, body)
	}
}