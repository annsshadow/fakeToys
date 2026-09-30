package domain

import (
	"testing"
)

// 暴击率的**分布**测试（与 miniapp/src/game/crit.test.ts 配对）。
//
// ⚠️ 补的是一个真实事故：Roll 曾经是万分比（0..9999）而判定阈值
// `roll >= 1000 - critPermille` 是千分比，两个量纲混用。
// 于是 CritPermille=50（标称 5%）实际产生 90.5% 暴击率。
//
// 为什么端点断言抓不到：既有测试只测 CritPermille=1000（必暴）与
// 0（必不暴），而这两个端点在 10× 错误下**都成立**。
// 只有统计能区分"阈值对"和"阈值差一个数量级"。
//
// 危害不只是数字错：所有取值都落在 90%+ 区间，于是"提高暴击率"
// 这个成长维度的边际收益被压平 —— 实测第 1 关 critPermille 从 0
// 提到 1000（满暴），分数只从 15542 变到 15545（+0.02%）。

// critAt 在给定 roll 上跑一次判定，返回是否暴击。
//
// HP 给得很大（1e6）让这次判定不杀不死：击杀会改变护盾扣除路径，
// 而这里只关心暴击这一个布尔。守方无元素层数、无抗性、无护甲。
func critAt(roll int64, critPermille int64) bool {
	def := &Defender{HP: 1_000_000, Shield: 0, ArmorPermille: 0}
	res := ResolveHit(Attacker{
		Attack:                 100,
		CritPermille:           critPermille,
		CritMultiplierPermille: 1500,
		ReactionMultPermille:   1000,
		ElementCap:             3,
		ReactionTier:           1,
		ElementCoefPermille:    1000,
	}, def, HitInput{
		SkillDamage:   100,
		SkillElement:  ElementFire,
		ForceReaction: "",
		Roll:          roll,
	})
	return res.Crit
}

func TestCritThresholdEndpoints(t *testing.T) {
	// 满暴：所有 roll 都触发
	for r := int64(0); r < 1000; r += 37 {
		if !critAt(r, 1000) {
			t.Errorf("CritPermille=1000 时 roll=%d 应必暴", r)
		}
	}
	// 零暴：没有 roll 会触发
	for r := int64(0); r < 1000; r += 37 {
		if critAt(r, 0) {
			t.Errorf("CritPermille=0 时 roll=%d 不应暴击", r)
		}
	}
	// 半暴：阈值 500
	for r := int64(0); r < 500; r += 41 {
		if critAt(r, 500) {
			t.Errorf("CritPermille=500 时 roll=%d 不应暴击", r)
		}
	}
	for r := int64(500); r < 1000; r += 41 {
		if !critAt(r, 500) {
			t.Errorf("CritPermille=500 时 roll=%d 应暴击", r)
		}
	}
}

func TestCritRateDistribution(t *testing.T) {
	// 穷举 1000 个 roll，统计实际暴击次数。
	// 期望：实际暴击率 ≈ CritPermille/1000，误差 < 1.5 个百分点。
	//
	// 修复前（roll 0..9999 而阈值按千分比算）：
	//   CritPermille=50  -> 实际 90.5%（标称 5%）
	//   CritPermille=1000 -> 实际 100%（标称 100%）  <- 端点仍然正确！
	// 正是"端点对、中间全错"的形状。
	cases := []struct {
		permille int64
		nominal  float64
	}{
		{50, 0.05}, {130, 0.13}, {250, 0.25},
		{500, 0.50}, {750, 0.75},
	}
	for _, c := range cases {
		crit := 0
		for r := int64(0); r < 1000; r++ {
			if critAt(r, c.permille) {
				crit++
			}
		}
		actual := float64(crit) / 1000
		if diff := actual - c.nominal; diff > 0.015 || diff < -0.015 {
			t.Errorf("CritPermille=%d 标称暴击率 %.1f%%，实际 %.1f%%",
				c.permille, c.nominal*100, actual*100)
		}
	}
}

func TestCritRateIsMonotonic(t *testing.T) {
	// 修复前所有取值都落在 90%+，这条会在中途变成"几乎不增长"而失败。
	prev := -1.0
	first, last := 0.0, 0.0
	for _, p := range []int64{0, 100, 200, 400, 600, 800, 1000} {
		crit := 0
		for r := int64(0); r < 1000; r++ {
			if critAt(r, p) {
				crit++
			}
		}
		rate := float64(crit) / 1000
		if rate <= prev {
			t.Errorf("CritPermille=%d 的暴击率 %.1f%% 未高于上一档", p, rate*100)
		}
		if p == 0 {
			first = rate
		}
		if p == 1000 {
			last = rate
		}
		prev = rate
	}
	if first != 0 {
		t.Errorf("CritPermille=0 的实际暴击率应为 0%%，实际 %.1f%%", first*100)
	}
	if last != 1 {
		t.Errorf("CritPermille=1000 的实际暴击率应为 100%%，实际 %.1f%%", last*100)
	}
}
