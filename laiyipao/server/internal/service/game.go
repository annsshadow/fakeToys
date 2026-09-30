package service

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strconv"
	"strings"
	"time"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/domain"
)

// ErrSlotBudgetExceeded 装备的技能槽位数超过预算。
//
// 预算是「基础 5 槽 + 专精额外插槽」。此前**完全没有这个校验** ——
// 客户端可以把任意多个技能写进 user_skill_slots，服务端照单全收。
// 引擎会忽略 slot >= activeSlots 的技能，所以这不是刷分漏洞，
// 但它意味着「玩家有几个槽位」完全由客户端说了算，
// 而槽位正是专精树花点数换来的东西。
var ErrSlotBudgetExceeded = errors.New("装备槽位数超过预算")

// --- 战斗 ---

// BattleTokenResp 是开局凭证下发内容。
type BattleTokenResp struct {
	TokenID int64 `json:"token_id"`
	// Seed 必须是字符串：int64 超过 2^53 后 JSON number 在 JS 侧解析即失精，
	// 客户端拿到的种子与库里那个不是同一个数 → 回放哈希必然对不上（I-6 失效）。
	Seed      string         `json:"seed"`
	ExpiresAt time.Time      `json:"expires_at"`
	Level     LevelResp      `json:"level"`
	Revived   bool           `json:"revived"`
	Build     map[string]any `json:"build"`
}

// LevelResp 是关卡下发结构。与 domain.GeneratedLevel 字段一致，
// 但把 64 位种子转成字符串以规避 JS 精度损失。
type LevelResp struct {
	domain.GeneratedLevel
	// SeedStr 是字符串形态的关卡种子
	SeedStr string `json:"seed_str"`
}

// NewLevelResp 把生成器输出转成可安全下发的结构。
func NewLevelResp(gl domain.GeneratedLevel) LevelResp {
	gl.Seed = 0 // 数字形态不再下发，避免被误用
	return LevelResp{GeneratedLevel: gl, SeedStr: strconv.FormatInt(levelSeedOf(gl), 10)}
}

// levelSeedOf 从关卡 ID 重新推导出种子（与 domain.GenerateLevel 内部一致）。
func levelSeedOf(gl domain.GeneratedLevel) int64 {
	// GeneratedLevel.Seed 已被清零，这里用可复现的推导：
	// domain 内部 seed = uint64(0x5CA1AB1E) ^ (uint64(id) * phi) 折回 int64
	const phi uint64 = 0x9E3779B97F4A7C15
	return int64(uint64(0x5CA1AB1E) ^ (uint64(gl.ID) * phi))
}

// StartBattle 为一局战斗申请一次性凭证。
//
// 扣体力、生成种子、插入 token 三步在同一事务内 —— 任何一步失败都整体回滚，
// 避免出现"扣了体力没发凭证"或"发了凭证没扣体力"的不一致状态。
func (s *Service) StartBattle(ctx context.Context, userID int64, levelID int) (BattleTokenResp, error) {
	if levelID < 1 || levelID > domain.TotalLevels {
		return BattleTokenResp{}, fmt.Errorf("%w: 关卡 %d 不存在", ErrBadInput, levelID)
	}
	gl := domain.GenerateLevel(levelID)

	// 关卡必须已解锁
	var maxStage int
	if err := s.pool.QueryRow(ctx,
		`SELECT max_stage FROM user_progress WHERE user_id = $1`, userID).Scan(&maxStage); err != nil {
		if !errors.Is(err, pgx.ErrNoRows) {
			return BattleTokenResp{}, fmt.Errorf("load progress: %w", err)
		}
	}
	if levelID > maxStage+1 {
		return BattleTokenResp{}, fmt.Errorf("%w: 第 %d 关未解锁（当前最高 %d）", ErrForbidden, levelID, maxStage)
	}

	// 种子取自服务端 PRNG，客户端不得自选 —— 否则客户端可以刷出想要的随机序列
	seed, err := s.newBattleSeed(ctx)
	if err != nil {
		return BattleTokenResp{}, err
	}

	expires := time.Now().Add(s.Cfg.BattleTokenTTL)
	var tokenID int64
	err = s.DB.Tx(ctx, func(tx pgx.Tx) error {
		if err := tx.QueryRow(ctx,
			`INSERT INTO battle_tokens (user_id, level_id, seed, expires_at) VALUES ($1,$2,$3,$4)
			 RETURNING id`, userID, levelID, seed, expires).Scan(&tokenID); err != nil {
			return fmt.Errorf("insert battle token: %w", err)
		}
		if err := s.grantWallet(ctx, tx, userID,
			map[string]int64{"energy": -int64(gl.EnergyCost)}, "battle_start", tokenID); err != nil {
			return err
		}
		return nil
	})
	if err != nil {
		return BattleTokenResp{}, err
	}

	build, err := s.LoadBuildSnapshot(ctx, userID)
	if err != nil {
		return BattleTokenResp{}, err
	}
	return BattleTokenResp{
		TokenID:   tokenID,
		Seed:      strconv.FormatInt(seed, 10),
		ExpiresAt: expires,
		Level:     NewLevelResp(gl),
		Build:     build,
	}, nil
}

