package httpapi

// 玩家删除自己防线的端到端测试（2026-10-31 实质优化第 32 轮）。
//
// 覆盖：
//   - 未认证 → 401；
//   - 删除成功 → 200 + 库中防线消失；
//   - 未设置防线 → 404。

import (
	"context"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EDeleteMyDefenseRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("DELETE", "/api/v1/defenses", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EDeleteMyDefenseSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, tok := e.newPlayer(t, "mydel")
	var defID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, '我的防线', 100, '{}'::jsonb, 'snap')
		 RETURNING id`, uid).Scan(&defID); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}

	req, _ := http.NewRequest("DELETE", "/api/v1/defenses", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("删除应 200，实际 %d", resp.StatusCode)
	}

	var n int64
	if err := e.pool.QueryRow(context.Background(),
		`SELECT COUNT(*) FROM defenses WHERE id = $1`, defID).Scan(&n); err != nil {
		t.Fatalf("查询防线失败：%v", err)
	}
	if n != 0 {
		t.Error("删除后防线仍存在")
	}
}

func TestE2EDeleteMyDefenseNotFound(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, tok := e.newPlayer(t, "mydel_none")
	req, _ := http.NewRequest("DELETE", "/api/v1/defenses", nil)
	req.Header.Set("Authorization", "Bearer "+tok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusNotFound {
		t.Errorf("未设置防线应 404，实际 %d", resp.StatusCode)
	}
}
