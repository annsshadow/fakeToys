// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 人类可读的文件大小。非有限数或负数返回空串（调用方据此省略该段）。
 * 原 doc.vue 只有 MB 与裸字节两档：1KB~1MB 的文件会显示成「500000 B」不可读，
 * 本函数补齐 KB/MB/GB 分档。
 */
export function formatFileSize(size: unknown): string {
  if (typeof size !== 'number' || !Number.isFinite(size) || size < 0) return ''
  if (size < 1024) return `${size} B`
  if (size < 1048576) return `${(size / 1024).toFixed(1)} KB`
  if (size < 1073741824) return `${(size / 1048576).toFixed(1)} MB`
  return `${(size / 1073741824).toFixed(1)} GB`
}
