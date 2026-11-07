// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// 构筑读路径的「一次取齐」折叠（性能：LoadBuildSnapshot 12 条往返 → 6 条）。
//
// # 背景：同一件事查两遍
//
// `LoadBuildSnapshot` 旧路径是逐个调 `loadSkillsAndSlots` / `loadEquipment` /
// `loadMasteryNodes` / `computeAttacker` / `ExtraSlots`，每个方法自己查库：
// 专精节点查 3 遍（槽位预算 1 + 攻方 1 + 额外槽 1）、
// 点数查 2 遍、装备查 2 遍（下发列表 1 + 攻方装配 1），
// 合计 **12 条串行往返**。它挂在 `/battle/token` 与 `/battle/settle`
// 两条最热路径上（结算还会再跑 `replaySkillsSegment`）。
//
// 本文件把「取」与「算」拆开：
//
//   - 取：`loadBuildParts` **一趟**查齐 6 张表（技能/槽位、装备、
//     专精节点、max_stage、点数、宝石），各自保留旧路径的逐查询错误语义；
//   - 算：纯函数 `attackerFromParts` / `masteryEffectFrom` /
//     `applyLoadoutContribution` 等。
//
// ⚠️ 旧方法（`loadLoadout` / `loadMasteryEffect` / `loadSkillsAndSlots`…）**保留原样**：
// 它们各有「单独查、单独报错」的行为被测试钉住（attacker_buy_test 把
// 装备表/宝石表分别改名验证 loadLoadout 两条查询各自失败），
// 所以共用的是**装配逻辑**（纯函数），查询各走各的。
// 「同一件事算两遍」只留在「取」上（6 条 vs 12 条），「算」只有一份。

package service

import (
	"context"
	"encoding/json"
	"fmt"
	"sort"
	"sync"

	"github.com/jackc/pgx/v5"

	"github.com/laiyipao/server/internal/domain"
)

// buildParts 是构筑读路径的全部原始数据，一趟查齐。
type buildParts struct {
	skills       map[string]any
	elements     []string
	occupied     map[int]bool
	equipment    []int
	masteryNodes []int
	maxStage     int
	// 点数。读失败时保持 0 —— 与旧 loadMasteryEffect 的「点数读不到 = 无加点」
	// 静默语义一致（那条旧路径只在 ExtraSlots 单独调用时可达，LoadBuildSnapshot
	// 内 computeAttacker 会先硬失败，所以新路径下它实际也只在库故障时取到 0）。
	masteryPoints int
	loadout       loadoutContribution
}

// 构筑读路径的五条 SQL。提取成常量是为了让 loadBuildParts 的批量版本与
// 旧单条方法共用**同一份**语句文本 —— 两处各写一份必然漂移。
const (
	buildSkillsSQL = `
			SELECT s.id, s.name, s.family, s.element, s.kind, COALESCE(us.level, 1),
			       COALESCE(sl.slot, -1)
			FROM user_skills us
			JOIN skills s ON s.id = us.skill_id
			LEFT JOIN user_skill_slots sl ON sl.user_id = us.user_id AND sl.skill_id = us.skill_id
			WHERE us.user_id = $1 ORDER BY s.id`
	buildEquipmentSQL = `
			SELECT equipment_id FROM user_equipment WHERE user_id = $1 AND equipped ORDER BY slot`
	buildMasteryNodesSQL = `
			SELECT node_id FROM user_mastery_nodes WHERE user_id = $1`
	buildProgressSQL = `
			SELECT COALESCE(MAX(max_stage), 0), COALESCE(MAX(mastery_points), 0)
			FROM user_progress WHERE user_id = $1`
	buildGemsSQL = `
			SELECT affixes FROM user_gems WHERE user_id = $1 AND equipped_slot <> ''`
)

// batchSender 是 loadBuildParts 批量版本需要的最小接口（*pgxpool.Conn 满足）。
// 单独抽成接口是为了让往返守卫能在测试里包一层计数器。
type batchSender interface {
	SendBatch(ctx context.Context, b *pgx.Batch) pgx.BatchResults
}

