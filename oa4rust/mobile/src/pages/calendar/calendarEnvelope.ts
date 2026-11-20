// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * 日历 myCalendars 响应 data 信封的运行时归一化。
 * 真接口 /calendar/list/my 返回 { myCalendars: [], unitCalendars: [] }（对象），
 * 而历史代码用 (cals.data ?? []) as CalendarRow[] 只做了类型断言，运行期对象没有
 * .map，导致日历整页抛错。这里在运行时把两种形状都归一为扁平数组。
 * 后端契约与其他页面不变；数组形状仍原样透传以兼容旧桩。
 */
import type { CalendarRow } from '@/services'

function toRows(value: unknown): CalendarRow[] {
  return Array.isArray(value) ? (value as CalendarRow[]) : []
}

export function normalizeCalendarRows(data: unknown): CalendarRow[] {
  if (Array.isArray(data)) return data as CalendarRow[]
  if (!data || typeof data !== 'object') return []
  const envelope = data as { myCalendars?: unknown; unitCalendars?: unknown }
  return [...toRows(envelope.myCalendars), ...toRows(envelope.unitCalendars)]
}
