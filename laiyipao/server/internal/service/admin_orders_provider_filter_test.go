package service

// 后台订单列表/导出按渠道筛选的测试（2026-11-01 新一轮第 67 轮）。
//
// 订单有 provider 字段（mock/apple/wechat 等），运营想查「某渠道的订单」
// 此前只能翻页或导出后筛。新增可选 provider（空串不过滤）。

import (
	"context"
	"strings"
	"testing"
)

func TestAdminListOrdersProviderFilter(t *testing.T) {
	ts := openScratchService(t) // 一次性库，避免共享库数据污染
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "oprov")
	// seedOrderFor 固定 provider='mock'；再手动造一条 'apple'
	_ = seedOrderFor(t, ts, uid, "paid")
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO orders (user_id, order_no, sku, amount, status, provider)
		 VALUES ($1, 'ADM_ORD_APPLE', 'gem_pack', 600, 'paid', 'apple')`, uid); err != nil {
		t.Fatalf("插入 apple 订单失败：%v", err)
	}

	// provider=apple：只含 apple 订单
	items, total, err := ts.AdminListOrders(ctx, 50, 0, "", nil, nil, "apple")
	if err != nil {
		t.Fatalf("provider=apple 查询失败：%v", err)
	}
	if total != 1 || len(items) != 1 {
		t.Fatalf("provider=apple 应 1 条，实际 total=%d", total)
	}
	if items[0].Provider != "apple" {
		t.Errorf("订单渠道应为 apple，实际 %q", items[0].Provider)
	}

	// provider=mock：至少 1 条（seedOrderFor 那条）
	items, total, err = ts.AdminListOrders(ctx, 50, 0, "", nil, nil, "mock")
	if err != nil {
		t.Fatalf("provider=mock 查询失败：%v", err)
	}
	if total < 1 {
		t.Error("provider=mock 应至少 1 条")
	}
	for _, it := range items {
		if it.Provider != "mock" {
			t.Errorf("provider=mock 过滤下出现 %q 渠道订单", it.Provider)
		}
	}

	// provider 空串：不过滤，两条都在
	items, total, err = ts.AdminListOrders(ctx, 50, 0, "", nil, nil, "")
	if err != nil {
		t.Fatalf("provider 空串查询失败：%v", err)
	}
	if total != 2 {
		t.Errorf("provider 空串应返回全部订单，实际 total=%d", total)
	}
}

func TestAdminExportOrdersProviderFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "oprov_exp")
	_ = seedOrderFor(t, ts, uid, "paid")
	if _, err := ts.db.Pool.Exec(ctx,
		`INSERT INTO orders (user_id, order_no, sku, amount, status, provider)
		 VALUES ($1, 'ADM_ORD_APPLE_EXP', 'gem_pack', 600, 'paid', 'apple')`, uid); err != nil {
		t.Fatalf("插入 apple 订单失败：%v", err)
	}

	// provider=apple：导出 1 行数据
	data, err := ts.AdminExportOrders(ctx, "", nil, nil, "apple")
	if err != nil {
		t.Fatalf("provider=apple 导出失败：%v", err)
	}
	lines := strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 2 {
		t.Errorf("provider=apple 导出行数应为 2（1 表头 + 1 数据），实际 %d", len(lines))
	}

	// provider 空串：导出全部（2 数据）
	data, err = ts.AdminExportOrders(ctx, "", nil, nil, "")
	if err != nil {
		t.Fatalf("provider 空串导出失败：%v", err)
	}
	lines = strings.Split(strings.TrimSpace(string(data)), "\n")
	if len(lines) != 3 {
		t.Errorf("provider 空串导出行数应为 3（1 表头 + 2 数据），实际 %d", len(lines))
	}
}