func (s *Service) newBattleSeed(ctx context.Context) (int64, error) {
	var seed int64
	// PostgreSQL 的 random() 足够随机且不需额外依赖；写库前用取模压到正数
	if err := s.pool.QueryRow(ctx,
		`SELECT abs(FLOOR(random() * 9223372036854775807))::bigint`).Scan(&seed); err != nil {
		return 0, fmt.Errorf("generate seed: %w", err)
	}
	return seed, nil
}

// SettleResp 是结算返回。
type SettleResp struct {
	BattleID    int64              `json:"battle_id"`
	Result      string             `json:"result"`
	Win         bool               `json:"win"`
	Score       int64              `json:"score"`
	Stars       int                `json:"stars"`
	Clamped     bool               `json:"clamped"`
	ClampNote   string             `json:"clamp_note,omitempty"`
	Loot        map[string]int64   `json:"loot"`
	Wallet      Wallet             `json:"wallet"`
	Power       int64              `json:"power"`
	Rating      domain.BuildRating `json:"build_rating"`
	NewMaxStage int                `json:"new_max_stage"`
}

// SettleBattle 结算一局战斗。
//
// 幂等性由 battle_token.used_at 保证：token 校验失败即拒绝，
// 同一个 token 无法结算两次。事务内同时写战报、盖 token、发掉落、推进进度。
func (s *Service) SettleBattle(ctx context.Context, userID, tokenID int64, in domain.SettleInput) (SettleResp, error) {
	var tok domain.BattleToken
	err := s.pool.QueryRow(ctx,
		`SELECT id, user_id, level_id, seed, issued_at, expires_at, used_at
		 FROM battle_tokens WHERE id = $1 AND user_id = $2`, tokenID, userID).
		Scan(&tok.ID, &tok.UserID, &tok.LevelID, &tok.Seed, &tok.IssuedAt, &tok.ExpiresAt, &tok.UsedAt)
	if errors.Is(err, pgx.ErrNoRows) {
		return SettleResp{}, fmt.Errorf("%w: battle token", ErrNotFound)
	}
	if err != nil {
		return SettleResp{}, fmt.Errorf("load battle token: %w", err)
	}

	gl := domain.GenerateLevel(tok.LevelID)
	limits := domain.SettleLimits{
		MinDurationMs:  int64(s.Cfg.MinBattleDuration / time.Millisecond),
		MaxScorePerSec: s.Cfg.MaxScorePerSecond,
	}
	res, err := domain.ValidateSettle(tok, gl, in, limits, time.Now())
	if err != nil {
		return SettleResp{}, fmt.Errorf("settle rejected: %w", err)
	}

	// ── replay_skills 语义比对（第 56 轮）──
	//
	// `replay_hash` 整体算不出来（它含战斗中选卡产生的 buff），
	// 但它前缀里的 **S 段**（技能槽配置）不含 buff，可以重算比对。
	//
	// 抓的是：伪造底伤（等价于假报技能等级）、上报另一套技能、槽位错位。
	//
	// ⚠️ 空串**放行**：老客户端不上报这个字段，直接拒会把它们全部锁死。
	// 「没上报」与「上报了但不对」要分开 —— 后者才是作弊。
	if in.ReplaySkills != "" {
		expect, err := s.replaySkillsSegment(ctx, userID)
		if err != nil {
			return SettleResp{}, fmt.Errorf("recompute replay skills: %w", err)
		}
		if in.ReplaySkills != expect {
			// ⚠️ 用 %q 而不是 %s：这两个串可能含大量数字，直接打出来会
			// 让人在日志里一眼扫过去 —— 而这正是排查这类问题最需要看清的东西。
			return SettleResp{}, fmt.Errorf(
				"%w：replay_skills 与构筑不符\n上报 %q\n重算 %q",
				ErrReplaySkillsMismatch, in.ReplaySkills, expect)
		}
	}

	// ⚠️ 三个上报集合**必须归一化 nil**，否则会写成 JSON `null`。
	//
	// `json.Marshal(map[string]int(nil))` 返回 `[]byte("null")`，
	// 存进 jsonb 列就是 jsonb `null`（注意不是 SQL NULL）。
	//
	// 后果有两条，都是实测过的：
	//
	//  1. **统计静默漏行**。看板的反应分布查询写的是
	//     `WHERE reactions_used <> '{}'::jsonb`，而
	//     `null <> '{}'` 在 SQL 里求值为 NULL（不是 true），
	//     所以这些行被排除。实测 58 行里有 8 行（13.8%）带 jsonb null。
	//
	//     ⚠️ **不要**拿「少统计了多少反应次数」来量化这个 bug。
	//     `reactions`（标量）与 `sum(reactions_used)`（map 之和）在引擎里
	//     是相邻两行自增（engine.ts 的 reactionsCount++ / reactionsUsed[k]++），
	//     本应恒等；但现有 58 行**全是 e2e 手搓的 payload**，两者互相矛盾
	//     （例：标量 90 / map 之和 53，全表 0 行相等）。
	//     拿它们相减得到的「漏了多少」是拿矛盾数据算出来的，没有意义。
	//     —— 这是我在本轮犯的第 13 次「前提不成立」。
	//  2. **任何 JSONB 函数调用会直接报错**。
	//     `jsonb_each_text(jsonb 'null')` 抛
	//     「不能在非对象上调用 jsonb_each_text」——
	//     将来谁写一条更合理的聚合 SQL，会当场崩掉。
	//
	// 触发条件不需要恶意：客户端省字段、第三方工具、老版本客户端
	// 都会走到这里。e2e 的「谎报星级」用例就是故意省掉这三个字段的。
	elementsRaw, _ := marshalJSON(orEmptyMap(in.ElementsUsed))
	reactionsRaw, _ := marshalJSON(orEmptyMap(in.ReactionsUsed))
	terrainRaw, _ := marshalJSON(orEmptySlice(in.TerrainUsed))

	// 选牌决策序列：I-6 重放闭环的最后一环。
	// 逗号分隔的紧凑文本，-1 表示该波跳过。
	picks := make([]string, 0, len(in.CardPicks))
	for _, p := range in.CardPicks {
		picks = append(picks, strconv.Itoa(p))
	}
	cardPicks := strings.Join(picks, ",")

	// ⚠️ build_snapshot 存的是**当时的玩家构筑**，不是 settle 上报原文。
	// 回放哈希由「关卡 + 种子 + 构筑」三者共同决定（见 I-6），
	// 若这里只存上报数据，客户端重放时拿不到技能与养成属性，
	// 算出的哈希必然对不上 —— 验真会把所有正常对局都判成伪造。
	buildSnapshot, err := s.LoadBuildSnapshot(ctx, userID)
	if err != nil {
		return SettleResp{}, fmt.Errorf("load build snapshot: %w", err)
	}
	// 上报原文另存一档，供审计与纠纷追溯
	buildSnapshot["settle_input"] = in
	buildRaw, _ := marshalJSON(buildSnapshot)
	lootRaw, _ := marshalJSON(res.Loot)

	resp := SettleResp{}
	err = s.DB.Tx(ctx, func(tx pgx.Tx) error {
		// 1) 抢占 token：条件更新保证并发下只有一个请求能成功
		tag, err := tx.Exec(ctx,
			`UPDATE battle_tokens SET used_at = now() WHERE id = $1 AND used_at IS NULL`, tokenID)
		if err != nil {
			return fmt.Errorf("claim token: %w", err)
		}
		if tag.RowsAffected() == 0 {
			return fmt.Errorf("%w: 该战斗凭证已被结算", ErrForbidden)
		}

		// 2) 写战报
		var battleID int64
		if err := tx.QueryRow(ctx, `
			INSERT INTO battle_records (user_id, level_id, battle_token_id, result, stars, score,
				kills, leaked, hp_left, wave_reached, duration_ms, shots, hits, reactions, heat_max,
				elements_used, reactions_used, terrain_used, build_snapshot, replay_hash, loot, card_picks)
			VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22)
			RETURNING id`,
			userID, gl.ID, tokenID, resultStr(res.Win), res.Stars, res.Score,
			in.Kills, in.Leaked, in.HPLeft, in.WaveReached, in.DurationMs,
			in.Shots, in.Hits, in.Reactions, in.HeatMax,
			elementsRaw, reactionsRaw, terrainRaw, buildRaw, in.ReplayHash, lootRaw, cardPicks,
		).Scan(&battleID); err != nil {
			return fmt.Errorf("insert battle record: %w", err)
		}

		// 3) 发掉落
		if err := s.grantWallet(ctx, tx, userID, res.Loot, "battle_reward", battleID); err != nil {
			return err
		}

		// 4) 推进进度与统计
		newMax, err := s.applyProgress(ctx, tx, userID, gl.ID, res.Win, int64(in.Kills), int64(in.Reactions))
		if err != nil {
			return err
		}

		// 5) 更新关卡星级（取历史最好）
		if _, err := tx.Exec(ctx, `
			INSERT INTO level_stars (user_id, level_id, stars, best_score, clears, min_power_clear)
			VALUES ($1,$2,$3,$4,$5,0)
			ON CONFLICT (user_id, level_id) DO UPDATE SET
				stars = GREATEST(level_stars.stars, EXCLUDED.stars),
				best_score = GREATEST(level_stars.best_score, EXCLUDED.best_score),
				clears = level_stars.clears + EXCLUDED.clears`,
			userID, gl.ID, res.Stars, res.Score, boolToInt(res.Win)); err != nil {
			return fmt.Errorf("upsert level stars: %w", err)
		}

		// 6) 任务进度
		if err := s.bumpTasks(ctx, tx, userID, map[string]int64{
			"kills":     int64(in.Kills),
			"clears":    int64(boolToInt(res.Win)),
			"reactions": int64(in.Reactions),
			"max_stage": int64(gl.ID),
		}, time.Now()); err != nil {
			return err
		}

		resp.BattleID = battleID
		resp.Result = resultStr(res.Win)
		resp.Win = res.Win
		resp.Score = res.Score
		resp.Stars = res.Stars
		resp.Clamped = res.Clamped
		resp.ClampNote = res.ClampReason
		resp.Loot = res.Loot
		resp.NewMaxStage = newMax
		return nil
	})
	if err != nil {
		return SettleResp{}, err
	}

	w, err := s.LoadWallet(ctx, userID)
	if err != nil {
		return SettleResp{}, err
	}
	resp.Wallet = w

	build, err := s.LoadBuildSnapshot(ctx, userID)
	if err != nil {
		return SettleResp{}, err
	}
	resp.Rating = s.computeRating(ctx, userID, build)
	resp.Power = s.computePower(ctx, userID, build)
	return resp, nil
}

