// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 解析服务端返回的时间（R152）。
 *
 * iOS WebView（Safari）里 `new Date('2026-11-07 10:30:00')` 是 Invalid Date，
 * Android/桌面 Chromium 却能手解析，于是同一份数据在 iPhone 上会退化成原始字符串。
 * 这里对「无时区」的 `YYYY-MM-DD[ T]HH:mm[:ss]` 显式按本地时间构造 Date，
 * 带 `Z`/`±hh:mm` 时区标记或其它可被标准解析的写法仍交给 `new Date`。
 */
const LOCAL_PATTERN = /^(\d{4})[-/](\d{1,2})[-/](\d{1,2})(?:[ T](\d{1,2}):(\d{2})(?::(\d{2}))?)?$/

export function parseServerDate(value: unknown): Date | null {
  if (value instanceof Date) return Number.isNaN(value.valueOf()) ? null : value
  if (typeof value === 'number') {
    // 有限数的毫秒时间戳总能构成合法 Date；NaN/Infinity 已在上面挡掉。
    return Number.isFinite(value) ? new Date(value) : null
  }
  if (typeof value !== 'string') return null
  const text = value.trim()
  if (!text) return null
  const local = LOCAL_PATTERN.exec(text)
  if (local) {
    const [, y, mo, d, h = '0', mi = '0', s = '0'] = local
    const date = new Date(Number(y), Number(mo) - 1, Number(d), Number(h), Number(mi), Number(s))
    // 2 月 30 日这类「格式对但日子不存在」的输入会被 rollover，回读校验后拒绝。
    if (date.getFullYear() !== Number(y) || date.getMonth() !== Number(mo) - 1 || date.getDate() !== Number(d)) {
      return null
    }
    return date
  }
  const parsed = new Date(text)
  return Number.isNaN(parsed.valueOf()) ? null : parsed
}
