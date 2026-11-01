package httpapi

// 后台战报 CSV 导出的端到端测试（2026-10-30 实质优化第 13 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - readonly 角色**可**导出（读操作，不挂 requireWritable）；
//   - 响应是带 BOM 的 CSV 且包含战报数据。

import (
	"bytes"
	"context"
	"encoding/csv"
	"fmt"
	"io"
	"net/http"
	"strconv"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminExportBattlesRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/admin/battles/export", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminExportBattlesReadonlyAllowed(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "exportbattro", "readonly")

	req, _ := http.NewRequest("GET", "/api/v1/admin/battles/export", nil)
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

func TestE2EAdminExportBattlesContent(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPlayer(t, "exportbattcsv")
	_, adminTok := e.newAdmin(t, "exportbattcsv")

	// 直接插入一条战报
	var battleID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO battle_records (user_id, level_id, result, replay_hash)
		 VALUES ($1, 1, 'win', $2)
		 RETURNING id`, uid, fmt.Sprintf("e2e_hash_%d", time.Now().UnixNano())).Scan(&battleID); err != nil {
		t.Fatalf("插入战报失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/admin/battles/export", nil)
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
		if rec[0] == strconv.FormatInt(battleID, 10) {
			found = true
			if rec[1] != strconv.FormatInt(uid, 10) {
				t.Errorf("user_id 应为 %d，实际 %q", uid, rec[1])
			}
		}
	}
	if !found {
		t.Errorf("CSV 应包含战报 id %d，实际 %v", battleID, records)
	}
}
