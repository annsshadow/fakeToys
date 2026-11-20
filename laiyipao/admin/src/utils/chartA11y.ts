// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 看板 ECharts Canvas 图的等价文本（WCAG 1.1.1）。
 * 把渲染进 canvas 的数据抽成读屏可读的一句话摘要 + 结构化表格。
 */
export interface ChartRow {
  label: string
  value: number
}

export interface ChartDescInput {
  title: string
  chartType: keyof typeof CHART_TYPE_LABEL
  unit: string
  metric: string
  rows: ChartRow[]
}

const CHART_TYPE_LABEL = {
  line: '折线图',
  bar: '柱状图',
  radar: '雷达图',
} as const

export function describeDashboardChart(input: ChartDescInput): { label: string; rows: ChartRow[] } {
  const type = CHART_TYPE_LABEL[input.chartType] ?? '统计图'
  const head = `${input.title}${type}`
  if (input.rows.length === 0) return { label: `${head}，暂无数据`, rows: [] }
  const sample = input.rows
    .slice(0, 5)
    .map((r) => `${r.label}${input.metric} ${r.value}`)
    .join('；')
  const more = input.rows.length > 5 ? ' 等' : ''
  return {
    label: `${head}，共 ${input.rows.length} ${input.unit}：${sample}${more}`,
    rows: input.rows,
  }
}
