// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R494 MeetingApp 列表卡片/日历条目键盘可达性契约（WCAG 2.1.1）：
 * :51 li.mc 与 :79 div.calendar-event 必须 role=button + tabindex=0 + keydown.enter/space 复用 viewMeeting(m)，
 * 原 @click/7 行内按钮 @click.stop/浮层/viewMeeting 函数体零改。真父版 RED、修复版 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'MeetingApp.vue'), 'utf8')

function lineWith(src: string, anchor: string): string {
  const at = src.indexOf(anchor)
  if (at < 0) return ''
  const start = src.lastIndexOf('\n', at) + 1
  const end = src.indexOf('\n', at)
  return src.slice(start, end < 0 ? src.length : end)
}

describe('R494 MeetingApp 卡片键盘可达性契约', () => {
  const card = lineWith(source, 'class="mc modern-card"')
  const calEvent = lineWith(source, 'class="calendar-event"')

  it('两处卡片 role=button + tabindex=0 + keydown.enter/space 复用 viewMeeting（核心）', () => {
    expect(card, '必须找到列表卡片 li.mc').not.toBe('')
    expect(calEvent, '必须找到日历条目 calendar-event').not.toBe('')
    for (const el of [card, calEvent]) {
      expect(el).toMatch(/role="button"/)
      expect(el).toMatch(/tabindex="0"/)
      expect(el).toMatch(/@keydown\.enter="viewMeeting\(m\)"/)
      expect(el).toMatch(/@keydown\.space\.prevent="viewMeeting\(m\)"/)
    }
  })

  it('业务零改守卫：原 @click=viewMeeting(m) 保留在两处卡片', () => {
    expect(card).toContain('@click="viewMeeting(m)"')
    expect(calEvent).toContain('@click="viewMeeting(m)"')
    expect(card).toContain('v-for="m in meetings"')
    expect(calEvent).toContain('v-for="m in day.meetings"')
  })

  it('业务零改守卫：7 行内按钮 @click.stop 全保留且无一调用 viewMeeting', () => {
    const stops = source.match(/@click\.stop="/g) || []
    expect(stops.length).toBe(7)
    for (const h of ['joinMeeting','showParticipants','inviteParticipant','manageInvitee','modifyMeetingTime','markMeetingCompleted','showCheckinCode']) {
      expect(source).toContain('@click.stop="' + h + '(m)"')
    }
    expect(source).not.toMatch(/@click\.stop="viewMeeting/)
  })

  it('业务零改守卫：viewMeeting 函数体与浮层模板保留', () => {
    expect(source).toContain('async function viewMeeting(m: M)')
    expect(source).toContain('const token = detailGeneration.next()')
    expect(source).toContain('detail.value = {')
    expect(source).toContain('v-if="detail.open"')
    expect(source).toContain('会议详情')
    expect(source).toContain('detail.loading')
    expect(source).toContain('detail.controlCount')
  })

  it('业务零改守卫：卡片 class 与关键结构零改', () => {
    expect(card).toContain('class="mc modern-card"')
    expect(calEvent).toContain('class="calendar-event"')
    expect(source).toContain('未来 7 天会议安排（点击日程查看详情）')
    expect((source.match(/viewMeeting\(m\)/g) || []).length).toBeGreaterThanOrEqual(2)
  })
})
