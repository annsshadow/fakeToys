// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R13 冷启动成长候选。
//
// 场景：新号/低战报数时 rec-v1 只能给 NO_HISTORY，玩家仍不知道"现在该做什么"。
// 这里从**当前构筑快照 + /api/v1/config 的真实数值**推导少量可执行候选，
// 并在响应里显式 based_on=rule_only，绝不伪装成历史战绩结论。

import "fmt"

const recColdStartThreshold = 3

// RecColdStart 是战报不足阈值时的规则候选容器。
type RecColdStart struct {
	BasedOn       string                  `json:"based_on"`
	Label         string                  `json:"label"`
	Threshold     int                     `json:"threshold"`
	BattleCount   int                     `json:"battle_count"`
	ConfigVersion string                  `json:"config_version"`
	Candidates    []RecColdStartCandidate `json:"candidates"`
}

// RecColdStartCandidate 是一条能直接跳转、且带来源的规则建议。
type RecColdStartCandidate struct {
	ID       string             `json:"id"`
	Headline string             `json:"headline"`
	Basis    []string           `json:"basis"`
	CTA      RecCTA             `json:"cta"`
	Source   RecColdStartSource `json:"source"`
}

// RecColdStartSource 记录候选来自哪份配置、哪条规则、哪些字段。
type RecColdStartSource struct {
	ConfigVersion string   `json:"config_version"`
	Rule          string   `json:"rule"`
	Fields        []string `json:"fields"`
}

// coldStartRecommendations 只用构建快照与配置常量推导候选，不查历史战报。
// 战报数由调用方从 battle_records 实读后传入，因此 threshold/battle_count 都是真值。
func (s *Service) coldStartRecommendations(build map[string]any, cfg GameConfig, cfgVersion string, battleCount int) *RecColdStart {
	cs := &RecColdStart{
		BasedOn:       "rule_only",
		Label:         "基于规则而非你的历史战报",
		Threshold:     recColdStartThreshold,
		BattleCount:   battleCount,
		ConfigVersion: cfgVersion,
		Candidates:    []RecColdStartCandidate{},
	}

	// 候选 1：升级当前已拥有、未满级的主战技能。等级来自真实构筑，
	// 费用来自与写入端同一个 skillRules，不臆造数值。
	if skills, ok := build["skills"].(map[string]any); ok {
		bestID, bestName, bestLevel := 0, "", 0
		for _, raw := range skills {
			m, ok := raw.(map[string]any)
			if !ok || fmt.Sprint(m["kind"]) != "active" {
				continue
			}
			id, level := int(impactInt(m, "id")), int(impactInt(m, "level"))
			if id <= 0 || level >= skillRules.MaxLevel {
				continue
			}
			if bestID == 0 || level < bestLevel || (level == bestLevel && id < bestID) {
				bestID, bestName, bestLevel = id, fmt.Sprint(m["name"]), level
			}
		}
		if bestID > 0 {
			cost := skillRules.CostFrom(bestLevel)
			cs.Candidates = append(cs.Candidates, RecColdStartCandidate{
				ID:       fmt.Sprintf("skill-upgrade-%d", bestID),
				Headline: fmt.Sprintf("把 %s 升到 %d 级", bestName, bestLevel+1),
				Basis: []string{
					fmt.Sprintf("当前等级 %d 来自你的真实构筑快照", bestLevel),
					fmt.Sprintf("升级消耗 %d 金币，按配置的 skill_rules 计算（base_cost=%d，每级系数 %d‰）", cost, cfg.SkillRules.BaseCost, cfg.SkillRules.CoefPermille),
					fmt.Sprintf("配置版本 %s，未使用历史战报", cfgVersion),
				},
				CTA: RecCTA{Label: "去升级技能", Route: "/pages/skills/skills", Params: map[string]any{"skill_id": bestID, "focus": "level"}},
				Source: RecColdStartSource{
					ConfigVersion: cfgVersion,
					Rule:          "lowest_level_owned_active_skill",
					Fields:        []string{"build.skills[].level", "skill_rules.base_cost", "skill_rules.coef_permille", "config.version"},
				},
			})
		}
	}

	// 候选 2：换同槽更高加成的已装备项。装备 id 来自真实构筑，
	// 加成数值来自当前配置，不引用未拥有物品。
	if equipped, ok := build["equipment"].([]int); ok && len(equipped) > 0 {
		byID := map[int]int64{}
		for _, eq := range cfg.Equipment {
			byID[eq.ID] = eq.BaseBonusPct
		}
		worstID, worstBonus := 0, int64(0)
		for _, id := range equipped {
			bonus, exists := byID[id]
			if !exists {
				continue
			}
			if worstID == 0 || int64(bonus) < worstBonus {
				worstID, worstBonus = id, int64(bonus)
			}
		}
		if worstID > 0 {
			cs.Candidates = append(cs.Candidates, RecColdStartCandidate{
				ID:       fmt.Sprintf("equipment-focus-%d", worstID),
				Headline: fmt.Sprintf("检查 %s 的同槽搭配", eqName(cfg, worstID)),
				Basis: []string{
					fmt.Sprintf("当前装备 id=%d 在配置中的同槽基础加成为 %d‰", worstID, worstBonus),
					fmt.Sprintf("配置版本 %s；候选只来自 /api/v1/config，未读取历史战报", cfgVersion),
				},
				CTA: RecCTA{Label: "查看装备搭配", Route: "/pages/equipment/equipment", Params: map[string]any{"equipment_id": worstID, "focus": "slot"}},
				Source: RecColdStartSource{
					ConfigVersion: cfgVersion,
					Rule:          "lowest_bonus_equipped_item",
					Fields:        []string{"build.equipment", "equipment[].base_bonus_pct", "equipment[].slot", "config.version"},
				},
			})
		}
	}

	// 候选 3：宝石属性只来自配置。战力公式当前不读宝石，所以这里只引导查看，
	// 不承诺战力提升 —— 避免把"成长建议"写成假收益。
	if len(cfg.Gems) > 0 {
		gem := cfg.Gems[0]
		cs.Candidates = append(cs.Candidates, RecColdStartCandidate{
			ID:       fmt.Sprintf("gem-attr-%d", gem.ID),
			Headline: fmt.Sprintf("了解 %s 的 %s 属性", gem.Name, gem.Attr),
			Basis: []string{
				fmt.Sprintf("配置 id=%d 的宝石属性为 %s：%s", gem.ID, gem.Attr, gem.Descr),
				"当前战力公式只读技能等级与专精，本候选仅帮助理解玩法，不承诺战力数值",
				fmt.Sprintf("配置版本 %s，未使用历史战报", cfgVersion),
			},
			CTA: RecCTA{Label: "去背包查看宝石", Route: "/pages/bag/bag", Params: map[string]any{"gem_id": gem.ID, "attr": gem.Attr}},
			Source: RecColdStartSource{
				ConfigVersion: cfgVersion,
				Rule:          "first_config_gem_attribute",
				Fields:        []string{"gems[].id", "gems[].attr", "gems[].descr", "config.version"},
			},
		})
	}
	return cs
}

// eqName 只做配置查名，避免候选 headline 出现编造的装备名。
func eqName(cfg GameConfig, id int) string {
	for _, eq := range cfg.Equipment {
		if eq.ID == id {
			return eq.Name
		}
	}
	return fmt.Sprintf("装备 %d", id)
}
