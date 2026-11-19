// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'QueryTableDesignerApp.vue'), 'utf8')
const openTableSrc = source.slice(
  source.indexOf('function openTable(item: TableSummary) {'),
  source.indexOf('async function loadTables()'),
)
const newTableSrc = source.slice(source.indexOf('function newTable() {'), source.indexOf('function openTable('))

describe('数据表设计器详情回填竞态守卫', () => {
  it('openTable 用递增序号只认最新详情，过期响应不得覆盖当前表列定义', () => {
    expect(source).toContain('let detailSeq = 0')
    expect(openTableSrc).toContain('const seq = ++detailSeq')
    expect(openTableSrc).toContain('if (seq !== detailSeq) return')
    expect(openTableSrc).toContain('form.value.columns = d.columns.map')
    expect(openTableSrc.indexOf('if (seq !== detailSeq) return')).toBeLessThan(
      openTableSrc.indexOf('form.value.columns = d.columns.map'),
    )
  })

  it('新建表使在途详情失效，旧表列不得回填到空白表单', () => {
    expect(newTableSrc).toContain('detailSeq++')
  })
})
