// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
// 真实导入实施模块：模块不存在时整个 suite RED（禁止内嵌参考实现）
import { MECHANISM_EMPTY, levelChapterText, buildLevelChapterAi, LEVEL_CHAPTER_AI_BASIS, LEVEL_CHAPTER_AI_HONESTY, LEVEL_CHAPTER_AI_CTA } from "../src/levelChapterText"
import { MECHANISM_EMPTY as LEVEL_MECHANISM_EMPTY } from "../src/levelMechanicsText"
const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const spec = read("docs/laiyipao/specs/round42-wiki-level-chapter/SPEC.md")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const generated = read("laiyipao/admin/src/api/types.gen.ts")
const levelgen = read("laiyipao/server/internal/domain/levelgen.go")
const levelHelper = read("laiyipao/admin/src/levelMechanicsText.ts")
const tokens = read("laiyipao/admin/src/styles/tokens.css")
const chapters = [
  { id: 1, name: "边境哨站", start_level: 1, end_level: 20, terrain_kind: "oil_drum", boss_enemy_id: 0 },
  { id: 2, name: "工业废墟", start_level: 21, end_level: 40, terrain_kind: "collapse_wall", boss_enemy_id: 0 },
  { id: 3, name: "冻原", start_level: 41, end_level: 60, terrain_kind: "charge_tower", boss_enemy_id: 16 },
  { id: 4, name: "感染矿井", start_level: 61, end_level: 80, terrain_kind: "tidal_gate", boss_enemy_id: 0 },
  { id: 5, name: "母巢外围", start_level: 81, end_level: 95, terrain_kind: "rotor_vane", boss_enemy_id: 19 },
  { id: 6, name: "母巢核心", start_level: 96, end_level: 100, terrain_kind: "charge_tower", boss_enemy_id: 22 },
]
const notePos = wiki.indexOf("r42-level-chapter-note")
const chapterNote = notePos >= 0 ? wiki.slice(notePos) : ""
describe("R42 wiki level chapter contract", () => {
  it("MECHANISM_EMPTY 与 @/levelMechanicsText 同源同值", () => {
    expect(MECHANISM_EMPTY).toBe("—")
    expect(MECHANISM_EMPTY).toBe(LEVEL_MECHANISM_EMPTY)
    expect(levelHelper).toContain("export const MECHANISM_EMPTY = '—'")
    expect(spec).toContain("MECHANISM_EMPTY 从 @/levelMechanicsText 同源复用")
    expect(spec).toContain("禁止在本模块重新声明")
  })
  it("权威 chapter 字段与 ChapterOf 有据（生成类型/服务端/本地类型）", () => {
    expect(generated).toContain("chapter: number")
    const summary = generated.slice(generated.indexOf("export interface LevelSummary"))
    expect(summary).toContain("chapter: number")
    expect(levelgen).toContain("ChapterOf 返回关卡所属章节")
    expect(levelgen).toContain("func ChapterOf(levelID int) Chapter")
    expect(wiki).toContain("chapter: number")
    expect(spec).toContain("types.gen.ts:510")
    expect(spec).toContain("levelgen.go:84")
  })
  it("levelChapterText 合法正整数映射章节名，未知章与非法空态", () => {
    for (const c of chapters) expect(levelChapterText(c.id, chapters)).toBe(c.name)
    expect(levelChapterText(1, chapters)).toBe("边境哨站")
    expect(levelChapterText(7, chapters)).toBe("未知章节#7")
    for (const v of [0, -1, 1.5, NaN, Infinity, -Infinity, "1", true, false, null, undefined, {}, []]) {
      expect(levelChapterText(v, chapters)).toBe(MECHANISM_EMPTY)
    }
  })
  it("levelChapterText 全 unknown 安全：chapters 脏值不抛、安全回退", () => {
    expect(levelChapterText(1, null)).toBe("未知章节#1")
    expect(levelChapterText(1, "bad")).toBe("未知章节#1")
    expect(levelChapterText(1, [])).toBe("未知章节#1")
    expect(levelChapterText(1, [null, {}, { id: "1", name: "x" }])).toBe("未知章节#1")
    expect(levelChapterText(1, [{ id: 1, name: "" }])).toBe("未知章节#1")
    expect(levelChapterText(1, [{ id: 0, name: "zero" }])).toBe("未知章节#1")
    expect(() => levelChapterText(null, null)).not.toThrow()
  })
  it("buildLevelChapterAi 确定性聚合 共N关，覆盖M章", () => {
    const levels = [
      { chapter: 1 }, { chapter: 1 }, { chapter: 2 }, { chapter: 6 },
      { chapter: 0 }, { chapter: null }, { chapter: "3" },
    ]
    expect(buildLevelChapterAi(levels, chapters)).toBe("共7关，覆盖3章")
    expect(buildLevelChapterAi(levels, chapters)).toBe(buildLevelChapterAi(levels, chapters))
    expect(buildLevelChapterAi([], chapters)).toBe("共0关，覆盖0章")
    for (const bad of ["undefined", "NaN", "Infinity", "[object Object]"]) {
      expect(buildLevelChapterAi(levels, chapters)).not.toContain(bad)
    }
  })
  it("buildLevelChapterAi 全 unknown 安全", () => {
    for (const v of [null, undefined, {}, "bad", 0, true]) {
      expect(buildLevelChapterAi(v, null)).toBe("共0关，覆盖0章")
    }
    expect(buildLevelChapterAi([{}, null, "x"], undefined)).toBe("共3关，覆盖0章")
    expect(() => buildLevelChapterAi(null, null)).not.toThrow()
  })
  it("逐字常量：依据、诚实红线、CTA（查看章节一览 →）", () => {
    expect(LEVEL_CHAPTER_AI_BASIS).toBe("依据：公开 /api/v1/config 的 levels.chapter 与 chapters（与服务端 LevelSummary.chapter / domain.ChapterOf 同源）")
    expect(LEVEL_CHAPTER_AI_HONESTY).toBe("章节是配置归属，不是地形使用 terrain_used，也不参与 replay_hash；实际战斗数据以结算为准，差异以公开 /api/v1/config 为准")
    expect(LEVEL_CHAPTER_AI_CTA).toBe("查看章节一览 →")
  })
  it("关卡表新增所属章节列、交叉引用链接与真实调用", () => {
    for (const s of ["levelChapterText", "所属章节", "r42-level-chapter-note"]) expect(wiki).toContain(s)
    expect(wiki).toContain("from '@/levelChapterText'")
    expect(wiki).toContain("levelChapterText(row.chapter, cfg?.chapters ?? [])")
    expect(wiki).toContain("goTo({ tab: 'levels', anchor: 'chapter-mechanics' })")
    expect(wiki).toContain("urlFor('levels', 'chapter-mechanics')")
    for (const label of ["关", "名称", "难度", "防线血量", "波次", "体力", "层数上限", "反应阶", "星级门槛", "首领", "护甲‰", "满分"]) {
      expect(wiki).toContain('label="' + label + '"')
    }
    expect(wiki).toContain("rowFocusClass('level')")
    expect(wiki).toContain("r34-level-table-scroll")
  })
  it("AI 四要素同屏与 CTA hash 安全", () => {
    expect(chapterNote).not.toBe("")
    expect(chapterNote).toContain("AI 章节交叉概览")
    expect(chapterNote).toContain(LEVEL_CHAPTER_AI_BASIS)
    expect(chapterNote).toContain(LEVEL_CHAPTER_AI_HONESTY)
    expect(chapterNote).toContain(LEVEL_CHAPTER_AI_CTA)
    expect(chapterNote).toContain("buildLevelChapterAi(cfg?.levels ?? [], cfg?.chapters ?? [])")
    expect(chapterNote).toContain("urlFor('levels')")
    expect(chapterNote).toContain("goTo({ tab: 'levels', anchor: 'chapter-mechanics' })")
    expect(chapterNote).not.toContain('href="#')
  })
  it("SPEC 钉死红线、零改动面、三视口与令牌锚点", () => {
    for (const s of ["禁止内嵌参考实现", "禁改 engine/后端/replay/hash/score/settle/loot/tokens", "不得混同权威战斗数据", "terrain_used", "replay_hash", "禁止新增 --r42-* 令牌", "390×844", "768×1024", "1280×800", "RED", "Failed to resolve import '../src/levelChapterText'"]) expect(spec).toContain(s)
    for (const anchor of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"]) expect(tokens).toContain(anchor)
    expect(tokens).not.toContain("--r42-")
    expect(wiki).not.toContain("--r42-")
  })
  it("搜索门禁复用 chapter 命中，既有 level/skin/rating 不回退", () => {
    expect(wiki).toContain("kind: 'chapter'")
    expect(wiki).toContain("kind: 'level'")
    expect(wiki).toContain("kind: 'skin'")
    expect(wiki).toContain("kind: 'rating'")
    expect(wiki).toContain("chapter: '章节'")
    expect(wiki).toContain("skin: '皮肤'")
    expect(wiki).toContain("rating: '评分'")
    expect(spec).toContain("不新增搜索 kind")
    for (const s of ["chapter", "章节", "rating", "评分", "skin", "皮肤", "不回退"]) expect(spec).toContain(s)
  })
})
