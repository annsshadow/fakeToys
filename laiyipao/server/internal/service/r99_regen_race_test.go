// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// r99_regen_race_test.go — R99 故障注入：并发 LoadWallet 的体力恢复竞态。
//
// 复现条件（生产可发生）：玩家离线 >1 格（6 分钟）后回到 App，
// 首页/商城/任务多个页面同时拉 fetchWallet → 同一 user 的
// LoadWallet 并发进入。regen 那条 SQL 用 `WITH cur AS (...)`
// 先按语句快照算 n，再 `UPDATE ... FROM cur`；并发时第二个事务
// 在行锁上等待，EPQ 用**新行**的 energy/energy_updated_at，
// 但 `cur.n` 仍是旧快照 —— 于是同一段离线时间被重复计入：
// 体力多加 n，且 energy_updated_at 被推进到**未来**（之后不再恢复）。
//
// 判据（会红的两个事实，缺一不可）：
//   1) 体力必须恰好等于经过的格数（20 并发不得放大）；
//   2) energy_updated_at 不得晚于 now（否则玩家白丢一段恢复时间）。

import (
	"context"
	"sync"
	"testing"
)

func TestR99EnergyRegenConcurrentNoDoubleCount(t *testing.T) {
	ts := openScratchService(t)
	ctx := context.Background()
	uid := ts.newUser(t, ctx)

	// 0 体力、已离线 720s（每格 360s）→ 应恢复 2 点，就 2 点。
	//
	// 单轮的并发重叠有调度随机性，故跑 3 轮（每轮都从同一起点重置）：
	// 破坏实验里只要有一轮被重复计入就红；修复后任何一轮都必须恰好 2。
	const rounds = 3
	const conc = 20
	for round := 0; round < rounds; round++ {
		if _, err := ts.pool.Exec(ctx,
			`UPDATE user_wallets
			    SET energy = 0,
			        energy_updated_at = now() - make_interval(secs => 720.0)
			  WHERE user_id = $1`, uid); err != nil {
			t.Fatalf("第 %d 轮布置起点失败：%v", round+1, err)
		}

		start := make(chan struct{})
		errCh := make(chan error, conc)
		var wg sync.WaitGroup
		for i := 0; i < conc; i++ {
			wg.Add(1)
			go func() {
				defer wg.Done()
				<-start
				if _, err := ts.LoadWallet(ctx, uid); err != nil {
					errCh <- err
				}
			}()
		}
		close(start)
		wg.Wait()
		close(errCh)
		for err := range errCh {
			t.Fatalf("第 %d 轮并发 LoadWallet 失败：%v", round+1, err)
		}

		w := ts.wallet(t, ctx, uid)
		if w.Energy != 2 {
			t.Fatalf("第 %d 轮：离线 720s（2 格）、%d 并发 LoadWallet 后体力 = %d，应为 2；"+
				"多出的 %d 点是同一段离线时间被重复计入（错误数据）",
				round+1, conc, w.Energy, w.Energy-2)
		}

		var secondsAhead float64
		if err := ts.pool.QueryRow(ctx,
			`SELECT EXTRACT(EPOCH FROM (energy_updated_at - now()))::float8
			   FROM user_wallets WHERE user_id = $1`, uid).Scan(&secondsAhead); err != nil {
			t.Fatalf("第 %d 轮读 energy_updated_at 失败：%v", round+1, err)
		}
		// 允许毫秒级时钟抖动；真实缺陷是 +720s 量级的「未来」。
		if secondsAhead > 1.0 {
			t.Fatalf("第 %d 轮：energy_updated_at 比 now 还晚 %.0fs —— 恢复时间轴被重复推进，"+
				"玩家会在这段时间内一点体力都恢复不了", round+1, secondsAhead)
		}

		// 竞态修好后，紧接着再读一次不得再涨（余数/时间轴一致）。
		if _, err := ts.LoadWallet(ctx, uid); err != nil {
			t.Fatalf("第 %d 轮二次 LoadWallet 失败：%v", round+1, err)
		}
		if e := ts.wallet(t, ctx, uid).Energy; e != 2 {
			t.Fatalf("第 %d 轮紧接着再读一次体力变成 %d（应为 2）——余数被重复计入", round+1, e)
		}
	}
}
