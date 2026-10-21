package httpapi

import (
	"context"
	"sort"
	"testing"
)

// TestE2EMasteryResetResponseMatchesGet 钉住 P3-7。
//
// # 背景
//
// `GET /mastery` 与 `POST /mastery/reset` 的响应形状此前**不一致**：
// reset 少了 `families`（专精系定义）。
//
// 后果：客户端重铸专精后如果靠这个响应刷新专精树，就必须**再拉一次**
// `/mastery`；而更糟的是「两个端点形状一样」这个假设一旦被默认，
// 下一个字段加在一边就会静默不同步 —— 没有任何测试会发现。
//
// # 判据
//
// 两个端点的响应**键集合必须相同**（reset 多一个 reset_cleared）。
//
// 这条能失败：把 `families` 从 reset 的响应里去掉，它立刻红。
func TestE2EMasteryResetResponseMatchesGet(t *testing.T) {
	e := newE2E(t)
	uid, tok := e.newPlayer(t, "mastery_reset_shape")

	// 先拿一个专精重铸券，否则 reset 会 400（缺道具）
	if _, err := e.pool.Exec(context.Background(),
		`INSERT INTO user_tokens (user_id, token, balance)
		 SELECT $1, 'mastery_reset', 1
		 WHERE NOT EXISTS (SELECT 1 FROM user_tokens
		                   WHERE user_id = $1 AND token = 'mastery_reset')`,
		uid); err != nil {
		t.Skipf("发放重铸券失败（表结构可能变了）：%v", err)
	}

	status, getBody := e.get(t, "/api/v1/mastery/", tok)
	if status != 200 {
		t.Fatalf("GET /mastery 应 200，实际 %d %v", status, getBody)
	}
	status, resetBody := e.post(t, "/api/v1/mastery/reset", tok, map[string]any{})
	if status != 200 {
		t.Fatalf("POST /mastery/reset 应 200，实际 %d %v", status, resetBody)
	}

	getKeys := sortedKeys(getBody)
	resetKeys := sortedKeys(resetBody)

	// reset 允许多一个 reset_cleared，但**不能少**任何一个键。
	for _, k := range getKeys {
		if !containsStr(resetKeys, k) {
			t.Errorf("POST /mastery/reset 的响应缺 %q（GET /mastery 有）—— "+
				"两个端点形状不一致，客户端要靠猜来决定能不能直接刷新 UI", k)
		}
	}
	// 反向：reset 不该有 GET 没有的键（除了 reset_cleared）
	for _, k := range resetKeys {
		if k == "reset_cleared" {
			continue
		}
		if !containsStr(getKeys, k) {
			t.Errorf("POST /mastery/reset 多出 %q（GET /mastery 没有）", k)
		}
	}
	if _, ok := resetBody["reset_cleared"]; !ok {
		t.Error("POST /mastery/reset 应带 reset_cleared（本次清掉的节点数）")
	}
	if resetBody["families"] == nil {
		t.Error("POST /mastery/reset 必须带 families（P3-7：此前缺这个字段）")
	}
}

func sortedKeys(m map[string]any) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}

func containsStr(list []string, s string) bool {
	for _, v := range list {
		if v == s {
			return true
		}
	}
	return false
}
