package domain

// 宝石词条池的形状不变量。
//
// # 为什么这些值得钉死
//
// 词条池是**内容**，而它的形状决定三件事能不能成立：
//
//  1. 金宝石要 roll 3 条 → 每种宝石的候选池必须 ≥ 3
//  2. 同名词条不能重复出现 → 否则玩家看到「两条一样的词条」，
//     而数值上等于白送一条
//  3. `affix` 的取值必须能被 `loadout_attacker` 识别 →
//     写错一个名字，它会被静默计入 `AffixCountIgnored`
//
// 第 3 条尤其阴险：那条路径**不报错**，只是把词条丢掉。
// 所以这里直接对着 `loadout_attacker` 的 case 列表核对。

import (
	"fmt"
	"testing"
)

// knownGemAffixKinds 是 loadout_attacker.gemAffix 能消化的全部词条名。
//
// ⚠️ 这份名单**必须**与 `internal/service/loadout_attacker.go` 的 switch 同步。
// 两边不同步的后果：多出来的名字被静默忽略，少掉的名字解析不出来。
var knownGemAffixKinds = map[string]bool{
	"attack":        true,
	"skill_damage":  true,
	"armor":         true,
	"crit":          true,
	"element_coef":  true,
	"element_cap":   true,
	"reaction_mult": true,
	"heat_cap":      true,
}

// TestGemAffixPoolIsSane 词条池的形状不变量。
func TestGemAffixPoolIsSane(t *testing.T) {
	byGem := map[int][]SeedGemAffix{}
	seen := map[string]bool{} // "gemID/affix" → 是否写过

	for _, a := range SeedGemAffixes {
		byGem[a.GemID] = append(byGem[a.GemID], a)

		key := fmt.Sprintf("%d/%s", a.GemID, a.Affix)
		if seen[key] {
			t.Errorf("宝石 %d 的候选池里 %q 出现了两次 —— roll 会抽出两条同名词条，"+
				"玩家看到的是「两条一样」，数值上却是白送一条", a.GemID, a.Affix)
		}
		seen[key] = true

		if !knownGemAffixKinds[a.Affix] {
			t.Errorf("宝石 %d 的词条 %q 不在 loadout_attacker 能消化的名单里 —— "+
				"它会被静默计入 AffixCountIgnored（不报错，只是不生效）", a.GemID, a.Affix)
		}
		if a.MinValue <= 0 {
			t.Errorf("宝石 %d 的词条 %q 最小值 %d ≤ 0 —— 宝石是纯正向加成",
				a.GemID, a.Affix, a.MinValue)
		}
		if a.MaxValue < a.MinValue {
			t.Errorf("宝石 %d 的词条 %q 区间反了：min=%d > max=%d",
				a.GemID, a.Affix, a.MinValue, a.MaxValue)
		}
	}

	// 每种宝石都要有池子，且 ≥ 最大词条数（金 3 条）
	maxCount := 0
	for _, q := range GemQualities {
		if q.Count > maxCount {
			maxCount = q.Count
		}
	}
	for _, g := range SeedGems {
		n := len(byGem[g.ID])
		if n == 0 {
			t.Errorf("宝石 %s(%d) 没有任何候选词条 —— 它 roll 不出任何东西", g.Code, g.ID)
			continue
		}
		if n < maxCount {
			t.Errorf("宝石 %s(%d) 只有 %d 条候选词条，roll %d 条（金）会失败",
				g.Code, g.ID, n, maxCount)
		}
	}

	// 反向：词条池里不能有内容表之外的宝石 id（那会让播种撞外键）
	for gemID := range byGem {
		found := false
		for _, g := range SeedGems {
			if g.ID == gemID {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("词条池里有宝石 id=%d，但 SeedGems 里没有它 —— 播种会撞外键", gemID)
		}
	}
}

// TestGemQualityWeightsAreComplete 品质权重必须覆盖全部品质且合计为 10000。
//
// 合计不等于 10000 的后果不是报错，而是「某档概率被悄悄改变」——
// 因为 roll 是按累计值落桶的，多出来的权重永远不会被抽到。
func TestGemQualityWeightsAreComplete(t *testing.T) {
	total := 0
	for _, q := range GemQualities {
		w, ok := GemQualityWeight[q.Name]
		if !ok {
			t.Errorf("品质 %q 没有权重 —— roll 永远抽不到它，等于这档不存在", q.Name)
			continue
		}
		if w <= 0 {
			t.Errorf("品质 %q 的权重为 %d（必须为正）", q.Name, w)
		}
		total += w
	}
	if total != 10000 {
		t.Errorf("品质权重合计 %d，应为 10000（万分比）—— 不等于这个数时，"+
			"超出/不足的部分会被静默吞掉", total)
	}
	for name := range GemQualityWeight {
		if GemQualityMult(name) == 1000 && name != "gray" {
			t.Errorf("权重里有 %q，但它不在 GemQualities 里", name)
		}
	}
}
