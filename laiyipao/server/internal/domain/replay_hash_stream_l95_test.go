// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package domain

// 第 95 轮的契约守卫：`ReplayHash` 的事件流从「拼接整串再哈希」改为
// 「边遍历边写哈希」后，**字节序列必须逐字节不变**。
//
// 这条测试的意义不在「新函数能跑」，而在钉住**旧实现**：
// 任何「看起来等价」的改动（少写一个 `;`、事件前不写分隔符、
// 前缀与事件之间多一个分隔符）都会让全量历史战报的 `replay_hash` 失配，
// 而症状是「所有战报都被判伪造」——完全看不出是这里。

import (
	"strings"
	"testing"
)

// replayHashOld 是第 95 轮之前的实现，逐字复刻，作为比对基准。
//
// ⚠️ 不要"顺手优化"它：它存在的唯一理由就是保留旧字节序列。
func replayHashOld(prefix string, events []ReplayEvent) string {
	parts := make([]string, 0, len(events)+1)
	parts = append(parts, prefix)
	for _, r := range events {
		parts = append(parts, r.String())
	}
	return hex16(fnv1a64(strings.Join(parts, ";")))
}

func l95Events(n int) []ReplayEvent {
	out := make([]ReplayEvent, 0, n)
	for i := 0; i < n; i++ {
		out = append(out, ReplayEvent{
			// 负数与 0 都覆盖：FormatInt 对负号的处理是哈希的一部分。
			T:    int64(i) - 3,
			Type: []string{"hit", "leak", "card_taken", ""}[i%4],
			A:    int64(i * 7 % 11),
			B:    int64(-i),
			C:    int64(i % 5),
		})
	}
	return out
}

// TestHashReplayEventsMatchesJoinImplementation 是新旧实现逐字节一致的守卫。
func TestHashReplayEventsMatchesJoinImplementation(t *testing.T) {
	prefixes := []string{
		"",
		"L1|A100.50.1500.1000.3.1.1000|S0:1:100:1:20|X0",
		"L7|A0.50.1500.1000.3.1.1000|S|X42",
	}
	for _, prefix := range prefixes {
		for _, n := range []int{0, 1, 2, 5, 9, 41} {
			events := l95Events(n)
			got := hex16(hashReplayEvents(prefix, events))
			want := replayHashOld(prefix, events)
			if got != want {
				t.Fatalf("事件数 %d、前缀 %q：流式哈希 %s != 旧实现 %s —— "+
					"字节序列变了，会让历史战报全部失配", n, prefix, got, want)
			}
		}
	}
}

// TestHashReplayEventsUsesSameSeparator 单独钉住「前缀 + 每条事件前一个 `;`」的形状。
//
// 与上一条不同：这条不依赖旧实现，直接按公式算期望值，
// 因此即使有人把旧实现一起改掉，它仍会红。
func TestHashReplayEventsUsesSameSeparator(t *testing.T) {
	events := []ReplayEvent{
		{T: 1, Type: "hit", A: 2, B: 3, C: 4},
		{T: 2, Type: "leak", A: 0, B: 0, C: 5},
	}
	want := fnv1a64("PFX;1|hit|2|3|4;2|leak|0|0|5")
	if got := hashReplayEvents("PFX", events); got != want {
		t.Fatalf("哈希 = %016x，期望 %016x（前缀后无 `;`，每条事件前一个 `;`）",
			got, want)
	}
}

func BenchmarkReplayHashJoin(b *testing.B) {
	events := l95Events(3000)
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		_ = replayHashOld("L1|A1.2.3", events)
	}
}

func BenchmarkReplayHashStream(b *testing.B) {
	events := l95Events(3000)
	prefix := "L1|A1.2.3"
	b.ReportAllocs()
	for i := 0; i < b.N; i++ {
		_ = hex16(hashReplayEvents(prefix, events))
	}
}
