// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package domain

// R66：未知地形告警的去重 map 是包级共享状态，`Terrain.Update` 在并发结算
// 的 goroutine 里跑。这个用例在 `-race` 下并发命中同一个未知类型，
// 若同步退化回「先读后写」，会报 data race（真实并发下则是 fatal map 读写）。

import (
	"sync"
	"testing"
)

func TestWarnUnknownKindConcurrent(t *testing.T) {
	const kind = "r66_probe_unknown"
	var wg sync.WaitGroup
	for i := 0; i < 64; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			warnUnknownKind(kind)
		}()
	}
	wg.Wait()

	unknownKindMu.Lock()
	got := unknownKindWarned[kind]
	unknownKindMu.Unlock()
	if !got {
		t.Fatal("并发告警后 map 未记录该类型")
	}
}
