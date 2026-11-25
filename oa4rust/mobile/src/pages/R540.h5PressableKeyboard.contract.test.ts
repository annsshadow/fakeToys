// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R540 RED→GREEN 契约：H5 工作台 index.vue 伪按钮键盘可达（WCAG 2.1 SC2.1.1 / 4.1.2）。
 * 方案B逐页显式补 role=button + tabindex=0 + aria-label + @keydown.enter/@keydown.space.prevent，
 * 复用业务 handler（goTab/goPage/loadNotices/viewNotice），uni-app H5 @tap 编译为 DOM click 已实证。
 * 真父 a018481e0 无这些属性 → 核心 RED；守卫业务零改须真父亦通过。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = resolve(import.meta.dirname, '..')
const read = (rel: string): string => readFileSync(resolve(src, rel), 'utf8')

const index = read('pages/index/index.vue')
const main = read('main.ts')
const cal = read('pages/calendar/calendar.vue')

function linesWith(source: string, needle: string): string[] {
  return source.split('\n').filter((l) => l.includes(needle))
}
function lineWith(source: string, needle: string): string {
  const line = source.split('\n').find((l) => l.includes(needle))
  expect(line, '缺少锚点: ' + needle).toBeDefined()
  return line ?? ''
}

const CELL_LABELS = ['消息', '审批', '文档', '通讯录', '考勤', '发起流程', '会议', '日程', '论坛', '浏览', '搜索', '回收站']

describe('R540 H5 工作台伪按钮键盘可达契约', () => {
  // ===== 核心：17 伪按钮 role/tabindex/aria-label/keydown（真父无 → RED）=====
  it('2 统计卡 role=button/tabindex/aria-label 并复用 goTab', () => {
    const cards = linesWith(index, 'stat-card ai-brand-fill ai-pressable')
    expect(cards.length).toBe(2)
    expect(cards[0]).toContain('aria-label="待办审批"')
    expect(cards[1]).toContain('aria-label="今日出勤"')
    cards.forEach((l) => {
      expect(l).toContain('role="button"')
      expect(l).toContain('tabindex="0"')
      expect(l).toContain('@keydown.enter="goTab(')
      expect(l).toContain('@keydown.space.prevent="goTab(')
    })
  })

  it('12 宫格 role=button/tabindex/aria-label/keydown 且复用 goTab/goPage', () => {
    const cells = linesWith(index, 'class="cell ai-pressable"')
    expect(cells.length).toBe(12)
    cells.forEach((l, i) => {
      expect(l, '宫格[' + i + '] aria-label').toContain('aria-label="' + CELL_LABELS[i] + '"')
      expect(l).toContain('role="button"')
      expect(l).toContain('tabindex="0"')
      expect(l).toContain('@tap=')
      expect(l).toMatch(/@keydown\.(enter|space\.prevent)=/)
      expect(l).toContain('@keydown.enter=')
      expect(l).toContain('@keydown.space.prevent=')
    })
  })

  it('公告项 role=button/tabindex/动态 aria-label 并复用 viewNotice', () => {
    const notice = lineWith(index, 'class="notice o2-list-item"')
    expect(notice).toContain('role="button"')
    expect(notice).toContain('tabindex="0"')
    expect(notice).toContain(':aria-label="n.title')
    expect(notice).toContain('@keydown.enter="viewNotice = n"')
    expect(notice).toContain('@keydown.space.prevent="viewNotice = n"')
  })

  it('重试×2 与刷新公告 role=button/tabindex/aria-label 并复用 loadNotices(true)', () => {
    const retries = linesWith(index, 'class="retry-btn"')
    expect(retries.length).toBe(2)
    retries.forEach((l) => {
      expect(l).toContain('role="button"')
      expect(l).toContain('tabindex="0"')
      expect(l).toContain('aria-label="重试"')
      expect(l).toContain('@keydown.enter="loadNotices(true)"')
      expect(l).toContain('@keydown.space.prevent="loadNotices(true)"')
    })
    const refresh = lineWith(index, 'class="empty-guide-link ai-pressable"')
    expect(refresh).toContain('role="button"')
    expect(refresh).toContain('tabindex="0"')
    expect(refresh).toContain('aria-label="刷新公告"')
    expect(refresh).toContain('@keydown.enter="loadNotices(true)"')
    expect(refresh).toContain('@keydown.space.prevent="loadNotices(true)"')
  })

  it('公告详情关闭原生 button 补 tabindex/aria-label（据实纳入）', () => {
    const close = lineWith(index, 'class="nd-close"')
    expect(close).toContain('tabindex="0"')
    expect(close).toContain('aria-label="关闭公告"')
    expect(close).toContain('@tap="viewNotice = null"')
  })

  // ===== 守卫：业务零改（真父亦须通过）=====
  it('守卫：goTab/goPage 业务函数体逐字零改', () => {
    expect(index).toContain('function goTab(url: string) {\n  uni.switchTab({ url })\n}')
    expect(index).toContain('function goPage(url: string) {\n  uni.navigateTo({ url })\n}')
  })

  it('守卫：loadNotices/viewNotice/onShow 业务逻辑逐字零改', () => {
    expect(index).toContain("noticesError.value = ''")
    expect(index).toContain("cachedRequest('announcement:list'")
    expect(index).toContain('async function loadNotices(force = false)')
    expect(index).toContain('const viewNotice = ref<{ title?: string; content?: string; createTime?: string } | null>(null)')
  })

  it('守卫：所有原 @tap 处理器保留（伪按钮鼠标行为零回归）', () => {
    const taps = linesWith(index, '@tap=')
    expect(taps.length).toBeGreaterThanOrEqual(17)
    expect(index).toContain('@tap="viewNotice = null"')
    expect(index).toContain('@tap.stop')
    expect(index).toContain('@tap="loadNotices(true)"')
    expect(index).toContain('@tap="viewNotice = n"')
  })

  it('守卫：main.ts R537 表单名增强器零改且未引入伪按钮全局增强器', () => {
    expect(main).toContain('function installH5NativeInputNameA11y')
    expect(main).toContain("HOST_SELECTOR = 'uni-input, uni-textarea, uni-picker'")
    expect(main).toContain('setAttribute(')
    expect(main).toContain('installH5NativeInputNameA11y()')
    expect(main).not.toContain('ai-pressable')
    expect(main).not.toContain('keydown.enter')
  })

  it('守卫：calendar.vue:295 正面对照与样式段零改', () => {
    expect(cal).toContain('<text class="calendar-create-btn ai-brand-fill" role="button" aria-label="新建日历" @tap="toggleCalendarForm">')
    const style = index.slice(index.indexOf('<style'))
    expect(style).not.toContain('role="button"')
    expect(style).not.toContain('tabindex=')
    expect(style).not.toContain('@keydown')
  })
})
