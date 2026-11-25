// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R556 RED→GREEN 契约：H5 论坛页 4 类 @tap 伪按钮键盘可达（WCAG 2.1 SC2.1.1 / 4.1.2）。
 * 方案B逐页显式补 role=button + tabindex=0 + 静态/动态 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * 真父 35ddb65a6 无这些属性 → 核心 RED；守卫业务零改须真父亦通过。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')

const bbs = read('pages/bbs/bbs.vue')
const main = read('main.ts')
const calendar = read('pages/calendar/calendar.vue')

function linesWith(source: string, needle: string): string[] {
  return source.split('\n').filter((l: string) => l.includes(needle))
}
/** 提取同时含 a 与 b 的完整开标签块（属性间会插入 a11y，故不能依赖相邻字面量） */
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
  expect(l).toContain('@keydown.enter=')
  expect(l).toContain('@keydown.space.prevent=')
}

describe('R556 H5 论坛页伪按钮键盘可达契约', () => {
  it('发帖按钮补 a11y 并复用 newTopic', () => {
    const b = blockWith2(bbs, 'class="new-btn ai-brand-fill"', '@tap="newTopic"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="发帖"')
    expect(b).toContain('@keydown.enter="newTopic"')
    expect(b).toContain('@keydown.space.prevent="newTopic"')
  })

  it('错误态与有行错误 banner 两处 retry 补 a11y 并复用 load', () => {
    const b = blockWith2(bbs, 'class="retry-btn"', '@tap="load"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="重试"')
    expect(b).toContain('@keydown.enter="load"')
    expect(b).toContain('@keydown.space.prevent="load"')
    expect(linesWith(bbs, 'class="retry-btn"').length).toBeGreaterThanOrEqual(2)
  })

  it('主题列表项补 a11y + 动态 aria-label 并复用 openTopic(row)', () => {
    const b = blockWith2(bbs, 'class="topic o2-list-item"', '@tap="openTopic(row)"')
    a11yAttrs(b)
    expect(b).toContain(':aria-label="\'查看\' + titleOf(row) + \'详情\'"')
    expect(b).toContain('@keydown.enter="openTopic(row)"')
    expect(b).toContain('@keydown.space.prevent="openTopic(row)"')
  })

  it('守卫：业务 handler 函数体逐字零改', () => {
    expect(bbs).toContain('function titleOf(row: Record<string, unknown>): string {')
    expect(bbs).toContain("return typeof v === 'string' && v ? v : '（无标题）'")
    expect(bbs).toContain('async function load() {')
    expect(bbs).toContain('await loadBbsTopics(() => bbsApi.mobileViewAll(), rows.value)')
    expect(bbs).toContain('function openTopic(row: Record<string, unknown>) {')
    expect(bbs).toContain("url: \`/pages/bbs/topic?id=\${encodeURIComponent(id)}\`")
    expect(bbs).toContain('function newTopic() {')
    expect(bbs).toContain("url: '/pages/bbs/new'")
  })

  it('守卫：所有原 @tap 处理器保留（鼠标零回归）+ bbsApi 零改', () => {
    expect(bbs).toContain('@tap="newTopic"')
    expect(bbs).toContain('@tap="load"')
    expect(bbs).toContain('@tap="openTopic(row)"')
    expect(bbs).toContain("from '@/services'")
  })

  it('守卫：main.ts R537 增强器零改且无全局伪按钮增强器', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain('HOST_SELECTOR = ' + "'uni-input, uni-textarea, uni-picker'")
    expect(main).not.toContain('openTopic')
    expect(main).not.toContain('new-btn')
  })

  it('守卫：样式段零改 + calendar:295 正面对照零改', () => {
    const style = bbs.slice(bbs.indexOf('<style'))
    expect(style).not.toContain('role="button"')
    expect(style).not.toContain('tabindex=')
    expect(style).not.toContain('@keydown')
    expect(calendar).toContain('aria-label="新建日历"')
    expect(calendar).toContain('@tap="toggleCalendarForm"')
  })
})