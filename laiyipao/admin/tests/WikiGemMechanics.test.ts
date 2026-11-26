// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
// 真实导入实施模块：模块不存在时整个 suite RED（禁止内嵌参考实现）
import { MECHANISM_EMPTY, attrText, qualityText, buildGemMechanicsAi, GEM_AFFIX_NOTE, GEM_AI_BASIS, GEM_AI_HONESTY, GEM_AI_CTA } from "../src/gemMechanicsText"

const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const spec = read("docs/laiyipao/specs/round35-wiki-gem-mechanics/SPEC.md")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const content = read("laiyipao/server/internal/domain/content.go")
const gemsGo = read("laiyipao/server/internal/domain/gems.go")
const serviceGems = read("laiyipao/server/internal/service/gems.go")
const service = read("laiyipao/server/internal/service/service.go")
const tokens = read("laiyipao/admin/src/styles/tokens.css")
const gemSectionStart = wiki.indexOf("activeTab === 'gems'")
const gemSection = gemSectionStart >= 0 ? wiki.slice(gemSectionStart, wiki.indexOf("</section>", gemSectionStart)) : ""

describe("R35 wiki gem mechanics text contract", () => {
  it("attrText 严格按权威 attr 映射，未知原样非法归空态", () => {
    const map: Record<string, string> = {
      attack: "攻击", element_coef: "元素系数", crit: "暴击率", reaction_mult: "反应倍率",
      element_cap: "元素层数上限", heat_cap: "热量上限", armor: "护甲", skill_damage: "技能伤害",
    }
    for (const [k, v] of Object.entries(map)) expect(attrText(k)).toBe(v)
    expect(attrText("some_new_attr")).toBe("some_new_attr")
    for (const v of ["", null, undefined, 0, 1, NaN, Infinity, {}, []]) expect(attrText(v)).toBe(MECHANISM_EMPTY)
    expect(MECHANISM_EMPTY).toBe("—")
  })

  it("qualityText 千分号系数不冒充百分比，字段非法归空态", () => {
    expect(qualityText({ name: "gray", affix_count: 1, mult: 1000 })).toBe("gray · 1 条 · 1000‰")
    expect(qualityText({ name: "gold", affix_count: 3, mult: 2000 })).toBe("gold · 3 条 · 2000‰")
    expect(qualityText({ name: "gold", affix_count: 3, mult: 1450 })).toContain("1450‰")
    expect(qualityText({ name: "custom", affix_count: 2, mult: 1000 })).toBe("custom · 2 条 · 1000‰")
    for (const v of [null, undefined, [], "gray", {}, { name: "", affix_count: 1, mult: 1000 }, { name: "gray", affix_count: 0, mult: 1000 }, { name: "gray", affix_count: 1.5, mult: 1000 }, { name: "gray", affix_count: 1, mult: 0 }, { name: "gray", affix_count: 1, mult: NaN }]) expect(qualityText(v)).toBe(MECHANISM_EMPTY)
  })

  it("buildGemMechanicsAi 确定性聚合与诚实空态", () => {
    expect(buildGemMechanicsAi([], [])).toBe("共 0 种宝石，0 档品质，词条 — 条")
    const gems = Array.from({ length: 8 }, (_, i) => ({ id: i + 1, attr: "attack" }))
    const qualities = [
      { name: "gray", affix_count: 1, mult: 1000 }, { name: "blue", affix_count: 1, mult: 1200 },
      { name: "purple", affix_count: 2, mult: 1450 }, { name: "red", affix_count: 2, mult: 1700 },
      { name: "gold", affix_count: 3, mult: 2000 },
    ]
    const expected = "共 8 种宝石，5 档品质，词条 1–3 条"
    expect(buildGemMechanicsAi(gems, qualities)).toBe(expected)
    expect(buildGemMechanicsAi(gems, qualities)).toBe(expected)
    expect(buildGemMechanicsAi([{}], [{ name: "gray", affix_count: 2, mult: 1000 }])).toBe("共 1 种宝石，1 档品质，词条 2 条")
  })

  it("逐字常量：词条随机说明、依据、红线、CTA", () => {
    expect(GEM_AFFIX_NOTE).toBe("词条随机，具体数值以游戏内为准")
    expect(GEM_AI_BASIS).toBe("依据：公开 /api/v1/config 的 gems 与 gem_qualities（与服务端 SeedGem / GemQualities 同源）；词条随机，具体数值以游戏内为准")
    expect(GEM_AI_HONESTY).toBe("辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准")
    expect(GEM_AI_CTA).toBe("查看宝石图鉴 →")
  })

  it("权威宝石字段与双端消费有据，词条区间不下发", () => {
    expect(content).toContain("type SeedGem struct")
    for (const f of ['json:"id"', 'json:"code"', 'json:"name"', 'json:"attr"', 'json:"descr"']) expect(content).toContain(f)
    for (const a of ['attack', 'element_coef', 'crit', 'reaction_mult', 'element_cap', 'heat_cap', 'armor', 'skill_damage']) expect(content).toContain('Attr: "' + a + '"')
    expect(content).toContain("var GemQualities")
    for (const q of ['{"gray", 1, 1000}', '{"blue", 1, 1200}', '{"purple", 2, 1450}', '{"red", 2, 1700}', '{"gold", 3, 2000}']) expect(content).toContain(q)
    expect(service).toContain('json:"gems"')
    expect(service).toContain('json:"gem_qualities"')
    expect(service).toContain("Gems:       domain.SeedGems")
    expect(service).toContain("QualityInfo{q.Name, q.Count, q.Mult}")
    expect(serviceGems).toContain("Value: r.Value * mult / 1000")
    expect(gemsGo).toContain("type SeedGemAffix struct")
    expect(spec).toContain("MinValue/MaxValue")
    expect(spec).toContain("禁止在 Wiki 任何位置展示具体数值范围")
  })

  it("WikiView 宝石 section 选择器、目录、导航与真实调用", () => {
    for (const s of ["'gems'", "宝石图鉴", "'gem'", "kind: 'gem'", "section: 'gems'", "rowFocusClass('gem')"]) expect(wiki).toContain(s)
    expect(wiki).toContain("KIND_LABEL: Record<HitKind, string>")
    expect(wiki).toContain("gem: '宝石'")
    expect(wiki).toContain("r35-gem-mechanics")
    expect(wiki).toContain("r35-gem-table-scroll")
    expect(wiki).toContain("r35-ai-note")
    expect(wiki).toContain("gems: Array<{")
    expect(wiki).toContain("gem_qualities: Array<{")
    expect(wiki).toContain("attrText, qualityText, buildGemMechanicsAi, GEM_AFFIX_NOTE, GEM_AI_BASIS, GEM_AI_HONESTY, GEM_AI_CTA")
    for (const call of ["attrText(row.attr)", "buildGemMechanicsAi(cfg?.gems ?? [], cfg?.gem_qualities ?? [])"]) expect(gemSection).toContain(call)
    for (const label of ["ID", "名称", "属性", "说明", "品质", "词条数", "系数"]) expect(gemSection).toContain(label)
    expect(wiki).toContain(".r35-gem-table-scroll")
    const css = wiki.slice(wiki.indexOf(".r35-gem-table-scroll"))
    expect(css).toContain("overflow-x: auto")
    expect(css).toContain("max-width: 100%")
  })

  it("AI 四要素与 hash 安全 CTA，词条随机说明同屏", () => {
    expect(gemSection).toContain("AI 机制概览")
    expect(gemSection).toContain("词条随机，具体数值以游戏内为准")
    expect(gemSection).toContain("依据：公开 /api/v1/config 的 gems 与 gem_qualities（与服务端 SeedGem / GemQualities 同源）；词条随机，具体数值以游戏内为准")
    expect(gemSection).toContain("辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准")
    expect(gemSection).toContain("urlFor('gems')")
    expect(gemSection).toContain("goTo({ tab: 'gems' })")
    expect(gemSection).not.toContain('href="#gems"')
  })

  it("R35 修复 P1：品质名可被搜索索引与高亮（gold）", () => {
    const siStart = wiki.indexOf("const searchIndex")
    const si = siStart >= 0 ? wiki.slice(siStart, wiki.indexOf("const SEARCH_LIMIT", siStart)) : ""
    // 索引遍历 gem_qualities 并产出 kind:quality 条目
    expect(si).toContain("c.gem_qualities")
    expect(si).toContain("kind: 'quality'")
    expect(si).toContain("qualityText(q)")
    expect(si).toContain("section: 'gems'")
    // 类型 / 注册表 / 展示标签
    for (const hit of ["'level'", "'enemy'", "'skill'", "'composite'", "'equip'", "'reaction'", "'mastery'", "'gem'", "'quality'"]) expect(wiki).toContain(hit)
    expect(wiki).toContain("quality: '宝石品质'")
    // rowFocusClass 仅对 quality 用 row.name 比较
    const rfStart = wiki.indexOf("function rowFocusClass")
    const rfBlock = rfStart >= 0 ? wiki.slice(rfStart, rfStart + 700) : ""
    expect(rfBlock).toContain("kind === 'quality'")
    expect(rfBlock).toContain("scope.row.name")
    // 品质表绑定高亮工厂
    expect(gemSection).toContain("rowFocusClass('quality')")
  })

  it("SPEC 锚点与令牌三端不回退", () => {
    for (const a of ["MECHANISM_EMPTY", "attrText", "qualityText", "buildGemMechanicsAi", "GEM_AI_CTA", "禁止内嵌参考实现", "RED", "词条随机，具体数值以游戏内为准"]) expect(spec).toContain(a)
    for (const t of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"]) expect(tokens).toContain(t)
    expect(tokens).not.toContain("--r35-")
    expect(wiki).not.toContain("--r35-")
  })
})