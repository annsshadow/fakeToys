// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R554 RED→GREEN 契约：H5 回收站页 6 类 @tap 伪按钮键盘可达（WCAG 2.1 SC2.1.1 / 4.1.2）。
 * 方案B逐页显式补 role=button + tabindex=0 + 动态 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * 真父 b134a06b4 无这些属性 → 核心 RED；守卫业务零改须真父亦通过。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')

const rc = read('pages/recycle/recycle.vue')
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

describe('R554 H5 回收站页伪按钮键盘可达契约', () => {
  it('错误态与有行错误 banner 两处 retry 补 a11y 并复用 load()', () => {
    const b = blockWith2(rc, 'class="retry-btn"', '@tap="load()"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="重试"')
    expect(b).toContain('@keydown.enter="load()"')
    expect(b).toContain('@keydown.space.prevent="load()"')
    expect(linesWith(rc, 'class="retry-btn"').length).toBeGreaterThanOrEqual(2)
  })

  it('空态刷新补 a11y 并复用 load()', () => {
    const b = blockWith2(rc, 'class="empty-guide-link ai-pressable"', '@tap="load()"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="刷新"')
    expect(b).toContain('@keydown.enter="load()"')
    expect(b).toContain('@keydown.space.prevent="load()"')
  })

  it('空态去文档补 a11y 并复用 goDoc（无括号）', () => {
    const b = blockWith2(rc, 'class="empty-guide-link ai-pressable primary"', '@tap="goDoc"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="去文档"')
    expect(b).toContain('@keydown.enter="goDoc"')
    expect(b).toContain('@keydown.space.prevent="goDoc"')
  })

  it('列表项 card-title 补 a11y + 动态 aria-label 并复用 showDetail(r)', () => {
    const b = blockWith2(rc, 'class="card-title"', '@tap="showDetail(r)"')
    a11yAttrs(b)
    expect(b).toContain(':aria-label="\'查看\' + nameOf(r) + \'详情\'"')
    expect(b).toContain('@keydown.enter="showDetail(r)"')
    expect(b).toContain('@keydown.space.prevent="showDetail(r)"')
  })

  it('详情关闭 detail-close 补 a11y 并复用 detail = null', () => {
    const b = blockWith2(rc, 'class="detail-close"', '@tap="detail = null"')
    a11yAttrs(b)
    expect(b).toContain('aria-label="关闭详情"')
    expect(b).toContain('@keydown.enter="detail = null"')
    expect(b).toContain('@keydown.space.prevent="detail = null"')
  })

  it('守卫：业务 handler 函数体逐字零改', () => {
    expect(rc).toContain('async function load(previousRows: Row[] = rows.value): Promise<void> {')
    expect(rc).toContain('() => recycleApi.list()')
    expect(rc).toContain("label: '回收站加载失败'")
    expect(rc).toContain('async function showDetail(r: Row): Promise<void> {')
    expect(rc).toContain('const token = detailGate.begin()')
    expect(rc).toContain('await recycleApi.detail(id)')
    expect(rc).toContain('function goDoc() {')
    expect(rc).toContain("url: '/pages/doc/doc'")
    expect(rc).toContain("r.name ?? r.title ?? r.fileName ?? r.id ?? '未命名'")
  })

  it('守卫：所有原 @tap 处理器保留（鼠标零回归）+ recycleApi 零改', () => {
    const taps = linesWith(rc, '@tap=')
    expect(taps.length).toBeGreaterThanOrEqual(8)
    expect(rc).toContain('@tap="load()"')
    expect(rc).toContain('@tap="goDoc"')
    expect(rc).toContain('@tap="showDetail(r)"')
    expect(rc).toContain('@tap="remove(r)"')
    expect(rc).toContain('@tap="emptyAll"')
    expect(rc).toContain('@tap="detail = null"')
    expect(rc).toContain("from '@/services'")
  })

  it('守卫：原生 button 族零改', () => {
    expect(rc).toContain('<button class="btn danger" :disabled="busy" @tap="remove(r)">彻底删除</button>')
    expect(rc).toContain('<button v-if="rows.length > 0" class="btn empty" :disabled="busy" @tap="emptyAll">清空回收站</button>')
    expect(linesWith(rc, '<button').length).toBeGreaterThanOrEqual(2)
  })

  it('守卫：main.ts R537 增强器零改且无全局伪按钮增强器', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain('HOST_SELECTOR = ' + "'uni-input, uni-textarea, uni-picker'")
    expect(main).not.toContain('showDetail')
    expect(main).not.toContain('detail-close')
  })

  it('守卫：样式段零改 + calendar:295 正面对照零改', () => {
    const style = rc.slice(rc.indexOf('<style'))
    expect(style).not.toContain('role="button"')
    expect(style).not.toContain('tabindex=')
    expect(style).not.toContain('@keydown')
    expect(calendar).toContain('aria-label="新建日历"')
    expect(calendar).toContain('@tap="toggleCalendarForm"')
  })
})
