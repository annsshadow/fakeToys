// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
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
	"os"
	"path/filepath"
	"strings"
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

// TestDashboardReactionNullSafetyLivesInTrigger 确认 jsonb null 的防线还在，
// 只是从「读侧查询」挪到了「写入侧触发器」。
//
// 第 63 轮把反应分布从「每次刷新扫全表拆 jsonb」改成
// 「战报写入时由触发器累加」（迁移 00025）。于是判据也换了地方：
//
//	读路径：不得再直接解析 battle_records 的 jsonb（那是 R63 之前、
//	        每次刷新扫 100+MB 的写法），只允许读计数器表；
//	触发器：必须 `jsonb_typeof(...) = 'object'` 判类型后才
//	        `jsonb_each_text` —— 否则存量 jsonb `null` 会让插入直接报错
//	        （jsonb_each_text 对非对象是硬错误）。
//
// 为什么这仍然是「不能依赖写侧产出 {}」：触发器是 DB 层，
// 第三方直连、手工修数据、迁移回填都走它；而读侧只剩计数器表，
// 不再有第二个解析 jsonb 的地方。
func TestDashboardReactionNullSafetyLivesInTrigger(t *testing.T) {
	src, err := readServiceSource("stats.go")
	if err != nil {
		t.Skipf("读不到 stats.go：%v（这条守卫未执行，不是「通过了」）", err)
	}

	// 1) 读路径不得直接解析 jsonb。
	if contains(src, "jsonb_each_text") {
		t.Error("stats.go 仍在读路径里解析 battle_records 的 jsonb —— " +
			"这正是第 63 轮消掉的全表扫描；应读 dashboard_reaction_counts")
	}
	if !contains(src, "dashboard_reaction_counts") {
		t.Error("stats.go 的反应分布没有读 dashboard_reaction_counts —— " +
			"计数器表是迁移 00025 建立的唯一数据源")
	}

	// 2) 不得再用 `<> '{}'` 过滤 —— 它对 jsonb null 求值为 NULL。
	if contains(src, "<> '{}'") {
		t.Error("stats.go 仍用 `WHERE col <> '{}'` 过滤 jsonb 列 —— " +
			"它对 jsonb `null` 求值为 NULL 而不是 true，会静默漏掉那些行")
	}

	// 3) 触发器函数必须保留 null 防线。
	raw, err := os.ReadFile(filepath.Join("..", "..", "migrations",
		"00025_dashboard_incremental_aggregates.sql"))
	if err != nil {
		t.Skipf("读不到迁移 00025：%v（这条守卫未执行，不是「通过了」）", err)
	}
	var code []string
	for _, l := range strings.Split(string(raw), "\n") {
		if !strings.HasPrefix(strings.TrimSpace(l), "--") {
			code = append(code, l)
		}
	}
	sql := strings.Join(code, "\n")
	if !contains(sql, "jsonb_typeof") || !contains(sql, "jsonb_each_text") {
		t.Error("迁移 00025 的触发器函数缺少 `jsonb_typeof` + `jsonb_each_text` —— " +
			"存量 jsonb null 会让战报插入直接报错")
	}
	if contains(sql, "<> '{}'") {
		t.Error("迁移 00025 仍用 `<> '{}'` 过滤 —— 对 jsonb null 求值为 NULL，会静默漏行")
	}
}
