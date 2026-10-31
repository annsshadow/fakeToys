package httpapi

// 后台战报列表按日期范围过滤的端到端测试（2026-10-31 新一轮第 14 轮）。

import (
	"context"
	"net/url"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminBattlesDateFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "bdate")
	uid, _ := e.newPlayer(t, "bdatep")
	now := time.Now()

	// 两条战报：3 小时前、1 小时前
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO battle_records (user_id, level_id, result, stars, replay_hash, created_at)
		 VALUES ($1, 1, 'win', 1, 'h1', $2), ($1, 2, 'win', 2, 'h2', $3)`,
		uid, now.Add(-3*time.Hour), now.Add(-1*time.Hour)); err != nil {
		t.Fatalf("插入战报失败：%v", err)
	}

	// 无日期过滤 → total=2
	status, body := e.get(t, "/api/v1/admin/battles", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("列表应 200，实际 %d %v", status, body)
	}
	totalAll, ok := body["total"].(float64)
	if !ok {
		t.Fatalf("响应缺少 total：%v", body)
	}
	if totalAll != 2 {
		t.Fatalf("无过滤应 2 条，实际 %v", totalAll)
	}

	// date_from = now-2h → 只包含 1 小时前的
	from := now.Add(-2 * time.Hour).Format(time.RFC3339)
	status, body = e.get(t, "/api/v1/admin/battles?date_from="+url.QueryEscape(from), adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("date_from 过滤应 200，实际 %d %v", status, body)
	}
	totalFiltered, ok := body["total"].(float64)
	if !ok {
		t.Fatalf("响应缺少 total：%v", body)
	}
	if totalFiltered != 1 {
		t.Errorf("date_from=-2h 应剩 1 条，实际 %v", totalFiltered)
	}

	// 非法日期 → 400（fail-loud，不是静默不过滤）
	status, body = e.get(t, "/api/v1/admin/battles?date_from=not-a-date", adminTok)
	if status != fiber.StatusBadRequest {
		t.Errorf("非法 date_from 应 400，实际 %d %v", status, body)
	}
}