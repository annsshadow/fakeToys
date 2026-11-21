// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R311 RED→GREEN 契约：H5 日历页必须提供可见的“建日历”入口。
 *
 * 背景（t69/t70 R310 审计）：干净数据下 calendars=[]，原页面只有“+ 新建”
 * 事件表单，仍需先选 calendarId 才能保存事件；用户无法从 H5 自己创建日历，
 * 导致“保存事件”这条业务链路在空数据下不可达。本契约钉死三件事：
 *   1. services 层暴露 calendarApi.create，打到与 list/my 同源的 assemble_control/calendar；
 *   2. calendar.vue 模板有可见的“+新建日历”入口与最小建日历表单；
 *   3. 创建成功后日历列表 +1 且 form.calendarId 指向新日历（不回退 t25 信封归一化）。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(__dirname, '../..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')
const services = read('services/index.ts')
const page = read('pages/calendar/calendar.vue')

describe('R311 H5 建日历入口契约', () => {
  it('services 暴露 calendarApi.create 且打到真实后端端点', () => {
    expect(services).toMatch(/export const calendarApi = \{[\s\S]*?\bcreate:/)
    expect(services).toContain("'/api/calendar_assemble_control/calendar'")
    expect(services).not.toContain("'/api/calendar/core/entity/calendar/create'")
    expect(services).toMatch(/create:\s*\([^)]*\)\s*=>\s*mapi\.post/)
  })

  it('日历页有可见的“+新建日历”入口', () => {
    expect(page).toContain('+新建日历')
    expect(page).toMatch(/class="[^"]*calendar-create-btn[^"]*"/)
  })

  it('日历页有最小建日历表单（名称输入 + 提交）', () => {
    expect(page).toContain('newCalendarName')
    expect(page).toContain('placeholder="日历名称"')
    expect(page).toContain('@tap="submitCalendar"')
    expect(page).toMatch(/async function submitCalendar\s*\(/)
  })

  it('成功后调用 calendarApi.create、列表 +1 并选中新日历', () => {
    expect(page).toMatch(/async function submitCalendar[\s\S]{0,800}calendarApi\.create\(/)
    expect(page).toMatch(/async function submitCalendar[\s\S]{0,1200}calendars\.value\s*=\s*\[/)
    expect(page).toMatch(/async function submitCalendar[\s\S]{0,1200}form\.value\.calendarId\s*=\s*String\(/)
    expect(page).toContain('void load()')
  })

  it('不回退 t25 信封归一化与 t51 品牌锚点', () => {
    expect(page).toContain("import { normalizeCalendarRows } from './calendarEnvelope'")
    expect(page).toContain('calendars.value = normalizeCalendarRows(cals.data)')
    expect(page).toContain('class="tabs ai-brand-fill"')
  })
})
