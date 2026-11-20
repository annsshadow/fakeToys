// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

// x_meeting.status 的权威值域是英文串（wait/accepted/confirmed/processing/
// completed/denied/rejected），历史列表曾以 '1'/'2' 表示进行中/已结束。
// 列表端点 meeting/list/apply 的 SELECT 不含 status 列，卡片拿不到该字段；
// 标记完成后须消费 manual/completed 响应的 completed 权威布尔值并就地落态，
// 不能依赖重载那份不含 status 的列表。渲染侧兼容多值域，避免误显示「未开始」。

type StatusTone = 'active' | 'ended' | 'pending'

function statusTone(status?: string): StatusTone {
  if (status === '1' || status === 'processing' || status === 'accepted' || status === 'confirmed') return 'active'
  if (status === '2' || status === 'completed' || status === 'denied' || status === 'rejected') return 'ended'
  return 'pending'
}

export function meetingStatusText(status?: string): string {
  const tone = statusTone(status)
  return tone === 'active' ? '进行中' : tone === 'ended' ? '已结束' : '未开始'
}

export function meetingStatusClass(status?: string): string {
  return statusTone(status)
}

export function meetingCompletedFromResponse(resp: unknown): boolean {
  if (typeof resp !== 'object' || resp === null) return false
  const data = (resp as { data?: unknown }).data
  if (typeof data !== 'object' || data === null) return false
  return (data as { completed?: unknown }).completed === true
}
