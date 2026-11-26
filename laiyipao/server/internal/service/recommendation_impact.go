// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R7 缺口3：把"推荐"和"当前构筑"用可追溯的数字连起来。
//
// 底线：
//   - 命中/波、反应/波、三星率来自真实 battle_records；
//   - 技能底伤、装备攻击/护甲、直接伤害权重来自 /config 与 domain 常量；
//   - 只换内存里的构筑副本再算评分/战力，不写库、不改 replay_hash；
//   - 星级没有可信因果模型就明说：给历史三星率，不给编造的概率。

import (
	"context"
	"fmt"
	"math"
	"sort"
	"strconv"
	"time"

	"github.com/laiyipao/server/internal/domain"
)

// ImpactParamError 是参数不合法（kind/id/replace 不存在或未装备）。
type ImpactParamError struct{ Msg string }

func (e ImpactParamError) Error() string { return e.Msg }

type ImpactSide struct {
	Power                  int64   `json:"power"`
	RatingTotal            int     `json:"rating_total"`
	ElementCoverage        int     `json:"element_coverage"`
	ReactionCoverage       int     `json:"reaction_coverage"`
	EquipmentSynergy       int     `json:"equipment_synergy"`
	AttackPermille         int64   `json:"attack_permille"`
	ArmorPermille          int64   `json:"armor_permille"`
	AvgSkillDamage         float64 `json:"avg_skill_damage"`
	HitsPerWave            float64 `json:"hits_per_wave"`
	ReactionsPerWave       float64 `json:"reactions_per_wave"`
	EstimatedDamagePerWave float64 `json:"estimated_damage_per_wave"`
}

type ImpactChanges struct {
	PowerDelta              int64    `json:"power_delta"`
	RatingTotalDelta        int      `json:"rating_total_delta"`
	AttackPermilleDelta     int64    `json:"attack_permille_delta"`
	ArmorPermilleDelta      int64    `json:"armor_permille_delta"`
	DamagePerWaveDelta      float64  `json:"damage_per_wave_delta"`
	DamagePerWavePct        float64  `json:"damage_per_wave_pct"`
	StarRateObserved        float64  `json:"star_rate_observed"`
	StarProbabilityEstimate *float64 `json:"star_probability_estimate"`
}

type ImpactComparison struct {
	Kind          string        `json:"kind"`
	CandidateID   int           `json:"candidate_id"`
	ReplacedID    int           `json:"replaced_id"`
	CandidateName string        `json:"candidate_name"`
	ReplacedName  string        `json:"replaced_name"`
	Baseline      ImpactSide    `json:"baseline"`
	Candidate     ImpactSide    `json:"candidate"`
	Changes       ImpactChanges `json:"changes"`
	Method        string        `json:"method"`
	StarRateNote  string        `json:"star_rate_note"`
}

type ImpactResponse struct {
	SchemaVersion string            `json:"schema_version"`
	GeneratedAt   string            `json:"generated_at"`
	Context       string            `json:"context"`
	Subject       RecSubject        `json:"subject"`
	InputSnapshot RecInputSnapshot  `json:"input_snapshot"`
	Evidence      []RecEvidence     `json:"evidence"`
	Insights      []RecInsight      `json:"insights"`
	Impact        *ImpactComparison `json:"impact"`
	EmptyReason   *RecEmptyReason   `json:"empty_reason"`
	Model         RecModel          `json:"model"`
}

type impactBattleStats struct {
	Count      int64
	Wins       int64
	ThreeStars int64 // stars=3 的场数；"三星率"不能拿胜率代替
	Stars      int64
	Hits       int64
	Waves      int64
	Reactions  int64
}

func (s *Service) impactBattleStats(ctx context.Context, userID int64, days int) (impactBattleStats, error) {
	var st impactBattleStats
	err := s.pool.QueryRow(ctx, `
		SELECT COUNT(*),
		       COUNT(*) FILTER (WHERE result = 'win'),
		       COUNT(*) FILTER (WHERE stars = 3),
		       COALESCE(SUM(stars), 0),
		       COALESCE(SUM(hits), 0),
		       COALESCE(SUM(wave_reached), 0),
		       COALESCE(SUM(reactions), 0)
		  FROM battle_records
		 WHERE user_id = $1
		   AND created_at >= now() - ($2::int * interval '1 day')`, userID, days).Scan(
		&st.Count, &st.Wins, &st.ThreeStars, &st.Stars, &st.Hits, &st.Waves, &st.Reactions)
	if err != nil {
		return st, fmt.Errorf("impact battle stats: %w", err)
	}
	return st, nil
}

