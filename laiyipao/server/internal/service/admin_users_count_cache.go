// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"context"
	"sync"
	"time"
)

// adminUsersCountCacheMaxKeys 是按键缓存的硬上限。
//
// 筛选组合是运营自由输入的（关键词 × 日期 × 游客 × 最后登录 × 需核查），
// 理论上无界。超过上限时整表清空而不是继续增长：清空只会让下一批请求
// 各多跑一次 COUNT(*)，**不会**返回错的 total；而无界增长是内存泄漏。
const adminUsersCountCacheMaxKeys = 64

// adminUsersCountCache 是「按筛选条件分键」的 total 缓存（R61）。
//
// # 为什么不用 ttlCache 单值缓存
//
// ttlCache 只有一个键。后台可能同时打开「无筛选」「按昵称搜」「按日期筛」
// 三个列表，单值缓存会互相顶掉 → 每个请求都未命中，等于没加缓存。
//
// # 语义
//
//   - 每个键独立 TTL、独立单飞（复用 ttlCache）；
//   - `ttl <= 0` 完全绕过缓存（`ADMIN_USERS_COUNT_CACHE_TTL=0` 时每次真值）；
//   - 加载失败不缓存（由 ttlCache 保证），下一次请求重试。
//
// 正确性边界：本缓存只影响 total 的**新鲜度上限**（默认 5 秒），
// 不影响任何写路径，也不存在「计数器漂移后永远修不回来」的可能 ——
// 每个键到期都会重新执行 `COUNT(*)`。
type adminUsersCountCache struct {
	mu      sync.Mutex
	entries map[string]*ttlCache[int64]
}

func (c *adminUsersCountCache) getOrLoad(
	ctx context.Context,
	now time.Time,
	ttl time.Duration,
	key string,
	load func(context.Context) (int64, error),
) (int64, error) {
	if ttl <= 0 {
		return load(ctx)
	}
	c.mu.Lock()
	if c.entries == nil {
		c.entries = make(map[string]*ttlCache[int64], adminUsersCountCacheMaxKeys)
	}
	if len(c.entries) >= adminUsersCountCacheMaxKeys {
		c.entries = make(map[string]*ttlCache[int64], adminUsersCountCacheMaxKeys)
	}
	e := c.entries[key]
	if e == nil {
		e = &ttlCache[int64]{}
		c.entries[key] = e
	}
	c.mu.Unlock()
	return e.getOrLoad(ctx, now, ttl, load)
}

// adminUsersCountKey 把完整筛选条件编码成缓存键。
//
// nil 与 false 必须可区分：`suspicious=nil` 时 SQL 根本不带需核查条件，
// 而 `suspicious=false` 会带 `= false`，两种口径的 total 不同。
func adminUsersCountKey(kw string, dateFrom, dateTo *time.Time, isGuest *bool,
	lastLoginFrom, lastLoginTo *time.Time, suspicious *bool) string {
	return kw + "|" + timeKey(dateFrom) + "|" + timeKey(dateTo) + "|" + boolKey(isGuest) +
		"|" + timeKey(lastLoginFrom) + "|" + timeKey(lastLoginTo) + "|" + boolKey(suspicious)
}

func timeKey(t *time.Time) string {
	if t == nil {
		return "-"
	}
	return t.UTC().Format(time.RFC3339Nano)
}

func boolKey(b *bool) string {
	if b == nil {
		return "-"
	}
	if *b {
		return "1"
	}
	return "0"
}
