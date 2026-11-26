// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package httpapi

import (
	"testing"

	"github.com/laiyipao/server/internal/domain"
)

// TestConfigGemsIsNonEmptyForCodex：宝石图鉴完全依赖 /config.gems，
// 客户端一旦自行加载就必须拿到权威非空表。此前基线截图显示「宝石表为空」，
// 容易误判成后端未播种；这条用例用真 app + 真库钉住服务端下发事实。
func TestConfigGemsIsNonEmptyForCodex(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newPlayer(t, "config-gems")

	status, body := e.get(t, "/api/v1/config", tok)
	if status != 200 {
		t.Fatalf("GET /api/v1/config = %d，期望 200", status)
	}
	gems, ok := body["gems"].([]any)
	if !ok {
		t.Fatalf("config.gems 应为数组，实际 %T", body["gems"])
	}
	if len(gems) == 0 || len(gems) != len(domain.SeedGems) {
		t.Fatalf("config.gems 应有 %d 条权威宝石定义，实际 %d", len(domain.SeedGems), len(gems))
	}
	for i, raw := range gems {
		g, ok := raw.(map[string]any)
		if !ok {
			t.Fatalf("config.gems[%d] 应为对象，实际 %T", i, raw)
		}
		for _, key := range []string{"code", "name", "attr"} {
			s, ok := g[key].(string)
			if !ok || s == "" {
				t.Errorf("config.gems[%d].%s 缺失或为空：%v", i, key, g[key])
			}
		}
	}
}
