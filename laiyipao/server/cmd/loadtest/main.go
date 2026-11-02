// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// loadtest 是一个最小闭环压测器：固定并发、固定时长、只测单个 URL。
//
// 为什么自带而不是用 wrk/hey：本仓的基线需要「别人照着能跑出接近的数」，
// 而外部工具的安装、参数语义、统计口径各不相同。这里只保留四个指标
// （QPS / P95 / P99 / 错误率）并显式打印并发与时长，口径固定。
//
// 注意：这是**闭环**压测（每个 worker 等上一个响应再发下一个）。
// 它衡量的是「在给定并发下的服务吞吐」，不是开环到达率模型；
// 后者需要固定 RPS 发生器，不在本工具范围内。
//
// 用法：
//
//	go run ./cmd/loadtest -url http://127.0.0.1:8080/api/v1/config -c 20 -d 20s
//	go run ./cmd/loadtest -url http://127.0.0.1:8080/api/v1/me/battles -token "$TOKEN"
package main

import (
	"context"
	"flag"
	"fmt"
	"io"
	"net/http"
	"os"
	"sort"
	"sync"
	"time"
)

func main() {
	url := flag.String("url", "", "目标 URL（必填）")
	token := flag.String("token", "", "可选：Bearer 令牌")
	conc := flag.Int("c", 20, "并发 worker 数")
	dur := flag.Duration("d", 20*time.Second, "压测时长（不含预热）")
	warmup := flag.Duration("w", 3*time.Second, "预热时长，统计前丢弃")
	label := flag.String("label", "", "指标前缀，默认用 URL 路径")
	flag.Parse()

	if *url == "" || *conc <= 0 || *dur <= 0 {
		fmt.Fprintln(os.Stderr, "用法: -url <URL> [-token T] [-c 20] [-d 20s] [-w 3s]")
		os.Exit(2)
	}
	if *label == "" {
		*label = *url
	}

	client := &http.Client{
		Transport: &http.Transport{
			MaxIdleConns:        *conc * 2,
			MaxIdleConnsPerHost: *conc,
			IdleConnTimeout:     30 * time.Second,
			DisableCompression:  true,
		},
		// 服务端最长请求耗时应远小于它；超时按错误计入，避免无限挂住。
		Timeout: 60 * time.Second,
	}

	req, err := http.NewRequest(http.MethodGet, *url, nil)
	if err != nil {
		fmt.Fprintln(os.Stderr, "构造请求失败:", err)
		os.Exit(1)
	}
	if *token != "" {
		req.Header.Set("Authorization", "Bearer "+*token)
	}

	// 预热：连接池建连、TLS 握手、服务器 JIT/缓存都在这段时间里就位。
	ctx, cancel := context.WithTimeout(context.Background(), *warmup)
	run(ctx, client, req, *conc)
	cancel()

	ctx, cancel = context.WithTimeout(context.Background(), *dur)
	start := time.Now()
	stats := run(ctx, client, req, *conc)
	elapsed := time.Since(start)
	cancel()

	stats.report(*label, *conc, elapsed)
	if stats.errs > 0 {
		os.Exit(1)
	}
}

type result struct {
	lats   []time.Duration // 成功响应的耗时（仅 2xx）
	status map[int]int
	errs   int // 网络错误 + 非 2xx
	total  int
}

func run(ctx context.Context, c *http.Client, req *http.Request, conc int) result {
	var (
		mu sync.Mutex
		wg sync.WaitGroup
	)
	out := result{status: map[int]int{}}
	for i := 0; i < conc; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			var local result
			local.status = map[int]int{}
			for ctx.Err() == nil {
				start := time.Now()
				resp, err := c.Do(req)
				if err != nil {
					if ctx.Err() != nil {
						break
					}
					local.errs++
					local.total++
					continue
				}
				// 必须读完 body 才能复用连接；内容本身不校验（压的是服务端路径）。
				_, _ = io.Copy(io.Discard, resp.Body)
				_ = resp.Body.Close()
				local.total++
				local.status[resp.StatusCode]++
				if resp.StatusCode < 200 || resp.StatusCode >= 300 {
					local.errs++
					continue
				}
				local.lats = append(local.lats, time.Since(start))
			}
			mu.Lock()
			out.total += local.total
			out.errs += local.errs
			out.lats = append(out.lats, local.lats...)
			for k, v := range local.status {
				out.status[k] += v
			}
			mu.Unlock()
		}()
	}
	wg.Wait()
	return out
}

func (r result) report(label string, conc int, elapsed time.Duration) {
	sort.Slice(r.lats, func(i, j int) bool { return r.lats[i] < r.lats[j] })
	qps := 0.0
	if elapsed > 0 {
		qps = float64(r.total) / elapsed.Seconds()
	}
	errRate := 0.0
	if r.total > 0 {
		errRate = float64(r.errs) / float64(r.total) * 100
	}
	fmt.Printf("label=%s c=%d duration=%s total=%d qps=%.1f err_rate=%.2f%% p50=%s p95=%s p99=%s max=%s\n",
		label, conc, elapsed.Round(time.Millisecond), r.total, qps, errRate,
		pct(r.lats, 0.50), pct(r.lats, 0.95), pct(r.lats, 0.99), pct(r.lats, 1.0))
	for code, n := range r.status {
		fmt.Printf("  status %d=%d\n", code, n)
	}
}

func pct(lats []time.Duration, p float64) time.Duration {
	if len(lats) == 0 {
		return 0
	}
	i := int(p * float64(len(lats)-1))
	if i < 0 {
		i = 0
	}
	if i >= len(lats) {
		i = len(lats) - 1
	}
	return lats[i].Round(time.Microsecond)
}
