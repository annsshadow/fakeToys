// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R537 RED→GREEN 契约：H5 表单控件可访问名须透传到 AT 实际聚焦的原生控件（WCAG 2.1 SC4.1.2 / 3.3.2）。
 * t36 终审证伪 t35 宿主层方案：aria-label 只落 uni-input 宿主（AX role=generic, focusable=false），
 * AT 聚焦播报的内层 input.uni-input-input（role=textbox, focused=true）name 仍为 null。
 * 核心：main.ts 有全局增强器，把宿主 aria-label 透传到内层可聚焦原生控件。
 * 守卫：t35 宿主层 aria-label 保留作 label 来源；v-model / placeholder / @tap / @change / handler / 模板 / 样式 / :295 正面对照 / 他页零改。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')
const main = read('main.ts')
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

describe('R537 H5 原生控件可访问名透传契约', () => {
  // ===== 核心：增强器存在且透传到内层原生控件（真父无增强器 → RED） =====
  it('main.ts 定义 R537 全局增强器 installH5NativeInputNameA11y', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain('installH5NativeInputNameA11y()')
  })

  it('增强器用 MutationObserver 观察异步插入并透传 aria-label 到内层原生控件', () => {
    const fn = block(main, 'function installH5NativeInputNameA11y', 'export function createApp')
    expect(fn).toContain('MutationObserver')
    expect(fn).toContain('childList: true')
    expect(fn).toContain('subtree: true')
    expect(fn).toContain("HOST_SELECTOR = 'uni-input, uni-textarea, uni-picker'")
    expect(fn).toContain("NATIVE_SELECTOR = 'input, textarea'")
    expect(fn).toContain("setAttribute('aria-label'")
    expect(fn).toContain('closest(HOST_SELECTOR)')
    expect(fn).toContain('querySelectorAll(NATIVE_SELECTOR)')
  })

  it('增强器在 window 守卫内随 createApp 安装（与 desktop 全局增强器同模式）', () => {
    expect(main).toContain("if (typeof window !== 'undefined')")
    const app = block(main, 'export function createApp', 'return { app }')
    expect(app).toContain('installH5NativeInputNameA11y()')
  })

  // ===== 守卫：t35 宿主层 label 来源保留 + 业务零改（真父亦须通过） =====
  it('t35 宿主层 aria-label 保留作透传 label 来源（9 处）', () => {
    expect(lineWith(cal, 'v-model="newCalendarName"')).toContain('aria-label="日历名称"')
    expect(block(cal, ':range="calendars"', '</picker>')).toContain('aria-label="日历选择"')
    expect(lineWith(cal, 'v-model="form.title"')).toContain('aria-label="事件标题"')
    expect(lineWith(cal, 'v-model="form.startTime"')).toContain('aria-label="开始时间"')
    expect(lineWith(cal, 'v-model="form.endTime"')).toContain('aria-label="结束时间"')
    expect(lineWith(cal, 'v-model="form.location"')).toContain('aria-label="地点"')
    expect(lineWith(bbs, ':range="forums.map')).toContain('aria-label="版块"')
    expect(lineWith(bbs, 'v-model="title"')).toContain('aria-label="标题"')
    expect(lineWith(bbs, 'v-model="content"')).toContain('aria-label="内容"')
  })

  it('main.ts 原有业务逐字保留（pinia/SSR/api/errorHandler/unhandledrejection）', () => {
    expect(main).toContain("import { createPinia } from 'pinia'")
    expect(main).toContain("import { createSSRApp } from 'vue'")
    expect(main).toContain("import { setApiBase } from '@/services/http'")
    expect(main).toContain('const app = createSSRApp(App)')
    expect(main).toContain('app.config.errorHandler = (err, _instance, info) =>')
    expect(main).toContain('window.addEventListener(\'unhandledrejection\'')
    expect(main).toContain("import.meta.env.VITE_OA_API_BASE || '/oa4rust-api'")
    expect(main).toContain('app.use(createPinia())')
  })

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

  it('发帖页与业务 handler 零改（仅 main.ts 增强器新增）', () => {
    expect(lineWith(bbs, 'v-model="title"')).toContain('maxlength="100"')
    expect(lineWith(bbs, 'v-model="title"')).toContain('placeholder="请输入标题"')
    expect(lineWith(bbs, 'v-model="content"')).toContain('placeholder="请输入内容"')
    expect(lineWith(bbs, ':range="forums.map')).toContain('@change="onForumChange"')
    expect(bbs).toContain('@tap="submit"')
    expect(cal).toContain('async function submitCalendar(')
    expect(cal).toContain('async function submitEvent(')
    expect(cal).toContain('calendarApi.create')
    expect(bbs).toContain('const canSubmit = () =>')
    expect(bbs).toContain('function onForumChange(')
    expect(bbs).toContain('topicCreate')
  })
})
