// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R232 图表文字替代契约（WCAG 1.1.1 Non-text Content）。
 *
 * EChartsView 用 CanvasRenderer 把统计图绘制到 <canvas>，生成的像素对读屏器
 * 完全不可见，且容器没有任何 role / 可访问名称 / 等价文本。查询分析页
 * QueryManagerDeep 的「执行分析」非表格结果只有这一条渲染路径，视障用户
 * 无法得到任何图表信息。本契约要求：图表容器暴露 role=img + 可访问名称，
 * 并附带一张视觉隐藏但语义完整的等价数据表；描述由纯函数生成，可单测验证。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { describeChart } from './chartA11y'

const source = readFileSync(resolve(import.meta.dirname, '../components/EChartsView.vue'), 'utf8')

describe('describeChart 纯函数（图表可访问名称）', () => {
  const rows = [
    { 月份: '1月', 销售额: 120 },
    { 月份: '2月', 销售额: 260 },
    { 月份: '3月', 销售额: 180 },
  ]

  it('声明图表类型、维度与指标', () => {
    const text = describeChart(rows, ['月份', '销售额'], 'bar', '月份', '销售额')
    expect(text).toContain('柱状图')
    expect(text).toContain('月份')
    expect(text).toContain('销售额')
  })

  it('包含实际数据点，而非仅有标题', () => {
    const text = describeChart(rows, ['月份', '销售额'], 'bar', '月份', '销售额')
    expect(text).toContain('1月')
    expect(text).toContain('120')
    expect(text).toContain('2月')
    expect(text).toContain('260')
  })

  it('数据较多时给出总数并限制样例长度（避免冗长播报）', () => {
    const many = Array.from({ length: 40 }, (_, i) => ({ 类别: `C${i}`, 数量: i }))
    const text = describeChart(many, ['类别', '数量'], 'line', '类别', '数量')
    expect(text).toContain('40 项')
    expect(text).not.toContain('C39')
  })

  it('无数据时明确说明为空', () => {
    expect(describeChart([], ['月份', '销售额'], 'pie', '月份', '销售额')).toContain('暂无数据')
  })

  it('缺省维度/指标时从 columns 推导，不抛异常', () => {
    const text = describeChart(rows, ['月份', '销售额'], 'line')
    expect(text).toContain('折线图')
    expect(text).toContain('1月')
    expect(text).toContain('120')
  })
})

describe('EChartsView 图表文字替代接线（WCAG 1.1.1）', () => {
  it('图表容器声明 role="img" 并绑定可访问名称', () => {
    const container = source.match(/<div[^>]*class="chart-container"[^>]*>/)?.[0] ?? ''
    expect(container).toMatch(/role="img"/)
    expect(container).toMatch(/:aria-label="chartDescription"/)
  })

  it('通过 computed 调用 describeChart 生成描述', () => {
    expect(source).toMatch(/chartDescription\s*=\s*computed\(/)
    expect(source).toContain('describeChart(')
    expect(source).toContain("from '../utils/chartA11y'")
  })

  it('提供视觉隐藏的等价数据表，含 caption 与列/行表头', () => {
    const table = source.match(/<table[^>]*data-oa-chart-data[^>]*>[\s\S]*?<\/table>/)?.[0] ?? ''
    expect(table).toContain('<caption>')
    expect(table).toMatch(/scope="col"/)
    expect(table).toMatch(/scope="row"/)
    expect(table).toMatch(/class="chart-data-sr"/)
  })

  it('等价数据表逐行渲染所有列，不丢失数据点', () => {
    expect(source).toMatch(/v-for="\(row, ri\) in data"/)
    expect(source).toMatch(/v-for="col in columns"/)
  })

  it('sr-only 样式采用 clip 视觉隐藏而非 display:none', () => {
    const css = source.match(/\.chart-data-sr\s*\{[^}]*\}/)?.[0] ?? ''
    expect(css).toMatch(/clip:\s*rect\(0, 0, 0, 0\)/)
    expect(css).toMatch(/position:\s*absolute/)
    expect(css).not.toMatch(/display:\s*none/)
  })
})
