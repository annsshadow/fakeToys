package httpapi

// 后台删除玩家防线的端到端测试（2026-10-31 实质优化第 26 轮）。
//
// 覆盖：
//   - 未认证 → 401；readonly → 403；
//   - 删除成功 → 200 + 库中防线消失；
//   - 不存在的防线 → 404。

import (
	"context"
	"net/http"
	"strconv"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminDeleteDefenseRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("DELETE", "/api/v1/admin/defenses/1", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminDeleteDefenseReadonlyForbidden(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "defdelro", "readonly")
	req, _ := http.NewRequest("DELETE", "/api/v1/admin/defenses/1", nil)
	req.Header.Set("Authorization", "Bearer "+roTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusForbidden {
		t.Fatalf("readonly 应 403，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminDeleteDefenseSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	ownerID, _ := e.newPlayer(t, "defdel_owner")
	_, adminTok := e.newAdmin(t, "defdel")
	var defID int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO defenses (owner_id, name, power, snapshot, snapshot_hash)
		 VALUES ($1, '违规防线', 100, '{}'::jsonb, 'snap')
		 RETURNING id`, ownerID).Scan(&defID); err != nil {
		t.Fatalf("插入防线失败：%v", err)
	}

	req, _ := http.NewRequest("DELETE", "/api/v1/admin/defenses/"+strconv.FormatInt(defID, 10), nil)
	req.Header.Set("Authorization", "Bearer "+adminTok)
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

func TestE2EAdminDeleteDefenseNotFound(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "defdelnf")
	req, _ := http.NewRequest("DELETE", "/api/v1/admin/defenses/999999", nil)
	req.Header.Set("Authorization", "Bearer "+adminTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusNotFound {
		t.Errorf("删除不存在的防线应 404，实际 %d", resp.StatusCode)
	}
}
