// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package domain

import "testing"

// TestReactionSplashPaysFullQuarter 钉死 t41 的对外口径：
// 文案写「受到 25% 范围伤害」，指的是 reactionDamage 的 25%。
// 旧实现把 splash 再塞进 ResolveHit 的 SkillDamage，
// direct 段会再乘 DirectWeightPermille=400‰，实收只有 10%。
func TestReactionSplashPaysFullQuarter(t *testing.T) {
	att := Attacker{Attack: 0, CritPermille: 0, CritMultiplierPermille: 1500,
		ReactionMultPermille: 1000, ElementCap: 3, ReactionTier: 1, ElementCoefPermille: 1000}
	e := newProbeEngine()
	e.Cfg.Enemies = map[int]SeedEnemy{1: {ID: 1, HP: 100000, FlyHeight: 0}}
	origin := &Enemy{UID: 1, X: 500000, Y: 500000, HP: 100000, MaxHP: 100000,
		SpawnProgress: SpawnProgressFull}
	near := &Enemy{UID: 2, X: 505000, Y: 500000, HP: 1000000, MaxHP: 1000000,
		SpawnProgress: SpawnProgressFull, ArmorPermille: 0}
	e.Enemies = []*Enemy{origin, near}

	e.triggerReactionAoe(origin, ReactionOverheat, 10000, att)

	want := int64(1000000 - 2500)
	if near.HP != want {
		t.Fatalf("邻居 HP = %d，期望 %d（reactionDmg=10000 → 25%%=2500）；"+
			"旧实现再乘 DirectWeightPermille=400‰ 只会扣 1000", near.HP, want)
	}
}
