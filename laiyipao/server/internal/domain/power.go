package domain

// 构筑评分（I-7）与战力计算。
//
// ⚠️ 双端公式一致性：本文件与 miniapp/src/game/rating.ts 必须保持一致。
// 改动任一公式需同步两处并更新 testdata/formula_vectors.json。

// MasteryNode 是专精树的一个节点（I-3）。
type MasteryNode struct {
	ID        int
	Family    string
	Layer     int // 1..3
	Slot      int // 0..3
	Name      string
	Kind      string // element_cap/reaction_mult/heat_cap/extra_slot/mechanic/armor
	Value     int
	PrereqFam string
	PrereqLay int
}

// MasteryFamily 是专精树的一系。
type MasteryFamily struct {
	Family string
	Name   string
	Nodes  []MasteryNode
}

// masteryFamilyOrder 八系固定顺序，TS 侧 mastery.ts 必须一致。
var masteryFamilyOrder = []struct{ code, name string }{
	{"flame", "焰"},
	{"frost", "冰"},
	{"volt", "电"},
	{"kinetic", "物理"},
	{"light", "光"},
	{"blight", "毒"},
	{"drone", "无人机"},
	{"engineering", "工程"},
}

// 每系 3 层 × 4 槽 = 12 个节点，每层 4 选 2。
//
// 节点价值随层数递增，但每层只能选 2 个 —— 这是"点数有限"之外
// 的第二重约束，保证不可能点满一棵树，强制玩家做方向选择。
var masteryLayerKinds = [3][4]string{
	{"element_cap", "reaction_mult", "heat_cap", "skill_damage"},
	{"element_cap", "reaction_mult", "heat_cap", "crit"},
	{"extra_slot", "mechanic", "reaction_mult", "armor"},
}

var masteryLayerValues = [3]int{1, 2, 3}

var masteryKindNames = map[string]string{
	"element_cap":   "元素上限",
	"reaction_mult": "反应倍率",
	"heat_cap":      "热量上限",
	"extra_slot":    "额外插槽",
	"mechanic":      "机制改造",
	"skill_damage":  "技能伤害",
	"crit":          "暴击率",
	"armor":         "防线护甲",
}

// AllMasteryFamilies 生成 8 系 × 12 节点的完整专精树。
// ID = familyIndex*100 + layer*10 + slot + 1，便于与前端对齐。
func AllMasteryFamilies() []MasteryFamily {
	out := make([]MasteryFamily, 0, len(masteryFamilyOrder))
	for fi, f := range masteryFamilyOrder {
		mf := MasteryFamily{Family: f.code, Name: f.name}
		for layer := 1; layer <= 3; layer++ {
			for slot := 0; slot < 4; slot++ {
				kind := masteryLayerKinds[layer-1][slot]
				mf.Nodes = append(mf.Nodes, MasteryNode{
					ID:        fi*100 + layer*10 + slot + 1,
					Family:    f.code,
					Layer:     layer,
					Slot:      slot,
					Name:      masteryKindNames[kind],
					Kind:      kind,
					Value:     masteryLayerValues[layer-1] * 10,
					PrereqLay: layer - 1,
				})
			}
		}
		out = append(out, mf)
	}
	return out
}

// PerLayerPickLimit 每层最多可选择的节点数（4 选 2）。
const PerLayerPickLimit = 2

// BaseSkillSlots 基础插槽数（4 主动 + 1 被动）。
const BaseSkillSlots = 5

// MasteryEffect 是专精点分配后的合计效果。
type MasteryEffect struct {
	ElementCapBonus   int64
	ReactionMultBonus int64 // 千分比
	HeatCapBonus      int64
	ExtraSlots        int64
	SkillDamageBonus  int64 // 千分比
	CritBonus         int64 // 千分比
	ArmorBonus        int64 // 千分比
}

