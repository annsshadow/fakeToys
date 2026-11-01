// @vitest-environment node
/**
 * 「皮肤图鉴搜索」功能守卫（2026-11-01 新一轮第 78 轮）。
 *
 * 皮肤图鉴有 6 款皮肤（稀有/史诗/传说），没有搜索时想找一款皮肤
 * 只能滚动翻找。本轮新增：
 *   - skins.vue 顶部搜索框
 *   - 按名称 / 稀有度中文过滤
 *
 * 该功能消失时编译照过、现有测试全绿（少一个输入框没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const SKINS_VUE = join(__dirname, '../pages/skins/skins.vue')

describe('皮肤图鉴搜索守卫', () => {
  it('skins.vue 必须提供搜索输入框', () => {
    const src = readFileSync(SKINS_VUE, 'utf-8')
    expect(src).toContain('placeholder="搜索皮肤名称 / 稀有度"')
    expect(src).toContain('v-model="keyword"')
  })

  it('必须按名称/稀有度中文过滤', () => {
    const src = readFileSync(SKINS_VUE, 'utf-8')
    expect(src).toContain('filteredSkins')
    expect(src).toContain('s.name.toLowerCase().includes(kw)')
    expect(src).toContain('rarityLabel(s.rarity).toLowerCase().includes(kw)')
  })
})