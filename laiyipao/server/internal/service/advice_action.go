// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

// R14：把「建议元素」翻译成服务端权威的可执行替换关系。
//
// 客户端只回传 element/reaction/source，绝不回传 candidate_id/replaced_id。
// 候选裁决完全复用 recImpactCTA（它已经用 recOwnedIDs 校验归属、用配置校验解锁与数值），
// 本文件只负责白名单、诚实空态、收益预估同步计算与证据留痕。

import (
	"context"
	"fmt"
	"strings"

	"github.com/laiyipao/server/internal/domain"
)

const adviceActionSchemaVersion = "advice-action-v1"

// AdviceActionEmptyReason 与 R6 空态口径一致，但 code 多两个本端点专用的原因。
type AdviceActionEmptyReason struct {
	Code     string `json:"code"`
	Headline string `json:"headline"`
	Message  string `json:"message"`
	CTA      string `json:"cta"`
	Route    string `json:"route"`
}

// AdviceAction 是 /me/advice/action 的完整响应；六字段是三端共同契约。
type AdviceAction struct {
	SchemaVersion    string                   `json:"schema_version"`
	Badge            string                   `json:"badge"`
	Element          string                   `json:"element"`
	ExpectedReaction string                   `json:"expected_reaction"`
	BasedOn          []string                 `json:"based_on"`
	Kind             any                      `json:"kind"`
	CandidateID      int                      `json:"candidate_id"`
	ReplacedID       int                      `json:"replaced_id"`
	CandidateName    string                   `json:"candidate_name"`
	ReplacedName     string                   `json:"replaced_name"`
	Params           any                      `json:"params"`
	PreviewRoute     string                   `json:"preview_route"`
	Evidence         []RecEvidence            `json:"evidence"`
	Impact           *ImpactComparison        `json:"impact"`
	EmptyReason      *AdviceActionEmptyReason `json:"empty_reason"`
}

func adviceEmpty(code, element, message string) AdviceAction {
	return AdviceAction{
		SchemaVersion:    adviceActionSchemaVersion,
		Badge:            "AI",
		Element:          element,
		ExpectedReaction: "",
		BasedOn:          []string{"battle_records", "/config"},
		Kind:             nil,
		CandidateID:      0,
		ReplacedID:       0,
		Params:           nil,
		Evidence:         []RecEvidence{},
		EmptyReason: &AdviceActionEmptyReason{
			Code:     code,
			Headline: "AI 暂时没有足够数据给出建议",
			Message:  message,
			CTA:      "去打一场战斗 →",
			Route:    "/pages/stage/stage",
		},
	}
}

// adviceJSONCount 从真实战报 JSONB 列按 key 汇总；没有命中时聚合仍返回一行 0。
func (s *Service) adviceJSONCount(ctx context.Context, userID int64, column, key string) (int64, error) {
	var n int64
	err := s.pool.QueryRow(ctx, fmt.Sprintf(`
		SELECT COALESCE(SUM(value::int), 0)
		  FROM battle_records, jsonb_each_text(%s)
		 WHERE user_id = $1 AND key = $2
		   AND created_at >= now() - ($3::int * interval '1 day')`, column),
		userID, key, recWindowDays).Scan(&n)
	if err != nil {
		return 0, fmt.Errorf("advice action %s stats: %w", column, err)
	}
	return n, nil
}

func adviceNames(cfg GameConfig, kind string, candidateID, replacedID int) (string, string) {
	var candidateName, replacedName string
	if kind == "skill" {
		all := append(append([]domain.SeedSkill{}, cfg.Skills...), cfg.Composite...)
		for _, sk := range all {
			if sk.ID == candidateID {
				candidateName = sk.Name
			}
			if sk.ID == replacedID {
				replacedName = sk.Name
			}
		}
		return candidateName, replacedName
	}
	for _, eq := range cfg.Equipment {
		if eq.ID == candidateID {
			candidateName = eq.Name
		}
		if eq.ID == replacedID {
			replacedName = eq.Name
		}
	}
	return candidateName, replacedName
}

