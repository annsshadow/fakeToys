// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
// 真实导入实施模块：模块不存在时整个 suite RED（禁止内嵌参考实现）
import { MECHANISM_EMPTY, kindText, valueText, prereqText, buildMasteryMechanicsAi, MASTERY_AI_BASIS, MASTERY_AI_HONESTY, MASTERY_AI_CTA } from "../src/masteryMechanicsText"

const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const spec = read("docs/laiyipao/specs/round37-wiki-mastery-nodes/SPEC.md")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const power = read("laiyipao/server/internal/domain/power.go")
const shape = read("laiyipao/server/internal/domain/mastery_json_shape_test.go")
const service = read("laiyipao/server/internal/service/service.go")
const generated = read("laiyipao/admin/src/api/types.gen.ts")
const miniTypes = read("laiyipao/miniapp/src/game/types.ts")
const engine = read("laiyipao/miniapp/src/game/engine.ts")
const enemyHelper = read("laiyipao/admin/src/enemyMechanicsText.ts")
const tokens = read("laiyipao/admin/src/styles/tokens.css")
const typeStart = wiki.indexOf("mastery_families: Array<{")
const masteryType = typeStart >= 0 ? wiki.slice(typeStart, wiki.indexOf("}\n", typeStart) + 2) : ""
const masteryStart = wiki.indexOf("activeTab === 'mastery'")
const masteryEnd = wiki.indexOf("<!-- 关卡 -->", masteryStart)
const masterySection = masteryStart >= 0 ? wiki.slice(masteryStart, masteryEnd > masteryStart ? masteryEnd : wiki.length) : ""

