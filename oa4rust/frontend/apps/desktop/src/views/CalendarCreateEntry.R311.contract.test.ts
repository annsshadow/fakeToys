// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R311 契约：PC 日历页补可见建日历入口，打通已有 calendar/create 端点。
 * 断言入口按钮存在、点击触发 calWriteCE(calCreate)、建日历不依赖 prompt、
 * 成功后刷新我的日历并自动选中、读路径与 ai-page 令牌不回退。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, 'CalendarApp.vue'), 'utf8')
const templateOf = (s: string): string => s.slice(s.indexOf('<template>'), s.lastIndexOf('</template>'))
const scriptOf = (s: string): string => s.slice(s.indexOf('<script'))
const template = templateOf(src)
const script = scriptOf(src)
const scopeAt = template.indexOf('cal-scope')
const scope = template.slice(scopeAt, template.indexOf('calendar-grid', scopeAt))
const calCreateBranch = (): string => {
  const at = script.indexOf("op === 'calCreate'")
  return script.slice(at, script.indexOf("op === 'calUpdate'", at))
}

describe('R311 PC 建日历入口契约', () => {
  it('我的日历区存在可见的 + 新建日历 入口按钮与 data-testid', () => {
    expect(scope, 'cal-scope 区缺新建日历入口').toContain('新建日历')
    expect(scope, '入口按钮缺 data-testid').toContain('data-testid="calendar-create-entry"')
    expect(scope, '入口必须是 button 元素').toContain('<button')
  })

  it('入口点击触发 calWriteCE(calCreate) 且 create 与列表同轨', () => {
    expect(scope, '入口未绑定建日历动作').toContain('calWriteCE')
    expect(scope, '入口传参不是 calCreate').toContain("'calCreate'")
    const branch = calCreateBranch()
    expect(branch, '建日历端点未保留').toContain('/api/calendar/calendar/create')
    expect(branch, 'create 与 list 双轨混用会导致新建日历在「我的日历」不可见').not.toContain('/api/calendar/core/entity/calendar/create')
  })

  it('建日历不再使用 prompt，改为可读表单输入', () => {
    const branch = calCreateBranch()
    expect(branch, 'calCreate 分支仍用 prompt').not.toContain('prompt(')
    expect(script, '缺日历名称表单状态').toContain('calCreateForm')
  })

  it('建日历成功后刷新我的日历列表并自动选中新日历', () => {
    const branch = calCreateBranch()
    expect(branch, '成功后未刷新我的日历列表').toContain("loadCalendars('my')")
    expect(branch, '成功后未自动选中新日历').toContain('selectCalendar(')
  })

  it('我的日历列表正确解析 my_calendars 信封字段', () => {
    expect(script, 'loadCalendars 未处理 my_calendars 信封').toContain('my_calendars')
    expect(script, '列表解析未做数组归一化').toMatch(/Array\.isArray\([^)]*my_calendars/)
  })

  it('既有读路径与 ai-page 令牌不回退', () => {
    expect(template, '根 ai-page 丢失').toContain('class="calendar-view ai-page"')
    expect(script, '我的日历读路径丢失').toContain('/api/calendar/calendar/list/my')
    expect(script, '事件创建读路径丢失').toContain('/api/calendar/event/create')
  })
})
