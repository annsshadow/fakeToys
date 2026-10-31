// @vitest-environment node
/**
 * 「宝石图鉴搜索」功能守卫（2026-11-01 新一轮第 75 轮）。
 *
 * 宝石图鉴有 8 种宝石（不同属性），没有搜索时想找一种宝石
 * 只能滚动翻找。本轮新增：
 *   - gems.vue 顶部搜索框
 *   - 按名称 / 属性中文过滤
 *
 * 该功能消失时编译照过、现有测试全绿（少一个输入框没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const GEMS_VUE = join(__dirname, '../pages/gems/gems.vue')

describe('宝石图鉴搜索守卫', () => {
  it('gems.vue 必须提供搜索输入框', () => {
    const src = readFileSync(GEMS_VUE, 'utf-8')
    expect(src).toContain('placeholder="搜索宝石名称 / 属性"')
    expect(src).toContain('v-model="keyword"')
  })

  it('必须按名称/属性中文过滤', () => {
    const src = readFileSync(GEMS_VUE, 'utf-8')
    expect(src).toContain('filteredGems')
    expect(src).toContain('g.name.toLowerCase().includes(kw)')
    expect(src).toContain('attrLabel(g.attr).toLowerCase().includes(kw)')
  })
})