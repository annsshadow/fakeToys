// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R6 §1E 推荐适配层契约：消费后端 recommendations 接口 + 本地真实数据兜底。
 * 后端 t55 未上线时走本地规则引擎，卡上必须诚实标注，禁编造置信度。
 */
import { beforeEach, describe, expect, it, vi } from "vitest"

const apiGet = vi.hoisted(() => vi.fn())
vi.mock("@/api/client", async (importOriginal) => {
  const actual = await importOriginal<typeof import("@/api/client")>()
  return { ...actual, api: { ...actual.api, get: apiGet } }
})

import {
  fetchAdminRecommendation,
  insightFromRecommendation,
  buildLocalRecommendation,
  type AdminRecommendationResp,
} from "@/utils/recommendation"

function resp(overrides: Partial<AdminRecommendationResp> = {}): AdminRecommendationResp {
  return {
    schema_version: "rec-v1",
    generated_at: "2026-11-22T02:00:00Z",
    context: "content",
    insights: [{
      id: "ann-backlog",
      headline: "当前 3 条公告未发布，建议复核后发布",
      basis: ["未发布公告占 60%", "最近 7 日无新公告"],
      cta: { label: "复核公告", route: "/admin/content", params: { tab: "announcements" } },
      confidence: "rule_based",
      model: { name: "rules", version: "rec-v1" },
    }],
    evidence: [{ metric: "未发布公告", value: 3, unit: "条", source: "announcements", window: "7d" }],
    empty_reason: null,
    model: { name: "rules", version: "rec-v1", kind: "rule_based" },
    ...overrides,
  }
}

const fallback = {
  kind: "empty" as const,
  headline: "数据不足：暂无可用内容数据",
  evidence: "本地规则引擎",
  ctaLabel: "",
  ctaPath: "",
  asOf: "",
}

beforeEach(() => vi.clearAllMocks())

describe("R6 推荐接口适配层（§1E.2）", () => {
  it("按 scope 命中 /admin/recommendations/<scope> 且用 soft 避免未上线 401 踢登录", async () => {
    apiGet.mockResolvedValue(resp())
    await fetchAdminRecommendation("content")
    expect(apiGet).toHaveBeenCalledWith("/admin/recommendations/content", { soft: true })
  })

  it("真实 insights[0] 转成 AI 卡结论：headline/依据/CTA/数据截至全部来自接口", () => {
    const out = insightFromRecommendation(resp(), fallback)
    expect(out.headline).toContain("3 条公告未发布")
    expect(out.evidence).toContain("未发布公告 3条")
    expect(out.evidence).toContain("未发布公告占 60%")
    expect(out.ctaLabel).toBe("复核公告")
    expect(out.ctaPath).toBe("/admin/content")
    expect(out.asOf).toBe("2026-11-22")
    expect(out.evidence).not.toContain("本地规则引擎")
  })

  it("insights=[] 且 empty_reason 时回退并诚实空态，不编造结论", () => {
    const out = insightFromRecommendation(
      resp({ insights: [], empty_reason: { code: "NO_HISTORY", message: "无历史数据" } }),
      fallback,
    )
    expect(out.kind).toBe("empty")
    expect(out.headline).toContain("数据不足")
    expect(out.ctaPath).toBe("")
  })

  it("接口返回 null/异常时回退本地兜底，不抛错", () => {
    expect(insightFromRecommendation(null, fallback).kind).toBe("empty")
    expect(insightFromRecommendation(undefined, fallback).headline).toContain("数据不足")
  })

  // R6-BUG-1（t57 真浏览器复查）：合法空态 insights=[] + empty_reason 时不能冒充本地规则引擎兜底。
  it("R6-BUG-1: 合法空态 NO_HISTORY 展示接口诚实空态，evidence 不含『本地规则引擎』", () => {
    const out = insightFromRecommendation(
      resp({ insights: [], empty_reason: { code: "NO_HISTORY", message: "无历史数据" } }),
      fallback,
    )
    expect(out.kind).toBe("empty")
    expect(out.evidence).not.toContain("本地规则引擎")
    expect(out.evidence).toContain("NO_HISTORY")
    expect(out.evidence).toContain("无历史数据")
    expect(out.headline).toContain("数据不足")
  })

  it("R6-BUG-1: 空态文案与 H5 口径一致（AI 暂时没有足够数据 + 缺什么/下一步）", () => {
    const out = insightFromRecommendation(
      resp({ insights: [], empty_reason: { code: "NOT_ENOUGH_DATA", message: "" } }),
      fallback,
    )
    expect(out.headline).toContain("没有足够数据")
    expect(out.evidence).toContain("战斗样本还不足以判断搭配优劣")
    expect(out.evidence).toContain("下一步")
  })

  it("R6-BUG-1: 真正网络失败（null）才标注本地规则引擎，合法空态不误伤有数据响应", () => {
    const failed = insightFromRecommendation(null, fallback)
    expect(failed.evidence).toContain("本地规则引擎")
    const ok = insightFromRecommendation(resp(), fallback)
    expect(ok.headline).toContain("3 条公告未发布")
    expect(ok.evidence).not.toContain("本地规则引擎")
  })
})

