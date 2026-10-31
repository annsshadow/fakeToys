package service

// 后台订单 CSV 导出的服务层测试（2026-11-01 新一轮第 40 轮）。
//
// 关键行为锚点：
//   - 输出是 UTF-8 BOM + CSV（Excel 中文兼容）；
//   - 表头 10 列齐全（id/user_id/nickname/order_no/sku/amount_cents/
//     status/provider/created_at/delivered_at）；
//   - 数据行与列表同源：昵称/订单号/SKU/金额（分）/状态/时间如实导出；
//   - NULL 的 delivered_at 导出为空串；
//   - 状态/日期筛选与列表同口径；
//   - 库故障必须报错而不是静默返回空 CSV。

import (
	"bytes"
	"context"
	"encoding/csv"
	"fmt"
	"testing"
	"time"
)

func TestAdminExportOrdersCSV(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "adm_ord_exp")
	orderNo := seedOrderFor(t, ts, uid, "paid")

	data, err := ts.AdminExportOrders(ctx, "", nil, nil)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	if !bytes.HasPrefix(data, []byte("\xEF\xBB\xBF")) {
		t.Error("CSV 必须以 UTF-8 BOM 开头")
	}

	records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF")))).ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}
	if len(records) < 2 {
		t.Fatalf("应有表头 + 至少 1 行数据，实际 %d 行", len(records))
	}

	wantHeader := []string{"id", "user_id", "nickname", "order_no", "sku", "amount_cents",
		"status", "provider", "created_at", "delivered_at"}
	if len(records[0]) != len(wantHeader) {
		t.Fatalf("表头列数 = %d，期望 %d：%v", len(records[0]), len(wantHeader), records[0])
	}
	for i, h := range wantHeader {
		if records[0][i] != h {
			t.Errorf("表头第 %d 列 = %q，期望 %q", i, records[0][i], h)
		}
	}

	var found bool
	for _, rec := range records[1:] {
		if rec[3] != orderNo {
			continue
		}
		found = true
		if rec[2] == "" {
			t.Error("导出应包含玩家昵称")
		}
		if rec[4] != "gem_pack" || rec[5] != "600" {
			t.Errorf("SKU/金额（分）不正确：%v / %v", rec[4], rec[5])
		}
		if rec[6] != "paid" {
			t.Errorf("状态 = %q，期望 paid", rec[6])
		}
		if rec[8] == "" {
			t.Error("created_at 不应为空")
		}
		if rec[9] != "" {
			t.Errorf("未发货订单的 delivered_at 应为空串，实际 %q", rec[9])
		}
	}
	if !found {
		t.Error("导出应包含测试订单")
	}
}

func TestAdminExportOrdersStatusFilter(t *testing.T) {
	ts := openTestService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "adm_ord_exp_f")
	paidNo := seedOrderFor(t, ts, uid, "paid")
	pendingNo := seedOrderFor(t, ts, uid, "pending")

	data, err := ts.AdminExportOrders(ctx, "pending", nil, nil)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF")))).ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}

	var sawPaid, sawPending bool
	for _, rec := range records[1:] {
		switch rec[3] {
		case paidNo:
			sawPaid = true
		case pendingNo:
			sawPending = true
		}
	}
	if sawPaid {
		t.Error("status=pending 筛选下不应导出 paid 订单")
	}
	if !sawPending {
		t.Error("status=pending 筛选下应导出 pending 订单")
	}
}

func TestAdminExportOrdersDateFilter(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()

	uid, _ := newGuestUser(t, ts, "adm_ord_exp_d")
	now := time.Now()
	// 造一条昨天的、一条今天的
	for i, d := range []time.Time{now.Add(-24 * time.Hour), now} {
		if _, err := ts.db.Pool.Exec(ctx,
			`INSERT INTO orders (user_id, order_no, sku, amount, status, provider, created_at)
			 VALUES ($1, $2, 'gem_pack', 600, 'paid', 'mock', $3)`,
			uid, fmt.Sprintf("ADM_ORD_D_%d_%d", i, time.Now().UnixNano()), d); err != nil {
			t.Fatalf("插入订单失败：%v", err)
		}
	}

	from := now.Add(-6 * time.Hour)
	data, err := ts.AdminExportOrders(ctx, "", &from, nil)
	if err != nil {
		t.Fatalf("导出失败：%v", err)
	}
	records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(data, []byte("\xEF\xBB\xBF")))).ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}
	// date_from = 6 小时前：只应导出今天的订单
	for _, rec := range records[1:] {
		created, err := time.Parse(time.RFC3339, rec[8])
		if err != nil {
			t.Fatalf("解析 created_at 失败：%v", err)
		}
		if created.Before(from) {
			t.Errorf("date_from 筛选下不应导出更早的订单：%v", created)
		}
	}
	if len(records) < 2 {
		t.Error("date_from 筛选下应至少导出 1 行")
	}
}

func TestAdminExportOrdersBrokenPool(t *testing.T) {
	ts := openBrokenService(t)
	ctx := context.Background()

	if _, err := ts.AdminExportOrders(ctx, "", nil, nil); err == nil {
		t.Fatal("连接池已关闭时导出必须报错，不能静默返回空 CSV")
	}
}