// loadBuildParts 一趟查齐构筑所需的全部数据。
//
// R148：五条**互不依赖**的读查询压进一次 pgx.Batch —— 一个连接、一次往返。
//
// # 为什么改（R147 后的实测）
//
// 改前每次 LoadBuildSnapshot 是 5 条串行往返（技能/装备/专精/进度/宝石）。
// 1M 库 c=8 实测 `build_load_pre` 段 **6.76ms**，而五条 SQL 的
// pg_stat 执行时间合计仅 **0.199ms** —— 差额是 5 次 Acquire + 往返，
// 不是查询本身。`/battle/token` 与 `/battle/settle` 共用这条读路径，
// 玩家开战与结算各吃一次；跨级时 `tail_load` 还会再读一次。
//
// 批量版本把往返固定为 **1**，且只 Acquire 一次。执行顺序与失败顺序
// 仍是 技能 → 装备 → 专精节点 → 进度 → 宝石（与改前逐查询语义一致）：
// pgx 的 Batch 结果按入队顺序返回，执行期错误在对应结果上原样上抛，
// 不会把后面表的结果混进来；prepare 期错误（如表被改名）统一在第一条
// 结果上抛出，PG 原始错误里仍带表名。
func (s *Service) loadBuildParts(ctx context.Context, userID int64) (buildParts, error) {
	var p buildParts
	conn, err := s.pool.Acquire(ctx)
	if err != nil {
		return p, err
	}
	defer conn.Release()
	return loadBuildPartsOn(ctx, conn, userID)
}

// loadBuildPartsOn 在给定 sender 上执行批量读（生产传池连接，测试传计数器）。
func loadBuildPartsOn(ctx context.Context, sender batchSender, userID int64) (buildParts, error) {
	var p buildParts
	var b pgx.Batch
	b.Queue(buildSkillsSQL, userID)
	b.Queue(buildEquipmentSQL, userID)
	b.Queue(buildMasteryNodesSQL, userID)
	b.Queue(buildProgressSQL, userID)
	b.Queue(buildGemsSQL, userID)
	br := sender.SendBatch(ctx, &b)
	defer br.Close()

	rows, err := br.Query()
	if err != nil {
		// Batch 在 CacheStatement 模式下会先把五条语句一起 prepare；任一语句
		// 解析失败都会在**第一条结果**上以 "error preprocessing batch" 抛出，
		// 因此这里只能给通用前缀。失败的表名仍在原始 PG 错误里
		// （relation "user_xxx" does not exist），可观测性没丢。
		return p, fmt.Errorf("load build parts: %w", err)
	}
	skills, elements, occupied, err := skillsRowsToMaps(rows)
	rows.Close()
	if err != nil {
		return p, err
	}
	if err := rows.Err(); err != nil {
		return p, err
	}
	p.skills, p.elements, p.occupied = skills, elements, occupied

	rows, err = br.Query()
	if err != nil {
		return p, fmt.Errorf("load equipment: %w", err)
	}
	equipment, err := rowsToInts(rows)
	if err != nil {
		return p, err
	}
	p.equipment = equipment

	rows, err = br.Query()
	if err != nil {
		return p, fmt.Errorf("load mastery nodes: %w", err)
	}
	nodes, err := rowsToInts(rows)
	if err != nil {
		return p, err
	}
	p.masteryNodes = nodes

	// max_stage + 点数：同一张 user_progress 表的两列，合并成一条查询
	// （第 6 轮）。行不存在（新号）时聚合恒有一行 (0, 0)，不会 ErrNoRows。
	// 点数失败必须 fail-loud：吞掉会下发少了专精加成的评分（P3-1）。
	if err := br.QueryRow().Scan(&p.maxStage, &p.masteryPoints); err != nil {
		return p, fmt.Errorf("load progress (max_stage + mastery_points): %w", err)
	}

	rows, err = br.Query()
	if err != nil {
		return p, fmt.Errorf("load gems: %w", err)
	}
	gemRaw, err := rowsToGemAffixes(rows)
	if err != nil {
		return p, err
	}
	p.loadout = applyLoadoutContribution(equipment, gemRaw)
	return p, nil
}

