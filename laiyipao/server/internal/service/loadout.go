package service

import (
	"context"
	"fmt"
	"strings"

	"github.com/laiyipao/server/internal/domain"
)

// MaxActiveSlots 是主动技能槽数量（I-2：4 主动 + 1 被动）。
const MaxActiveSlots = 4

// skillRules 是技能升级规则的包级实例。
//
// ⚠️ 这里**不再定义**等级上限、系数、费用 —— 那些在
// `domain/skillrules.go`（`domain.DefaultSkillRules()`）。
//
// 为什么不留在 service：费用与系数都必须让客户端看见
// （按钮要显示费用、战斗要算伤害），而 `cmd/vectors` 只能导出 domain。
// 规则放 domain 才能进契约夹具、被跨端守卫比对 ——
// 这正是 `score_rules` 当初的做法（那次修的是 5 处跨端硬编码）。
// 在 service 和 TS 各留一份常量，就是同一个坑。
var skillRules = domain.DefaultSkillRules()

// MaxSkillLevel 是技能等级上限（转发 domain，供本包内使用）。
const MaxSkillLevel = domain.MaxSkillLevel

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

// UpgradeSkill 把一个已拥有的技能升一级，扣除金币。返回升级后的等级。
//
// ## 事务边界
//
// 「读等级 → 扣费 → 升级」必须在一个事务里，且扣费要**条件扣**。
// 分成两步的话，两个并发请求会都读到 level=1、都通过校验，
// 然后扣两次钱但只升一级。
// `grantWallet` 的负数分支用 `WHERE coin >= $3 RETURNING`，
// 余额不足时**不产生行**（`pgx.ErrNoRows`），所以「扣不到钱」这个分支
// 本身是原子的。
//
// ## 为什么要 `FOR UPDATE`
//
// 没有它，两个并发请求都读到 level=9、都算出费用 900、
// 都通过「未满级」校验、都扣钱，结果一个升到 10、另一个升到 11 ——
// 而 11 超过上限。扣费是原子的，但**读-判-写整体不是**。
//
// 升级时的 UPDATE 额外带 `AND level < $3` 做二次保险。
func (s *Service) UpgradeSkill(ctx context.Context, userID, skillID int64) (int, error) {
	if skillID <= 0 {
		return 0, fmt.Errorf("%w: 技能 id 非法 %d", ErrBadInput, skillID)
	}
	var newLevel int
	err := s.DB.Tx(ctx, func(tx txType) error {
		var level int
		if err := tx.QueryRow(ctx,
			`SELECT level FROM user_skills WHERE user_id = $1 AND skill_id = $2 FOR UPDATE`,
			userID, skillID).Scan(&level); err != nil {
			return fmt.Errorf("%w: 技能 %d 未拥有", ErrBadInput, skillID)
		}
		if level >= skillRules.MaxLevel {
			return fmt.Errorf("%w: 技能 %d 已是满级 %d", ErrBadInput, skillID, skillRules.MaxLevel)
		}
		if err := s.grantWallet(ctx, tx, userID,
			map[string]int64{"coin": -skillRules.CostFrom(level)},
			"skill_upgrade", skillID); err != nil {
			return err
		}
		if err := tx.QueryRow(ctx,
			`UPDATE user_skills SET level = level + 1
			 WHERE user_id = $1 AND skill_id = $2 AND level < $3
			 RETURNING level`,
			userID, skillID, skillRules.MaxLevel).Scan(&newLevel); err != nil {
			return fmt.Errorf("%w: 升级失败（等级可能已被并发推满）", ErrBadInput)
		}
		return nil
	})
	if err != nil {
		return 0, err
	}
	return newLevel, nil
}

// SkillLevels 返回玩家所有已拥有技能的等级（skill_id → level）。
//
// 客户端必须知道每个技能的等级才能算对伤害，而 I-6 要求重放时用**同一份**等级。
// 超出 [1, MaxLevel] 的值在此夹紧 —— 脏数据不该变成
// 一个客户端与服务端解释不同的伤害。
func (s *Service) SkillLevels(ctx context.Context, userID int64) (map[int]int, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT skill_id, level FROM user_skills WHERE user_id = $1`, userID)
	if err != nil {
		return nil, fmt.Errorf("load skill levels: %w", err)
	}
	defer rows.Close()
	out := make(map[int]int)
	for rows.Next() {
		var id, level int
		if err := rows.Scan(&id, &level); err != nil {
			return nil, err
		}
		out[id] = skillRules.ClampLevel(level)
	}
	return out, rows.Err()
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
