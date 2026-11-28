import { readFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = import.meta.dirname ?? dirname(fileURLToPath(import.meta.url))
const src = readFileSync(resolve(here, 'QueryDesigner.vue'), 'utf8')

describe('R630a QueryDesigner chartType 四形态真接线契约', () => {
  it('接入 EChartsView 兄弟组件并绑定 chart-type prop', () => {
    expect(src).toContain("import EChartsView from '../components/EChartsView.vue'")
    expect(src).toContain(':chart-type=')
    expect(src).toContain('dimension="name"')
    expect(src).toContain('metric="value"')
  })

  it('chartType 被真正消费而非仅声明（chartType.value 读取）', () => {
    expect(src).toMatch(/chartType\.value/)
  })

  it('area 形态映射为 line（EChartsView line 分支已带 areaStyle）', () => {
    expect(src).toMatch(/chartType\.value\s*===\s*'area'\s*\?\s*'line'/)
  })

  it('四形态选项 bar/line/pie/area 全部保留', () => {
    for (const t of ['bar', 'line', 'pie', 'area']) {
      expect(src).toContain(`value="${t}"`)
    }
  })

  it('旧硬编码 chart-bars 独占渲染被替换为 EChartsView 数据绑定', () => {
    expect(src).not.toContain('class="chart-bars"')
    expect(src).toContain(':data="chartEchartsData"')
    expect(src).toContain('chartEchartsData')
  })
})
