// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R6 §1E：真实数据驱动的推荐接口（只读规则引擎）。
//
// 设计底线：
//   - 结论必须能追到 battle_records / user_progress / config 版本 / 看板聚合；
//   - 没有历史就明说 NO_HISTORY，不回退写死文案；
//   - user_key 只输出 hash，不把昵称或账号 id 送进响应；
//   - confidence 是文字标记，不编造小数置信度，也不冒充 LLM。

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"strconv"
	"strings"
	"time"

	"github.com/jackc/pgx/v5"
)

const (
	recSchemaVersion = "rec-v1"
	recWindowDays    = 7
)

var recUserContexts = map[string]bool{
	"home": true, "equipment": true, "reactions": true, "skills": true,
	"stage": true, "bag": true, "gems": true, "shop": true,
}

var recAdminScopes = map[string]bool{
	"battles": true, "dashboard": true, "levels": true, "economy": true,
	"skills": true, "users": true, "recomputes": true, "content": true,
	"defenses": true, "equipment": true, "wiki": true,
}

// RecEvidence 是一条可追溯依据。
type RecEvidence struct {
	Metric string  `json:"metric"`
	Value  float64 `json:"value"`
	Unit   string  `json:"unit"`
	Source string  `json:"source"`
	Window string  `json:"window"`
}

// RecCTA 是前端必须照着跳转的行动入口（不许前端写死路由）。
type RecCTA struct {
	Label  string         `json:"label"`
	Route  string         `json:"route"`
	Params map[string]any `json:"params"`
}

// RecInsight 是一条结论 + 依据 + CTA。
type RecInsight struct {
	ID         string   `json:"id"`
	Headline   string   `json:"headline"`
	Basis      []string `json:"basis"`
	CTA        RecCTA   `json:"cta"`
	Confidence string   `json:"confidence"`
	Model      RecModel `json:"model"`
}

// RecEmptyReason 说明缺什么数据、下一步该做什么。
type RecEmptyReason struct {
	Code    string `json:"code"`
	Message string `json:"message"`
}

// RecModel 明确标注是规则引擎，不宣称 GPT/LLM。
type RecModel struct {
	Name    string `json:"name"`
	Version string `json:"version"`
	Kind    string `json:"kind"`
}

// RecSubject 只带 hash 化标识与真实进度数值。
type RecSubject struct {
	UserKey string `json:"user_key"`
	Level   int    `json:"level"`
	Power   int64  `json:"power"`
}

// RecInputSnapshot 写实际参与计算的窗口与版本。
type RecInputSnapshot struct {
	BattleCount   int    `json:"battle_count"`
	WindowDays    int    `json:"window_days"`
	ConfigVersion string `json:"config_version"`
}

// RecResponse 是用户侧与管理侧共用的响应形状。
type RecResponse struct {
	SchemaVersion string           `json:"schema_version"`
	GeneratedAt   string           `json:"generated_at"`
	Context       string           `json:"context"`
	Subject       RecSubject       `json:"subject"`
	InputSnapshot RecInputSnapshot `json:"input_snapshot"`
	Evidence      []RecEvidence    `json:"evidence"`
	Insights      []RecInsight     `json:"insights"`
	EmptyReason   *RecEmptyReason  `json:"empty_reason"`
	Model         RecModel         `json:"model"`
}

func recModel() RecModel {
	return RecModel{Name: "rules", Version: recSchemaVersion, Kind: "rule_based"}
}

func recUserKey(kind string, id int64) string {
	sum := sha256.Sum256([]byte("laiyipao-rec-v1:" + kind + ":" + strconv.FormatInt(id, 10)))
	return hex.EncodeToString(sum[:16])
}

func recInsight(id, headline, route, label string, basis []string, params map[string]any) RecInsight {
	if params == nil {
		params = map[string]any{}
	}
	if basis == nil {
		basis = []string{}
	}
	return RecInsight{
		ID: id, Headline: headline, Basis: basis,
		CTA:        RecCTA{Label: label, Route: route, Params: params},
		Confidence: "rule_based", Model: recModel(),
	}
}

