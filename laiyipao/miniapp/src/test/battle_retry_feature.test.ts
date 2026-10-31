// @vitest-environment node
/**
 * 「战斗结算后再来一局」功能守卫（2026-11-01 新一轮第 53 轮）。
 *
 * 结算后玩家只能「返回关卡」再选一次同一关 —— 刷关/补星时
 * 每次都要重复两步操作。本轮新增：
 *   - battle.vue 结算面板「再来一局」按钮
 *   - 用 redirectTo 替换当前页（避免页面栈无限增长）
 *
 * 该功能消失时编译照过、现有测试全绿（少一个区块没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const BATTLE_VUE = join(__dirname, '../pages/battle/battle.vue')

describe('战斗结算后再来一局守卫', () => {
  it('battle.vue 结算面板必须提供「再来一局」按钮', () => {
    const src = readFileSync(BATTLE_VUE, 'utf-8')
    expect(src).toContain('再来一局')
    expect(src).toContain('retryLevel()')
  })

  it('再来一局必须用 redirectTo 重新进入同一关（不叠加页面栈）', () => {
    const src = readFileSync(BATTLE_VUE, 'utf-8')
    expect(src).toContain("uni.redirectTo({ url: `/pages/battle/battle?level=${levelId.value}` })")
  })
})