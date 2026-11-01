package service

// L-3 阵型预设（第 48 轮接线）。
//
// `user_presets` 表（00003_progression.sql L-3）自建表以来**零代码引用**：
// 玩家每次换构筑都要手动重配技能/装备/宝石，多套配装无法一键切换。
// 本文件提供预设的存取面：
//   - SaveCurrentPreset 读取玩家**当前构筑**（技能槽/已装备/已佩戴宝石）
//     固化为预设；
//   - ListPresets / DeletePreset / SetCurrentPreset 管理预设集合。
//
// 「应用预设」由客户端消费 slots 后复用现有写端点（SaveLoadout /
// SetEquippedEquipment / EquipGem）完成 —— 服务端只存数据，不重复实现
// 三套写入逻辑。

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/jackc/pgx/v5"
)

// PresetSlots 是一份阵型预设的完整构筑快照（应用时客户端逐项消费）。
type PresetSlots struct {
	// Skills 是已出战技能 id，按槽位升序（0..MaxActiveSlots-1）。
	Skills []int `json:"skills"`
	// Equipment 是已装配装备 id，按槽位升序。
	Equipment []int `json:"equipment"`
	// Gems 是已佩戴宝石的实例 id，按槽位升序。
	Gems []int `json:"gems"`
}

// PresetView 是玩家可见的预设。
type PresetView struct {
	ID        int64       `json:"id"`
	Name      string      `json:"name"`
	Slots     PresetSlots `json:"slots"`
	IsCurrent bool        `json:"is_current"`
	CreatedAt time.Time   `json:"created_at"`
}

// maxPresetsPerUser 是每人预设上限（防 JSONB 无界堆积）。
const maxPresetsPerUser = 20

// currentPresetSlots 读取玩家当前构筑（技能槽/已装备/已佩戴宝石）。
// 三个查询各自 fail-loud；顺序固定为 技能 → 装备 → 宝石。
func (s *Service) currentPresetSlots(ctx context.Context, userID int64) (PresetSlots, error) {
	var out PresetSlots

	rows, err := s.pool.Query(ctx,
		`SELECT skill_id FROM user_skill_slots WHERE user_id = $1 ORDER BY slot`, userID)
	if err != nil {
		return out, fmt.Errorf("load preset skills: %w", err)
	}
	defer rows.Close()
	for rows.Next() {
		var id int
		if err := rows.Scan(&id); err != nil {
			return out, fmt.Errorf("scan preset skills: %w", err)
		}
		out.Skills = append(out.Skills, id)
	}
	if err := rows.Err(); err != nil {
		return out, fmt.Errorf("iterate preset skills: %w", err)
	}

	erows, err := s.pool.Query(ctx,
		`SELECT equipment_id FROM user_equipment WHERE user_id = $1 AND equipped ORDER BY slot`, userID)
	if err != nil {
		return out, fmt.Errorf("load preset equipment: %w", err)
	}
	defer erows.Close()
	for erows.Next() {
		var id int
		if err := erows.Scan(&id); err != nil {
			return out, fmt.Errorf("scan preset equipment: %w", err)
		}
		out.Equipment = append(out.Equipment, id)
	}
	if err := erows.Err(); err != nil {
		return out, fmt.Errorf("iterate preset equipment: %w", err)
	}

	grows, err := s.pool.Query(ctx,
		`SELECT id FROM user_gems WHERE user_id = $1 AND equipped_slot <> '' ORDER BY equipped_slot`, userID)
	if err != nil {
		return out, fmt.Errorf("load preset gems: %w", err)
	}
	defer grows.Close()
	for grows.Next() {
		var id int64
		if err := grows.Scan(&id); err != nil {
			return out, fmt.Errorf("scan preset gems: %w", err)
		}
		out.Gems = append(out.Gems, int(id))
	}
	if err := grows.Err(); err != nil {
		return out, fmt.Errorf("iterate preset gems: %w", err)
	}
	return out, nil
}

// ListPresets 返回玩家全部预设（按创建时间升序）。
func (s *Service) ListPresets(ctx context.Context, userID int64) ([]PresetView, error) {
	rows, err := s.pool.Query(ctx, `
		SELECT id, name, slots, is_current, created_at
		  FROM user_presets
		 WHERE user_id = $1
		 ORDER BY created_at, id`, userID)
	if err != nil {
		return nil, fmt.Errorf("list presets: %w", err)
	}
	defer rows.Close()

	out := []PresetView{}
	for rows.Next() {
		var v PresetView
		var raw []byte
		if err := rows.Scan(&v.ID, &v.Name, &raw, &v.IsCurrent, &v.CreatedAt); err != nil {
			return nil, fmt.Errorf("scan preset: %w", err)
		}
		if err := json.Unmarshal(raw, &v.Slots); err != nil {
			// fail-loud：预设 slots 由本文件写入，坏 JSONB 说明数据被外部改坏。
			return nil, fmt.Errorf("preset %d slots 非法 jsonb: %w", v.ID, err)
		}
		out = append(out, v)
	}
	return out, rows.Err()
}

