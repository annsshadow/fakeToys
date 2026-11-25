// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R535 RED→GREEN 契约：H5 表单控件必须有非空可访问名（WCAG 2.1 SC4.1.2）。
 * R534 CDP 判定：uni-app H5 的 placeholder 未进入无障碍树的 name，
 * calendar 建日历/事件表单与 bbs 发帖表单多控件 AX name=null。
 * 核心：9 个控件开标签/块内必须有精确 aria-label。
 * 守卫：v-model / placeholder / @tap / @change / handler 全部零改，
 * 且 calendar.vue 第 295 行 role=button + aria-label=新建日历 正面对照保留。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')
const cal = read('pages/calendar/calendar.vue')
const bbs = read('pages/bbs/new.vue')

function lineWith(source: string, needle: string): string {
  const line = source.split('\n').find((l) => l.includes(needle))
  expect(line, '缺少锚点: ' + needle).toBeDefined()
  return line ?? ''
}

function block(source: string, start: string, end: string): string {
  const s = source.indexOf(start)
  expect(s, '缺少起点: ' + start).toBeGreaterThan(-1)
  const e = source.indexOf(end, s)
  expect(e, '缺少终点: ' + end).toBeGreaterThan(s)
  return source.slice(s, e)
}

describe('R535 H5 表单控件可访问名契约', () => {
  // ===== 核心：aria-label 必须存在且文案精确（真父缺失 → RED） =====
  it('日历页 6 个控件均有精确 aria-label', () => {
    expect(lineWith(cal, 'v-model="newCalendarName"')).toContain('aria-label="日历名称"')
    expect(block(cal, ':range="calendars"', '</picker>')).toContain('aria-label="日历选择"')
    expect(lineWith(cal, 'v-model="form.title"')).toContain('aria-label="事件标题"')
    expect(lineWith(cal, 'v-model="form.startTime"')).toContain('aria-label="开始时间"')
    expect(lineWith(cal, 'v-model="form.endTime"')).toContain('aria-label="结束时间"')
    expect(lineWith(cal, 'v-model="form.location"')).toContain('aria-label="地点"')
  })

  it('发帖页 3 个控件均有精确 aria-label', () => {
    expect(lineWith(bbs, ':range="forums.map')).toContain('aria-label="版块"')
    expect(lineWith(bbs, 'v-model="title"')).toContain('aria-label="标题"')
    expect(lineWith(bbs, 'v-model="content"')).toContain('aria-label="内容"')
  })

  // ===== 守卫：正面对照与业务零改（真父亦须通过） =====
  it('正面对照按钮 role=button + aria-label=新建日历 保留', () => {
    const btn = lineWith(cal, 'calendar-create-btn')
    expect(btn).toContain('role="button"')
    expect(btn).toContain('aria-label="新建日历"')
    expect(btn).toContain('@tap="toggleCalendarForm"')
  })

  it('日历页 v-model / placeholder / @tap / picker handler 零改', () => {
    expect(lineWith(cal, 'v-model="newCalendarName"')).toContain('placeholder="日历名称"')
    expect(lineWith(cal, 'v-model="form.title"')).toContain('placeholder="事件标题"')
    expect(lineWith(cal, 'v-model="form.startTime"')).toContain('placeholder="开始 2026-09-22 09:00"')
    expect(lineWith(cal, 'v-model="form.endTime"')).toContain('placeholder="结束 2026-09-22 10:00"')
    expect(lineWith(cal, 'v-model="form.location"')).toContain('placeholder="地点（可选）"')
    expect(cal).toContain('@tap="submitCalendar"')
    expect(cal).toContain('@tap="submitEvent"')
    const picker = block(cal, ':range="calendars"', '</picker>')
    expect(picker).toContain('range-key="name"')
    expect(picker).toContain('form.calendarId = String(c.id')
  })

  it('发帖页 v-model / placeholder / maxlength / @change / @tap 零改', () => {
    expect(lineWith(bbs, 'v-model="title"')).toContain('maxlength="100"')
    expect(lineWith(bbs, 'v-model="title"')).toContain('placeholder="请输入标题"')
    expect(lineWith(bbs, 'v-model="content"')).toContain('placeholder="请输入内容"')
    expect(lineWith(bbs, ':range="forums.map')).toContain('@change="onForumChange"')
    expect(bbs).toContain('@tap="submit"')
  })

  it('业务 handler / 结构锚点零改（仅模板标记变更）', () => {
    expect(cal).toContain('async function submitCalendar(')
    expect(cal).toContain('async function submitEvent(')
    expect(cal).toContain('calendarApi.create')
    expect(bbs).toContain('const canSubmit = () =>')
    expect(bbs).toContain('function onForumChange(')
    expect(bbs).toContain('topicCreate')
  })
})
