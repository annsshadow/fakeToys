// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { describe, it, expect } from "vitest"
import { readFileSync } from "node:fs"
import { execFileSync } from "node:child_process"
import { resolve } from "node:path"
// 真实导入实施模块：模块不存在时整个 suite RED（禁止内嵌参考实现）
import { speedText, armorText, shieldHpText, attackText, attackRangeText, attackIntervalText, flyHeightText, boolText, MECHANISM_EMPTY, buildEnemyMechanicsAi, ENEMY_AI_CTA } from "../src/enemyMechanicsText"

const REPO = resolve(__dirname, "../../..")
const read = (p: string) => readFileSync(resolve(REPO, p), "utf-8")
const spec = read("docs/laiyipao/specs/round33-wiki-enemy-mechanics/SPEC.md")
const wiki = read("laiyipao/admin/src/views/WikiView.vue")
const goSrc = read("laiyipao/server/internal/domain/content.go")
const goEngine = read("laiyipao/server/internal/domain/engine.go")
const goWave = read("laiyipao/server/internal/domain/engine_wave.go")
const engine = read("laiyipao/miniapp/src/game/engine.ts")
const tsTypes = read("laiyipao/miniapp/src/game/types.ts")
const tokens = read("laiyipao/admin/src/styles/tokens.css")
const enemiesStart = wiki.indexOf("activeTab === 'enemies'")
const enemiesEnd = wiki.indexOf("<!-- 技能 -->")
const enemyTable = wiki.slice(enemiesStart, enemiesEnd)
const typeStart = wiki.indexOf("enemies: Array<{")
const enemyType = wiki.slice(typeStart, wiki.indexOf("skills: Array<{", typeStart))

