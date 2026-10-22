package service

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/jackc/pgx/v5"
)

// AdminUserDetailResp 是后台「玩家详情」抽屉的响应（2026-10-21 P5-7）。
//
// # 为什么需要它
//
// 此前后台**只能看到用户列表的一行**：昵称、状态、战力、两个货币、验真计数。
// 而运营真正要回答的问题往往是：
//
//   - 「这个人最近打了什么？」→ 战报
//   - 「他的钱去哪了？」→ 钱包流水
//   - 「他的构筑是什么？为什么打不过第 40 关？」→ 构筑快照
//   - 「他被验真验出不匹配了吗？哪几局？」→ 验真明细
//
// 这四件事**都要单独去别处查**，而「去别处查」的第一步往往是
// 「先在列表里找到这个人」—— 于是运营只能靠 user_id 手工拼 URL。
//
// 一个**不能看的玩家详情**比没有这个功能更糟：它让运营以为「没有信息」
// 意味着「没有问题」。
type AdminUserDetailResp struct {
	// User 是列表那一行（含验真计数）。
	User AdminUserView `json:"user"`
	// Wallet 是当前余额（含道具）。
	Wallet map[string]int64 `json:"wallet"`
	// MasteryNodes 是已点亮的专精节点 id。
	MasteryNodes []int `json:"mastery_nodes"`
	// EquippedSkills 是出战槽位的技能 id（按槽位顺序，-1 = 空）。
	EquippedSkills []int `json:"equipped_skills"`
	// EquippedEquipment 是已装配的装备 id。
	EquippedEquipment []int `json:"equipped_equipment"`
	// RecentBattles 是最近 10 场战报。
	RecentBattles []AdminBattleView `json:"recent_battles"`
	// Verifications 是最近 10 条验真记录（谁验的、结论是什么）。
	Verifications []AdminVerificationView `json:"verifications"`
	// FlowCount 是钱包流水的条数（用于「有没有流水」这个问题的快答）。
	FlowCount int `json:"flow_count"`
}

// AdminVerificationView 是一条验真记录。
type AdminVerificationView struct {
	ID         int64     `json:"id"`
	BattleID   int64     `json:"battle_id"`
	VerifierID *int64    `json:"verifier_id"`
	AdminID    *int64    `json:"admin_verifier_id"`
	Expected   string    `json:"expected_hash"`
	Actual     string    `json:"actual_hash"`
	Matched    bool      `json:"matched"`
	CreatedAt  time.Time `json:"created_at"`
}

