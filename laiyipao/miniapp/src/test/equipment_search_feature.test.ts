// @vitest-environment node
/**
 * 「装备图鉴搜索」功能守卫（2026-11-01 新一轮第 73 轮）。
 *
 * 装备图鉴有 18 件装备（武器/头盔/胸甲/手套/靴子/护符、多元素），
 * 没有搜索时想找一件装备只能滚动翻找。本轮新增：
 *   - equipment.vue 顶部搜索框
 *   - 按名称 / 部位中文 / 元素过滤
 *
 * 该功能消失时编译照过、现有测试全绿（少一个输入框没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const EQUIP_VUE = join(__dirname, '../pages/equipment/equipment.vue')

describe('装备图鉴搜索守卫', () => {
  it('equipment.vue 必须提供搜索输入框', () => {
    const src = readFileSync(EQUIP_VUE, 'utf-8')
    expect(src).toContain('placeholder="搜索装备名称 / 部位 / 元素"')
    expect(src).toContain('v-model="keyword"')
  })

  it('必须按名称/部位中文/元素过滤', () => {
    const src = readFileSync(EQUIP_VUE, 'utf-8')
    expect(src).toContain('filteredItems')
    expect(src).toContain('e.name.toLowerCase().includes(kw)')
    expect(src).toContain('slotLabel(e.slot).toLowerCase().includes(kw)')
    expect(src).toContain('elementName(e.element).toLowerCase().includes(kw)')
  })
})