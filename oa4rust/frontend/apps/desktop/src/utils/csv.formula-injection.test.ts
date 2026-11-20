// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { buildCsv, parseCsv } from './csv'

const TAB = String.fromCharCode(9)
const CR = String.fromCharCode(13)

describe('buildCsv CSV 公式注入防护（CWE-1236）', () => {
  it('危险前缀单元格被单引号中和，Excel/WPS 不再当公式执行', () => {
    const csv = buildCsv(['内容'], [['=1+1'], ['+SUM(A1)'], ['-2+3'], ['@cmd'], [`${TAB}=cmd`], [`${CR}=cmd`]])
    expect(csv).toContain(`"'=1+1"`)
    expect(csv).toContain(`"'+SUM(A1)"`)
    expect(csv).toContain(`"'-2+3"`)
    expect(csv).toContain(`"'@cmd"`)
    expect(csv).toContain(`"'${TAB}=cmd"`)
    expect(csv).toContain(`"'${CR}=cmd"`)
  })

  it('普通单元格保持 RFC4180 引号包裹原样，不被误加前缀', () => {
    expect(buildCsv(['姓名', '部门'], [['张三', '研发']])).toBe('"姓名","部门"\n"张三","研发"')
  })

  it('中和后的危险值经 parseCsv 读回为带单引号的纯文本', () => {
    const parsed = parseCsv(buildCsv(['v'], [['=SUM(1,2)']]))
    expect(parsed.rows).toEqual([["'=SUM(1,2)"]])
  })
})
