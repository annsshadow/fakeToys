package httpapi

// 后台防线挑战记录 CSV 导出的端到端测试（2026-10-31 新一轮第 32 轮）。

import (
	"context"
	"io"
	"net/http"
	"strings"
	"testing"
	"time"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminExportDefenseChallenges(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "dcexp")
	ownerID, _ := e.newPlayer(t, "dcexp_owner")
	challengerID, _ := e.newPlayer(t, "dcexp_ch")

	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, '铁壁', 100, '{}'::jsonb, 's1')`, ownerID); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO defense_challenges (defense_id, challenger_id, seed, won, duration_ms, hp_left_pct, loot, settled, created_at)
		 VALUES ((SELECT id FROM defenses WHERE owner_id = $1), $2, 1, TRUE, 1500, 90, '{"coin":100}'::jsonb, TRUE, $3)`,
		ownerID, challengerID, time.Now().Add(-time.Hour)); err != nil {
		t.Fatalf("插入挑战记录失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/admin/defense-challenges/export", nil)
	req.Header.Set("Authorization", "Bearer "+adminTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("导出请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("导出应 200，实际 %d", resp.StatusCode)
	}
	body, err := io.ReadAll(resp.Body)
	if err != nil {
		t.Fatalf("读取响应失败：%v", err)
	}
	text := string(body)
	if !strings.HasPrefix(text, "\xEF\xBB\xBF") {
		t.Error("CSV 应带 UTF-8 BOM")
	}
	lines := strings.Split(strings.TrimSpace(text), "\n")
	if len(lines) != 2 {
		t.Fatalf("应有 2 行（1 表头 + 1 数据），实际 %d", len(lines))
	}
	if !strings.Contains(lines[1], "铁壁") || !strings.Contains(lines[1], "coin:100") {
		t.Errorf("数据行内容不完整：%s", lines[1])
	}
}