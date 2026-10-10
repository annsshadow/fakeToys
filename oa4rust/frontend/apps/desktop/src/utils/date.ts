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
