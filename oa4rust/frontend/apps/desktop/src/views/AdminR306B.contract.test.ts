// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R306-B 契约：query/portal/query-explorer/query-view/portal-dict/portal-page 六个
 * admin 设计器页从双未消费（srcVar=0 且 DOM aiPage=0）升级为令牌深度消费。
 * 用 hasAiRule 逐选择器断言真实令牌规则，aiVarCount>=5，禁假断言/子串误判。
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
  let from = 0
  const needle = 'var(--ai-' + token + ')'
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

type Page = { label: string; src: string; root: string; header: string; panel: string; primaryBtn: string; skeleton: string; emptyText: string }
const pages: Page[] = [
  { label: 'QueryDesigner', src: read('QueryDesigner.vue'), root: 'query-designer', header: 'view-header', panel: '.sidebar', primaryBtn: '.btn-create', skeleton: '.sk', emptyText: '暂无查询定义' },
  { label: 'PortalDesigner', src: read('PortalDesigner.vue'), root: 'designer-shell', header: 'designer-header', panel: '.panel', primaryBtn: '.btn.primary', skeleton: '.sk', emptyText: '暂无设计' },
  { label: 'QueryExplorerApp', src: read('QueryExplorerApp.vue'), root: 'crud-view', header: 'view-header', panel: '.content-panel', primaryBtn: '.btn-refresh', skeleton: '.skel', emptyText: '暂无查询视图' },
  { label: 'QueryViewDesignerApp', src: read('QueryViewDesignerApp.vue'), root: 'designer-shell', header: 'designer-header', panel: '.editor', primaryBtn: '.btn.primary', skeleton: '.sk', emptyText: '该 Query 下暂无视图' },
  { label: 'PortalDictDesignerApp', src: read('PortalDictDesignerApp.vue'), root: 'crud-view', header: 'view-header', panel: '.content-panel', primaryBtn: '.btn-primary', skeleton: '.skel', emptyText: '暂无门户字典' },
  { label: 'PortalPageDesignerApp', src: read('PortalPageDesignerApp.vue'), root: 'crud-view', header: 'view-header', panel: '.content-panel', primaryBtn: '.btn-primary', skeleton: '.skel', emptyText: '暂无门户页面' },
]

describe('R306-B 六页 admin 设计器令牌深度消费契约', () => {
  it('六页根组件与页头均打上 ai-page，DOM 口径 aiPage>=2', () => {
    for (const p of pages) {
      const tpl = templateOf(p.src)
      expect(aiPageCount(tpl), p.label + ' 的 ai-page 容器少于 2').toBeGreaterThanOrEqual(2)
      expect(tpl, p.label + ' 根缺 ai-page').toContain('class="' + p.root + ' modern-page ai-page')
      expect(tpl, p.label + ' 页头缺 ai-page ai-panel').toContain(p.header + ' glass-card ai-page ai-panel')
    }
  })

  it('六页 ai-panel 玻璃面板、8px 栅格、品牌渐变均用真实令牌规则落地', () => {
    for (const p of pages) {
      const css = styleOf(p.src)
      expect(hasAiRule(css, '.' + p.root, 'grid-2'), p.label + ' 根缺 8px 栅格').toBe(true)
      expect(hasAiRule(css, '.' + p.header, 'gradient-surface'), p.label + ' 页头缺渐变表面').toBe(true)
      expect(hasAiRule(css, p.panel, 'glass-bg'), p.label + ' 面板缺玻璃背景').toBe(true)
      expect(hasAiRule(css, p.primaryBtn, 'gradient-brand'), p.label + ' 主按钮缺品牌渐变').toBe(true)
    }
  })

  it('六页骨架屏使用 --ai-skeleton 令牌，空态保留可读引导文案', () => {
    for (const p of pages) {
      const css = styleOf(p.src)
      const tpl = templateOf(p.src)
      expect(
        hasAiRule(css, p.skeleton, 'skeleton-shine') || hasAiRule(css, p.skeleton, 'skeleton-base'),
        p.label + ' 骨架缺令牌'
      ).toBe(true)
      expect(tpl, p.label + ' 空态引导缺失').toContain(p.emptyText)
    }
  })

  it('AdminPageHeader/AdminFormSection 使用页必须真正 import，消除 setup ReferenceError', () => {
    for (const label of ['QueryExplorerApp','QueryViewDesignerApp']) {
      const p = pages.find(x => x.label === label)!
      const script = p.src.slice(p.src.indexOf('<script'))
      expect(script, label + ' 模板用了 AdminPageHeader 却未 import').toMatch(/import\s*\{[^}]*AdminPageHeader[^}]*\}\s*from\s*'\.\.\/components\/admin\//)
      expect(script, label + ' 模板用了 AdminFormSection 却未 import').toMatch(/import\s*\{[^}]*AdminFormSection[^}]*\}\s*from\s*'\.\.\/components\/admin\//)
      const template = p.src.slice(0, p.src.indexOf('<script'))
      const used = [...new Set((template.match(/<Admin[A-Z][A-Za-z]*/g) || []).map(x => x.slice(1)))]
      expect(used.length, label + ' 契约应至少识别到共享组件').toBeGreaterThan(0)
      for (const comp of used) {
        expect(script, label + ' 模板使用 ' + comp + ' 却未 import').toMatch(new RegExp('\\b' + comp + '\\b[^\\n]*from\\s+\'\\.\\.\\/components\\/admin\\/'))
      }
      expect(script, label + ' 不应保留 void 组件引用的规避写法').not.toMatch(/void\s+Admin[A-Z]/)
    }
  })

  it('六页源码 var(--ai-*) >=5 且保留业务标题结构', () => {
    for (const p of pages) {
      expect(aiVarCount(p.src), p.label + ' 的 var(--ai-*) 少于 5').toBeGreaterThanOrEqual(5)
      expect(templateOf(p.src), p.label + ' 缺标题结构').toMatch(/<h[1-3][ >]|AdminPageHeader/)
    }
  })
})