// EvaluateMastery 根据已选节点集合计算合计效果。
// 非法分配（超点数 / 违反每层 2 个上限 / 层前置不满足）返回 error，
// 绝不做静默截断 —— 后台与客户端都依赖这个校验来提示玩家。
func EvaluateMastery(allNodes []MasteryNode, selectedIDs map[int]bool, points int) (MasteryEffect, error) {
	byID := make(map[int]MasteryNode, len(allNodes))
	for _, n := range allNodes {
		byID[n.ID] = n
	}
	perLayer := map[string]int{}
	effect := MasteryEffect{}

	// 先按层排序校验：第 2、3 层必须先满足第 1、2 层
	selected := make([]MasteryNode, 0, len(selectedIDs))
	for id := range selectedIDs {
		n, ok := byID[id]
		if !ok {
			return effect, errUnknownMasteryNode(id)
		}
		selected = append(selected, n)
	}
	// 按 family + layer 升序
	sortMasteryNodes(selected)

	lastLayer := map[string]int{}
	for _, n := range selected {
		// 计数必须按 (family, layer)，不能只按 family ——
		// 否则"每层 4 选 2"会被误判成"每系总共 2 个"，合法分配被错误拒绝。
		key := n.Family + "#" + itoa(n.Layer)
		perLayer[key]++
		if perLayer[key] > PerLayerPickLimit {
			return effect, errMasteryLayerLimit(n)
		}
		if n.Layer > 1 && lastLayer[n.Family] < n.Layer-1 {
			return effect, errMasteryPrereq(n)
		}
		lastLayer[n.Family] = n.Layer
	}
	if len(selected) > points {
		return effect, errMasteryPoints(len(selected), points)
	}

	for _, n := range selected {
		v := int64(n.Value)
		switch n.Kind {
		case "element_cap":
			effect.ElementCapBonus += v
		case "reaction_mult":
			effect.ReactionMultBonus += v
		case "heat_cap":
			effect.HeatCapBonus += v
		case "extra_slot":
			effect.ExtraSlots += 1
		case "skill_damage":
			effect.SkillDamageBonus += v
		case "crit":
			effect.CritBonus += v
		case "armor":
			effect.ArmorBonus += v
		}
	}
	return effect, nil
}

// BuildRating 是构筑评分（I-7）。取代"战力 9999"作为主指标，
// 引导玩家思考搭配而不是堆数值。
type BuildRating struct {
	ElementCoverage  int `json:"element_coverage"`  // 0..5
	ReactionCoverage int `json:"reaction_coverage"` // 0..7
	MasteryDone      int `json:"mastery_done"`      // 0..8 已投入专精的系数
	MasteryPicked    int `json:"mastery_picked"`    // 本次选择的节点数
	EquipmentSynergy int `json:"equipment_synergy"` // 0..18 装备与技能同系的数量
	MechanicDepth    int `json:"mechanic_depth"`    // 0..8
	Total            int `json:"total"`

	// 以下是**实际生效的战斗乘区**（千分比），不是评分维度。
	//
	// ⚠️ 之前它们只存在于 RatingInput、算完即丢，于是 computeAttacker
	// 只能硬编码常量 —— 专精树的 reaction_mult / crit 节点成了
	// "只写不读"的空节点，玩家投入后看不到任何效果。
	//
	// 放进 BuildRating 还顺带解决了一个体验问题：玩家在构筑页能看到
	// 「反应倍率 +400‰」这类真实数值，而不是一个与战斗无关的总分。
	ReactionMultBonus int64 `json:"reaction_mult_bonus"` // 千分比
	CritBonus         int64 `json:"crit_bonus"`          // 千分比
	ElementCapBonus   int64 `json:"element_cap_bonus"`   // 层数
	ArmorBonus        int64 `json:"armor_bonus"`         // 千分比
	// 由 ReactionMultBonus 反推的节点数（用于决定反应阶）
	ReactionMultNodes int `json:"reaction_mult_nodes"`

	Weaknesses []string `json:"weaknesses"` // 给玩家的提示
}

// MasteryReactionMultPerNode 是每个 reaction_mult 专精节点提供的倍率（千分比）。
// 由 ReactionMultBonus 反推节点数时用它：nodes = bonus / perNode。
const MasteryReactionMultPerNode = 200
// RatingWeights 评分权重，见 GAME_DESIGN I-7。
//
// ⚠️ 这组 json tag 不是可有可无的装饰。
// 它嵌在 ContentResp.rating_weights 里下发，一旦漏掉 tag，
// Go 会按字段名序列化成 PascalCase（ElementCoverage / ReactionCoverage ...），
// 而同一个响应里 BuildRating 用的是 snake_case —— 同一份 JSON 里两种命名风格。
// 客户端按惯例写 rating_weights.element_coef_permille 只会拿到 undefined，
// **且不报错**（undefined 一路传到模板里显示空）。
// 这类缺陷只能靠显式 tag 挡住，struct 名字本身不提供任何保护。
type RatingWeights struct {
	ElementCoverage  int `json:"element_coverage"`
	ReactionCoverage int `json:"reaction_coverage"`
	MasteryDone      int `json:"mastery_done"`
	EquipmentSynergy int `json:"equipment_synergy"`
	MechanicDepth    int `json:"mechanic_depth"`
}

