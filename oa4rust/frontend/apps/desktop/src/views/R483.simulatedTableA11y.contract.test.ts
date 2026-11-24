// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R483 考勤模拟表格 ARIA 语义补齐契约：
 * div.pt/span.th + div.tr + span.td 模拟表格必须补齐 table/row/columnheader/cell
 * 语义角色，且业务模板/样式/data-testid 零改。真父版必 RED、修复版必 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AttendanceApp.vue'), 'utf8')

/** 取表头行片段（首个 class="pt" 的 div 到其 </div>） */
function headerLine(src: string): string {
  const at = src.indexOf('class="pt"')
  if (at < 0) return ''
  const start = src.lastIndexOf('<div', at)
  const end = src.indexOf('</div>', at)
  return src.slice(start, end + 6)
}

/** 取数据行模板片段（v-for records 的 div.tr 到其 </div>） */
function dataRow(src: string): string {
  const at = src.indexOf('v-for="r in records"')
  if (at < 0) return ''
  const start = src.lastIndexOf('<div', at)
  const cellAt = src.indexOf('class="td rec-act"', at)
  const end = src.indexOf('</div>', cellAt)
  return src.slice(start, end + 6)
}

describe('R483 考勤模拟表格 ARIA 语义补齐契约', () => {
  const header = headerLine(source)
  const row = dataRow(source)

  it('表头 div.pt 行容器带 role=row 且 6 个 span.th 带 role=columnheader', () => {
    expect(header, '必须找到表头行').not.toBe('')
    expect(header).toMatch(/<div[^>]*class="pt"[^>]*role="row"/)
    const ths = header.match(/<span[^>]*class="th"[^>]*>/g) || []
    expect(ths.length, '表头列数必须为 6').toBe(6)
    for (const th of ths) expect(th).toMatch(/role="columnheader"/)
  })

  it('表格 wrapper（考勤记录 section）带 role=table', () => {
    expect(source).toMatch(/<section[^>]*class="modern-section glass-card content-panel"[^>]*role="table"/)
  })

  it('数据行 div.tr（v-for records）带 role=row', () => {
    expect(row, '必须找到数据行').not.toBe('')
    expect(row).toMatch(/<div[^>]*class="tr"[^>]*role="row"/)
  })

  it('7 个 span.td 单元格均带 role=cell', () => {
    const tds = row.match(/<span[^>]*class="td[^"]*"[^>]*>/g) || []
    expect(tds.length, '单元格数必须为 7').toBe(7)
    for (const td of tds) expect(td).toMatch(/role="cell"/)
  })

  it('业务零改守卫：class/data-testid/v-for/@click/动态 class/display:grid 保留', () => {
    expect(source).toContain('data-testid="attendance-stats"')
    expect(source).toContain('data-testid="attendance-empty-cta"')
    expect(source).toContain('v-for="r in records"')
    expect(source).toContain(':class="{late:r.isLate}"')
    expect(source).toContain('@click="detailInfo(r)"')
    expect(source).toContain('@click="analyseOne(r)"')
    expect(source).toContain('@click="archiveOne(r)"')
    expect(source).toContain('@click="deleteDetail(r)"')
    expect(source).toContain('.tr{display:grid;grid-template-columns:80px 80px 90px 90px 60px 70px 1fr')
  })
})
