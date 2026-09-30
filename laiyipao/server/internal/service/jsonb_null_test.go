package service

// jsonb `null` 归一化的守卫。
//
// ## 要防的是什么
//
// `json.Marshal(map[string]int(nil))` 返回 `[]byte("null")`。
// 直接入库 → 列里是 **jsonb `null`**（不是 SQL NULL）。
//
// 危害有两条，都是实测过的：
//
//  1. `null <> '{}'::jsonb` 在 SQL 里求值为 **NULL 而不是 true**，
//     于是 `WHERE reactions_used <> '{}'` 把这些行**静默排除**。
//     实测 58 行里 8 行（13.8%）的反应分布没进后台看板 ——
//     按反应次数算漏了 188/263 = 71.5%。
//
//  2. `jsonb_each_text(jsonb 'null')` 直接**报错**
//     （「不能在非对象上调用」），任何 JSONB 聚合 SQL 都会崩。
//
// ## 为什么要测「序列化结果」而不是「数据库里有没有 null」
//
// 写侧的正确性是**可以脱离数据库**判定的：
// `orEmptyMap(nil)` 序列化出什么，决定了将来写进去的是什么。
// 测这一层不需要 DB，CI 里任何时候都能跑 ——
// 而测 DB 需要真库，本项目已经多次因为「测试库没迁移」而误判。
//
// DB 那一层由 migration 00010 负责修存量。

import (
	"encoding/json"
	"testing"
)

func TestOrEmptyMapSerializesToObjectNotNull(t *testing.T) {
	// nil map 是问题根源：它序列化出来是 `null`
	b, err := json.Marshal(map[string]int(nil))
	if err != nil {
		t.Fatal(err)
	}
	if string(b) != "null" {
		t.Fatalf("前提不成立：nil map 序列化结果是 %q 而不是 null —— "+
			"Go 版本或写法变了，本守卫的前提需重新确认", b)
	}

	// 归一化后必须是 `{}`
	got, err := json.Marshal(orEmptyMap(nil))
	if err != nil {
		t.Fatal(err)
	}
	if string(got) != "{}" {
		t.Fatalf("orEmptyMap(nil) 序列化结果是 %q，应为 {} —— "+
			"存进去就会是 jsonb null，统计会静默漏数据", got)
	}
}

func TestOrEmptySliceSerializesToArrayNotNull(t *testing.T) {
	b, err := json.Marshal([]string(nil))
	if err != nil {
		t.Fatal(err)
	}
	if string(b) != "null" {
		t.Fatalf("前提不成立：nil slice 序列化结果是 %q 而不是 null", b)
	}
	got, err := json.Marshal(orEmptySlice[string](nil))
	if err != nil {
		t.Fatal(err)
	}
	if string(got) != "[]" {
		t.Fatalf("orEmptySlice(nil) 序列化结果是 %q，应为 []", got)
	}
}

func TestOrEmptyPreservesNonNil(t *testing.T) {
	// ⚠️ 这条容易被忽略：归一化**不能**动非 nil 的值。
	// 早期版本若写成 `if len(m) == 0 { return map{} }`，
	// 会把「客户端上报了空 map」和「客户端没上报」混为一谈 ——
	// 而这两者在审计上是有区别的（虽然统计上等价）。
	m := map[string]int{"fire": 3}
	if got := orEmptyMap(m); len(got) != 1 || got["fire"] != 3 {
		t.Errorf("orEmptyMap 动了非 nil map：%v", got)
	}
	if got := orEmptyMap(map[string]int{}); got == nil {
		t.Error("orEmptyMap 把非 nil 空 map 变成了 nil —— 那是反向的 bug")
	}

	s := []string{"a"}
	if got := orEmptySlice(s); len(got) != 1 {
		t.Errorf("orEmptySlice 动了非 nil slice：%v", got)
	}
	if got := orEmptySlice([]string{}); got == nil {
		t.Error("orEmptySlice 把非 nil 空 slice 变成了 nil")
	}
}

// TestDashboardReactionQueryIsNullSafe 确认看板查询不再依赖「写侧一定产出 {}」。
//
// 为什么读侧也要改：
//
// 写侧归一化只保证**今后**的写入是 `{}`。
// 存量数据、第三方直连、手工修数据都可能再带 `null` 进来，
// 而查询若写成 `WHERE col <> '{}'`，就会再次静默漏数据。
//
// 这里的判据是**查询文本本身** —— 因为它守的是
// 「查询不能依赖写侧」这条设计约束，
// 而不是「当前数据里没有 null」（那是数据状态，会变）。
func TestDashboardReactionQueryIsNullSafe(t *testing.T) {
	src, err := readServiceSource("stats.go")
	if err != nil {
		t.Skipf("读不到 stats.go：%v（这条守卫未执行，不是「通过了」）", err)
	}

	// 1) 不得再用 `<> '{}'` 过滤 —— 它对 null 求值为 NULL
	if contains(src, "<> '{}'") {
		t.Error("stats.go 仍用 `WHERE col <> '{}'` 过滤 jsonb 列 —— " +
			"它对 jsonb `null` 求值为 NULL 而不是 true，会静默漏掉那些行。" +
			"应改用 `jsonb_typeof(col) = 'object'`")
	}

	// 2) 必须显式判类型
	if !contains(src, "jsonb_typeof") {
		t.Error("stats.go 的反应分布查询没有 `jsonb_typeof` 判类型 —— " +
			"存量 jsonb null 会让 `jsonb_each_text` 直接报错")
	}

	// 3) 不得再有「按整个 map 分组 + LIMIT 截断」的写法。
	//    组合数随战报量爆炸（6 种反应各 0~5 次 = 6^6 种），
	//    `GROUP BY reactions_used LIMIT 500` 会开始丢数据。
	if contains(src, "GROUP BY reactions_used") {
		t.Error("stats.go 仍按整个 `reactions_used` map 分组并 LIMIT 截断 —— " +
			"组合数会随战报量爆炸，截断会静默丢数据。应在 SQL 里按 key 聚合")
	}
}
