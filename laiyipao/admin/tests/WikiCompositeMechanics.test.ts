// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
// 真实导入实施模块：模块不存在时整个 suite RED（禁止内嵌参考实现）
import { MECHANISM_EMPTY, compositeUnlockText, buildCompositeMechanicsAi, COMPOSITE_AI_BASIS, COMPOSITE_AI_HONESTY, COMPOSITE_AI_CTA } from "../src/compositeMechanicsText"

const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const spec = read("docs/laiyipao/specs/round36-wiki-composite-skills/SPEC.md")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const content = read("laiyipao/server/internal/domain/content.go")
const service = read("laiyipao/server/internal/service/service.go")
const generated = read("laiyipao/admin/src/api/types.gen.ts")
const engine = read("laiyipao/miniapp/src/game/engine.ts")
const tokens = read("laiyipao/admin/src/styles/tokens.css")
const typeStart = wiki.indexOf("composite_skills: Array<{")
const compositeType = typeStart >= 0 ? wiki.slice(typeStart, wiki.indexOf("}>", typeStart) + 2) : ""
const compositeStart = wiki.indexOf('id="composite"')
const compositeSection = compositeStart >= 0 ? wiki.slice(compositeStart, wiki.indexOf("</section>", compositeStart)) : ""

describe("R36 wiki composite skills mechanics contract", () => {
  it("MECHANISM_EMPTY 与 compositeUnlockText 只认正整数，0/非法归空态", () => {
    expect(MECHANISM_EMPTY).toBe("—")
    expect(compositeUnlockText(1)).toBe("1 级解锁")
    expect(compositeUnlockText(30)).toBe("30 级解锁")
    expect(compositeUnlockText(70)).toBe("70 级解锁")
    for (const v of [0, -1, 1.5, Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, "30", null, undefined, {}, []]) {
      expect(compositeUnlockText(v)).toBe(MECHANISM_EMPTY)
    }
  })

  it("buildCompositeMechanicsAi 由配置确定性聚合总数/主动/多目标，空白诚实", () => {
    const rows = [
      { kind: "active", pierce: 0, aoe_radius: 190, chain: 1 },
      { kind: "active", pierce: 2, aoe_radius: 0, chain: 2 },
      { kind: "passive", pierce: 0, aoe_radius: 0, chain: 0 },
      { kind: "active", pierce: 0, aoe_radius: 0, chain: 0 },
      { kind: "weird", pierce: 1.5, aoe_radius: "4", chain: Number.NaN },
    ]
    expect(buildCompositeMechanicsAi(rows)).toBe("共 5 个合成技能：主动 3 个，多目标 2 个")
    expect(buildCompositeMechanicsAi(rows)).toBe(buildCompositeMechanicsAi(rows))
    expect(buildCompositeMechanicsAi([])).toBe("共 0 个合成技能：主动 0 个，多目标 0 个")
    expect(spec).toContain("共 {total} 个合成技能：主动 {active} 个，多目标 {multi} 个")
    expect(spec).toContain("pierce > 0 || aoe_radius > 0 || chain > 0")
  })

  it("逐字常量：依据、红线、CTA 固定文案", () => {
    expect(COMPOSITE_AI_BASIS).toBe("依据：公开 /api/v1/config 的 composite_skills（与服务端 SeedSkill / SeedCompositeSkills 同源）；格式化仅展示，不改结算")
    expect(COMPOSITE_AI_HONESTY).toBe("辅助展示非权威战斗数据，展示为配置基值，实际效果以引擎结算为准；权威回放以 replay_hash 为准")
    expect(COMPOSITE_AI_CTA).toBe("查看合成技能 →")
  })

  it("权威 SeedSkill / SeedCompositeSkills / 公开 config / 引擎消费有据", () => {
    expect(service).toContain('json:"composite_skills"')
    expect(service).toContain("domain.ScaleAllCompositeSkills()")
    expect(service).toContain("domain.ScaleAllSkills()")
    expect(content).toContain("type SeedSkill struct")
    for (const f of ['json:"kind"', 'json:"base_damage"', 'json:"heat_cost"', 'json:"cooldown_ms"', 'json:"pierce"', 'json:"aoe_radius"', 'json:"apply_stacks"', 'json:"chain"', 'json:"unlock_level"']) expect(content).toContain(f)
    expect(content).toContain("var SeedCompositeSkills = []SeedSkill{")
    for (const id of [31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48]) expect(content).toContain("ID: " + id + ",")
    expect(generated).toContain("export interface SeedSkill")
    expect(generated).toContain("kind: string")
    for (const f of ["base_damage: number", "heat_cost: number", "cooldown_ms: number", "pierce: number", "aoe_radius: number", "apply_stacks: number", "chain: number", "unlock_level: number"]) expect(generated).toContain(f)
    for (const s of ["const evasiveCapable = s.aoeRadius > 0 || s.pierce > 0", "pierceLeft: s.pierce + this.buffs.pierceBonus", "chainLeft: s.chain + this.buffs.chainBonus", "aoeRadius: s.aoeRadius + this.buffs.aoeBonus"]) expect(engine).toContain(s)
    expect(wiki).toContain("api.get<ConfigResp>('/config', { auth: false })")
  })

  it("ConfigResp.composite_skills 类型补齐权威字段，防四字段漂移回归", () => {
    expect(typeStart).toBeGreaterThanOrEqual(0)
    for (const f of ["id:", "name:", "element:", "descr:", "kind:", "base_damage:", "heat_cost:", "cooldown_ms:", "pierce:", "aoe_radius:", "apply_stacks:", "chain:", "unlock_level:"]) expect(compositeType).toContain(f)
  })

  it("合成技能表新增九列并真实调用复用格式化函数", () => {
    expect(compositeSection).toContain("r36-composite-mechanics")
    expect(compositeSection).toContain("r36-composite-table-scroll")
    const css = wiki.slice(wiki.indexOf(".r36-composite-table-scroll"))
    expect(css).toContain("overflow-x: auto")
    expect(css).toContain("max-width: 100%")
    for (const label of ["类型", "冷却", "伤害", "热量", "穿透", "范围", "连锁", "层数", "解锁等级"]) expect(compositeSection).toContain('label="' + label + '"')
    for (const call of ["kindText(row.kind)", "cooldownText(row.cooldown_ms)", "pierceText(row.pierce)", "aoeRadiusText(row.aoe_radius)", "chainText(row.chain)", "compositeUnlockText(row.unlock_level)"]) expect(compositeSection).toContain(call)
    expect(wiki).toContain("@/compositeMechanicsText")
  })

  it("AI 四要素同屏且 CTA 走 hash-router 安全链接", () => {
    expect(compositeSection).toContain("AI 机制概览")
    expect(compositeSection).toContain(COMPOSITE_AI_BASIS)
    expect(compositeSection).toContain(COMPOSITE_AI_HONESTY)
    expect(compositeSection).toContain(COMPOSITE_AI_CTA)
    expect(compositeSection).toContain("buildCompositeMechanicsAi(cfg?.composite_skills ?? [])")
    expect(compositeSection).toContain("urlFor('skills', 'composite')")
    expect(compositeSection).toContain("@click.prevent=\"goTo({ tab: 'skills', anchor: 'composite' })\"")
    expect(compositeSection).not.toContain('href="#composite"')
  })

  it("SPEC 钉死复用/零改动面/诚实边界与令牌三锚点", () => {
    for (const s of ["禁止内嵌参考实现", "复用 @/skillMechanicsText", "配置基值", "replay_hash", "compositeUnlockText", "unlock_level", "r36-composite-mechanics", "r36-composite-table-scroll", "overflow-x: auto", "max-width: 100%", "禁止新增 `--r36-*` 令牌", "RED"]) expect(spec).toContain(s)
    for (const anchor of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"]) expect(tokens).toContain(anchor)
    expect(tokens).not.toContain("--r36-")
    expect(wiki).not.toContain("--r36-")
  })
})
