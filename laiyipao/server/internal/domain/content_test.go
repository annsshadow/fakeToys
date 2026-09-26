package domain

import "testing"

// 专精树必须是 8 系 × 12 节点 = 96。
func TestMasteryTreeShape(t *testing.T) {
	fams := AllMasteryFamilies()
	if len(fams) != 8 {
		t.Fatalf("应为 8 系专精树，实际 %d", len(fams))
	}
	ids := map[int]bool{}
	for _, f := range fams {
		if len(f.Nodes) != 12 {
			t.Errorf("专精树 %s 应有 12 节点，实际 %d", f.Family, len(f.Nodes))
		}
		for _, n := range f.Nodes {
			if n.Layer < 1 || n.Layer > 3 {
				t.Errorf("节点 %d 层数 %d 越界", n.ID, n.Layer)
			}
			if n.Slot < 0 || n.Slot > 3 {
				t.Errorf("节点 %d 槽位 %d 越界", n.ID, n.Slot)
			}
			if ids[n.ID] {
				t.Errorf("专精节点 ID 重复：%d", n.ID)
			}
			ids[n.ID] = true
		}
	}
	if len(ids) != 96 {
		t.Errorf("专精节点总数应为 96，实际 %d", len(ids))
	}
}

// 专精点分配的三条硬约束都必须被拒绝，且错误信息要说清原因。
func TestMasteryConstraints(t *testing.T) {
	fams := AllMasteryFamilies()
	var nodes []MasteryNode
	for _, f := range fams {
		nodes = append(nodes, f.Nodes...)
	}
	byFamily := map[string][]MasteryNode{}
	for _, n := range nodes {
		byFamily[n.Family] = append(byFamily[n.Family], n)
	}
	fire := byFamily["flame"]

	t.Run("每层超过2个必须被拒", func(t *testing.T) {
		sel := map[int]bool{fire[0].ID: true, fire[1].ID: true, fire[2].ID: true}
		if _, err := EvaluateMastery(nodes, sel, 10); err == nil {
			t.Error("第 1 层选了 3 个节点，应被拒绝")
		}
	})

	t.Run("跳过前置层必须被拒", func(t *testing.T) {
		// 只选第 2 层，不选第 1 层
		var layer2 []int
		for _, n := range fire {
			if n.Layer == 2 {
				layer2 = append(layer2, n.ID)
			}
		}
		sel := map[int]bool{layer2[0]: true}
		if _, err := EvaluateMastery(nodes, sel, 10); err == nil {
			t.Error("未点亮第 1 层就选第 2 层，应被拒绝")
		}
	})

	t.Run("点数不足必须被拒", func(t *testing.T) {
		sel := map[int]bool{fire[0].ID: true, fire[1].ID: true, fire[4].ID: true, fire[5].ID: true}
		if _, err := EvaluateMastery(nodes, sel, 2); err == nil {
			t.Error("4 个节点仅 2 点，应被拒绝")
		}
	})

	t.Run("未知节点必须被拒", func(t *testing.T) {
		if _, err := EvaluateMastery(nodes, map[int]bool{99999: true}, 10); err == nil {
			t.Error("未知节点应被拒绝")
		}
	})

	t.Run("合法分配必须通过", func(t *testing.T) {
		sel := map[int]bool{fire[0].ID: true, fire[1].ID: true, fire[4].ID: true, fire[5].ID: true}
		eff, err := EvaluateMastery(nodes, sel, 4)
		if err != nil {
			t.Fatalf("合法分配被拒：%v", err)
		}
		if eff.ElementCapBonus <= 0 {
			t.Error("选了元素上限节点但效果为 0")
		}
		if eff.ReactionMultBonus <= 0 {
			t.Error("选了反应倍率节点但效果为 0")
		}
	})
}

// 装备契合度：同系加成必须显著高于异系（红线 OtherPct*3 <= SamePct）。
func TestSynergySameVsOther(t *testing.T) {
	if SynergyOtherFamilyPct*3 > SynergySameFamilyPct {
		t.Errorf("异系加成过高：%d‰ ×3 > 同系 %d‰，玩家没有动力成套搭配",
			SynergyOtherFamilyPct, SynergySameFamilyPct)
	}
	if got := SynergyPct(ElementFire, ElementFire); got != SynergySameFamilyPct {
		t.Errorf("同系契合应为 %d‰，实际 %d‰", SynergySameFamilyPct, got)
	}
	if got := SynergyPct(ElementIce, ElementFire); got != SynergyOtherFamilyPct {
		t.Errorf("异系契合应为 %d‰，实际 %d‰", SynergyOtherFamilyPct, got)
	}
	if got := SynergyPct("", ElementFire); got != 0 {
		t.Errorf("空标签应无加成，实际 %d‰", got)
	}
}

