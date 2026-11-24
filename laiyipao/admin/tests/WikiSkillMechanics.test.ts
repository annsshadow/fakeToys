// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
// 真实导入实施模块：模块不存在时整个 suite RED（禁止内嵌参考实现）
import { kindText, cooldownText, pierceText, aoeRadiusText, chainText, MECHANISM_EMPTY, buildSkillMechanicsAi, SKILL_AI_CTA } from "../src/skillMechanicsText"

const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const spec = read("docs/laiyipao/specs/round32-wiki-skill-mechanics/SPEC.md")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const goSrc = read("laiyipao/server/internal/domain/content.go")
const goCombat = read("laiyipao/server/internal/domain/engine_combat.go")
const engine = read("laiyipao/miniapp/src/game/engine.ts")
const tokens = read("laiyipao/admin/src/styles/tokens.css")
const skipTable = wiki.slice(wiki.indexOf("id=\"skill-table\""), wiki.indexOf("id=\"composite\""))

describe("R32 skill mechanics text contract", () => {
  it("kindText 区分主动/被动/未知，不伪造空值", () => {
    expect(kindText("active")).toBe("主动")
    expect(kindText("passive")).toBe("被动")
    expect(kindText("")).toBe("未知类型")
    expect(kindText("weird")).toBe("未知类型")
    expect(kindText(null)).toBe("未知类型")
    expect(kindText(1)).toBe("未知类型")
  })

  it("cooldownText 毫秒转秒且 0/非法走安全空态", () => {
    expect(cooldownText(800)).toBe("0.8秒")
    expect(cooldownText(1500)).toBe("1.5秒")
    expect(cooldownText(3000)).toBe("3秒")
    expect(cooldownText(0)).toBe(MECHANISM_EMPTY)
    expect(cooldownText(-1)).toBe(MECHANISM_EMPTY)
    expect(cooldownText(Number.NaN)).toBe(MECHANISM_EMPTY)
    expect(cooldownText(Number.POSITIVE_INFINITY)).toBe(MECHANISM_EMPTY)
    expect(cooldownText("800")).toBe(MECHANISM_EMPTY)
  })

  it("pierce/aoe/chain 必须整数，小数/NaN/Infinity/非数字归空态", () => {
    expect(pierceText(1)).toBe("穿透 1")
    expect(pierceText(8)).toBe("穿透 8")
    expect(pierceText(0)).toBe(MECHANISM_EMPTY)
    expect(pierceText(-2)).toBe(MECHANISM_EMPTY)
    expect(pierceText(1.5)).toBe(MECHANISM_EMPTY)
    expect(pierceText(Number.NaN)).toBe(MECHANISM_EMPTY)
    expect(pierceText(Number.POSITIVE_INFINITY)).toBe(MECHANISM_EMPTY)
    expect(pierceText("3")).toBe(MECHANISM_EMPTY)
    expect(pierceText(null)).toBe(MECHANISM_EMPTY)
    expect(aoeRadiusText(180)).toBe("范围 180")
    expect(aoeRadiusText(0)).toBe(MECHANISM_EMPTY)
    expect(aoeRadiusText(1.5)).toBe(MECHANISM_EMPTY)
    expect(aoeRadiusText(Number.NaN)).toBe(MECHANISM_EMPTY)
    expect(aoeRadiusText(Number.POSITIVE_INFINITY)).toBe(MECHANISM_EMPTY)
    expect(aoeRadiusText(undefined)).toBe(MECHANISM_EMPTY)
    expect(chainText(4)).toBe("连锁 4")
    expect(chainText(2)).toBe("连锁 2")
    expect(chainText(0)).toBe(MECHANISM_EMPTY)
    expect(chainText(-1)).toBe(MECHANISM_EMPTY)
    expect(chainText(1.5)).toBe(MECHANISM_EMPTY)
    expect(chainText(Number.NaN)).toBe(MECHANISM_EMPTY)
    expect(chainText(Number.POSITIVE_INFINITY)).toBe(MECHANISM_EMPTY)
    expect(chainText({})).toBe(MECHANISM_EMPTY)
  })

  it("AI 聚合由 cfg.skills 确定性推导，不依赖选中行", () => {
    const rows = [
      { kind: "active", pierce: 1, aoe_radius: 180, chain: 1 },
      { kind: "passive", pierce: 0, aoe_radius: 0, chain: 0 },
      { kind: "active", pierce: 0, aoe_radius: 0, chain: 2 },
      { kind: "active", pierce: 0, aoe_radius: 0, chain: 0 },
    ]
    expect(buildSkillMechanicsAi(rows)).toBe("共 4 个技能：主动 3 个，被动 1 个，多目标 2 个")
    expect(buildSkillMechanicsAi([])).toBe("共 0 个技能：主动 0 个，被动 0 个，多目标 0 个")
    expect(SKILL_AI_CTA).toBe("查看合成技能 →")
    expect(spec).toContain("共 {total} 个技能：主动 {active} 个，被动 {passive} 个，多目标 {multi} 个")
    expect(spec).toContain("pierce > 0 || aoe_radius > 0 || chain > 0")
    expect(spec).toContain("查看合成技能 →")
    expect(spec).toContain("#composite")
    expect(spec).not.toContain("当前行")
  })

  it("SPEC 钉死像素半径/整数口径/零值文案", () => {
    for (const s of ["像素半径整数", "Number.isInteger", "unknown", "0.8秒", "1.5秒", "3秒", "未知类型", "穿透 N", "范围 N", "连锁 N", "MECHANISM_EMPTY = '—'", "ms === 0", "n === 0", "不产 NaN", "1.5"]) expect(spec).toContain(s)
    expect(spec).not.toContain("原始定点半径整数")
  })

  it("像素半径证据在 Go/TS 双方引擎坐实", () => {
    expect(goCombat).toContain("rr := ToFixed(float64(radius))")
    expect(engine).toContain("private applyAoe(p: Projectile, origin: Enemy, radius: number): void")
    expect(engine).toContain("const rr = toFixed(radius)")
  })

  it("权威结构与引擎消费点未回退", () => {
    for (const s of ["Kind            string  `json:\"kind\"`", "Pierce          int64   `json:\"pierce\"`", "AoeRadius       int64   `json:\"aoe_radius\"`", "Chain           int64   `json:\"chain\"`", "CooldownMs      int64   `json:\"cooldown_ms\"`"])
      expect(goSrc).toContain(s)
    for (const s of ["const evasiveCapable = s.aoeRadius > 0 || s.pierce > 0", "pierceLeft: s.pierce + this.buffs.pierceBonus", "chainLeft: s.chain + this.buffs.chainBonus", "aoeRadius: s.aoeRadius + this.buffs.aoeBonus"])
      expect(engine).toContain(s)
  })

  it("Wiki 技能表真实导入并调用五个格式化函数与 AI 聚合，列名齐全", () => {
    expect(wiki).toContain("import { kindText, cooldownText, pierceText, aoeRadiusText, chainText")
    for (const name of ["kindText", "cooldownText", "pierceText", "aoeRadiusText", "chainText", "buildSkillMechanicsAi"])
      expect(wiki).toMatch(new RegExp("\\b" + name + "\\s*\\("))
    for (const col of ["label=\"类型\"", "label=\"冷却\"", "label=\"穿透\"", "label=\"范围\"", "label=\"连锁\""])
      expect(skipTable).toContain(col)
  })

  it("Wiki 技能表新增 Scroll 容器与 AI 四要素同屏", () => {
    expect(skipTable).toContain("r32-skill-mechanics")
    expect(skipTable).toContain("r32-skill-table-scroll")
    const aiBlock = skipTable.slice(skipTable.indexOf("r32-ai-note"))
    expect(aiBlock).toContain("AI")
    expect(wiki).toContain("<strong>AI 机制概览：</strong>{{ buildSkillMechanicsAi(cfg?.skills ?? []) }}")
    expect(aiBlock).not.toContain("全部技能由配置决定")
    expect(aiBlock).not.toContain("按公开配置汇总")
    expect(aiBlock).toContain("依据：公开 /api/v1/config 的 skills（与 H5 引擎同源 SeedSkill）；格式化仅展示，不改结算")
    expect(aiBlock).toContain("查看合成技能 →")
    // 旧守卫只认裸字符串 #composite；修复后链接改走 hash-router 安全形式
    expect(aiBlock).toContain("urlFor('skills', 'composite')")
    expect(aiBlock).toContain("辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准")
    for (const ph of ["{total}", "{active}", "{passive}", "{multi}"]) expect(aiBlock).not.toContain(ph)
    // R32 修复：裸 href="#composite" 在 createWebHashHistory 下会替换整段 hash，
    // 被 catch-all 重定向到 /admin/dashboard（真实缺陷）。CTA 必须复用目录导航的安全链接。
    expect(aiBlock).not.toContain('href="#composite"')
    expect(aiBlock).toContain(':href="urlFor(\'skills\', \'composite\')"')
    expect(aiBlock).toContain('@click.prevent="goTo({ tab: \'skills\', anchor: \'composite\' })"')
  })

  it("只读展示不改引擎与后端权威面", () => {
    expect(spec).toContain("不改 `engine.ts` / 后端 / replay / hash / score / settle / loot / tokens")
    expect(spec).toContain("实际效果以引擎结算为准")
    expect(spec).toContain("权威回放以 replay_hash 为准")
  })

  it("三端 token 锚点保留且禁 --r32-", () => {
    for (const anchor of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"])
      expect(tokens).toContain(anchor)
    expect(spec).toContain("禁止新增 `--r32-*` 令牌")
    expect(tokens).not.toContain("--r32-")
  })
})