// DefaultRatingWeights 返回设计文档中的权重。
func DefaultRatingWeights() RatingWeights {
	return RatingWeights{
		ElementCoverage:  12,
		ReactionCoverage: 15,
		MasteryDone:      10,
		EquipmentSynergy: 12,
		MechanicDepth:    8,
	}
}

// RatingInput 是评分输入。
type RatingInput struct {
	SkillElements     []Element // 已装备技能携带的元素
	SkillSkills       int       // 已装备技能数
	MasteryFamilies   []string  // 已投入专精点的系
	MasteryPicked     int       // 已选专精节点总数
	EquipmentElements []Element // 已穿装备的契合标签
	PassiveCount      int       // 已解锁的机制型被动数量
	// 专精节点实际提供的战斗乘区（来自 EvaluateMastery 的 MasteryEffect）。
	//
	// ⚠️ 加这四个字段的原因：这些 bonus 之前只存在于 MasteryEffect 里、
	// 算完就丢，于是 service.computeAttacker 只能硬编码常量 ——
	// 专精树的 reaction_mult / crit / element_cap / armor 节点
	// 全部变成「只写不读」的空节点，玩家投入后看不到任何效果。
	ReactionMultBonus int64 // 千分比
	CritBonus         int64 // 千分比
	ElementCapBonus   int64 // 层数
	ArmorBonus        int64 // 千分比
}

// ComputeBuildRating 计算构筑评分并给出短板提示。
func ComputeBuildRating(in RatingInput, w RatingWeights) BuildRating {
	// 元素覆盖：去重后统计
	elemSet := map[Element]bool{}
	for _, e := range in.SkillElements {
		elemSet[e] = true
	}
	// 反应覆盖：该搭配能触发的反应链数量
	reactions := 0
	seen := map[ReactionKey]bool{}
	for _, existing := range AllElements() {
		if !elemSet[existing] {
			continue
		}
		for _, incoming := range AllElements() {
			if k, ok := LookupReaction(existing, incoming); ok && !seen[k] {
				seen[k] = true
				reactions++
			}
		}
	}

	masteryDone := len(in.MasteryFamilies)
	// 装备契合：与技能元素同系的装备计入（I-3：同系给大加成，异系给小额）
	skillElemSet := map[Element]bool{}
	for _, e := range in.SkillElements {
		skillElemSet[e] = true
	}
	synergy := 0
	for _, e := range in.EquipmentElements {
		if skillElemSet[e] {
			synergy++
		}
	}

	r := BuildRating{
		ElementCoverage:  len(elemSet),
		ReactionCoverage: reactions,
		MasteryDone:      masteryDone,
		MasteryPicked:    in.MasteryPicked,
		EquipmentSynergy: synergy,
		MechanicDepth:    int(minInt64(int64(in.PassiveCount), 8)),
		// 把实际生效的乘区透出给 service 层。
		//
		// ⚠️ 之前这些 bonus 只存在于 RatingInput 里、算完就丢，
		// 于是 computeAttacker 只能硬编码常量 —— 专精树的
		// reaction_mult / crit 节点变成"只写不读"的空节点。
		//
		// 放进 BuildRating 还有第二个作用：build_rating 会下发给客户端，
		// 玩家能在构筑页看到「元素系数 +240‰ / 反应倍率 +400‰」这类实际生效值，
		// 而不是一个与战斗无关的总分。
		ReactionMultBonus: in.ReactionMultBonus,
		CritBonus:         in.CritBonus,
		ElementCapBonus:   in.ElementCapBonus,
		ArmorBonus:        in.ArmorBonus,
		// 每 3 个 reaction_mult 节点提升 1 阶（封顶由 service 层处理）
		ReactionMultNodes: int(in.ReactionMultBonus / MasteryReactionMultPerNode),
	}
	r.Total = len(elemSet)*w.ElementCoverage +
		reactions*w.ReactionCoverage +
		masteryDone*w.MasteryDone +
		synergy*w.EquipmentSynergy +
		r.MechanicDepth*w.MechanicDepth

	// 短板提示：明确告诉玩家缺什么、为什么缺
	if len(elemSet) < 5 {
		var missing []string
		for _, e := range AllElements() {
			if !elemSet[e] {
				missing = append(missing, elementName(e))
			}
		}
		r.Weaknesses = append(r.Weaknesses,
			"你缺少 "+joinCN(missing)+" 系技能 → 无法触发部分反应链")
	}
	if reactions < 7 {
		r.Weaknesses = append(r.Weaknesses,
			"当前搭配只能触发 "+itoa(reactions)+"/7 条反应链，补齐元素可解锁更多")
	}
	if synergy == 0 {
		r.Weaknesses = append(r.Weaknesses,
			"没有任何装备与技能同系 → 契合加成为 0，建议重新搭配")
	}
	if masteryDone == 0 {
		r.Weaknesses = append(r.Weaknesses, "尚未投入任何专精点")
	}
	return r
}

