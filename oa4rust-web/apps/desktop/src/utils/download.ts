// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 触发浏览器下载 Blob 并释放 ObjectURL。
 * 此前各视图自行 createElement/createObjectURL，从不 revoke——长会话中每次导出
 * 都泄漏一块二进制内存。统一走本工具（revoke 延迟到下一宏任务，确保点击生效）。
 */
export function downloadBlob(blob: Blob, filename: string): void {
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = filename
  a.click()
  setTimeout(() => URL.revokeObjectURL(url), 0)
}
