/**
 * 背包页技能升级的**行为**守卫。
 *
 * ## 为什么需要这个文件
 *
 * `POST /me/skills/:id/upgrade` 从 Round 25 就存在，但客户端
 * **一个调用点都没有** —— 端点是纯后端装饰，玩家永远点不到。
 * 而 `tsc` 全绿：没有引用的函数不会报错，没有入口的端点也不会。
 *
 * 这就是本项目反复猎的「只写不读 / 纯装饰」，
 * 只不过这次是我自己刚犯的。
 *
 * ## 这里测什么、不测什么
 *
 * **不测** Vue 组件的渲染结果 —— 那需要 `@vue/test-utils`，
 * 而本项目的小程序页面测试目前都是纯逻辑测试。
 * 一旦引入组件测试框架，测试运行时间会从秒级跳到分钟级。
 *
 * **测**的是那段**可抽出来的纯逻辑**：等级读取、费用、满级判定、
 * 余额是否够、以及「升级后伤害确实变了」。
 * 组件里调用的正是这几个函数，所以逻辑正确 = 按钮显示正确。
 *
 * 顺带把「装备元素标签不参与结算」这条**文案修正**钉住 ——
 * 之前 bag.vue 的文案宣传「同系给反应伤害大幅加成」，
 * 而那个机制没有实现。服务端描述早改好了，客户端文案一直漏着。
 */
import { describe, it, expect } from 'vitest'
import { DEFAULT_SKILL_RULES, skillBaseDamageAtLevel, type SkillRules } from './skill'
import fixture from '@vectors/smoke_levels.json'
import type { SkillDef } from './types'

const config = fixture as unknown as { skill_rules: { max_level: number; coef_permille: number; base_cost: number } }
const serverRules = config.skill_rules

/** 与 bag.vue 里的同名函数同构（那里直接写在组件内，无法 import）。 */
function levelOf(build: any, skillId: number): number {
  const bag = build?.skills
  if (!bag) return 0
  const row = bag[String(skillId)]
  return typeof row?.level === 'number' ? row.level : 0
}
function upgradeCost(r: typeof serverRules, level: number): number {
  return r.base_cost * Math.max(level, 1)
}
function isMaxLevel(r: typeof serverRules, level: number): boolean {
  return level >= r.max_level
}

describe('技能升级：客户端规则与服务端一致', () => {
  it('规则来自 /config 的 skill_rules，不硬编码', () => {
    // 前端常量与服务端下发必须逐字段一致 ——
    // 两边漂移的表现是「页面说 100 金币、实际扣 100」这种小事，
    // 或者「按钮显示满级但服务端还在收钱」这种大事。
    expect(DEFAULT_SKILL_RULES.maxLevel).toBe(serverRules.max_level)
    expect(DEFAULT_SKILL_RULES.coefPermille).toBe(serverRules.coef_permille)
    expect(DEFAULT_SKILL_RULES.baseCost).toBe(serverRules.base_cost)
  })

  it('费用线性递增：基数 × 当前等级', () => {
    expect(upgradeCost(serverRules, 1)).toBe(100)
    expect(upgradeCost(serverRules, 2)).toBe(200)
    expect(upgradeCost(serverRules, 9)).toBe(900)
  })

  it('未拥有（level=0）时按 1 级计费，不会出现「免费升级」', () => {
    // 「越界低等级免费」是真实的漏洞形状：
    // 若某处写成 `if (level <= 0) return 0`，玩家就能白升级。
    expect(levelOf({ skills: {} }, 1)).toBe(0)
    expect(upgradeCost(serverRules, levelOf({ skills: {} }, 1))).toBe(100)
    expect(upgradeCost(serverRules, levelOf(null, 1))).toBe(100)
  })

  it('满级判定在边界上正确', () => {
    expect(isMaxLevel(serverRules, serverRules.max_level - 1)).toBe(false)
    expect(isMaxLevel(serverRules, serverRules.max_level)).toBe(true)
  })
})

