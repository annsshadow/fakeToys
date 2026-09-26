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
}

// toView 把 domain.Attacker 转成可下发的结构。
func toView(a domain.Attacker) AttackerView {
	return AttackerView{
		Attack:                 a.Attack,
		CritPermille:           a.CritPermille,
		CritMultiplierPermille: a.CritMultiplierPermille,
		ReactionMultPermille:   a.ReactionMultPermille,
		ElementCap:             a.ElementCap,
		ReactionTier:           a.ReactionTier,
		ElementCoefPermille:    a.ElementCoefPermille,
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
	if a.ElementCap > 8 {
		a.ElementCap = 8
	}
	// 元素系数：每个专精节点 +15‰，封顶 +600‰
	// 这一项刻意强于攻击力成长 —— 它直接放大反应伤害，符合 I-1 的反通胀意图
	coef := int64(len(mastery)) * 15
	if coef > 600 {
		coef = 600
	}
	a.ElementCoefPermille += coef

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

	return a, nil
}
