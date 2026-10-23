package service

import (
	"context"
	"errors"
	"fmt"
	"sync"
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
//
// # 并发形状（2026-10-24）
//
// 曾经的 12 次往返全程串行（用户行 1 + LoadWallet 3 + LoadMastery 2 +
// Loadout 1 + equipment 1 + AdminListBattles 2 + verifications 1 +
// 流水 1）——后台每开一个玩家详情多等约 10 个 RTT。第 2)~7) 段互不依赖，
// 用 WaitGroup 并发（pgxpool 并发安全；唯一的写 regen 只碰 user_wallets，
// 与其余段读的表不重叠）。
//
// 错误语义保持不变：
//   - 用户行段**保持顺序执行**——它的 `ErrNotFound`→404 必须确定，
//     不能被其他段的错误抢先；
//   - 其余段的错误按**段序**收集、按段序优先返回——与改动前
//     「逐段串行返回第一个错误」完全一致，不因调度顺序改变用户可见的错误。
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

	// 2)~7) 并发段。段序 = 曾经的串行执行顺序；errs[i] 非 nil 即该段失败，
	// wg.Wait() 后按序优先返回——与串行版的「第一个错误」一致。
	var (
		wg            sync.WaitGroup
		wallet        Wallet
		nodes         []int
		skillIDs      []int
		equipmentIDs  []int
		battles       []AdminBattleView
		verifications []AdminVerificationView
		flowCount     int
		errs          [7]error
	)
	seg := func(i int, fn func()) {
		wg.Add(1)
		go func() {
			defer wg.Done()
			fn()
		}()
	}

	// 2) 钱包（含体力 regen 写——与其余段读的表不重叠，可并发）
	seg(0, func() {
		var err error
		wallet, err = s.LoadWallet(ctx, userID)
		if err != nil {
			errs[0] = fmt.Errorf("load user wallet: %w", err)
		}
	})

	// 3) 专精节点
	seg(1, func() {
		var err error
		_, nodes, err = s.LoadMastery(ctx, userID)
		if err != nil {
			errs[1] = fmt.Errorf("load user mastery: %w", err)
		}
	})

	// 4) 出战槽位
	seg(2, func() {
		var err error
		skillIDs, err = s.Loadout(ctx, userID)
		if err != nil {
			errs[2] = fmt.Errorf("load user loadout: %w", err)
		}
	})

	// 5) 装备
	seg(3, func() {
		rows, err := s.pool.Query(ctx,
			`SELECT equipment_id FROM user_equipment WHERE user_id = $1 AND equipped ORDER BY slot`, userID)
		if err != nil {
			errs[3] = fmt.Errorf("load user equipment: %w", err)
			return
		}
		defer rows.Close()
		for rows.Next() {
			var id int
			if err := rows.Scan(&id); err != nil {
				errs[3] = err
				return
			}
			equipmentIDs = append(equipmentIDs, id)
		}
	})

	// 6) 最近战报
	seg(4, func() {
		var err error
		battles, _, err = s.AdminListBattles(ctx, userID, 0, 10, false, 0)
		if err != nil {
			errs[4] = fmt.Errorf("load user battles: %w", err)
		}
	})

	// 7) 最近验真记录
	seg(5, func() {
		vrows, err := s.pool.Query(ctx, `
		SELECT id, battle_id, verifier_id, admin_verifier_id,
		       expected_hash, actual_hash, matched, created_at
		FROM replay_verifications
		WHERE verifier_id = $1 OR battle_id IN (SELECT id FROM battle_records WHERE user_id = $1)
		ORDER BY id DESC LIMIT 10`, userID)
		if err != nil {
			errs[5] = fmt.Errorf("load user verifications: %w", err)
			return
		}
		defer vrows.Close()
		for vrows.Next() {
			var v AdminVerificationView
			if err := vrows.Scan(&v.ID, &v.BattleID, &v.VerifierID, &v.AdminID,
				&v.Expected, &v.Actual, &v.Matched, &v.CreatedAt); err != nil {
				errs[5] = err
				return
			}
			verifications = append(verifications, v)
		}
	})

	// 8) 流水条数
	seg(6, func() {
		if err := s.pool.QueryRow(ctx,
			`SELECT COUNT(*) FROM wallet_flows WHERE user_id = $1`, userID).Scan(&flowCount); err != nil {
			errs[6] = fmt.Errorf("count user flows: %w", err)
		}
	})

	wg.Wait()
	for _, err := range errs {
		if err != nil {
			return AdminUserDetailResp{}, err
		}
	}

	out.Wallet = map[string]int64{
		"coin":   wallet.Coin,
		"gem":    wallet.Gem,
		"energy": wallet.Energy,
		"keys":   wallet.Keys,
	}
	out.MasteryNodes = nodes
	out.EquippedSkills = skillIDs
	out.EquippedEquipment = equipmentIDs
	out.RecentBattles = battles
	out.Verifications = verifications
	out.FlowCount = flowCount

	return out, nil
}