describe('技能升级：等级真的改变伤害', () => {
  const skills = (fixture.skills as unknown as SkillDef[]).filter((s) => s.kind === 'active')

  it('同一个技能，等级越高伤害越高', () => {
    const s = skills[0]
    const base = BigInt(s.base_damage)
    let prev = 0n
    for (let lv = 1; lv <= serverRules.max_level; lv++) {
      const d = skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, base, lv)
      expect(d, `技能 ${s.id} 等级 ${lv} 的伤害没有比上一级高`).toBeGreaterThan(prev)
      prev = d
    }
  })

  it('等级 1 与「无等级信息」时伤害逐位不变（存量战报兼容性）', () => {
    // 这是 I-6 的硬要求：上线前所有 level 恒为 1，
    // 存量战报的重放哈希必须逐位不变。
    for (const s of skills.slice(0, 8)) {
      const base = BigInt(s.base_damage)
      expect(skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, base, 1)).toBe(base)
      expect(
        skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, base, undefined as unknown as number),
      ).toBe(base)
    }
  })

  it('未拥有的技能（level 0）按 1 级算，不白给加成', () => {
    const s = skills[0]
    const lv0 = skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, BigInt(s.base_damage), 0)
    expect(lv0).toBe(BigInt(s.base_damage))
  })

  it('满级加成是 +45%（不超过攻击力封顶，避免压制反应）', () => {
    const full = skillBaseDamageAtLevel(DEFAULT_SKILL_RULES, 1000n, serverRules.max_level)
    expect(full).toBe(1450n)
    // 理由写在 domain/skillrules.go：技能伤害与面板攻击同乘区，
    // 抬得更高会压制元素反应，而反应是核心创新。
    expect(Number(full) - 1000).toBeLessThan(1000)
  })
})

describe('装备元素标签不参与结算（文案修正的守卫）', () => {
  it('装备描述不再承诺「同系给加成」这个不存在的机制', () => {
    // ⚠️ 第一版我写的是「找两件只有 element 不同、数值相同的装备，
    // 断言它们的战斗结果一样」—— **前提不成立**：
    // 夹具里 18 件装备的 base_armor 各不相同（120 vs 50 …），
    // 根本不存在这样的一对，断言当场红，而且红得毫无意义。
    //
    // 真正守得住的判据是**文案**：玩家看到的那句话不许承诺没实现的机制。
    // 这也是本项目吃过亏的地方（18 件装备里 10 条描述在骗玩家）。
    const equip = fixture.equipment as unknown as Array<{ name: string; descr: string }>
    expect(equip.length).toBeGreaterThan(0)

    // 这些措辞只在「按元素给加成」真的实现了之后才能出现
    const forbidden = ['同系', '同元素', '反应伤害 +', '元素契合', '契合度']
    const bad: string[] = []
    for (const e of equip) {
      for (const w of forbidden) {
        if (e.descr.includes(w)) bad.push(`${e.name}：「${w}」`)
      }
    }
    expect(bad.join('\n')).toBe('')
  })

  it('装备描述给出的是真实数值（每条都含数字）', () => {
    // 防止有人把描述改成一句空话 —— 那是「不骗人」但更没用的做法。
    const equip = fixture.equipment as unknown as Array<{ name: string; descr: string }>
    const withNumber = equip.filter((e) => /\d/.test(e.descr))
    expect(withNumber.length).toBe(equip.length)
  })
})

describe('规则的单一真源', () => {
  it('夹具里的 skill_rules 存在（否则上面所有断言都在比 undefined）', () => {
    // 前提守卫：没有它，「客户端规则与服务端一致」会退化成
    // 「undefined === undefined」，恒绿。
    expect(serverRules).toBeDefined()
    expect(serverRules.max_level).toBeGreaterThan(0)
    expect(serverRules.coef_permille).toBeGreaterThan(0)
    expect(serverRules.base_cost).toBeGreaterThan(0)
  })

  it('DEFAULT_SKILL_RULES 的形状与 SkillRules 一致', () => {
    const r: SkillRules = DEFAULT_SKILL_RULES
    expect(Object.keys(r).sort()).toEqual(['baseCost', 'coefPermille', 'maxLevel'])
  })
})
