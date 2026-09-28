// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'QueryManagerDeep.vue'), 'utf8')

describe('QueryManagerDeep 表格设计（tableConfig）真应用', () => {
  it('主题落到表格 CSS 类（default/striped/bordered）', () => {
    expect(source).toContain(":class=\"['tc-' + tableConfig.theme")
    expect(source).toContain('.res-table.tc-striped')
    expect(source).toContain('.res-table.tc-bordered')
  })

  it('可排序：点表头切列排序，带方向指示', () => {
    expect(source).toContain('@click="toggleSort(h)"')
    expect(source).toContain('function toggleSort(')
    expect(source).toContain("sortState.value.dir === 'asc' ? { key: h, dir: 'desc' } : null")
    expect(source).toContain('.res-table.sortable th{cursor:pointer')
  })

  it('可筛选：快速全列过滤输入驱动 displayRows', () => {
    expect(source).toContain('v-model="quickFilter"')
    expect(source).toContain('tableConfig.value.filterable && quickFilter.value.trim()')
    expect(source).toContain('const q = quickFilter.value.trim().toLowerCase()')
    expect(source).toContain('viewHeaders.value.some((h) =>')
    expect(source).toContain('.includes(q)')
  })

  it('行选择：按行内容签名稳定选中并计数（排序后不错位）', () => {
    expect(source).toContain('function rowKey(')
    expect(source).toContain('return JSON.stringify(row)')
    expect(source).toContain('selectedKeys.has(rowKey(row))')
    expect(source).toContain('已选 {{ selectedKeys.size }} 行')
  })

  it('交互层叠加在 viewRows 之上，表格渲染 displayRows', () => {
    expect(source).toContain('const displayRows = computed')
    expect(source).toContain('let rows = viewRows.value')
    expect(source).toContain('v-for="(row,i) in displayRows"')
  })

  it('导出仍走视图投影（viewRows/viewHeaders），不被交互层污染', () => {
    expect(source).toContain('const rows = viewRows.value.map')
    expect(source).toContain('const header = viewHeaders.value.join')
  })
})