describe("R37 wiki mastery nodes mechanics contract", () => {
  it("MECHANISM_EMPTY 复用 R33 同源同值，非法值一律空态", () => {
    expect(MECHANISM_EMPTY).toBe("—")
    expect(spec).toContain("MECHANISM_EMPTY 从 @/enemyMechanicsText 同源复用")
    expect(spec).toContain("禁止在本模块重新声明")
    expect(enemyHelper).toContain("export const MECHANISM_EMPTY = '—'")
  })

  it("kindText 权威八种效果名，未知合法原样、非法空态", () => {
    const mapping: Record<string, string> = {
      element_cap: "元素层数上限", reaction_mult: "反应倍率", heat_cap: "热量上限", extra_slot: "额外插槽",
      skill_damage: "技能伤害", crit: "暴击", armor: "护甲", mechanic: "机制改造",
    }
    for (const [kind, label] of Object.entries(mapping)) expect(kindText(kind)).toBe(label)
    expect(kindText("custom_kind")).toBe("custom_kind")
    for (const v of ["", 0, null, undefined, {}, []]) expect(kindText(v)).toBe(MECHANISM_EMPTY)
  })

  it("valueText 按 power.go 语义输出诚实口径，heat_cap 不二次换算", () => {
    expect(valueText("heat_cap", 10)).toBe("+10‰")
    expect(valueText("heat_cap", 30)).toBe("+30‰")
    expect(valueText("element_cap", 2)).toBe("+2层")
    expect(valueText("extra_slot", 7)).toBe("+1 槽")
    expect(valueText("reaction_mult", 200)).toBe("+200‰")
    expect(valueText("skill_damage", 80)).toBe("+80‰")
    expect(valueText("crit", 80)).toBe("+80‰")
    expect(valueText("armor", 100)).toBe("+100‰")
    expect(valueText("mechanic", 200)).toBe("+200‰")
    expect(valueText("custom_kind", 9)).toContain("单位以游戏内结算为准")
    for (const v of [0, -1, 1.5, Number.NaN, Number.POSITIVE_INFINITY, "30", null, undefined]) expect(valueText("heat_cap", v)).toBe(MECHANISM_EMPTY)
    expect(valueText("heat_cap", 300)).not.toContain("%")
  })

  it("prereqText 无前置/有前置/非法边界逐字", () => {
    expect(prereqText("", 0)).toBe("无前置")
    expect(prereqText(null, 0)).toBe("无前置")
    expect(prereqText("flame", 2)).toBe("前置：flame 第2层")
    for (const [f, l] of [["flame", 0], ["", 2], ["flame", 1.5], ["flame", -1], [null, 2], ["flame", Number.NaN]]) {
      expect(prereqText(f, l)).toBe(MECHANISM_EMPTY)
    }
  })

  it("buildMasteryMechanicsAi 确定性聚合且空态诚实", () => {
    const families = [
      { family: "flame", name: "焰", nodes: [
        { id: 1, family: "flame", layer: 1, slot: 0, name: "A", kind: "extra_slot", value: 1, prereq_family: "", prereq_layer: 0 },
        { id: 2, family: "flame", layer: 2, slot: 1, name: "B", kind: "heat_cap", value: 20, prereq_family: "flame", prereq_layer: 1 },
        { id: 3, family: "flame", layer: 3, slot: 2, name: "C", kind: "reaction_mult", value: 200, prereq_family: "flame", prereq_layer: 2 },
      ] },
      { family: "frost", name: "冰", nodes: [
        { id: 4, family: "frost", layer: 1, slot: 0, name: "D", kind: "armor", value: 100, prereq_family: "", prereq_layer: 0 },
        { id: 5, family: "frost", layer: 2, slot: 1, name: "", kind: "crit", value: 80, prereq_family: "", prereq_layer: 0 },
      ] },
      { family: "", name: "脏系", nodes: [{ id: 6, family: "", layer: 1, slot: 0, name: "E", kind: "extra_slot", value: 1, prereq_family: "", prereq_layer: 0 }] },
    ]
    expect(buildMasteryMechanicsAi(families)).toBe("共 2 系：4 个节点，含前置 2 个，额外插槽 1 个")
    expect(buildMasteryMechanicsAi(families)).toBe(buildMasteryMechanicsAi(families))
    expect(buildMasteryMechanicsAi([])).toBe("共 0 系：0 个节点，含前置 0 个，额外插槽 0 个")
    expect(spec).toContain("共 {families} 系：{nodes} 个节点，含前置 {gated} 个，额外插槽 {slots} 个")
  })

  it("逐字常量：依据、红线、CTA 固定文案", () => {
    expect(MASTERY_AI_BASIS).toBe("依据：公开 /api/v1/config 的 mastery_families（与服务端 MasteryFamily / MasteryNode 同源）；效果按 power.go 口径展示，实际以引擎结算为准")
    expect(MASTERY_AI_HONESTY).toBe("辅助展示配置基值，非权威战斗数据；实际效果以引擎结算为准，权威回放以 replay_hash 为准")
    expect(MASTERY_AI_CTA).toBe("查看专精树 →")
  })

  it("权威 MasteryNode / MasteryFamily / 公开 config / 双端消费有据", () => {
    expect(power).toContain("type MasteryNode struct")
    for (const f of ['json:"id"', 'json:"layer"', 'json:"slot"', 'json:"kind"', 'json:"value"', 'json:"prereq_family"', 'json:"prereq_layer"']) expect(power).toContain(f)
    for (const s of ["effect.HeatCapBonus += v * 10", "effect.ElementCapBonus += v", "effect.ExtraSlots += 1", "effect.ReactionMultBonus += v", "const MasteryReactionMultPerNode = 200"]) expect(power).toContain(s)
    for (const k of ['"id"', '"layer"', '"slot"', '"kind"', '"value"', '"prereq_family"', '"prereq_layer"']) expect(shape).toContain(k)
    expect(service).toContain('json:"mastery_families"')
    expect(service).toContain("domain.AllMasteryFamilies()")
    expect(wiki).toContain("api.get<ConfigResp>('/config', { auth: false })")
    for (const s of ["export interface MasteryNode", "id: number", "prereq_family: string", "prereq_layer: number"]) expect(generated).toContain(s)
    for (const s of ["export interface MasteryNode", "prereq_family: string", "prereq_layer: number"]) expect(miniTypes).toContain(s)
    expect(engine).toContain("switch (e.kind)")
  })

  it("ConfigResp.mastery_families 类型补齐九字段，防漂移回归", () => {
    expect(typeStart).toBeGreaterThanOrEqual(0)
    for (const f of ["family:", "name:", "nodes:", "id:", "layer:", "slot:", "kind:", "value:", "prereq_family:", "prereq_layer:"]) expect(masteryType).toContain(f)
  })

  it("专精节点表七列与真实调用齐全", () => {
    expect(masterySection).toContain("r37-mastery-mechanics")
    expect(masterySection).toContain("r37-mastery-table-scroll")
    const css = wiki.slice(wiki.indexOf(".r37-mastery-table-scroll"))
    expect(css).toContain("overflow-x: auto")
    expect(css).toContain("max-width: 100%")
    for (const label of ["系", "层", "槽", "节点", "效果", "数值", "前置"]) expect(masterySection).toContain('label="' + label + '"')
    for (const call of ["kindText(n.kind)", "valueText(n.kind, n.value)", "prereqText(n.prereq_family, n.prereq_layer)", "buildMasteryMechanicsAi(cfg?.mastery_families ?? [])"]) expect(masterySection).toContain(call)
    expect(wiki).toContain("@/masteryMechanicsText")
  })

  it(".r37-ai-note 四要素同屏且 CTA 走 hash-router 安全链接", () => {
    const ai = masterySection.slice(masterySection.indexOf("r37-ai-note"))
    expect(ai).toContain("AI 机制概览")
    expect(ai).toContain(MASTERY_AI_BASIS)
    expect(ai).toContain(MASTERY_AI_HONESTY)
    expect(ai).toContain(MASTERY_AI_CTA)
    expect(ai).toContain("urlFor('mastery')")
    expect(ai).toContain("goTo({ tab: 'mastery' })")
    expect(ai).not.toContain('href="#mastery"')
    for (const ph of ["{families}", "{nodes}", "{gated}", "{slots}"]) expect(ai).not.toContain(ph)
  })

  it("SPEC 钉死零改动面、诚实边界与令牌三锚点", () => {
    for (const s of ["禁止内嵌参考实现", "不改 engine/后端/replay/hash/score/settle/loot/tokens", "配置基值", "replay_hash", "heat_cap", "千分比", "单位以游戏内结算为准", "r37-mastery-mechanics", "r37-mastery-table-scroll", "r37-ai-note", "overflow-x: auto", "max-width: 100%", "禁止新增 --r37-* 令牌", "RED"]) expect(spec).toContain(s)
    for (const anchor of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"]) expect(tokens).toContain(anchor)
    expect(tokens).not.toContain("--r37-")
    expect(wiki).not.toContain("--r37-")
  })
})
