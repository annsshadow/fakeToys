package service

import (
	"context"
	"fmt"
	"strings"
)

// MaxActiveSlots 是主动技能槽数量（I-2：4 主动 + 1 被动）。
const MaxActiveSlots = 4

// SaveLoadoutInput 是保存出战技能配置。
type SaveLoadoutInput struct {
	// SkillIDs 按槽位顺序排列，索引即 slot（0..MaxActiveSlots-1）。
	// 允许稀疏（中间为 0 表示空槽），但不允许重复。
	SkillIDs []int `json:"skill_ids"`
}

// SaveLoadout 保存出战技能配置。
//
// 为什么必须有这个接口（而不是只在前端存）：
//  1. 回放哈希依赖技能槽位。槽位存在哪、前端还是后端，
//     决定了重放方能否重建同一份战斗 —— 必须由服务端裁决。
//  2. 防线值守（I-5）会用「当前出战配置」冻结快照，同样需要权威来源。
//
// 槽位变化会影响回放哈希，因此这里只允许**显式调用**时变更，
// 且每次变更都会写审计日志。
func (s *Service) SaveLoadout(ctx context.Context, userID int64, in SaveLoadoutInput) ([]int, error) {
	// 1) 校验：长度、重复、越界
	if len(in.SkillIDs) > MaxActiveSlots {
		return nil, fmt.Errorf("%w: 最多 %d 个主动技能槽，收到 %d", ErrBadInput, MaxActiveSlots, len(in.SkillIDs))
	}
	seen := map[int]bool{}
	for i, id := range in.SkillIDs {
		if id == 0 {
			continue // 空槽
		}
		if seen[id] {
			return nil, fmt.Errorf("%w: 技能 %d 重复出现在多个槽位", ErrBadInput, id)
		}
		seen[id] = true
		_ = i
	}

	err := s.DB.Tx(ctx, func(tx txType) error {
		// 2) 技能必须已解锁且为主动技能
		for slot, id := range in.SkillIDs {
			if id == 0 {
				continue
			}
			var kind string
			err := tx.QueryRow(ctx,
				`SELECT s.kind FROM user_skills us
				 JOIN skills s ON s.id = us.skill_id
				 WHERE us.user_id = $1 AND us.skill_id = $2`, userID, id).Scan(&kind)
			if err != nil {
				return fmt.Errorf("%w: 技能 %d 未解锁", ErrBadInput, id)
			}
			if kind != "active" {
				return fmt.Errorf("%w: 技能 %d 是被动，不占主动槽", ErrBadInput, id)
			}
			_ = slot
		}

		// 3) 全量替换槽位
		if _, err := tx.Exec(ctx, `DELETE FROM user_skill_slots WHERE user_id = $1`, userID); err != nil {
			return fmt.Errorf("clear slots: %w", err)
		}
		for slot, id := range in.SkillIDs {
			if id == 0 {
				continue
			}
			if _, err := tx.Exec(ctx,
				`INSERT INTO user_skill_slots (user_id, slot, skill_id) VALUES ($1,$2,$3)`,
				userID, slot, id); err != nil {
				return fmt.Errorf("insert slot %d: %w", slot, err)
			}
		}

		// 4) 审计：槽位变更必须可追溯（它直接影响回放哈希的可复现性）。
		// admin_id = 0 表示玩家自助操作，不是管理员操作。
		ids := make([]string, 0, len(in.SkillIDs))
		for _, id := range in.SkillIDs {
			ids = append(ids, fmt.Sprintf("%d", id))
		}
		if _, err := tx.Exec(ctx,
			`INSERT INTO admin_audit_logs (admin_id, username, action, target, detail)
			 VALUES (0, $1, $2, $3, $4)`,
			fmt.Sprintf("user#%d", userID), "update_loadout",
			fmt.Sprintf("user#%d", userID), "slots="+strings.Join(ids, ",")); err != nil {
			return fmt.Errorf("audit log: %w", err)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}
	return in.SkillIDs, nil
}

// Loadout 返回当前出战槽位（按 slot 升序，空槽填 0）。
func (s *Service) Loadout(ctx context.Context, userID int64) ([]int, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT slot, skill_id FROM user_skill_slots WHERE user_id = $1 ORDER BY slot`, userID)
	if err != nil {
		return nil, fmt.Errorf("load slots: %w", err)
	}
	defer rows.Close()

	out := make([]int, MaxActiveSlots)
	for rows.Next() {
		var slot, id int
		if err := rows.Scan(&slot, &id); err != nil {
			return nil, err
		}
		if slot >= 0 && slot < MaxActiveSlots {
			out[slot] = id
		}
	}
	return out, rows.Err()
}