func resultStr(win bool) string {
	if win {
		return "win"
	}
	return "lose"
}

func boolToInt(b bool) int {
	if b {
		return 1
	}
	return 0
}

// applyProgress 推进关卡进度与累计统计。
func (s *Service) applyProgress(ctx context.Context, tx pgx.Tx, userID int64, levelID int, win bool, kills, reactions int64) (int, error) {
	var newMax int
	err := tx.QueryRow(ctx, `
		UPDATE user_progress
		   SET max_stage = GREATEST(max_stage, CASE WHEN $2 THEN $3 ELSE max_stage END),
		       level_exp = level_exp + 50 + $4,
		       total_battles = total_battles + 1,
		       total_kills = total_kills + $4,
		       mastery_points = 3 + FLOOR((level_exp + 50 + $4) / 1000)::int,
		       updated_at = now()
		 WHERE user_id = $1
		 RETURNING max_stage`,
		userID, win, levelID, kills).Scan(&newMax)
	if err != nil {
		return 0, fmt.Errorf("update progress: %w", err)
	}
	return newMax, nil
}

// --- 构筑 ---

// BuildSnapshot 是一次构建的完整快照。
type BuildSnapshot struct {
	Skills      map[string]any `json:"skills"`
	Slots       []int          `json:"slots"`
	Equipment   []int          `json:"equipment"`
	Mastery     map[string]any `json:"mastery"`
	MasteryList []int          `json:"mastery_nodes"`
	Elements    []string       `json:"elements"`
	Works       []int          `json:"works"`
	Raw         map[string]any `json:"raw"`
}

