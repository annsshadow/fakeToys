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
//   - Attack:        随最高通关关卡线性增长（玩得越深打得越远）
//   - CritPermille:  固定小额，避免把暴击做成主要输出手段
//   - ElementCap:    随关卡缓慢提升，但封顶 6 层
//   - ElementCoef:   这是关键 —— 元素系数只随专精投入与关卡小幅提升，
//     保证「搭配正确的玩家」始终强于「只堆面板的玩家」（I-1 反通胀）
func (s *Service) computeAttacker(ctx context.Context, userID int64) (domain.Attacker, error) {
	var maxStage int
	if err := s.pool.QueryRow(ctx,
		`SELECT COALESCE(MAX(max_stage), 0) FROM user_progress WHERE user_id = $1`,
		userID).Scan(&maxStage); err != nil {
		return domain.Attacker{}, fmt.Errorf("load max stage: %w", err)
	}

	var masteryPicked int
	if err := s.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM user_mastery_nodes WHERE user_id = $1`,
		userID).Scan(&masteryPicked); err != nil {
		return domain.Attacker{}, fmt.Errorf("load mastery count: %w", err)
	}

	stage := int64(maxStage)
	if stage < 0 {
		stage = 0
	}
	mastery := int64(masteryPicked)
	if mastery < 0 {
		mastery = 0
	}

	a := domain.DefaultAttacker()
	// 攻击力：每通过 10 关 +200‰，即 1 关 +20‰。第 100 关约 +2000‰。
	a.Attack += stage * 20
	// 元素层数上限：每 25 关 +1，封顶 6
	a.ElementCap += stage / 25
	if a.ElementCap > 6 {
		a.ElementCap = 6
	}
	// 元素系数：每个专精点 +15‰，封顶 +600‰
	// 这一项刻意强于攻击力成长 —— 它直接放大反应伤害，符合 I-1 的反通胀意图
	coef := mastery * 15
	if coef > 600 {
		coef = 600
	}
	a.ElementCoefPermille += coef
	// 暴击率不随进度提升：暴击是锦上添花，不能成为主要胜负手
	a.CritPermille = 50
	a.CritMultiplierPermille = 1500
	a.ReactionMultPermille = 1000
	a.ReactionTier = 1
	return a, nil
}
