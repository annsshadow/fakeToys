// @vitest-environment node
/**
 * 「战报详情直达验真」功能守卫（2026-11-01 新一轮第 47 轮）。
 *
 * 验真页支持 `?battle_id=` 深链，但战报页没有任何入口 ——
 * 玩家想验真一局，必须先记下 battle_id 再手动切到验真页输入。
 * 本轮打通：
 *   - battles.vue 战报详情新增「去验真此局」按钮，跳转验真页并携带 battle_id
 *   - verify.vue 深链增强：携带合法 battle_id 时自动拉取复现信息
 *
 * 该功能消失时编译照过、现有测试全绿（少一个区块没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const BATTLES_VUE = join(__dirname, '../pages/battles/battles.vue')
const VERIFY_VUE = join(__dirname, '../pages/verify/verify.vue')

describe('战报详情直达验真守卫', () => {
  it('battles.vue 战报详情必须提供「去验真此局」入口并携带 battle_id 跳转', () => {
    const src = readFileSync(BATTLES_VUE, 'utf-8')
    expect(src).toContain('去验真此局')
    expect(src).toContain("`/pages/verify/verify?battle_id=${battleId}`")
  })

  it('verify.vue 深链必须自动填充并自动拉取复现信息', () => {
    const src = readFileSync(VERIFY_VUE, 'utf-8')
    expect(src).toContain('battleId.value = String(bid)')
    expect(src).toContain('Number.isInteger(id) && id > 0')
    expect(src).toContain('void fetchReplay()')
  })
})