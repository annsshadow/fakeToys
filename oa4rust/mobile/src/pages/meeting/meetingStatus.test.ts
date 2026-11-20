// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { describe, expect, it } from 'vitest'
import { meetingStatusText } from './meetingStatus'

const now = new Date('2026-01-15T12:00:00')

describe('移动端会议状态展示（R290）', () => {
  it('权威英文状态值域映射为进行中/已结束/未开始', () => {
    expect(meetingStatusText({ status: 'processing' }, now)).toBe('进行中')
    expect(meetingStatusText({ status: 'accepted' }, now)).toBe('进行中')
    expect(meetingStatusText({ status: 'confirmed' }, now)).toBe('进行中')
    expect(meetingStatusText({ status: 'completed' }, now)).toBe('已结束')
    expect(meetingStatusText({ status: 'denied' }, now)).toBe('已结束')
    expect(meetingStatusText({ status: 'rejected' }, now)).toBe('已结束')
    expect(meetingStatusText({ status: 'wait' }, now)).toBe('未开始')
  })

  it('历史数字状态值仍兼容', () => {
    expect(meetingStatusText({ status: '1' }, now)).toBe('进行中')
    expect(meetingStatusText({ status: '2' }, now)).toBe('已结束')
    expect(meetingStatusText({ status: '3' }, now)).toBe('已取消')
  })

  it('列表无 status 时按起止时间推断进行中与已结束', () => {
    const time = { startTime: '2026-01-15 10:00:00', endTime: '2026-01-15 14:00:00' }
    expect(meetingStatusText(time, now)).toBe('进行中')
    expect(meetingStatusText({ startTime: '2026-01-15 10:00:00', endTime: '2026-01-15 11:00:00' }, now)).toBe('已结束')
    expect(meetingStatusText({ startTime: '2026-01-15 13:00:00', endTime: '2026-01-15 14:00:00' }, now)).toBe('未开始')
    expect(meetingStatusText({}, now)).toBe('未开始')
    // 权威 status 优先于时间窗，避免 accepted 会议被未来开始时间覆盖为未开始。
    expect(meetingStatusText({ status: 'accepted', startTime: '2026-01-20 10:00:00' }, now)).toBe('进行中')
  })
})
