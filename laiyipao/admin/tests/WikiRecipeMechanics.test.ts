// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
// 真实导入实施模块：模块不存在时整个 suite RED（禁止内嵌参考实现）
import { MECHANISM_EMPTY, recipeMaterialsText, tierText, buildRecipeMechanicsAi, RECIPE_AI_BASIS, RECIPE_AI_HONESTY, RECIPE_AI_CTA } from "../src/recipeMechanicsText"

const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const spec = read("docs/laiyipao/specs/round38-wiki-recipes/SPEC.md")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const service = read("laiyipao/server/internal/service/service.go")
const content = read("laiyipao/server/internal/domain/content.go")
const generated = read("laiyipao/admin/src/api/types.gen.ts")
const bag = read("laiyipao/miniapp/src/pages/bag/bag.vue")
const enemyHelper = read("laiyipao/admin/src/enemyMechanicsText.ts")
const tokens = read("laiyipao/admin/src/styles/tokens.css")
const recipeStart = wiki.indexOf("r38-recipe-mechanics")
const recipeSection = recipeStart >= 0 ? wiki.slice(recipeStart, wiki.indexOf("</section>", recipeStart)) : ""
const searchStart = wiki.indexOf("const searchIndex = computed")
const searchIndex = searchStart >= 0 ? wiki.slice(searchStart, wiki.indexOf("const doSearch", searchStart) > searchStart ? wiki.indexOf("const doSearch", searchStart) : searchStart + 6000) : ""
const names = new Map<number, string>([[1, "制式炮管"], [2, "燃油弹"], [3, "制式重锤"], [11, "爆裂弹"], [12, "棱镜散射"], [15, "攻城锤"], [31, "灼热弹幕"], [33, "焦土重锤"], [48, "光棱攻城"]])

