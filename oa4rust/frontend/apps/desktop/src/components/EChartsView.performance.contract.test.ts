// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'EChartsView.vue'), 'utf8')

describe('EChartsView 图表性能与主题适配契约', () => {
  it('容器尺寸变化时通过 ResizeObserver 调用 chart.resize（布局切换不拉伸失真）', () => {
    expect(source).toContain('ResizeObserver')
    expect(source.match(/\.resize\(\)/g)?.length ?? 0).toBeGreaterThanOrEqual(1)
    expect(source).toContain('disconnect()')
  })

  it('图表实例复用且深度 watch 仅在数据/配置变化时 setOption', () => {
    expect(source).toContain('if (!chart) chart = echarts.init(chartRef.value)')
    expect(source).toContain('chart.setOption(option, true)')
    expect(source).toContain('{ immediate: true, deep: true }')
  })

  it('按需 echarts 模块 + 卸载时 dispose 仍在（不回退既有性能契约）', () => {
    expect(source).toContain("import * as echarts from 'echarts/core'")
    expect(source).toContain('echarts.use([')
    expect(source).toContain('chart?.dispose()')
  })

  it('图表调色板使用主题令牌读取，不再散落硬编码色值', () => {
    expect(source).toContain('CHART_PALETTE')
    expect(source).toContain('getComputedStyle')
    expect(source).not.toContain("'#00d4ff'")
    expect(source).not.toContain("'#f59e0b'")
    expect(source).not.toContain("'#ef4444'")
  })
})
