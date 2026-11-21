// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R306-A 契约：query / jpush / queryview / form / view / fileinfo 六个 admin 页
 * 从双未消费（源码无 var(--ai-*) 且 DOM 无 ai-page）升级为令牌深度消费。
 * 断言：每页 DOM ai-page>=2、玻璃 ai-panel、栅格/渐变/玻璃/骨架令牌规则真实落地、
 * 空态 AI 引导 class+可读文案、源码 var(--ai-*)>=5。禁止假断言/子串误判。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string): string => readFileSync(resolve(import.meta.dirname, name), 'utf8')
const templateOf = (s: string): string => s.slice(s.indexOf('<template>'), s.lastIndexOf('</template>'))
const styleOf = (s: string): string => s.slice(s.indexOf('<style'))
const aiVarCount = (s: string): number => (s.match(/var\(--ai-[a-z0-9-]+\)/g) || []).length
const aiPageCount = (tpl: string): number => (tpl.match(/class="[^"]*ai-page[^"]*"/g) || []).length
const hasAiRule = (style: string, selector: string, token: string): boolean => {
  const needle = 'var(--ai-' + token + ')'
  let from = 0
  while (from < style.length) {
    const at = style.indexOf(selector, from)
    if (at < 0) return false
    const open = style.indexOf('{', at)
    const close = style.indexOf('}', open)
    if (open > at && close > open && style.slice(open, close).includes(needle)) return true
    from = close > open ? close + 1 : at + selector.length
  }
  return false
}

type Page = { label: string; file: string; root: string; panel: string; skeleton: string; emptyText: string }
const pages: Page[] = [
  { label: 'query', file: 'QueryManager.vue', root: 'query-view', panel: '.qs', skeleton: '.sk2', emptyText: '暂无查询定义' },
  { label: 'jpush', file: 'JPushApp.vue', root: 'mod-view', panel: '.content-panel', skeleton: '.sk', emptyText: '暂无设备' },
  { label: 'queryview', file: 'QueryViewApp.vue', root: 'mod-view', panel: '.content-panel', skeleton: '.sk', emptyText: '暂无查询视图' },
  { label: 'form', file: 'FormApp.vue', root: 'mod-view', panel: '.content-panel', skeleton: '.sk', emptyText: '暂无表单' },
  { label: 'view', file: 'ViewApp.vue', root: 'mod-view', panel: '.content-panel', skeleton: '.sk', emptyText: '暂无视图数据' },
  { label: 'fileinfo', file: 'FileInfoApp.vue', root: 'mod-view', panel: '.content-panel', skeleton: '.sk', emptyText: '暂无文件信息' },
]
const sources = pages.map((p) => ({ ...p, src: read(p.file) }))

describe('R306-A 六页 admin 令牌深度消费契约', () => {
  it('六页根组件与玻璃面板均打上 ai-page / ai-panel，DOM 口径 aiPage>=2', () => {
    for (const p of sources) {
      const tpl = templateOf(p.src)
      expect(aiPageCount(tpl), p.label + ' 的 ai-page 容器少于 2').toBeGreaterThanOrEqual(2)
      expect(tpl, p.label + ' 根缺 ai-page').toMatch(new RegExp('class="[^"]*' + p.root + '[^"]*ai-page[^"]*"'))
      expect(tpl, p.label + ' 缺玻璃 ai-panel 容器').toContain('ai-panel')
    }
  })

  it('六页 ai-panel 玻璃面板 + 8px 栅格 + 品牌渐变令牌规则真实落地', () => {
    for (const p of sources) {
      const css = styleOf(p.src)
      expect(hasAiRule(css, p.panel, 'glass-bg'), p.label + ' 面板缺玻璃令牌').toBe(true)
      expect(css, p.label + ' 缺 8px 栅格令牌').toContain('var(--ai-grid-')
      expect(css, p.label + ' 缺品牌渐变令牌').toMatch(/var\(--ai-gradient-(brand|surface)\)/)
    }
  })

  it('六页骨架屏使用 --ai-skeleton 令牌，空态保留 AI 引导 class + 可读文案', () => {
    for (const p of sources) {
      const css = styleOf(p.src)
      const tpl = templateOf(p.src)
      expect(hasAiRule(css, p.skeleton, 'skeleton-shine'), p.label + ' 骨架缺令牌').toBe(true)
      expect(tpl, p.label + ' 空态缺 ai-empty 引导').toContain('ai-empty')
      expect(tpl, p.label + ' 空态引导文案缺失').toContain(p.emptyText)
    }
  })

  it('六页源码 var(--ai-*) >=5（真令牌非子串），且保留业务根 class', () => {
    for (const p of sources) {
      expect(aiVarCount(p.src), p.label + ' 的 var(--ai-*) 少于 5').toBeGreaterThanOrEqual(5)
      expect(templateOf(p.src), p.label + ' 业务根 class 丢失').toContain(p.root)
    }
  })
})
