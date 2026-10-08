package httpapi

import (
	"net/url"
	"testing"
)

// 任务周期的非法取值必须 **400**，而不是「200 + 空列表」（第 106 轮）。
//
// # 缺陷（修前实测）
//
//	?scope=daily      status=200 len(items)=104  回显=daily
//	?scope=daliy      status=200 len(items)=0    回显=daliy
//	?scope=Daily      status=200 len(items)=0    回显=Daily
//
// 玩家看到的是「**今天没有任务**」—— 一个**看起来完全合理**的答案，
// 而且它按玩家、按天出现，很可能永远不会被发现。
//
// 关键点：响应里 `"scope"` **回显了那个未知的值**，
// 所以客户端也察觉不到异常。
func TestTasksRejectsUnknownScope(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newPlayer(t, "scope106")

	// 三个合法取值必须照常工作，且**列表非空** ——
	// 「非空」是关键：只看 200 的话，一个永远返回空列表的端点也「通过」。
	for _, sc := range []string{"daily", "weekly", "achievement"} {
		st, body := e.get(t, "/api/v1/tasks/?scope="+sc, tok)
		if st != 200 {
			t.Errorf("scope=%s 应 200，实际 %d：%v", sc, st, body)
			continue
		}
		items, _ := body["items"].([]any)
		if len(items) == 0 {
			t.Errorf("scope=%s 返回了空列表 —— 种子数据里该有任务", sc)
		}
		if got := body["scope"]; got != sc {
			t.Errorf("scope=%s 的响应回显 %v —— 回显必须与实际数据一致", sc, got)
		}
		// 每一项的 scope 字段也要自洽
		for _, it := range items {
			m, ok := it.(map[string]any)
			if !ok {
				continue
			}
			if m["scope"] != sc {
				t.Errorf("请求 scope=%s，返回里却混进了 scope=%v 的条目", sc, m["scope"])
			}
		}
	}

	// 不带参数 → daily（默认值必须仍然可用）
	st, body := e.get(t, "/api/v1/tasks/", tok)
	if st != 200 {
		t.Fatalf("不带 scope 应 200（默认 daily），实际 %d：%v", st, body)
	}
	if body["scope"] != "daily" {
		t.Errorf("不带 scope 时应回显 daily，实际 %v", body["scope"])
	}

	// 非法取值一律 400
	//
	// ⚠️ 必须用 `url.QueryEscape` 编码。
	// 我第一版把 `"daily "` 和 `"%20daily"` 直接拼进 URL，
	// 而 `httptest.NewRequest` 遇到未编码的空格会
	// **panic: malformed HTTP version** —— 崩掉，不是红。
	// 一个会崩的判据既没测出东西，又把真正的失败原因藏起来了。
	for _, bad := range []string{
		"bogus", "daliy", "Daily", "DAILY", "daily ", " daily",
		"achievment", "achievements", "week", "weeklies", "\x00",
	} {
		st, body := e.get(t, "/api/v1/tasks/?scope="+url.QueryEscape(bad), tok)
		if st != 400 {
			t.Errorf("scope=%q 应 400，实际 %d：%v\n"+
				"返回 200 + 空列表会让玩家以为「今天没有任务」—— "+
				"一个看起来完全合理的错误答案。", bad, st, body)
			continue
		}
		// 400 的响应里**不得**出现 items（那会让人以为「查到了，只是没有」）
		if _, has := body["items"]; has {
			t.Errorf("scope=%q 的 400 响应里带了 items：%v", bad, body)
		}
	}

	// `?scope=`（显式空串）也是非法 —— 与第 104 轮 queryInt 同理：
	// 「显式传空」与「没传」是两种输入，Fiber 的 c.Query(key,default) 会把它们压成同一个值。
	if st, _ := e.get(t, "/api/v1/tasks/?scope=", tok); st != 400 {
		t.Errorf("?scope= （显式空串）应 400，实际 %d\n"+
			"前端从不发空参数（admin/src/api 用 URLSearchParams 且只在非空时 set），"+
			"所以这里判非法不会误伤任何真实调用。", st)
	}
}
