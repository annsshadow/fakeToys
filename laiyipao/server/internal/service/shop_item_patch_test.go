package service

import (
	"context"
	"encoding/json"
	"errors"
	"testing"
)

// 第 113 轮：商城 patch **整单拒绝 + price/payload 形状校验**。
//
// 修前的两个缺陷：
//
//  1. 部分生效 + 成功回执（第 107 轮同族）：合法键写进去、
//     非法键被静默丢弃，整单回 200。
//
//  2. price/payload 只判「能不能 json.Marshal」：
//     标量 `100` / 字符串 / 数组 都能进 jsonb，
//     而玩家 Buy 读它们是 `Unmarshal(&map[string]int)`——
//     一旦写进标量，该商品购买链路永久 500。
func TestAdminShopItemPatchIsAllOrNothing(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	// 共享库：先把 item 1 整行存下来，测试结束时原样恢复，
	// 不污染其它用例（同 TestAdminUpdateShopItem 的约定）。
	var (
		origName    string
		origPrice   []byte
		origPayload []byte
		origLimit   int
		origSort    int
		origEnabled bool
	)
	if err := ts.pool.QueryRow(ctx,
		`SELECT name, price, payload, limit_per_day, sort_order, enabled
		   FROM shop_items WHERE id = 1`).
		Scan(&origName, &origPrice, &origPayload, &origLimit, &origSort, &origEnabled); err != nil {
		t.Skipf("无商城种子（item 1）：%v", err)
	}
	t.Cleanup(func() {
		_, _ = ts.pool.Exec(ctx,
			`UPDATE shop_items SET name=$1, price=$2, payload=$3, limit_per_day=$4, sort_order=$5, enabled=$6 WHERE id=1`,
			origName, origPrice, origPayload, origLimit, origSort, origEnabled)
	})

	// 把基线定住
	nameBefore := func() string {
		var s string
		_ = ts.pool.QueryRow(ctx, `SELECT name FROM shop_items WHERE id = 1`).Scan(&s)
		return s
	}
	base := nameBefore()

	cases := []struct {
		name  string
		patch map[string]any
	}{
		{"合法 name + 标量 price", map[string]any{"name": "P113改名", "price": 100.0}},
		{"合法 name + 数组 price", map[string]any{"name": "P113改名", "price": []any{1.0, 2.0}}},
		{"合法 name + 未知货币 key", map[string]any{"name": "P113改名", "price": map[string]any{"cino": 1.0}}},
		{"合法 name + 负价格", map[string]any{"name": "P113改名", "price": map[string]any{"coin": -1.0}}},
		{"合法 name + 负限购", map[string]any{"name": "P113改名", "limit_per_day": -5.0}},
		{"合法 name + 未知键 limit", map[string]any{"name": "P113改名", "limit": 3.0}},
		{"合法 name + enabled 类型错", map[string]any{"name": "P113改名", "enabled": "yes"}},
		{"合法 payload + 标量 payload", map[string]any{"payload": map[string]any{"coin": 1.0}, "price": "不是对象"}},
	}
	for _, c := range cases {
		out, err := ts.AdminUpdateShopItem(ctx, 1, c.patch)
		if !errors.Is(err, ErrBadInput) {
			t.Errorf("%s：应 ErrBadInput，实际 %v", c.name, err)
		}
		if out != nil {
			t.Errorf("%s：整单拒绝时不应有返回值，实际 %v", c.name, out)
		}
		// 关键：零副作用 —— 任何一个字段都不能被写进去
		if now := nameBefore(); now != base {
			t.Errorf("%s：name 被改成了 %q（应为 %q）—— 整单拒绝必须是零副作用", c.name, now, base)
		}
	}

	// 全部合法 → 成功，且 price/payload 真的写成了 map
	out, err := ts.AdminUpdateShopItem(ctx, 1, map[string]any{
		"name": "P113全合法", "price": map[string]any{"gem": 9.0},
		"payload": map[string]any{"coin": 1.0}, "limit_per_day": 4.0, "enabled": true,
	})
	if err != nil {
		t.Fatalf("全部合法应成功，实际 %v", err)
	}
	if out == nil {
		t.Fatal("成功时应有返回")
	}
	// （整行恢复由测试顶部的 t.Cleanup 统一负责，这里不再单独清 name）
	var priceJSON, payloadJSON []byte
	if err := ts.pool.QueryRow(ctx,
		`SELECT price, payload FROM shop_items WHERE id = 1`).Scan(&priceJSON, &payloadJSON); err != nil {
		t.Fatalf("读回 price/payload 失败：%v", err)
	}
	// 玩家 Buy 的读取方式：必须是 map[string]int 才不 500
	var p, pl map[string]int
	if err := json.Unmarshal(priceJSON, &p); err != nil || p["gem"] != 9 {
		t.Errorf("price 落库后不是 {gem:9} 的对象：raw=%s err=%v", priceJSON, err)
	}
	if err := json.Unmarshal(payloadJSON, &pl); err != nil || pl["coin"] != 1 {
		t.Errorf("payload 落库后不是 {coin:1} 的对象：raw=%s err=%v", payloadJSON, err)
	}
}
