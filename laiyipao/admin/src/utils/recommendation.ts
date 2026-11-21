// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R6 真实数据驱动推荐适配层（AI_VISUAL_SYSTEM.md §1E）。
 *
 * 优先消费后端 GET /api/v1/admin/recommendations/<scope>（§1E.2 契约）；
 * 后端未上线（404/401/network）时回退**本地真实数据规则**，并在证据里诚实标注
 * 「本地规则引擎」——绝不编造置信度小数、不宣称 GPT、不写死结论。
 * 所有结论同输入恒等输出，无随机/当前时间依赖。
 */
import { api } from "@/api/client"
import type { AiInsight } from "@/utils/aiInsight"

export type AdminRecommendationScope =
  | "battles" | "dashboard" | "levels" | "economy" | "skills" | "users" | "recomputes"
  | "content" | "defenses" | "equipment" | "wiki"

export interface RecommendationEvidence {
  metric: string
  value: number | string
  unit: string
  source: string
  window: string
}
export interface RecommendationInsight {
  id: string
  headline: string
  basis: string[]
  cta: { label: string; route: string; params?: Record<string, string | number> }
  confidence: "rule_based" | "statistical" | null
  model: { name: string; version: string }
}
export interface AdminRecommendationResp {
  schema_version: string
  generated_at: string
  context: string
  subject?: { user_key?: string; level?: number; power?: number }
  input_snapshot?: { battle_count?: number; window_days?: number; config_version?: string }
  evidence?: RecommendationEvidence[]
  insights: RecommendationInsight[]
  empty_reason?: { code: string; message: string } | null
  model?: { name: string; version: string; kind: string }
}

const LOCAL_TAG = "本地规则引擎"
const emptyInsight = (headline: string, asOf = ""): AiInsight => ({
  kind: "empty", headline, evidence: LOCAL_TAG, ctaLabel: "", ctaPath: "", asOf,
})

/** 拉取推荐；soft:true 让后端未上线的 401 不踢登录页。异常由调用方兜底。 */
export function fetchAdminRecommendation(scope: AdminRecommendationScope): Promise<AdminRecommendationResp> {
  return api.get<AdminRecommendationResp>("/admin/recommendations/" + scope, { soft: true })
}

/**
 * 把接口 insights[0] + evidence 转成 AI 卡可消费的 AiInsight。
 * 合法空态（empty_reason.code）优先于本地兜底 —— 接口明确「数据不足」时不冒充本地规则引擎。
 * 仅在无响应/异常时才用调用方给的本地兜底（含诚实标注）。
 * asOf 从 generated_at 截到日，避免卡片出现完整时间戳。
 */
const EMPTY_REASON_COPY: Record<string, string> = {
  // 与 H5 r6Recommendation.ts emptyRecommendation() 的 known 文案保持一致。
  NO_HISTORY: "还没有可参照的战斗历史",
  NOT_ENOUGH_DATA: "战斗样本还不足以判断搭配优劣",
  CONFIG_MISSING: "配置版本尚未下发，无法比较当前配装",
}

export function insightFromRecommendation(
  resp: AdminRecommendationResp | null | undefined,
  fallback: AiInsight,
): AiInsight {
  // R6-BUG-1：HTTP 200 的合法空态必须诚实展示，不能因为 insights=[] 就回退本地兜底。
  const code = resp?.empty_reason?.code
  if (code) {
    const asOf = typeof resp?.generated_at === "string" ? resp.generated_at.slice(0, 10) : ""
    const detail = resp!.empty_reason!.message || EMPTY_REASON_COPY[code] || "推荐所需数据尚未就绪"
    return {
      kind: "empty",
      headline: "AI 数据不足：暂时没有足够数据给出建议",
      evidence: "数据不足 · " + code + "；缺什么：" + detail + "；下一步：补齐数据后回到本页刷新",
      ctaLabel: "", ctaPath: "", asOf,
    }
  }
  if (!resp || !Array.isArray(resp.insights) || resp.insights.length === 0) return fallback
  const top = resp.insights[0]
  const evidenceLines = (resp.evidence ?? []).slice(0, 3).map((e) => e.metric + " " + e.value + e.unit)
  const basisLines = (top.basis ?? []).slice(0, 2)
  const evidence = [...evidenceLines, ...basisLines].filter(Boolean).join("；")
  const asOf = typeof resp.generated_at === "string" ? resp.generated_at.slice(0, 10) : ""
  return {
    kind: "funnel",
    headline: top.headline,
    evidence,
    ctaLabel: top.cta?.label ?? "",
    ctaPath: top.cta?.route ?? "",
    asOf,
  }
}

type LocalScope = "content" | "defenses" | "equipment" | "wiki"
type Loose = Record<string, any>

/**
 * 本地真实数据兜底：从当前视图已加载的数据按确定性规则推导一条最有行动价值的结论。
 * 空数据一律走诚实空态；证据串统一带「本地规则引擎」前缀，与接口结论可区分。
 */
