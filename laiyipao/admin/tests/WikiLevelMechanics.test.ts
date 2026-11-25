// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
// 真实导入实施模块：模块不存在时整个 suite RED（禁止内嵌参考实现）
import { bossText, terrainText, armorPermilleText, maxScoreText, MECHANISM_EMPTY, buildLevelMechanicsAi, LEVEL_AI_CTA } from "../src/levelMechanicsText"

const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const spec = read("docs/laiyipao/specs/round34-wiki-level-mechanics/SPEC.md")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const tsTypes = read("laiyipao/admin/src/api/types.gen.ts")
const gameTypes = read("laiyipao/miniapp/src/game/types.ts")
const tokens = read("laiyipao/admin/src/styles/tokens.css")
const levelsStart = wiki.indexOf("<!-- 关卡 -->")
const levelsEnd = wiki.indexOf("<!-- 实战统计 -->")
const levelSection = wiki.slice(levelsStart, levelsEnd)
const typeStart = wiki.indexOf("levels: Array<{")
const levelType = wiki.slice(typeStart, wiki.indexOf("enemies: Array<{", typeStart))

describe("R34 wiki level mechanics text contract", () => {
  it("bossText 严格 true 才是是，其余均空态", () => {
    expect(bossText(true)).toBe("是")
    for (const v of [false, 0, 1, "true", null, undefined, {}, []]) expect(bossText(v)).toBe(MECHANISM_EMPTY)
    expect(MECHANISM_EMPTY).toBe("—")
  })

  it("terrainText 按 kind 去重计数并映射中文，未知 kind 原样", () => {
    expect(terrainText([])).toBe(MECHANISM_EMPTY)
    expect(terrainText(null)).toBe(MECHANISM_EMPTY)
    expect(terrainText("oil_drum")).toBe(MECHANISM_EMPTY)
    expect(terrainText([{ kind: "oil_drum" }])).toBe("油桶×1")
    expect(terrainText([{ kind: "oil_drum" }, { kind: "oil_drum" }, { kind: "charge_tower" }])).toBe("油桶×2、蓄能塔×1")
    expect(terrainText([{ kind: "collapse_wall" }, { kind: "tidal_gate" }])).toBe("塌墙×1、tidal_gate×1")
    expect(terrainText([{ kind: "rotor_vane" }])).toBe("rotor_vane×1")
    expect(terrainText([{ kind: 123 }, null, { kind: "" }])).toBe(MECHANISM_EMPTY)
  })

  it("armorPermilleText 千分号不冒充百分比，非法值归空态", () => {
    expect(armorPermilleText(250)).toBe("250‰")
    expect(armorPermilleText(1)).toBe("1‰")
    for (const v of [0, -2, 1.5, Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, "250", null, undefined, {}]) expect(armorPermilleText(v)).toBe(MECHANISM_EMPTY)
  })

  it("maxScoreText 正整数原值，零与非法值归空态", () => {
    expect(maxScoreText(1200)).toBe("1200")
    expect(maxScoreText(1)).toBe("1")
    for (const v of [0, -1, 1.2, Number.NaN, Number.POSITIVE_INFINITY, "1200", null, undefined, {}]) expect(maxScoreText(v)).toBe(MECHANISM_EMPTY)
  })

  it("buildLevelMechanicsAi 确定性聚合与诚实空态", () => {
    expect(buildLevelMechanicsAi([])).toBe("共 0 关：首领关 0 个，含地形关 0 个，地形种类 0 种")
    const levels: any[] = [
      { is_boss: true, terrain: [{ kind: "oil_drum" }, { kind: "oil_drum" }, { kind: "charge_tower" }] },
      { is_boss: false, terrain: [{ kind: "oil_drum" }] },
      { is_boss: true, terrain: [] },
      { is_boss: "true", terrain: [{ kind: "rotor_vane" }] },
    ]
    const expected = "共 4 关：首领关 2 个，含地形关 3 个，地形种类 3 种"
    expect(buildLevelMechanicsAi(levels)).toBe(expected)
    expect(buildLevelMechanicsAi(levels)).toBe(expected)
    expect(LEVEL_AI_CTA).toBe("查看关卡一览 →")
  })

  it("权威类型与地形种类有据可查", () => {
    expect(tsTypes).toContain("export interface LevelSummary")
    for (const f of ["terrain: TerrainPlacement[]", "is_boss: boolean", "armor_permille: number", "max_score: number"]) expect(tsTypes).toContain(f)
    for (const k of ["oil_drum", "tidal_gate", "rotor_vane", "collapse_wall", "charge_tower"]) expect(gameTypes).toContain(k)
    expect(read("laiyipao/miniapp/src/game/terrain.ts")).toContain("逻辑单位")
  })

  it("WikiView 关卡区选择器、列名与真实调用", () => {
    expect(levelSection).toContain("r34-level-mechanics")
    expect(levelSection).toContain("r34-level-table-scroll")
    expect(levelSection).toContain("r34-ai-note")
    expect(levelSection).toContain("配置基值")
    for (const label of ["首领", "地形", "护甲‰", "满分"]) expect(levelSection).toContain(label)
    for (const call of ["bossText(row.is_boss)", "terrainText(row.terrain)", "armorPermilleText(row.armor_permille)", "maxScoreText(row.max_score)", "buildLevelMechanicsAi(cfg?.levels ?? [])"]) expect(levelSection).toContain(call)
    expect(wiki).toContain("bossText, terrainText, armorPermilleText, maxScoreText, MECHANISM_EMPTY, buildLevelMechanicsAi, LEVEL_AI_CTA")
    for (const field of ["is_boss: boolean", "armor_permille", "max_score", "terrain: unknown[]"]) expect(levelType).toContain(field)
    expect(wiki).toContain(".r34-level-table-scroll")
    const css = wiki.slice(wiki.indexOf(".r34-level-table-scroll"))
    expect(css).toContain("overflow-x: auto")
    expect(css).toContain("max-width: 100%")
  })

  it("AI 四要素与 hash 安全 CTA", () => {
    expect(levelSection).toContain("AI 机制概览")
    expect(levelSection).toContain("依据：公开 /api/v1/config 的 levels（与 H5/Go 引擎同源 SeedLevel）；地形实际触发以结算 terrain_used 为准")
    expect(levelSection).toContain("辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准")
    expect(levelSection).toContain("urlFor")
    expect(levelSection).toContain("goTo({ tab:")
    expect(levelSection).toContain("data-r34-cta")
    expect(levelSection).not.toContain("#levels")
  })

  it("SPEC 锚点与令牌三端不回退", () => {
    expect(spec).toContain("MECHANISM_EMPTY")
    expect(spec).toContain("buildLevelMechanicsAi")
    expect(spec).toContain("terrain_used 为准")
    expect(spec).toContain("禁止内嵌参考实现")
    expect(spec).toContain("RED")
    for (const t of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"]) expect(tokens).toContain(t)
    expect(tokens).not.toContain("--r34-")
    expect(wiki).not.toContain("--r34-")
  })
})