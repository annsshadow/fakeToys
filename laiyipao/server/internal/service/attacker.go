package service

import (
	"context"
	"fmt"

	"github.com/laiyipao/server/internal/domain"
)

// AttackerView 是下发给客户端的攻方属性，字段名与 TS 侧 game/damage.ts 的
// Attacker 接口一一对应（snake_case）。
//
// ⚠️ 这是 I-6 能否成立的关键：回放哈希由「关卡 + 种子 + 攻方属性 + 技能」
// 四者共同决定。若客户端自己猜攻击力（哪怕公式完全合理），
// 服务端与客户端算出的哈希就必然不同 —— 验真会把所有正常对局判成伪造。
//
// 因此：**攻方属性的唯一权威来源必须是服务端**。客户端在开局时从
// /battle/token 的 build.attacker 读取，重放时从 /battle/{id}/replay
// 的 build.attacker 读取，两次拿到的必须是同一份数值。
type AttackerView struct {
	Attack                 int64 `json:"attack"`
	CritPermille           int64 `json:"crit_permille"`
	CritMultiplierPermille int64 `json:"crit_multiplier_permille"`
	ReactionMultPermille   int64 `json:"reaction_mult_permille"`
	ElementCap             int64 `json:"element_cap"`
	ReactionTier           int64 `json:"reaction_tier"`
	ElementCoefPermille    int64 `json:"element_coef_permille"`
	// ⚠️ 下面三个字段必须下发，否则客户端算不出与本局一致的结果 ——
	// 而 I-6 要求重放哈希逐位一致，客户端自己猜数值必然算错。
	//
	// 这三个字段此前既不存在于 Attacker，也不存在于本结构：
	// 专精树的「热量上限」「护甲」「机制改造」与全部装备、宝石
	// 因此对战斗完全没有影响（详见 domain.Attacker 的注释）。
	HeatCapPermille  int64 `json:"heat_cap_permille"`
	ArmorPermille    int64 `json:"armor_permille"`
	MechanicPermille int64 `json:"mechanic_permille"`
}

// toView 把 domain.Attacker 转成可下发的结构。
//
// ⚠️ 逐字段列举，刻意不用循环或反射：
// 新增 Attacker 字段时若忘了加到这里，客户端就拿不到那一项，
// 而**编译与测试都不会报**（JSON 少一个字段是合法的）。
// 守这一条靠 TestAttackerViewCoversEveryField（用反射比对字段名）。
func toView(a domain.Attacker) AttackerView {
	return AttackerView{
		Attack:                 a.Attack,
		CritPermille:           a.CritPermille,
		CritMultiplierPermille: a.CritMultiplierPermille,
		ReactionMultPermille:   a.ReactionMultPermille,
		ElementCap:             a.ElementCap,
		ReactionTier:           a.ReactionTier,
		ElementCoefPermille:    a.ElementCoefPermille,
		HeatCapPermille:        a.HeatCapPermille,
		ArmorPermille:          a.ArmorPermille,
		MechanicPermille:       a.MechanicPermille,
	}
}

