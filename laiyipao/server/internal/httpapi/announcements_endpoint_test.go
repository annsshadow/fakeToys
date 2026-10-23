package httpapi

// `GET /api/v1/announcements` 的形状契约。
//
// # 为什么单独立一条形状断言
//
// service 层已经测了「只返回已发布」「排序」「limit 夹住」。
// 这里测的是**客户端真正依赖的那一层**：响应顶层是 `{items: [...]}`，
// 且空时是 `[]` 而不是 `null`。
//
// 这条不是形式主义：本仓库已经栽过一次 —— Go 把 nil 切片编码成 `null`，
// 而生成的 TS 类型写的是 `T[]`，于是后台抽屉对多数记录直接抛 TypeError
// （见 `2026-10-21-laiyipao-three-end-optimization.md` 附录 B-1）。
// 那次之后，凡是「客户端会 `.map()` / `.length` 的数组」都要在**响应边界**上钉一次。
//
// 无 Postgres 时 `newE2E` 会 Skip —— 那一刻这些断言是**未执行**，不是「通过」。

import "testing"

func TestAnnouncementsEndpointShape(t *testing.T) {
	e := newE2E(t)

	status, body := e.get(t, "/api/v1/announcements", "")
	if status != 200 {
		t.Fatalf("状态码 = %d，期望 200（这是公开端点，不带令牌也必须能读）", status)
	}
	raw, ok := body["items"]
	if !ok {
		t.Fatalf("响应里没有 items 字段（顶层键：%v）—— 客户端按 `body.items` 取值", keysOf(body))
	}
	// JSON 数组解码后是 []any；`null` 解出来是 nil。
	// 两者在 Go 侧都是 `any`，必须显式判 nil —— 这正是 B-1 的形态。
	if raw == nil {
		t.Fatal("items 是 null —— 空列表必须编码成 []，" +
			"否则客户端对 null 调 .length / .map 会抛")
	}
	if _, ok := raw.([]any); !ok {
		t.Fatalf("items 不是数组，实际类型 %T", raw)
	}
}

func keysOf(m map[string]any) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	return out
}
