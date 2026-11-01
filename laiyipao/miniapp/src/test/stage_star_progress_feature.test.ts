// @vitest-environment node
/**
 * 「选关页章节星级进度」功能守卫（2026-11-01 新一轮第 52 轮）。
 *
 * 选关页每个关卡都显示星级，但没有汇总 —— 玩家看不出本章节
 * 一共拿了多少星、离全清还差多少。本轮新增：
 *   - stage.vue 用 starOf 汇总当前章节已获星星（服务端 level_stars 口径）
 *   - 显示「本章节星级 x / y」（y = 关卡数 × 3）
 *
 * 该功能消失时编译照过、现有测试全绿（少一行没有报错），
 * 所以用源码扫描把它变成结构约束。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const STAGE_VUE = join(__dirname, '../pages/stage/stage.vue')

describe('选关页章节星级进度守卫', () => {
  it('stage.vue 必须按 starOf 汇总当前章节星级', () => {
    const src = readFileSync(STAGE_VUE, 'utf-8')
    expect(src).toContain('chapterStars')
    expect(src).toContain('starOf(lv.id)')
    expect(src).toContain('chapterLevels.value.length * 3')
  })

  it('模板必须渲染「本章节星级 x / y」', () => {
    const src = readFileSync(STAGE_VUE, 'utf-8')
    expect(src).toContain('本章节星级')
    expect(src).toContain('{{ chapterStars }} / {{ chapterTotal }}')
  })
})