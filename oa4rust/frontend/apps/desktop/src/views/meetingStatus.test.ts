// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { meetingCompletedFromResponse, meetingStatusClass, meetingStatusText } from './meetingStatus'

describe('会议状态渲染兼容后端权威值域（R266）', () => {
  it('会议状态文案兼容后端英文状态值', () => {
    expect(meetingStatusText('completed')).toBe('已结束')
    expect(meetingStatusText('processing')).toBe('进行中')
    expect(meetingStatusText('accepted')).toBe('进行中')
    expect(meetingStatusText('2')).toBe('已结束')
    expect(meetingStatusText('1')).toBe('进行中')
    expect(meetingStatusText(undefined)).toBe('未开始')
  })

  it('状态样式类与文案共用同一权威值域', () => {
    expect(meetingStatusClass('completed')).toBe('ended')
    expect(meetingStatusClass('processing')).toBe('active')
    expect(meetingStatusClass('2')).toBe('ended')
    expect(meetingStatusClass('1')).toBe('active')
    expect(meetingStatusClass(undefined)).toBe('pending')
  })

  it('只认后端 manual/completed 响应里权威布尔 completed=true', () => {
    expect(meetingCompletedFromResponse({ success: true, data: { id: 'm1', completed: true } })).toBe(true)
    expect(meetingCompletedFromResponse({ success: true, data: { id: 'm1', completed: false } })).toBe(false)
    expect(meetingCompletedFromResponse({ success: true, data: {} })).toBe(false)
    expect(meetingCompletedFromResponse(undefined)).toBe(false)
  })
})

describe('MeetingApp 标记完成消费权威态（R266）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'MeetingApp.vue'), 'utf8')
  const markStart = source.indexOf('async function markMeetingCompleted(')
  const markBody =
    markStart > -1 ? source.slice(markStart, source.indexOf('async function showCheckinCode', markStart)) : ''

  it('状态渲染委托给权威值域映射函数', () => {
    expect(source).toContain("from './meetingStatus'")
    const txtStart = source.indexOf('function statusTxt(')
    const txtBody = source.slice(txtStart, source.indexOf('function statusCls', txtStart))
    expect(txtBody).toContain('meetingStatusText')
    expect(txtBody.includes("m.status === '1'")).toBe(false)
  })

  it('标记完成消费响应权威态并就地落态，不重载无 status 的列表', () => {
    expect(markBody).toContain('meetingCompletedFromResponse(r)')
    expect(markBody).toContain("m.status = 'completed'")
    expect(markBody.includes('loadMeetings(')).toBe(false)
  })
})
