// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { parseCsv, rowsToObjects } from './csv'

describe('parseCsv', () => {
  it('以首个非空行为表头，解析普通行列', () => {
    const { headers, rows } = parseCsv('name,age\n张三,31\n李四,28')
    expect(headers).toEqual(['name', 'age'])
    expect(rows).toEqual([
      ['张三', '31'],
      ['李四', '28'],
    ])
  })

  it('引号字段内嵌分隔符、换行与 "" 转义引号', () => {
    const { rows } = parseCsv('a,b\n"x,1","line\n2"\n"c""q",3')
    expect(rows[0]).toEqual(['x,1', 'line\n2'])
    expect(rows[1]).toEqual(['c"q', '3'])
  })

  it('CRLF 与孤立 CR 行尾都收行，并剥掉 Excel BOM', () => {
    expect(parseCsv('﻿a,b\r\n1,2\r3,4').rows).toEqual([
      ['1', '2'],
      ['3', '4'],
    ])
  })

  it('字面 \\t 选项值映射为真制表符分隔符', () => {
    const { headers, rows } = parseCsv('a\tb\n1\t2', '\\t')
    expect(headers).toEqual(['a', 'b'])
    expect(rows).toEqual([['1', '2']])
  })

  it('分号分隔符可用，空行被丢弃', () => {
    const { headers, rows } = parseCsv('a;b\n1;2\n\n3;4', ';')
    expect(headers).toEqual(['a', 'b'])
    expect(rows).toEqual([
      ['1', '2'],
      ['3', '4'],
    ])
  })

  it('行尾空字段保留；尾随换行不产生幻影行；空输入返回空表头', () => {
    expect(parseCsv('a,b\n1,').rows).toEqual([['1', '']])
    expect(parseCsv('a,b\n1,2\n').rows).toEqual([['1', '2']])
    expect(parseCsv('').headers).toEqual([])
    expect(parseCsv('').rows).toEqual([])
  })
})

describe('rowsToObjects', () => {
  it('表头+行数组折成行对象，缺列补空串', () => {
    expect(rowsToObjects(['h1', 'h2'], [['v1'], ['v2', 'v3']])).toEqual([
      { h1: 'v1', h2: '' },
      { h1: 'v2', h2: 'v3' },
    ])
  })
})
