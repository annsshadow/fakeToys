// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

/**
 * A 类页面业务化契约：工作日历默认视图必须是任务导向的日程管理界面，
 * 而不是把 /api/* 端点平铺成调试按钮。断言只取 <template> 段，
 * <script setup> 的 API 调用与 handler 定义完整保留。
 */
describe('CalendarApp 业务视图契约', () => {
  const source = readFileSync(fileURLToPath(new URL('./CalendarApp.vue', import.meta.url)), 'utf8')
  const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))

  it('默认视图提供日历头部、范围切换、月历网格与日程业务区块', () => {
    expect(template).toContain('工作日历')
    for (const id of [
      'calendar-header',
      'calendar-scope-tabs',
      'calendar-month-grid',
      'calendar-day-events',
      'calendar-event-manager',
    ]) {
      expect(template, `缺少业务区块 data-testid: ${id}`).toContain(`data-testid="${id}"`)
    }
  })

  it('月历网格保留星期头与日期单元格', () => {
    expect(template).toContain('calendar-grid')
    expect(template).toContain('weekday-row')
    expect(template).toContain('days-grid')
    expect(template).toMatch(/v-for="cell in calendarCells"/)
    expect(template).toContain('class="day-cell"')
  })

  it('日程管理区有新建日程表单与当日日程列表', () => {
    const compose = template.slice(template.indexOf('cal-compose'))
    expect(compose).toContain('<form')
    expect(compose).toContain('type="datetime-local"')
    expect(compose).toMatch(/<button[^>]*type="submit"/)
    expect(template).toContain('当日日程')
  })

  it('日历范围切换为我的/公共日历 tab，非端点按钮', () => {
    expect(template).toContain('我的日历')
    expect(template).toContain('公共日历')
    expect(template).toMatch(/role="tablist"/)
  })

  it('默认视图不渲染 /api/* 端点调试按钮，副标题不暴露 API 路径', () => {
    expect(template).not.toContain('/api/')
    for (const handler of ['loadCalSettings', 'calWrite', 'calMore', 'calWriteCE', 'loadCalCoreEntities']) {
      expect(template, `端点调试 handler 不应出现在模板: ${handler}`).not.toContain(handler)
    }
  })
})
