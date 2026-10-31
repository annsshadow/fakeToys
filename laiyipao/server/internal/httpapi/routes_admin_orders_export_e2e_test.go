package httpapi

// 后台订单 CSV 导出的端到端测试（2026-11-01 新一轮第 40 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - readonly 角色**可**导出（读操作，不挂 requireWritable）；
//   - 响应是带 BOM 的 CSV，且包含订单数据（订单号/玩家/状态/金额）。

import (
	"bytes"
	"context"
	"encoding/csv"
	"fmt"
	"io"
	"net/http"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminExportOrdersRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/admin/orders/export", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminExportOrdersReadonlyAllowed(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "exportorderro", "readonly")

	req, _ := http.NewRequest("GET", "/api/v1/admin/orders/export", nil)
	req.Header.Set("Authorization", "Bearer "+roTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("readonly 导出应 200，实际 %d", resp.StatusCode)
	}
	if ct := resp.Header.Get("Content-Type"); ct != "text/csv; charset=utf-8" {
		t.Errorf("Content-Type 应为 text/csv，实际 %q", ct)
	}
}

func TestE2EAdminExportOrdersContent(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPlayer(t, "export_orders")
	_, adminTok := e.newAdmin(t, "export_orders")
	orderNo := fmt.Sprintf("E2E_ORD_EXP_%d", time.Now().UnixNano())
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO orders (user_id, order_no, sku, amount, status, provider)
		 VALUES ($1, $2, 'gem_pack', 600, 'paid', 'mock')`, uid, orderNo); err != nil {
		t.Fatalf("插入订单失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/admin/orders/export", nil)
	req.Header.Set("Authorization", "Bearer "+adminTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	raw, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatalf("读响应失败：%v", err)
	}
	if !bytes.HasPrefix(raw, []byte("\xEF\xBB\xBF")) {
		t.Error("CSV 必须以 UTF-8 BOM 开头")
	}
	records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(raw, []byte("\xEF\xBB\xBF")))).ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}
	if len(records) < 2 {
		t.Fatalf("应有表头 + 数据，实际 %d 行", len(records))
	}
	found := false
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
	}
	if !found {
		t.Errorf("CSV 应包含测试订单 %q，实际 %v", orderNo, records)
	}
}