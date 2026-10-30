package httpapi

// 后台公告编辑/删除端点的端到端测试（2026-10-31 实质优化第 23 轮）。
//
// 覆盖：
//   - 未认证 → 401；readonly → 403；
//   - 更新成功 → 200 + 库中标题/发布状态更新；
//   - 删除成功 → 200 + 库中记录消失；
//   - 更新/删除不存在的公告 → 404。

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"strconv"
	"testing"

	"github.com/gofiber/fiber/v2"
)

// annPut 构造带 JSON body 的 PUT 请求（公告更新）。
func annPut(url, token string, payload map[string]any) *http.Request {
	raw, _ := json.Marshal(payload)
	req, _ := http.NewRequest("PUT", url, bytes.NewReader(raw))
	req.Header.Set("Authorization", "Bearer "+token)
	req.Header.Set("Content-Type", "application/json")
	return req
}

func TestE2EAdminUpdateAnnouncementRequiresAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	req, _ := http.NewRequest("PUT", "/api/v1/admin/announcements/1", nil)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusUnauthorized {
		t.Fatalf("未认证应 401，实际 %d", resp.StatusCode)
	}
}

func TestE2EAdminUpdateAnnouncementReadonlyForbidden(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, roTok := e.newAdminRole(t, "annro", "readonly")
	req, _ := http.NewRequest("PUT", "/api/v1/admin/announcements/1", nil)
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

func TestE2EAdminUpdateAnnouncementSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "annupdate")
	var id int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO announcements (title, body, published)
		 VALUES ('旧标题', '旧正文', FALSE)
		 RETURNING id`).Scan(&id); err != nil {
		t.Fatalf("插入公告失败：%v", err)
	}

	req := annPut("/api/v1/admin/announcements/"+strconv.FormatInt(id, 10), adminTok, map[string]any{
		"title": "新标题", "body": "新正文", "published": true,
	})
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("更新应 200，实际 %d", resp.StatusCode)
	}

	var title, body string
	var published bool
	if err := e.pool.QueryRow(context.Background(),
		`SELECT title, body, published FROM announcements WHERE id = $1`, id).
		Scan(&title, &body, &published); err != nil {
		t.Fatalf("查询公告失败：%v", err)
	}
	if title != "新标题" || body != "新正文" {
		t.Errorf("库中公告未更新：title=%q body=%q", title, body)
	}
	if !published {
		t.Error("发布状态应为 true")
	}
}

func TestE2EAdminDeleteAnnouncementSucceeds(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "anndel")
	var id int64
	if err := e.pool.QueryRow(context.Background(),
		`INSERT INTO announcements (title, body, published)
		 VALUES ('待删', '正文', FALSE)
		 RETURNING id`).Scan(&id); err != nil {
		t.Fatalf("插入公告失败：%v", err)
	}

	req, _ := http.NewRequest("DELETE", "/api/v1/admin/announcements/"+strconv.FormatInt(id, 10), nil)
	req.Header.Set("Authorization", "Bearer "+adminTok)
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("删除应 200，实际 %d", resp.StatusCode)
	}

	var n int
	if err := e.pool.QueryRow(context.Background(),
		`SELECT COUNT(*) FROM announcements WHERE id = $1`, id).Scan(&n); err != nil {
		t.Fatalf("查询公告失败：%v", err)
	}
	if n != 0 {
		t.Error("删除后公告仍存在")
	}
}

func TestE2EAdminUpdateDeleteNotFound(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)

	_, adminTok := e.newAdmin(t, "annnf")

	// 更新不存在 → 404
	req := annPut("/api/v1/admin/announcements/999999", adminTok, map[string]any{
		"title": "标题", "body": "正文", "published": false,
	})
	resp, err := e.app.Test(req, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusNotFound {
		t.Errorf("更新不存在的公告应 404，实际 %d", resp.StatusCode)
	}

	// 删除不存在 → 404
	req2, _ := http.NewRequest("DELETE", "/api/v1/admin/announcements/999999", nil)
	req2.Header.Set("Authorization", "Bearer "+adminTok)
	resp2, err := e.app.Test(req2, 10000)
	if err != nil {
		t.Fatalf("请求失败：%v", err)
	}
	defer resp2.Body.Close()
	if resp2.StatusCode != fiber.StatusNotFound {
		t.Errorf("删除不存在的公告应 404，实际 %d", resp2.StatusCode)
	}
}
