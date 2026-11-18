// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

describe('会议管理详情与会议室列表只认最新一次（R244）', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'MeetingApp.vue'), 'utf8')
  const viewStart = source.indexOf('async function viewMeeting(')
  const viewBody =
    viewStart > -1 ? source.slice(viewStart, source.indexOf('async function showParticipants', viewStart)) : ''
  const roomListStart = source.indexOf('async function loadRoomList(')
  const roomListBody =
    roomListStart > -1 ? source.slice(roomListStart, source.indexOf('async function addRoom', roomListStart)) : ''
  const formRoomStart = source.indexOf('async function loadRooms(')
  const formRoomBody =
    formRoomStart > -1 ? source.slice(formRoomStart, source.indexOf('async function loadMeetings', formRoomStart)) : ''

  it('竞态真实可达：会议卡片/楼栋下拉在加载中仍可点击，不是被 disabled 挡掉的假竞态', () => {
    const cardLine = source.split('\n').find((l) => l.includes('@click="viewMeeting(m)"')) ?? ''
    expect(cardLine).toContain('viewMeeting')
    expect(cardLine.includes('disabled')).toBe(false)
    // 详情浮层加载中可被背景点击/关闭按钮关掉，再点另一张卡片即产生并发详情请求。
    expect(viewBody.includes('if (detail.value.loading)')).toBe(false)
    const selectLine = source.split('\n').find((l) => l.includes('@change="loadRoomList"')) ?? ''
    expect(selectLine).toContain('loadRoomList')
    expect(selectLine.includes('disabled')).toBe(false)
    const formSelectLine = source.split('\n').find((l) => l.includes('@change="loadRooms"')) ?? ''
    expect(formSelectLine).toContain('loadRooms')
    expect(formSelectLine.includes('disabled')).toBe(false)
  })

  it('viewMeeting() 复用 latestOnly 代际令牌，在发起请求前登记', () => {
    expect(source).toContain("from './latestOnly'")
    expect(source).toContain('const detailGeneration = createGeneration()')
    const begin = viewBody.indexOf('const token = detailGeneration.next()')
    expect(begin).toBeGreaterThan(-1)
    expect(begin).toBeLessThan(viewBody.indexOf('await Promise.all'))
  })

  it('过期详情响应不写 detail 字段，也不提前复位新请求的 loading', () => {
    const guard = viewBody.indexOf('if (!detailGeneration.isCurrent(token)) return')
    expect(guard).toBeGreaterThan(-1)
    expect(viewBody.indexOf('detail.value.title =')).toBeGreaterThan(guard)
    expect(viewBody.indexOf('detail.value.controlCount =')).toBeGreaterThan(guard)
    expect(viewBody.indexOf('detail.value.extra =')).toBeGreaterThan(guard)
    expect(viewBody.indexOf('detail.value.loading = false')).toBeGreaterThan(guard)
  })

  it('loadRoomList() 快速切换楼栋时，旧楼栋响应不覆盖 roomList/roomError', () => {
    expect(source).toContain('const roomListGeneration = createGeneration()')
    const begin = roomListBody.indexOf('const token = roomListGeneration.next()')
    expect(begin).toBeGreaterThan(-1)
    expect(begin).toBeLessThan(roomListBody.indexOf('await api.get'))
    const guard = roomListBody.indexOf('if (!roomListGeneration.isCurrent(token)) return')
    expect(guard).toBeGreaterThan(-1)
    expect(roomListBody.indexOf('roomList.value = (r.data')).toBeGreaterThan(guard)
    // 失败分支同理：过期错误不得把新楼栋的错误态改掉。
    const catchBlock = roomListBody.slice(roomListBody.indexOf('catch ('))
    expect(catchBlock.indexOf('if (!roomListGeneration.isCurrent(token)) return')).toBeGreaterThan(-1)
    expect(catchBlock.indexOf('roomError.value =')).toBeGreaterThan(
      catchBlock.indexOf('if (!roomListGeneration.isCurrent(token)) return'),
    )
  })

  it('loadRooms() 新建表单快速切换楼栋时，旧响应不覆盖 rooms 选项', () => {
    expect(source).toContain('const formRoomGeneration = createGeneration()')
    const begin = formRoomBody.indexOf('const token = formRoomGeneration.next()')
    expect(begin).toBeGreaterThan(-1)
    expect(begin).toBeLessThan(formRoomBody.indexOf('await api.get'))
    const guard = formRoomBody.indexOf('if (!formRoomGeneration.isCurrent(token)) return')
    expect(guard).toBeGreaterThan(-1)
    // await 之后的落地写必须在门后（早退分支的同步清空不受影响）。
    expect(formRoomBody.lastIndexOf('rooms.value =')).toBeGreaterThan(guard)
  })
})
