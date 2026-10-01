// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'QueryManagerDeep.vue'), 'utf8')
const chart = readFileSync(resolve(import.meta.dirname, '../components/EChartsView.vue'), 'utf8')

describe('QueryManagerDeep 统计分析图表落地', () => {
  it('图表类型选择不再是死配置：非表格时用 EChartsView 渲染', () => {
    // 此前用户选柱/饼/折线只落到键值列表，chartType 形同虚设
    expect(source).toContain("import EChartsView from '../components/EChartsView.vue'")
    expect(source).toContain('<EChartsView')
    expect(source).toContain("statConfig.chartType !== 'table'")
    expect(source).toContain(':chart-type="statConfig.chartType"')
  })

  it('表格模式仍回退键值列表，无数据不渲染图表', () => {
    expect(source).toContain('statChartData.length')
    expect(source).toContain('<template v-else>')
    expect(source).toContain('v-for="(v,k) in statResult"')
  })

  it('statResult 映射转 ECharts 行集（name/value）', () => {
    expect(source).toContain('const statChartData = computed(')
    expect(source).toContain('Object.entries(statResult.value).map(([name, value]) => ({ name, value }))')
  })

  it('EChartsView 用按需模块化 echarts（tree-shakable，避免全量打包）', () => {
    expect(chart).toContain("import * as echarts from 'echarts/core'")
    expect(chart).toContain("from 'echarts/charts'")
    expect(chart).toContain('echarts.use([')
    expect(chart).not.toContain("import * as echarts from 'echarts'\n")
  })
})
