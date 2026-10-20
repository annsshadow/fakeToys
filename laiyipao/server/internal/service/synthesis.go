package service

import (
	"context"
	"errors"
	"fmt"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/domain"
)

// 技能熔炼（合成系统）。
//
// # 它补的是哪条闭环
//
// 已知边界第 29 条：等级解锁落地后，18 个合成技能（`cmp_*`）仍无获取途径。
// `skill_recipes` 配方表 18 条早已在库里
// （`output_skill_id` / `a_skill_id` / `b_skill_id` / `output_tier`），
// 但**没有任何代码消费它** —— 消费侧（升级/装备）齐了、发放侧空着，
// 与第 25 条「闭环缺发放侧」同型。
//
// # 产品拍板的三件事（2026-10-20）
//
//  1. **材料熔炼掉**：A、B 两份材料被消耗，产出 C。
//     选它而不是「保留材料」：熔炼语义天然防重复合成 ——
//     材料没了就做不了第二次，不需要额外的「已合成过」标记列。
//  2. **设等级门槛**：产物技能的 `unlock_level`（30..62）作门槛，
//     玩家等级达标才能熔炼。等级口径只有一个定义（`domain.PlayerLevelDivisor`）。
//  3. **纯材料成本**：不消耗货币、不消耗道具 ——
//     避免引入道具型货币就必须走完的三段闭环
//     （消费端点 + 发放途径 + 客户端入口，见第 16 条）。
//
// # 四处容易写错的地方
//
//  1. **权威侧是库表，不是 /config 的 Go 常量**。
//     `/config` 的 `recipes` 下发的是 `domain.SeedRecipes`（进程内静态表），
//     而后台 `PUT /admin/skills/:id` 会改库 —— 两者会漂（历史上漂过）。
//     判定必须查 `skill_recipes` 表；客户端拿 `/config` 只为**展示**。
//  2. **配方按 (a,b) 有序唯一**，而玩家可能反着选 —— 查表用
//     LEAST/GREATEST 规范化（否则 (2,1) 查不到配方，症状是「明明有配方却说没有」）。
//  3. **熔炼后必须清槽位**。`user_skill_slots.skill_id` 有 FK 且 NOT NULL，
//     不能置 0；材料行删掉后槽位若还指着它，就是**幽灵槽位** ——
//     引擎按 `user_skills ⋈ user_skill_slots` 统计，会静默少一个可用技能
//     （同类形状见工程约束 19）。
//  4. **材料必须同时拥有**，且删除要**条件删除 + 检查 RowsAffected** ——
//     与 `ClaimTask` 的 `claimed_at IS NULL`、`Buy` 的 `FOR UPDATE` 同一模式：
//     并发两个请求用同一对材料时，必须只有一个成功。
//
// # 零货币
//
// 熔炼不花钱，于是它**不写 wallet_flows**；只写审计日志
// （与 `SaveLoadout` 同一张表），以便事后查「这个产物怎么来的」。

