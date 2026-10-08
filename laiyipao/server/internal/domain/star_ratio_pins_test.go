package domain

// 第 130 轮：钉住星级/速率锚点常量 StarTargetRatio 与 ScoreFullAtSec。
//
// 这两个常量是整个「星级门槛 + 速率裁剪」的地基：
//   - StarTargetRatio（千分比）决定一/二/三星门槛 = 理论满分 × ratio/1000；
//   - ScoreFullAtSec 是速率裁剪的锚（scoreCapFor 的除数）。
//
// 它们既是生成器（levelgen 估算）的输入，也是 /config 下发给客户端的
// 数值（DefaultScoreRules），还是结算裁剪（scoreCapFor）的除数。
// 改错任何一个都会**静默**把两端星级口径带偏，且现有测试（只查门槛
// 「递增」）抓不到——递增在 ratio 整体放大时仍然成立。
//
// 本文件把「数值本身合法、且单一事实源没漂移」钉死。

import "testing"

// TestStarTargetRatioIsMonotonicAndInPermille 钉住三档占比的**数值形状**：
// 严格递增且每档都在 (0, 1000‰] 内。
//
// 关键判据：StarTargetRatio[2] ≤ permille —— 3 星门槛 = 理论满分 × 980‰
// 才「可达」。一旦有人把它调到 >1000（比如误用万分比 9800、或 1050），
// 3 星门槛就**高于**理论满分、数学上永远拿不到，且「递增」仍成立 → 现有
// 门槛递增测试全绿，只有这里会红。
func TestStarTargetRatioIsMonotonicAndInPermille(t *testing.T) {
	r := StarTargetRatio
	if !(r[0] > 0) {
		t.Errorf("一星占比 %d 必须为正（0/负 = 空门槛）", r[0])
	}
	if !(r[0] < r[1] && r[1] < r[2]) {
		t.Errorf("三档占比必须严格递增，实际 %v", r)
	}
	for i, v := range r {
		if v > permille {
			t.Errorf("第 %d 档占比 %d‰ 超过理论满分（>1000‰）——该星在数学上不可达", i, v)
		}
	}
}

// TestScoreFullAtSecIsPositive 钉住裁剪锚必须为正：
// scoreCapFor 里 `full*sec/ScoreFullAtSec` 用它做除数，
// ≤0 会除零 panic，或让「sec>=ScoreFullAtSec」恒真、裁剪彻底失效。
func TestScoreFullAtSecIsPositive(t *testing.T) {
	if ScoreFullAtSec <= 0 {
		t.Fatalf("ScoreFullAtSec = %d，必须为正（是 scoreCapFor 的除数）", ScoreFullAtSec)
	}
}

// TestDefaultScoreRulesIsSingleSourceOfTruth 钉住「下发值 == 生成器用的常量」。
// DefaultScoreRules 是唯一出口（/config、cmd/vectors、levelgen 全走它），
// 若有人改了常量却忘了同步它（或反之），服务端下发与客户端本地星级
// 会悄悄漂移。这里把两者钉成恒等。
func TestDefaultScoreRulesIsSingleSourceOfTruth(t *testing.T) {
	rules := DefaultScoreRules()
	if rules.StarTargetRatio != StarTargetRatio {
		t.Errorf("下发的 StarTargetRatio %v ≠ 生成器常量 %v（两端星级口径漂移）",
			rules.StarTargetRatio, StarTargetRatio)
	}
	if rules.ScoreFullAtSec != ScoreFullAtSec {
		t.Errorf("下发的 ScoreFullAtSec %d ≠ 生成器常量 %d（裁剪锚漂移）",
			rules.ScoreFullAtSec, ScoreFullAtSec)
	}
}

// TestThreeStarIsReachableOnRealLevels 把「3 星门槛 ≤ 理论满分」落到**真实关卡**上：
// 对生成器输出的关卡断言 StarTargets[2] ≤ MaxScore（MaxScore>0 时）。
// 常量正确但某条生成路径漏乘 ratio、或 MaxScore 算错，都会在这里露出。
func TestThreeStarIsReachableOnRealLevels(t *testing.T) {
	// 抽样覆盖首/中/末 + 全章边界，不逐关跑（100 关已有单调测试守着）。
	for _, id := range []int{1, 25, 50, 75, 100} {
		gl := GenerateLevel(id)
		if gl.MaxScore <= 0 {
			t.Errorf("关卡 %d 的 MaxScore=%d，应为正（满分基准）", id, gl.MaxScore)
			continue
		}
		if len(gl.StarTargets) != 3 {
			t.Errorf("关卡 %d 应有 3 档门槛，实际 %d", id, len(gl.StarTargets))
			continue
		}
		if gl.StarTargets[2] > gl.MaxScore {
			t.Errorf("关卡 %d 的 3 星门槛 %d 超过理论满分 %d —— 该星不可达",
				id, gl.StarTargets[2], gl.MaxScore)
		}
		// 满分只在 ScoreFullAtSec 秒可得：验证裁剪锚与满分的对应关系
		if cap := scoreCapFor(gl, ScoreFullAtSec); cap != gl.MaxScore {
			t.Errorf("关卡 %d：满 %d 秒应放行理论满分 %d，实际裁剪为 %d",
				id, ScoreFullAtSec, gl.MaxScore, cap)
		}
	}
}
