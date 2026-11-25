// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R550 RED→GREEN 契约：H5 日历页 9 类 @tap 伪按钮键盘可达（WCAG 2.1 SC2.1.1 / 4.1.2）。
 * 方案B逐页显式补 role=button + tabindex=0 + 动态 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * 真父 4c72d0cfe 无这些属性 → 核心 RED；守卫业务零改须真父亦通过。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')

const cal = read('pages/calendar/calendar.vue')
const main = read('main.ts')

function linesWith(source: string, needle: string): string[] {
  return source.split('\n').filter((l: string) => l.includes(needle))
}
function lineWith(source: string, needle: string): string {
  const line = source.split('\n').find((l: string) => l.includes(needle))
  expect(line, '缺少锚点: ' + needle).toBeDefined()
  return line ?? ''
}
function a11yAttrs(l: string): void {
  expect(l).toContain('role="button"')
  expect(l).toContain('tabindex="0"')
  expect(l).toMatch(/aria-label=/)
  expect(l).toContain('@keydown.enter=')
  expect(l).toContain('@keydown.space.prevent=')
}

describe('R550 H5 日历页伪按钮键盘可达契约', () => {
  it('范围页签 v-for 补 a11y 属性并复用 switchRange', () => {
    const tabs = linesWith(cal, 'class="tab ai-brand-fill"')
    expect(tabs.length).toBe(1)
    const l = tabs[0]
    a11yAttrs(l)
    expect(l).toContain(':aria-label="r.label"')
    expect(l).toContain('@tap="switchRange(r.key)"')
    expect(l).toContain('@keydown.enter="switchRange(r.key)"')
    expect(l).toContain('@keydown.space.prevent="switchRange(r.key)"')
  })

  it('新建事件 compose 补 a11y 属性并复用 toggleCompose', () => {
    const l = lineWith(cal, 'class="compose-btn"')
    a11yAttrs(l)
    expect(l).toContain(':aria-label=')
    expect(l).toContain('@tap="toggleCompose"')
    expect(l).toContain('@keydown.enter="toggleCompose"')
    expect(l).toContain('@keydown.space.prevent="toggleCompose"')
  })

  it('创建日历 c-submit 补 a11y 属性并复用 submitCalendar', () => {
    const l = lineWith(cal, '@tap="submitCalendar"')
    a11yAttrs(l)
    expect(l).toContain('aria-label="创建日历"')
    expect(l).toContain('@keydown.enter="submitCalendar"')
  })

  it('保存事件 c-submit 补 a11y 属性并复用 submitEvent', () => {
    const l = lineWith(cal, '@tap="submitEvent"')
    a11yAttrs(l)
    expect(l).toContain('aria-label="保存事件"')
    expect(l).toContain('@keydown.enter="submitEvent"')
  })

  it('两处 retry 补 a11y 属性并复用 load', () => {
    const retries = linesWith(cal, 'class="retry-btn"')
    expect(retries.length).toBe(2)
    retries.forEach((l: string) => {
      a11yAttrs(l)
      expect(l).toContain('aria-label="重新加载日程"')
      expect(l).toContain('@tap="load()"')
      expect(l).toContain('@keydown.enter="load()"')
      expect(l).toContain('@keydown.space.prevent="load()"')
    })
  })

  it('事件卡片 v-for 补 a11y 属性并复用 openEvent', () => {
    const l = lineWith(cal, '@tap="openEvent(e)"')
    a11yAttrs(l)
    expect(l).toContain(':aria-label=')
    expect(l).toContain('e.title')
    expect(l).toContain('@keydown.enter="openEvent(e)"')
    expect(l).toContain('@keydown.space.prevent="openEvent(e)"')
  })

  it('详情关闭补 a11y 属性并复用 detail = null', () => {
    const l = lineWith(cal, '@tap="detail = null"')
    a11yAttrs(l)
    expect(l).toContain('aria-label="关闭日程详情"')
    expect(l).toContain('@keydown.enter="detail = null"')
    expect(l).toContain('@keydown.space.prevent="detail = null"')
  })

  it('删除事件补 a11y 属性并复用 removeEvent', () => {
    const l = lineWith(cal, '@tap="removeEvent(detail)"')
    a11yAttrs(l)
    expect(l).toContain(':aria-label=')
    expect(l).toContain('detail.title')
    expect(l).toContain('@keydown.enter="removeEvent(detail)"')
    expect(l).toContain('@keydown.space.prevent="removeEvent(detail)"')
  })

  it('守卫：业务 handler 函数体逐字零改', () => {
    expect(cal).toContain('function toggleCalendarForm(): void {')
    expect(cal).toContain('async function submitCalendar(): Promise<void> {')
    expect(cal).toContain('function toggleCompose(): void {')
    expect(cal).toContain('async function submitEvent(): Promise<void> {')
    expect(cal).toContain('async function removeEvent(e: EventRow): Promise<void> {')
    expect(cal).toContain('async function load(): Promise<void> {')
    expect(cal).toContain('function switchRange(key: RangeKey): void {')
    expect(cal).toContain('async function openEvent(e: EventRow): Promise<void> {')
  })

  it('守卫：calendarApi 调用与状态字段零改', () => {
    expect(cal).toContain('calendarApi.create(name)')
    expect(cal).toContain('calendarApi.coreEventCreate({')
    expect(cal).toContain('calendarApi.coreEventRemove(id)')
    expect(cal).toContain('calendarApi.myCalendars()')
    expect(cal).toContain('calendarApi.eventsFilter({ calendarId: c.id })')
    expect(cal).toContain('calendarApi.eventDetail(id)')
    expect(cal).toContain('const showCalendarForm = ref(false)')
    expect(cal).toContain('const newCalendarName = ref')
  })

  it('守卫：同页 :295 正面对照与表单 input 零改', () => {
    expect(cal).toContain('<text class="calendar-create-btn ai-brand-fill" role="button" aria-label="新建日历" @tap="toggleCalendarForm">+新建日历</text>')
    expect(cal).toContain('v-model="newCalendarName"')
    expect(cal).toContain('aria-label="日历名称"')
    expect(cal).toContain('aria-label="日历选择"')
    expect(cal).toContain('aria-label="开始时间"')
    expect(cal).toContain('aria-label="地点"')
  })

  it('守卫：所有原 @tap 处理器保留（鼠标零回归）', () => {
    const taps = linesWith(cal, '@tap=')
    expect(taps.length).toBeGreaterThanOrEqual(10)
    expect(cal).toContain('@tap="toggleCalendarForm"')
    expect(cal).toContain('@tap="load()"')
    expect(cal).toContain('@tap="openEvent(e)"')
  })

  it('守卫：main.ts R537 增强器零改且无全局伪按钮增强器', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain("HOST_SELECTOR = 'uni-input, uni-textarea, uni-picker'")
    expect(main).not.toContain('switchRange')
  })

  it('守卫：样式段零改且未被注入 role/tabindex', () => {
    const style = cal.slice(cal.indexOf('<style'))
    expect(style).not.toContain('role="button"')
    expect(style).not.toContain('tabindex=')
    expect(style).not.toContain('@keydown')
  })
})