// SynthesizeSkills 把两个已拥有的材料技能熔炼成一个高阶技能，返回产物技能 id。
func (s *Service) SynthesizeSkills(ctx context.Context, userID int64, aID, bID int64) (int64, error) {
	if aID <= 0 || bID <= 0 {
		return 0, fmt.Errorf("%w: 技能 id 非法 %d/%d", ErrBadInput, aID, bID)
	}
	if aID == bID {
		return 0, fmt.Errorf("%w: 熔炼需要两个不同的技能", ErrBadInput)
	}
	lo, hi := aID, bID
	if lo > hi {
		lo, hi = hi, lo
	}

	var output int64
	err := s.DB.Tx(ctx, func(tx txType) error {
		// 1) 配方：权威侧是库表；用 LEAST/GREATEST 抹平顺序
		if err := tx.QueryRow(ctx, `
			SELECT output_skill_id FROM skill_recipes
			 WHERE LEAST(a_skill_id, b_skill_id) = $1
			   AND GREATEST(a_skill_id, b_skill_id) = $2`,
			lo, hi).Scan(&output); err != nil {
			if errors.Is(err, pgx.ErrNoRows) {
				return fmt.Errorf("%w: 技能 %d + %d 没有配方", ErrBadInput, lo, hi)
			}
			return fmt.Errorf("查配方: %w", err)
		}
		if output == lo || output == hi {
			return fmt.Errorf("%w: 配方 %d+%d 的产物与材料相同", ErrBadInput, lo, hi)
		}

		// 2) 等级门槛（唯一口径见 domain.PlayerLevelDivisor）
		var levelExp int64
		if err := tx.QueryRow(ctx,
			`SELECT level_exp FROM user_progress WHERE user_id = $1`, userID).Scan(&levelExp); err != nil {
			return fmt.Errorf("读等级: %w", err)
		}
		var need int
		if err := tx.QueryRow(ctx,
			`SELECT unlock_level FROM skills WHERE id = $1`, output).Scan(&need); err != nil {
			return fmt.Errorf("读产物等级门槛: %w", err)
		}
		if domain.PlayerLevel(levelExp) < need {
			return fmt.Errorf("%w: 熔炼产物需要 %d 级（当前 %d 级）",
				ErrBadInput, need, domain.PlayerLevel(levelExp))
		}

		// 3) 锁住材料行（按 id 排序锁，避免两个请求交叉持锁死锁）
		rows, err := tx.Query(ctx, `
			SELECT skill_id FROM user_skills
			 WHERE user_id = $1 AND skill_id IN ($2, $3)
			 ORDER BY skill_id FOR UPDATE`, userID, lo, hi)
		if err != nil {
			return fmt.Errorf("锁材料: %w", err)
		}
		owned := 0
		for rows.Next() {
			owned++
		}
		rows.Close()
		if err := rows.Err(); err != nil {
			return fmt.Errorf("读材料: %w", err)
		}
		if owned != 2 {
			return fmt.Errorf("%w: 材料技能 %d、%d 未全部拥有", ErrBadInput, lo, hi)
		}

		// 4) 产物不能已拥有（否则这次熔炼只是白烧材料）
		var exists bool
		if err := tx.QueryRow(ctx, `
			SELECT EXISTS (SELECT 1 FROM user_skills WHERE user_id = $1 AND skill_id = $2)`,
			userID, output).Scan(&exists); err != nil {
			return fmt.Errorf("查产物: %w", err)
		}
		if exists {
			return fmt.Errorf("%w: 已拥有产物技能 %d", ErrBadInput, output)
		}

		// 5) 条件删除材料：RowsAffected 必须是 2 —— 并发下这才是真判据
		tag, err := tx.Exec(ctx, `
			DELETE FROM user_skills
			 WHERE user_id = $1 AND skill_id IN ($2, $3)`, userID, lo, hi)
		if err != nil {
			return fmt.Errorf("删材料: %w", err)
		}
		if tag.RowsAffected() != 2 {
			return fmt.Errorf("%w: 材料已被其他请求熔炼", ErrBadInput)
		}

		// 6) 清槽位：材料行没了，槽位不能还指着它（幽灵槽位）
		if _, err := tx.Exec(ctx, `
			DELETE FROM user_skill_slots
			 WHERE user_id = $1 AND skill_id IN ($2, $3)`, userID, lo, hi); err != nil {
			return fmt.Errorf("清槽位: %w", err)
		}

		// 7) 发放产物（等级 1）
		if _, err := tx.Exec(ctx, `
			INSERT INTO user_skills (user_id, skill_id) VALUES ($1, $2)
			 ON CONFLICT (user_id, skill_id) DO NOTHING`, userID, output); err != nil {
			return fmt.Errorf("发放产物: %w", err)
		}

		// 8) 审计（零货币，故不写 wallet_flows）
		if _, err := tx.Exec(ctx, `
			INSERT INTO admin_audit_logs (admin_id, username, action, target, detail)
			 VALUES (NULL, $1, $2, $3, $4)`,
			fmt.Sprintf("user#%d", userID), "synthesize_skill",
			fmt.Sprintf("user#%d", userID),
			fmt.Sprintf("{\"a\":%d,\"b\":%d,\"output\":%d}", lo, hi, output)); err != nil {
			return fmt.Errorf("audit log: %w", err)
		}
		return nil
	})
	if err != nil {
		return 0, err
	}
	return output, nil
}