// recOwnedIDs 返回玩家真实拥有的技能/装备 id（不等于"当前装备"）。
// 写入端 SaveLoadout / SetEquippedEquipment 都以这两张表为归属事实源，
// 推荐侧必须读同一事实源，否则两步确认弹层会指向必然 400 的 id。
func (s *Service) recOwnedIDs(ctx context.Context, userID int64, kind string) (map[int]bool, error) {
	query := `SELECT equipment_id FROM user_equipment WHERE user_id = $1`
	if kind == "skill" {
		query = `SELECT skill_id FROM user_skills WHERE user_id = $1`
	}
	rows, err := s.pool.Query(ctx, query, userID)
	if err != nil {
		return nil, fmt.Errorf("recommendation owned %s: %w", kind, err)
	}
	defer rows.Close()
	owned := map[int]bool{}
	for rows.Next() {
		var id int
		if err := rows.Scan(&id); err != nil {
			return nil, fmt.Errorf("scan owned %s: %w", kind, err)
		}
		owned[id] = true
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("iterate owned %s: %w", kind, err)
	}
	return owned, nil
}

// recImpactCTA 从当前构筑、配置与**玩家已拥有集合**推导可用于 /me/recommendations/impact 的替换关系。
//
// 只下发"配置里真实存在、玩家进度已解锁、玩家已拥有、且当前确实装在身上"的关系：
// 候选必须是同槽（装备）或同为主战（技能）且数值更优的项；任何一项不满足
// 就不给 id/replace，让前端跳过收益卡而不是发出注定 400 的请求。
func (s *Service) recImpactCTA(ctx context.Context, userID int64, build map[string]any, cfg GameConfig, maxStage int, context string) (map[string]any, string) {
	if maxStage <= 0 {
		return nil, ""
	}
	switch context {
	case "skills":
		equipped, _ := build["skills"].(map[string]any)
		owned, err := s.recOwnedIDs(ctx, userID, "skill")
		if err != nil {
			return nil, "" // 归属查不到就 fail closed，绝不下发未经验证的 id
		}
		damage := map[int]int64{}
		kind := map[int]string{}
		unlock := map[int]int{}
		name := map[int]string{}
		all := make([]int, 0, len(cfg.Skills)+len(cfg.Composite))
		for _, sk := range cfg.Skills {
			damage[sk.ID], kind[sk.ID], unlock[sk.ID], name[sk.ID] = sk.BaseDamage, sk.Kind, sk.UnlockLevel, sk.Name
			all = append(all, sk.ID)
		}
		for _, sk := range cfg.Composite {
			damage[sk.ID], kind[sk.ID], unlock[sk.ID], name[sk.ID] = sk.BaseDamage, sk.Kind, sk.UnlockLevel, sk.Name
			all = append(all, sk.ID)
		}
		replacedID, replacedDamage := 0, int64(0)
		for _, raw := range equipped {
			m, _ := raw.(map[string]any)
			id := int(impactInt(m, "id"))
			slot := int(impactInt(m, "slot"))
			if id <= 0 || slot < 0 || kind[id] != "active" {
				continue
			}
			if replacedID == 0 || damage[id] < replacedDamage || (damage[id] == replacedDamage && id < replacedID) {
				replacedID, replacedDamage = id, damage[id]
			}
		}
		if replacedID == 0 {
			return nil, ""
		}
		for _, id := range all {
			if kind[id] != "active" || unlock[id] > maxStage || damage[id] <= replacedDamage {
				continue
			}
			if !owned[id] {
				continue
			}
			// build.skills 包含所有已拥有技能，未上槽的 slot=-1；只有真正
			// 占槽的才排除，否则刚授予的候选会被误判为"已装备"而永远推不出来。
			if m, ok := equipped[strconv.Itoa(id)].(map[string]any); ok && impactInt(m, "slot") >= 0 {
				continue
			}
			return map[string]any{"id": id, "replace": replacedID},
				fmt.Sprintf("候选 %s(id=%d) 已在你的已拥有技能中，基础伤害 %d，高于当前 %s(id=%d) 的 %d（配置版本 %s）",
					name[id], id, damage[id], name[replacedID], replacedID, replacedDamage, cfg.Version)
		}
		return nil, fmt.Sprintf("已拥有技能里没有同为主战且基础伤害高于当前 %s(id=%d) 的候选，未给出替换 id", name[replacedID], replacedID)
	case "equipment":
		equipped, _ := build["equipment"].([]int)
		owned, err := s.recOwnedIDs(ctx, userID, "equipment")
		if err != nil {
			return nil, "" // 归属查不到就 fail closed
		}
		bonus := map[int]int64{}
		slot := map[int]string{}
		unlock := map[int]int{}
		name := map[int]string{}
		for _, eq := range cfg.Equipment {
			bonus[eq.ID], slot[eq.ID], unlock[eq.ID], name[eq.ID] = eq.BaseBonusPct, eq.Slot, eq.UnlockLevel, eq.Name
		}
		on := map[int]bool{}
		replacedID, replacedBonus := 0, int64(0)
		for _, id := range equipped {
			on[id] = true
			if _, ok := bonus[id]; !ok {
				continue
			}
			if replacedID == 0 || bonus[id] < replacedBonus || (bonus[id] == replacedBonus && id < replacedID) {
				replacedID, replacedBonus = id, bonus[id]
			}
		}
		if replacedID == 0 {
			return nil, ""
		}
		for _, eq := range cfg.Equipment {
			if on[eq.ID] || eq.Slot != slot[replacedID] || eq.UnlockLevel > maxStage || eq.BaseBonusPct <= replacedBonus {
				continue
			}
			if !owned[eq.ID] {
				continue
			}
			return map[string]any{"id": eq.ID, "replace": replacedID},
				fmt.Sprintf("候选 %s(id=%d) 已在你的已拥有装备中，同槽加成 %d‰，高于当前 %s(id=%d) 的 %d‰（配置版本 %s）",
					eq.Name, eq.ID, eq.BaseBonusPct, name[replacedID], replacedID, replacedBonus, cfg.Version)
		}
		return nil, fmt.Sprintf("已拥有装备里没有同槽且加成高于当前 %s(id=%d) 的候选，未给出替换 id", name[replacedID], replacedID)
	}
	return nil, ""
}

