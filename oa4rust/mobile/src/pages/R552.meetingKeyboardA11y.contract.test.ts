// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R552 RED→GREEN 契约：H5 会议页 6 类 @tap 伪按钮键盘可达（WCAG 2.1 SC2.1.1 / 4.1.2）。
 * 方案B逐页显式补 role=button + tabindex=0 + 动态 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * 真父 16ed9784e 无这些属性 → 核心 RED；守卫业务零改须真父亦通过。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')

const meet = read('pages/meeting/meeting.vue')
const main = read('main.ts')

function linesWith(source: string, needle: string): string[] {
  return source.split('\n').filter((l: string) => l.includes(needle))
}
function lineWith(source: string, needle: string): string {
  const line = source.split('\n').find((l: string) => l.includes(needle))
  expect(line, '缺少锚点: ' + needle).toBeDefined()
  return line ?? ''
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
/** 提取起始锚点所在的完整开标签块（多行，如页签 v-for） */
function blockWith(source: string, a: string): string {
  const all = source.split('\n')
  const i = all.findIndex((l: string) => l.includes(a))
  expect(i, '缺少锚点: ' + a).toBeGreaterThanOrEqual(0)
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

describe('R552 H5 会议页伪按钮键盘可达契约', () => {
  it('页签 v-for 补 a11y 属性并复用 switchTab', () => {
    const b = blockWith(meet, 'class="tab ai-brand-fill"')
    a11yAttrs(b)
    expect(b).toContain(':aria-label="t.label"')
    expect(b).toContain('@keydown.enter="switchTab(t.key)"')
    expect(b).toContain('@keydown.space.prevent="switchTab(t.key)"')
  })

  it('错误态 retry 补 a11y 属性并复用 load()', () => {
    const b = blockWith2(meet, 'class="retry-btn"', '@tap="load()"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="重试"')
    expect(b).toContain('@keydown.enter="load()"')
    expect(b).toContain('@keydown.space.prevent="load()"')
  })

  it('空态刷新补 a11y 属性并复用 load()', () => {
    const b = blockWith2(meet, 'class="empty-guide-link ai-pressable"', '@tap="load()"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="刷新"')
    expect(b).toContain('@keydown.enter="load()"')
    expect(b).toContain('@keydown.space.prevent="load()"')
  })

  it('空态换页签补 a11y 属性并复用 switchTab', () => {
    const b = blockWith2(meet, 'class="empty-guide-link ai-pressable primary"', 'switchTab(tab ===')
    a11yAttrs(b)
    expect(b).toContain('aria-label="换个页签"')
    expect(b).toContain('@keydown.enter="switchTab(tab ===')
    expect(b).toContain('@keydown.space.prevent="switchTab(tab ===')
  })

  it('footer retry 补 a11y 属性并复用 loadMore', () => {
    const b = blockWith2(meet, 'class="retry-btn"', '@tap="loadMore"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="重试"')
    expect(b).toContain('@keydown.enter="loadMore"')
    expect(b).toContain('@keydown.space.prevent="loadMore"')
  })

  it('load-more 补 a11y 属性并复用 loadMore', () => {
    const b = blockWith2(meet, 'class="load-more"', '@tap="loadMore"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="加载更多"')
    expect(b).toContain('@keydown.enter="loadMore"')
    expect(b).toContain('@keydown.space.prevent="loadMore"')
  })

  it('守卫：业务 handler 函数体逐字零改', () => {
    expect(meet).toContain('async function load(previousRows: MeetingRow[] = rows.value): Promise<void> {')
    expect(meet).toContain('async function loadMore(): Promise<void> {')
    expect(meet).toContain('function switchTab(key: TabKey): void {')
    expect(meet).toContain('if (tab.value === key) return')
    expect(meet).toContain('const token = listGate.begin()')
  })

  it('守卫：meetingApi 调用零改', () => {
    expect(meet).toContain('meetingApi.invitedWait()')
    expect(meet).toContain('meetingApi.waitConfirm()')
    expect(meet).toContain('meetingApi.applied(1, MEETING_PAGE_SIZE)')
    expect(meet).toContain('meetingApi.applied(next, MEETING_PAGE_SIZE)')
  })

  it('守卫：原生 button 族零改', () => {
    expect(meet).toContain('@tap="accept(m)"')
    expect(meet).toContain('@tap="reject(m)"')
    expect(meet).toContain('@tap="allow(m)"')
    expect(meet).toContain('@tap="deny(m)"')
    expect(meet).toContain('@tap="checkin(m)"')
    expect(meet).toContain('@tap="showCode(m)"')
    expect(linesWith(meet, '<button').length).toBeGreaterThanOrEqual(6)
  })

  it('守卫：所有原 @tap 处理器保留（鼠标零回归）', () => {
    const taps = linesWith(meet, '@tap=')
    expect(taps.length).toBeGreaterThanOrEqual(10)
    expect(meet).toContain('@tap="switchTab(t.key)"')
    expect(meet).toContain('@tap="load()"')
    expect(meet).toContain('@tap="loadMore"')
  })

  it('守卫：main.ts R537 增强器零改且无全局伪按钮增强器', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain("HOST_SELECTOR = 'uni-input, uni-textarea, uni-picker'")
    expect(main).not.toContain('switchTab')
  })

  it('守卫：样式段零改且未被注入 role/tabindex', () => {
    const style = meet.slice(meet.indexOf('<style'))
    expect(style).not.toContain('role="button"')
    expect(style).not.toContain('tabindex=')
    expect(style).not.toContain('@keydown')
  })
})
