// @vitest-environment node
/**
 * 「我的页面累计星级」功能守卫（2026-11-01 新一轮第 64 轮）。
 *
 * 选关页有章节星级进度，但全局没有汇总 —— 玩家在「我的」页看不到
 * 总共拿了多少星。本轮新增：
 *   - me.vue 用 fetchMyStars 汇总全部关卡星级
 *   - 账号卡显示「累计星级 x / y」（y = 关卡总数 × 3）
 *   - 与其它我的页数据并行加载
 *
 * 该功能消失时编译照过、现有测试全绿（少一行没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const ME_VUE = join(__dirname, '../pages/me/me.vue')

describe('我的页面累计星级守卫', () => {
  it('me.vue 必须调用 fetchMyStars 并汇总星级', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain('api.fetchMyStars()')
    expect(src).toContain('Object.values(r.stars).reduce')
  })

  it('必须显示「累计星级 x / y」，上限 = 关卡数 × 3', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain('累计星级 {{ totalStars }} / {{ totalStarCap }}')
    expect(src).toContain('store.config?.levels?.length')
  })

  it('累计星级必须与其它我的页数据并行加载', () => {
    const src = readFileSync(ME_VUE, 'utf-8')
    expect(src).toContain('loadStars(),')
    expect(src).toContain('await Promise.all([')
  })
})