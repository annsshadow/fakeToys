import { describe, it, expect } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import {
  DEFAULT_SKILL_RULES,
  skillRulesFromServer,
  clampLevel,
  coefPermilleAt,
  skillBaseDamageAtLevel,
  upgradeCostFrom,
  isMaxLevel,
  type SkillRules,
} from './skill'

/**
 * 技能升级的守卫。
 *
 * ## 这个功能此前的形状
 *
 * `user_skills.level` 是一列**永远为 1** 的数据：
 * 被读进 build 快照、从无写入、没有升级端点。
 * 而服务端 `HitInput.SkillDamage` 的注释声称伤害公式含「等级系数」。
 *
 * 接线要三件事同时成立：升级路径（已做）、等级进公式（本文件）、
 * 两端一致（replay.ts 是唯一构造点）。少一件就是纯装饰。
 */

// ── 跨端一致性 ────────────────────────────────────────────────

describe('TestSkillRulesMatchServerContract', () => {
  it('DEFAULT_SKILL_RULES 与服务端 skill_rules 逐字段一致', () => {
    const server = fixture.skill_rules
    const d = DEFAULT_SKILL_RULES
    expect(server.max_level).toBe(d.maxLevel)
    expect(server.coef_permille).toBe(d.coefPermille)
    expect(server.base_cost).toBe(d.baseCost)
  })

  it('skillRulesFromServer 解析后与默认值全等', () => {
    const parsed = skillRulesFromServer(fixture.skill_rules)
    expect(parsed).toEqual(DEFAULT_SKILL_RULES)
  })
})

// ── 系数 ────────────────────────────────────────────────────

describe('coefPermilleAt', () => {
  it('等级 1 精确等于 1000‰（无加成）', () => {
    expect(coefPermilleAt(DEFAULT_SKILL_RULES, 1)).toBe(1000)
  })

  it('每级 +coef_permille‰，满级 1450‰', () => {
    expect(coefPermilleAt(DEFAULT_SKILL_RULES, 2)).toBe(1050)
    expect(coefPermilleAt(DEFAULT_SKILL_RULES, 5)).toBe(1200)
    expect(coefPermilleAt(DEFAULT_SKILL_RULES, 10)).toBe(1450)
  })

  it('越界输入被夹紧（脏数据不该变成伤害）', () => {
    expect(coefPermilleAt(DEFAULT_SKILL_RULES, 0)).toBe(1000)
    expect(coefPermilleAt(DEFAULT_SKILL_RULES, -5)).toBe(1000)
    expect(coefPermilleAt(DEFAULT_SKILL_RULES, 99)).toBe(1450)
    expect(coefPermilleAt(DEFAULT_SKILL_RULES, 1e9)).toBe(1450)
    expect(coefPermilleAt(DEFAULT_SKILL_RULES, NaN)).toBe(1000)
  })

  it('夹紧后满级加成只有 +45% —— 不喧宾夺主', () => {
    // 理由写在 domain/skillrules.go：技能伤害与面板攻击同乘区，
    // 提高它与提高攻击力一样压制反应（实测 3000‰ 时反应掉 25%）。
    // 满级 1450‰ 里的 450‰ 只有攻击封顶（1000‰）的一半。
    expect(coefPermilleAt(DEFAULT_SKILL_RULES, 10) - 1000).toBe(450)
    expect(coefPermilleAt(DEFAULT_SKILL_RULES, 10) - 1000).toBeLessThan(1000)
  })
})

describe('clampLevel', () => {
  it('夹到 [1, maxLevel]', () => {
    expect(clampLevel(DEFAULT_SKILL_RULES, -3)).toBe(1)
    expect(clampLevel(DEFAULT_SKILL_RULES, 0)).toBe(1)
    expect(clampLevel(DEFAULT_SKILL_RULES, 1)).toBe(1)
    expect(clampLevel(DEFAULT_SKILL_RULES, 7)).toBe(7)
    expect(clampLevel(DEFAULT_SKILL_RULES, 10)).toBe(10)
    expect(clampLevel(DEFAULT_SKILL_RULES, 11)).toBe(10)
  })
})

// ── 伤害缩放：这里的「精确恒等」是兼容性铁律 ──────────────────