// ---- 「取」的公共件：旧方法与 loadBuildParts 共用同一份 SQL 与装配 ----

// loadSkillsRows 读取玩家已解锁技能的槽位/元素/等级，返回装配三件套。
// 槽位预算校验由调用方做（旧 loadSkillsAndSlots 保留原样）。
func (s *Service) loadSkillsRows(ctx context.Context, userID int64) (map[string]any, []string, map[int]bool, error) {
	rows, err := s.pool.Query(ctx, buildSkillsSQL, userID)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("load skills: %w", err)
	}
	defer rows.Close()

	skills, elements, occupied, err := skillsRowsToMaps(rows)
	if err != nil {
		return nil, nil, nil, err
	}
	return skills, elements, occupied, rows.Err()
}

// skillsRowsToMaps 把技能行迭代装配成 (按 id 索引的技能表, 元素列表, 已占槽位)。
// 纯装配 —— 等级夹紧、槽位去重、元素去重都在这里，旧方法与新路径共用。
// Scan 失败必须上抛：这是 /me、/battle/token、settle 的共享路径，
// 吞掉错误会让坏行静默变成半份构筑，回放哈希漂移把诚实玩家误判成作弊。
func skillsRowsToMaps(rows pgx.Rows) (map[string]any, []string, map[int]bool, error) {
	skills := map[string]any{}
	elements := map[string]bool{}
	occupied := map[int]bool{}
	for rows.Next() {
		var id, level, slot int
		var name, family, element, kind string
		if err := rows.Scan(&id, &name, &family, &element, &kind, &level, &slot); err != nil {
			return nil, nil, nil, fmt.Errorf("scan skills row: %w", err)
		}
		// 等级夹紧的理由见 game.go loadSkillsAndSlots 的原注释（I-6：等级进重放哈希，
		// 脏数据 99 级会变成 4900‰ 伤害加成而服务端攻击封顶不知道）。
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
	// 稳定顺序：elements 在语义上是集合，但这条列表会直接进 /me 与
	// /battle/token 的 JSON。map 迭代顺序随机会让同一玩家的响应逐次抖动，
	// 也会让 R148 的「串行 vs 批量」等价性守卫（逐字段比较）随机变红。
	sort.Strings(list)
	return skills, list, occupied, nil
}

// loadEquipmentRows 读玩家已装备装备的 id 列表（按 slot 排序）。
func (s *Service) loadEquipmentRows(ctx context.Context, userID int64) ([]int, error) {
	rows, err := s.pool.Query(ctx, buildEquipmentSQL, userID)
	if err != nil {
		return nil, fmt.Errorf("load equipment: %w", err)
	}
	return rowsToInts(rows)
}

// rowsToInts 消费单列 int 结果集（装备 id / 专精节点），并在结束时关闭。
func rowsToInts(rows pgx.Rows) ([]int, error) {
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

// loadMasteryNodeRows 读玩家已点亮的专精节点。
func (s *Service) loadMasteryNodeRows(ctx context.Context, userID int64) ([]int, error) {
	rows, err := s.pool.Query(ctx, buildMasteryNodesSQL, userID)
	if err != nil {
		return nil, fmt.Errorf("load mastery nodes: %w", err)
	}
	return rowsToInts(rows)
}

// loadGemsAffixRows 读玩家已佩戴宝石的词条 JSON（每颗一份原始 JSONB）。
func (s *Service) loadGemsAffixRows(ctx context.Context, userID int64) ([][]byte, error) {
	rows, err := s.pool.Query(ctx, buildGemsSQL, userID)
	if err != nil {
		return nil, fmt.Errorf("load gems: %w", err)
	}
	return rowsToGemAffixes(rows)
}

// rowsToGemAffixes 消费宝石词条结果集，并在结束时关闭。
func rowsToGemAffixes(rows pgx.Rows) ([][]byte, error) {
	defer rows.Close()
	var out [][]byte
	for rows.Next() {
		var raw []byte
		if err := rows.Scan(&raw); err != nil {
			return nil, err
		}
		out = append(out, raw)
	}
	return out, rows.Err()
}

// applyLoadoutContribution 把「装备 id + 宝石词条 JSON」装配成未封顶的贡献。
// 纯函数 —— 未知装备跳过（白拿属性比没有更糟）、坏词条/未知词条计数上抛，
// 口径与旧 loadLoadout 逐行一致，两个调用路径共用这一份。
func applyLoadoutContribution(equipment []int, gemRaw [][]byte) loadoutContribution {
	var c loadoutContribution
	for _, id := range equipment {
		eq, ok := domain.EquipmentByID(int64(id))
		if !ok {
			// 未知 id：跳过而不是回落到默认值（回落 = 白拿一份属性）。
			continue
		}
		c.EquipmentCount++
		c.Attack += eq.BaseBonusPct
		c.Armor += eq.BaseArmor
	}
	for _, raw := range gemRaw {
		var affixes []gemAffix
		if err := json.Unmarshal(raw, &affixes); err != nil {
			// 解析失败**不能静默跳过**：一颗词条读不出来的宝石，
			// 静默跳过等于玩家付了成本却没生效，且没有任何迹象。
			c.AffixCountIgnored++
			continue
		}
		c.EquippedGemCount++
		for _, af := range affixes {
			switch af.Affix {
			case "attack", "skill_damage":
				c.Attack += af.Value
			case "armor":
				c.Armor += af.Value
			case "crit":
				c.Crit += af.Value
			case "element_coef":
				c.ElementCoef += af.Value
			case "element_cap":
				c.ElementCap += af.Value
			case "reaction_mult":
				c.ReactionMult += af.Value
			case "heat_cap":
				c.HeatCap += af.Value
			default:
				// 未知词条：与解析失败同等对待 —— 不能默默吞掉。
				c.AffixCountIgnored++
			}
		}
	}
	return c
}

// ---- 「算」的共享件 ----

// allMasteryNodes 是 8 系 × 3 层 × 4 选 2 展平后的全部节点（96 个），
// 进程内只构建一次（第 19 轮）。此前 4 处调用点各自重新展平 ——
// masteryEffectFrom / AllocateMastery / computeRating 每次调用都白付一遍。
var allMasteryNodes = sync.OnceValue(func() []domain.MasteryNode {
	all := make([]domain.MasteryNode, 0, 96)
	for _, f := range domain.AllMasteryFamilies() {
		all = append(all, f.Nodes...)
	}
	return all
})

// masteryNodeFamily 是 nodeID → 所属系的映射（与 allMasteryNodes 同源一次构建）。
// computeRating 此前对每个已投入节点做 O(96) 的双层循环找 family。
var masteryNodeFamily = sync.OnceValue(func() map[int]string {
	m := make(map[int]string, 96)
	for _, f := range domain.AllMasteryFamilies() {
		for _, node := range f.Nodes {
			m[node.ID] = f.Family
		}
	}
	return m
})

// masteryEffectFrom 是 loadMasteryEffect 的纯计算部分：
// 由「已点亮节点 + 剩余点数」求专精合计效果。
// 出错时返回零值（无加成）—— 与旧 loadMasteryEffect 的静默降级一致，
// 两处路径共用这一份降级语义。
func masteryEffectFrom(nodes []int, points int) domain.MasteryEffect {
	if len(nodes) == 0 {
		return domain.MasteryEffect{}
	}
	selected := make(map[int]bool, len(nodes))
	for _, n := range nodes {
		selected[n] = true
	}
	eff, err := domain.EvaluateMastery(allMasteryNodes(), selected, points)
	if err != nil {
		// 专精点数/前置不合法属于玩家侧状态问题，不该让整局战斗打不开。
		return domain.MasteryEffect{}
	}
	return eff
}

// extraSlotsFrom 从专精合计效果取「额外插槽」（负数按 0）。
func extraSlotsFrom(eff domain.MasteryEffect) int64 {
	if eff.ExtraSlots < 0 {
		return 0
	}
	return eff.ExtraSlots
}

// slotBudgetCheck 槽位预算的纯判定：已占槽位数 > 基础 + 专精额外 → 拒。
// 判据（去重后的槽位数而非技能条数）理由见 game.go checkSlotBudget 原注释。
func slotBudgetCheck(occupied map[int]bool, extra int64) error {
	budget := int(domain.BaseSkillSlots + extra)
	if n := len(occupied); n > budget {
		return fmt.Errorf(
			"%w：已装备 %d 个槽位，预算 %d 个（基础 %d + 专精额外 %d）",
			ErrSlotBudgetExceeded, n, budget, domain.BaseSkillSlots, extra,
		)
	}
	return nil
}

// attackerFromParts 由一趟取齐的零件算出攻方属性 —— 旧 computeAttacker
// 函数体的纯计算部分（专精/装备/宝石装配 + 各乘区封顶），不含任何查询。
// 旧 computeAttacker（仍被测试直接调用）与新 LoadBuildSnapshot 共用这一份。
func attackerFromParts(p buildParts) domain.Attacker {
	stage := int64(p.maxStage)
	if stage < 0 {
		stage = 0
	}

	eff := masteryEffectFrom(p.masteryNodes, p.masteryPoints)

	a := domain.DefaultAttacker()
	// 攻击力：每通过 10 关 +200‰（1 关 +20‰）。
	a.Attack += stage * 20
	// 元素层数上限：每 25 关 +1，再叠专精 element_cap 节点。
	a.ElementCap += stage / 25
	if eff.ElementCapBonus > 0 {
		a.ElementCap += eff.ElementCapBonus
	}
	if a.ElementCap > domain.MaxElementCap {
		a.ElementCap = domain.MaxElementCap
	}
	// 元素系数：每个专精节点 +15‰，封顶 +600‰。
	coef := int64(len(p.masteryNodes)) * 15
	if coef > 600 {
		coef = 600
	}
	a.ElementCoefPermille += coef
	// 四个此前只写不读的专精维度（技能伤害折进 Attack 防反通胀后门）。
	if eff.SkillDamageBonus > 0 {
		a.Attack += eff.SkillDamageBonus
	}
	if eff.HeatCapBonus > 0 {
		a.HeatCapPermille += eff.HeatCapBonus
	}
	if eff.ArmorBonus > 0 {
		a.ArmorPermille += eff.ArmorBonus
	}
	if eff.MechanicBonus > 0 {
		a.MechanicPermille += eff.MechanicBonus
	}
	// 反应倍率：来自 reaction_mult 节点，封顶 +800‰（上限已按 I-1 收紧）。
	rm := eff.ReactionMultBonus
	if rm > 800 {
		rm = 800
	}
	if rm < 0 {
		rm = 0
	}
	a.ReactionMultPermille += rm
	// 反应阶：每 3 个 reaction_mult 节点 +1 阶，封顶 4。
	rt := int64(1) + (rm/domain.MasteryReactionMultPerNode)/3
	if rt > 4 {
		rt = 4
	}
	a.ReactionTier = rt
	// 暴击：crit 节点封顶 +500‰（总上限 550‰）。
	cp := eff.CritBonus
	if cp > 500 {
		cp = 500
	}
	if cp < 0 {
		cp = 0
	}
	a.CritPermille = 50 + cp
	a.CritMultiplierPermille = 1500

	// 装备与宝石贡献（封顶在 applyLoadout，且必须在专精贡献之后）。
	applyLoadout(&a, p.loadout)

	// 元素层数封顶在装配**之后**再钳一次：宝石 element_cap 也进这一项。
	if a.ElementCap > domain.MaxElementCap {
		a.ElementCap = domain.MaxElementCap
	}
	return a
}