func impactCloneMap(in map[string]any) map[string]any {
	out := make(map[string]any, len(in))
	for k, v := range in {
		switch t := v.(type) {
		case map[string]any:
			out[k] = impactCloneMap(t)
		case []int:
			out[k] = append([]int(nil), t...)
		case []string:
			out[k] = append([]string(nil), t...)
		case []any:
			out[k] = append([]any(nil), t...)
		default:
			out[k] = v
		}
	}
	return out
}

func impactInt(m map[string]any, key string) int64 {
	if n, ok := m[key].(int); ok {
		return int64(n)
	}
	if n, ok := m[key].(int64); ok {
		return n
	}
	v, _ := m[key].(float64)
	return int64(v)
}

func impactSkillDamage(build map[string]any, cfg GameConfig) (float64, int) {
	skills, _ := build["skills"].(map[string]any)
	if len(skills) == 0 {
		return 0, 0
	}
	byID := map[int]domain.SeedSkill{}
	for _, sk := range cfg.Skills {
		byID[sk.ID] = sk
	}
	for _, sk := range cfg.Composite {
		byID[sk.ID] = sk
	}
	var sum int64
	count := 0
	for _, raw := range skills {
		m, ok := raw.(map[string]any)
		if !ok {
			continue
		}
		def, ok := byID[int(impactInt(m, "id"))]
		if !ok || def.Kind != "active" {
			continue
		}
		lv := int(impactInt(m, "level"))
		if lv < 1 {
			lv = 1
		}
		sum += domain.SkillBaseDamageAtLevel(cfg.SkillRules, def.BaseDamage, lv)
		count++
	}
	if count == 0 {
		return 0, 0
	}
	return float64(sum) / float64(count), count
}

func impactEstimate(hitsPerWave, avgSkillDamage float64, attack int64) float64 {
	base := avgSkillDamage * float64(1000+attack) / 1000
	return hitsPerWave * base * float64(domain.DirectWeightPermille) / 1000
}

func impactElementsFromBuild(build map[string]any) []string {
	skills, _ := build["skills"].(map[string]any)
	set := map[string]bool{}
	for _, raw := range skills {
		if m, ok := raw.(map[string]any); ok {
			if e, ok := m["element"].(string); ok && e != "" {
				set[e] = true
			}
		}
	}
	out := make([]string, 0, len(set))
	for e := range set {
		out = append(out, e)
	}
	sort.Strings(out)
	return out
}

func impactSynergy(build map[string]any, cfg GameConfig) int {
	elements := map[domain.Element]bool{}
	for _, e := range impactElementsFromBuild(build) {
		elements[domain.Element(e)] = true
	}
	byID := map[int]domain.SeedEquipment{}
	for _, eq := range cfg.Equipment {
		byID[eq.ID] = eq
	}
	equipped, _ := build["equipment"].([]int)
	n := 0
	for _, id := range equipped {
		eq, ok := byID[id]
		if ok && elements[eq.Element] {
			n++
		}
	}
	return n
}

type impactAttacker struct {
	Attack int64
	Armor  int64
}

// build["attacker"] 在 loadBuildSnapshotFull 里是 AttackerView 结构体，
// 而历史快照/测试也可能塞 map[string]any。收益预估只读攻击与护甲两项，
// 因此显式兼容两种表示，避免对 nil map 赋值导致 500。
func impactReadAttacker(build map[string]any) impactAttacker {
	switch a := build["attacker"].(type) {
	case AttackerView:
		return impactAttacker{Attack: a.Attack, Armor: a.ArmorPermille}
	case map[string]any:
		return impactAttacker{Attack: impactInt(a, "attack"), Armor: impactInt(a, "armor_permille")}
	}
	return impactAttacker{}
}

func impactWriteAttacker(build map[string]any, a impactAttacker) {
	switch v := build["attacker"].(type) {
	case AttackerView:
		v.Attack = a.Attack
		v.ArmorPermille = a.Armor
		build["attacker"] = v
	case map[string]any:
		v["attack"] = a.Attack
		v["armor_permille"] = a.Armor
		build["attacker"] = v
	}
}

func impactSide(build map[string]any, cfg GameConfig, rating domain.BuildRating, hitsPerWave, reactionsPerWave float64) ImpactSide {
	avg, _ := impactSkillDamage(build, cfg)
	att := impactReadAttacker(build)
	return ImpactSide{
		Power:                  computePower(build),
		RatingTotal:            rating.Total,
		ElementCoverage:        rating.ElementCoverage,
		ReactionCoverage:       rating.ReactionCoverage,
		EquipmentSynergy:       impactSynergy(build, cfg),
		AttackPermille:         att.Attack,
		ArmorPermille:          att.Armor,
		AvgSkillDamage:         avg,
		HitsPerWave:            hitsPerWave,
		ReactionsPerWave:       reactionsPerWave,
		EstimatedDamagePerWave: impactEstimate(hitsPerWave, avg, att.Attack),
	}
}