describe("R6 本地真实数据兜底规则（§1E.2 诚实标注）", () => {
  it("content：从未发布公告真实数推导结论，空数据走诚实空态", () => {
    const out = buildLocalRecommendation("content", {
      announcements: [{ published: false }, { published: false }, { published: true }],
      codes: [], logs: [],
    })
    expect(out.headline).toContain("2 条公告未发布")
    expect(out.evidence).toContain("本地规则引擎")
    expect(out.ctaPath).toBe("/admin/content")
    const empty = buildLocalRecommendation("content", { announcements: [], codes: [], logs: [] })
    expect(empty.kind).toBe("empty")
    expect(empty.headline).toContain("数据不足")
  })

  it("defenses：从无护盾/高败场防线真实字段推导薄弱点", () => {
    const out = buildLocalRecommendation("defenses", {
      rows: [
        { name: "前哨", shielded_until: null, wins: 2, losses: 9 },
        { name: "主堡", shielded_until: "2999-01-01T00:00:00Z", wins: 8, losses: 1 },
      ],
    })
    expect(out.headline).toContain("前哨")
    expect(out.evidence).toContain("本地规则引擎")
    expect(out.ctaPath).toBe("/admin/defenses")
  })

  it("equipment：从元素/槽位覆盖真实统计推导配置建议", () => {
    const out = buildLocalRecommendation("equipment", {
      equipment: [{ element: "fire", slot: "weapon" }, { element: "fire", slot: "armor" }],
      gems: [], skins: [],
    })
    expect(out.headline).toContain("覆盖")
    expect(out.evidence).toContain("本地规则引擎")
    expect(out.ctaPath).toBe("/admin/equipment")
  })

  it("wiki：从 config 真实条目统计推导知识缺口，CTA 指向文档站内锚点", () => {
    const out = buildLocalRecommendation("wiki", {
      levels: [{ id: 1 }, { id: 2 }],
      enemies: [{ code: "e1", name: "史莱姆", descr: "" }, { code: "e2", name: "哥布林", descr: "说明" }],
      skills: [{ id: 1, name: "火球", descr: "说明" }],
    })
    expect(out.headline).toContain("史莱姆")
    expect(out.evidence).toContain("本地规则引擎")
    expect(out.ctaPath).toContain("/wiki")
  })

  it("所有本地结论禁编造置信度小数（0.9x/0.8x）", () => {
    const scopes = ["content", "defenses", "equipment", "wiki"] as const
    const payloads: Record<string, unknown> = {
      content: { announcements: [{ published: false }], codes: [], logs: [] },
      defenses: { rows: [{ name: "前哨", shielded_until: null, wins: 0, losses: 3 }] },
      equipment: { equipment: [{ element: "fire", slot: "weapon" }], gems: [], skins: [] },
      wiki: { enemies: [{ code: "e1", name: "史莱姆", descr: "" }] },
    }
    for (const s of scopes) {
      const out = buildLocalRecommendation(s, payloads[s])
      expect(out.headline + out.evidence).not.toMatch(/0\.[89]\d/)
    }
  })
})