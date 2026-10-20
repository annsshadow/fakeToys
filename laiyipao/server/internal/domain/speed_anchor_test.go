package domain

// 速率锚点（D3，2026-10-20，DECISIONS D3）。
//
// 背景：已知边界第 5 条 —— 速率裁剪锚点曾是固定 `ScoreFullAtSec = 60`，
// 而实际通关 146~594s，于是**裁剪从不生效**（「通关速度不进分数」）。
// 现在锚点按关卡推导：`理论最短通关时长 × SpeedAnchorPermille`。
//
// 三条判据：
//
//  1. 锚点随关卡内容缩放（高关卡刷怪排布更长 → 锚点更大）；
//  2. 锚点 = 下界 × 18 倍（标定依据可核对，不是魔数）；
//  3. **零误伤**：当前默认构筑的实测通关时长（IMPLEMENTATION 记录）
//     全部在锚点之上 —— 这条是「只裁异常拖时间，不裁正常玩家」的封条。
//     若有人把 SpeedAnchorPermille 调小，这里会先红。

import "testing"

// TestSpeedAnchorScalesWithLevelContent 锚点随关卡内容缩放。
func TestSpeedAnchorScalesWithLevelContent(t *testing.T) {
	l1 := speedAnchorSec(GenerateLevel(1))
	l100 := speedAnchorSec(GenerateLevel(100))
	if l1 <= 0 || l100 <= 0 {
		t.Fatalf("锚点应为正：L1=%d L100=%d", l1, l100)
	}
	if l100 <= l1 {
		t.Errorf("第 100 关锚点 %ds 应大于第 1 关 %ds（刷怪排布更长）", l100, l1)
	}
	// ⚠️ 不查逐关单调：**波次刷怪排布本身不单调**
	// （第 3 关可能比第 1 关更短）。锚点跟内容走，内容不单调它就不单调 ——
	// 第一版写成单调断言，红在「第 3 关 216s < 第 1 关 270s」，那是误报。
}

// TestSpeedAnchorMatchesCalibration 锚点 = 下界 × SpeedAnchorPermille。
//
// 钉住标定依据：倍率不是随手取的，它来自「实测 ratio 12.8~15.1 + 19% 余量」。
// 下界本身由波次排布推导（theoreticalClearSec）。
func TestSpeedAnchorMatchesCalibration(t *testing.T) {
	for _, id := range []int{1, 50, 100} {
		gl := GenerateLevel(id)
		lower := theoreticalClearSec(gl)
		if lower <= 0 {
			t.Fatalf("第 %d 关推不出理论最短时长", id)
		}
		want := lower * SpeedAnchorPermille / 1000
		if got := speedAnchorSec(gl); got != want {
			t.Errorf("第 %d 关锚点 %ds ≠ 下界 %ds × %d‰ = %ds",
				id, got, lower, SpeedAnchorPermille, want)
		}
	}
}

// TestSpeedAnchorNeverCutsRealClearTimes 零误伤：实测通关时长全部在锚点之上。
//
// 数据来源：IMPLEMENTATION.md「平衡基准」的跨关卡实测
// （默认构筑，第 1/10/25/50/75/100 关：194/181/228/285/318/594 秒）。
// 这些是**当前内容下正常玩家的真实耗时** —— 它们若被裁，
// 说明锚点定得太小，正常玩家会被无缘无故降分。
func TestSpeedAnchorNeverCutsRealClearTimes(t *testing.T) {
	cases := []struct {
		id   int
		sec  int64
		note string
	}{
		{1, 194, "实测"},
		{10, 181, "实测"},
		{25, 228, "实测"},
		{50, 285, "实测"},
		{75, 318, "实测"},
		{100, 594, "实测"},
	}
	for _, c := range cases {
		gl := GenerateLevel(c.id)
		anchor := speedAnchorSec(gl)
		if c.sec >= anchor {
			continue // 正常：实测时长达到锚点 → 拿满分
		}
		// 被裁时给出可诊断的差量
		cut := gl.MaxScore * c.sec / anchor
		t.Errorf("第 %d 关实测 %ds 被裁到 %d/%d 分（锚点 %ds）—— "+
			"锚点小于真实通关时长，正常玩家被降分；"+
			"调大 SpeedAnchorPermille（当前 %d‰）",
			c.id, c.sec, cut, gl.MaxScore, anchor, SpeedAnchorPermille)
	}
}

// TestSpeedCapLinearBelowAnchor 锚点以下线性裁剪、锚点处满分。
//
// 与 battle_validate_test.go 的 TestScoreCapNeverAllowsInstantFull 互补：
// 那条守「短时长拿不到满分」，这条守「线性关系与端点」。
func TestSpeedCapLinearBelowAnchor(t *testing.T) {
	gl := GenerateLevel(50)
	anchor := speedAnchorSec(gl)
	full := gl.MaxScore

	half := scoreCapFor(gl, anchor/2)
	if want := full * (anchor / 2) / anchor; half != want {
		t.Errorf("锚点一半（%ds）的额度应为 %d，实际 %d", anchor/2, want, half)
	}
	if cap := scoreCapFor(gl, anchor); cap != full {
		t.Errorf("锚点处（%ds）应为满分 %d，实际 %d", anchor, full, cap)
	}
	if cap := scoreCapFor(gl, anchor+1000); cap != full {
		t.Errorf("超过锚点仍应为满分 %d，实际 %d", full, cap)
	}
}