// LoadBuildSnapshot 读取玩家当前构筑。
func (s *Service) LoadBuildSnapshot(ctx context.Context, userID int64) (map[string]any, error) {
	slots, elements, err := s.loadSkillsAndSlots(ctx, userID)
	if err != nil {
		return nil, err
	}
	equip, err := s.loadEquipment(ctx, userID)
	if err != nil {
		return nil, err
	}
	masteryNodes, err := s.loadMasteryNodes(ctx, userID)
	if err != nil {
		return nil, err
	}
	// 攻方属性由服务端权威下发。客户端绝不能自己猜 ——
	// 两端各算一套的话，回放哈希必然对不上，I-6 会变成永远失败的机制。
	att, err := s.computeAttacker(ctx, userID)
	if err != nil {
		return nil, err
	}
	// ⚠️ build 快照里 `skill_ids` 这个 key 是历史遗留的**误命名**：
	// 它装的其实是 loadSkillsAndSlots 返回的「元素名集合」（fire/ice/...），
	// 不是技能 id。而 /me/loadout 响应里的 `skill_ids` 是真的技能 id 数组 ——
	// 同名不同义，任何按名字推断语义的消费者都会踩坑。
	//
	// 这里同时下发两个 key：elements 是正确名，skill_ids 作为 deprecated
	// 别名保留，因为历史战报的 build_snapshot（存在 battle_records 里）
	// 只有旧名，改名会让那些战报无法重放。读取方一律走 buildElements()。
	// ⚠️ `active_slots` 与 `attacker` 放在一起下发，而不是让客户端另外查
	// `/mastery` —— 两者都是「服务端权威的战斗输入」，
	// 客户端绝不能自己算（I-6 的前提：两端各算一套哈希必然对不上）。
	// 放在同一个响应里也避免了战斗路径上多一次往返。
	extra, err := s.ExtraSlots(ctx, userID)
	if err != nil {
		return nil, err
	}
	return map[string]any{
		"snapshot_keys_version": 2,
		"skills":                slots,
		"elements":              elements,
		"skill_ids":             elements, // deprecated: 实际是元素名
		"equipment":             equip,
		"mastery_nodes":         masteryNodes,
		"attacker":              toView(att),
		"active_slots":          int(domain.BaseSkillSlots + extra),
	}, nil
}

