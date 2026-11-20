// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { parseServerDate } from '@/utils/datetime'

/** 事件起止时间校验失败原因：格式非法（含不存在的日期/越界时分秒）或结束早于开始。 */
export type EventTimeRangeError = 'invalid-format' | 'end-before-start'

/**
 * 新建事件前校验起止时间：两端都必须能被 parseServerDate 严格解析，
 * 且结束不得早于开始（相等 = 零时长日程，允许）。
 */
export function validateEventTimeRange(startTime: string, endTime: string): EventTimeRangeError | null {
  const start = parseServerDate(startTime)
  const end = parseServerDate(endTime)
  if (!start || !end) return 'invalid-format'
  if (end.valueOf() < start.valueOf()) return 'end-before-start'
  return null
}
