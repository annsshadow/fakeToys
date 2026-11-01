// @vitest-environment node
/**
 * 「商店购买二次确认」功能守卫（2026-11-01 新一轮第 91 轮）。
 *
 * 商店购买会消耗货币（金币/钻石），误点会直接扣款。
 * 本轮新增：
 *   - shop.vue doBuy 先弹 uni.showModal 确认
 *   - 确认后才发起购买请求
 *
 * 该功能消失时编译照过、现有测试全绿（少一次确认没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const SHOP_VUE = join(__dirname, '../pages/shop/shop.vue')

describe('商店购买二次确认守卫', () => {
  it('shop.vue 购买前必须弹确认框（uni.showModal）', () => {
    const src = readFileSync(SHOP_VUE, 'utf-8')
    expect(src).toContain('确认购买')
    expect(src).toContain('uni.showModal')
    expect(src).toContain('将花费')
  })

  it('确认后才继续购买流程', () => {
    const src = readFileSync(SHOP_VUE, 'utf-8')
    expect(src).toContain('if (!confirmed) return')
  })
})