// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// O27：流程引擎「定时器」按钮的请求构造。
//
// 后端契约（`processplatform_service_processing::start_timer`）：
//   POST /api/processplatform/service/processing/timer/start
//   body = { workId: string, fireAt: "YYYY-MM-DD HH:MM:SS" }
// `fire_at` 落 `TIMESTAMP`，ticker 用 `Utc::now().naive_utc()` 比较（见 lib.rs:4105），
// 因此 **fireAt 必须是 UTC naive 时间**；而用户在界面里填写的是北京时间（UTC+8）。
// 两者相差 8 小时：直接提交本地时间会让任务晚 8 小时触发，故在此统一换算。
// 业务时区固定 UTC+8，与 O26-1 的业务日界口径一致。

export const BUSINESS_UTC_OFFSET_HOURS = 8
const BUSINESS_OFFSET_MS = BUSINESS_UTC_OFFSET_HOURS * 60 * 60 * 1000
const FIRE_AT_RE = /^(\d{4})-(\d{2})-(\d{2}) (\d{2}):(\d{2}):(\d{2})$/

export interface TimerStartPayload {
  workId: string
  fireAt: string
}

export type TimerStartBuildResult = { ok: true; payload: TimerStartPayload } | { ok: false; error: string }

function formatUtcTimestamp(ms: number): string {
  const d = new Date(ms)
  const pad = (n: number) => String(n).padStart(2, '0')
  return [
    `${d.getUTCFullYear()}-${pad(d.getUTCMonth() + 1)}-${pad(d.getUTCDate())}`,
    `${pad(d.getUTCHours())}:${pad(d.getUTCMinutes())}:${pad(d.getUTCSeconds())}`,
  ].join(' ')
}

/// 当前北京时间（UTC+8），用于 prompt 默认值；与时区无关、不依赖浏览器本地时区。
export function beijingNow(now: Date = new Date()): string {
  return formatUtcTimestamp(now.getTime() + BUSINESS_OFFSET_MS)
}

/// 把用户输入的北京时间换算为后端要求的 UTC naive 时间串。
/// 非法输入（格式不符、月日越界如 2026-02-30）返回 null。
export function beijingWallClockToUtcNaive(input: string): string | null {
  const matched = FIRE_AT_RE.exec(input.trim())
  if (!matched) return null
  const [year, month, day, hour, minute, second] = matched.slice(1).map((part) => Number(part))
  if (hour > 23 || minute > 59 || second > 59) return null

  const wallClockMs = Date.UTC(year, month - 1, day, hour, minute, second)
  // Date.UTC 会把越界日期顺延（2026-02-30 → 2026-03-02）；逐字段回读确认输入真实存在。
  const check = new Date(wallClockMs)
  if (check.getUTCFullYear() !== year || check.getUTCMonth() !== month - 1 || check.getUTCDate() !== day) {
    return null
  }
  return formatUtcTimestamp(wallClockMs - BUSINESS_OFFSET_MS)
}

/// 构造 `timer/start` 请求体；任何字段不合法都返回可展示的错误，不发出半成品请求。
export function buildTimerStartPayload(workId: string, localFireAt: string): TimerStartBuildResult {
  const trimmedWorkId = workId.trim()
  if (trimmedWorkId === '') {
    return { ok: false, error: '流程实例 ID 必填' }
  }
  const utcFireAt = beijingWallClockToUtcNaive(localFireAt)
  if (utcFireAt === null) {
    return {
      ok: false,
      error: '触发时间格式应为 YYYY-MM-DD HH:MM:SS（北京时间），且必须是真实存在的时刻',
    }
  }
  return { ok: true, payload: { workId: trimmedWorkId, fireAt: utcFireAt } }
}
