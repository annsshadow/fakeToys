// @vitest-environment node
/**
 * 「商城每日限购进度」功能守卫（2026-11-01 新一轮第 49 轮）。
 *
 * 商城商品有 `limit_per_day` / `bought_today` 字段，但此前只展示
 * 「今日已买满」/「购买」两种状态 —— 玩家看不到还剩几次购买机会，
 * 限购商品只能靠试错。本轮新增：
 *   - shop.vue 在限购商品行显示「今日已购 x/y」进度文案
 *   - 买满仍显示「今日已买满」；不限购（limit_per_day=0）不显示
 *
 * 该功能消失时编译照过、现有测试全绿（少一行文字没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const SHOP_VUE = join(__dirname, '../pages/shop/shop.vue')

describe('商城每日限购进度守卫', () => {
  it('shop.vue 必须计算限购进度文案（已购 x/y）', () => {
    const src = readFileSync(SHOP_VUE, 'utf-8')
    expect(src).toContain('it.limit_per_day <= 0')
    expect(src).toContain('it.bought_today >= it.limit_per_day')
    expect(src).toContain('今日已购')
  })

  it('模板必须渲染限购进度文案', () => {
    const src = readFileSync(SHOP_VUE, 'utf-8')
    expect(src).toContain('purchaseProgressText(it)')
    expect(src).toContain('v-if="purchaseProgressText(it)"')
  })
})