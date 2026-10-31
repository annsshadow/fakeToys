// @vitest-environment node
/**
 * 「反应图鉴搜索」功能守卫（2026-11-01 新一轮第 77 轮）。
 *
 * 反应图鉴有多种反应链，没有搜索时想找一种反应只能滚动翻找。
 * 本轮新增：
 *   - reactions.vue 顶部搜索框
 *   - 按反应名过滤
 *
 * 该功能消失时编译照过、现有测试全绿（少一个输入框没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const REACTIONS_VUE = join(__dirname, '../pages/reactions/reactions.vue')

describe('反应图鉴搜索守卫', () => {
  it('reactions.vue 必须提供搜索输入框', () => {
    const src = readFileSync(REACTIONS_VUE, 'utf-8')
    expect(src).toContain('placeholder="搜索反应名称"')
    expect(src).toContain('v-model="keyword"')
  })

  it('必须按反应名过滤', () => {
    const src = readFileSync(REACTIONS_VUE, 'utf-8')
    expect(src).toContain('filteredReactions')
    expect(src).toContain('r.name.toLowerCase().includes(kw)')
  })
})