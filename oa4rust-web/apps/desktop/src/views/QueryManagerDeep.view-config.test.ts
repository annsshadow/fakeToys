// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'QueryManagerDeep.vue'), 'utf8')

describe('QueryManagerDeep 视图配置真应用', () => {
  it('applyViewConfig 不再是空壳：校验列/排序列并给出应用摘要', () => {
    expect(source).not.toContain('/* apply config to current query */')
    expect(source).toContain('视图列不存在')
    expect(source).toContain('排序列不存在')
    expect(source).toContain('视图配置已应用')
  })

  it('结果网格与导出走视图投影（所见即所得）', () => {
    expect(source).toContain('v-for="h in viewHeaders"')
    expect(source).toContain('v-for="(row,i) in viewRows"')
    expect(source).toContain('const header = viewHeaders.value.join')
    expect(source).toContain('const rows = viewRows.value.map')
  })

  it('过滤表达式客户端求值：等值/不等/包含/裸子串，无注入面', () => {
    expect(source).toContain('expr.match(/^(\\w+)\\s*=\\s*(.*)$/)')
    expect(source).toContain('expr.match(/^(\\w+)\\s*!=\\s*(.*)$/)')
    expect(source).toContain('expr.match(/^(\\w+)\\s*~\\s*(.*)$/)')
  })
})
