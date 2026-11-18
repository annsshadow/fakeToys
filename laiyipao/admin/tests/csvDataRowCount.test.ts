// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 导出提示条数必须来自导出的 CSV 本身，而非当前页列表长度。
 * 服务端导出是全量筛选（无分页），当前页长度会把"已导出 N 条"少报成一页。
 * csvDataRowCount 稳健统计 CSV 数据行数（扣表头、处理引号内换行与 BOM/CRLF）。
 */
import { describe, expect, it } from 'vitest'
import { csvDataRowCount } from '@/utils/format'

describe('csvDataRowCount：CSV 数据行数（不含表头）', () => {
  it('BOM + 表头 + 两条数据', () => {
    expect(csvDataRowCount('\uFEFFid,name\n1,张三\n2,李四\n')).toBe(2)
  })

  it('引号内换行不拆成两行', () => {
    expect(csvDataRowCount('\uFEFFid,name\n1,"张\n三"\n')).toBe(1)
  })

  it('只有表头、无数据', () => {
    expect(csvDataRowCount('\uFEFFid,name')).toBe(0)
  })

  it('空文本', () => {
    expect(csvDataRowCount('')).toBe(0)
  })

  it('CRLF 行尾 + 引号内转义双引号', () => {
    expect(csvDataRowCount('\uFEFFid,note\r\n1,"a""b"\r\n2,c\r\n')).toBe(2)
  })
})
