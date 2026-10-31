// @vitest-environment node
/**
 * 「战斗结算后查看本局战报」功能守卫（2026-11-01 新一轮第 43 轮）。
 *
 * 结算响应带 `battle_id`，但此前没有任何消费 —— 玩家打完只能看到
 * 结算面板上的数字，想回顾掉落/元素/反应/地形没有入口。本轮打通：
 *   - battle.vue 结算面板展示「查看本局战报」按钮，跳转战报页并携带 battle_id
 *   - battles.vue 读取 `?battle_id=` 深链，加载列表后自动展开该局并拉取详情
 *
 * 该功能消失时编译照过、现有测试全绿（少一个区块没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const BATTLE_VUE = join(__dirname, '../pages/battle/battle.vue')
const BATTLES_VUE = join(__dirname, '../pages/battles/battles.vue')

describe('战斗结算后查看本局战报守卫', () => {
  it('battle.vue 结算面板必须提供「查看本局战报」入口并携带 battle_id 跳转', () => {
    const src = readFileSync(BATTLE_VUE, 'utf-8')
    expect(src).toContain('查看本局战报')
    expect(src).toContain('settleResult.value?.battle_id')
    expect(src).toContain("`/pages/battles/battles?battle_id=${id}`")
  })

  it('battles.vue 必须解析 battle_id 深链并自动展开对应战报', () => {
    const src = readFileSync(BATTLES_VUE, 'utf-8')
    expect(src).toContain('query?.battle_id')
    expect(src).toContain('focusBattleId.value = id')
    expect(src).toContain('expandedId.value = target.id')
  })
})