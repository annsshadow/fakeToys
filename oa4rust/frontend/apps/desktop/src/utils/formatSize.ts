// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 人类可读的文件大小（R163）。桌面端 FileInfoApp / FileManager / ProcessWork
 * 三页原先各写一份：空值占位符 '-' / '—' / '0 B' 不一致，前两页止于 MB，
 * 1 GB 以上显示成 "1024.0 MB"；本工具统一为下列契约：
 * - 无效值（缺失、空串、非数字、NaN/Infinity、负数）返回 '—'；
 * - 0 显示 '0 B'；按 1024 分档 B/KB/MB/GB，B 档整数、其余保留 1 位小数，
 *   GB 以上封顶为 "N GB"，不再溢出到 TB。
 */
export function formatFileSize(bytes: unknown): string {
  const n = typeof bytes === 'string' && bytes.trim() ? Number(bytes) : bytes
  if (typeof n !== 'number' || !Number.isFinite(n) || n < 0) return '—'
  if (n < 1024) return `${n} B`
  if (n < 1048576) return `${(n / 1024).toFixed(1)} KB`
  if (n < 1073741824) return `${(n / 1048576).toFixed(1)} MB`
  return `${(n / 1073741824).toFixed(1)} GB`
}