func recEmpty(code, message string) *RecEmptyReason {
	return &RecEmptyReason{Code: code, Message: message}
}

type recBattleStats struct {
	Count       int64
	Wins        int64
	Losses      int64
	Stars       int64
	Kills       int64
	Waves       int64
	FailWaves   int64
	Reactions   int64
	TopReaction string
	ReactionN   int64
	TopElement  string
	ElementN    int64
}

func (s *Service) recBattleStats(ctx context.Context, userID int64, days int) (recBattleStats, error) {
	var st recBattleStats
	err := s.pool.QueryRow(ctx, `
		SELECT COUNT(*),
		       COUNT(*) FILTER (WHERE result = 'win'),
		       COUNT(*) FILTER (WHERE result = 'lose'),
		       COALESCE(SUM(stars), 0),
		       COALESCE(SUM(kills), 0),
		       COALESCE(SUM(wave_reached), 0),
		       COALESCE(SUM(wave_reached) FILTER (WHERE result = 'lose'), 0),
		       COALESCE(SUM(reactions), 0)
		  FROM battle_records
		 WHERE user_id = $1
		   AND created_at >= now() - ($2::int * interval '1 day')`, userID, days).Scan(
		&st.Count, &st.Wins, &st.Losses, &st.Stars, &st.Kills, &st.Waves, &st.FailWaves, &st.Reactions)
	if err != nil {
		return st, fmt.Errorf("recommendation battle stats: %w", err)
	}
	// 反应/元素分布来自真实 JSONB 列；没有触发时留空字符串，由调用方走诚实空态。
	if err := s.pool.QueryRow(ctx, `
		SELECT COALESCE(key, ''), COALESCE(SUM(value::int), 0)
		  FROM battle_records, jsonb_each_text(reactions_used)
		 WHERE user_id = $1
		   AND created_at >= now() - ($2::int * interval '1 day')
		 GROUP BY key ORDER BY 2 DESC, key LIMIT 1`, userID, days).Scan(&st.TopReaction, &st.ReactionN); err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			st.TopReaction, st.ReactionN = "", 0
		} else {
			return st, fmt.Errorf("recommendation reaction stats: %w", err)
		}
	}
	if err := s.pool.QueryRow(ctx, `
		SELECT COALESCE(key, ''), COALESCE(SUM(value::int), 0)
		  FROM battle_records, jsonb_each_text(elements_used)
		 WHERE user_id = $1
		   AND created_at >= now() - ($2::int * interval '1 day')
		 GROUP BY key ORDER BY 2 DESC, key LIMIT 1`, userID, days).Scan(&st.TopElement, &st.ElementN); err != nil {
		if errors.Is(err, pgx.ErrNoRows) {
			st.TopElement, st.ElementN = "", 0
		} else {
			return st, fmt.Errorf("recommendation element stats: %w", err)
		}
	}
	return st, nil
}

