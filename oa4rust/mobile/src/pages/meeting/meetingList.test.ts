// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'
import { MEETING_PAGE_SIZE, meetingHasMore, mergeMeetingRows } from './meetingList'

describe('会议列表分页（R149）', () => {
  it('按每页大小判定是否还有下一页', () => {
    expect(MEETING_PAGE_SIZE).toBe(50)
    expect(meetingHasMore(50)).toBe(true)
    expect(meetingHasMore(49)).toBe(false)
    expect(meetingHasMore(0)).toBe(false)
    expect(meetingHasMore(51, 60)).toBe(false)
  })

  it('追加下一页时按 id 去重，保留无 id 的行', () => {
    const first = [{ id: 'a' }, { id: 'b' }]
    const second = [{ id: 'b' }, { id: 'c' }]
    expect(mergeMeetingRows(first, second)).toEqual([{ id: 'a' }, { id: 'b' }, { id: 'c' }])
    const noId = [{ title: 'x' }, { id: 'd' }]
    expect(mergeMeetingRows(noId, [{ title: 'y' }])).toEqual([{ title: 'x' }, { id: 'd' }, { title: 'y' }])
  })

  it('页面按页请求并在底部提供加载更多与到底提示', () => {
    const source = readFileSync(resolve(import.meta.dirname, 'meeting.vue'), 'utf8')
    // 规模下限：防止文件被截断后断言空转。
    expect(source.split('\n').length).toBeGreaterThan(250)
    expect(source).toContain("from './meetingList'")
    expect(source).toContain('mergeMeetingRows')
    expect(source).toContain('meetingHasMore')
    expect(source).toContain("hasMore.value = tab.value === 'mine' && result.ok && meetingHasMore(result.rows.length)")
    expect(source).toContain('rows.value = mergeMeetingRows(rows.value, result.rows)')
    expect(source).toContain('hasMore.value = meetingHasMore(result.rows.length)')
    expect(source).toContain('function loadMore')
    expect(source).toContain('@tap="loadMore"')
    expect(source).toContain('加载更多')
    expect(source).toContain('已加载')
    // 旧实现：固定请求第 1 页 50 条，超出部分不可达。
    expect(source).not.toContain('meetingApi.applied(1, 50)')
    expect(source).toContain('meetingApi.applied(1, MEETING_PAGE_SIZE)')
    expect(source).toContain('meetingApi.applied(next, MEETING_PAGE_SIZE)')
  })
})
