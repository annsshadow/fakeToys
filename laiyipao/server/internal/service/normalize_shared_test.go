// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// 第 64 轮的事故回归：`NormalizeNilSlices` 曾经就地改写**共享**响应。
//
// ## 事故现场
//
// `/admin/economy` 的响应缓存 30 秒，缓存里存的是 `AdminEconomyResp`，
// 其中 `Shop[].Price` / `Shop[].Payload` 都是 `map[string]int`。
// 归一器做的是浅拷贝 + `SetMapIndex` 回填 —— map 头部仍指向缓存里的同一个 map。
// 两个并发请求各持有一个「副本」，实际写的是同一张 map：
//
//   fatal error: concurrent map iteration and map write
//   goroutine ... [running]:
//   service.normalizeReflect({0x...})
//       internal/service/normalize.go:82
//
// 这是 **fatal**，不是 panic —— `recover` 中间件拦不住，进程直接退出。
// 实测对 `/admin/economy` 打 10 并发，8 秒内必挂。
//
// ## 判据
//
//  1. 归一化后**入参原对象的内容不变**（用「归一化会补空切片」的输入来证明：
//     若归一再回写到共享 map，原来的 nil 会变成 `[]` —— 可稳定复现）；
//  2. 同一份共享响应被并发归一 1 万次，结果仍与原始内容一致；
//     配合 `-race` 跑，任何对共享 map/切片的写都会被报告。

import (
	"sync"
	"testing"
)

// 归一化一定会改的形状：map 的值是 nil 切片 → 归一后应变成 `[]`。
// 如果这个 `[]` 被写回**共享的 map**，第二次读到的就不是 nil 了。
func TestNormalizeNilSlicesDoesNotMutateSharedMap(t *testing.T) {
	type row struct {
		Payload map[string][]string `json:"payload"`
	}
	type resp struct {
		Items []row `json:"items"`
	}
	shared := resp{Items: []row{{Payload: map[string][]string{"coin": nil}}}}

	if _, changed := shared.Items[0].Payload["coin"]; !changed {
		t.Fatal("前提不成立：map 里没有 coin 键")
	}
	got := NormalizeNilSlices(shared)
	// 副本必须被归一
	g, ok := got.(resp)
	if !ok {
		t.Fatalf("返回类型变了：%T", got)
	}
	if s, ok := g.Items[0].Payload["coin"]; !ok || s == nil {
		t.Errorf("副本没有被归一：coin=%v（应为空切片）", s)
	}
	// 原对象必须原样（nil）
	if shared.Items[0].Payload["coin"] != nil {
		t.Error("NormalizeNilSlices 把归一结果写回了入参的共享 map —— " +
			"缓存响应被并发改写会 fatal（R64 事故）")
	}
}

// 用真实的缓存响应形状（`AdminShopItemView` 的 Price/Payload）做并发回归。
func TestNormalizeNilSlicesConcurrentOnCachedShape(t *testing.T) {
	type resp struct {
		Shop []AdminShopItemView `json:"shop"`
	}
	items := make([]AdminShopItemView, 20)
	for i := range items {
		items[i] = AdminShopItemView{
			ID:      i + 1,
			Code:    "item",
			Price:   map[string]int{"coin": i + 1},
			Payload: map[string]int{"coin": i + 1},
			Enabled: true,
		}
	}
	shared := resp{Shop: items}

	var wg sync.WaitGroup
	for g := 0; g < 64; g++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for i := 0; i < 200; i++ {
				_ = NormalizeNilSlices(shared)
			}
		}()
	}
	wg.Wait()

	for i, it := range shared.Shop {
		if it.Price["coin"] != i+1 || it.Payload["coin"] != i+1 || !it.Enabled {
			t.Fatalf("并发归一改动了共享响应：第 %d 行 = %+v", i, it)
		}
	}
}
