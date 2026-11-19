// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * RFC 4180 子集 CSV 解析 —— 与 QueryManagerDeep.exportResults 的写出方言互逆：
 * 引号包裹字段、"" 转义、CRLF/CR/LF 行尾、可配置分隔符（`,` `;` 制表符）。
 * Excel 导出的 CSV 带 BOM，此处剥掉；选项值 `\t`（字面反斜杠 t）映射为真制表符。
 */
export function parseCsv(text: string, delimiter = ','): { headers: string[]; rows: string[][] } {
  const delim = delimiter === '\\t' ? '\t' : delimiter
  const src = text.charCodeAt(0) === 0xfeff ? text.slice(1) : text
  const rows: string[][] = []
  let row: string[] = []
  let field = ''
  let inQuotes = false
  let dirty = false
  for (let i = 0; i < src.length; i++) {
    const ch = src[i]
    if (inQuotes) {
      if (ch === '"') {
        if (src[i + 1] === '"') {
          field += '"'
          i++
        } else {
          inQuotes = false
        }
      } else {
        field += ch
      }
      dirty = true
    } else if (ch === '"') {
      inQuotes = true
      dirty = true
    } else if (ch === delim) {
      row.push(field)
      field = ''
      dirty = true
    } else if (ch === '\n') {
      row.push(field)
      field = ''
      rows.push(row)
      row = []
      dirty = false
    } else if (ch === '\r') {
      if (src[i + 1] !== '\n') {
        row.push(field)
        field = ''
        rows.push(row)
        row = []
        dirty = false
      }
    } else {
      field += ch
      dirty = true
    }
  }
  if (dirty || field !== '' || row.length > 0) {
    row.push(field)
    rows.push(row)
  }
  const nonEmpty = rows.filter((r) => r.length > 1 || (r[0] ?? '').trim() !== '')
  const [headers = [], ...body] = nonEmpty
  return { headers, rows: body }
}

/** 把表头+行数组折成行对象（缺列补空串），与后端 table/{flag}/row 的整包 body 存储契约对齐。 */
export function rowsToObjects(headers: string[], rows: string[][]): Record<string, string>[] {
  return rows.map((r) => Object.fromEntries(headers.map((h, i) => [h, r[i] ?? ''])))
}

/**
 * RFC 4180 子集 CSV 序列化 —— parseCsv 的逆操作。表头与字段一律双引号包裹，
 * 内嵌双引号以 doubled（""）转义，防止列名/值中的逗号、引号、换行破坏列对齐。
 * 行尾统一 LF，与 parseCsv 的方言互逆。
 */
export function buildCsv(headers: string[], rows: string[][]): string {
  const quote = (v: string) => {
    // CWE-1236：以 = + - @（含前导空白/制表符/回车）开头的单元格在 Excel/WPS 中会被当公式求值，
    // 前置单引号强制其为纯文本；普通值行为不变。
    const safe = /^\s*[=+\-@]/.test(v) ? `'${v}` : v
    return `"${safe.replace(/"/g, '""')}"`
  }
  const line = (cells: string[]) => cells.map((c) => quote(String(c ?? ''))).join(',')
  return [line(headers), ...rows.map(line)].join('\n')
}
