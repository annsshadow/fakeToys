package httpapi

// 后台兑换码使用记录按码/用户过滤的端到端测试（2026-10-31 新一轮第 23 轮）。

import (
	"context"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminRedeemUsagesFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "ru_f")
	uidA, _ := e.newPlayer(t, "ru_f_a")
	uidB, _ := e.newPlayer(t, "ru_f_b")

	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO redeem_codes (id, code, reward, max_uses)
		 VALUES (9201, 'LYP-E2E-A', '{"coin":1}'::jsonb, 100),
		        (9202, 'LYP-E2E-B', '{"coin":1}'::jsonb, 100),
		        (9203, 'LYP-E2E-C', '{"coin":1}'::jsonb, 100)`); err != nil {
		t.Fatalf("插入兑换码失败：%v", err)
	}
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO redeem_usages (code_id, user_id) VALUES (9201, $1), (9202, $1), (9203, $2)`,
		uidA, uidB); err != nil {
		t.Fatalf("插入使用记录失败：%v", err)
	}

	// 无筛选 → total=3
	status, body := e.get(t, "/api/v1/admin/redeem-usages?limit=50", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("列表应 200，实际 %d %v", status, body)
	}
	totalAll, ok := body["total"].(float64)
	if !ok || totalAll != 3 {
		t.Fatalf("无筛选应 3 条，实际 %v", body["total"])
	}

	// 按码搜索 → 1 条
	status, body = e.get(t, "/api/v1/admin/redeem-usages?limit=50&code_keyword=E2E-B", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("码搜索应 200，实际 %d %v", status, body)
	}
	totalKw, ok := body["total"].(float64)
	if !ok || totalKw != 1 {
		t.Errorf("搜 E2E-B 应 1 条，实际 %v", body["total"])
	}

	// 按用户过滤 → 1 条
	status, body = e.get(t, "/api/v1/admin/redeem-usages?limit=50&user_id="+fmtInt(uidB), adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("用户过滤应 200，实际 %d %v", status, body)
	}
	totalUser, ok := body["total"].(float64)
	if !ok || totalUser != 1 {
		t.Errorf("uidB 应 1 条，实际 %v", body["total"])
	}
}