package domain

import (
	"sort"
	"testing"
)

// TestReplayContractCoversPlayerBuildDimensions 断言契约向量里
// **确实有玩家会真的配出来的构筑**，而不只是一套固定模板换关卡。
//
// # 为什么要单独一条
//
// 2026-10-15 加这批变体之前，5 个用例**共用同一套构筑**：
// 技能 id 1/2/3/4、全是 `active`、元素只有 fire 与 ice、
// `active_slots` 恒为 4。于是下面这几条路径**从来没被跨端重放过**，
// 而 `TestWholeBattleReplayMatchesCrossEnd` 一直是绿的：
//
//	kind = "passive" 的技能（夹具 id 19 侦察无人机 / 22 阻击工事）
//	corrosion / lightning 两个元素（反应组合与 fire/ice 完全不同）
//	composite_skills（id 31~48）
//	**active_slots > 4** —— 最要紧的一条
//
// 最后这条尤其危险：`engine.go` 的注释明写「只有玩家在专精树点了
// 『额外插槽』时才会 > 4」，也就是 5/6 槽是**真实生产场景**；
// 而 `ActiveSlots` **不在哈希锚点里**（A 段只有 7 项攻方系数，
// X 段只有额外血量），它只通过「哪些技能能进牌池」间接影响事件流。
// 不测它，就不知道专精玩家会不会在接线后被判成伪造。
//
// # 为什么是「守卫」而不是「多写几个用例」
//
// 光加用例不够：有人把变体删掉、或者生成器里的 BUILD_VARIANTS 清空，
// 那 13 个用例会变回 5 个，跨端重放照样全绿 —— **绿的含义变了，
// 但没有任何东西会告诉你**。这条断言就是让那种删除会立刻变红。
func TestReplayContractCoversPlayerBuildDimensions(t *testing.T) {
	doc := loadReplayVectors(t)

	var (
		elements    = map[string]int{} // 元素 -> 出现在几个用例里
		kinds       = map[string]int{}
		slotCounts  = map[int]int{}
		composite   int // 引用了 composite_skills（id >= 31）的用例数
		passiveCase int
		leakCase    int
	)

	for _, c := range doc.Cases {
		slotCounts[c.Build.ActiveSlots]++

		// leaked > 0 覆盖「敌人漏进底线」那条分支。
		// 它由关卡难度与构筑共同决定 —— 弱构筑更容易漏，
		// 所以构筑变体顺带把它带出来了。
		if c.ExpectedStats.Leaked > 0 {
			leakCase++
		}

		caseHasPassive := false
		caseHasComposite := false
		for _, s := range c.Build.Skills {
			elements[s.Element]++
			kinds[s.Kind]++
			if s.Kind == "passive" {
				caseHasPassive = true
			}
			// 夹具里 base skills 是 id 1~24，composite_skills 是 id 31~48。
			// 用 id 区间判定而不是加一张表 —— 加表就得同步维护，
			// 而两张表合并的产物 `skills` map（replay.go 里的那个）
			// 本来就是按同一套 id 空间建的。
			if s.ID >= 31 {
				caseHasComposite = true
			}
		}
		if caseHasPassive {
			passiveCase++
		}
		if caseHasComposite {
			composite++
		}
	}

	// ── passive ──
	if kinds["passive"] == 0 {
		t.Error("向量里没有任何 kind=passive 的技能 —— " +
			"被动技能走的是与 active 不同的分支（不开火、只挂效果），" +
			"不覆盖它就无法排除「被动在两端算出不同结果」")
	}
	if passiveCase == 0 {
		t.Error("没有任何一个用例带被动技能")
	}

	// ── 五种元素 ──
	// 夹具的元素空间就是这五个，少一个就意味着某个元素的反应组合没测过。
	for _, el := range []string{"fire", "ice", "lightning", "kinetic", "corrosion"} {
		if elements[el] == 0 {
			t.Errorf("向量里没有 %s 元素的技能（已覆盖：%v）—— "+
				"反应组合是按元素对分的，漏一个元素就漏掉一整类反应路径",
				el, keysOf(elements))
		}
	}

	// ── 复合技能 ──
	if composite == 0 {
		t.Error("没有任何用例引用 composite_skills（id >= 31）—— " +
			"复合技能同时挂两个元素的 apply，反应与加成都与单元素不同")
	}

	// ── active_slots > 4（专精额外插槽）──
	maxSlots := 0
	for n := range slotCounts {
		if n > maxSlots {
			maxSlots = n
		}
	}
	if maxSlots <= 4 {
		t.Errorf("向量里最大的 active_slots = %d，没有一个用例超过 4 —— "+
			"专精树的「额外插槽」是真实生产场景（engine.go 的注释明写"+
			"「只有玩家在专精树点了额外插槽时才会 > 4」），而 ActiveSlots "+
			"不在哈希锚点里，只通过牌池间接影响事件流。"+
			"不覆盖它，接线后专精玩家可能被判成伪造。已覆盖的槽位数：%v",
			maxSlots, slotCounts)
	}

	// ── 漏怪分支 ──
	if leakCase == 0 {
		t.Error("没有任何一个用例 leaked > 0 —— 「敌人漏进底线」是独立的引擎分支")
	}

	// 失败时把实际分布打出来，否则只能看到「缺了哪个」，
	// 看不到「现在是什么样」，排查要去翻 13 个 JSON 用例。
	if t.Failed() {
		t.Logf("当前覆盖：")
		t.Logf("  元素分布: %v", elements)
		t.Logf("  技能类型: %v", kinds)
		t.Logf("  槽位数分布: %v", slotCounts)
		t.Logf("  带被动的用例: %d  带复合技能的用例: %d  有漏怪的用例: %d",
			passiveCase, composite, leakCase)
	}
}

func keysOf(m map[string]int) []string {
	out := make([]string, 0, len(m))
	for k := range m {
		out = append(out, k)
	}
	sort.Strings(out)
	return out
}
