// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R544 RED→GREEN 契约：H5 审批页 process.vue 伪按钮键盘可达（WCAG 2.1 SC2.1.1 / 4.1.2）。
 * 逐页显式补 role=button + tabindex=0 + aria-label + @keydown.enter/.space.prevent，复用原业务 handler。
 * 真父 5ee4a9b92 无这些属性 → 核心 RED；守卫业务零改须真父亦通过。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')

const P = read('pages/process/process.vue')
const main = read('main.ts')
const cal = read('pages/calendar/calendar.vue')
const pages = read('pages.json')

function linesWith(source: string, needle: string): string[] {
  return source.split('\n').filter((l: string) => l.includes(needle))
}
function lineWith(source: string, needle: string): string {
  const line = source.split('\n').find((l: string) => l.includes(needle))
  expect(line, '缺少锚点: ' + needle).toBeDefined()
  return line ?? ''
}
function expectPressable(line: string, labelExpr: string): void {
  expect(line).toContain('role="button"')
  expect(line).toContain('tabindex="0"')
  expect(line).toContain(labelExpr)
  expect(line).toContain('@tap=')
  expect(line).toContain('@keydown.enter=')
  expect(line).toContain('@keydown.space.prevent=')
}

describe('R544 H5 审批页伪按钮键盘可达契约', () => {
  // ===== 核心：6 类伪按钮 role/tabindex/aria-label/keydown（真父无 → RED）=====
  it('3 个 tab 切换 role=button/tabindex/动态 aria-label 并复用 switchTab', () => {
    const tab = lineWith(P, 'class="tab ai-brand-fill"')
    expectPressable(tab, ':aria-label="t.label"')
    expect(tab).toContain('@keydown.enter="switchTab(t.key)"')
    expect(tab).toContain('@keydown.space.prevent="switchTab(t.key)"')
    expect(tab).toContain('@tap="switchTab(t.key)"')
  })

  it('列表项 role=button/tabindex/动态 titleOf aria-label 并复用 openDetail', () => {
    const item = lineWith(P, 'class="item o2-list-item"')
    expectPressable(item, ':aria-label="titleOf(row, i)"')
    expect(item).toContain('@keydown.enter="openDetail(row, i)"')
    expect(item).toContain('@keydown.space.prevent="openDetail(row, i)"')
    expect(item).toContain('@tap="openDetail(row, i)"')
  })

  it('三处重试 role=button/tabindex/aria-label 并分别复用 loadFirst/loadMore/retryDetail', () => {
    const retries = linesWith(P, 'class="retry-btn"')
    expect(retries.length).toBe(3)
    retries.forEach((l: string) => expectPressable(l, 'aria-label="重试"'))
    expect(linesWith(P, '@keydown.enter="loadFirst"').length).toBeGreaterThan(0)
    expect(linesWith(P, '@keydown.enter="loadMore"').length).toBeGreaterThan(0)
    expect(linesWith(P, '@keydown.enter="retryDetail"').length).toBeGreaterThan(0)
    expect(linesWith(P, '@keydown.space.prevent="loadFirst"').length).toBeGreaterThan(0)
    expect(linesWith(P, '@keydown.space.prevent="loadMore"').length).toBeGreaterThan(0)
    expect(linesWith(P, '@keydown.space.prevent="retryDetail"').length).toBeGreaterThan(0)
  })

  it('加载更多 role=button/tabindex/aria-label 并复用 loadMore', () => {
    const lm = lineWith(P, 'class="load-more"')
    expectPressable(lm, 'aria-label="加载更多"')
    expect(lm).toContain('@keydown.enter="loadMore"')
    expect(lm).toContain('@keydown.space.prevent="loadMore"')
    expect(lm).toContain('@tap="loadMore"')
  })

  it('详情关闭 mask 与 sheet-close role=button/tabindex/aria-label 并复用 closeDetail', () => {
    const mask = lineWith(P, 'class="mask"')
    expectPressable(mask, 'aria-label="关闭详情"')
    expect(mask).toContain('@keydown.enter="closeDetail"')
    expect(lineWith(P, 'class="sheet-close"')).toContain('@keydown.space.prevent="closeDetail"')
    const close = lineWith(P, 'class="sheet-close"')
    expectPressable(close, 'aria-label="关闭详情"')
    expect(close).toContain('@keydown.enter="closeDetail"')
  })

  it('发起流程 fab role=button/tabindex/aria-label 并复用 goStart', () => {
    const fab = lineWith(P, 'class="fab"')
    expectPressable(fab, 'aria-label="发起流程"')
    expect(fab).toContain('@keydown.enter="goStart"')
    expect(fab).toContain('@keydown.space.prevent="goStart"')
    expect(fab).toContain('@tap="goStart"')
  })

  // ===== 守卫：业务零改（真父亦须通过）=====
  it('守卫：6+1 业务 handler 函数体逐字零改', () => {
    expect(P).toContain('function switchTab(key: TabKey) {\n  if (active.value === key) return\n  active.value = key\n  loadFirst()\n}')
    expect(P).toContain('const token = pageGate.begin()')
    expect(P).toContain('const token = detailGate.begin()')
    expect(P).toContain('function retryDetail() {\n  const target = detailTarget.value')
    expect(P).toContain('function closeDetail() {\n  detailGate.begin()')
    expect(P).toContain('async function loadMore()')
    expect(P).toContain("uni.navigateTo({ url: '/pages/process/start' })")
  })

  it('守卫：真实业务 API processApi.* 逐字零改', () => {
    for (const api of ['pendingList', 'completedList', 'startedList', 'attachments', 'records', 'worklogs', 'completeTask', 'rejectTask']) {
      expect(P, 'API 丢失: ' + api).toContain('processApi.' + api)
    }
  })

  it('守卫：原生 button 驳回/通过逐字零改且未被注入伪按钮属性', () => {
    const btns = linesWith(P, '<button size="mini"')
    expect(btns.length).toBe(2)
    btns.forEach((b: string) => {
      expect(b).toContain('@tap.stop="act(')
      expect(b).not.toContain('role="button"')
      expect(b).not.toContain('tabindex=')
    })
  })

  it('守卫：main.ts 表单名增强器零改且未引入伪按钮全局增强器', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain("HOST_SELECTOR = 'uni-input, uni-textarea, uni-picker'")
    expect(main).not.toContain('ai-brand-fill')
    expect(main).not.toContain('switchTab(t.key)')
    expect(main).not.toContain('keydown.enter')
  })

  it('守卫：pages.json 零改且 calendar:295 正面对照零改', () => {
    expect(pages).toContain('"tabBar"')
    expect(pages).not.toContain('role="button"')
    expect(cal).toContain('<text class="calendar-create-btn ai-brand-fill" role="button" aria-label="新建日历" @tap="toggleCalendarForm">')
  })

  it('守卫：样式段零改（无 role/tabindex/keydown 注入）', () => {
    const style = P.slice(P.indexOf('<style'))
    expect(style).toContain('.fab {')
    expect(style).not.toContain('role="button"')
    expect(style).not.toContain('tabindex=')
    expect(style).not.toContain('@keydown')
  })

  it('守卫：9 处原 @tap 鼠标处理器全部保留', () => {
    for (const tap of ['switchTab(t.key)', 'openDetail(row, i)', 'loadFirst', 'loadMore', 'retryDetail', 'closeDetail', 'goStart']) {
      expect(P, '@tap 丢失: ' + tap).toContain('@tap="' + tap + '"')
    }
    expect(linesWith(P, '@tap="closeDetail"').length).toBe(2)
  })
})
