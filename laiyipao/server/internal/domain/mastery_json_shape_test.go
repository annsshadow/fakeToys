package domain

import (
	"encoding/json"
	"strings"
	"testing"
)

// TestMasteryTreeJSONShapeIsLowercase 钉住专精树在 wire 上的**键名大小写**。
//
// # 背景（2026-10-21 发现的一个 P0）
//
// `MasteryFamily` / `MasteryNode` 此前**没有 json tag**，于是 Go 按字段名
// 序列化成 **PascalCase**（`Family` / `Name` / `Nodes` / `ID` / `PrereqFam`…）。
//
// 而两端客户端都读**小写**：
//
//   - 小程序专精页 `families.find(x => x.family === curFamily)` → 永远匹配不上
//     → **整棵专精树不渲染**；
//   - 后台文档站 `f.nodes.filter(...)` → TypeError。
//
// 两边的测试都是绿的，因为**夹具按前端形状写** ——
// 正是本仓库反复要消灭的「fixture 写前端形状 → 永远绿」形态：
// 夹具和线上**一致地错**，于是缺陷对两侧都不可见。
//
// # 判据
//
// 编出来的 JSON 必须是**小写键**，且**不得**出现大写键。
// 这条能失败：把 json tag 去掉，它立刻红。
func TestMasteryTreeJSONShapeIsLowercase(t *testing.T) {
	fams := AllMasteryFamilies()
	if len(fams) == 0 {
		t.Fatal("专精树是空的 —— 这个测试会静默地什么也不检查")
	}

	raw, err := json.Marshal(fams[0])
	if err != nil {
		t.Fatalf("序列化专精系失败：%v", err)
	}
	got := string(raw)

	// 客户端读的就是这些键
	for _, key := range []string{`"family"`, `"name"`, `"nodes"`} {
		if !strings.Contains(got, key) {
			t.Errorf("专精系的 JSON 里没有 %s —— 客户端读的是小写键：%s", key, got)
		}
	}
	// 大写键一律不许出现
	for _, key := range []string{`"Family"`, `"Name"`, `"Nodes"`, `"ID"`, `"PrereqFam"`, `"PrereqLay"`} {
		if strings.Contains(got, key) {
			t.Errorf("专精系的 JSON 里出现了大写键 %s —— 客户端读的是小写：%s", key, got)
		}
	}

	// 节点也是同一件事
	nodes := fams[0].Nodes
	if len(nodes) == 0 {
		t.Fatal("专精系没有节点 —— 这个测试会静默地什么也不检查")
	}
	nraw, err := json.Marshal(nodes[0])
	if err != nil {
		t.Fatalf("序列化专精节点失败：%v", err)
	}
	ngot := string(nraw)
	for _, key := range []string{`"id"`, `"layer"`, `"slot"`, `"kind"`, `"value"`, `"prereq_family"`, `"prereq_layer"`} {
		if !strings.Contains(ngot, key) {
			t.Errorf("专精节点的 JSON 里没有 %s：%s", key, ngot)
		}
	}
	for _, key := range []string{`"ID"`, `"Family"`, `"Layer"`, `"Slot"`, `"Name"`, `"Kind"`, `"Value"`, `"PrereqFam"`, `"PrereqLay"`} {
		if strings.Contains(ngot, key) {
			t.Errorf("专精节点的 JSON 里出现了大写键 %s：%s", key, ngot)
		}
	}
}
