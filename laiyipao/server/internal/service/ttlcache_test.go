package service

// 进程内缓存的纯逻辑守卫（2026-10-23）。
//
// 这一组**不需要数据库**，所以它是本轮唯一能在无 PG 环境下真正执行的
// 缓存判据 —— 其余（看板命中、榜单命中）都要查库，本机只能 Skip。
//
// 为什么值得单独测：缓存的失效方式全是静默的 ——
//   · TTL 判断写反 → 永远不命中（性能问题，没人报错）
//   · 过期不失效   → 永远命中（运营看到的是启动时的快照）
//   · `ttl<=0` 不短路 → 「关闭缓存」这个开关变成摆设

import (
	"errors"
	"sync"
	"sync/atomic"
	"testing"
	"time"
)

func TestTTLCacheHitWithinTTL(t *testing.T) {
	var c ttlCache[int]
	now := time.Now()
	if _, ok := c.get(now, time.Second); ok {
		t.Fatal("空缓存不该命中")
	}
	c.put(42, now)
	if v, ok := c.get(now.Add(500*time.Millisecond), time.Second); !ok || v != 42 {
		t.Fatalf("TTL 内应命中 42，实际 v=%d ok=%v", v, ok)
	}
}

func TestTTLCacheExpires(t *testing.T) {
	var c ttlCache[string]
	now := time.Now()
	c.put("x", now)
	// 恰好等于 TTL 也算过期（`>=`）—— 边界取「不命中」是刻意的：
	// 取「命中」会让 TTL 实际变成 TTL+1 个时钟粒度，且难以解释
	if _, ok := c.get(now.Add(time.Second), time.Second); ok {
		t.Fatal("到达 TTL 应视为过期")
	}
}

func TestTTLCacheDisabledWhenTTLIsZeroOrNegative(t *testing.T) {
	for _, ttl := range []time.Duration{0, -time.Second} {
		var c ttlCache[int]
		now := time.Now()
		c.put(7, now)
		if _, ok := c.get(now, ttl); ok {
			t.Fatalf("ttl=%v 应恒不命中（关闭缓存），实际命中", ttl)
		}
	}
}

func TestTTLCacheFlush(t *testing.T) {
	var c ttlCache[int]
	now := time.Now()
	c.put(1, now)
	c.flush()
	if _, ok := c.get(now, time.Minute); ok {
		t.Fatal("flush 后不该命中")
	}
	// flush 后再 put 应恢复正常
	c.put(2, now)
	if v, ok := c.get(now, time.Minute); !ok || v != 2 {
		t.Fatalf("flush 后重新 put 应命中 2，实际 v=%d ok=%v", v, ok)
	}
}

// TestSliceLeaderboard 截断不越界，且 **cap 必须等于 len**。
//
// # 为什么 cap 也要断言
//
// `all[:limit]` 的 cap 仍是底层数组的长度。调用方一旦 `append`，
// 就会写进缓存那份底层数组的第 limit+1 个位置 —— 榜单内容随机变化，
// 且没有任何报错。`ttlcache` 的「调用方必须只读」是口头契约，编译器不检查；
// 三索引切片 `all[:limit:limit]` 把它变成结构约束。
//
// 这条断言的作用就是：谁把三索引改回两索引，这里立刻红。
func TestSliceLeaderboard(t *testing.T) {
	all := make([]LeaderboardEntry, 3, 200) // 模拟缓存那份「cap=200」的切片
	for i := range all {
		all[i].UserID = int64(i + 1)
	}

	got := sliceLeaderboard(all, 2)
	if len(got) != 2 || got[1].UserID != 2 {
		t.Fatalf("limit=2 应取前两条，实际 %+v", got)
	}
	if cap(got) != 2 {
		t.Fatalf("cap 必须被夹到 2（否则 append 会写进缓存底层数组），实际 %d", cap(got))
	}

	// limit 大于长度：原样返回（不 panic、不补零），cap 同样收紧
	all2 := sliceLeaderboard(all, 50)
	if len(all2) != 3 {
		t.Fatalf("limit=50 应返回全部 3 条，实际 %d 条", len(all2))
	}
	if cap(all2) != 3 {
		t.Fatalf("limit 超长时 cap 也必须等于 len，实际 %d", cap(all2))
	}

	// 实证：对返回值 append 不得污染原切片
	_ = append(got, LeaderboardEntry{UserID: 999})
	if all[2].UserID != 3 {
		t.Fatalf("append 污染了底层数组：all[2].UserID=%d", all[2].UserID)
	}
}

// TestTTLCacheConcurrentAccess 并发读写下不 panic、不撕裂。
//
// # 为什么要它
//
// `get` 返回的是切片头（3 个字），`put` 会整体替换 `c.val`。若把 `mu.Lock`
// 删掉或把 `put` 改成「原地改元素」，读方可能拿到撕裂的切片头 ——
// 这类失效**只能靠 `-race` 发现**，而单 goroutine 用例永远触发不了它。
//
// 断言刻意只要求「不 panic + 结构自洽」：并发下读到的值是新是旧都合法，
// 真正的判据是 `go test -race` 不报数据竞争。
func TestTTLCacheConcurrentAccess(t *testing.T) {
	var c ttlCache[[]int]
	var writer, readers sync.WaitGroup
	stop := make(chan struct{})

	writer.Add(1)
	go func() {
		defer writer.Done()
		for i := 0; i < 2000; i++ {
			c.put([]int{i, i + 1, i + 2}, time.Now())
			if i%100 == 0 {
				c.flush()
			}
		}
	}()

	// ⚠️ 写者与读者用**两个** WaitGroup：写者结束时关 stop 让读者退出。
	// 合成一个会死锁（读者等 stop，主协程等 readers）。
	for r := 0; r < 4; r++ {
		readers.Add(1)
		go func() {
			defer readers.Done()
			for {
				select {
				case <-stop:
					return
				default:
				}
				if v, ok := c.get(time.Now(), time.Hour); ok {
					// 长度必须恒为 3：读到撕裂的切片头时这里会崩
					if len(v) != 3 {
						t.Errorf("读到长度 %d 的切片，期望 3", len(v))
						return
					}
				}
			}
		}()
	}

	writer.Wait()
	close(stop)
	readers.Wait()
}

