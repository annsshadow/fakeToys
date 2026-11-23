// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package domain

import "testing"

// TestPierceProjectileCountsOneHitPerShot 守住跨端命中口径：
// hits 是「有多少次开火命中了东西」，不是「有多少次敌人结算」。
//
// 穿透弹丸一发穿过 4 只敌人时，旧实现在 checkProjectileHit 的主目标分支
// 对每个敌人都 e.Hits++，于是 hits=4 > shots=1，服务端 ValidateSettle
// 以 ErrInvalidHitRate 拒掉整局（真实 R13 电+腐蚀局 165>160 被拒）。
// 服务端校验本身是对的，必须在引擎侧修计数。
func TestPierceProjectileCountsOneHitPerShot(t *testing.T) {
	e := NewBattleEngine(BattleConfig{
		Level:       GeneratedLevel{ID: 1, BaseHP: 1000, Waves: []Wave{{Index: 0}}},
		ActiveSlots: ActiveSlots,
		ScoreRules:  DefaultScoreRules(),
	})
	e.Shots = 1
	for i, x := range []int64{200, 300, 400, 500} {
		e.Enemies = append(e.Enemies, &Enemy{
			UID:           int64(i + 1),
			X:             ToFixed(float64(x)),
			Y:             ToFixed(500),
			HP:            1_000_000,
			MaxHP:         1_000_000,
			SpawnProgress: SpawnProgressFull,
			Resist:        map[Element]int64{},
			Stacks:        map[Element]int64{},
		})
	}
	e.Projectiles = append(e.Projectiles, &Projectile{
		UID:        9000,
		X:          ToFixed(200),
		Y:          ToFixed(500),
		VX:         ToFixed(20) * TickMs,
		Damage:     5000,
		PierceLeft: 3,
		ChainLeft:  0,
		HitSet:     map[int64]bool{},
		Slot:       0,
	})
	for i := 0; i < 12 && len(e.Projectiles) > 0; i++ {
		e.updateProjectiles()
	}
	damaged := 0
	for _, en := range e.Enemies {
		if en.HP < 1_000_000 {
			damaged++
		}
	}
	if damaged != 4 {
		t.Fatalf("穿透夹具应打到 4 只敌人，实际 %d —— 夹具无效，不能作为命中口径证据", damaged)
	}
	if e.Hits > e.Shots {
		t.Fatalf("穿透弹丸一发多命中却记成 hits=%d > shots=%d —— 服务端 ValidateSettle 会拒掉这局",
			e.Hits, e.Shots)
	}
}
