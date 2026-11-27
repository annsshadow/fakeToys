// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R618b RED→GREEN: statResult 是后端嵌套响应，statChartData 必须映射 data[] 统计行而非顶层元数据。
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import ts from 'typescript'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'QueryManagerDeep.vue'), 'utf8')
const script = source.match(/<script setup[^>]*>([\s\S]*?)<\/script>/)?.[1] ?? ''
const js = ts.transpileModule(script, {
  compilerOptions: { target: ts.ScriptTarget.ES2020, module: ts.ModuleKind.ESNext },
}).outputText

/** 按大括号配对提取纯函数源码，使其在 Node 中真实执行。 */
function extractFn(name: string): string {
  const i = js.indexOf('function ' + name + '(')
  if (i < 0) return ''
  const open = js.indexOf('{', i)
  let depth = 0
  for (let p = open; p < js.length; p++) {
    if (js[p] === '{') depth++
    else if (js[p] === '}') {
      depth--
      if (depth === 0) return js.slice(i, p + 1)
    }
  }
  return ''
}

const fnSrc = extractFn('mapStatRows')
const mapStatRows = fnSrc
  ? (new Function(fnSrc + '; return mapStatRows;')() as (resp: any) => Array<{ name: string; value: number }>)
  : null

/** 后端 u2_closures.rs stat/do 真实响应包络 data 字段形状。 */
const backendShape = {
  id: 'q-1',
  dimension: 'region',
  metric: 'rowCount',
  data: [
    { dimension: '华北', rowCount: 120, metricTotal: '120' },
    { dimension: '华南', rowCount: 80, metricTotal: '80' },
  ],
}

describe('QueryManagerDeep 统计映射 data[] 真修复契约', () => {
  it('存在可执行的 mapStatRows, statChartData 由它派生', () => {
    expect(mapStatRows).toBeTypeOf('function')
    expect(script).toMatch(/const statChartData = computed\(\(\) => mapStatRows\(statResult\.value\)\)/)
  })

  it('真实后端形状产出 2 个统计行 name/value 正确', () => {
    expect(mapStatRows?.(backendShape)).toEqual([
      { name: '华北', value: 120 },
      { name: '华南', value: 80 },
    ])
  })

  it('不含 id/dimension/metric 顶层元数据行', () => {
    const rows = mapStatRows?.(backendShape) ?? []
    expect(rows).toHaveLength(2)
    for (const r of rows) {
      expect(['id', 'dimension', 'metric']).not.toContain(r.name)
    }
  })

  it('null/空 data 安全返回空数组', () => {
    expect(mapStatRows?.(null)).toEqual([])
    expect(mapStatRows?.({ ...backendShape, data: [] })).toEqual([])
  })

  it('源码不再用 Object.entries(statResult.value) 错映射', () => {
    expect(script).not.toContain('Object.entries(statResult.value)')
  })
})