// UserRecommendations 返回某个 H5 上下文的推荐。context 必须是 §1E.2 枚举值。
func (s *Service) UserRecommendations(ctx context.Context, userID int64, rawContext string) (RecResponse, error) {
	contextName := strings.TrimSpace(rawContext)
	if !recUserContexts[contextName] {
		return RecResponse{}, fmt.Errorf("%w: 不支持的推荐 context %q", ErrBadInput, rawContext)
	}
	build, rating, maxStage, err := s.LoadMeProfile(ctx, userID)
	if err != nil {
		return RecResponse{}, err
	}
	cfgVersion := s.ConfigVersion()
	gameCfg := s.LoadGameConfig()
	resp := RecResponse{
		SchemaVersion: recSchemaVersion,
		GeneratedAt:   time.Now().UTC().Format(time.RFC3339),
		Context:       contextName,
		Subject: RecSubject{
			UserKey: recUserKey("user", userID),
			Level:   maxStage,
			Power:   s.ComputePowerFor(build),
		},
		InputSnapshot: RecInputSnapshot{WindowDays: recWindowDays, ConfigVersion: cfgVersion},
		Evidence:      []RecEvidence{},
		Insights:      []RecInsight{},
		Model:         recModel(),
	}
	if cfgVersion == "" {
		resp.EmptyReason = recEmpty("CONFIG_MISSING", "当前没有可用的配置版本：请等待配置发布完成，再从首页重新进入推荐卡。")
		return resp, nil
	}
	st, err := s.recBattleStats(ctx, userID, recWindowDays)
	if err != nil {
		return RecResponse{}, err
	}
	resp.InputSnapshot.BattleCount = int(st.Count)
	winRate := float64(0)
	avgStars := float64(0)
	avgWaves := float64(0)
	if st.Count > 0 {
		winRate = float64(st.Wins) * 100 / float64(st.Count)
		avgStars = float64(st.Stars) / float64(st.Count)
		avgWaves = float64(st.Waves) / float64(st.Count)
	}
	resp.Evidence = append(resp.Evidence,
		RecEvidence{Metric: "近 7 场胜率", Value: winRate, Unit: "%", Source: "battles", Window: "7d"},
		RecEvidence{Metric: "近 7 场平均星级", Value: avgStars, Unit: "star", Source: "battles", Window: "7d"},
		RecEvidence{Metric: "构筑评分", Value: float64(rating.Total), Unit: "point", Source: "profile", Window: "profile"},
		RecEvidence{Metric: "元素反应触发数", Value: float64(st.Reactions), Unit: "count", Source: "battles", Window: "7d"},
	)
	if st.Count == 0 {
		resp.EmptyReason = recEmpty("NO_HISTORY", "近 7 天还没有战斗记录：先完成一场对局，推荐卡会按你的真实战绩与构筑生成结论。")
		return resp, nil
	}

	weakness := ""
	if len(rating.Weaknesses) > 0 {
		weakness = rating.Weaknesses[0]
	}
	winText := fmt.Sprintf("%.0f", winRate)
	starText := fmt.Sprintf("%.1f", avgStars)
	waveText := fmt.Sprintf("%.1f", avgWaves)

	switch contextName {
	case "home":
		resp.Insights = append(resp.Insights, recInsight(
			"home-form", "近 7 场胜率 "+winText+"%，平均 "+starText+" 星", "/pages/stage/stage", "去看关卡",
			[]string{fmt.Sprintf("近 7 天共 %d 场战斗", st.Count), fmt.Sprintf("胜率 %s%% 来自 battle_records.result", winText)},
			map[string]any{"context": "stage"}))
	case "equipment":
		headline := fmt.Sprintf("当前装备协同 %d/18，专精已投入 %d 点", rating.EquipmentSynergy, rating.MasteryDone)
		if weakness != "" {
			headline = "构筑短板「" + weakness + "」：优先调整装备与专精"
		}
		params := map[string]any{"slot": 0}
		basis := []string{fmt.Sprintf("装备协同 %d/18、元素覆盖 %d/5 来自 /me 构筑快照", rating.EquipmentSynergy, rating.ElementCoverage), fmt.Sprintf("配置版本 %s", cfgVersion)}
		impactParams, impactBasis := s.recImpactCTA(ctx, userID, build, gameCfg, maxStage, "equipment")
		if impactBasis != "" {
			basis = append(basis, impactBasis)
		}
		for k, v := range impactParams {
			params[k] = v
		}
		resp.Insights = append(resp.Insights, recInsight(
			"equip-synergy", headline, "/pages/equipment/equipment", "查看搭配", basis, params))
	case "reactions":
		if st.TopReaction == "" {
			resp.EmptyReason = recEmpty("NOT_ENOUGH_DATA", "近 7 天有战斗但没有记录元素反应触发：在战斗中用两种元素命中同一目标后，再看反应链结论。")
			return resp, nil
		}
		resp.Insights = append(resp.Insights, recInsight(
			"reaction-chain", "主反应「"+st.TopReaction+"」近 7 天触发 "+fmt.Sprint(st.ReactionN)+" 次", "/pages/reactions/reactions", "查看反应链",
			[]string{fmt.Sprintf("来自 battle_records.reactions_used，近 7 天共 %d 次触发", st.Reactions), fmt.Sprintf("元素覆盖 %d/5，短板 %q", rating.ElementCoverage, weakness)},
			map[string]any{"reaction": st.TopReaction}))
	case "skills":
		params := map[string]any{"context": "skills"}
		basis := []string{fmt.Sprintf("场均击杀来自 battle_records.kills（%d 场）", st.Count), fmt.Sprintf("元素覆盖 %d/5", rating.ElementCoverage)}
		impactParams, impactBasis := s.recImpactCTA(ctx, userID, build, gameCfg, maxStage, "skills")
		if impactBasis != "" {
			basis = append(basis, impactBasis)
		}
		for k, v := range impactParams {
			params[k] = v
		}
		resp.Insights = append(resp.Insights, recInsight(
			"skill-tempo", "近 7 场场均击杀 "+fmt.Sprintf("%.1f", float64(st.Kills)/float64(st.Count))+"，平均推进到第 "+waveText+" 波", "/pages/skills/skills", "调整技能", basis, params))
	case "stage":
		failText := "0"
		if st.Losses > 0 {
			failText = fmt.Sprintf("%.1f", float64(st.FailWaves)/float64(st.Losses)*10/10)
		}
		resp.Insights = append(resp.Insights, recInsight(
			"stage-difficulty", "近 7 场胜率 "+winText+"%，失败平均到第 "+failText+" 波", "/pages/stage/stage", "选择关卡",
			[]string{fmt.Sprintf("失败波次来自 battle_records.wave_reached（%d 场失败）", st.Losses), fmt.Sprintf("当前最高关卡 %d", maxStage)},
			map[string]any{"max_stage": maxStage}))
	case "bag":
		headline := fmt.Sprintf("背包构筑评分 %d，装备协同 %d/18", rating.Total, rating.EquipmentSynergy)
		if weakness != "" {
			headline = "背包里优先补「" + weakness + "」方向的装备/宝石"
		}
		resp.Insights = append(resp.Insights, recInsight(
			"bag-build", headline, "/pages/bag/bag", "整理背包",
			[]string{fmt.Sprintf("评分来自 /me 构筑快照，战力 %d", resp.Subject.Power), fmt.Sprintf("配置版本 %s", cfgVersion)},
			map[string]any{"context": "bag"}))
	case "gems":
		headline := fmt.Sprintf("当前元素覆盖 %d/5，常用元素 %q", rating.ElementCoverage, st.TopElement)
		if weakness != "" {
			headline = "宝石优先补「" + weakness + "」：当前元素覆盖 " + fmt.Sprint(rating.ElementCoverage) + "/5"
		}
		resp.Insights = append(resp.Insights, recInsight(
			"gem-element", headline, "/pages/gems/gems", "查看宝石",
			[]string{fmt.Sprintf("常用元素来自 battle_records.elements_used（%d 场）", st.Count), fmt.Sprintf("配置版本 %s", cfgVersion)},
			map[string]any{"element": st.TopElement}))
	case "shop":
		headline := fmt.Sprintf("当前战力 %d、评分 %d，优先兑换构筑缺口", resp.Subject.Power, rating.Total)
		if weakness != "" {
			headline = "兑换优先补「" + weakness + "」：构筑短板来自真实快照"
		}
		resp.Insights = append(resp.Insights, recInsight(
			"shop-gap", headline, "/pages/shop/shop", "去商店",
			[]string{fmt.Sprintf("近 7 场胜率 %s%%，平均 %s 星", winText, starText), fmt.Sprintf("评分 %d 来自 /me 构筑快照", rating.Total)},
			map[string]any{"context": "shop"}))
	}
	if len(resp.Insights) == 0 {
		resp.EmptyReason = recEmpty("NOT_ENOUGH_DATA", "当前窗口的数据不足以给出可执行结论：再打几场对局或完善构筑后再看。")
	}
	return resp, nil
}

