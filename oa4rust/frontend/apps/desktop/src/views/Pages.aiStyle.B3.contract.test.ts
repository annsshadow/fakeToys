// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R300 B3 沟通协作域 AI 风格契约：会议 / 通讯录 / 日程 / 审批四页消费 t1 的 --ai-* 令牌，
 * 保留全部业务 testid、表单与 API 调用，仅要求视图层令牌化与空态引导。
 * 消息中心 IMChat 由 B2 个人域契约覆盖，本文件不重复断言，避免写范围冲突。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string) => readFileSync(resolve(import.meta.dirname, name), 'utf8')
const meeting = read('MeetingApp.vue')
const org = read('OrgDirectoryApp.vue')
const calendar = read('CalendarApp.vue')
const work = read('ProcessWork.vue')
const templateOf = (s: string) => s.slice(s.indexOf('<template>'), s.indexOf('</template>'))

describe('R300 B3 沟通协作域 AI 风格契约', () => {
  it('会议页采用 AI 玻璃 hero、品牌渐变主操作与骨架令牌，空态给出预订引导', () => {
    const tpl = templateOf(meeting)
    expect(tpl).toContain('class="meeting-view modern-page ai-page"')
    expect(tpl).toContain('class="meeting-hero modern-hero ai-panel"')
    expect(tpl).toContain('暂无会议，点击「新建会议」预订第一场')
    expect(meeting).toMatch(/\.ai-page\s+\.meeting-hero\s*\{[^}]*var\(--ai-gradient-surface\)/)
    expect(meeting).toMatch(/\.ai-page\s+\.bs\s*\{[^}]*var\(--ai-gradient-brand\)/)
    expect(meeting).toMatch(/\.ai-page\s+\.sk\s*\{[^}]*var\(--ai-skeleton-shine\)/)
    expect(meeting).toContain('var(--ai-motion-fast)')
    expect(meeting).toContain('var(--ai-ease-spring)')
    for (const s of ['booking', 'mine', 'calendar']) expect(tpl).toContain('class="' + s)
    expect(tpl).toContain('type="date"')
    expect(tpl).toContain('calendar-grid')
  })

  it('通讯录采用 AI 玻璃面板、渐变头像与栅格间距，业务 testid 不丢失', () => {
    const tpl = templateOf(org)
    expect(tpl).toContain('class="dir-view modern-page ai-page"')
    expect(tpl).toContain('class="modern-hero ai-panel"')
    expect(org).toMatch(/\.ai-page\s+\.person-avatar\s*\{[^}]*var\(--ai-gradient-brand\)/)
    expect(org).toMatch(/\.ai-page\s+\.tree-node\s*\{[^}]*var\(--ai-glass-bg\)/)
    expect(org).toMatch(/\.ai-page\s+\.search-form\s*\{[^}]*var\(--ai-grid-2\)/)
    expect(org).toContain('var(--ai-motion-fast)')
    for (const id of ['org-search-input', 'org-department-tree', 'org-person-list', 'org-person-pager']) {
      expect(tpl).toContain('data-testid="' + id + '"')
    }
    expect(tpl).toContain('产品部')
  })

  it('日程页采用 AI 渐变标题、今日高亮与栅格动效，业务区块不丢失', () => {
    const tpl = templateOf(calendar)
    expect(tpl).toContain('class="calendar-view ai-page"')
    expect(tpl).toContain('class="cal-header view-header glass-card ai-panel"')
    expect(calendar).toMatch(/\.ai-page\s+\.view-header\s+h1\s*\{[^}]*var\(--ai-gradient-text\)/)
    expect(calendar).toMatch(/\.ai-page\s+\.day-cell\.today\s*\{[^}]*var\(--ai-gradient-surface\)/)
    expect(calendar).toMatch(/\.ai-page\s+\.days-grid\s*\{[^}]*var\(--ai-grid-1\)/)
    expect(calendar).toContain('var(--ai-ease-spring)')
    for (const id of ['calendar-header', 'calendar-scope-tabs', 'calendar-month-grid', 'calendar-day-events', 'calendar-event-manager']) {
      expect(tpl).toContain('data-testid="' + id + '"')
    }
    expect(tpl).toContain('type="datetime-local"')
  })

  it('审批页采用 AI 表面渐变、玻璃卡片与品牌主按钮，空态给出发起引导', () => {
    const tpl = templateOf(work)
    expect(tpl).toContain('class="work-view modern-page ai-page ai-panel"')
    expect(tpl).toContain('class="work-hero modern-hero ai-panel"')
    expect(tpl).toContain('暂无待办任务，去流程中心发起')
    expect(work).toMatch(/\.ai-page\s+\.work-hero\s*\{[^}]*var\(--ai-gradient-surface\)/)
    expect(work).toMatch(/\.ai-page\s+\.item-card\s*\{[^}]*var\(--ai-glass-bg\)/)
    expect(work).toMatch(/\.ai-page\s+\.btn-sm\.primary\s*\{[^}]*var\(--ai-gradient-brand\)/)
    expect(work).toContain('var(--ai-motion-fast)')
    for (const id of ['work-pending-list', 'work-started-list', 'work-completed-list']) {
      expect(tpl).toContain('data-testid="' + id + '"')
    }
    expect(tpl).not.toContain('/api/')
  })
})
