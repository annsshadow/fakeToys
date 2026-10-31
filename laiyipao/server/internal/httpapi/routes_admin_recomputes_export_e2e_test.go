package httpapi

// 后台重算台账 CSV 导出的端到端测试（2026-11-01 新一轮第 45 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - readonly 角色**可**导出（读操作，不挂 requireWritable）；
//   - 响应是带 BOM 的 CSV 且包含重算数据（玩家/关卡/不符字段/模式）。

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

func TestE2EAdminExportRecomputesRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/admin/recomputes/export", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminExportRecomputesReadonlyAllowed(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "exportrecompro", "readonly")

	req, _ := http.NewRequest("GET", "/api/v1/admin/recomputes/export", nil)
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

func TestE2EAdminExportRecomputesContent(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPlayer(t, "export_recompute")
	_, adminTok := e.newAdmin(t, "export_recompute")
	levelID := 7
	now := time.Now()
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO settle_recomputes
		   (battle_id, user_id, level_id, reported_score, reported_kills,
		    reported_duration_ms, reported_hash, mismatched_fields, mode, rejected, engine_version, created_at)
		 VALUES ($1, $2, $3, 100, 10, 60000, 'hh', ARRAY['score'], 'shadow', FALSE, 'v1', $4)`,
		uid*100, uid, levelID, now); err != nil {
		t.Fatalf("插入重算记录失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/admin/recomputes/export", nil)
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
		if rec[4] != fmt.Sprint(levelID) {
			continue
		}
		found = true
		if rec[3] == "" {
			t.Error("导出应包含玩家昵称")
		}
		if rec[13] != "score" {
			t.Errorf("mismatched_fields 应为 score，实际 %q", rec[13])
		}
		if rec[14] != "shadow" {
			t.Errorf("mode 应为 shadow，实际 %q", rec[14])
		}
	}
	if !found {
		t.Errorf("CSV 应包含测试重算行，实际 %v", records)
	}
}