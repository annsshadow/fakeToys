package httpapi

import (
	"net/url"
	"testing"
)

// 第 111 轮：关卡列表的章节/关键词过滤**必须真的到达 SQL**。
//
// 修前 handler（adminLevels）对 `?chapter=` 与 `?keyword=` 一个都不收，
// service 的 AdminListLevels 也没这两个参数——
// UI 发了也白发，永远回全量 100 关。
//
// 判据：无参 = 全量；chapter = 只剩该章；keyword = 命中子集；
// chapter 越界（第 99 章）= 0 行（而不是全量）；坏参数 = 400。
func TestAdminLevelsFiltersReachSQL(t *testing.T) {
	e := newE2E(t)
	_, tok := e.newAdmin(t, "lvlfilter111")

	// e.get 返回 (状态码, 响应体)
	_, full := e.get(t, "/api/v1/admin/levels", tok)
	fullItems, _ := full["items"].([]any)
	if len(fullItems) == 0 {
		t.Fatal("全量关卡列表为空，无法建立基准")
	}

	// 章节 3（冻原）
	_, ch := e.get(t, "/api/v1/admin/levels?chapter=3", tok)
	chItems, _ := ch["items"].([]any)
	if len(chItems) == 0 {
		t.Fatal("第 3 章过滤后为空——要么过滤没生效（应非空），要么该章无数据")
	}
	for _, it := range chItems {
		m, _ := it.(map[string]any)
		if c, ok := m["chapter"]; !ok || toF(c) != 3 {
			t.Fatalf("第 3 章过滤混进了其它章的行：%v", m["chapter"])
		}
	}

	// 越界章节：第 99 章不存在 → 必须 0 行（不是全量）。
	// 注意：空结果在 JSON 里是 null（Go 侧 nil），comma-ok 取法才不会 panic。
	_, none := e.get(t, "/api/v1/admin/levels?chapter=99", tok)
	nItems, _ := none["items"].([]any)
	if n := len(nItems); n != 0 {
		t.Errorf("不存在的第 99 章应回 0 行，实际 %d（过滤没生效就会回全量）", n)
	}

	// 关键词：关卡名形如「冻原 · 41」，「· 41」应只命中第 41 关。
	// 查询串里的空格/全角字符必须 URL 编码，否则 URL 本身非法。
	_, kw := e.get(t, "/api/v1/admin/levels?keyword="+url.QueryEscape("· 41"), tok)
	kwItems, _ := kw["items"].([]any)
	if n := len(kwItems); n != 1 {
		t.Errorf("关键词「· 41」应命中 1 行，实际 %d", n)
	}

	// 参数解析失败必须 400（第 104 轮同族：坏参数不当成「不过滤」）
	st, _ := e.get(t, "/api/v1/admin/levels?chapter=abc", tok)
	if st != 400 {
		t.Errorf("chapter=abc（非数字）应 400，实际 %d（静默不过滤会回全量）", st)
	}
}
