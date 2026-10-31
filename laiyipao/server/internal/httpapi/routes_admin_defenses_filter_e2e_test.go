package httpapi

// 后台防线列表按防线名搜索的端到端测试（2026-10-31 新一轮第 19 轮）。

import (
	"context"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminDefensesKeywordFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "def_f")
	uidA, _ := e.newPlayer(t, "def_f_a")
	uidB, _ := e.newPlayer(t, "def_f_b")

	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, '铁壁防线', 100, '{}'::jsonb, 's1'), ($2, '尖刺陷阱', 200, '{}'::jsonb, 's2')`,
		uidA, uidB); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}

	// 无筛选 → total=2
	status, body := e.get(t, "/api/v1/admin/defenses?limit=50", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("防线列表应 200，实际 %d %v", status, body)
	}
	totalAll, ok := body["total"].(float64)
	if !ok || totalAll != 2 {
		t.Fatalf("无筛选应 2 条，实际 %v", body["total"])
	}

	// keyword=铁壁 → 1 条
	status, body = e.get(t, "/api/v1/admin/defenses?limit=50&keyword=铁壁", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("keyword 筛选应 200，实际 %d %v", status, body)
	}
	totalKw, ok := body["total"].(float64)
	if !ok || totalKw != 1 {
		t.Errorf("搜「铁壁」应 1 条，实际 %v", body["total"])
	}

	// owner_id 过滤 → 1 条
	status, body = e.get(t, "/api/v1/admin/defenses?limit=50&owner_id="+fmtInt(uidA), adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("owner_id 过滤应 200，实际 %d %v", status, body)
	}
	totalOwner, ok := body["total"].(float64)
	if !ok || totalOwner != 1 {
		t.Errorf("owner_id=%d 应 1 条，实际 %v", uidA, body["total"])
	}
}