describe("R38 wiki recipe mechanics contract", () => {
  it("MECHANISM_EMPTY 复用 R33 同源同值", () => {
    expect(MECHANISM_EMPTY).toBe("—")
    expect(spec).toContain("MECHANISM_EMPTY 从 @/enemyMechanicsText 同源复用")
    expect(spec).toContain("禁止在本模块重新声明")
    expect(enemyHelper).toContain("export const MECHANISM_EMPTY = '—'")
  })

  it("权威 recipes 四字段、公开 config 与玩家端熔炼消费有据", () => {
    expect(service).toContain('json:"recipes"')
    expect(service).toContain("Recipes:    domain.SeedRecipes")
    expect(wiki).toContain("api.get<ConfigResp>('/config', { auth: false })")
    expect(content).toContain("type SeedRecipe struct")
    for (const f of ['json:"output"', 'json:"a"', 'json:"b"', 'json:"out_tier"']) expect(content).toContain(f)
    expect(content).toContain("{Output: 31, A: 1, B: 2, OutTier: 2}")
    expect(content).toContain("{Output: 48, A: 12, B: 15, OutTier: 3}")
    expect(generated).toContain("export interface SeedRecipe")
    for (const f of ["output: number", "a: number", "b: number", "out_tier: number"]) expect(generated).toContain(f)
    expect(bag).toContain("store.config?.recipes ?? []")
    expect(bag).toContain("function skillNameOf(id: number): string")
  })

  it("recipeMaterialsText 映射 id 为技能名，未知诚实标注", () => {
    expect(recipeMaterialsText({ output: 31, a: 1, b: 2, out_tier: 2 }, names)).toBe("制式炮管 + 燃油弹")
    expect(recipeMaterialsText({ output: 33, a: 3, b: 11, out_tier: 3 }, (id: number) => names.get(id))).toBe("制式重锤 + 爆裂弹")
    expect(recipeMaterialsText({ a: 1, b: 999 }, names)).toBe("制式炮管 + （未知技能#999）")
    expect(recipeMaterialsText({ a: 999, b: 998 }, {})).toBe("（未知技能#999） + （未知技能#998）")
    expect(recipeMaterialsText({ a: 1, b: 2 }, () => { throw new Error("bad map") })).not.toContain("bad map")
    for (const bad of [null, undefined, "r", [], { a: 0, b: 2 }, { a: 1.5, b: 2 }, { a: Number.NaN, b: 2 }, { a: 1, b: -2 }, { a: 1, b: Infinity }, { a: 1, b: "2" }]) {
      expect(recipeMaterialsText(bad, names)).toBe(MECHANISM_EMPTY)
    }
  })

  it("tierText 仅正整数有效，unknown 全边界空态", () => {
    expect(tierText(2)).toBe("2 阶")
    expect(tierText(3)).toBe("3 阶")
    for (const bad of [0, -1, 1.5, Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, null, undefined, "2", {}, []]) expect(tierText(bad)).toBe(MECHANISM_EMPTY)
  })

  it("buildRecipeMechanicsAi 确定性聚合，仅合法整数计数", () => {
    const recipes = [
      { output: 31, a: 1, b: 2, out_tier: 2 },
      { output: 33, a: 3, b: 11, out_tier: 3 },
      { output: 48, a: 12, b: 15, out_tier: 3 },
      { output: 49, a: 1, b: 2, out_tier: 4 },
      { output: 50, a: 1, b: 2, out_tier: "3" },
      { output: 0, a: 1, b: 2, out_tier: 2 },
      null,
    ]
    expect(buildRecipeMechanicsAi(recipes, [{ id: 1, name: "制式炮管" }], [{ id: 31, name: "灼热弹幕" }])).toBe("共 4 条配方：二阶 1 条、三阶 2 条")
    expect(buildRecipeMechanicsAi(recipes, [], [])).toBe("共 4 条配方：二阶 1 条、三阶 2 条")
    expect(buildRecipeMechanicsAi([], [], [])).toBe("共 0 条配方：二阶 0 条、三阶 0 条")
    expect(buildRecipeMechanicsAi(recipes, [], [])).toBe(buildRecipeMechanicsAi(recipes, [], []))
    for (const bad of [null, undefined, {}, "recipes"]) expect(buildRecipeMechanicsAi(bad, [], [])).toBe("共 0 条配方：二阶 0 条、三阶 0 条")
    expect(spec).toContain("共 {n} 条配方：二阶 {t2} 条、三阶 {t3} 条")
  })

  it("逐字常量：依据、诚实红线、CTA 固定文案", () => {
    expect(RECIPE_AI_BASIS).toBe("依据：公开 /api/v1/config 的 recipes（与服务端 SeedRecipe / SeedRecipes 同源）；仅展示配方，不改熔炼结算")
    expect(RECIPE_AI_HONESTY).toBe("能否熔炼以游戏内材料拥有状态为准；配置基值非权威战斗数据，实际效果以引擎结算为准，权威回放以 replay_hash 为准")
    expect(RECIPE_AI_CTA).toBe("查看合成配方 →")
  })

  it("ConfigResp 补 recipes 四字段，snake_case 防漂移", () => {
    expect(wiki).toContain("recipes: Array<{")
    for (const f of ["output:", "a:", "b:", "out_tier:"]) expect(wiki).toContain(f)
    expect(wiki).not.toContain("outTier")
  })

  it("R38 配方表四列、选择器、真实调用齐全且不覆盖 R36", () => {
    expect(recipeSection).toContain("r38-recipe-table-scroll")
    expect(wiki.indexOf("r36-composite-mechanics")).toBeLessThan(recipeStart)
    const css = wiki.slice(wiki.indexOf(".r38-recipe-table-scroll"))
    expect(css).toContain("overflow-x: auto")
    expect(css).toContain("max-width: 100%")
    for (const label of ["产物", "材料A", "材料B", "阶数"]) expect(recipeSection).toContain('label="' + label + '"')
    for (const call of ["recipeMaterialsText(row)", "tierText(row.out_tier)", "buildRecipeMechanicsAi(cfg?.recipes ?? [], cfg?.skills ?? [], cfg?.composite_skills ?? [])"]) expect(recipeSection).toContain(call)
    expect(wiki).toContain("@/recipeMechanicsText")
  })

  it("AI 四要素同屏、hash 安全 CTA、searchIndex 可搜配方", () => {
    const ai = recipeSection.slice(recipeSection.indexOf("r38-ai-note"))
    expect(ai).toContain("AI 机制概览")
    expect(ai).toContain(RECIPE_AI_BASIS)
    expect(ai).toContain(RECIPE_AI_HONESTY)
    expect(ai).toContain(RECIPE_AI_CTA)
    expect(ai).toContain("urlFor('skills', 'composite')")
    expect(ai).toContain("goTo({ tab: 'skills', anchor: 'composite' })")
    expect(ai).not.toContain('href="#')
    for (const ph of ["{n}", "{t2}", "{t3}"]) expect(ai).not.toContain(ph)
    expect(searchIndex).toContain("recipe")
    expect(searchIndex).toContain("anchor: 'composite'")
    expect(searchIndex).toContain("c.recipes ?? []")
  })

  it("SPEC 钉死诚实红线、零改动面、三视口与令牌锚点", () => {
    for (const s of ["禁止内嵌参考实现", "禁改 engine/后端/replay/hash/score/settle/loot/tokens", "能否熔炼以游戏内材料拥有状态为准", "配置基值非权威战斗数据", "replay_hash", "MinValue/MaxValue", "（未知技能#N）", "r38-recipe-mechanics", "r38-recipe-table-scroll", "overflow-x:auto", "max-width:100%", "searchIndex", "禁止新增 --r38- 令牌", "390×844", "RED"]) expect(spec).toContain(s)
    for (const anchor of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"]) expect(tokens).toContain(anchor)
    expect(tokens).not.toContain("--r38-")
    expect(wiki).not.toContain("--r38-")
  })
})
