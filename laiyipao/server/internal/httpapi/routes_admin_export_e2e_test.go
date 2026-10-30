package httpapi

// 后台用户 CSV 导出的端到端测试（2026-10-30 实质优化第 12 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - readonly 角色**可**导出（读操作，不挂 requireWritable）；
//   - 响应是带 BOM 的 CSV 且包含用户数据。

import (
	"bytes"
	"encoding/csv"
	"io"
	"net/http"
	"strconv"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminExportUsersRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/admin/users/export", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminExportUsersReadonlyAllowed(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	e.newPlayer(t, "exportro")
	_, roTok := e.newAdminRole(t, "exportro", "readonly")

	req, _ := http.NewRequest("GET", "/api/v1/admin/users/export", nil)
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

func TestE2EAdminExportUsersContent(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPlayer(t, "exportcsv")
	_, adminTok := e.newAdmin(t, "exportcsv")

	req, _ := http.NewRequest("GET", "/api/v1/admin/users/export", nil)
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

	// BOM
	if !bytes.HasPrefix(raw, []byte("\xEF\xBB\xBF")) {
		t.Error("CSV 必须以 UTF-8 BOM 开头")
	}
	// 解析并断言包含新玩家
	records, err := csv.NewReader(bytes.NewReader(bytes.TrimPrefix(raw, []byte("\xEF\xBB\xBF")))).ReadAll()
	if err != nil {
		t.Fatalf("解析 CSV 失败：%v", err)
	}
	if len(records) < 2 {
		t.Fatalf("应有表头 + 数据，实际 %d 行", len(records))
	}
	found := false
	for _, rec := range records[1:] {
		if rec[0] == strconv.FormatInt(uid, 10) {
			found = true
		}
	}
	if !found {
		t.Errorf("CSV 应包含新玩家 id %d，实际 %v", uid, records)
	}
}
