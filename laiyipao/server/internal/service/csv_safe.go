// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
package service

import (
	"encoding/csv"
	"strconv"
)

// csvSafe 把可能被表格软件当作公式执行的单元格前缀成文本。
//
// 威胁模型：导出 CSV 里有运营/用户可控文本（昵称、公告标题、审计 target、
// 战报昵称等）。Excel / WPS / LibreOffice 打开时，以 `=` `+` `@` / TAB / CR
// 开头的单元格会被当公式求值，于是
// `=HYPERLINK("http://evil","点我")`、`@SUM(...)` 之类的内容在**运营的机器上**
// 执行，属于 CSV injection。
//
// 规则的关键是别破坏数字列：
//   - `=` / `+` / `@` / TAB / CR 开头 → 前缀 `'`
//   - `-` 开头但**不是合法数字** → 前缀 `'`（负数 `-5` 保持数值语义）
//   - 其余原样返回
//
// 前缀 `'` 是各家表格软件公认的「文本标记」写法；导入 CSV 时该字符
// 不会改变危险单元格的求值属性。
func csvSafe(s string) string {
	if s == "" {
		return s
	}
	switch s[0] {
	case '=', '+', '@', '\t', '\r':
		return "'" + s
	case '-':
		if _, err := strconv.ParseFloat(s, 64); err != nil {
			return "'" + s
		}
	}
	return s
}

// writeCSVRow 是导出统一出口：先中和公式注入再交给 csv.Writer。
//
// 所有 `w.Write(rec)` 都必须走这里；只改部分导出会让某张表成为
// 绕过点（漏一处等于没修）。
func writeCSVRow(w *csv.Writer, rec []string) error {
	for i, cell := range rec {
		rec[i] = csvSafe(cell)
	}
	return w.Write(rec)
}