// buildElements 从构筑快照里取元素名集合。
//
// 优先读新名 elements，回退到旧的误命名 skill_ids ——
// 这样历史战报（build_snapshot 是在改动之前写进库的）仍能算出构筑评分。
func buildElements(build map[string]any) []string {
	if v, ok := build["elements"].([]string); ok && len(v) > 0 {
		return v
	}
	if v, ok := build["skill_ids"].([]string); ok {
		return v
	}
	return nil
}

// checkSlotBudget 校验「已装备的槽位数」不超过预算。
//
// 预算 = domain.BaseSkillSlots（4 主动 + 1 被动）+ 专精「额外插槽」。
//
// ⚠️ 判定用**去重后的槽位个数**，不用技能条数：
// 多个不同 skill 写进同一 slot 仍然只占一个槽，
// 而按条数判会把「数据写错」误报成「超预算」。
func (s *Service) checkSlotBudget(ctx context.Context, userID int64, occupied map[int]bool) error {
	extra, err := s.ExtraSlots(ctx, userID)
	if err != nil {
		return err
	}
	budget := int(domain.BaseSkillSlots + extra)
	if n := len(occupied); n > budget {
		return fmt.Errorf(
			"%w：已装备 %d 个槽位，预算 %d 个（基础 %d + 专精额外 %d）",
			ErrSlotBudgetExceeded, n, budget, domain.BaseSkillSlots, extra,
		)
	}
	return nil
}

func (s *Service) loadSkillsAndSlots(ctx context.Context, userID int64) (map[string]any, []string, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT s.id, s.name, s.family, s.element, s.kind, COALESCE(us.level, 1),
		        COALESCE(sl.slot, -1)
		 FROM user_skills us
		 JOIN skills s ON s.id = us.skill_id
		 LEFT JOIN user_skill_slots sl ON sl.user_id = us.user_id AND sl.skill_id = us.skill_id
		 WHERE us.user_id = $1 ORDER BY s.id`, userID)
	if err != nil {
		return nil, nil, fmt.Errorf("load skills: %w", err)
	}
	defer rows.Close()

	skills := map[string]any{}
	elements := map[string]bool{}
	// occupied 收集**去重后**被占用的槽位。
	// slot = -1 表示"已拥有但未装备"，不占槽位。
	occupied := map[int]bool{}
	for rows.Next() {
		var id, level, slot int
		var name, family, element, kind string
		if err := rows.Scan(&id, &name, &family, &element, &kind, &level, &slot); err != nil {
			return nil, nil, err
		}
		// ⚠️ 等级必须夹紧才能下发。
		//
		// `COALESCE(us.level, 1)` 只挡 NULL，挡不住越界值。
		// 客户端拿这个 level 直接缩放技能伤害（系数 = 1000 + (level-1)*coef_permille），
		// 所以库里一行 `level = 99` 会变成 **4900‰** 的伤害加成 ——
		// 而服务端的攻击封顶（1000‰）完全不知道这件事。
		//
		// 这不是「假设脏数据不会发生」，是**读路径必须必然产出合法值**：
		// 迁移事故、手工改库、未来某个端点忘了带上限，任何一个都能写进 99。
		//
		// 更要紧的是 I-6：等级进了重放哈希，而哈希是「这局确实是这样打的」的凭证。
		// 一个能被随手改成 4900‰ 的因子进入哈希，那这个哈希证明不了任何事。
		level = skillRules.ClampLevel(level)
		if slot >= 0 {
			occupied[slot] = true
		}
		skills[fmt.Sprintf("%d", id)] = map[string]any{
			"id": id, "name": name, "family": family, "element": element,
			"kind": kind, "level": level, "slot": slot,
		}
		elements[element] = true
	}
	list := make([]string, 0, len(elements))
	for e := range elements {
		list = append(list, e)
	}

	// 槽位校验：装备的技能数不得超过「基础槽位 + 专精额外插槽」。
	//
	// ⚠️ 此前**完全没有这个校验** —— 客户端可以把任意多个技能写进
	// `user_skill_slots`，服务端照单全收。
	// 引擎会忽略 `slot >= activeSlots` 的技能，所以这不是刷分漏洞，
	// 但它意味着**「玩家有几个槽位」完全由客户端说了算**，
	// 而槽位正是专精树第 3 层花点数换来的东西。
	//
	// 判据用**去重后的槽位数**而不是技能条数：同一槽位重复写
	// （`user_skill_slots` 有 (user_id, skill_id) 主键，重复的是不同 skill_id
	// 写进同一 slot）不该被算成两个槽。
	if err := rows.Err(); err != nil {
		return nil, nil, err
	}
	if err := s.checkSlotBudget(ctx, userID, occupied); err != nil {
		return nil, nil, err
	}
	return skills, list, nil
}

func (s *Service) loadEquipment(ctx context.Context, userID int64) ([]int, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT equipment_id FROM user_equipment WHERE user_id = $1 AND equipped ORDER BY slot`, userID)
	if err != nil {
		return nil, fmt.Errorf("load equipment: %w", err)
	}
	defer rows.Close()
	var out []int
	for rows.Next() {
		var id int
		if err := rows.Scan(&id); err != nil {
			return nil, err
		}
		out = append(out, id)
	}
	return out, rows.Err()
}

