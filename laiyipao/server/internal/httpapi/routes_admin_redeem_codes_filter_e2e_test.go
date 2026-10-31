package httpapi

// 后台兑换码列表按码搜索 / 启用状态 / 过期筛选的端到端测试
// （2026-10-31 新一轮第 20 轮）。

import (
	"context"
	"net/url"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminRedeemCodesFilter(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "code_f")
	now := time.Now()
	expiredAt := now.Add(-time.Hour)

	// 三个码：可用的 A、停用的 B、已过期的 C
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO redeem_codes (id, code, reward, max_uses, used_count, enabled, expires_at)
		 VALUES (9001, 'LYP-ACTIVE-01', '{"coin":100}'::jsonb, 1, 0, TRUE, NULL),
		        (9002, 'LYP-DISABLED-02', '{"coin":100}'::jsonb, 1, 0, FALSE, NULL),
		        (9003, 'LYP-EXPIRED-03', '{"coin":100}'::jsonb, 1, 0, TRUE, $1)`,
		expiredAt); err != nil {
		t.Fatalf("插入兑换码失败：%v", err)
	}

	// 按码搜索 → 1 条
	status, body := e.get(t, "/api/v1/admin/redeem-codes?limit=50&keyword=ACTIVE", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("码搜索应 200，实际 %d %v", status, body)
	}
	totalKw, ok := body["total"].(float64)
	if !ok || totalKw != 1 {
		t.Errorf("搜 ACTIVE 应 1 条，实际 %v", body["total"])
	}

	// 只看停用 → 1 条
	status, body = e.get(t, "/api/v1/admin/redeem-codes?limit=50&enabled=0", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("停用筛选应 200，实际 %d %v", status, body)
	}
	totalDisabled, ok := body["total"].(float64)
	if !ok || totalDisabled != 1 {
		t.Errorf("停用应 1 条，实际 %v", body["total"])
	}

	// 只看过期 → 1 条
	status, body = e.get(t, "/api/v1/admin/redeem-codes?limit=50&expired=1", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("过期筛选应 200，实际 %d %v", status, body)
	}
	totalExpired, ok := body["total"].(float64)
	if !ok || totalExpired != 1 {
		t.Errorf("过期应 1 条，实际 %v", body["total"])
	}

	// 启用 + 未过期 → 包含 seeder 码 + 9001（不含 9002/9003）
	status, body = e.get(t, "/api/v1/admin/redeem-codes?limit=50&enabled=1&expired=0", adminTok)
	if status != fiber.StatusOK {
		t.Fatalf("组合筛选应 200，实际 %d %v", status, body)
	}
	items, _ := body["items"].([]any)
	found := map[string]bool{}
	for _, it := range items {
		m := it.(map[string]any)
		found[m["code"].(string)] = true
	}
	if !found["LYP-ACTIVE-01"] {
		t.Errorf("启用+未过期应包含 LYP-ACTIVE-01，实际 %v", body["items"])
	}
	if found["LYP-DISABLED-02"] || found["LYP-EXPIRED-03"] {
		t.Errorf("启用+未过期不应包含停用/过期码，实际 %v", body["items"])
	}

	// 非法 keyword（超长）→ 400
	status, _ = e.get(t, "/api/v1/admin/redeem-codes?keyword="+url.QueryEscape(string(make([]byte, 200))), adminTok)
	if status != fiber.StatusBadRequest {
		t.Errorf("超长 keyword 应 400，实际 %d", status)
	}
}