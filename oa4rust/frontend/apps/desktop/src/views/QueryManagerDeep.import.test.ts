// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string) => readFileSync(resolve(import.meta.dirname, name), 'utf8')

describe('QueryManagerDeep CSV 导入', () => {
  const source = read('QueryManagerDeep.vue')

  it('占位 toast 已被真导入流程替换（解析走 utils/csv）', () => {
    expect(source).not.toContain('导入功能开发中')
    expect(source).toContain("import { parseCsv } from '../utils/csv'")
    expect(source).toContain('parseCsv(text, importConfig.value.delimiter)')
  })

  it('每行真实 POST 到表行插入端点（handler 整包存 body 为行 data）', () => {
    expect(source).toContain('/api/query/assemble/designer/table/${encodeURIComponent(flag)}/row')
  })

  it('四道闸齐备：文件在位/Excel 拒收/空解析拦截/行数上限', () => {
    expect(source).toContain('IMPORT_ROW_LIMIT = 500')
    expect(source).toContain('importFileEl.value?.files?.[0]')
    expect(source).toContain('(xlsx|xls)')
    expect(source).toContain('未解析到有效数据行')
    expect(source).toContain('超出上限截断')
  })

  it('文件选择器 accept 收窄到实际支持的文本格式', () => {
    expect(source).toContain('accept=".csv,.tsv,.txt"')
    expect(source).not.toContain('accept=".csv,.xlsx,.xls"')
  })
})