// 构筑评分：元素覆盖少时必须给出可读短板提示（I-7 的核心目的）。
func TestBuildRatingWeaknesses(t *testing.T) {
	w := DefaultRatingWeights()

	t.Run("只带焰系应提示缺元素", func(t *testing.T) {
		r := ComputeBuildRating(RatingInput{
			SkillElements: []Element{ElementFire, ElementFire},
		}, w)
		if r.ElementCoverage != 1 {
			t.Errorf("元素覆盖应为 1，实际 %d", r.ElementCoverage)
		}
		// 单元素并非打不出反应：敌人身上带冰/电/毒/动能时，焰系仍可触发 4 条。
		// 这里是断言"恰好 4 条"，提醒读者单元素搭配的真实上限。
		if r.ReactionCoverage != 4 {
			t.Errorf("只带焰系应能触发 4 条反应链，实际 %d", r.ReactionCoverage)
		}
		if len(r.Weaknesses) == 0 {
			t.Fatal("必须给出短板提示")
		}
		joined := ""
		for _, s := range r.Weaknesses {
			joined += s
		}
		if !contains(joined, "冰") || !contains(joined, "电") {
			t.Errorf("短板提示应点名缺失元素，实际：%s", joined)
		}
	})

	t.Run("五元素齐备应有高反应覆盖", func(t *testing.T) {
		r := ComputeBuildRating(RatingInput{
			SkillElements: AllElements(),
		}, w)
		if r.ElementCoverage != 5 {
			t.Errorf("元素覆盖应为 5，实际 %d", r.ElementCoverage)
		}
		if r.ReactionCoverage != 7 {
			t.Errorf("五元素齐备应能触发全部 7 条反应链，实际 %d", r.ReactionCoverage)
		}
		if r.Total <= 0 {
			t.Error("总分应大于 0")
		}
	})

	t.Run("无同系装备必须提示契合为0", func(t *testing.T) {
		r := ComputeBuildRating(RatingInput{
			SkillElements:     []Element{ElementFire, ElementIce},
			EquipmentElements: []Element{ElementLightning},
		}, w)
		if r.EquipmentSynergy != 0 {
			t.Errorf("不同系装备不计契合，实际 %d", r.EquipmentSynergy)
		}
		found := false
		for _, s := range r.Weaknesses {
			if contains(s, "契合") {
				found = true
			}
		}
		if !found {
			t.Errorf("应提示契合为 0，实际：%v", r.Weaknesses)
		}
	})
}

// 战力必须随养成单调不减。
func TestPowerMonotonic(t *testing.T) {
	prev := int64(-1)
	base := PowerInput{
		SkillLevels:   map[int]int64{1: 1},
		EquipmentLvls: map[int]int64{1: 1},
		MasteryPicked: 0,
		ElementCap:    2,
	}
	for step := 0; step < 20; step++ {
		in := base
		in.SkillLevels = map[int]int64{1: int64(step + 1)}
		in.EquipmentLvls = map[int]int64{1: int64(step + 1)}
		in.MasteryPicked = step
		got := ComputePower(in)
		if got < prev {
			t.Fatalf("第 %d 步战力倒退：%d < %d", step, got, prev)
		}
		prev = got
	}
}

// 技能表合法性：24 个基础技能（8 系各 3 个）+ 18 个配方产物 = 42 行。
func TestSeedSkillsIntegrity(t *testing.T) {
	if len(SeedSkills) != 24 {
		t.Errorf("应有 24 个基础技能，实际 %d", len(SeedSkills))
	}
	if len(SeedCompositeSkills) != 18 {
		t.Errorf("应有 18 个配方产物技能，实际 %d", len(SeedCompositeSkills))
	}
	byFamily := map[string]int{}
	ids := map[int]bool{}
	for _, s := range SeedSkills {
		byFamily[s.Family]++
		if ids[s.ID] {
			t.Errorf("技能 ID 重复：%d", s.ID)
		}
		ids[s.ID] = true
		if !IsElement(string(s.Element)) {
			t.Errorf("技能 %s 的元素非法：%q", s.Name, s.Element)
		}
		if s.Kind == "active" && s.HeatCost <= 0 {
			t.Errorf("主动技能 %s 热量消耗必须为正", s.Name)
		}
		if s.Kind == "active" && s.HeatCost > 100 {
			t.Errorf("技能 %s 热量消耗 %d 超过单次上限 100", s.Name, s.HeatCost)
		}
	}
	if len(byFamily) != 8 {
		t.Errorf("基础技能应有 8 个系，实际 %d", len(byFamily))
	}
	for fam, n := range byFamily {
		if n != 3 {
			t.Errorf("基础技能系 %s 应有 3 个技能，实际 %d", fam, n)
		}
	}
	// 配方产物的行必须存在（否则外键约束会在落库时炸）
	for _, s := range SeedCompositeSkills {
		if ids[s.ID] {
			t.Errorf("配方产物技能 ID 与基础技能冲突：%d", s.ID)
		}
		ids[s.ID] = true
	}
	// 配方引用的 A/B 必须是已存在的技能，Output 也必须真实存在
	for _, r := range SeedRecipes {
		if !ids[r.A] {
			t.Errorf("配方引用了不存在的技能 A=%d", r.A)
		}
		if !ids[r.B] {
			t.Errorf("配方引用了不存在的技能 B=%d", r.B)
		}
		if !ids[r.Output] {
			t.Errorf("配方产物技能不存在：output=%d（外键会失败）", r.Output)
		}
	}
	// 每个配方产物都必须至少有一条配方指向它
	producedBy := map[int]int{}
	for _, r := range SeedRecipes {
		producedBy[r.Output]++
	}
	for _, s := range SeedCompositeSkills {
		if producedBy[s.ID] == 0 {
			t.Errorf("配方产物技能 %s（ID %d）没有任何配方指向它", s.Name, s.ID)
		}
	}
}

