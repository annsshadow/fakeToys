// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string) => readFileSync(resolve(import.meta.dirname, name), 'utf8')

describe('R620b QueryStatementDesigner vizType three shapes', () => {
  it('vizType selector drives a real EChartsView for line/pie and native bars for bar', () => {
    const s = read('QueryStatementDesigner.vue')
    const q = String.fromCharCode(39)
    expect(s).toContain('v-model="vizType"')
    expect(s).toContain('<EChartsView')
    expect(s).toContain(':chart-type="vizType"')
    expect(s).toContain(':columns="vizChartColumns"')
    expect(s).toContain(':data="vizChartData"')
    expect(s).toContain(`import EChartsView from ${q}../components/EChartsView.vue${q}`)
    expect(s).toContain(`const vizType = ref(${q}bar${q})`)
    expect(s).toContain('const vizChartData = computed(')
    expect(s).toContain('name: d.label')
    expect(s).toContain('value: d.value')
    expect(s).toMatch(/vizType\s*===\s*'bar'/)
  })
})
