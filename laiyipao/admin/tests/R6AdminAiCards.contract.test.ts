// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R6 admin 五视图 AI 元素契约（§1E.1/§1E.4）：4 个无 AI 视图补真实数据洞察卡，
 * LoginView 只加轻量诚实提示（不伪造洞察）。DOM 四判据：AI 汉字 + 真实结论 + 依据 + CTA。
 */
import { describe, expect, it } from "vitest"
import { readFileSync } from "node:fs"
import { fileURLToPath } from "node:url"
import { dirname, join } from "node:path"
import { mount } from "@vue/test-utils"
import AiInsightCard from "@/components/AiInsightCard.vue"
import { insightFromRecommendation, buildLocalRecommendation } from "@/utils/recommendation"

const dir = dirname(fileURLToPath(import.meta.url))
function view(name: string): string {
  return readFileSync(join(dir, "..", "src", "views", name + ".vue"), "utf8")
}

const AI_VIEWS = ["ContentView", "DefensesView", "EquipmentView", "WikiView"]
const DATA_ANCHOR: Record<string, string[]> = {
  ContentView: ["announcements", "codes", "logs"],
  DefensesView: ["rows", "shielded_until", "losses"],
  EquipmentView: ["equipment", "gems", "skins"],
  WikiView: ["cfg", "enemies", "skills"],
}

describe("R6 五视图 AI 元素源码契约（§1E.1）", () => {
  for (const name of AI_VIEWS) {
    it(name + " 集成真实数据洞察卡（AI 汉字 + 依据 + CTA + 本地兜底）", () => {
      const src = view(name)
      expect(src, name + " 缺少 AiInsightCard").toContain("AiInsightCard")
      expect(src, name + " 缺少 aiInsight computed").toContain("aiInsight")
      expect(src, name + " 缺少接口适配").toContain("insightFromRecommendation")
      expect(src, name + " 缺少本地兜底").toContain("buildLocalRecommendation")
      expect(src, name + " 缺少真实数据源字段").toContain("buildLocalRecommendation(")
      for (const field of DATA_ANCHOR[name]) {
        expect(src, name + " 缺少真实字段 " + field).toContain(field)
      }
      // AI 卡必须出现在 KPI 带之前（看板同款信息层级）；WikiView 用文档头锚点。
      const cardPos = src.indexOf("<AiInsightCard")
      const kpiPos = src.indexOf("class=\"kpi-strip\"")
      expect(cardPos, name + " 未渲染 AiInsightCard").toBeGreaterThan(0)
      if (kpiPos > 0) expect(cardPos, name + " AI 卡应在 KPI 前").toBeLessThan(kpiPos)
    })
  }

  it("LoginView 只加轻量 role=note AI 提示，不伪造洞察/不调接口", () => {
    const src = view("LoginView")
    expect(src).toContain("role=\"note\"")
    expect(src).toContain("AI")
    expect(src).not.toContain("AiInsightCard")
    expect(src).not.toContain("buildLocalRecommendation")
    expect(src).not.toMatch(/0\.[89]\d/)
  })

  it("AI 卡不消费裸元素 soft+panel-3 反模式（§1E.4）", () => {
    const card = readFileSync(join(dir, "..", "src", "components", "AiInsightCard.vue"), "utf8")
    expect(card).toMatch(/var\(--lyp-ai-accent\)/)
    expect(card).toMatch(/var\(--lyp-ai-halo\)/)
  })
})

describe("R6 洞察卡 DOM 四判据（§1E.4）", () => {
  const insight = insightFromRecommendation(
    {
      schema_version: "rec-v1", generated_at: "2026-11-22T02:00:00Z", context: "content",
      insights: [{
        id: "x", headline: "当前 3 条公告未发布，建议复核后发布",
        basis: ["未发布公告占 60%"],
        cta: { label: "复核公告", route: "/admin/content" },
        confidence: "rule_based", model: { name: "rules", version: "rec-v1" },
      }],
      evidence: [{ metric: "未发布公告", value: 3, unit: "条", source: "announcements", window: "7d" }],
      empty_reason: null, model: { name: "rules", version: "rec-v1", kind: "rule_based" },
    },
    buildLocalRecommendation("content", { announcements: [], codes: [], logs: [] }),
  )

  it("可见汉字 AI 徽章 + 真实数据结论 + 依据 + CTA 四判据齐全", () => {
    const w = mount(AiInsightCard, { props: { insight, loading: false } })
    expect(w.find(".ai-badge").text()).toContain("AI")
    expect(w.find(".ai-headline").text()).toContain("3 条公告未发布")
    expect(w.find(".ai-evidence").text()).toContain("未发布公告占 60%")
    // CTA 是真实 router-link；用 stub 检查 to 路径。
    const w2 = mount(AiInsightCard, {
      props: { insight, loading: false },
      global: { stubs: { RouterLink: { props: ["to"], template: "<a :data-to=\"to\"><slot /></a>" } } },
    })
    expect(w2.find(".ai-cta").attributes("data-to")).toBe("/admin/content")
  })
})