package httpapi

// 后台兑换码使用记录 CSV 导出的端到端测试（2026-10-31 新一轮第 34 轮）。

import (
	"context"
	"io"
	"net/http"
	"strings"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminExportRedeemUsages(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "ruexp")
	uid, _ := e.newPlayer(t, "ruexp_p")
	var codeID int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT id FROM redeem_codes LIMIT 1`).Scan(&codeID); err != nil {
		t.Fatalf("取兑换码失败：%v", err)
	}
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO redeem_usages (code_id, user_id) VALUES ($1, $2)`, codeID, uid); err != nil {
		t.Fatalf("插入使用记录失败：%v", err)
	}

	req, _ := http.NewRequest("GET", "/api/v1/admin/redeem-usages/export", nil)
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
	if !strings.Contains(lines[1], fmtInt(uid)) || !strings.Contains(lines[1], "coin:") {
		t.Errorf("数据行内容不完整：%s", lines[1])
	}
}