func (s *Service) loadMasteryNodes(ctx context.Context, userID int64) ([]int, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT node_id FROM user_mastery_nodes WHERE user_id = $1`, userID)
	if err != nil {
		return nil, fmt.Errorf("load mastery nodes: %w", err)
	}
	defer rows.Close()
	var out []int
	for rows.Next() {
		var id int
		if err := rows.Scan(&id); err != nil {
			return nil, err
		}
		out = append(out, id)
	}
	return out, rows.Err()
}

func (s *Service) computeRating(ctx context.Context, userID int64, build map[string]any) domain.BuildRating {
	in := domain.RatingInput{}
	for _, elem := range buildElements(build) {
		in.SkillElements = append(in.SkillElements, domain.Element(elem))
	}
	if nodes, ok := build["mastery_nodes"].([]int); ok {
		in.MasteryPicked = len(nodes)
		fams := map[string]bool{}
		for _, n := range nodes {
			for _, f := range domain.AllMasteryFamilies() {
				for _, node := range f.Nodes {
					if node.ID == n {
						fams[f.Family] = true
					}
				}
			}
		}
		for f := range fams {
			in.MasteryFamilies = append(in.MasteryFamilies, f)
		}
		// 把专精节点实际提供的战斗乘区填进 RatingInput。
		//
		// ⚠️ 这一段是 reaction_mult / crit / element_cap / armor 四类节点
		// 从「只写不读」变成真正生效的关键。少了它，computeAttacker 只能
		// 硬编码常量，玩家投入这些节点后战斗数值完全不变。
		//
		// EvaluateMastery 需要「全部节点定义 + 已投入的 id 集合 + 剩余点数」，
		// 前两样可以从 build 拿到，点数需要读 user_progress。
		// 评估失败时按"无加成"处理 —— 不能因为读不到点数就让整次结算失败。
		selected := make(map[int]bool, len(nodes))
		for _, n := range nodes {
			selected[n] = true
		}
		points := 0
		if err := s.pool.QueryRow(ctx,
			`SELECT mastery_points FROM user_progress WHERE user_id = $1`, userID).Scan(&points); err != nil {
			points = 0
		}
		// EvaluateMastery 需要「全部节点定义 + 已投入的 id 集合 + 剩余点数」。
		// 全部节点从 AllMasteryFamilies() 展平（它是 8 系 × 3 层 × 4 选 2 的来源）。
		allNodes := make([]domain.MasteryNode, 0, 96)
		for _, f := range domain.AllMasteryFamilies() {
			allNodes = append(allNodes, f.Nodes...)
		}
		if eff, err := domain.EvaluateMastery(allNodes, selected, points); err == nil {
			in.ReactionMultBonus = eff.ReactionMultBonus
			in.CritBonus = eff.CritBonus
			in.ElementCapBonus = eff.ElementCapBonus
			in.ArmorBonus = eff.ArmorBonus
		}
	}
	return domain.ComputeBuildRating(in, domain.DefaultRatingWeights())
}

func (s *Service) computePower(ctx context.Context, userID int64, build map[string]any) int64 {
	in := domain.PowerInput{
		SkillLevels:   map[int]int64{},
		EquipmentLvls: map[int]int64{},
	}
	if skills, ok := build["skills"].(map[string]any); ok {
		for k, v := range skills {
			var id int
			fmt.Sscanf(k, "%d", &id)
			if m, ok := v.(map[string]any); ok {
				if lv, ok := m["level"].(int); ok {
					in.SkillLevels[id] = int64(lv)
				}
			}
		}
	}
	if nodes, ok := build["mastery_nodes"].([]int); ok {
		in.MasteryPicked = len(nodes)
	}
	return domain.ComputePower(in)
}

// --- 任务 ---

// TaskView 是任务展示对象。
type TaskView struct {
	ID       int            `json:"id"`
	Code     string         `json:"code"`
	Name     string         `json:"name"`
	Scope    string         `json:"scope"`
	Target   int            `json:"target"`
	Metric   string         `json:"metric"`
	Progress int            `json:"progress"`
	Reward   map[string]int `json:"reward"`
	Claimed  bool           `json:"claimed"`
	Done     bool           `json:"done"`
}

// LoadTasks 返回指定周期的任务列表。
func (s *Service) LoadTasks(ctx context.Context, userID int64, scope string) ([]TaskView, error) {
	day := periodStart(time.Now(), scope)
	rows, err := s.pool.Query(ctx, `
		SELECT t.id, t.code, t.name, t.scope, t.target, t.metric, t.reward,
		       COALESCE(ut.progress, 0), ut.claimed_at IS NOT NULL
		FROM tasks t
		LEFT JOIN user_tasks ut
		       ON ut.task_id = t.id AND ut.user_id = $1 AND ut.task_date = $2
		WHERE t.enabled AND t.scope = $3
		ORDER BY t.id`, userID, day, scope)
	if err != nil {
		return nil, fmt.Errorf("load tasks: %w", err)
	}
	defer rows.Close()

	var out []TaskView
	for rows.Next() {
		var v TaskView
		var rewardRaw []byte
		if err := rows.Scan(&v.ID, &v.Code, &v.Name, &v.Scope, &v.Target, &v.Metric,
			&rewardRaw, &v.Progress, &v.Claimed); err != nil {
			return nil, err
		}
		_ = json.Unmarshal(rewardRaw, &v.Reward)
		v.Done = v.Progress >= v.Target
		out = append(out, v)
	}
	return out, rows.Err()
}

// bumpTasks 按指标累加任务进度。周期内的同一任务只保留最大值，
// 避免反复战斗把累计型任务刷爆。
func (s *Service) bumpTasks(ctx context.Context, tx pgx.Tx, userID int64, deltas map[string]int64, now time.Time) error {
	for metric, delta := range deltas {
		// 累计型指标取"当前值"，单局型取"本局增量"
		value := delta
		rows, err := tx.Query(ctx,
			`SELECT id, target, scope FROM tasks WHERE enabled AND metric = $1`, metric)
		if err != nil {
			return fmt.Errorf("query tasks metric %s: %w", metric, err)
		}
		type taskRow struct {
			id, target int
			scope      string
		}
		var list []taskRow
		for rows.Next() {
			var t taskRow
			if err := rows.Scan(&t.id, &t.target, &t.scope); err != nil {
				rows.Close()
				return err
			}
			list = append(list, t)
		}
		rows.Close()
		if err := rows.Err(); err != nil {
			return err
		}

		for _, t := range list {
			day := periodStart(now, t.scope)
			if t.scope == "achievement" {
				// 成就类记录的是"达到过的最大值"，用 GREATEST 防止倒退
				if _, err := tx.Exec(ctx, `
					INSERT INTO user_tasks (user_id, task_id, task_date, progress)
					VALUES ($1,$2,$3,$4)
					ON CONFLICT (user_id, task_id, task_date) DO UPDATE SET
					  progress = GREATEST(user_tasks.progress, EXCLUDED.progress)`,
					userID, t.id, day, value); err != nil {
					return fmt.Errorf("bump achievement %d: %w", t.id, err)
				}
				continue
			}
			// 日/周任务累加，但同一周期内不超过目标值（超出无意义）
			if _, err := tx.Exec(ctx, `
				INSERT INTO user_tasks (user_id, task_id, task_date, progress)
				VALUES ($1,$2,$3,$4)
				ON CONFLICT (user_id, task_id, task_date) DO UPDATE SET
				  progress = LEAST($5, GREATEST(user_tasks.progress, user_tasks.progress + $4))`,
				userID, t.id, day, value, t.target); err != nil {
				return fmt.Errorf("bump task %d: %w", t.id, err)
			}
		}
	}
	return nil
}

// ClaimTask 领取任务奖励。
func (s *Service) ClaimTask(ctx context.Context, userID int64, taskID int) (map[string]int64, error) {
	var out map[string]int64
	err := s.DB.Tx(ctx, func(tx pgx.Tx) error {
		var scope string
		var target, progress int
		var claimedAt *time.Time
		var rewardRaw []byte
		err := tx.QueryRow(ctx,
			`SELECT t.scope, t.target, t.reward, COALESCE(ut.progress,0), ut.claimed_at
			 FROM tasks t
			 LEFT JOIN user_tasks ut
			        ON ut.task_id = t.id AND ut.user_id = $1 AND ut.task_date = $2
			 WHERE t.id = $3 AND t.enabled`, userID, periodStart(time.Now(), "daily"), taskID).
			Scan(&scope, &target, &rewardRaw, &progress, &claimedAt)
		if errors.Is(err, pgx.ErrNoRows) {
			return fmt.Errorf("%w: task %d", ErrNotFound, taskID)
		}
		if err != nil {
			return err
		}
		day := periodStart(time.Now(), scope)
		if claimedAt != nil {
			return fmt.Errorf("%w: 奖励已领取", ErrForbidden)
		}
		if progress < target {
			return fmt.Errorf("%w: 任务未完成（%d/%d）", ErrForbidden, progress, target)
		}
		var reward map[string]int
		if err := json.Unmarshal(rewardRaw, &reward); err != nil {
			return err
		}
		tag, err := tx.Exec(ctx,
			`UPDATE user_tasks SET claimed_at = now()
			 WHERE user_id = $1 AND task_id = $2 AND task_date = $3 AND claimed_at IS NULL`,
			userID, taskID, day)
		if err != nil {
			return err
		}
		// ⚠️ 必须检查 RowsAffected。
		// 上面的 `claimedAt != nil` 预检在并发下是无效的：
		// 多个请求可能都在彼此提交前读到"未领取"，然后全部通过预检。
		// 条件 UPDATE 才是真正的互斥点 —— 并发时只有一方匹配到行，
		// 其余的 RowsAffected 为 0。若不检查就会各自 grantWallet，
		// 把奖励发放任意倍数。
		// 同一文件 SettleBattle 认领 battle_token 时就是正确写法。
		if tag.RowsAffected() == 0 {
			return fmt.Errorf("%w: 奖励已领取", ErrForbidden)
		}
		deltas := map[string]int64{}
		for k, v := range reward {
			deltas[k] = int64(v)
		}
		if err := s.grantWallet(ctx, tx, userID, deltas, "task_reward", int64(taskID)); err != nil {
			return err
		}
		out = deltas
		return nil
	})
	return out, err
}

func periodStart(t time.Time, scope string) time.Time {
	d := time.Date(t.Year(), t.Month(), t.Day(), 0, 0, 0, 0, t.Location())
	switch scope {
	case "weekly":
		weekday := int(d.Weekday())
		if weekday == 0 {
			weekday = 7 // Go 的周日是 0，转成 1..7
		}
		return d.AddDate(0, 0, -(weekday - 1))
	default:
		return d
	}
}

// ErrReplaySkillsMismatch 表示上报的技能槽配置与服务端按构筑重算的结果不符。
//
// 抓的是：伪造底伤（等价于假报技能等级 —— 等级必须烘进底伤）、
// 上报一套与 `user_skill_slots` 不同的技能、槽位错位。
//
// ⚠️ 与「`replay_hash` 对不上」不同：那个算不出来（它含战斗中选卡的 buff），
// 所以这是目前**唯一**能廉价抓到的技能侧伪造。攻方系数仍然抓不到。
var ErrReplaySkillsMismatch = errors.New("回放技能配置与构筑不符")

/**
 * 重算本局应上报的回放前缀 S 段（第 56 轮）。
 *
 * 数据来源与客户端**同一套**：`user_skill_slots`（槽位）+ `user_skills.level`（等级）
 * + `domain.SeedSkills`（底伤/叠层/热量）。
 *
 * ⚠️ 客户端在 `equippedFromSnapshot` 里用 `DEFAULT_SKILL_RULES` 烘等级，
 * 而这里用 `domain.DefaultSkillRules()`。两者必须相同 ——
 * `TestSkillRulesMatchServerContract` 守着这条。
 * 若哪天服务端改了 `SkillLevelCoefPermille` 而客户端的默认值没跟着改，
 * 这里会开始拒绝**所有合法对局**。
 */
func (s *Service) replaySkillsSegment(ctx context.Context, userID int64) (string, error) {
	slots, err := s.Loadout(ctx, userID)
	if err != nil {
		return "", err
	}
	levels, err := s.SkillLevels(ctx, userID)
	if err != nil {
		return "", err
	}
	bySlot := make(map[int]int, len(slots))
	for i, id := range slots {
		if id > 0 {
			bySlot[i] = id
		}
	}
	return domain.BuildReplaySkillsSegment(
		domain.SeedSkills, bySlot, levels, domain.DefaultSkillRules(),
	), nil
}