// SaveCurrentPreset 把玩家当前构筑固化为预设。
// 首个预设自动设为 current（部分唯一索引允许每用户一条 current）。
func (s *Service) SaveCurrentPreset(ctx context.Context, userID int64, name string) (PresetView, error) {
	if name == "" {
		return PresetView{}, fmt.Errorf("%w: 预设名不能为空", ErrBadInput)
	}
	if len([]rune(name)) > 20 {
		return PresetView{}, fmt.Errorf("%w: 预设名最多 20 字", ErrBadInput)
	}
	slots, err := s.currentPresetSlots(ctx, userID)
	if err != nil {
		return PresetView{}, err
	}
	raw, err := json.Marshal(slots)
	if err != nil {
		return PresetView{}, fmt.Errorf("marshal preset slots: %w", err)
	}

	var v PresetView
	err = s.DB.Tx(ctx, func(tx pgx.Tx) error {
		// 预设数上限（事务内查，防并发超限——同一用户并发保存预设的窗口极小）。
		var n int
		if err := tx.QueryRow(ctx,
			`SELECT COUNT(*) FROM user_presets WHERE user_id = $1`, userID).Scan(&n); err != nil {
			return fmt.Errorf("count presets: %w", err)
		}
		if n >= maxPresetsPerUser {
			return fmt.Errorf("%w: 最多保存 %d 个预设", ErrBadInput, maxPresetsPerUser)
		}
		var isCurrent bool
		if err := tx.QueryRow(ctx,
			`SELECT NOT EXISTS (SELECT 1 FROM user_presets WHERE user_id = $1 AND is_current)`,
			userID).Scan(&isCurrent); err != nil {
			return fmt.Errorf("check current preset: %w", err)
		}
		if err := tx.QueryRow(ctx, `
			INSERT INTO user_presets (user_id, name, slots, is_current)
			VALUES ($1, $2, $3::jsonb, $4)
			RETURNING id, name, slots, is_current, created_at`,
			userID, name, raw, isCurrent).Scan(&v.ID, &v.Name, &raw, &v.IsCurrent, &v.CreatedAt); err != nil {
			return fmt.Errorf("insert preset: %w", err)
		}
		return nil
	})
	if err != nil {
		return PresetView{}, err
	}
	if err := json.Unmarshal(raw, &v.Slots); err != nil {
		return PresetView{}, fmt.Errorf("preset %d slots 非法 jsonb: %w", v.ID, err)
	}
	return v, nil
}

// DeletePreset 删除一个预设（归属校验；删除 current 后不自动补位）。
func (s *Service) DeletePreset(ctx context.Context, userID int64, presetID int64) error {
	tag, err := s.pool.Exec(ctx,
		`DELETE FROM user_presets WHERE id = $1 AND user_id = $2`, presetID, userID)
	if err != nil {
		return fmt.Errorf("delete preset: %w", err)
	}
	if tag.RowsAffected() == 0 {
		return fmt.Errorf("%w: 预设 %d 不存在", ErrNotFound, presetID)
	}
	return nil
}

// SetCurrentPreset 把某预设设为当前（清掉其它 current 后置位，事务内）。
func (s *Service) SetCurrentPreset(ctx context.Context, userID int64, presetID int64) (PresetView, error) {
	var v PresetView
	var raw []byte
	err := s.DB.Tx(ctx, func(tx pgx.Tx) error {
		if _, err := tx.Exec(ctx,
			`UPDATE user_presets SET is_current = FALSE WHERE user_id = $1 AND is_current`, userID); err != nil {
			return fmt.Errorf("clear current presets: %w", err)
		}
		if err := tx.QueryRow(ctx, `
			UPDATE user_presets SET is_current = TRUE
			 WHERE id = $2 AND user_id = $1
			 RETURNING id, name, slots, is_current, created_at`,
			userID, presetID).Scan(&v.ID, &v.Name, &raw, &v.IsCurrent, &v.CreatedAt); err != nil {
			if err == pgx.ErrNoRows {
				return fmt.Errorf("%w: 预设 %d 不存在", ErrNotFound, presetID)
			}
			return fmt.Errorf("set current preset: %w", err)
		}
		return nil
	})
	if err != nil {
		return PresetView{}, err
	}
	if err := json.Unmarshal(raw, &v.Slots); err != nil {
		return PresetView{}, fmt.Errorf("preset %d slots 非法 jsonb: %w", v.ID, err)
	}
	return v, nil
}
