// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R546 RED→GREEN 契约：H5 通讯录页 9 类 <view>/<text> @tap 伪按钮键盘可达（WCAG 2.1 SC2.1.1 / 4.1.2）。
 * 逐页显式补 role=button + tabindex=0 + 动态 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * 真父 033f05bb2 无这些属性 → 核心 RED；守卫业务零改须真父亦通过。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')

const contacts = read('pages/contacts/contacts.vue')
const main = read('main.ts')
const cal = read('pages/calendar/calendar.vue')

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

describe('R546 H5 通讯录伪按钮键盘可达契约', () => {
  it('3 个 mode tab 补 a11y 属性并复用 switchMode', () => {
    const modes = linesWith(contacts, 'class="mode"')
    expect(modes.length).toBe(3)
    expect(modes[0]).toContain('aria-label="人员"')
    expect(modes[1]).toContain('aria-label="单位"')
    expect(modes[2]).toContain('aria-label="群组"')
    modes.forEach((l: string) => {
      a11yAttrs(l)
      expect(l).toContain('@tap="switchMode(')
      expect(l).toContain('@keydown.enter="switchMode(')
    })
  })

  it('人员 retry 与人员项 body 补 a11y 属性', () => {
    const personRetry = lineWith(contacts, '@tap="search()"')
    a11yAttrs(personRetry)
    expect(personRetry).toContain('aria-label="重试加载联系人"')
    expect(personRetry).toContain('@keydown.enter="search()"')
    const personBody = lineWith(contacts, '@tap="openPerson(row)"')
    a11yAttrs(personBody)
    expect(personBody).toContain(':aria-label=')
    expect(personBody).toContain('nameOf(row)')
    expect(personBody).toContain('@keydown.enter="openPerson(row)"')
  })

  it('单位 retry 与单位项 body 补 a11y 属性', () => {
    const orgRetry = lineWith(contacts, '@tap="loadOrg()"')
    a11yAttrs(orgRetry)
    expect(orgRetry).toContain(':aria-label=')
    expect(orgRetry).toContain('@keydown.enter="loadOrg()"')
    const unitBody = lineWith(contacts, "mode === 'unit' ? openUnit(o) : undefined")
    a11yAttrs(unitBody)
    expect(unitBody).toContain('orgNameOf(o)')
    expect(unitBody).toContain('@keydown.space.prevent=')
  })

  it('单位人员详情关闭/retry 与人员项 body 补 a11y 属性', () => {
    const closeUnit = lineWith(contacts, '@tap="closeUnitPeople"')
    a11yAttrs(closeUnit)
    expect(closeUnit).toContain('aria-label="关闭单位人员详情"')
    const unitRetry = lineWith(contacts, '@tap="openUnit(unitPeopleTarget)"')
    a11yAttrs(unitRetry)
    expect(unitRetry).toContain('aria-label="重试加载单位人员"')
    const unitPersonBody = lineWith(contacts, '@tap="openPerson(p)"')
    a11yAttrs(unitPersonBody)
    expect(unitPersonBody).toContain('nameOf(p)')
  })

  it('人员详情关闭与 retry 补 a11y 属性', () => {
    const closeDetail = lineWith(contacts, '@tap="detail = null"')
    a11yAttrs(closeDetail)
    expect(closeDetail).toContain('aria-label="关闭人员详情"')
    const detailRetry = lineWith(contacts, '@tap="openPerson(detail)"')
    a11yAttrs(detailRetry)
    expect(detailRetry).toContain('aria-label="重试加载人员详情"')
  })

  it('守卫：业务 handler 函数体逐字零改', () => {
    expect(contacts).toContain('async function search() {')
    expect(contacts).toContain('orgApi.personSearch(keyword.value.trim() || undefined)')
    expect(contacts).toContain('async function loadOrg(): Promise<void> {')
    expect(contacts).toContain('function switchMode(next: Mode): void {')
    expect(contacts).toContain('async function openPerson(row: Record<string, unknown>): Promise<void> {')
    expect(contacts).toContain('async function openUnit(row: Record<string, unknown>): Promise<void> {')
    expect(contacts).toContain('function closeUnitPeople(): void {')
    expect(contacts).toContain('async function startChat(row: Record<string, unknown>): Promise<void> {')
  })

  it('守卫：业务 API 调用与 gate 逻辑零改', () => {
    expect(contacts).toContain('orgApi.unitList()')
    expect(contacts).toContain('orgApi.groupList()')
    expect(contacts).toContain('orgApi.personIdentities(flag)')
    expect(contacts).toContain('orgApi.personRoles(flag)')
    expect(contacts).toContain('orgApi.personByUnitDirect([unit])')
    expect(contacts).toContain('personDetailGate.isLatest(token)')
    expect(contacts).toContain('unitPeopleGate.isLatest(token)')
  })

  it('守卫：原生 uni-button :204/:234 逐字零改', () => {
    expect(contacts).toContain('class="go ai-brand-fill" size="mini" type="primary" :disabled="loading" @tap="doSearch"')
    expect(contacts).toContain('class="chat-btn" size="mini" type="primary" :disabled=')
    expect(contacts).toContain('@tap="startChat(row)"')
  })

  it('守卫：搜索 input aria-label/v-model 与所有原 @tap 保留', () => {
    expect(contacts).toContain('v-model="keyword"')
    expect(contacts).toContain('aria-label="通讯录搜索关键词"')
    expect(contacts).toContain('@confirm="doSearch"')
    const taps = linesWith(contacts, '@tap=')
    expect(taps.length).toBeGreaterThanOrEqual(12)
  })

  it('守卫：main.ts R537 增强器零改且无全局伪按钮增强器', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain("HOST_SELECTOR = 'uni-input, uni-textarea, uni-picker'")
    expect(main).not.toContain('switchMode')
  })

  it('守卫：calendar:295 正面对照与样式段零改', () => {
    expect(cal).toContain('<text class="calendar-create-btn ai-brand-fill" role="button" aria-label="新建日历" @tap="toggleCalendarForm">')
    const style = contacts.slice(contacts.indexOf('<style'))
    expect(style).not.toContain('role="button"')
    expect(style).not.toContain('@keydown')
  })
})
