package service

import (
	"sync"
	"time"
)

// ttlCache 是「单值 + 有效期」的进程内缓存。
//
// # 为什么手写而不是引一个库
//
// 需要的语义只有三条：TTL 内命中、过期不命中、可手动失效。
// 引一个缓存库会顺带引入容量淘汰 / 单飞 / 指标，而本仓库目前
// 只缓存两个**全量快照**（看板、榜单），没有容量问题 ——
// 加了淘汰策略反而要在文档里解释「为什么永远淘汰不到」。
//
// # 为什么零值是「未命中」
//
// 结构体零值即可用（不需要构造），于是 `Service` 里的字段不用初始化，
// 也就不存在「忘了初始化 → 每次都不命中」这种静默失效。
//
// # 并发
//
// `sync.Mutex` 而不是 RWMutex：临界区只有一次字段拷贝，
// 读写锁的额外复杂度换不来什么。
//
// # 缓存值是可共享的
//
// `get` 返回的是**同一个值**（切片/结构体按值拷贝，但切片底层数组共享）。
// 调用方必须只读。本仓库两个使用点都是直接 JSON 编码，满足这条。
type ttlCache[T any] struct {
	mu  sync.Mutex
	at  time.Time
	val T
	ok  bool
}

// get 在 TTL 内返回值。
//
// `ttl <= 0` 恒不命中 —— 这是「关闭缓存」的表达方式，也是测试与
// 排障时的开关（把环境变量设成 0 即回到「每次查库」）。
func (c *ttlCache[T]) get(now time.Time, ttl time.Duration) (T, bool) {
	var zero T
	if ttl <= 0 {
		return zero, false
	}
	c.mu.Lock()
	defer c.mu.Unlock()
	if !c.ok || now.Sub(c.at) >= ttl {
		return zero, false
	}
	return c.val, true
}

func (c *ttlCache[T]) put(v T, now time.Time) {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.val = v
	c.at = now
	c.ok = true
}

// flush 立刻失效。
//
// 用途：测试里「改完数据要读到新值」，以及将来运营点「刷新」。
func (c *ttlCache[T]) flush() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.ok = false
	var zero T
	c.val = zero
}