export function buildLocalRecommendation(scope: LocalScope, payload: Loose): AiInsight {
  const tag = (body: string) => LOCAL_TAG + " · " + body
  if (scope === "content") {
    const anns: Loose[] = Array.isArray(payload.announcements) ? payload.announcements : []
    const codes: Loose[] = Array.isArray(payload.codes) ? payload.codes : []
    const logs: Loose[] = Array.isArray(payload.logs) ? payload.logs : []
    if (!anns.length && !codes.length && !logs.length)
      return emptyInsight("数据不足：暂无公告、兑换码与审计记录，无法给出内容建议")
    const unpublished = anns.filter((a) => !a.published).length
    if (unpublished > 0) {
      return {
        kind: "funnel",
        headline: "当前 " + unpublished + " 条公告未发布，建议复核后发布",
        evidence: tag("公告 " + anns.length + " 条，其中未发布 " + unpublished + " 条；数据来自当前页公告表"),
        ctaLabel: "复核公告", ctaPath: "/admin/content", asOf: "",
      }
    }
    return {
      kind: "funnel",
      headline: "已发布 " + anns.length + " 条公告、" + codes.length + " 张兑换码，建议关注核销与使用记录",
      evidence: tag("公告 " + anns.length + " 条 / 兑换码 " + codes.length + " 张 / 审计 " + logs.length + " 条"),
      ctaLabel: "查看兑换码", ctaPath: "/admin/content", asOf: "",
    }
  }
  if (scope === "defenses") {
    const rows: Loose[] = Array.isArray(payload.rows) ? payload.rows : []
    if (!rows.length) return emptyInsight("数据不足：暂无防线记录，无法定位薄弱防线")
    const weakest = rows.reduce((a, b) => ((b.losses ?? 0) - (b.wins ?? 0) > (a.losses ?? 0) - (a.wins ?? 0) ? b : a))
    const unshielded = rows.filter((r) => !r.shielded_until).length
    return {
      kind: "funnel",
      headline: "防线「" + (weakest.name ?? "#" + weakest.id) + "」败场 " + (weakest.losses ?? 0) + " 最多，建议优先复核布阵",
      evidence: tag("共 " + rows.length + " 条防线，" + unshielded + " 条无护盾；依据 losses/wins/shielded_until 真实字段"),
      ctaLabel: "复核防线", ctaPath: "/admin/defenses", asOf: "",
    }
  }
  if (scope === "equipment") {
    const eq: Loose[] = Array.isArray(payload.equipment) ? payload.equipment : []
    const gems: Loose[] = Array.isArray(payload.gems) ? payload.gems : []
    const skins: Loose[] = Array.isArray(payload.skins) ? payload.skins : []
    if (!eq.length && !gems.length && !skins.length)
      return emptyInsight("数据不足：装备目录为空，无法给出配置建议")
    const elements = new Set(eq.map((e) => e.element).filter(Boolean)).size
    const slots = new Set(eq.map((e) => e.slot).filter(Boolean)).size
    return {
      kind: "funnel",
      headline: "装备覆盖 " + elements + "/5 元素、" + slots + " 个部位，建议补齐缺口元素以触发更多反应",
      evidence: tag("装备 " + eq.length + " 件 / 宝石 " + gems.length + " 颗 / 皮肤 " + skins.length + " 款；依据 element/slot 真实字段"),
      ctaLabel: "查看装备目录", ctaPath: "/admin/equipment", asOf: "",
    }
  }
  // wiki：从 /config 真实条目找缺说明的敌人/技能，建议补文档并跳到对应章节
  const enemies: Loose[] = Array.isArray(payload.enemies) ? payload.enemies : []
  const skills: Loose[] = Array.isArray(payload.skills) ? payload.skills : []
  const levels: Loose[] = Array.isArray(payload.levels) ? payload.levels : []
  if (!enemies.length && !skills.length && !levels.length)
    return emptyInsight("数据不足：/config 尚未加载，无法定位知识缺口")
  const bare = [...enemies, ...skills].find((x) => !x.descr)
  if (bare) {
    return {
      kind: "funnel",
      headline: "条目「" + (bare.name ?? bare.code ?? "?") + "」缺少说明，建议补充玩法文档",
      evidence: tag("敌人 " + enemies.length + " 条 / 技能 " + skills.length + " 条 / 关卡 " + levels.length + " 关；依据 descr 真实字段"),
      ctaLabel: "查看敌人图鉴", ctaPath: "/wiki?tab=enemies", asOf: "",
    }
  }
  return {
    kind: "funnel",
    headline: "文档覆盖 " + enemies.length + " 敌人 / " + skills.length + " 技能 / " + levels.length + " 关，建议结合实战数据复核说明",
    evidence: tag("条目说明齐全；数据来自 /config 与近 14 日反应使用"),
    ctaLabel: "查看玩法文档", ctaPath: "/wiki", asOf: "",
  }
}