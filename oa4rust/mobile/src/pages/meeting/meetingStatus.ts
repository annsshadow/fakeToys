// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { parseServerDate } from '@/utils/datetime'

// x_meeting.status 的权威值域是英文串（wait/accepted/confirmed/processing/
// completed/denied/rejected）；历史列表曾以 '1'/'2'/'3' 表示进行中/已结束/已取消。
// 移动端列表端点（meeting/list/apply|invited/wait|wait/confirm）的 SELECT 不含
// status 列，卡片拿不到该字段，若只认数字值会永远落「未开始」。渲染侧兼容
// 多值域，并在 status 缺失时按真实返回的 startTime/endTime 推断展示，避免
// 进行中/已结束的会议恒显「未开始」。

type StatusTone = 'active' | 'ended' | 'pending' | 'cancelled'

function statusTone(status?: string): StatusTone {
  if (status === '1' || status === 'processing' || status === 'accepted' || status === 'confirmed') return 'active'
  if (status === '2' || status === 'completed' || status === 'denied' || status === 'rejected') return 'ended'
  if (status === '3') return 'cancelled'
  return 'pending'
}

/** 列表端点不返回 status 时，按起止时间推断当前是否正在进行或已结束。 */
function toneFromWindow(startTime?: string, endTime?: string, now: Date = new Date()): StatusTone {
  // R293：服务端时间串一律走 parseServerDate（对齐 R152/R154 契约）。
  // 内联 new Date(替换空格) 解析不了斜杠日期等后端历史格式，NaN 会让进行中/已结束的会议恒判「未开始」。
  const start = startTime ? (parseServerDate(startTime)?.getTime() ?? NaN) : NaN
  if (!Number.isFinite(start)) return 'pending'
  const end = endTime ? (parseServerDate(endTime)?.getTime() ?? NaN) : NaN
  const nowMs = now.getTime()
  if (Number.isFinite(end)) {
    if (nowMs > end) return 'ended'
    if (nowMs >= start) return 'active'
    return 'pending'
  }
  // 仅给了开始时间：开始后即视为进行中（无结束时间的会议暂无结束信号）。
  if (nowMs >= start) return 'active'
  return 'pending'
}

export interface MeetingStatusSource {
  status?: string
  startTime?: string
  endTime?: string
}

/** 供卡片展示：优先权威 status，缺失时用时间窗兜底。 */
export function meetingStatusText(source: MeetingStatusSource, now: Date = new Date()): string {
  const tone = statusTone(source.status)
  const resolved = tone === 'pending' ? toneFromWindow(source.startTime, source.endTime, now) : tone
  if (resolved === 'active') return '进行中'
  if (resolved === 'ended') return '已结束'
  if (resolved === 'cancelled') return '已取消'
  return '未开始'
}