describe("R33 enemy mechanics text contract", () => {
  it("speedText 定点转像素每秒，零与非法走空态", () => {
    expect(speedText(40000)).toBe("40像素/秒")
    expect(speedText(75000)).toBe("75像素/秒")
    expect(speedText(42500)).toBe("42.5像素/秒")
    expect(speedText(0)).toBe(MECHANISM_EMPTY)
    expect(speedText(-1)).toBe(MECHANISM_EMPTY)
    expect(speedText(Number.NaN)).toBe(MECHANISM_EMPTY)
    expect(speedText(Number.POSITIVE_INFINITY)).toBe(MECHANISM_EMPTY)
    expect(speedText("40000")).toBe(MECHANISM_EMPTY)
    expect(speedText(null)).toBe(MECHANISM_EMPTY)
  })

  it("armorText 千分比不冒充百分比，零与非整数归空态", () => {
    expect(armorText(50)).toBe("50‰")
    expect(armorText(250)).toBe("250‰")
    expect(armorText(350)).toBe("350‰")
    expect(armorText(0)).toBe(MECHANISM_EMPTY)
    expect(armorText(1.5)).toBe(MECHANISM_EMPTY)
    expect(armorText(-2)).toBe(MECHANISM_EMPTY)
    expect(armorText(Number.NaN)).toBe(MECHANISM_EMPTY)
    expect(armorText(Number.POSITIVE_INFINITY)).toBe(MECHANISM_EMPTY)
    expect(armorText("50")).toBe(MECHANISM_EMPTY)
  })

  it("shield/attack/range/flyHeight 原值整数且 0 归空态", () => {
    expect(shieldHpText(60)).toBe("60")
    expect(shieldHpText(3000)).toBe("3000")
    expect(shieldHpText(0)).toBe(MECHANISM_EMPTY)
    expect(shieldHpText(1.5)).toBe(MECHANISM_EMPTY)
    expect(shieldHpText(Number.NaN)).toBe(MECHANISM_EMPTY)
    expect(shieldHpText(undefined)).toBe(MECHANISM_EMPTY)
    expect(attackText(8)).toBe("8")
    expect(attackText(35)).toBe("35")
    expect(attackText(0)).toBe(MECHANISM_EMPTY)
    expect(attackText(-1)).toBe(MECHANISM_EMPTY)
    expect(attackText("8")).toBe(MECHANISM_EMPTY)
    expect(attackRangeText(150)).toBe("150")
    expect(attackRangeText(500)).toBe("500")
    expect(attackRangeText(0)).toBe(MECHANISM_EMPTY)
    expect(attackRangeText(1.5)).toBe(MECHANISM_EMPTY)
    expect(attackRangeText({})).toBe(MECHANISM_EMPTY)
    expect(flyHeightText(180)).toBe("180")
    expect(flyHeightText(300)).toBe("300")
    expect(flyHeightText(0)).toBe(MECHANISM_EMPTY)
    expect(flyHeightText(Number.POSITIVE_INFINITY)).toBe(MECHANISM_EMPTY)
  })

  it("attackIntervalText 毫秒转秒且 0/非法不产 NaN秒", () => {
    expect(attackIntervalText(2000)).toBe("2秒")
    expect(attackIntervalText(2500)).toBe("2.5秒")
    expect(attackIntervalText(3200)).toBe("3.2秒")
    expect(attackIntervalText(4000)).toBe("4秒")
    expect(attackIntervalText(0)).toBe(MECHANISM_EMPTY)
    expect(attackIntervalText(-1)).toBe(MECHANISM_EMPTY)
    expect(attackIntervalText(Number.NaN)).toBe(MECHANISM_EMPTY)
    expect(attackIntervalText(Number.POSITIVE_INFINITY)).toBe(MECHANISM_EMPTY)
    expect(attackIntervalText("2000")).toBe(MECHANISM_EMPTY)
    expect(attackIntervalText(null)).toBe(MECHANISM_EMPTY)
  })

  it("boolText 严格布尔：true 是，false/非法均为空态", () => {
    expect(boolText(true)).toBe("是")
    expect(boolText(false)).toBe(MECHANISM_EMPTY)
    expect(boolText(1)).toBe(MECHANISM_EMPTY)
    expect(boolText("true")).toBe(MECHANISM_EMPTY)
    expect(boolText(null)).toBe(MECHANISM_EMPTY)
    expect(boolText(undefined)).toBe(MECHANISM_EMPTY)
    expect(boolText({})).toBe(MECHANISM_EMPTY)
    expect(MECHANISM_EMPTY).toBe("—")
  })

  it("AI 聚合由 cfg.enemies 确定性推导且空态诚实", () => {
    const rows = [
      { is_boss: true, category: "boss", burrow: false, armor: 250 },
      { is_boss: false, category: "flying", burrow: false, armor: 0 },
      { is_boss: false, category: "special", burrow: true, armor: 100 },
      { is_boss: false, category: "normal", burrow: false, armor: 50 },
    ]
    expect(buildEnemyMechanicsAi(rows)).toBe("共 4 种敌人：首领 1 种，飞行 1 种，潜地 1 种，带护甲 3 种")
    expect(buildEnemyMechanicsAi([])).toBe("共 0 种敌人：首领 0 种，飞行 0 种，潜地 0 种，带护甲 0 种")
    expect(ENEMY_AI_CTA).toBe("查看敌人列表 →")
    expect(spec).toContain("共 {total} 种敌人：首领 {boss} 种，飞行 {fly} 种，潜地 {burrow} 种，带护甲 {armor} 种")
    expect(spec).toContain("category==='flying'")
  })

  it("SPEC 钉死单位口径、零值文案与非法值边界", () => {
    for (const s of ["定点 x1000", "像素/秒", "‰", "千分比", "3.2秒", "40像素/秒", "MECHANISM_EMPTY = '—'", "Number.isInteger", "Number.isFinite", "unknown", "1.5", "NaN秒", "0 是真实配置值"])
      expect(spec).toContain(s)
    expect(spec).toContain("不改 engine.ts / 后端 / replay / hash / score / settle / loot / tokens")
  })

  it("权威结构与双端引擎消费点未回退", () => {
    for (const s of ["json:\"speed\"", "json:\"armor\"", "json:\"shield_hp\"", "json:\"attack\"", "json:\"attack_range\"", "json:\"attack_interval\"", "json:\"fly_height\"", "json:\"burrow\"", "json:\"is_boss\""])
      expect(goSrc).toContain(s)
    for (const s of ["ArmorPermille int64", "FlyHeight     int64", "Burrow        bool", "IsBoss        bool", "Speed          int64", "AttackRange    int64"])
      expect(goEngine).toContain(s)
    for (const s of ["Shield:          def.ShieldHP", "ArmorPermille:   def.Armor", "scaledMs(def.AttackEvery", "Burrow:          def.Burrow"])
      expect(goWave).toContain(s)
    for (const s of ["toFixed(28 + def.fly_height * 0.1)", "BigInt(def.armor || 0)", "toFixed(def.attack_range)", "Math.max(1, Math.round((def.attack_interval * this.tuning.enemyAttackIntervalPermille) / 1000))"])
      expect(engine).toContain(s)
    for (const s of ["speed: number", "shield_hp: number", "attack_interval: number", "fly_height: number", "is_boss: boolean"])
      expect(tsTypes).toContain(s)
  })

  it("Wiki 敌人表真实导入调用九个格式化函数与 AI 聚合，列名与类型字段齐全", () => {
    expect(wiki).toContain("import { speedText, armorText, shieldHpText, attackText, attackRangeText, attackIntervalText, flyHeightText, boolText, MECHANISM_EMPTY, buildEnemyMechanicsAi, ENEMY_AI_CTA }")
    for (const name of ["speedText", "armorText", "shieldHpText", "attackText", "attackRangeText", "attackIntervalText", "flyHeightText", "boolText", "buildEnemyMechanicsAi"])
      expect(enemyTable).toMatch(new RegExp("\\b" + name + "\\s*\\("))
    for (const col of ["label=\"速度\"", "label=\"护甲\"", "label=\"护盾\"", "label=\"攻击\"", "label=\"射程\"", "label=\"攻击间隔\"", "label=\"飞行高度\"", "label=\"潜地\"", "label=\"首领\""])
      expect(enemyTable).toContain(col)
    for (const col of ["label=\"ID\"", "label=\"名称\"", "label=\"类型\"", "label=\"血量\"", "label=\"五维抗性\"", "label=\"说明\""])
      expect(enemyTable).toContain(col)
    for (const f of ["speed: number", "armor: number", "shield_hp: number", "attack: number", "attack_range: number", "attack_interval: number", "fly_height: number", "burrow: boolean", "is_boss: boolean"])
      expect(enemyType).toContain(f)
    expect(enemyTable).toContain("rowFocusClass('enemy')")
    expect(enemyTable).toContain("r33-enemy-mechanics")
    expect(enemyTable).toContain("r33-enemy-table-scroll")
  })

  it("敌人表表内滚动 CSS 与 AI 四要素同屏，CTA 走 hash 安全导航", () => {
    const cssStart = wiki.indexOf(".r33-enemy-table-scroll")
    const css = wiki.slice(cssStart, cssStart + 200)
    expect(css).toContain("overflow-x: auto")
    expect(css).toContain("max-width: 100%")
    const aiBlock = enemyTable.slice(enemyTable.indexOf("r33-ai-note"))
    expect(aiBlock).toContain("AI")
    expect(wiki).toContain("<strong>AI 机制概览：</strong>{{ buildEnemyMechanicsAi(cfg?.enemies ?? []) }}")
    expect(aiBlock).toContain("依据：公开 /api/v1/config 的 enemies（与 H5/Go 引擎同源 SeedEnemy）；展示为配置基值，实际效果以引擎结算为准")
    expect(aiBlock).toContain("查看敌人列表 →")
    expect(aiBlock).toContain("urlFor('enemies')")
    expect(aiBlock).toContain("goTo({ tab: 'enemies' })")
    expect(aiBlock).not.toContain("href=\"#enemies\"")
    expect(aiBlock).toContain("辅助展示非权威战斗数据，实际效果以引擎结算为准；权威回放以 replay_hash 为准")
    for (const ph of ["{total}", "{boss}", "{fly}", "{burrow}", "{armor}"]) expect(aiBlock).not.toContain(ph)
  })

  it("token 三锚点保留且禁 --r33-", () => {
    for (const anchor of ["--lyp-token-version: lyp-tokens-v1", "--lyp-r3-version: lyp-tokens-v3-r3", "--lyp-r6-version: lyp-tokens-r6"])
      expect(tokens).toContain(anchor)
    expect(spec).toContain("禁止新增 --r33-* 令牌")
    expect(tokens).not.toContain("--r33-")
  })
})

