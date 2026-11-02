// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"encoding/json"
	"testing"
)

// 归一器的调用方全部是 `c.JSON(NormalizeNilSlices(Resp{...}))` —— **传值**。
// 值类型从 interface 里取出后字段不可写，旧实现在这里空转：
// 归一后的 JSON 仍是 null，客户端对 null 调 .length 直接抛。
func TestNormalizeNilSlicesValueStruct(t *testing.T) {
	type resp struct {
		Items []int          `json:"items"`
		Stars map[string]int `json:"stars"`
	}
	got := marshal(t, NormalizeNilSlices(resp{}))
	if got != `{"items":[],"stars":{}}` {
		t.Fatalf("值类型归一失败：got %s，want {\"items\":[],\"stars\":{}}", got)
	}
}

// 指针调用是既有用法，修完必须保持可用。
func TestNormalizeNilSlicesPointerStruct(t *testing.T) {
	type resp struct {
		Items []int `json:"items"`
	}
	r := &resp{}
	got := marshal(t, NormalizeNilSlices(r))
	if got != `{"items":[]}` {
		t.Fatalf("指针归一失败：got %s", got)
	}
	if r.Items == nil {
		t.Fatal("指针入参应就地修改")
	}
}

// 嵌套字段（列表行里的子切片）也要归一。
func TestNormalizeNilSlicesNestedStruct(t *testing.T) {
	type row struct {
		Tags []string `json:"tags"`
	}
	type resp struct {
		Items []row `json:"items"`
	}
	got := marshal(t, NormalizeNilSlices(resp{Items: []row{{Tags: nil}}}))
	if got != `{"items":[{"tags":[]}]}` {
		t.Fatalf("嵌套归一失败：got %s", got)
	}
}

// 玩家端列表响应的真实形状是 `fiber.Map{"items": items}`（顶层 map），
// 归一器此前只处理 struct，map 里的 nil 值会原样编码成 null。
func TestNormalizeNilSlicesTopLevelMap(t *testing.T) {
	got := marshal(t, NormalizeNilSlices(map[string]any{
		"items": []TaskView(nil),
		"stars": map[string]int(nil),
		"scope": "daily",
	}))
	if got != `{"items":[],"scope":"daily","stars":{}}` {
		t.Fatalf("顶层 map 归一失败：got %s", got)
	}
}

// 非接口元素的 typed map：nil 切片同样要变 []。
func TestNormalizeNilSlicesTypedMapValue(t *testing.T) {
	got := marshal(t, NormalizeNilSlices(map[string][]int{"x": nil}))
	if got != `{"x":[]}` {
		t.Fatalf("typed map 归一失败：got %s", got)
	}
}

// 非空集合必须原样保留（归一不是清空）。
func TestNormalizeNilSlicesKeepsNonEmpty(t *testing.T) {
	got := marshal(t, NormalizeNilSlices(map[string]any{
		"items": []int{1, 2},
		"stars": map[string]int{"1": 3},
	}))
	if got != `{"items":[1,2],"stars":{"1":3}}` {
		t.Fatalf("非空集合被改动：got %s", got)
	}
}

func marshal(t *testing.T, v any) string {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatalf("编码失败：%v", err)
	}
	return string(raw)
}