// computeAttacker 从玩家进度与构筑推导攻方属性。
//
// 设计取舍：这里的公式是**故意简单**的线性成长，不是复杂曲线。
// 理由是这类公式一旦复杂，客户端与服务端各自实现一遍就必然漂移，
// 而 I-6 要求两边逐位一致。简单公式 + 服务端下发 = 唯一真相源。
//
// 成长维度：
//   - Attack:            随最高通关关卡线性增长（玩得越深打得越远）
//   - CritPermille:      固定小额，避免把暴击做成主要输出手段
//   - ElementCap:        随关卡缓慢提升，但封顶 6 层
//   - ElementCoef:       随专精投入提升，封顶 +600‰
//   - ReactionMult:      随 reaction_mult 类专精节点提升，封顶 +800‰
//   - ReactionTier:      随 reaction_mult 节点提升，封顶 4 阶
//   - CritPermille(覆盖): 专精 crit 节点与 gem 叠加后再封顶 600‰
//
// ⚠️ 后三项之前是**硬编码常量**，于是专精树里 24 个节点中的 6 个
// reaction_mult 节点、以及全部 crit 节点都只写不读 —— 玩家投入了却
// 看不到任何效果。现在它们都从专精树真实取值。
//
// 封顶值是刻意保守的：这些乘区一旦无界，I-1 的"搭配正确 > 堆面板"
// 就会被"无脑堆单一维度"打破。
func (s *Service) computeAttacker(ctx context.Context, userID int64) (domain.Attacker, error) {
	var maxStage int
	if err := s.pool.QueryRow(ctx,
		`SELECT COALESCE(MAX(max_stage), 0) FROM user_progress WHERE user_id = $1`,
		userID).Scan(&maxStage); err != nil {
		return domain.Attacker{}, fmt.Errorf("load max stage: %w", err)
	}

	stage := int64(maxStage)
	if stage < 0 {
		stage = 0
	}

	// 专精节点的实际加成。
	//
	// ⚠️ 这里**绝不能**调 s.LoadBuildSnapshot / s.computeRating：
	// 两者都会回调本函数（build 快照里要含 attacker），
	// 于是 computeAttacker → LoadBuildSnapshot → computeAttacker → …
	// 无限递归。表现为测试跑满超时后 panic，且栈里全是这两个函数互相调用 ——
	// 这个坑很隐蔽，因为「让攻方属性反映构筑」的需求看起来很自然。
	//
	// 攻方属性只需要专精节点，所以直接读那一项数据。
	masteryNodes, err := s.loadMasteryNodes(ctx, userID)
	if err != nil {
		return domain.Attacker{}, fmt.Errorf("load mastery nodes: %w", err)
	}
	eff := domain.MasteryEffect{}
	if len(masteryNodes) > 0 {
		selected := make(map[int]bool, len(masteryNodes))
		for _, n := range masteryNodes {
			selected[n] = true
		}
		points := 0
		if err := s.pool.QueryRow(ctx,
			`SELECT mastery_points FROM user_progress WHERE user_id = $1`,
			userID).Scan(&points); err != nil {
			points = 0
		}
		allNodes := make([]domain.MasteryNode, 0, 96)
		for _, f := range domain.AllMasteryFamilies() {
			allNodes = append(allNodes, f.Nodes...)
		}
		if e, err := domain.EvaluateMastery(allNodes, selected, points); err == nil {
			eff = e
		}
	}

	mastery := masteryNodes

	a := domain.DefaultAttacker()
	// 攻击力：每通过 10 关 +200‰，即 1 关 +20‰。第 100 关约 +2000‰。
	a.Attack += stage * 20
	// 元素层数上限：每 25 关 +1
	a.ElementCap += stage / 25
	// 专精 element_cap 节点：每层 +1
	if eff.ElementCapBonus > 0 {
		a.ElementCap += eff.ElementCapBonus
	}
	// 这里先钳一次，装配（宝石 element_cap）之后再钳一次 ——
	// 见函数末尾。两处都用同一个常量。
	if a.ElementCap > domain.MaxElementCap {
		a.ElementCap = domain.MaxElementCap
	}
	// 元素系数：每个专精节点 +15‰，封顶 +600‰
	// 这一项刻意强于攻击力成长 —— 它直接放大反应伤害，符合 I-1 的反通胀意图
	coef := int64(len(mastery)) * 15
	if coef > 600 {
		coef = 600
	}
	a.ElementCoefPermille += coef

	// ⚠️ 以下四项此前**从未被装配** —— `MasteryEffect` 累加了它们，
	// 而这里没有把它们加进攻方。于是专精树里：
	//   第 1/2 层槽 2「热量上限」  8 系 × 2 层 = 16 个节点 → 纯装饰
	//   第 1 层槽 3「技能伤害」    8 系 × 1 层 =  8 个节点 → 纯装饰
	//   第 3 层槽 0「额外插槽」    8 系 × 1 层 =  8 个节点 → 纯装饰（见下）
	//   第 3 层槽 2「护甲」        8 系 × 1 层 =  8 个节点 → 纯装饰
	//   第 3 层槽 1「机制改造」    8 系 × 1 层 =  8 个节点 → 连 case 都没有
	// 合计 40/96 个节点是惰性的。
	if eff.SkillDamageBonus > 0 {
		// 刻意折进 Attack 而不是单开乘区 ——
		// Attack 是 I-1 反通胀公式的攻击力侧，单开乘区就绕过了 30‰ 红线。
		a.Attack += eff.SkillDamageBonus
	}
	if eff.HeatCapBonus > 0 {
		a.HeatCapPermille += eff.HeatCapBonus
	}
	if eff.ArmorBonus > 0 {
		a.ArmorPermille += eff.ArmorBonus
	}
	if eff.MechanicBonus > 0 {
		a.MechanicPermille += eff.MechanicBonus
	}
	// ⚠️ eff.ExtraSlots 仍然不装配。
	//
	// 「额外插槽」要生效，改的不是攻方属性，而是**技能槽位上限** ——
	// 它由客户端的 ACTIVE_SLOTS / PASSIVE_SLOT 决定，服务端也需要一份
	// 并据此校验上报的 equipped 数量。跨端 + 校验两层改动，
	// 且会改变已有构筑的合法形状，单独作为一个批次做（见 README「未做」）。

	// 反应倍率：来自 reaction_mult 类专精节点，封顶 +800‰（2 倍上限之下）
	//
	// ⚠️ 收紧了 domain 侧的 `wE/((1-w)·M)` 上限（见 damage.go），
	// 所以这里即使给到 2 倍，反应伤害里攻击力的占比也不会越过 30% 红线。
	// 不收紧上限就放大倍率，等于给专精树开了一扇绕过 I-1 的后门。
	rm := eff.ReactionMultBonus
	if rm > 800 {
		rm = 800
	}
	if rm < 0 {
		rm = 0
	}
	a.ReactionMultPermille += rm

	// 反应阶：每 3 个 reaction_mult 节点提升 1 阶，封顶 4 阶
	// 阶数放大的是「元素侧」（与养成完全无关的那一段），
	// 所以提高它不会让攻击力占比上升 —— 这是 I-1 允许的成长方向。
	rt := int64(1) + (rm/domain.MasteryReactionMultPerNode)/3
	if rt > 4 {
		rt = 4
	}
	a.ReactionTier = rt

	// 暴击率：来自 crit 类专精节点，封顶 +500‰（即总上限 550‰）
	cp := eff.CritBonus
	if cp > 500 {
		cp = 500
	}
	if cp < 0 {
		cp = 0
	}
	a.CritPermille = 50 + cp
	a.CritMultiplierPermille = 1500

	// --- 装备与宝石 ---
	//
	// ⚠️ 此前**完全没有这一段**：18 件装备与 8 种宝石对战斗数值零影响。
	// 它们出现在 `/config`（玩家看得见）与 I-7 构筑评分（分数会变），
	// 但没有一个数字进到战斗里 —— 玩家花资源升级武器看不到任何变化。
	//
	// 封顶在 applyLoadout 里做，且必须在专精贡献**之后** ——
	// 否则"专精 800‰ + 装备 2000‰"能绕过单一来源的上限。
	loadout, err := s.loadLoadout(ctx, userID)
	if err != nil {
		return domain.Attacker{}, fmt.Errorf("load loadout: %w", err)
	}
	applyLoadout(&a, loadout)

	// 元素层数封顶 8 要在装配**之后**再钳一次：
	// 上面按 stage/25 与专精算完时钳过一次，但宝石的 element_cap 是后加的。
	if a.ElementCap > domain.MaxElementCap {
		a.ElementCap = domain.MaxElementCap
	}

	return a, nil
}
