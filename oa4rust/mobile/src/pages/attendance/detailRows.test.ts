// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { toMyDetailRows } from './detailRows'

describe('月度考勤明细值归一（R250）', () => {
  it('真数组原样保留（含 id/date/status 字段）', () => {
    const rows = [
      { id: 'a', date: '2026-11-19', status: '正常' },
      { id: 'b', date: '2026-11-18', status: '异常' },
    ]
    expect(toMyDetailRows(rows)).toEqual(rows)
  })

  it('对象信封（如 { list: [...] }）不得塞进 ref，否则模板 length 失效、v-for 渲染半截数据', () => {
    expect(toMyDetailRows({ list: [{ id: 'a' }], total: 1 })).toEqual([])
  })

  it('null / undefined / 标量都归一为空数组，保持「暂无考勤明细」可判定', () => {
    expect(toMyDetailRows(null)).toEqual([])
    expect(toMyDetailRows(undefined)).toEqual([])
    expect(toMyDetailRows('oops')).toEqual([])
    expect(toMyDetailRows(42)).toEqual([])
  })
})

describe('attendance.vue 接线（R250）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'attendance.vue'), 'utf8')

  it('myDetails 必须经 toMyDetailRows 归一，不再裸 as MyDetailRow[]', () => {
    expect(source).toContain("import { type MyDetailRow, toMyDetailRows } from './detailRows'")
    expect(source).toContain('myDetails.value = toMyDetailRows(detailResp.value.data)')
    expect(source).not.toContain('(detailResp.value.data ?? []) as MyDetailRow[]')
  })
})
