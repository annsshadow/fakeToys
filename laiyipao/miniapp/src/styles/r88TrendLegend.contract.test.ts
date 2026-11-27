// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R88 battles 战绩趋势图图例修正契约（test-first RED）。
 * 图上三种视觉编码：蓝实线 trend-line=分数(var(--ice),越高越好)、红虚线 leak-line=漏怪率(var(--bad),越低越好)、
 * 圆点胜负 trend-win=胜(var(--ok)) / trend-lose=负(var(--bad))。
 * 旧图例 ● 分数（蓝）　● 漏怪率（红）　越低越好 有三重错位：无胜负色点解释、实心圆点符号标注折线、
 * 越低越好字面误导分数(实际越高越好)。唯一改点=图例文字一行，须覆盖四条语义且不动三编码结构与颜色。
 * 先 RED 再实现；令牌锚点 v1/v3-r3/r6 不变，禁新增 r88 视觉令牌。
 * 本契约只读页面源码与令牌样式，不触碰受保护域。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const linesOf = (text: string) => text.split(String.fromCharCode(10))

describe('R88 battles 战绩趋势图图例修正契约', () => {
  const battles = readSrc('pages/battles/battles.vue')
  const tokens = readSrc('styles/tokens.css')
  const lines = linesOf(battles)
  const legendIdx = lines.findIndex(l => l.indexOf('class="trend-legend"') > -1)
  // 图例文字块：trend-legend 容器及其后 3 行（文案在容器内的 text 节点）
  const legendBlock = legendIdx > -1 ? lines.slice(legendIdx, legendIdx + 4).join(String.fromCharCode(10)) : ''

  it('图例解释胜负色点：绿=胜、红=负', () => {
    expect(legendIdx, 'battles 须有 trend-legend 图例容器').toBeGreaterThan(-1)
    expect(legendBlock, '图例须解释圆点胜负语义：含胜').toContain('胜')
    expect(legendBlock, '图例须解释圆点胜负语义：含负').toContain('负')
    expect(legendBlock, '图例胜负色点须含颜色词（绿/红）').toMatch(/绿|红/)
  })

  it('图例标注分数越高越好、漏怪率越低越好并就近绑定', () => {
    expect(legendBlock, '图例须标注分数越高越好').toContain('越高越好')
    expect(legendBlock, '图例须标注漏怪率越低越好').toContain('越低越好')
    expect(legendBlock, '分数须与越高越好就近绑定').toMatch(/分数[^\n]{0,12}越高越好/)
    expect(legendBlock, '漏怪率须与越低越好就近绑定').toMatch(/漏怪率[^\n]{0,14}越低越好/)
  })

  it('图例线型符号与折线对应，不再用实心圆点标注折线', () => {
    expect(legendBlock, '不得再用实心圆点标注分数折线').not.toMatch(/●\s*分数/)
    expect(legendBlock, '不得再用实心圆点标注漏怪率折线').not.toMatch(/●\s*漏怪率/)
    expect(legendBlock, '须用线型符号标注折线（实线/虚线）').toMatch(/━|┄|─|—/)
  })

  it('不变量：三种图形编码结构与胜负判定保留', () => {
    expect(battles, '蓝实线 trend-line 须保留').toContain('class="trend-line"')
    expect(battles, '红虚线 leak-line 须保留').toContain('class="leak-line"')
    expect(battles, '胜圆点 trend-win 须保留').toContain('trend-win')
    expect(battles, '负圆点 trend-lose 须保留').toContain('trend-lose')
    expect(battles, '胜负判定 result===win 须保留').toContain("win: it.result === 'win'")
  })

  it('不变量：CSS 三编码颜色与模板 v-if/aria-label 保留', () => {
    expect(battles, '分数线 var(--ice) 须保留').toContain('stroke: var(--ice)')
    expect(battles, '漏怪率线 var(--bad) dasharray 须保留').toContain('stroke-dasharray: 4 3')
    expect(battles, '胜点 var(--ok) 须保留').toContain('.trend-win { fill: var(--ok); }')
    expect(battles, '负点 var(--bad) 须保留').toContain('.trend-lose { fill: var(--bad); }')
    expect(battles, 'count>=2 显示守卫须保留').toContain('v-if="trend.count >= 2"')
    expect(battles, 'aria-label 须保留').toContain('aria-label="战绩分数趋势"')
  })

  it('令牌三锚点保留、零 r88 令牌、写域边界', () => {
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(battles + tokens, '不得新增 r88 视觉令牌').not.toContain('--r88-')
    const allowed = new Set(['pages/battles/battles.vue', 'styles/tokens.css'])
    const self = readFileSync(__filename, 'utf-8')
    const calls = self.match(/readSrc\([^)]+\)/g) || []
    for (const call of calls) {
      const m = call.match(/readSrc\(['"]([^'"]+)['"]\)/)
      if (m) expect(allowed.has(m[1]), '写域越界').toBe(true)
    }
  })
})