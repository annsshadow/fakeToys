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
	"net/http"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminExportRedeemCodesRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/admin/redeem-codes/export", nil)
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
	req, _ := http.NewRequest("GET", "/api/v1/admin/redeem-codes/export", nil)
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

	req, _ := http.NewRequest("GET", "/api/v1/admin/redeem-codes/export", nil)
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
