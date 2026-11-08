// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * 纯日期计算助手（无本地时区依赖，便于单测与跨端一致）。
 */

/**
 * 计算给定 "YYYY-MM" 月份的最后一天（返回 "YYYY-MM-DD"）。
 *
 * 全程用 Date.UTC 计算，避免 `new Date(y, m, 0).toISOString()` 这类「按本地时区
 * 构造、再按 UTC 序列化」在 UTC+ 时区下把月末当天 00:00 折成前一天 16:00 UTC、
 * 从而 `slice(0,10)` 少算一天（如 2026-10 的月末被算成 2026-10-30）的问题。
 *
 * @param ym 形如 "2026-10" 的年月（月为 1-12）
 * @returns 该月最后一天的 "YYYY-MM-DD"，如 "2026-10-31"
 */
export function monthEndYMD(ym: string): string {
  const [ys, ms] = ym.split('-')
  const y = Number(ys)
  const m = Number(ms)
  // 该月（1 基）最后一天 = 下一个月索引的 0 号（UTC），getUTCDate 取回天
  const lastDay = new Date(Date.UTC(y, m, 0)).getUTCDate()
  const yy = String(y).padStart(4, '0')
  const mm = String(m).padStart(2, '0')
  return `${yy}-${mm}-${String(lastDay).padStart(2, '0')}`
}

const LOCAL_DATETIME_PATTERN = /^(\d{4})[-/](\d{1,2})[-/](\d{1,2})(?:[ T](\d{1,2}):(\d{2})(?::(\d{2}))?)?$/

/**
 * 解析服务端时间（R154）。
 *
 * iOS WebView（Safari）里 `new Date('2026-11-07 10:30:00')` 是 Invalid Date，
 * Android/桌面 Chromium 却能隐式解析 —— 同一份数据在 iPhone/移动浏览器上会
 * 显示成 `Invalid Date` 或退化为原始字符串。这里对「无时区」的
 * `YYYY-MM-DD[ T]HH:mm[:ss]`（含 `/`）显式按本地时间构造 Date，并回读校验拒绝
 * `2026-02-30` 这类会被 Date 顺延的假日期；带 `Z`/`±hh:mm` 时区标记或其它标准
 * 写法仍交给 `new Date`。数字时间戳与合法 Date 实例直通，非法/空返回 null。
 */
export function parseServerDate(value: unknown): Date | null {
  if (value instanceof Date) return Number.isNaN(value.valueOf()) ? null : value
  if (typeof value === 'number') {
    // 有限数的毫秒时间戳总能构成合法 Date；NaN/Infinity 已在上面挡掉。
    return Number.isFinite(value) ? new Date(value) : null
  }
  if (typeof value !== 'string') return null
  const text = value.trim()
  if (!text) return null
  const local = LOCAL_DATETIME_PATTERN.exec(text)
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

/**
 * 格式化服务端时间（R154）：列表页 `fmtTime` 的统一实现。
 * 解析失败时回落原始值，避免 iOS 上把 `Invalid Date` 直接渲染到界面。
 */
export function formatServerDateTime(value: unknown, options?: Intl.DateTimeFormatOptions): string {
  const date = parseServerDate(value)
  if (!date) return String(value)
  return date.toLocaleString(
    'zh-CN',
    options ?? { month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit' },
  )
}