describe('skillBaseDamageAtLevel', () => {
  it('等级 1 是**精确**恒等变换（不是近似）', () => {
    // 为什么是铁律而不是巧合：
    // 升级功能上线前所有 level 恒为 1，所有**存量战报**的
    // replayHash 必须逐位不变。replayHash 的锚点里含 baseDamage，
    // 所以只要 level 1 不是精确恒等，历史战报会静默全部失配 ——
    // 而失配表现为「验真失败」，看起来像作弊，
    // 实际是上线了一个数学常数。
    for (const base of [1n, 7n, 100n, 12345n, 999999n, 10n ** 15n]) {
      expect(skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, base, 1)).toBe(base)
    }
  })

  it('缺失等级（undefined）按 1 处理，伤害逐位不变', () => {
    // build.skills[].level 在旧快照里可能不存在。
    // 走 undefined 路径必须与 level=1 完全一致。
    const base = 54321n
    expect(skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, base, undefined as unknown as number)).toBe(
      base,
    )
  })

  it('等级越高伤害越高，且严格单调', () => {
    const base = 1000n
    let prev = 0n
    for (let lv = 1; lv <= 10; lv++) {
      const dmg = skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, base, lv)
      expect(dmg).toBeGreaterThan(prev)
      prev = dmg
    }
  })

  it('满级 10 = +45%（精确整除，不丢精度）', () => {
    // 1000 * 1450 / 1000 = 1450
    expect(skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, 1000n, 10)).toBe(1450n)
    // 50000 * 1450 / 1000 = 72500
    expect(skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, 50000n, 10)).toBe(72500n)
  })

  it('先乘后除，不因整除丢精度', () => {
    // 若实现成 base/1000*coef，base=1500 时会得到 1500/1000=1 → 1*1450=1450，
    // 而正确值是 2175。两者差 50%，且在低伤害技能上更离谱。
    const base = 1500n
    const correct = skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, base, 10)
    expect(correct).toBe((base * 1450n) / 1000n)
    // 明确断言它**不等于**先除后乘的错解
    expect(correct).not.toBe((base / 1000n) * 1450n)
  })

  it('越界等级不产生超额伤害', () => {
    const base = 1000n
    const atMax = skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, base, 10)
    expect(skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, base, 99)).toBe(atMax)
    expect(skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, base, 1000000)).toBe(atMax)
  })
})

// ── 费用 ────────────────────────────────────────────────────

describe('upgradeCostFrom', () => {
  it('线性递增：baseCost × 当前等级', () => {
    expect(upgradeCostFrom(DEFAULT_SKILL_RULES, 1)).toBe(100)
    expect(upgradeCostFrom(DEFAULT_SKILL_RULES, 2)).toBe(200)
    expect(upgradeCostFrom(DEFAULT_SKILL_RULES, 9)).toBe(900)
  })

  it('满级返回 0（而不是抛错或返回一个魔数）', () => {
    expect(upgradeCostFrom(DEFAULT_SKILL_RULES, 10)).toBe(0)
    expect(upgradeCostFrom(DEFAULT_SKILL_RULES, 11)).toBe(0)
  })

  it('越界低等级按 1 计费，不会因为 level=0 而免费', () => {
    // 「越界低等级免费」是个真实的漏洞形状：
    // 若 level 来自脏数据且某分支写了 `if (level <= 0) return 0`，
    // 玩家就能白升级。夹紧到 1 后按 100 计费。
    expect(upgradeCostFrom(DEFAULT_SKILL_RULES, 0)).toBe(100)
    expect(upgradeCostFrom(DEFAULT_SKILL_RULES, -5)).toBe(100)
  })

  it('满级单个技能总花费 4500（1+2+…+9 = 45 × 100）', () => {
    let total = 0
    for (let lv = 1; lv < 10; lv++) total += upgradeCostFrom(DEFAULT_SKILL_RULES, lv)
    expect(total).toBe(4500)
  })
})

describe('isMaxLevel', () => {
  it('等级达到上限即为满级', () => {
    expect(isMaxLevel(DEFAULT_SKILL_RULES, 9)).toBe(false)
    expect(isMaxLevel(DEFAULT_SKILL_RULES, 10)).toBe(true)
    expect(isMaxLevel(DEFAULT_SKILL_RULES, 99)).toBe(true)
    expect(isMaxLevel(DEFAULT_SKILL_RULES, 0)).toBe(false)
  })
})

// ── 变更规则的守卫 ────────────────────────────────────────────

describe('规则变更时这些断言必须先被重新审视', () => {
  it('改 coefPermille 会同时改动伤害与费用之外的量 —— 需重跑平衡', () => {
    // 不是断言「平衡」，是提醒：这个数一动，
    // tier_impact.test.ts / balance.probe.test.ts 的结论就过期了。
    // 把它们绑在一起，避免「改了常数、忘了重测」。
    const custom: SkillRules = { ...DEFAULT_SKILL_RULES, coefPermille: 200 }
    expect(coefPermilleAt(custom, 10)).toBe(2800)
    // 2800‰ 远超攻击封顶 1000‰，会把反应压到几乎不触发
    // —— 所以这不是一个能随手填的数。
    expect(coefPermilleAt(custom, 10)).toBeGreaterThan(1000 + 1000)
  })
})
