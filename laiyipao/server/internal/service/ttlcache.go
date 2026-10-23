package service

import (
	"sync"
	"sync/atomic"
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
// # 缓存击穿（singleflight）
//
// TTL 到期的那一刻，若有 N 个请求同时打到未命中，它们会**各自**
// 跑一遍加载函数 —— 看板是 9 段查询、榜单是 3 次 GROUP BY + 200 行，
// 这就是缓存击穿（惊群）。`getOrLoad` 用单飞把它压成一次：
// 第一个请求当 loader，其余阻塞等它的结果，而不是各打一次库。
// 手写而不引 `x/sync/singleflight`：那样会把「单值缓存」和
// 「多键单飞组」两种语义揉在一起，而这里每个 `ttlCache` 只有一个键。
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
	// inflight 非 nil 表示已有一个 goroutine 正在加载。后到的goroutine
	// 不复算，转而等这份结果。零值 nil = 无在途加载，结构体零值仍可用。
	inflight *loadCall[T]
	// hits/misses 是缓存命中/未命中计数（getOrLoad 口径），供 /metrics 观测。
	// 原子计数而非在 mu 下：读侧（/metrics）不应与写路径抢同一把锁。
	hits   atomic.Int64
	misses atomic.Int64
}

// loadCall 表示一次正在进行的加载。等待者阻塞在 done 上，
// 加载完成后（无论成败）读 val / err。
type loadCall[T any] struct {
	done chan struct{}
	val  T
	err  error
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

// getOrLoad 命中即返回；未命中则加载，且**并发未命中只加载一次**（singleflight）。
//
// 这是 `get` + `load` + `put` 的合并版，用来替换调用方手写的
// 「先 get、miss 再算、算完 put」—— 那种写法在 TTL 边界会把同一份
// 昂贵计算并发跑 N 遍（缓存击穿）。收敛到这里后，惊群被压成一次。
//
// 语义与分别调用 `get`/`put` 完全一致：
//   - `ttl <= 0` 每次都加载（关闭缓存的表达方式不变）；
//   - 加载**失败不缓存**：等待者拿到同一个 error，下一个请求会重试，
//     不会把一次失败固化成 15~30 秒的假数据。
//
// `load` 在锁外执行，绝不会持锁跑查询。
func (c *ttlCache[T]) getOrLoad(now time.Time, ttl time.Duration, load func() (T, error)) (T, error) {
	// 快路径：TTL 内命中，不碰 inflight。
	if v, ok := c.get(now, ttl); ok {
		c.hits.Add(1)
		return v, nil
	}
	// 未命中即计入 misses（只计一次）：无论是自己加载还是等别人的结果，
	// 都代表「这次请求没能直接从热缓存拿到」。
	c.misses.Add(1)
	c.mu.Lock()
	// 双检：等锁期间可能刚有别的goroutine加载完并 put。
	if c.ok && now.Sub(c.at) < ttl {
		v := c.val
		c.mu.Unlock()
		return v, nil
	}
	if c.inflight != nil {
		// 已有人在加载：等它的结果，不重复打库。
		call := c.inflight
		c.mu.Unlock()
		<-call.done
		return call.val, call.err
	}
	// 我来当这次加载的 loader。
	call := &loadCall[T]{done: make(chan struct{})}
	c.inflight = call
	c.mu.Unlock()

	call.val, call.err = load()
	if call.err == nil {
		c.put(call.val, time.Now())
	}
	c.mu.Lock()
	c.inflight = nil
	c.mu.Unlock()
	// close 之前的写（val/err/inflight=nil）对 <-done 之后可见，
	// 这是等待者能安全读 call.val/err 的 happens-before 保证。
	close(call.done)
	return call.val, call.err
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

// Stats 返回命中/未命中计数（getOrLoad 口径）。
func (c *ttlCache[T]) Stats() (hits, misses int64) {
	return c.hits.Load(), c.misses.Load()
}
