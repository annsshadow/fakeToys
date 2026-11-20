// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * 工作台首页必须是任务导向的业务聚合视图，而不是把后端 /api/* 端点平铺成调试按钮。
 * 断言只读取 <template> 段：<script> 里的 API 调用与 handler 锚点保持原样。
 */
describe('Dashboard 工作台业务视图契约', () => {
  const source = readFileSync(resolve(import.meta.dirname, 'Dashboard.vue'), 'utf8')
  const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))

  it('提供「我的待办 / 今日日程 / 通知公告」三个业务区块', () => {
    for (const [title, cls] of [
      ['我的待办', 'todo-section'],
      ['今日日程', 'schedule-section'],
      ['通知公告', 'notice-section'],
    ] as const) {
      expect(template, `缺少业务标题: ${title}`).toContain(title)
      expect(template, `缺少业务区块 class: ${cls}`).toContain(`class="${cls}`)
      expect(template, `缺少业务区块 testid: ${cls}`).toContain(`data-testid="dashboard-${cls.split('-')[0]}-section"`)
    }
  })

  it('我的待办区有列表控件与业务处理入口，不是端点触发按钮', () => {
    const section = template.slice(template.indexOf('class="todo-section'), template.indexOf('class="schedule-section'))
    expect(section).toContain('<ul')
    expect(section).toContain('data-testid="dashboard-todo-list"')
    expect(section).toContain('to="/app/process"')
    expect(section).toContain('@click="navigateTo(')
  })

  it('今日日程区有日历网格与时间槽控件', () => {
    const section = template.slice(
      template.indexOf('class="schedule-section'),
      template.indexOf('class="notice-section'),
    )
    expect(section).toContain('data-testid="dashboard-schedule-grid"')
    expect(section).toContain('slot-time')
    expect(section).toContain('to="/app/calendar"')
  })

  it('通知公告区有消息列表与跳转按钮', () => {
    const section = template.slice(template.indexOf('class="notice-section'))
    expect(section).toContain('data-testid="dashboard-notice-list"')
    expect(section).toContain('data-testid="dashboard-notice-more"')
    expect(section).toContain('to="/app/bbs"')
  })

  it('区块使用语义化中文标题，副标题不暴露 API 路径', () => {
    expect(template).not.toContain('/api/')
    expect((template.match(/<h2>/g) ?? []).length).toBeGreaterThanOrEqual(3)
  })

  it('不渲染端点调试按钮（查询本页 / 管理动作等）', () => {
    expect(template).not.toMatch(/@click="[^"]*(?:loadRecentAll|manage\(|orgAuth\(|runCurrentTab|runPerson|runUnit)/)
    expect(template).not.toContain('▶')
  })
})