describe("R33 零真值审计脚本安全守卫（t263：扫描器不得排除自身）", () => {
  const SCAN = "docs/laiyipao/audits/round33-wiki-enemy-mechanics/zero-secret-scan.sh"
  it("审计脚本不含真实口令/个人路径字面，通用正则天然不自匹配且不得排除扫描器自身", () => {
    const script = read(SCAN)
    expect(script).not.toContain("admin12345")
    expect(script).not.toMatch(/[A-Za-z]:\/WORKSPACE/)
    expect(script).not.toMatch(/C:\/Users/)
    expect(script).not.toMatch(/\/home\/[a-z]+\//)
    expect(script).toMatch(/\[A-Za-z0-9_-\]/)
    expect(script).not.toContain("--exclude=zero-secret-scan.sh")
  })
  it("审计脚本自身纳入扫描且实跑 BAD_COUNT=0", () => {
    const out = execFileSync("bash", [resolve(REPO, SCAN)], { encoding: "utf-8" })
    expect(out).toContain("BAD_COUNT=0")
    expect(out).not.toContain("BAD_FOUND:")
  })
  it("审计脚本 bash -n 语法检查通过", () => {
    expect(() => execFileSync("bash", ["-n", resolve(REPO, SCAN)])).not.toThrow()
  })
})