// AdminRecommendations 从看板聚合推导管理侧结论。window_days=0 表示全量累计口径。
func (s *Service) AdminRecommendations(ctx context.Context, adminID int64, scope string) (RecResponse, error) {
	scopeName := strings.TrimSpace(scope)
	if !recAdminScopes[scopeName] {
		return RecResponse{}, fmt.Errorf("%w: 不支持的推荐 scope %q", ErrBadInput, scope)
	}
	d, err := s.AdminDashboard(ctx)
	if err != nil {
		return RecResponse{}, err
	}
	cfgVersion := s.ConfigVersion()
	resp := RecResponse{
		SchemaVersion: recSchemaVersion,
		GeneratedAt:   time.Now().UTC().Format(time.RFC3339),
		Context:       scopeName,
		Subject:       RecSubject{UserKey: recUserKey("admin", adminID)},
		InputSnapshot: RecInputSnapshot{BattleCount: int(d.Battles.Total), WindowDays: 0, ConfigVersion: cfgVersion},
		Evidence:      []RecEvidence{},
		Insights:      []RecInsight{},
		Model:         recModel(),
	}
	if cfgVersion == "" {
		resp.EmptyReason = recEmpty("CONFIG_MISSING", "当前没有可用的配置版本：请等待配置发布完成，再从看板重新进入洞察卡。")
		return resp, nil
	}
	resp.Evidence = append(resp.Evidence,
		RecEvidence{Metric: "累计战报", Value: float64(d.Battles.Total), Unit: "count", Source: "dashboard", Window: "all"},
		RecEvidence{Metric: "累计胜场", Value: float64(d.Battles.Wins), Unit: "count", Source: "dashboard", Window: "all"},
		RecEvidence{Metric: "配置版本", Value: 0, Unit: "sha", Source: "config", Window: "current"},
	)
	if d.Battles.Total == 0 {
		resp.EmptyReason = recEmpty("NO_HISTORY", "看板还没有战报：等玩家完成首场对局后再看，或到关卡配置页检查发布状态。")
		return resp, nil
	}
	total := fmt.Sprint(d.Battles.Total)
	wins := fmt.Sprint(d.Battles.Wins)
	rate := float64(d.Battles.Wins) * 100 / float64(d.Battles.Total)
	rateText := fmt.Sprintf("%.0f%%", rate)
	add := func(id, headline, route, label string, basis []string, params map[string]any) {
		resp.Insights = append(resp.Insights, recInsight(id, headline, route, label, basis, params))
	}
	switch scopeName {
	case "battles", "dashboard":
		add("admin-battle-volume", "全站累计 "+total+" 场、胜 "+wins+" 场（"+rateText+"）", "/admin/battles", "查看战报",
			[]string{fmt.Sprintf("来自 AdminDashboard.Battles，今日 %d 场", d.Battles.Today), fmt.Sprintf("配置版本 %s", cfgVersion)},
			map[string]any{"scope": scopeName})
	case "levels":
		peaks := len(d.StageFunnel)
		add("admin-level-funnel", "关卡漏斗覆盖 "+fmt.Sprint(peaks)+" 关，累计 "+total+" 场尝试", "/admin/levels", "查看关卡",
			[]string{fmt.Sprintf("漏斗数据来自 dashboard_battle_stats 聚合（%d 关）", peaks), fmt.Sprintf("累计胜场 %s", wins)},
			map[string]any{"scope": "levels"})
	case "economy":
		add("admin-economy-flow", "金币流入 "+fmt.Sprint(d.Economy.CoinIn)+" / 流出 "+fmt.Sprint(d.Economy.CoinOut), "/admin/economy", "查看经济",
			[]string{fmt.Sprintf("来自 AdminDashboard.Economy，累计战报 %s 场", total), fmt.Sprintf("配置版本 %s", cfgVersion)},
			map[string]any{"scope": "economy"})
	case "skills":
		top := ""
		if len(d.ReactionUsage) > 0 {
			top = "高频反应：" + d.ReactionUsage[0].Reaction + " " + fmt.Sprint(d.ReactionUsage[0].Count) + " 次"
		} else {
			top = "反应分布暂未记录"
		}
		add("admin-reaction-mix", top, "/admin/skills", "查看技能",
			[]string{fmt.Sprintf("来自 AdminDashboard.ReactionUsage（%d 种反应）", len(d.ReactionUsage)), fmt.Sprintf("累计战报 %s 场", total)},
			map[string]any{"scope": "skills"})
	case "users":
		add("admin-user-split", "注册用户 "+fmt.Sprint(d.Users.Total)+"，游客 "+fmt.Sprint(d.Users.Guests)+"，今日新增 "+fmt.Sprint(d.Users.NewToday), "/admin/users", "查看用户",
			[]string{fmt.Sprintf("来自 AdminDashboard.Users，累计战报 %s 场", total), fmt.Sprintf("配置版本 %s", cfgVersion)},
			map[string]any{"scope": "users"})
	case "recomputes":
		add("admin-recompute-health", "验真 "+fmt.Sprint(d.Verification.Checked)+" 次，不符 "+fmt.Sprint(d.Verification.Mismatched)+" 次", "/admin/recomputes", "查看重算",
			[]string{fmt.Sprintf("来自 AdminDashboard.Verification，累计战报 %s 场", total), fmt.Sprintf("配置版本 %s", cfgVersion)},
			map[string]any{"scope": "recomputes"})
	case "content", "wiki":
		add("admin-content-version", "当前配置版本 "+cfgVersion+"，反应条目 "+fmt.Sprint(len(d.ReactionUsage))+" 类", "/admin/content", "查看内容",
			[]string{fmt.Sprintf("版本来自 /config，累计战报 %s 场", total), fmt.Sprintf("看板反应分布 %d 类", len(d.ReactionUsage))},
			map[string]any{"scope": scopeName, "version": cfgVersion})
	case "defenses":
		add("admin-defense", "防线 "+fmt.Sprint(d.Defenses.Total)+" 条，今日挑战 "+fmt.Sprint(d.Defenses.ChallengesToday)+" 次", "/admin/defenses", "查看防线",
			[]string{fmt.Sprintf("来自 AdminDashboard.Defenses，累计战报 %s 场", total), fmt.Sprintf("配置版本 %s", cfgVersion)},
			map[string]any{"scope": "defenses"})
	case "equipment":
		add("admin-equipment-power", "玩家平均战力 "+fmt.Sprintf("%.0f", d.Progression.AvgPower)+"，平均关卡 "+fmt.Sprintf("%.1f", d.Progression.AvgMaxStage), "/admin/equipment", "查看装备",
			[]string{fmt.Sprintf("来自 AdminDashboard.Progression，累计战报 %s 场", total), fmt.Sprintf("配置版本 %s", cfgVersion)},
			map[string]any{"scope": "equipment"})
	}
	return resp, nil
}
