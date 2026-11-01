// @vitest-environment node
/**
 * 「敌人图鉴搜索」功能守卫（2026-11-01 新一轮第 70 轮）。
 *
 * 敌人图鉴有几十种敌人（普通/远程/飞行/精英/Boss），没有搜索时
 * 想找一种敌人只能滚动翻找。本轮新增：
 *   - enemies.vue 顶部搜索框
 *   - 按名称 / 类别中文 / 原始类别 key 过滤
 *
 * 该功能消失时编译照过、现有测试全绿（少一个输入框没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const ENEMIES_VUE = join(__dirname, '../pages/enemies/enemies.vue')

describe('敌人图鉴搜索守卫', () => {
  it('enemies.vue 必须提供搜索输入框', () => {
    const src = readFileSync(ENEMIES_VUE, 'utf-8')
    expect(src).toContain('placeholder="搜索敌人名称 / 类别（如 Boss、飞行）"')
    expect(src).toContain('v-model="keyword"')
  })

  it('必须按名称/类别中文/原始类别过滤', () => {
    const src = readFileSync(ENEMIES_VUE, 'utf-8')
    expect(src).toContain('filteredEnemies')
    expect(src).toContain('en.name.toLowerCase().includes(kw)')
    expect(src).toContain('categoryLabel(en.category).toLowerCase().includes(kw)')
    expect(src).toContain('en.category.toLowerCase().includes(kw)')
  })
})