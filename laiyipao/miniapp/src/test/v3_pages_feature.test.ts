// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * 第七批「实质优化 100 轮」其它页面功能守卫。
 * 只认完整功能（雷达图 / 矩阵 / 对比 / 图表），不认 hover/描边/计数角标式小打磨。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const read = (rel: string) => readFileSync(join(__dirname, rel), 'utf-8')

describe('第七批 其它页面实质功能守卫', () => {
  it('G61 首页构筑五维雷达图', () => {
    const src = read('../pages/index/index.vue')
    expect(src).toContain('radar-svg')
    expect(src).toContain('radarPolygonActual')
    expect(src).toContain('radar-ring')
    expect(src).toContain('radarAxisEnds')
  })

  it('G73 敌人抗性雷达图', () => {
    const src = read('../pages/enemies/enemies.vue')
    expect(src).toContain('enemyResistPoints')
    expect(src).toContain('resistScale')
    expect(src).toContain('expandedEnemy')
    expect(src).toContain('rr-shape')
  })

  it('G74 元素反应覆盖矩阵（5×5）', () => {
    const src = read('../pages/reactions/reactions.vue')
    expect(src).toContain('reactionMatrix')
    expect(src).toContain('lookupReaction')
    expect(src).toContain('matrix-cell')
    expect(src).toContain('元素反应覆盖矩阵')
  })

  it('G70 战绩趋势折线', () => {
    const src = read('../pages/battles/battles.vue')
    expect(src).toContain('const trend = computed')
    expect(src).toContain('trend-line')
    expect(src).toContain('trend-win')
    expect(src).toContain('战绩趋势')
  })
})