// Power 是战力值。保留是为了满足玩家对"数值成长"的直接诉求，
// 但首页主指标已改为构筑评分（I-7）。
type PowerInput struct {
	SkillLevels   map[int]int64 // skillID → level
	EquipmentLvls map[int]int64 // equipmentID → level
	MasteryPicked int
	ElementCap    int64
}

// ComputePower 计算战力。
func ComputePower(in PowerInput) int64 {
	var total int64
	for _, lv := range in.SkillLevels {
		total += lv * 120
	}
	for _, lv := range in.EquipmentLvls {
		total += lv * 60
	}
	total += int64(in.MasteryPicked) * 80
	total += in.ElementCap * 40
	return total
}

// --- 装备契合度（I-3） ---

// SynergySameFamilyPct 装备与技能同系时的加成（千分比）。
const SynergySameFamilyPct = 300

// SynergyOtherFamilyPct 装备与技能不同系时的加成（千分比）。
// 硬性约束：必须明显小于同系加成，否则玩家没有动力成套搭配。
// 设计目标：OtherPct * 3 <= SamePct。
const SynergyOtherFamilyPct = 100

// SynergyPct 返回单件装备对某个元素的契合加成。
// 规则：装备标签元素与该技能元素相同 → 大加成；否则给小额。
func SynergyPct(equipElement, skillElement Element) int64 {
	if equipElement == "" || skillElement == "" {
		return 0
	}
	if equipElement == skillElement {
		return SynergySameFamilyPct
	}
	return SynergyOtherFamilyPct
}

// --- 内部工具 ---

type domainError string

func (e domainError) Error() string { return string(e) }

func errUnknownMasteryNode(id int) error {
	return domainError("专精节点不存在：" + itoa(id))
}

func errMasteryLayerLimit(n MasteryNode) error {
	return domainError("第 " + itoa(n.Layer) + " 层最多只能选 " + itoa(PerLayerPickLimit) + " 个节点（" + n.Family + "）")
}

func errMasteryPrereq(n MasteryNode) error {
	return domainError("节点 " + n.Name + " 需要先点亮第 " + itoa(n.PrereqLay) + " 层")
}

func errMasteryPoints(used, avail int) error {
	return domainError("专精点不足：需要 " + itoa(used) + " 点，仅有 " + itoa(avail) + " 点")
}

func sortMasteryNodes(ns []MasteryNode) {
	// 简单插入排序：节点数很少（≤96），无需引入 sort 依赖
	for i := 1; i < len(ns); i++ {
		x := ns[i]
		j := i - 1
		for j >= 0 && (ns[j].Family > x.Family ||
			(ns[j].Family == x.Family && ns[j].Layer > x.Layer) ||
			(ns[j].Family == x.Family && ns[j].Layer == x.Layer && ns[j].Slot > x.Slot)) {
			ns[j+1] = ns[j]
			j--
		}
		ns[j+1] = x
	}
}

func elementName(e Element) string {
	switch e {
	case ElementFire:
		return "焰"
	case ElementIce:
		return "冰"
	case ElementLightning:
		return "电"
	case ElementCorrosion:
		return "毒"
	case ElementKinetic:
		return "动能"
	}
	return string(e)
}

func joinCN(items []string) string {
	switch len(items) {
	case 0:
		return ""
	case 1:
		return items[0]
	case 2:
		return items[0] + "、" + items[1]
	}
	out := ""
	for i, s := range items {
		if i > 0 {
			out += "、"
		}
		if i == len(items)-1 {
			out += s
			continue
		}
		out += s
	}
	return out
}
