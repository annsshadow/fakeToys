package httpapi

// 第 48 轮：L-3 阵型预设端点的端到端测试。
// 覆盖：未认证 401、保存当前构筑 → 列表 → 设当前 → 删除全流程、
// 他人预设隔离（404）。

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"testing"

	"github.com/gofiber/fiber/v2"
)

func TestE2EPresetsRequireAuth(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	for _, tc := range []struct {
		method, path string
	}{
		{"GET", "/api/v1/me/presets"},
		{"POST", "/api/v1/me/presets"},
		{"POST", "/api/v1/me/presets/1/current"},
		{"DELETE", "/api/v1/me/presets/1"},
	} {
		req, _ := http.NewRequest(tc.method, tc.path, nil)
		resp, err := e.app.Test(req, 10000)
		if err != nil {
			t.Fatalf("%s %s 请求失败：%v", tc.method, tc.path, err)
		}
		defer resp.Body.Close()
		if resp.StatusCode != fiber.StatusUnauthorized {
			t.Errorf("%s %s 未认证应 401，实际 %d", tc.method, tc.path, resp.StatusCode)
		}
	}
}

func TestE2EPresetLifecycle(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	_, tok := e.newPlayer(t, "presets")

	authed := func(method, path string, body any) (*http.Response, error) {
		var r *http.Request
		var err error
		if body == nil {
			r, err = http.NewRequest(method, path, nil)
		} else {
			b, _ := json.Marshal(body)
			r, err = http.NewRequest(method, path, bytes.NewReader(b))
		}
		if err != nil {
			return nil, err
		}
		r.Header.Set("Authorization", "Bearer "+tok)
		if body != nil {
			r.Header.Set("Content-Type", "application/json")
		}
		return e.app.Test(r, 10000)
	}

	// 保存当前构筑为预设（新号有默认技能/装备 → slots 非空）。
	resp, err := authed("POST", "/api/v1/me/presets", map[string]any{"name": "默认套"})
	if err != nil {
		t.Fatalf("保存预设失败：%v", err)
	}
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("保存预设应 200，实际 %d", resp.StatusCode)
	}
	var created struct {
		Preset struct {
			ID        int64  `json:"id"`
			Name      string `json:"name"`
			IsCurrent bool   `json:"is_current"`
			Slots     struct {
				Skills []int `json:"skills"`
			} `json:"slots"`
		} `json:"preset"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&created); err != nil {
		t.Fatalf("解析保存响应失败：%v", err)
	}
	if created.Preset.Name != "默认套" || !created.Preset.IsCurrent {
		t.Fatalf("首个预设应为 current：%+v", created.Preset)
	}
	if len(created.Preset.Slots.Skills) == 0 {
		t.Fatal("新号的预设应有出战技能")
	}
	presetID := created.Preset.ID

	// 再存一个（非 current）。
	resp, _ = authed("POST", "/api/v1/me/presets", map[string]any{"name": "备用套"})
	defer resp.Body.Close()
	var second struct {
		Preset struct {
			ID int64 `json:"id"`
		} `json:"preset"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&second); err != nil {
		t.Fatal(err)
	}

	// 列表：2 个，只有一个是 current。
	resp, _ = authed("GET", "/api/v1/me/presets", nil)
	defer resp.Body.Close()
	var list struct {
		Presets []struct {
			ID        int64  `json:"id"`
			Name      string `json:"name"`
			IsCurrent bool   `json:"is_current"`
		} `json:"presets"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&list); err != nil {
		t.Fatal(err)
	}
	if len(list.Presets) != 2 {
		t.Fatalf("应有 2 个预设，实际 %d", len(list.Presets))
	}
	currentCount := 0
	for _, p := range list.Presets {
		if p.IsCurrent {
			currentCount++
		}
	}
	if currentCount != 1 {
		t.Errorf("任意时刻只能 1 个 current，实际 %d", currentCount)
	}

	// 设备用套为当前。
	resp, _ = authed("POST", fmt.Sprintf("/api/v1/me/presets/%d/current", second.Preset.ID), nil)
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("设当前应 200，实际 %d", resp.StatusCode)
	}

	// 删除备用套。
	resp, _ = authed("DELETE", fmt.Sprintf("/api/v1/me/presets/%d", second.Preset.ID), nil)
	defer resp.Body.Close()
	if resp.StatusCode != fiber.StatusOK {
		t.Fatalf("删除应 200，实际 %d", resp.StatusCode)
	}

	// 列表只剩一个。
	resp, _ = authed("GET", "/api/v1/me/presets", nil)
	defer resp.Body.Close()
	var after struct {
		Presets []struct {
			ID int64 `json:"id"`
		} `json:"presets"`
	}
	if err := json.NewDecoder(resp.Body).Decode(&after); err != nil {
		t.Fatal(err)
	}
	if len(after.Presets) != 1 || after.Presets[0].ID != presetID {
		t.Fatalf("删除后应只剩第一个预设，实际 %+v", after.Presets)
	}
}

func TestE2EPresetForeignIsolation(t *testing.T) {
	e2eScratchDB(t)
	e := newE2E(t)
	owner, ownerTok := e.newPlayer(t, "preset-owner")
	_, attackerTok := e.newPlayer(t, "preset-attacker")

	req, _ := http.NewRequest("POST", "/api/v1/me/presets", nil)
	req.Header.Set("Authorization", "Bearer "+ownerTok)
	req.Header.Set("Content-Type", "application/json")
	// 空 body 直接 400（bad_json）——先建一个合法的
	// （上面测试已覆盖创建；这里直接造数据）。
	var presetID int64
	if err := e.pool.QueryRow(
		context.Background(),
		`INSERT INTO user_presets (user_id, name, slots, is_current)
		 VALUES ($1, 'x', '{"skills":[],"equipment":[],"gems":[]}'::jsonb, TRUE)
		 RETURNING id`, owner).Scan(&presetID); err != nil {
		t.Fatalf("造预设失败：%v", err)
	}
	_ = attackerTok

	// 别人读不到：列表为空。
	req2, _ := http.NewRequest("GET", "/api/v1/me/presets", nil)
	req2.Header.Set("Authorization", "Bearer "+attackerTok)
	resp2, err := e.app.Test(req2, 10000)
	if err != nil {
		t.Fatal(err)
	}
	defer resp2.Body.Close()
	var list struct {
		Presets []any `json:"presets"`
	}
	if err := json.NewDecoder(resp2.Body).Decode(&list); err != nil {
		t.Fatal(err)
	}
	if len(list.Presets) != 0 {
		t.Fatalf("他人预设列表应为空，实际 %d", len(list.Presets))
	}

	// 别人的预设：设当前/删除均 404。
	for _, tc := range []struct{ method, path string }{
		{"POST", "/current"},
		{"DELETE", ""},
	} {
		req3, _ := http.NewRequest(tc.method, fmt.Sprintf("/api/v1/me/presets/%d%s", presetID, tc.path), nil)
		req3.Header.Set("Authorization", "Bearer "+attackerTok)
		resp3, err := e.app.Test(req3, 10000)
		if err != nil {
			t.Fatal(err)
		}
		defer resp3.Body.Close()
		if resp3.StatusCode != fiber.StatusNotFound {
			t.Errorf("%s 他人预设应 404，实际 %d", tc.method, resp3.StatusCode)
		}
	}
}
