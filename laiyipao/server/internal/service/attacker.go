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
// loadMasteryEffect 读专精树并算出合计效果。
//
// ⚠️ 这里**绝不能**调 s.LoadBuildSnapshot / s.computeRating：
// 两者都会回调 computeAttacker（build 快照里要含 attacker），
// 于是 computeAttacker → LoadBuildSnapshot → computeAttacker → …
// 无限递归。表现为测试跑满超时后 panic，且栈里全是这两个函数互相调用 ——
// 这个坑很隐蔽，因为「让攻方属性反映构筑」的需求看起来很自然。
//
// 抽出独立函数是为了让 `ExtraSlots`（额外插槽）与 `computeAttacker`
// 共用同一份读取逻辑：两处各自实现一遍的话，改一处忘另一处就会
// 让「槽位数」与「攻方加成」对不上，且没有任何测试会发现。
func (s *Service) loadMasteryEffect(ctx context.Context, userID int64) (domain.MasteryEffect, []int, error) {
	nodes, err := s.loadMasteryNodeRows(ctx, userID)
	if err != nil {
		return domain.MasteryEffect{}, nil, err
	}
	points := 0
	// ⚠️ 与旧实现同一静默语义：点数读不到 = 无加点（0），不让整局打不开。
	// 只在有节点时才读点数（旧实现在 len(nodes)==0 时提前返回、不读）。
	if len(nodes) > 0 {
		if err := s.pool.QueryRow(ctx,
			`SELECT mastery_points FROM user_progress WHERE user_id = $1`,
			userID).Scan(&points); err != nil {
			points = 0
		}
	}
	// 求值走 build_parts.go 的共享纯函数（与 LoadBuildSnapshot 同一份降级语义）。
	return masteryEffectFrom(nodes, points), nodes, nil
}

// ExtraSlots 返回专精「额外插槽」提供的额外槽位数。
//
// 专精树第 3 层槽 0 共 8 个节点此前**完全惰性**：
// 客户端用编译期常量 ACTIVE_SLOTS = 4，根本不读服务端下发的 base_slots，
// 所以玩家点出来的槽位在战斗里不存在。
func (s *Service) ExtraSlots(ctx context.Context, userID int64) (int64, error) {
	eff, _, err := s.loadMasteryEffect(ctx, userID)
	if err != nil {
		return 0, fmt.Errorf("load mastery: %w", err)
	}
	// 取「额外插槽」的判定走 build_parts.go 的共享纯函数（负数按 0）。
	return extraSlotsFrom(eff), nil
}

// computeAttacker 从玩家进度与构筑推导攻方属性。
//
// 装配逻辑（专精/装备/宝石 → 各乘区 + 封顶）全部在 build_parts.go 的
// `attackerFromParts` 里 —— 本函数只负责「取」，取完直接走共享纯计算。
// 这样 `LoadBuildSnapshot`（走 loadBuildParts）与直接调用 computeAttacker
// 的两条路径算出**同一份**攻方属性，不会出现「两处各算一遍然后漂移」。
func (s *Service) computeAttacker(ctx context.Context, userID int64) (domain.Attacker, error) {
	parts, err := s.loadBuildParts(ctx, userID)
	if err != nil {
		return domain.Attacker{}, err
	}
	return attackerFromParts(parts), nil
}