// AdviceAction 只用真实战报、当前构筑与配置版本裁决；任何一步不满足都返回空态。
func (s *Service) AdviceAction(ctx context.Context, userID int64, rawElement, rawReaction, rawSource string) (AdviceAction, error) {
	element := strings.ToLower(strings.TrimSpace(rawElement))
	reaction := strings.TrimSpace(rawReaction)
	source := strings.ToLower(strings.TrimSpace(rawSource))
	resp := adviceEmpty("", element, "")
	resp.ExpectedReaction = reaction

	if !domain.IsElement(element) {
		resp.EmptyReason.Code = "INVALID_ELEMENT"
		resp.EmptyReason.Message = "元素不在五元素白名单里，不能给出一键替换建议。"
		return resp, nil
	}
	if userID == 0 {
		resp.EmptyReason.Code = "NOT_AUTHENTICATED"
		resp.EmptyReason.Message = "请先登录，再根据你自己的战报和已拥有目录生成替换建议。"
		return resp, nil
	}
	cfgVersion := s.ConfigVersion()
	if cfgVersion == "" {
		resp.EmptyReason.Code = "CONFIG_MISSING"
		resp.EmptyReason.Message = "当前没有可用的配置版本，无法按真实数值裁决候选。"
		return resp, nil
	}
	cfg := s.LoadGameConfig()
	build, _, maxStage, err := s.LoadMeProfile(ctx, userID)
	if err != nil {
		return AdviceAction{}, err
	}
	st, err := s.recBattleStats(ctx, userID, recWindowDays)
	if err != nil {
		return AdviceAction{}, err
	}
	if st.Count == 0 {
		resp.EmptyReason.Code = "NO_HISTORY"
		resp.EmptyReason.Message = "近 7 天还没有真实战报，先打一场再给一键替换建议。"
		return resp, nil
	}
	if st.Count < recColdStartThreshold {
		resp.EmptyReason.Code = "NOT_ENOUGH_DATA"
		resp.EmptyReason.Message = "真实战报不到 3 场，样本不足，不编造一键替换建议。"
		return resp, nil
	}

	elementUses, err := s.adviceJSONCount(ctx, userID, "elements_used", element)
	if err != nil {
		return AdviceAction{}, err
	}
	reactionUses := int64(0)
	if reaction != "" {
		reactionUses, err = s.adviceJSONCount(ctx, userID, "reactions_used", reaction)
		if err != nil {
			return AdviceAction{}, err
		}
	}
	resp.Evidence = append(resp.Evidence,
		RecEvidence{Metric: "近 7 日战报数", Value: float64(st.Count), Unit: "count", Source: "battle_records", Window: "7d"},
		RecEvidence{Metric: "该元素近 7 日使用次数", Value: float64(elementUses), Unit: "count", Source: "battle_records.elements_used", Window: "7d"})
	if reaction != "" {
		resp.Evidence = append(resp.Evidence,
			RecEvidence{Metric: "期望反应近 7 日触发次数", Value: float64(reactionUses), Unit: "count", Source: "battle_records.reactions_used", Window: "7d"})
	}

	var params map[string]any
	var basis, kind, route string
	for _, candidate := range []struct{ context, kind, route string }{
		{"skills", "skill", "/pages/skills/skills"},
		{"equipment", "equipment", "/pages/equipment/equipment"},
	} {
		p, why := s.recImpactCTA(ctx, userID, build, cfg, maxStage, candidate.context, element)
		if why != "" {
			basis = why
		}
		if p != nil {
			params, kind, route = p, candidate.kind, candidate.route
			break
		}
	}
	if params == nil {
		resp.EmptyReason.Code = "NO_OWNED_CANDIDATE"
		detail := basis
		if detail == "" {
			detail = "当前构筑没有可比对的出战项"
		}
		resp.EmptyReason.Message = "已拥有技能/装备里没有同槽且数值更优的候选：" + detail + "。先补齐目录再回来。"
		return resp, nil
	}

	candidateID, replacedID := int(impactInt(params, "id")), int(impactInt(params, "replace"))
	impactResp, err := s.RecommendationImpact(ctx, userID, kind, candidateID, replacedID)
	if err != nil {
		return AdviceAction{}, err
	}
	if impactResp.EmptyReason != nil || impactResp.Impact == nil {
		code, message := "NOT_ENOUGH_DATA", "真实战报缺少可用的波次/命中样本，不给出预估数值。"
		if impactResp.EmptyReason != nil {
			code, message = impactResp.EmptyReason.Code, impactResp.EmptyReason.Message
		}
		resp.EmptyReason.Code, resp.EmptyReason.Message = code, message
		return resp, nil
	}

	candidateName, replacedName := adviceNames(cfg, kind, candidateID, replacedID)
	resp.Kind = kind
	resp.CandidateID, resp.ReplacedID = candidateID, replacedID
	resp.CandidateName, resp.ReplacedName = candidateName, replacedName
	resp.Params = map[string]any{"kind": kind, "id": candidateID, "replace": replacedID}
	resp.PreviewRoute = route
	resp.Impact = impactResp.Impact
	resp.EmptyReason = nil
	resp.BasedOn = []string{
		"battle_records.elements_used", "battle_records.reactions_used",
		"user_skills", "user_equipment",
		"/config(v=" + cfgVersion + ")",
	}
	if source != "" {
		resp.BasedOn = append(resp.BasedOn, "source="+source)
	}
	resp.Evidence = append(resp.Evidence, impactResp.Evidence...)
	return resp, nil
}
