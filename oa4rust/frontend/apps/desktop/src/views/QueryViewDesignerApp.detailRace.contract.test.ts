// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'QueryViewDesignerApp.vue'), 'utf8')
const openViewSrc = source.slice(
  source.indexOf('async function openView(id: string) {'),
  source.indexOf('async function saveView()'),
)
const newViewSrc = source.slice(source.indexOf('function newView() {'), source.indexOf('function addFilter()'))

describe('查询视图设计器详情回填竞态守卫', () => {
  it('openView 用递增序号只认最新详情，过期响应不得覆盖当前视图表单', () => {
    expect(source).toContain('let detailSeq = 0')
    expect(openViewSrc).toContain('const seq = ++detailSeq')
    expect(openViewSrc).toContain('if (seq !== detailSeq) return')
    expect(openViewSrc.indexOf('const seq = ++detailSeq')).toBeLessThan(
      openViewSrc.indexOf('if (seq !== detailSeq) return'),
    )
    expect(openViewSrc.indexOf('if (seq !== detailSeq) return')).toBeLessThan(
      openViewSrc.indexOf('activeId.value = id'),
    )
  })

  it('新建视图使在途详情失效，旧视图不得回填到空白表单', () => {
    expect(newViewSrc).toContain('detailSeq++')
  })
})
