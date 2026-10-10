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

// loadBuildParts 一趟查齐构筑所需的全部数据（6 条查询）。
//
// 查询顺序刻意与旧 LoadBuildSnapshot 的失败顺序一致：
// 技能 → 装备 → 专精节点 → max_stage → 点数 → 宝石，
// 旧路径里第 i 个方法先失败的场景，新路径仍在第 i 张表上失败。
func (s *Service) loadBuildParts(ctx context.Context, userID int64) (buildParts, error) {
	var p buildParts

	skills, elements, occupied, err := s.loadSkillsRows(ctx, userID)
	if err != nil {
		return p, err
	}
	p.skills, p.elements, p.occupied = skills, elements, occupied

	equipment, err := s.loadEquipmentRows(ctx, userID)
	if err != nil {
		return p, err
	}
	p.equipment = equipment

	nodes, err := s.loadMasteryNodeRows(ctx, userID)
	if err != nil {
		return p, err
	}
	p.masteryNodes = nodes

	// max_stage：COALESCE(MAX(...)) 恒有一行，查询本身失败才算错 —— 与旧
	// computeAttacker 的「load max stage」硬失败语义一致。
	if err := s.pool.QueryRow(ctx,
		`SELECT COALESCE(MAX(max_stage), 0) FROM user_progress WHERE user_id = $1`,
		userID).Scan(&p.maxStage); err != nil {
		return p, fmt.Errorf("load max stage: %w", err)
	}

	// 点数：与旧 loadMasteryEffect 相同 —— 读不到按 0（无加点）处理。
	// 只有查询本身失败才静默；行存在但列是 NULL 仍是 PG 扫描错误。
	if err := s.pool.QueryRow(ctx,
		`SELECT mastery_points FROM user_progress WHERE user_id = $1`,
		userID).Scan(&p.masteryPoints); err != nil {
		p.masteryPoints = 0
	}

	gemRaw, err := s.loadGemsAffixRows(ctx, userID)
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
	rows, err := s.pool.Query(ctx,
		`SELECT s.id, s.name, s.family, s.element, s.kind, COALESCE(us.level, 1),
		        COALESCE(sl.slot, -1)
		 FROM user_skills us
		 JOIN skills s ON s.id = us.skill_id
		 LEFT JOIN user_skill_slots sl ON sl.user_id = us.user_id AND sl.skill_id = us.skill_id
		 WHERE us.user_id = $1 ORDER BY s.id`, userID)
	if err != nil {
		return nil, nil, nil, fmt.Errorf("load skills: %w", err)
	}
	defer rows.Close()

	skills, elements, occupied := skillsRowsToMaps(rows)
	return skills, elements, occupied, rows.Err()
}

// skillsRowsToMaps 把技能行迭代装配成 (按 id 索引的技能表, 元素列表, 已占槽位)。
// 纯装配 —— 等级夹紧、槽位去重、元素去重都在这里，旧方法与新路径共用。
func skillsRowsToMaps(rows pgx.Rows) (map[string]any, []string, map[int]bool) {
	skills := map[string]any{}
	elements := map[string]bool{}
	occupied := map[int]bool{}
	for rows.Next() {
		var id, level, slot int
		var name, family, element, kind string
		if err := rows.Scan(&id, &name, &family, &element, &kind, &level, &slot); err != nil {
			return skills, nil, nil
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
	return skills, list, occupied
}

// loadEquipmentRows 读玩家已装备装备的 id 列表（按 slot 排序）。
func (s *Service) loadEquipmentRows(ctx context.Context, userID int64) ([]int, error) {
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

// loadMasteryNodeRows 读玩家已点亮的专精节点。
func (s *Service) loadMasteryNodeRows(ctx context.Context, userID int64) ([]int, error) {
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

// loadGemsAffixRows 读玩家已佩戴宝石的词条 JSON（每颗一份原始 JSONB）。
func (s *Service) loadGemsAffixRows(ctx context.Context, userID int64) ([][]byte, error) {
	rows, err := s.pool.Query(ctx,
		`SELECT affixes FROM user_gems WHERE user_id = $1 AND equipped_slot <> ''`, userID)
	if err != nil {
		return nil, fmt.Errorf("load gems: %w", err)
	}
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
	allNodes := make([]domain.MasteryNode, 0, 96)
	for _, f := range domain.AllMasteryFamilies() {
		allNodes = append(allNodes, f.Nodes...)
	}
	eff, err := domain.EvaluateMastery(allNodes, selected, points)
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
