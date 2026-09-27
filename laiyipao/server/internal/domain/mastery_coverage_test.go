package domain

// 守卫：专精树的**每一种 kind 都必须被 EvaluateMastery 处理**。
//
// ## 为什么需要这条
//
// 本项目曾有一个节点种类**完全惰性**：第 3 层槽 1 的 `mechanic`（"机制改造"），
// 8 系 × 1 个节点 = 8 个真实节点出现在 `/config`、出现在 I-7 构筑评分里，
// 玩家花点数点得出来 —— 而 `EvaluateMastery` 的 switch 里**连 case 都没有**，
// 于是它对任何数值的贡献恒为 0。
//
// 没有任何测试会红：
//   - 树结构测试只检查节点数量、层间约束、ID 规则 —— `mechanic` 是合法节点
//   - 评分测试只检查总分区间 —— `mechanic` 贡献 0 也在区间内
//   - 战斗测试用固定攻方，不经过专精树
//
// 三个测试都"覆盖"了这个节点，却没有一个断言"它有效果"。
//
// **形状**：switch 的 case 列表是从前几层抄的，第 3 层新增的 kind 没跟着补。
// 这与本项目已修的多个缺陷同形 —— 接线存在，但没有"全覆盖"守卫。

import (
	"testing"
)

// allMasteryKinds 返回专精树里出现过的全部 kind（去重、按出现顺序）。
func allMasteryKinds() []string {
	seen := map[string]bool{}
	var out []string
	for layer := range masteryLayerKinds {
		for _, k := range masteryLayerKinds[layer] {
			if !seen[k] {
				seen[k] = true
				out = append(out, k)
			}
		}
	}
	return out
}

// TestEveryMasteryKindIsEvaluated 逐个 kind 断言"选中后效果非零"。
//
// 判据是**行为**而不是源码文本扫描：
// 源码扫描（grep `case "mechanic"`）会因为大小写、别名、
// 或把 case 挪到别处而误判，而"选中它之后效果有没有变"是无法伪造的。
func TestEveryMasteryKindIsEvaluated(t *testing.T) {
	all := AllMasteryFamilies()
	allNodes := make([]MasteryNode, 0, 96)
	for _, f := range all {
		allNodes = append(allNodes, f.Nodes...)
	}

	for _, kind := range allMasteryKinds() {
		// 找一个该 kind 的节点（取**最小层数**，value 最小，避免数值噪声）
		var node MasteryNode
		found := false
		for _, n := range allNodes {
			if n.Kind == kind && (!found || n.Layer < node.Layer) {
				node, found = n, true
			}
		}
		if !found {
			t.Errorf("kind %q 出现在 masteryLayerKinds 里，但树上找不到对应节点", kind)
			continue
		}

		// 基线：不选任何节点
		base, err := EvaluateMastery(allNodes, map[int]bool{}, 100)
		if err != nil {
			t.Fatalf("kind %q: 基线 EvaluateMastery 失败: %v", kind, err)
		}

		// 选中它，**连同它的前置链**。
		//
		// ⚠️ 第 2/3 层节点有 `PrereqLay` 约束（需要先点亮上一层），
		// 不补前置就会得到「需要先点亮第 N 层」的 error ——
		// 而那是**前置校验在正常工作**，不是本测试要找的惰性。
		prereqSel := map[int]bool{}
		for l := 1; l < node.Layer; l++ {
			if prereq, ok := findPrereqNode(allNodes, l, node.Slot); ok {
				prereqSel[prereq.ID] = true
			}
		}
		withPrereq, err := EvaluateMastery(allNodes, prereqSel, 100)
		if err != nil {
			t.Errorf("kind %q: 选前置节点失败: %v", kind, err)
			continue
		}

		sel := map[int]bool{node.ID: true}
		for id := range prereqSel {
			sel[id] = true
		}
		got, err := EvaluateMastery(allNodes, sel, 100)
		if err != nil {
			t.Errorf("kind %q: 选中节点 %d 后 EvaluateMastery 失败: %v", kind, node.ID, err)
			continue
		}

		// ⚠️ 判据是**增量**，不是"这组选择有没有效果"。
		//
		// 第一版写成 `effectIsZero(got)`，结果**变异测试没抓到**
		// 「删掉 mechanic 的 case」—— 因为 mechanic 在第 3 层，
		// 它的前置节点是 heat_cap，**本身就有效果**，
		// 于是 got 非零，惰性的 mechanic 被前置的效果掩盖了。
		//
		// 这是本项目第四次在"度量点选错"上栽跟头：
		// 「加入 A 之后整体有变化」不能证明「变化来自 A」。
		// 正确的对照是「只加前置」vs「前置 + A」。
		if withPrereq == got {
			t.Errorf(
				"kind %q（节点 %d「%s」，value=%d）：加上它之后 MasteryEffect 与"+
					"「只选前置节点」完全相同（%+v）—— 该 kind 在 EvaluateMastery 的 "+
					"switch 里没有对应的 case，选它等于什么都没发生",
				kind, node.ID, node.Name, node.Value, got,
			)
		}
		_ = base
	}
}

// findPrereqNode 找一个第 layer 层、槽位尽量与 target 相同的节点作为前置。
func findPrereqNode(all []MasteryNode, layer, slot int) (MasteryNode, bool) {
	for _, n := range all {
		if n.Layer == layer && n.Slot == slot {
			return n, true
		}
	}
	for _, n := range all {
		if n.Layer == layer {
			return n, true
		}
	}
	return MasteryNode{}, false
}

// TestMasteryKindsHaveDisplayNames 确认每个 kind 都有中文名。
//
// 这条看似无关，实际是同一形状的另一个表现：
// kind 若漏了映射，UI 会显示空字符串或 key 原文 ——
// 而"节点名称"是玩家做选择时唯一的信息。
func TestMasteryKindsHaveDisplayNames(t *testing.T) {
	for _, kind := range allMasteryKinds() {
		name, ok := masteryKindNames[kind]
		if !ok {
			t.Errorf("kind %q 在 masteryKindNames 里没有显示名 —— 节点会显示成空或 key 原文", kind)
			continue
		}
		if name == "" {
			t.Errorf("kind %q 的显示名是空字符串", kind)
		}
	}
}

func effectIsZero(e MasteryEffect) bool {
	return e.ElementCapBonus == 0 &&
		e.ReactionMultBonus == 0 &&
		e.HeatCapBonus == 0 &&
		e.ExtraSlots == 0 &&
		e.SkillDamageBonus == 0 &&
		e.CritBonus == 0 &&
		e.ArmorBonus == 0 &&
		e.MechanicBonus == 0
}
