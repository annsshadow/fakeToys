// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R558 RED→GREEN 契约：H5 收藏页 6 类 @tap 伪按钮键盘可达（WCAG 2.1 SC2.1.1 / 4.1.2）。
 * 方案B逐页显式补 role=button + tabindex=0 + 静态/动态 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * :107 删除按钮 keydown 追加 .stop 与 @tap.stop 冒泡语义等价；业务零改。
 * 真父 1f24c1440 无这些属性 → 核心 RED；守卫业务零改须真父亦通过。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')

const collect = read('pages/collect/collect.vue')
const main = read('main.ts')
const calendar = read('pages/calendar/calendar.vue')

function linesWith(source: string, needle: string): string[] {
  return source.split('\n').filter((l: string) => l.includes(needle))
}
function blockWith2(source: string, a: string, b: string): string {
  const all = source.split('\n')
  const i = all.findIndex((l: string) => l.includes(a) && l.includes(b))
  expect(i, '缺少锚点: ' + a + ' + ' + b).toBeGreaterThanOrEqual(0)
  const out: string[] = []
  for (let k = i; k < Math.min(i + 12, all.length); k++) {
    out.push(all[k])
    if (all[k].trimEnd().endsWith('>')) break
  }
  return out.join('\n')
}
function a11yAttrs(l: string): void {
  expect(l).toContain('role="button"')
  expect(l).toContain('tabindex="0"')
  expect(l).toMatch(/aria-label=/)
  expect(l).toContain('@keydown.enter')
  expect(l).toContain('@keydown.space')
}

describe('R558 H5 收藏页伪按钮键盘可达契约', () => {
  it('错误态与有行错误 banner 两处 retry 补 a11y 并复用 load', () => {
    const b = blockWith2(collect, 'class="retry-btn"', '@tap="load"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="重试"')
    expect(b).toContain('@keydown.enter="load"')
    expect(b).toContain('@keydown.space.prevent="load"')
    expect(linesWith(collect, 'class="retry-btn"').length).toBeGreaterThanOrEqual(2)
  })

  it('空态常驻-去浏览内容补 a11y 并复用 goBrowse', () => {
    const b = blockWith2(collect, 'class="empty-guide-link ai-pressable primary"', '@tap="goBrowse"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="去浏览内容"')
    expect(b).toContain('@keydown.enter="goBrowse"')
    expect(b).toContain('@keydown.space.prevent="goBrowse"')
  })

  it('空态常驻-刷新补 a11y 并复用 load', () => {
    const b = blockWith2(collect, 'class="empty-guide-link ai-pressable"', '@tap="load"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="刷新"')
    expect(b).toContain('@keydown.enter="load"')
    expect(b).toContain('@keydown.space.prevent="load"')
  })

  it('收藏列表项补 a11y + 动态 aria-label 并复用 openUrl(r)', () => {
    const b = blockWith2(collect, 'class="item"', '@tap="openUrl(r)"')
    a11yAttrs(b)
    expect(b).toContain(':aria-label="\'查看\' + r.title + \'详情\'"')
    expect(b).toContain('@keydown.enter="openUrl(r)"')
    expect(b).toContain('@keydown.space.prevent="openUrl(r)"')
  })

  it('删除项补 a11y + 动态 aria-label，复用 removeRow(r) 且保留 .stop 冒泡语义', () => {
    const b = blockWith2(collect, 'class="item-del"', '@tap.stop="removeRow(r)"')
    a11yAttrs(b)
    expect(b).toContain(':aria-label="\'删除\' + r.title"')
    expect(b).toContain('@keydown.enter.stop="removeRow(r)"')
    expect(b).toContain('@keydown.space.stop.prevent="removeRow(r)"')
  })

  it('守卫：业务 handler 函数体逐字零改 + collectApi 零改', () => {
    expect(collect).toContain('async function load() {')
    expect(collect).toContain('await loadCollectRows(() => collectApi.list(), session.user.unique, rows.value)')
    expect(collect).toContain('function openUrl(row: { url: string }) {')
    expect(collect).toContain('uni.setClipboardData({')
    expect(collect).toContain('title: ' + "'链接已复制'")
    expect(collect).toContain('async function removeRow(row: { id: string; title: string }) {')
    expect(collect).toContain('await collectApi.remove(row.id)')
    expect(collect).toContain('function goBrowse() {')
    expect(collect).toContain("uni.navigateTo({ url: '/pages/browse/browse' })")
    expect(collect).toContain("import { collectApi } from '@/services'")
  })

  it('守卫：所有原 @tap 处理器保留（含 .stop，鼠标零回归）', () => {
    expect(collect).toContain('@tap="load"')
    expect(collect).toContain('@tap="goBrowse"')
    expect(collect).toContain('@tap="openUrl(r)"')
    expect(collect).toContain('@tap.stop="removeRow(r)"')
  })

  it('守卫：main.ts R537 增强器零改且无全局伪按钮增强器', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain('HOST_SELECTOR = ' + "'uni-input, uni-textarea, uni-picker'")
    expect(main).not.toContain('openUrl')
    expect(main).not.toContain('removeRow')
  })

  it('守卫：样式段零改 + calendar:295 正面对照零改', () => {
    const style = collect.slice(collect.indexOf('<style'))
    expect(style).not.toContain('role="button"')
    expect(style).not.toContain('tabindex=')
    expect(style).not.toContain('@keydown')
    expect(calendar).toContain('aria-label="新建日历"')
    expect(calendar).toContain('@tap="toggleCalendarForm"')
  })
})
