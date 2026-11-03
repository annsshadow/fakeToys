// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R66：缓存出参私有化的成本基准。
//
// 口径与 §4.1 的端点 p95 对比用：这些函数都在**每次请求**调用一次，
// ns/op × 端点 QPS 就是新增的 CPU 占比。基准数据量取线上典型值：
// 商城 12 件、任务 12 条、签到 7 天、经济页 50 行流水 + 12 件商品。
//
// 运行：go test -bench 'BenchmarkCache' -benchmem -run '^$' ./internal/service

import "testing"

func benchShopItems(n int) []shopItem {
	out := make([]shopItem, n)
	for i := range out {
		out[i] = shopItem{
			ID: i + 1, Code: "item", Name: "name", Category: "cat", Limit: 3,
			Price:   map[string]int{"coin": 100, "gem": 2, "energy": 1},
			Payload: map[string]int{"coin": 200, "gem": 4, "energy": 2},
		}
	}
	return out
}

func BenchmarkCacheCloneShopItems(b *testing.B) {
	items := benchShopItems(12)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		_ = cloneShopItems(items)
	}
}

func BenchmarkCacheCloneTaskConfigs(b *testing.B) {
	cfgs := make([]taskConfig, 12)
	for i := range cfgs {
		cfgs[i] = taskConfig{
			ID: i + 1, Code: "task", Name: "name", Scope: "daily", Target: 3,
			Metric: "battles", Reward: map[string]int{"coin": 100, "gem": 1, "energy": 1},
		}
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		_ = cloneTaskConfigs(cfgs)
	}
}

func BenchmarkCacheCloneSignInCalendar(b *testing.B) {
	days := make([]SignInCalendarDay, 7)
	for i := range days {
		days[i] = SignInCalendarDay{Day: i + 1, Reward: map[string]int{"coin": 100, "gem": 1}}
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		_ = cloneSignInCalendar(days)
	}
}

// BenchmarkCacheNormalizeEconomy 是 R64 之后 /admin/economy 每次请求的
// 深拷贝成本（handler 的 NormalizeNilSlices）。
func BenchmarkCacheNormalizeEconomy(b *testing.B) {
	resp := AdminEconomyResp{Flows: make([]AdminEconomyFlowView, 50), Shop: make([]AdminShopItemView, 12)}
	for i := range resp.Flows {
		resp.Flows[i] = AdminEconomyFlowView{Reason: "battle_reward", Currency: "coin", Count: 10, Delta: 100}
	}
	for i := range resp.Shop {
		resp.Shop[i] = AdminShopItemView{
			ID: i + 1, Code: "item", Name: "name", Category: "cat", LimitPerDay: 3,
			Price:   map[string]int{"coin": 100, "gem": 2, "energy": 1},
			Payload: map[string]int{"coin": 200, "gem": 4, "energy": 2},
		}
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		_ = NormalizeNilSlices(resp)
	}
}

// BenchmarkCacheNormalizeShopResponse 是 /shop 每次请求的归一化成本：
// 克隆（service 返回处）+ 归一化（handler 深拷贝）各一次。
func BenchmarkCacheNormalizeShopResponse(b *testing.B) {
	items := benchShopItems(12)
	views := make([]ShopItemView, len(items))
	for i, it := range items {
		views[i] = ShopItemView{
			ID: it.ID, Code: it.Code, Name: it.Name, Category: it.Category,
			Price: it.Price, Payload: it.Payload, Limit: it.Limit,
		}
	}
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		var v any = map[string]any{"items": views}
		_ = NormalizeNilSlices(v)
	}
}
