package httpapi

// 后台单用户钱包流水导出的端到端测试（2026-11-01 新一轮第 82 轮）。

import (
	"bytes"
	"context"
	"encoding/csv"
	"fmt"
	"io"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminExportUserFlowsRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/admin/users/1/flows/export", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminExportUserFlowsReadonlyAllowed(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "exportflowsro", "readonly")

	req, _ := http.NewRequest("GET", "/api/v1/admin/users/1/flows/export", nil)
	req.Header.Set("Authorization", "Bearer "+roTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("readonly 导出应 200，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminExportUserFlowsContent(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPlayer(t, "exportflows")
	_, adminTok := e.newAdmin(t, "exportflows")
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO wallet_flows (user_id, currency, delta, balance, reason)
		 VALUES ($1, 'coin', 1000, 1000, 'admin_grant')`, uid); err != nil {
		t.Fatalf("插入流水失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/admin/users/"+fmt.Sprint(uid)+"/flows/export", nil)
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
		t.Fatalf("应有表头 + 流水数据，实际 %d 行", len(records))
	}
	found := false
	for _, rec := range records[1:] {
		if rec[1] == "coin" && rec[2] == "1000" && rec[4] == "admin_grant" {
			found = true
		}
	}
	if !found {
		t.Errorf("CSV 应包含 coin +1000 admin_grant 流水，实际 %v", records)
	}
}