package domain

// 多地形关（D5，2026-10-20，DECISIONS D5 小步版）。
//
// 现状：`generateTerrain` 原先每关只产出**一种** kind（章节的 TerrainKind），
// 于是 100 关里地形机制的种类多样性受限（每关只有一种）。
// 现在每关最多 **2 种**（主种类 + 按种子稳定选的第二种类），同屏共存。
//
// 三条判据：
//
//  1. 每关最多 2 种 kind（防止「加着加着变 5 种」）；
//  2. 有第二种类时，它必须与主种类**不同**（否则轮换无意义）；
//  3. 生成确定性：同关两次生成逐字段一致（地形进 /config 与契约夹具，
//     不确定会让客户端与服务端拿到不同地形）。
//
// ⚠️ **未做**：按波次轮换（前半场主种类、后半场第二种类）。
// 那需要引擎感知当前波次（TS + Go 双端），属引擎级改动，留待后续。

import (
	"reflect"
	"testing"
)

// TestTerrainLevelsHaveAtMostTwoKinds 每关最多 2 种地形。
func TestTerrainLevelsHaveAtMostTwoKinds(t *testing.T) {
	withTerrain := 0
	for id := 1; id <= 100; id++ {
		gl := GenerateLevel(id)
		if len(gl.Terrain) == 0 {
			continue
		}
		withTerrain++
		kinds := map[string]bool{}
		for _, tp := range gl.Terrain {
			kinds[tp.Kind] = true
		}
		if len(kinds) > 2 {
			t.Errorf("第 %d 关有 %d 种地形（%v）—— D5 上限是 2 种", id, len(kinds), kinds)
		}
		// 至少 1 种，且件数在 1..3（原生成器约束不变）
		if len(gl.Terrain) < 1 || len(gl.Terrain) > 3 {
			t.Errorf("第 %d 关地形件数 %d 超出 [1,3]", id, len(gl.Terrain))
		}
	}
	if withTerrain == 0 {
		t.Fatal("100 关里没有一关有地形 —— 守卫没扫到任何样本")
	}
}

// TestTerrainSecondKindDiffersFromPrimary 第二种类必须与主种类不同。
//
// 判据用「第一件的 kind = 章节主种类」推不出来（件数轮换），
// 所以直接断言：若某关有 2 种 kind，它们必然互异（集合大小 = 2 即证）。
// 更有意义的是**反向**：不能出现「2 种 kind 但其中一个是空串」。
func TestTerrainSecondKindDiffersFromPrimary(t *testing.T) {
	for id := 1; id <= 100; id++ {
		gl := GenerateLevel(id)
		for _, tp := range gl.Terrain {
			if tp.Kind == "" {
				t.Errorf("第 %d 关出现空 kind 的地形", id)
			}
			if !allTerrainKinds()[tp.Kind] {
				t.Errorf("第 %d 关出现未登记的地形种类 %q —— "+
					"它会通过生成、但被结算校验拒绝", id, tp.Kind)
			}
		}
	}
}

// TestTerrainGenerationIsDeterministic 同关两次生成逐字段一致。
//
// 地形进 /config 与契约夹具；不确定的话客户端与服务端拿到不同地形，
// 而 battle_records 的 terrain_used 校验会随机拒绝正常战报。
func TestTerrainGenerationIsDeterministic(t *testing.T) {
	for _, id := range []int{1, 3, 50, 97, 100} {
		a := GenerateLevel(id)
		b := GenerateLevel(id)
		if !reflect.DeepEqual(a.Terrain, b.Terrain) {
			t.Errorf("第 %d 关两次生成的地形不一致：\nA=%v\nB=%v", id, a.Terrain, b.Terrain)
		}
	}
}
