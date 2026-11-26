// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
// 真实导入实施模块：模块不存在时整个 suite RED（禁止内嵌参考实现）
import { MECHANISM_EMPTY, rangeText, terrainKindText, bossText, buildChapterMechanicsAi, CHAPTER_AI_BASIS, CHAPTER_AI_HONESTY, CHAPTER_AI_CTA } from "../src/chapterMechanicsText"
import { MECHANISM_EMPTY as LEVEL_MECHANISM_EMPTY } from "../src/levelMechanicsText"
const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const spec = read("docs/laiyipao/specs/round41-wiki-chapters/SPEC.md")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const generated = read("laiyipao/admin/src/api/types.gen.ts")
const levelgen = read("laiyipao/server/internal/domain/levelgen.go")
const service = read("laiyipao/server/internal/service/service.go")
const battleCollections = read("laiyipao/server/internal/domain/battle_collections.go")
const levelHelper = read("laiyipao/admin/src/levelMechanicsText.ts")
const tokens = read("laiyipao/admin/src/styles/tokens.css")
const kinds = ["oil_drum", "collapse_wall", "charge_tower", "tidal_gate", "rotor_vane"] as const
const kindLabels: Record<string, string> = {
  oil_drum: "油桶",
  collapse_wall: "塌墙",
  charge_tower: "蓄能塔",
  tidal_gate: "潮汐闸门",
  rotor_vane: "旋翼风扇",
}
const chapterStart = wiki.indexOf("r41-chapter-mechanics")
const chapterSection = chapterStart >= 0 ? wiki.slice(chapterStart, wiki.indexOf("</section>", chapterStart)) : ""
describe("R41 wiki chapters contract", () => {
  it("MECHANISM_EMPTY 与 @/levelMechanicsText 同源同值", () => {
    expect(MECHANISM_EMPTY).toBe("—")
    expect(MECHANISM_EMPTY).toBe(LEVEL_MECHANISM_EMPTY)
    expect(levelHelper).toContain("export const MECHANISM_EMPTY = '—'")
    expect(spec).toContain("MECHANISM_EMPTY 从 @/levelMechanicsText 同源复用")
    expect(spec).toContain("禁止在本模块重新声明")
  })
  it("权威章节六字段与五地形有据（服务端/生成类型/权威定义）", () => {
    const chapterType = generated.slice(generated.indexOf("export interface ChapterInfo"), generated.indexOf("export interface Dashboard"))
    for (const f of ["id: number", "name: string", "start_level: number", "end_level: number", "terrain_kind: string", "boss_enemy_id: number"]) expect(chapterType).toContain(f)
    const svc = service.slice(service.indexOf("type ChapterInfo struct"), service.indexOf("// LoadGameConfig"))
    for (const f of ['json:"id"', 'json:"name"', 'json:"start_level"', 'json:"end_level"', 'json:"terrain_kind"', 'json:"boss_enemy_id"']) expect(svc).toContain(f)
    expect(service).toContain("domain.AllChapters()")
    const chDefs = levelgen.slice(levelgen.indexOf("var chapters = []Chapter{"), levelgen.indexOf("// AllChapters"))
    for (const k of kinds) expect(chDefs).toContain('"' + k + '"')
    expect(levelgen).toContain("oil_drum / tidal_gate / rotor_vane / collapse_wall / charge_tower")
    expect(chDefs).toContain("BossEnemyID: 16")
    expect(chDefs).toContain("BossEnemyID: 19")
    expect(chDefs).toContain("BossEnemyID: 22")
    expect(battleCollections).toContain("terrain_used")
    expect(spec).toContain("types.gen.ts:435-441")
    expect(spec).toContain("levelgen.go:42-73")
  })
  it("terrainKindText 五类映射与 levelMechanicsText 同值，未知原样、非法空态", () => {
    for (const k of kinds) expect(terrainKindText(k)).toBe(kindLabels[k])
    expect(levelHelper).toContain("oil_drum: '油桶'")
    expect(levelHelper).toContain("collapse_wall: '塌墙'")
    expect(levelHelper).toContain("charge_tower: '蓄能塔'")
    expect(terrainKindText("future_terrain")).toBe("future_terrain")
    for (const v of ["", 0, 1, true, false, null, undefined, NaN, Infinity, -Infinity, {}, []]) expect(terrainKindText(v as unknown)).toBe(MECHANISM_EMPTY)
  })
  it("rangeText 合法正整数区间用 en-dash，其余空态", () => {
    expect(rangeText(41, 60)).toBe("41–60关")
    expect(rangeText(1, 100)).toBe("1–100关")
    for (const bad of [0, -1, 1.5, NaN, Infinity, "1", true, null, undefined, {}, []]) expect(rangeText(bad, 60)).toBe(MECHANISM_EMPTY)
    for (const bad of [0, -1, 1.5, NaN, Infinity, "60", false, null, undefined, {}, []]) expect(rangeText(41, bad)).toBe(MECHANISM_EMPTY)
    expect(rangeText(41, 60)).not.toContain("-")
  })
  it("bossText 仅正整数显示 #N，布尔一律空态（首领敌人 id 非布尔）", () => {
    expect(bossText(16)).toBe("#16")
    expect(bossText(19)).toBe("#19")
    expect(bossText(22)).toBe("#22")
    for (const v of [0, -1, 1.5, NaN, Infinity, -Infinity, "16", true, false, null, undefined, {}, []]) expect(bossText(v as unknown)).toBe(MECHANISM_EMPTY)
    expect(bossText(true)).not.toContain("#")
  })
  it("buildChapterMechanicsAi 确定性聚合六章，无脏值残留", () => {
    const six = [
      { start_level: 1, end_level: 20, terrain_kind: "oil_drum", boss_enemy_id: 0 },
      { start_level: 21, end_level: 40, terrain_kind: "collapse_wall", boss_enemy_id: 0 },
      { start_level: 41, end_level: 60, terrain_kind: "charge_tower", boss_enemy_id: 16 },
      { start_level: 61, end_level: 80, terrain_kind: "tidal_gate", boss_enemy_id: 0 },
      { start_level: 81, end_level: 95, terrain_kind: "rotor_vane", boss_enemy_id: 19 },
      { start_level: 96, end_level: 100, terrain_kind: "charge_tower", boss_enemy_id: 22 },
    ]
    expect(buildChapterMechanicsAi(six)).toBe("共 6 章：含首领 3 章，地形 5 种")
    expect(buildChapterMechanicsAi(six)).toBe(buildChapterMechanicsAi(six))
    expect(buildChapterMechanicsAi([])).toBe("共 0 章：含首领 0 章，地形 0 种")
    expect(buildChapterMechanicsAi(null)).toBe("共 0 章：含首领 0 章，地形 0 种")
    expect(buildChapterMechanicsAi([{ terrain_kind: "tidal_gate", boss_enemy_id: true }, { terrain_kind: "future_kind", boss_enemy_id: 0 }, { terrain_kind: "", boss_enemy_id: NaN }])).toBe("共 3 章：含首领 0 章，地形 2 种")
    for (const bad of ["undefined", "NaN", "Infinity", "[object Object]"]) expect(buildChapterMechanicsAi(six)).not.toContain(bad)
  })
  it("逐字常量：依据、诚实红线、CTA（查看章节一览 →）", () => {
    expect(CHAPTER_AI_BASIS).toBe("依据：公开 /api/v1/config 的 chapters（与服务端 domain.AllChapters 同源）；章节 terrain_kind 为配置基值，实际地形触发以结算 terrain_used 为准")
    expect(CHAPTER_AI_HONESTY).toBe("章节 terrain_kind 是配置基值，与关卡 terrain_used（参与 replay_hash）不同概念，不得混同权威战斗数据；示例值不硬编码，差异以公开 /api/v1/config 为准")
    expect(CHAPTER_AI_CTA).toBe("查看章节一览 →")
  })
  it("R41 章节表、选择器、真实调用与 AI 四要素齐全", () => {
    for (const s of ["r41-chapter-mechanics", "r41-chapter-table-scroll", "r41-ai-note"]) expect(wiki).toContain(s)
    const css = wiki.slice(wiki.indexOf(".r41-chapter-table-scroll"))
    expect(css).toContain("overflow-x: auto")
    expect(css).toContain("max-width: 100%")
    for (const label of ["ID", "名称", "关卡区间", "地形", "首领敌人", "说明"]) expect(chapterSection).toContain('label="' + label)
    for (const call of [":data=\"cfg?.chapters ?? []\"", "rangeText(row.start_level, row.end_level)", "terrainKindText(row.terrain_kind)", "bossText(row.boss_enemy_id)", "buildChapterMechanicsAi(cfg?.chapters ?? [])"]) expect(chapterSection).toContain(call)
    expect(chapterSection).toContain(":row-class-name=\"rowFocusClass('chapter')\"")
    expect(chapterSection).toContain("配置基值")
    expect(wiki).toContain("from '@/chapterMechanicsText'")
    const ai = chapterSection.slice(chapterSection.indexOf("r41-ai-note"))
    expect(ai).toContain("AI 章节概览")
    expect(ai).toContain(CHAPTER_AI_BASIS)
    expect(ai).toContain(CHAPTER_AI_HONESTY)
    expect(ai).toContain(CHAPTER_AI_CTA)
    expect(ai).toContain("urlFor('levels')")
    expect(ai).toContain("goTo({ tab: 'levels' })")
    expect(ai).not.toContain('href="#')
  })
  it("SPEC 钉死诚实红线、零改动面、三视口与令牌锚点", () => {
    for (const s of ["禁止内嵌参考实现", "禁改 engine/后端/replay/hash/score/settle/loot/tokens", "不得混同权威战斗数据", "terrain_used", "replay_hash", "r41-chapter-mechanics", "overflow-x:auto", "max-width:100%", "禁止新增 --r41-* 令牌", "390×844", "768×1024", "1280×800", "RED", 'Failed to resolve import "../src/chapterMechanicsText"']) expect(spec).toContain(s)
    for (const anchor of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"]) expect(tokens).toContain(anchor)
    expect(tokens).not.toContain("--r41-")
    expect(wiki).not.toContain("--r41-")
  })
  it("全站搜索索引章节 chapter 且不回退评分/皮肤：类型、索引、高亮与 SPEC 门禁", () => {
    expect(wiki).toMatch(/type HitKind = [^\n]*'chapter'/)
    expect(wiki).toMatch(/const HIT_KINDS: HitKind\[\] = \[[^\]]*'chapter'[^\]]*\]/)
    expect(wiki).toMatch(/const KIND_LABEL: Record<HitKind, string> = \{[\s\S]*?chapter: '章节'/)
    const loopStart = wiki.indexOf("for (const ch of c.chapters ?? [])")
    expect(loopStart).toBeGreaterThan(-1)
    const loop = wiki.slice(loopStart, wiki.indexOf("return hits", loopStart))
    for (const s of ["kind: 'chapter'", "id: String(ch.id)", "title: ch.name", "rangeText(ch.start_level, ch.end_level)", "terrainKindText(ch.terrain_kind)", "ch.terrain_kind", "section: 'levels'", "anchor: 'chapter-mechanics'"]) expect(loop).toContain(s)
    expect(chapterSection).toContain('id="chapter-mechanics"')
    expect(chapterSection).toContain("rowFocusClass('chapter')")
    expect(wiki).toContain("focus: { kind: hit.kind, id: hit.id }")
    expect(wiki).toContain("scope.row.id ?? scope.row.key")
    expect(wiki).toMatch(/type HitKind = [^\n]*'skin'[^\n]*'rating'/)
    expect(wiki).toMatch(/const HIT_KINDS: HitKind\[\] = \[[^\]]*'skin'[^\]]*'rating'[^\]]*\]/)
    expect(wiki).toMatch(/const KIND_LABEL: Record<HitKind, string> = \{[\s\S]*?skin: '皮肤'[\s\S]*?rating: '评分'/)
    expect(wiki).toContain("kind: 'rating'")
    expect(wiki).toContain("kind: 'skin'")
    for (const s of ["chapter", "章节", "rating", "评分", "skin", "皮肤", "terrain_kind", "配置基值，不是权威结算"]) expect(spec).toContain(s)
  })
})