func (s *Service) RecommendationImpact(ctx context.Context, userID int64, kind string, candidateID, replacedID int) (ImpactResponse, error) {
	resp := ImpactResponse{
		SchemaVersion: recSchemaVersion,
		GeneratedAt:   time.Now().UTC().Format(time.RFC3339),
		Context:       kind,
		Model:         recModel(),
		Evidence:      []RecEvidence{},
		Insights:      []RecInsight{},
	}
	if kind != "skill" && kind != "equipment" {
		return resp, ImpactParamError{Msg: "kind 只能是 skill 或 equipment"}
	}
	if candidateID <= 0 {
		return resp, ImpactParamError{Msg: "id 必须是正整数"}
	}

	cfg := s.LoadGameConfig()
	if cfg.Version == "" {
		resp.EmptyReason = recEmpty("CONFIG_MISSING", "配置版本缺失，无法按真实数值预估")
		return resp, nil
	}
	st, err := s.impactBattleStats(ctx, userID, recWindowDays)
	if err != nil {
		return resp, err
	}
	resp.InputSnapshot = RecInputSnapshot{BattleCount: int(st.Count), WindowDays: recWindowDays, ConfigVersion: cfg.Version}

	build, parts, err := s.loadBuildSnapshotFull(ctx, userID)
	if err != nil {
		return resp, err
	}
	resp.Subject = RecSubject{UserKey: recUserKey("impact", userID), Level: parts.maxStage, Power: computePower(build)}
	if st.Count == 0 {
		resp.EmptyReason = recEmpty("NO_HISTORY", "最近 7 天没有战报，无法估算真实收益")
		return resp, nil
	}
	if st.Waves <= 0 || st.Hits <= 0 {
		resp.EmptyReason = recEmpty("NOT_ENOUGH_DATA", "战报缺少波次或命中样本，无法估算每波收益")
		return resp, nil
	}

	hitsPerWave := float64(st.Hits) / float64(st.Waves)
	reactionsPerWave := float64(st.Reactions) / float64(st.Waves)
	starRate := float64(st.ThreeStars) / float64(st.Count) * 100
	candidateBuild := impactCloneMap(build)

	var candidateName, replacedName string
	var attackDelta, armorDelta int64
	if kind == "skill" {
		byID := map[int]domain.SeedSkill{}
		for _, sk := range cfg.Skills {
			byID[sk.ID] = sk
		}
		for _, sk := range cfg.Composite {
			byID[sk.ID] = sk
		}
		def, ok := byID[candidateID]
		if !ok || def.Kind != "active" {
			return resp, ImpactParamError{Msg: "id 必须是配置里存在的主战技能"}
		}
		skills, _ := candidateBuild["skills"].(map[string]any)
		oldKey := strconv.Itoa(replacedID)
		oldRaw, ok := skills[oldKey]
		if !ok {
			return resp, ImpactParamError{Msg: "replace 必须是当前已装备的技能 id"}
		}
		old, _ := oldRaw.(map[string]any)
		replacedName, _ = old["name"].(string)
		candidateName = def.Name
		level := int(impactInt(old, "level"))
		if level < 1 {
			level = 1
		}
		slot := int(impactInt(old, "slot"))
		newSkills := impactCloneMap(skills)
		delete(newSkills, oldKey)
		newSkills[strconv.Itoa(candidateID)] = map[string]any{
			"id": candidateID, "name": def.Name, "family": def.Family,
			"element": string(def.Element), "kind": def.Kind,
			"level": level, "slot": slot,
		}
		candidateBuild["skills"] = newSkills
		candidateBuild["elements"] = impactElementsFromBuild(candidateBuild)
		candidateBuild["skill_ids"] = candidateBuild["elements"]
	} else {
		byID := map[int]domain.SeedEquipment{}
		for _, eq := range cfg.Equipment {
			byID[eq.ID] = eq
		}
		def, ok := byID[candidateID]
		if !ok {
			return resp, ImpactParamError{Msg: "id 必须是配置里存在的装备"}
		}
		equipped, _ := candidateBuild["equipment"].([]int)
		idx := -1
		for i, id := range equipped {
			if id == replacedID {
				idx = i
				break
			}
		}
		if idx < 0 {
			return resp, ImpactParamError{Msg: "replace 必须是当前已装备的装备 id"}
		}
		old, ok := byID[replacedID]
		if !ok {
			return resp, ImpactParamError{Msg: "replace 不是有效装备"}
		}
		replacedName, candidateName = old.Name, def.Name
		armorDelta = def.BaseArmor - old.BaseArmor
		att := impactReadAttacker(candidateBuild)
		baseAttack := domain.DefaultAttacker().Attack
		gearAttack := att.Attack - baseAttack
		if gearAttack < 0 {
			gearAttack = 0
		}
		newGearAttack := gearAttack + def.BaseBonusPct - old.BaseBonusPct
		if newGearAttack > MaxLoadoutAttackPermille {
			newGearAttack = MaxLoadoutAttackPermille
		}
		if newGearAttack < 0 {
			newGearAttack = 0
		}
		attackDelta = baseAttack + newGearAttack - att.Attack
		newArmor := att.Armor + armorDelta
		if newArmor > domain.MaxArmorPermille {
			newArmor = domain.MaxArmorPermille
		}
		if newArmor < 0 {
			newArmor = 0
		}
		armorDelta = newArmor - att.Armor
		impactWriteAttacker(candidateBuild, impactAttacker{Attack: baseAttack + newGearAttack, Armor: newArmor})
		next := append([]int(nil), equipped...)
		next[idx] = candidateID
		candidateBuild["equipment"] = next
	}

	baseline := impactSide(build, cfg, computeRating(build, parts.masteryPoints), hitsPerWave, reactionsPerWave)
	candSide := impactSide(candidateBuild, cfg, computeRating(candidateBuild, parts.masteryPoints), hitsPerWave, reactionsPerWave)
	pct := 0.0
	if baseline.EstimatedDamagePerWave > 0 {
		pct = (candSide.EstimatedDamagePerWave - baseline.EstimatedDamagePerWave) / baseline.EstimatedDamagePerWave * 100
	}
	resp.Impact = &ImpactComparison{
		Kind: kind, CandidateID: candidateID, ReplacedID: replacedID,
		CandidateName: candidateName, ReplacedName: replacedName,
		Baseline: baseline, Candidate: candSide,
		Changes: ImpactChanges{
			PowerDelta:              candSide.Power - baseline.Power,
			RatingTotalDelta:        candSide.RatingTotal - baseline.RatingTotal,
			AttackPermilleDelta:     attackDelta,
			ArmorPermilleDelta:      armorDelta,
			DamagePerWaveDelta:      candSide.EstimatedDamagePerWave - baseline.EstimatedDamagePerWave,
			DamagePerWavePct:        pct,
			StarRateObserved:        starRate,
			StarProbabilityEstimate: nil,
		},
		Method:       "命中/波×技能底伤(等级)×(1000+攻击)/1000×直接权重400‰；攻击/装备数值来自 /config，命中来自 battle_records",
		StarRateNote: "只给历史三星率；分数由漏怪率主导（见 score_rules 注释），换装到星级的因果模型不存在，故不推测概率",
	}

	resp.Evidence = []RecEvidence{
		{Metric: "历史命中/波", Value: hitsPerWave, Unit: "次/波", Source: "battle_records", Window: "7d"},
		{Metric: "历史反应/波", Value: reactionsPerWave, Unit: "次/波", Source: "battle_records", Window: "7d"},
		{Metric: "历史三星率", Value: starRate, Unit: "%", Source: "battle_records", Window: "7d"},
		{Metric: "当前攻击力", Value: float64(baseline.AttackPermille), Unit: "‰", Source: "build.attacker", Window: "current"},
		{Metric: "技能底伤均值", Value: baseline.AvgSkillDamage, Unit: "点", Source: "config:" + cfg.Version, Window: "current"},
		{Metric: "直接伤害权重", Value: float64(domain.DirectWeightPermille), Unit: "‰", Source: "domain.damage.go", Window: "current"},
	}
	route := "/pages/equipment/equipment"
	label := "去换装"
	if kind == "skill" {
		route = "/pages/skills/skills"
		label = "去换技能"
	}
	headline := fmt.Sprintf("换上「%s」后，按最近 7 天节奏估算每波伤害 %+.1f%%", candidateName, pct)
	if math.Abs(pct) < 0.05 {
		headline = fmt.Sprintf("换上「%s」后，按最近 7 天节奏估算伤害基本不变", candidateName)
	}
	resp.Insights = []RecInsight{recInsight(
		"impact-"+kind+"-"+strconv.Itoa(candidateID),
		headline, route, label,
		[]string{
			fmt.Sprintf("近 7 天 %d 场，命中/波 %.1f", st.Count, hitsPerWave),
			fmt.Sprintf("攻击 %d‰ → %d‰", baseline.AttackPermille, candSide.AttackPermille),
			"星级概率无可信模型，未做推测",
		},
		map[string]any{"kind": kind, "id": candidateID, "replace": replacedID},
	)}
	return resp, nil
}
