// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package domain

// R96 P3：重放事件流的对外读取（此前 0% 覆盖）。
//
// `ReplayEvents` 的契约是「返回**副本**」：调用方（校验/导出/客户端）
// 会拿着它做哈希与比对。如果实现直接返回内部切片，任何调用方
// 一次排序/改写就会污染引擎后续记录，表现为「同一局重放哈希突然不符」——
// 而症状完全不指向这里。断言就钉这条：改返回值不得影响引擎。
//
// 同文件顺带把 `GemQualityAffixCount` 的查表契约钉住：未知品质回退到 1，
// 而不是 0 或 panic —— 0 会让「宝石词条为空」这种配置错误静默通过。
//
// ⚠️ 故意不测 `ReplaySegmentA`：它是 A 阶段留下的**空实现骨架**
// （`return ""`），没有任何调用方。给它写断言等于把「暂时不实现」固化成语义，
// 反而会掩盖真正的做法（补齐 G 段或删掉死代码）。这条记在台账里。

import "testing"

func TestBattleEngineReplayEventsAreCopyAndCounted(t *testing.T) {
	e := &BattleEngine{}

	// 空局：必须是空且非 nil —— JSON 里 null 会让客户端 `events.map()` 直接崩。
	events := e.ReplayEvents()
	if events == nil {
		t.Fatal("空局应返回非 nil 的空切片，实际 nil")
	}
	if len(events) != 0 {
		t.Fatalf("空局不该有事件，实际 %d 条", len(events))
	}
	if got := e.ReplayEventCount(); got != 0 {
		t.Fatalf("空局计数应为 0，实际 %d", got)
	}

	e.record(3, "hit", 11, 22, 0)
	e.record(4, "skill", 7, 0, 0)

	if got := e.ReplayEventCount(); got != 2 {
		t.Fatalf("记两条事件后计数应为 2，实际 %d", got)
	}
	events = e.ReplayEvents()
	if len(events) != 2 {
		t.Fatalf("读取条数应与写入一致：2，实际 %d", len(events))
	}
	// 字段原样带出（顺序、时间戳与操作数都是哈希输入）。
	if events[0].T != 3 || events[0].Type != "hit" || events[0].A != 11 || events[0].B != 22 {
		t.Errorf("第 1 条事件字段不符：%+v", events[0])
	}
	if events[1].T != 4 || events[1].Type != "skill" || events[1].A != 7 {
		t.Errorf("第 2 条事件字段不符：%+v", events[1])
	}

	// 契约核心：返回的是副本。改返回值不得影响引擎内部流。
	events[0].Type = "tampered"
	events[0].A = 999
	if got := e.ReplayEvents()[0]; got.Type != "hit" || got.A != 11 {
		t.Errorf("ReplayEvents 必须返回副本：调用方改写返回值污染了引擎内部事件流（%+v）", got)
	}

	// 追加一条后，之前拿到的副本不该跟着变长。
	before := len(events)
	e.record(5, "wave", 0, 0, 0)
	if len(events) != before {
		t.Errorf("已发出的副本不应被后续写入影响：期望 %d 条，实际 %d", before, len(events))
	}
	if got := e.ReplayEventCount(); got != 3 {
		t.Errorf("追加后计数应为 3，实际 %d", got)
	}
}

func TestGemQualityAffixCountFollowsTableAndFallsBack(t *testing.T) {
	if len(GemQualities) == 0 {
		t.Fatal("品质表为空 —— 种子/内容表没加载")
	}
	for _, q := range GemQualities {
		if got := GemQualityAffixCount(q.Name); got != q.Count {
			t.Errorf("品质 %q 的词条数应为表中 %d，实际 %d", q.Name, q.Count, got)
		}
	}
	// 未知品质回退 1（而不是 0）：0 会让「词条为空」的配置错误静默通过。
	for _, unknown := range []string{"", "unknown", "Rare", "史诗", "\x00"} {
		if got := GemQualityAffixCount(unknown); got != 1 {
			t.Errorf("未知品质 %q 应回退 1 条词条，实际 %d", unknown, got)
		}
	}
}
