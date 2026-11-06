// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

// 后台兑换码 CSV 导出的端到端测试（2026-10-31 实质优化第 37 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - readonly 可导出（读操作）；
//   - 响应带 BOM 且包含兑换码数据。

import (
	"bytes"
	"context"
	"encoding/csv"
	"fmt"
	"io"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminExportRedeemCodesRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := testReq("GET", "/api/v1/admin/redeem-codes/export", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminExportRedeemCodesReadonlyAllowed(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "redeemro", "readonly")
	req, _ := testReq("GET", "/api/v1/admin/redeem-codes/export", nil)
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

func TestE2EAdminExportRedeemCodesContent(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "redeemexp")
	code := fmt.Sprintf("E2E_REDEEM_%d", time.Now().UnixNano())
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO redeem_codes (code, reward, max_uses)
		 VALUES ($1, '{"coin":100}'::jsonb, 5)`, code); err != nil {
		t.Fatalf("插入兑换码失败：%v", err)
	}

	req, _ := testReq("GET", "/api/v1/admin/redeem-codes/export", nil)
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
		if rec[1] == code {
			found = true
			if rec[2] != "coin:100" {
				t.Errorf("reward 文本应为 coin:100，实际 %q", rec[2])
			}
		}
	}
	if !found {
		t.Errorf("CSV 应包含测试兑换码 %q", code)
	}
}

// R30：导出必须与列表同口径（keyword / expired），否则运营筛完再导出拿到的是全量。
func TestE2EAdminExportRedeemCodesFilters(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	_, adminTok := e.newAdmin(t, "redeemexpf")

	tag := fmt.Sprintf("E2E_FILTER_%d", time.Now().UnixNano())
	past := time.Now().Add(-time.Hour)
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO redeem_codes (code, reward, max_uses) VALUES ($1, '{"coin":1}'::jsonb, 5)`,
		tag+"_live"); err != nil {
		t.Fatalf("插入未过期码失败：%v", err)
	}
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO redeem_codes (code, reward, max_uses, expires_at)
		 VALUES ($1, '{"coin":1}'::jsonb, 5, $2)`, tag+"_dead", past); err != nil {
		t.Fatalf("插入已过期码失败：%v", err)
	}

	for _, q := range []string{"keyword=" + tag, "expired=1", "expired=0"} {
		listURL := "/api/v1/admin/redeem-codes?limit=50&" + q
		status, body := e.get(t, listURL, adminTok)
		if status != 200 {
			t.Fatalf("%s 应 200，实际 %d", listURL, status)
		}
		total, _ := body["total"].(float64)
		status, raw := e.getRaw(t, "/api/v1/admin/redeem-codes/export?"+q, adminTok)
		if status != 200 {
			t.Fatalf("导出 %s 应 200，实际 %d", q, status)
		}
		records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(raw, []byte("\xEF\xBB\xBF")))).ReadAll()
		if err != nil {
			t.Fatalf("解析 CSV 失败：%v", err)
		}
		if got := float64(len(records) - 1); got != total {
			t.Fatalf("%s 导出 %.0f 行，列表 total=%.0f（口径不一致）", q, got, total)
		}
	}
}
