// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { formatFileSize } from './formatSize'

describe('formatFileSize：文件大小统一格式（R163）', () => {
  it('按 1024 分档输出 B/KB/MB/GB，B 档整数、其余一位小数', () => {
    expect(formatFileSize(0)).toBe('0 B')
    expect(formatFileSize(512)).toBe('512 B')
    expect(formatFileSize(1024)).toBe('1.0 KB')
    expect(formatFileSize(1536)).toBe('1.5 KB')
    expect(formatFileSize(1048576)).toBe('1.0 MB')
    expect(formatFileSize(1073741824)).toBe('1.0 GB')
    // GB 以上封顶，不再溢出到 TB：多页旧实现会把 1 TB 显示成 1024.0 GB
    expect(formatFileSize(1099511627776)).toBe('1024.0 GB')
  })

  it('接受数值字符串，保持与后端数字字段一致的结果', () => {
    expect(formatFileSize('2048')).toBe('2.0 KB')
    expect(formatFileSize('0')).toBe('0 B')
  })

  it('无效值（缺失/空串/非数字/NaN/负数）统一占位符', () => {
    for (const bad of [null, undefined, '', 'abc', Number.NaN, Number.POSITIVE_INFINITY, -1]) {
      expect(formatFileSize(bad)).toBe('—')
    }
  })
})
