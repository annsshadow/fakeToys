// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/** 图表类型对应的中文名称，用于构造可访问名称。 */
const TYPE_LABEL: Record<string, string> = {
  bar: '柱状图',
  pie: '饼图',
  line: '折线图',
  table: '数据表',
}

/** 最多在可访问名称中列出的样例数据点数，避免读屏器长篇播报。 */
const SAMPLE_LIMIT = 5

/**
 * 为 Canvas 图表生成文字替代描述（WCAG 1.1.1）。
 * 描述包含图表类型、维度/指标以及前若干个真实数据点；数据较多时给出总数。
 */
export function describeChart(
  rows: Record<string, unknown>[],
  columns: string[],
  chartType: string,
  dimension?: string,
  metric?: string,
): string {
  const label = TYPE_LABEL[chartType] ?? '图表'
  const dim = dimension || columns[0] || ''
  const met = metric || columns.find((c) => c !== dim) || columns[0] || ''
  if (rows.length === 0) return `${label}：暂无数据`
  const sample = rows
    .slice(0, SAMPLE_LIMIT)
    .map((row) => `${String(row[dim] ?? '')}: ${String(row[met] ?? '')}`)
    .join('，')
  const more = rows.length > SAMPLE_LIMIT ? `（共 ${rows.length} 项，仅列出前 ${SAMPLE_LIMIT} 项）` : ''
  return `${label}：按${dim}展示${met}，${sample}${more}`
}
