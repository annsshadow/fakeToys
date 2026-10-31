// @vitest-environment node
/**
 * 「背包技能升级收益预览」功能守卫（2026-11-01 新一轮第 61 轮）。
 *
 * 背包页只显示当前等级伤害与升级费用 —— 玩家看不到升级后伤害提升多少，
 * 「花金币值不值」只能先点了升级再看。本轮新增：
 *   - bag.vue 用 skillBaseDamageAtLevel 计算等级 +1 的伤害预览
 *   - 未满级技能行显示「伤害 X → Y」
 *
 * 该功能消失时编译照过、现有测试全绿（少一行没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const BAG_VUE = join(__dirname, '../pages/bag/bag.vue')

describe('背包技能升级收益预览守卫', () => {
  it('bag.vue 必须用 skillBaseDamageAtLevel 计算升级后伤害', () => {
    const src = readFileSync(BAG_VUE, 'utf-8')
    expect(src).toContain('function nextDamageAt(s: SkillDef): number')
    expect(src).toContain('skillBaseDamageAtLevel')
    expect(src).toContain('levelOf(s.id) + 1')
  })

  it('未满级技能行必须显示「伤害 X → Y」', () => {
    const src = readFileSync(BAG_VUE, 'utf-8')
    expect(src).toContain('→ {{ nextDamageAt(s) }}')
    expect(src).toContain('!isMaxLevel(s.id)')
  })
})