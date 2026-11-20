// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { buildCsv, parseCsv, rowsToObjects } from './csv'

describe('buildCsv 导出往返（RFC 4180）', () => {
  it('表头与字段一律引号包裹，内嵌引号以 doubled 转义', () => {
    expect(buildCsv(['h1', 'h2'], [['a"b', 'c,d']])).toBe('"h1","h2"\n"a""b","c,d"')
  })

  it('含逗号/引号/换行的列名与值可无损往返，不再错列（真实 bug 复现）', () => {
    const headers = ['姓名, 昵称', '部门']
    const rows = [['张三, 阿三', '研发']]
    const csv = buildCsv(headers, rows)

    const parsed = parseCsv(csv)
    expect(parsed.headers).toEqual(headers)
    expect(parsed.rows).toEqual(rows)
    expect(rowsToObjects(parsed.headers, parsed.rows)).toEqual([{ '姓名, 昵称': '张三, 阿三', 部门: '研发' }])
  })

  it('字段名带换行与双引号时也保持列对齐', () => {
    const headers = ['列\nA', '列"B']
    const rows = [['值\n1', '值"2']]
    const parsed = parseCsv(buildCsv(headers, rows))
    expect(parsed.headers).toEqual(headers)
    expect(parsed.rows).toEqual(rows)
  })

  it('空字段保留为空字符串而不是被丢弃', () => {
    const parsed = parseCsv(
      buildCsv(
        ['a', 'b'],
        [
          ['', 'v'],
          ['x', ''],
        ],
      ),
    )
    expect(parsed.headers).toEqual(['a', 'b'])
    expect(parsed.rows).toEqual([
      ['', 'v'],
      ['x', ''],
    ])
  })
})
