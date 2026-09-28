// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'QueryViewApp.vue'), 'utf8')

describe('QueryViewApp Excel 导出真契约', () => {
  it('调用真实双段路由（与 execute 同形状），不再打 404 单段地址', () => {
    expect(source).toContain("/api/queryview/excel/${encodeURIComponent(v.flag || 'view')}/${encodeURIComponent(v.id)}")
    expect(source).not.toContain('`/api/queryview/excel/${v.flag || v.id}`')
  })

  it('消费 excelData 真契约：base64 落 xlsx，文本落 csv，空值如实提示', () => {
    expect(source).toContain("r.data?.excelData ?? ''")
    expect(source).toContain('application/vnd.ms-excel')
    expect(source).toContain('该视图尚未生成 Excel 数据')
    expect(source).not.toContain('Excel导出暂未生成URL')
    expect(source).not.toContain('r.data?.url')
  })

  it('邻居执行失败文案无双冒号缺陷', () => {
    expect(source).toContain("`执行失败: ${e?.message ?? '未知错误'}`")
    expect(source).not.toContain('执行失败: :')
  })
})
