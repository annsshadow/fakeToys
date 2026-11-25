// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R548 RED→GREEN 契约：H5 浏览页 7 类 @tap 伪按钮键盘可达（WCAG 2.1 SC2.1.1 / 4.1.2）。
 * 方案B逐页显式补 role=button + tabindex=0 + 动态 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * 真父 0b5297d1d 无这些属性 → 核心 RED；守卫业务零改须真父亦通过。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')

const browse = read('pages/browse/browse.vue')
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

describe('R548 H5 浏览页伪按钮键盘可达契约', () => {
  it('页签 v-for 补 a11y 属性并复用 switchTab（动态 label）', () => {
    const tab = lineWith(browse, 'class="tab ai-brand-fill"')
    a11yAttrs(tab)
    expect(tab).toContain(':aria-label="t.label"')
    expect(tab).toContain('@tap="switchTab(t.key)"')
    expect(tab).toContain('@keydown.enter="switchTab(t.key)"')
    expect(tab).toContain('@keydown.space.prevent="switchTab(t.key)"')
  })

  it('两处 retry 补 a11y 属性并复用 load()', () => {
    const retries = linesWith(browse, 'class="retry-btn"')
    expect(retries.length).toBe(2)
    retries.forEach((l: string) => {
      a11yAttrs(l)
      expect(l).toContain('aria-label="重新加载浏览数据"')
      expect(l).toContain('@tap="load()"')
      expect(l).toContain('@keydown.enter="load()"')
    })
  })

  it('空态刷新/去搜索补 a11y 属性并复用 load()/goSearch', () => {
    const refresh = lineWith(browse, 'aria-label="刷新浏览数据"')
    a11yAttrs(refresh)
    expect(refresh).toContain('aria-label="刷新浏览数据"')
    const goSearch = lineWith(browse, '@tap="goSearch"')
    a11yAttrs(goSearch)
    expect(goSearch).toContain('aria-label="去搜索"')
    expect(goSearch).toContain('@keydown.enter="goSearch"')
  })

  it('列表卡片 v-for 补 a11y 属性并复用 open(r)（动态 name）', () => {
    const card = lineWith(browse, 'class="card"')
    a11yAttrs(card)
    expect(card).toContain(':aria-label=')
    expect(card).toContain('nameOf(r)')
    expect(card).toContain('@tap="open(r)"')
    expect(card).toContain('@keydown.enter="open(r)"')
    expect(card).toContain('@keydown.space.prevent="open(r)"')
  })

  it('详情关闭补 a11y 属性并复用 detail = null', () => {
    const close = lineWith(browse, 'class="detail-close"')
    a11yAttrs(close)
    expect(close).toContain('aria-label="关闭详情"')
    expect(close).toContain('@tap="detail = null"')
    expect(close).toContain('@keydown.enter="detail = null"')
  })

  it('守卫：业务 handler 函数体与 API 调用逐字零改', () => {
    expect(browse).toContain('async function load(previousRows: Row[] = rows.value): Promise<void> {')
    expect(browse).toContain('function switchTab(key: TabKey): void {')
    expect(browse).toContain('async function open(r: Row): Promise<void> {')
    expect(browse).toContain('function goSearch() {')
    expect(browse).toContain('portalApi.pageList(')
    expect(browse).toContain('queryviewApi.viewList(')
    expect(browse).toContain('cmsApi.columnList()')
    expect(browse).toContain('listGate.isLatest(token)')
    expect(browse).toContain('detailGate.isLatest(token)')
  })

  it('守卫：所有原 @tap 处理器保留（鼠标零回归）', () => {
    const taps = linesWith(browse, '@tap=')
    expect(taps.length).toBeGreaterThanOrEqual(7)
    expect(browse).toContain('@tap="switchTab(t.key)"')
    expect(browse).toContain('@tap="open(r)"')
    expect(browse).toContain('@tap="detail = null"')
    expect(browse).toContain('@tap="goSearch"')
  })

  it('守卫：main.ts R537 增强器零改且无全局伪按钮增强器', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain("HOST_SELECTOR = 'uni-input, uni-textarea, uni-picker'")
    expect(main).not.toContain('switchTab')
  })

  it('守卫：calendar:295 正面对照与样式段零改', () => {
    expect(cal).toContain('<text class="calendar-create-btn ai-brand-fill" role="button" aria-label="新建日历" @tap="toggleCalendarForm">')
    const style = browse.slice(browse.indexOf('<style'))
    expect(style).not.toContain('role="button"')
    expect(style).not.toContain('@keydown')
  })
})