// 敌人表合法性：BOSS 必须同时具备高抗与负抗，否则 I-1 的"以弱胜强"设计在
// BOSS 身上失效（玩家找不到可行解）。
func TestSeedEnemiesIntegrity(t *testing.T) {
	if len(SeedEnemies) < 20 {
		t.Errorf("敌人数量应不少于 20，实际 %d", len(SeedEnemies))
	}
	ids := map[int]bool{}
	for _, e := range SeedEnemies {
		if ids[e.ID] {
			t.Errorf("敌人 ID 重复：%d", e.ID)
		}
		ids[e.ID] = true
		if e.HP <= 0 {
			t.Errorf("敌人 %s 血量非法", e.Name)
		}
		if e.Speed <= 0 {
			t.Errorf("敌人 %s 速度非法", e.Name)
		}
		// 五维抗性必须完整，且在合法范围
		for _, el := range AllElements() {
			v, ok := e.Resist[el]
			if !ok {
				t.Errorf("敌人 %s 缺少 %s 抗性配置", e.Name, el)
				continue
			}
			if v < -MaxResistPermille || v > MaxResistPermille {
				t.Errorf("敌人 %s 的 %s 抗性 %d 越界", e.Name, el, v)
			}
		}
		if e.Category == "flying" && e.FlyHeight <= 0 {
			t.Errorf("飞行敌人 %s 必须有飞行高度", e.Name)
		}
		if e.Category == "ranged" && e.AttackRange <= 0 {
			t.Errorf("远程敌人 %s 必须有射程", e.Name)
		}
		if e.Category == "special" && !e.Burrow && e.ShieldHP <= 0 {
			t.Errorf("特殊敌人 %s 既不钻地也无护盾，定位不明", e.Name)
		}

		if e.IsBoss {
			hasHigh, hasLow := false, false
			for _, v := range e.Resist {
				if v >= 300 {
					hasHigh = true
				}
				if v <= -300 {
					hasLow = true
				}
			}
			if !hasHigh || !hasLow {
				t.Errorf("BOSS %s 必须同时具备高抗(≥300‰)与负抗(≤-300‰)，"+
					"否则玩家找不到可行解（当前 high=%v low=%v）", e.Name, hasHigh, hasLow)
			}
		}
	}
}

// levelgen 引用的所有敌人 ID 都必须在敌人表中存在。
func TestLevelGenReferencesExistingEnemies(t *testing.T) {
	ids := map[int]bool{}
	for _, e := range SeedEnemies {
		ids[e.ID] = true
	}
	for _, gl := range GenerateAllLevels() {
		for _, w := range gl.Waves {
			for _, s := range w.Spawns {
				if !ids[s.EnemyID] {
					t.Errorf("关卡 %d 引用了敌人表中不存在的 ID %d", gl.ID, s.EnemyID)
				}
			}
		}
	}
}

// 装备与宝石表完整性。
func TestSeedEquipmentAndGems(t *testing.T) {
	if len(SeedEquipmentList) != 18 {
		t.Errorf("应有 18 件装备（6 部位 × 3 阶），实际 %d", len(SeedEquipmentList))
	}
	perSlot := map[string]int{}
	for _, e := range SeedEquipmentList {
		perSlot[e.Slot]++
		if !IsElement(string(e.Element)) {
			t.Errorf("装备 %s 的契合元素非法：%q", e.Name, e.Element)
		}
		if e.Tier < 1 || e.Tier > 3 {
			t.Errorf("装备 %s 阶级 %d 越界", e.Name, e.Tier)
		}
	}
	if len(perSlot) != 6 {
		t.Errorf("应有 6 个装备部位，实际 %d", len(perSlot))
	}
	for slot, n := range perSlot {
		if n != 3 {
			t.Errorf("部位 %s 应有 3 件装备，实际 %d", slot, n)
		}
	}

	if len(SeedGems) != 8 {
		t.Errorf("应有 8 种宝石属性，实际 %d", len(SeedGems))
	}
	if len(GemQualities) != 5 {
		t.Errorf("应有 5 档宝石品质，实际 %d", len(GemQualities))
	}
	if len(SeedSkins) != 6 {
		t.Errorf("应有 6 款皮肤，实际 %d", len(SeedSkins))
	}
	for _, s := range SeedSkins {
		if s.Passive == "" {
			t.Errorf("皮肤 %s 必须带机制型被动", s.Name)
		}
	}
}

func contains(s, sub string) bool {
	return len(sub) == 0 || indexOf(s, sub) >= 0
}

func indexOf(s, sub string) int {
	for i := 0; i+len(sub) <= len(s); i++ {
		if s[i:i+len(sub)] == sub {
			return i
		}
	}
	return -1
}
