// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package domain

import "testing"

// TestReactionSplashMatchesCrossEndVectors 是 t41 的绝对值契约。
//
// 与 armor_contract_test.go 同分工：Go 侧这份 + TS 侧
// reaction_splash_contract.test.ts 读同一个 formula_vectors.json。
// 向量若被删空、或某一端偷偷把溅射塞回 ResolveHit，这里必须红。
func TestReactionSplashMatchesCrossEndVectors(t *testing.T) {
	v := loadVectors(t)
	cases := v.ReactionSplash.Cases
	if len(cases) == 0 {
		t.Fatal("formula_vectors.json 里 reaction_splash.cases 是空的 —— " +
			"t41 修掉的 25%→10% 回归会失去守卫")
	}
	for _, c := range cases {
		t.Run(c.Name, func(t *testing.T) {
			def := NewDefender(c.HP, c.Shield, c.ArmorPermille)
			got := ApplyReactionSplash(&def, c.Splash)
			if got.Damage != c.Expected.Damage || def.HP != c.Expected.HP ||
				def.Shield != c.Expected.Shield ||
				got.ShieldBroken != c.Expected.ShieldBroken || got.Killed != c.Expected.Killed {
				t.Fatalf("ApplyReactionSplash(splash=%d, hp=%d, shield=%d, armor=%d) = "+
					"{damage:%d hp:%d shield:%d broken:%v killed:%v}，契约期望 "+
					"{damage:%d hp:%d shield:%d broken:%v killed:%v}",
					c.Splash, c.HP, c.Shield, c.ArmorPermille,
					got.Damage, def.HP, def.Shield, got.ShieldBroken, got.Killed,
					c.Expected.Damage, c.Expected.HP, c.Expected.Shield,
					c.Expected.ShieldBroken, c.Expected.Killed)
			}
		})
	}
}

// TestReactionSplashIgnoresCritAndStacks 钉住「纯函数」这两个字。
// 溅射若哪天又被改回走 ResolveHit，att.crit / 守方层数会重新影响结果。
func TestReactionSplashIgnoresCritAndStacks(t *testing.T) {
	def := NewDefender(10000, 0, 0)
	def.ElementStacks[ElementFire] = 3
	def.ElementStacks[ElementLightning] = 2
	got := ApplyReactionSplash(&def, 1000)
	if got.Damage != 1000 || def.HP != 9000 {
		t.Fatalf("纯溅射应只结算护甲→护盾→扣血，得到 damage=%d hp=%d", got.Damage, def.HP)
	}
	if def.Stacks(ElementFire) != 3 || def.Stacks(ElementLightning) != 2 {
		t.Fatalf("溅射不得写元素层数：fire=%d lightning=%d",
			def.Stacks(ElementFire), def.Stacks(ElementLightning))
	}
}
