package httpapi

import (
	"context"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
)

// 第 113 轮：商城 patch 的**写路径**走整单拒绝 + price/payload 形状校验。
//
// 修前：
//   - 标量 price `100` 能进 jsonb → 玩家 Buy `Unmarshal(&map[string]int)` 必 500
//   - 未知键 `limit`（旧 UI）被静默吞掉 → 「没有可更新的字段」400，运营不知道键名错
//   - 合法键 + 非法键混合 → 部分生效 + 成功回执（第 107 轮同族）
//
// 判据：坏价格 400 且**零副作用**（price 列不变）；正确键 200 且真落库。
func TestAdminShopItemPatchShape(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "shoptgt113")

	// item 1 的 price 种子是 {"gem":10}，读 gem 键。
	// ⚠️ 必须先 ::int 再 COALESCE —— `COALESCE(text, 0)` 直接类型错，
	// 查询失败被 `_ =` 吞掉后 p 恒 0，会把「没写进去」误判成「没写进去」。
	priceNow := func() int64 {
		var p int64
		_ = e.pool.QueryRow(context.Background(),
			`SELECT COALESCE((price->>'gem')::int, 0) FROM shop_items WHERE id = 1`).Scan(&p)
		return p
	}
	base := priceNow()

	// 标量 price：必须 400 且 price 列不动
	if st, _ := e.put(t, "/api/v1/admin/shop/1", tok, map[string]any{"price": 100}); st != 400 {
		t.Errorf("标量 price 应 400，实际 %d（进 jsonb 会打坏玩家购买）", st)
	}
	if got := priceNow(); got != base {
		t.Errorf("被拒的 patch 改动了 price 列：%d → %d（零副作用）", base, got)
	}

	// 未知键 limit：400 且提示是白名单问题
	var st int
	var body map[string]any
	st, body = e.put(t, "/api/v1/admin/shop/1", tok, map[string]any{"limit": 3})
	if st != fiber.StatusBadRequest {
		t.Errorf("未知键 limit 应 400，实际 %d", st)
	} else {
		em, _ := body["error"].(map[string]any)
		msg, _ := em["message"].(string)
		if !strings.Contains(msg, "白名单") {
			t.Errorf("400 应提示键不在白名单（旧 UI 的 limit 键名要能被发现），实际 %q", msg)
		}
	}

	// 合法 map → 200，且真落库
	st, body = e.put(t, "/api/v1/admin/shop/1", tok, map[string]any{
		"price": map[string]any{"gem": base + 7},
	})
	if st != 200 {
		t.Fatalf("合法 price map 应 200，实际 %d %v", st, body)
	}
	if got := priceNow(); got != base+7 {
		t.Errorf("price.gem 应落库 %d，实际 %d", base+7, got)
	}
	// 复原
	_, _ = e.put(t, "/api/v1/admin/shop/1", tok, map[string]any{"price": map[string]any{"gem": base}})
}
