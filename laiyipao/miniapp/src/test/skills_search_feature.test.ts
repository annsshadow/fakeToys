// @vitest-environment node
/**
 * 「技能图鉴搜索」功能守卫（2026-11-01 新一轮第 71 轮）。
 *
 * 技能图鉴条目多（主动/被动、多元素），没有搜索时想找一种技能
 * 只能滚动翻找。本轮新增：
 *   - skills.vue 顶部搜索框
 *   - 按名称 / 元素中文 / 主动被动过滤
 *
 * 该功能消失时编译照过、现有测试全绿（少一个输入框没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const SKILLS_VUE = join(__dirname, '../pages/skills/skills.vue')

describe('技能图鉴搜索守卫', () => {
  it('skills.vue 必须提供搜索输入框', () => {
    const src = readFileSync(SKILLS_VUE, 'utf-8')
    expect(src).toContain('placeholder="搜索技能名称 / 元素 / 主动被动"')
    expect(src).toContain('v-model="keyword"')
  })

  it('必须按名称/元素中文/主动被动过滤', () => {
    const src = readFileSync(SKILLS_VUE, 'utf-8')
    expect(src).toContain('filteredSkills')
    expect(src).toContain('s.name.toLowerCase().includes(kw)')
    expect(src).toContain('elementName(s.element).toLowerCase().includes(kw)')
    expect(src).toContain("(s.kind === 'active' ? '主动' : '被动')")
  })
})