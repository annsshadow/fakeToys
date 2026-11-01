package httpapi

// 后台按 user_id 搜索的端到端测试（2026-10-31 实质优化第 31 轮）。
//
// 覆盖：
//   - admin 用 `keyword=<user_id>` 能精确搜到该玩家；
//   - 未认证 401。

import (
	"encoding/json"
	"net/http"
	"strconv"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EAdminUsersSearchByID(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	uid, _ := e.newPlayer(t, "idsearch")
	_, adminTok := e.newAdmin(t, "idsearch")

	req, _ := http.NewRequest("GET", "/api/v1/admin/users?keyword="+strconv.FormatInt(uid, 10), nil)
	req.Header.Set("Authorization", "Bearer "+adminTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	var body struct {
		Items []map[string]any `json:"items"`
		Total int              `json:"total"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&body); err != nil {
		t.Fatalf("解析响应失败：%v", err)
	}
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("应 200，实际 %d", resp.StatusCode)
	}
	if body.Total != 1 || len(body.Items) != 1 {
		t.Fatalf("按 id 搜索应返回 1 条，实际 total=%d items=%d", body.Total, len(body.Items))
	}
	item := body.Items[0]
	if item["id"] != float64(uid) {
		t.Errorf("返回的 id 应为 %d，实际 %v", uid, item["id"])
	}
}

func TestE2EAdminUsersSearchByIDRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("GET", "/api/v1/admin/users?keyword=1", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}
