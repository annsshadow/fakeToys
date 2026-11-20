// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R298 t3-B：会议管理 / 工作流待办两页现代潮流样式契约。
 * node 环境无法加载 .vue SFC，沿用仓库既有源码级契约风格：
 * 断言现代结构类名与关键 data-testid 仍在，且业务流程/端点一个不丢。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string): string => readFileSync(resolve(import.meta.dirname, name), 'utf8')

const h1Of = (src: string): string => {
  const start = src.indexOf('<h1')
  const end = src.indexOf('</h1>', start)
  return end < 0 ? '' : src.slice(start, end)
}
const countOf = (src: string, needle: string): number => src.split(needle).length - 1

const meetingLabels = [
  '+ 新建会议',
  '预订会议室',
  '搜索',
  '加入',
  '参会人',
  '邀请',
  '邀请管理',
  '改时间',
  '标记完成',
  '签到码',
  '取消',
  '创建',
  '关闭',
]
const workLabels = ['发起流程', '刷新待办', '刷新我发起的', '刷新已办', '办理', '查看', '关闭', '重试']
const workEndpoints = [
  '/api/processplatform/assemble/surface/task/list/my/paging/1/size/20',
  '/api/processplatform/assemble/surface/taskcompleted/list/my/paging/1/size/20',
  '/api/processplatform/assemble/surface/work/list/my/paging/1/size/20',
]

describe('R298 t3-B 会议管理 / 工作流待办现代潮流样式', () => {
  const meeting = read('MeetingApp.vue')

  it('会议页套用现代页头/分区/网格/卡片/空态/徽标/统计原语', () => {
    expect(meeting).toContain('meeting-view')
    expect(meeting).toContain('modern-page')
    expect(meeting).toContain('modern-hero')
    expect(meeting).toContain('modern-section')
    expect(meeting).toContain('modern-grid')
    expect(meeting).toContain('modern-card')
    expect(meeting).toContain('modern-empty')
    expect(meeting).toContain('modern-chip')
    expect(meeting).toContain('modern-stat')
    expect(meeting, '预订表单必须网格化').toContain('class="booking-form modern-grid"')
    expect(meeting, '状态徽标使用 modern-chip').toContain('class="ms modern-chip"')
    expect(meeting, '空态使用 modern-empty').toContain('class="es modern-empty"')
  })

  it('会议页保留全部业务按钮、弹窗与端点，标题唯一且不暴露接口', () => {
    for (const label of meetingLabels) {
      expect(meeting, `按钮 ${label} 不能丢`).toContain(label)
    }
    expect(meeting).toContain('v-if="showCreate"')
    expect(meeting).toContain('v-if="detail.open"')
    expect(meeting).toContain('@submit.prevent="createMeeting"')
    expect(meeting).toContain('@click="viewMeeting(m)"')
    expect(meeting).toContain('/api/meeting/')
    expect(countOf(meeting, '<h1'), '页面主标题只能有一处').toBe(1)
    expect(h1Of(meeting), '标题不得暴露 /api/').not.toContain('/api/')
  })

  const work = read('ProcessWork.vue')

  it('工作流列表改为卡片行：流程名/状态/时间为主，原始 id 降级为次级详情', () => {
    expect(work).toContain('work-view')
    expect(work).toContain('modern-page')
    expect(work).toContain('modern-hero')
    expect(work).toContain('modern-section')
    expect(work).toContain('modern-grid')
    expect(work).toContain('modern-card')
    expect(work).toContain('modern-empty')
    expect(work).toContain('modern-chip')
    expect(work).toContain('class="item-card item-body modern-card"')
    expect(work, '卡片行展示流程名').toContain('item.processName')
    expect(work, '卡片行展示创建时间').toContain('item.createTime')
    expect(work, '原始 id 仅作次级详情').toContain('class="item-id muted"')
    expect(work, '主标题不得再把原始 id 当展示名').not.toContain('{{ item.title || item.processName || item.id }}')
    expect(work, '空态使用 modern-empty').toContain('modern-empty')
  })

  it('工作流保留三个 data-testid、弹窗与全部端点，标题唯一且不暴露接口', () => {
    for (const id of ['work-pending-list', 'work-started-list', 'work-completed-list']) {
      expect(work, `data-testid ${id} 必须保留`).toContain(`data-testid="${id}"`)
    }
    for (const label of workLabels) {
      expect(work, `按钮 ${label} 不能丢`).toContain(label)
    }
    expect(work).toContain('v-if="opened"')
    expect(work).toContain('<XformRuntime')
    for (const endpoint of workEndpoints) {
      expect(work, `端点 ${endpoint} 必须保留`).toContain(endpoint)
    }
    expect(countOf(work, '<h1'), '页面主标题只能有一处').toBe(1)
    expect(h1Of(work), '标题不得暴露 /api/').not.toContain('/api/')
  })
})
