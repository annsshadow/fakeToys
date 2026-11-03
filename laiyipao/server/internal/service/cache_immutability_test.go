// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R66：共享缓存的可变性审计。
//
// 判据不是「现在没人改」，而是「出参必须让调用方无法写进缓存」：
//   - 含 map 的出参（shop / tasks / signin calendar）在返回处克隆；
//   - 含纯值类型切片的出参（leaderboard / dashboard）只读共享，
//     由三索引切片 + 只读消费约束保证 append 不污染缓存。
//
// 这些用例直接对**缓存对象**做断言：如果克隆退化回「直接返回缓存引用」，
// 下面第一条用例会立即失败（改副本却改到了缓存）。

import (
	"context"
	"fmt"
	"testing"
	"time"
)

func TestCloneCachePayloadsArePrivate(t *testing.T) {
	shop := []shopItem{{
		ID: 1, Code: "s1", Name: "n", Category: "c",
		Price: map[string]int{"coin": 10}, Payload: map[string]int{"coin": 5},
	}}
	shopCopy := cloneShopItems(shop)
	if len(shopCopy) != 1 {
		t.Fatalf("克隆切片长度必须保持 1，实际 %d", len(shopCopy))
	}
	shopCopy[0].Price["coin"] = 999
	shopCopy[0].Payload["coin"] = 999
	if shop[0].Price["coin"] != 10 || shop[0].Payload["coin"] != 5 {
		t.Fatalf("改克隆后的 map 污染了缓存原件：price=%d payload=%d",
			shop[0].Price["coin"], shop[0].Payload["coin"])
	}

	tasks := []taskConfig{{ID: 1, Code: "t1", Reward: map[string]int{"gem": 1}}}
	tasksCopy := cloneTaskConfigs(tasks)
	tasksCopy[0].Reward["gem"] = 999
	if tasks[0].Reward["gem"] != 1 {
		t.Fatalf("改克隆后的任务奖励污染了缓存原件：%d", tasks[0].Reward["gem"])
	}

	days := []SignInCalendarDay{{Day: 1, Reward: map[string]int{"coin": 100}}}
	daysCopy := cloneSignInCalendar(days)
	daysCopy[0].Reward["coin"] = 999
	if days[0].Reward["coin"] != 100 {
		t.Fatalf("改克隆后的签到日历污染了缓存原件：%d", days[0].Reward["coin"])
	}

	// nil 的语义必须保留：`nil` 不是空 map，直接 clone 成 `{}` 会改变
	// 归一化前后的形状，也会让「无奖励」与「奖励为空」变得不可区分。
	if cloneStringIntMap(nil) != nil || cloneShopItems(nil) != nil ||
		cloneTaskConfigs(nil) != nil || cloneSignInCalendar(nil) != nil {
		t.Fatal("nil 输入必须原样返回 nil")
	}
}

func TestSignInCalendarReturnsPrivateCopy(t *testing.T) {
	var s Service
	s.Cfg().SignInCalendarCacheTTL = time.Minute
	days := []SignInCalendarDay{{Day: 1, Reward: map[string]int{"coin": 100}}}
	s.signInCalendarCache.put(days, time.Now())

	got, err := s.SignInCalendar(context.Background())
	if err != nil {
		t.Fatalf("SignInCalendar: %v", err)
	}
	got[0].Reward["coin"] = 999

	again, err := s.SignInCalendar(context.Background())
	if err != nil {
		t.Fatalf("SignInCalendar（第二次）: %v", err)
	}
	if again[0].Reward["coin"] != 100 {
		t.Fatalf("缓存被出参改写污染：coin=%d，期望 100", again[0].Reward["coin"])
	}
}

// TestLeaderboardAppendCannotCorruptCache 钉住三索引切片的结构约束。
//
// 这是「共享只读」唯一的机械保证：元素仍与缓存共享（不复制 200 条），
// 但 `append` 必然分配新数组，所以调用方无法通过 append 覆盖缓存尾部。
func TestLeaderboardAppendCannotCorruptCache(t *testing.T) {
	var s Service
	s.leaderboardCache = map[string]*ttlCache[[]LeaderboardEntry]{"power": {}}
	s.Cfg().LeaderboardCacheTTL = time.Minute
	cached := make([]LeaderboardEntry, leaderboardCacheRows)
	for i := range cached {
		cached[i] = LeaderboardEntry{Rank: i + 1, UserID: int64(i + 1), Nickname: fmt.Sprintf("u%d", i+1)}
	}
	s.leaderboardCache["power"].put(cached, time.Now())

	got, err := s.Leaderboard(context.Background(), "power", 50)
	if err != nil {
		t.Fatalf("Leaderboard: %v", err)
	}
	if len(got) != 50 || cap(got) != 50 {
		t.Fatalf("三索引切片必须 len=cap=50，实际 len=%d cap=%d", len(got), cap(got))
	}
	// 追加写入的是新数组，不能落到缓存的第 51 条。
	_ = append(got, LeaderboardEntry{UserID: -1, Nickname: "boom"})

	again, err := s.Leaderboard(context.Background(), "power", 50)
	if err != nil {
		t.Fatalf("Leaderboard（第二次）: %v", err)
	}
	if again[49].UserID != 50 {
		t.Fatalf("缓存尾部被 append 覆盖：第 50 条 user_id=%d，期望 50", again[49].UserID)
	}
	if len(again) != 50 {
		t.Fatalf("缓存长度被改写：%d", len(again))
	}
}