// TestTTLCacheGetOrLoadSingleflight：并发未命中只加载一次（缓存击穿防护）。
//
// # 判据为什么是「loader 调用次数 == 1」
//
// 若把 `getOrLoad` 改回「各自 get / 各自 load / 各自 put」的旧形状，
// TTL 到期那一刻 N 个并发请求会各跑一遍 loader —— 看板是 9 段查询、
// 榜单是 3 次 GROUP BY。这里的 loader 用一个原子计数器点名跑了几遍。
//
// loader 故意阻塞在 release 上：在它放行前，没有任何请求能靠缓存完成，
// 所以「调用次数」与 goroutine 调度时序无关，恒为 1 才是对的值。
func TestTTLCacheGetOrLoadSingleflight(t *testing.T) {
	var c ttlCache[int]
	var loads int32
	release := make(chan struct{})
	load := func() (int, error) {
		atomic.AddInt32(&loads, 1)
		<-release // 阻塞，逼所有并发方pile up成等待者
		return 7, nil
	}

	const n = 8
	results := make([]int, n)
	errs := make([]error, n)
	var wg sync.WaitGroup
	for i := 0; i < n; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			results[i], errs[i] = c.getOrLoad(time.Now(), time.Minute, load)
		}(i)
	}
	// 给足时间让所有 goroutine 进入 getOrLoad（loader 阻塞中，它们必然
	// 停在锁段内：要么当上 loader 阻塞在 load()，要么成为等待者阻塞在 <-done）。
	time.Sleep(50 * time.Millisecond)
	close(release)
	wg.Wait()

	if got := atomic.LoadInt32(&loads); got != 1 {
		t.Fatalf("singleflight 失效：loader 被并发调用了 %d 次，期望 1 次", got)
	}
	for i := 0; i < n; i++ {
		if errs[i] != nil || results[i] != 7 {
			t.Fatalf("第 %d 个调用方应拿到 v=7 err=nil，实际 v=%d err=%v", i, results[i], errs[i])
		}
	}
}

// TestTTLCacheGetOrLoadErrorNotCached：失败不缓存，成功才缓存。
//
// 与 AdminDashboard 的「错误不缓存」注释对应：一次失败若被固化进缓存，
// 运营会连续看到同一条报错，而数据库其实已经好了。
func TestTTLCacheGetOrLoadErrorNotCached(t *testing.T) {
	var c ttlCache[int]
	var loads int32
	boom := errors.New("boom")

	if _, err := c.getOrLoad(time.Now(), time.Minute, func() (int, error) {
		atomic.AddInt32(&loads, 1)
		return 0, boom
	}); !errors.Is(err, boom) {
		t.Fatalf("应把 loader 的错误原样返回，实际 %v", err)
	}
	// 失败未被缓存：第二次调用重新走 loader 并成功
	v, err := c.getOrLoad(time.Now(), time.Minute, func() (int, error) {
		atomic.AddInt32(&loads, 1)
		return 5, nil
	})
	if err != nil || v != 5 {
		t.Fatalf("失败后应重试并成功，实际 v=%d err=%v", v, err)
	}
	if got := atomic.LoadInt32(&loads); got != 2 {
		t.Fatalf("失败不应被缓存：loader 应已被调用 2 次，实际 %d", got)
	}
	// 成功已被缓存：第三次调用命中，不再跑 loader
	v, err = c.getOrLoad(time.Now(), time.Minute, func() (int, error) {
		atomic.AddInt32(&loads, 1)
		return 999, nil
	})
	if err != nil || v != 5 {
		t.Fatalf("成功后应命中缓存值 5，实际 v=%d err=%v", v, err)
	}
	if got := atomic.LoadInt32(&loads); got != 2 {
		t.Fatalf("成功应被缓存：loader 调用次数应仍为 2，实际 %d", got)
	}
}

// TestTTLCacheGetOrLoadCountsHitsMisses：命中/未命中计数（/metrics 的数据源）。
//
// 缓存命中率是这两个缓存**唯一**的健康信号，且它的失效全是静默的
// （TTL 设太小 → 命中率掉到 0，没有任何报错，只是变慢）。计数器错了，
// /metrics 就会把「缓存没在工作」显示成「命中率良好」—— 比没有指标更坏。
func TestTTLCacheGetOrLoadCountsHitsMisses(t *testing.T) {
	var c ttlCache[int]
	// 第一次：未命中 → 走 loader
	if _, err := c.getOrLoad(time.Now(), time.Minute, func() (int, error) { return 1, nil }); err != nil {
		t.Fatalf("加载失败: %v", err)
	}
	if h, m := c.Stats(); h != 0 || m != 1 {
		t.Fatalf("首次后应 hits=0 misses=1，实际 hits=%d misses=%d", h, m)
	}
	// 第二次：命中
	if _, err := c.getOrLoad(time.Now(), time.Minute, func() (int, error) { return 2, nil }); err != nil {
		t.Fatalf("加载失败: %v", err)
	}
	if h, m := c.Stats(); h != 1 || m != 1 {
		t.Fatalf("第二次后应 hits=1 misses=1，实际 hits=%d misses=%d", h, m)
	}
}
