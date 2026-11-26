// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
// 真实导入实施模块：模块不存在时整个 suite RED（禁止内嵌参考实现）
import { MECHANISM_EMPTY, RATING_KEYS, ratingLabel, ratingValue, buildRatingMechanicsAi, RATING_AI_BASIS, RATING_AI_HONESTY, RATING_AI_CTA } from "../src/ratingMechanicsText"
const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const spec = read("docs/laiyipao/specs/round40-wiki-rating-weights/SPEC.md")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const power = read("laiyipao/server/internal/domain/power.go")
const generated = read("laiyipao/admin/src/api/types.gen.ts")
const wikiTest = read("laiyipao/admin/tests/WikiView.test.ts")
const indexPage = read("laiyipao/miniapp/src/pages/index/index.vue")
const miniFixtures = read("laiyipao/miniapp/src/test/fixtures.ts")
const enemyHelper = read("laiyipao/admin/src/enemyMechanicsText.ts")
const tokens = read("laiyipao/admin/src/styles/tokens.css")
const recon = read("docs/laiyipao/recon/ROUND40_RECON.md")
const ratingKeys = ["element_coverage", "reaction_coverage", "mastery_done", "equipment_synergy", "mechanic_depth"] as const
const ratingLabels: Record<string, string> = {
  element_coverage: "元素覆盖",
  reaction_coverage: "反应链",
  mastery_done: "专精投入",
  equipment_synergy: "装备契合",
  mechanic_depth: "机制深度",
}
const ratingValues = { element_coverage: 12, reaction_coverage: 15, mastery_done: 10, equipment_synergy: 12, mechanic_depth: 8 }
const ratingStart = wiki.indexOf("r40-rating-mechanics")
const ratingSection = ratingStart >= 0 ? wiki.slice(ratingStart, wiki.indexOf("</section>", ratingStart)) : ""
describe("R40 wiki rating weights contract", () => {
  it("MECHANISM_EMPTY 复用 R33 同源同值", () => {
    expect(MECHANISM_EMPTY).toBe("—")
    expect(spec).toContain("MECHANISM_EMPTY 从 @/enemyMechanicsText 同源复用")
    expect(spec).toContain("禁止在本模块重新声明")
    expect(enemyHelper).toContain("export const MECHANISM_EMPTY = '—'")
  })
  it("权威 RatingWeights 五键、默认权重与双端消费有据", () => {
    expect(power).toContain("type RatingWeights struct")
    for (const key of ratingKeys) expect(power).toContain('json:"' + key + '"')
    expect(power).toContain("ElementCoverage:  12,")
    expect(power).toContain("ReactionCoverage: 15,")
    expect(power).toContain("MasteryDone:      10,")
    expect(power).toContain("EquipmentSynergy: 12,")
    expect(power).toContain("MechanicDepth:    8,")
    expect(power).toContain("r.Total = len(elemSet)*w.ElementCoverage +")
    expect(power).toContain("reactions*w.ReactionCoverage +")
    expect(generated).toContain("export interface RatingWeights")
    for (const key of ratingKeys) expect(generated).toContain(key + ": number")
    expect(indexPage).toContain("wOf('element_coverage')")
    expect(indexPage).toContain("wOf('reaction_coverage')")
    expect(indexPage).toContain("wOf('mastery_done')")
    expect(indexPage).toContain("wOf('equipment_synergy')")
    expect(indexPage).toContain("wOf('mechanic_depth')")
    expect(miniFixtures).toContain("rating_weights: {")
    for (const key of ratingKeys) expect(miniFixtures).toContain(key + ": 1")
  })
  it("RATING_KEYS 与五权威键顺序一致", () => {
    expect([...RATING_KEYS]).toEqual([...ratingKeys])
  })
  it("ratingLabel 五键映射、未知键逐字兜底、空与脏值为 —", () => {
    for (const key of ratingKeys) expect(ratingLabel(key)).toBe(ratingLabels[key])
    expect(ratingLabel("damage")).toBe("（未知维度#damage）")
    expect(ratingLabel("future_dimension")).toBe("（未知维度#future_dimension）")
    for (const v of ["", "   ", 1, 0, true, false, null, undefined, NaN, Infinity, -Infinity, {}, [], () => 1]) expect(ratingLabel(v as unknown)).toBe(MECHANISM_EMPTY)
  })
  it("ratingValue 只显示有限整数，0 合法，绝不写百分号或千分号", () => {
    for (const key of ratingKeys) expect(ratingValue(ratingValues[key as keyof typeof ratingValues])).toBe(String(ratingValues[key as keyof typeof ratingValues]))
    expect(ratingValue(0)).toBe("0")
    for (const v of [null, undefined, 0.5, -1, NaN, Infinity, -Infinity, "12", true, {}, []]) expect(ratingValue(v as unknown)).toBe(MECHANISM_EMPTY)
    expect(ratingValue(12)).not.toContain("%")
    expect(ratingValue(12)).not.toContain("‰")
  })
  it("buildRatingMechanicsAi 确定性聚合五键齐备数，无脏值残留", () => {
    expect(buildRatingMechanicsAi(ratingValues)).toBe("权重 5/5 项齐备")
    expect(buildRatingMechanicsAi(ratingValues)).toBe(buildRatingMechanicsAi(ratingValues))
    expect(buildRatingMechanicsAi({})).toBe("权重 0/5 项齐备")
    expect(buildRatingMechanicsAi(null)).toBe("权重 0/5 项齐备")
    expect(buildRatingMechanicsAi([])).toBe("权重 0/5 项齐备")
    expect(buildRatingMechanicsAi({ element_coverage: 0, reaction_coverage: 15, mastery_done: null, equipment_synergy: 1.5, mechanic_depth: NaN })).toBe("权重 2/5 项齐备")
    for (const bad of ["undefined", "NaN", "Infinity", "{present}"]) expect(buildRatingMechanicsAi(ratingValues)).not.toContain(bad)
  })
  it("逐字常量：依据、诚实红线、CTA 固定文案", () => {
    expect(RATING_AI_BASIS).toBe("依据：公开 /api/v1/config 的 rating_weights，与服务端 RatingWeights / DefaultRatingWeights 同源；仅展示配置基值")
    expect(RATING_AI_HONESTY).toBe("评分权重是配置基值，不是权威结算；实际收益与边界以游戏内为准；权威回放以 replay_hash 为准")
    expect(RATING_AI_CTA).toBe("查看构筑评分 →")
  })
  it("本地 ConfigResp 收紧为权威五字段，旧夹具与断言必须在实施阶段修正", () => {
    expect(wiki).not.toContain("rating_weights: Record<string, number>")
    expect(wiki).toContain("rating_weights: {")
    for (const key of ratingKeys) expect(wiki).toContain(key + ": number")
    expect(spec).toContain("WikiView.test.ts:69")
    expect(spec).toContain(":157-164")
    expect(spec).toContain("damage/30/tempo")
    expect(spec).toContain("中文标签")
    expect(wikiTest).not.toContain("damage: 30, survival: 20, element: 25, economy: 15, tempo: 10")
    expect(wikiTest).not.toContain("expect(text).toContain('damage')")
    expect(wikiTest).not.toContain("expect(text).toContain('tempo')")
    expect(wikiTest).toContain("元素覆盖")
    expect(wikiTest).toContain("机制深度")
  })
  it("R40 评分表、选择器、真实调用与 AI 四要素齐全", () => {
    for (const s of ["r40-rating-mechanics", "r40-rating-table-scroll", "r40-ai-note"]) expect(wiki).toContain(s)
    const css = wiki.slice(wiki.indexOf(".r40-rating-table-scroll"))
    expect(css).toContain("overflow-x: auto")
    expect(css).toContain("max-width: 100%")
    for (const label of ["维度", "权重"]) expect(ratingSection).toContain('label="' + label + '"')
    expect(wiki).not.toContain("Object.entries(cfg?.rating_weights ?? {})")
    for (const call of ["RATING_KEYS", "ratingLabel(", "ratingValue(cfg?.rating_weights", "buildRatingMechanicsAi(cfg?.rating_weights ?? {})"]) expect(wiki).toContain(call)
    expect(wiki).toContain("@/ratingMechanicsText")
    const ai = ratingSection.slice(ratingSection.indexOf("r40-ai-note"))
    expect(ai).toContain("AI 机制概览")
    expect(ai).toContain(RATING_AI_BASIS)
    expect(ai).toContain(RATING_AI_HONESTY)
    expect(ai).toContain(RATING_AI_CTA)
    expect(ai).toContain("urlFor('rating')")
    expect(ai).toContain("goTo({ tab: 'rating' })")
    expect(ai).not.toContain('href="#')
    expect(ai).not.toContain("{present}")
  })
  it("SPEC 钉死诚实红线、零改动面、夹具修正、三视口与令牌锚点", () => {
    for (const s of ["禁止内嵌参考实现", "禁改 engine/后端/replay/hash/score/settle/loot/tokens", "配置基值，不是权威结算", "replay_hash", "r40-rating-mechanics", "r40-rating-table-scroll", "overflow-x:auto", "max-width:100%", "禁止新增 --r40-* 令牌", "390×844", "RED"]) expect(spec).toContain(s)
    for (const anchor of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"]) expect(tokens).toContain(anchor)
    expect(tokens).not.toContain("--r40-")
    expect(wiki).not.toContain("--r40-")
    expect(recon).toContain("候选 A")
  })

  it("全站搜索索引评分五维度并高亮：rating 类型、索引、行高亮与 SPEC 门禁", () => {
    // 1) HitKind 'rating' 同步三处
    expect(wiki).toMatch(/type HitKind = [^\n]*'rating'/)
    expect(wiki).toMatch(/const HIT_KINDS: HitKind\[\] = \[[^\]]*'rating'[^\]]*\]/)
    expect(wiki).toMatch(/const KIND_LABEL: Record<HitKind, string> = \{[\s\S]*?rating: '评分'/)
    // 2) searchIndex 遍历 RATING_KEYS 构造 rating 命中，body 含 key+中文标签+权重
    const ratingLoopStart = wiki.indexOf("for (const key of RATING_KEYS)")
    expect(ratingLoopStart).toBeGreaterThan(-1)
    const ratingLoop = wiki.slice(ratingLoopStart, wiki.indexOf("return hits", ratingLoopStart))
    expect(ratingLoop).toContain("kind: 'rating'")
    expect(ratingLoop).toContain("id: key")
    expect(ratingLoop).toContain("title: ratingLabel(key)")
    expect(ratingLoop).toContain("section: 'rating'")
    expect(ratingLoop).toContain("body: `")
    expect(ratingLoop).toContain("${key}")
    expect(ratingLoop).toContain("${ratingLabel(key)}")
    expect(ratingLoop).toContain("${ratingValue(")
    // 3) 评分表绑定行高亮；点击命中后 focus=rating:<key> 由 pick() 通用写入
    expect(ratingSection).toContain(":row-class-name=\"rowFocusClass('rating')\"")
    expect(wiki).toContain("focus: { kind: hit.kind, id: hit.id }")
    expect(wiki).toContain("scope.row.id ?? scope.row.key")
    // 4) SPEC 搜索门禁同步 rating 类型与示例，诚实口径不回退
    expect(spec).toContain("rating")
    expect(spec).toContain("评分")
    expect(spec).toContain("反应链")
    expect(spec).toContain("mechanic_depth")
    expect(spec).toContain("配置基值，不是权威结算")
    // 5) 真实 import：Wiki 消费导入的 RATING_KEYS / ratingLabel / ratingValue，不内嵌参考实现
    expect(wiki).toContain("from '@/ratingMechanicsText'")
    for (const call of ["RATING_KEYS", "ratingLabel(", "ratingValue("]) expect(wiki).toContain(call)
  })
})