// AdminUserDetail 读一个玩家的详情。
func (s *Service) AdminUserDetail(ctx context.Context, userID int64) (AdminUserDetailResp, error) {
	out := AdminUserDetailResp{}

	// 1) 列表那一行
	//
	// ⚠️ 查询与 `AdminListUsers` **逐字同形**（同样的 JOIN 与同样的验真子查询）。
	// 另写一份就会漂 —— 而漂了的表现是「详情页的验真计数与列表页不一样」，
	// 运营会以为数据在变。
	var u AdminUserView
	err := s.pool.QueryRow(ctx, `
		SELECT u.id, u.nickname, u.is_guest, u.status,
		       COALESCE(p.max_stage, 0), COALESCE(p.level_exp, 0),
		       COALESCE(w.coin, 0), COALESCE(w.gem, 0),
		       u.created_at, u.last_login_at,
		       COALESCE(v.checked, 0), COALESCE(v.mismatched, 0)
		FROM users u
		LEFT JOIN user_progress p ON p.user_id = u.id
		LEFT JOIN user_wallets w ON w.user_id = u.id
		LEFT JOIN (
			SELECT br.user_id,
			       COUNT(*) AS checked,
			       COUNT(*) FILTER (WHERE NOT rv.matched) AS mismatched
			FROM replay_verifications rv
			JOIN battle_records br ON br.id = rv.battle_id
			GROUP BY br.user_id
		) v ON v.user_id = u.id
		WHERE u.id = $1`, userID).
		Scan(&u.ID, &u.Nickname, &u.IsGuest, &u.Status, &u.MaxStage,
			&u.Power, &u.Coin, &u.Gem, &u.CreatedAt, &u.LastLoginAt,
			&u.VerifyChecked, &u.VerifyMismatched)
	if err != nil {
		// ⚠️ 2026-10-21（二轮审计）：「不存在」与「查不了」必须分开。
		//
		// 此前一律包成普通错误 → `failErr` 不匹配任何哨兵 → **500**，
		// 于是「查一个不存在的用户」和「数据库挂了」返回同一种响应。
		//
		// 对照：同轮的 `AdminBattleDetail` 做了这个区分（运营端本来就能
		// 列全部战报，没有预言机要防），这里是对称的。
		if errors.Is(err, pgx.ErrNoRows) {
			return AdminUserDetailResp{}, fmt.Errorf("%w: 用户 %d 不存在", ErrNotFound, userID)
		}
		return AdminUserDetailResp{}, fmt.Errorf("load user detail: %w", err)
	}
	out.User = u

	// 2) 钱包
	wallet, err := s.LoadWallet(ctx, userID)
	if err != nil {
		return AdminUserDetailResp{}, fmt.Errorf("load user wallet: %w", err)
	}
	out.Wallet = map[string]int64{
		"coin":   wallet.Coin,
		"gem":    wallet.Gem,
		"energy": wallet.Energy,
		"keys":   wallet.Keys,
	}

	// 3) 专精节点
	_, nodes, err := s.LoadMastery(ctx, userID)
	if err != nil {
		return AdminUserDetailResp{}, fmt.Errorf("load user mastery: %w", err)
	}
	out.MasteryNodes = nodes

	// 4) 出战槽位 + 装备
	ids, err := s.Loadout(ctx, userID)
	if err != nil {
		return AdminUserDetailResp{}, fmt.Errorf("load user loadout: %w", err)
	}
	out.EquippedSkills = ids

	rows, err := s.pool.Query(ctx,
		`SELECT equipment_id FROM user_equipment WHERE user_id = $1 AND equipped ORDER BY slot`, userID)
	if err != nil {
		return AdminUserDetailResp{}, fmt.Errorf("load user equipment: %w", err)
	}
	for rows.Next() {
		var id int
		if err := rows.Scan(&id); err != nil {
			rows.Close()
			return AdminUserDetailResp{}, err
		}
		out.EquippedEquipment = append(out.EquippedEquipment, id)
	}
	rows.Close()

	// 5) 最近战报
	battles, _, err := s.AdminListBattles(ctx, userID, 0, 10, false, 0)
	if err != nil {
		return AdminUserDetailResp{}, fmt.Errorf("load user battles: %w", err)
	}
	out.RecentBattles = battles

	// 6) 最近验真记录
	vrows, err := s.pool.Query(ctx, `
		SELECT id, battle_id, verifier_id, admin_verifier_id,
		       expected_hash, actual_hash, matched, created_at
		FROM replay_verifications
		WHERE verifier_id = $1 OR battle_id IN (SELECT id FROM battle_records WHERE user_id = $1)
		ORDER BY id DESC LIMIT 10`, userID)
	if err != nil {
		return AdminUserDetailResp{}, fmt.Errorf("load user verifications: %w", err)
	}
	for vrows.Next() {
		var v AdminVerificationView
		if err := vrows.Scan(&v.ID, &v.BattleID, &v.VerifierID, &v.AdminID,
			&v.Expected, &v.Actual, &v.Matched, &v.CreatedAt); err != nil {
			vrows.Close()
			return AdminUserDetailResp{}, err
		}
		out.Verifications = append(out.Verifications, v)
	}
	vrows.Close()

	// 7) 流水条数
	if err := s.pool.QueryRow(ctx,
		`SELECT COUNT(*) FROM wallet_flows WHERE user_id = $1`, userID).Scan(&out.FlowCount); err != nil {
		return AdminUserDetailResp{}, fmt.Errorf("count user flows: %w", err)
	}

	return out, nil
}
