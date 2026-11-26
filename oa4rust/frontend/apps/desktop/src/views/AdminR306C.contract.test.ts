// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R306 C 组 admin 设计器视觉契约：六页双未消费必须落地 .ai-page 根标记、
 * .ai-panel 玻璃、--ai-gradient-brand 渐变、8px 栅格、骨架与空态引导。
 * 断言读取真实源码、逐选择器扫描 CSS 令牌规则，不改业务接口与路由。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string): string => readFileSync(resolve(import.meta.dirname, name), 'utf8')
const templateOf = (s: string): string => s.slice(s.indexOf('<template>'), s.indexOf('</template>'))
const styleOf = (s: string): string => s.slice(s.indexOf('<style'), s.lastIndexOf('</style>'))
const aiVarCount = (s: string): number => (s.match(/var[(]--ai-/g) ?? []).length

/** 逐条扫描同名 CSS 规则，只要有一条包含目标令牌即通过（避免只看首个匹配造成误判）。 */
function hasAiRule(style: string, selector: string, token: string): boolean {
  const re = new RegExp(selector.replace(/[.]/g, '[.]') + ' *[{]([^}]*)[}]', 'g')
  let match: RegExpExecArray | null
  while ((match = re.exec(style)) !== null) {
    if (match[1].includes(token)) return true
  }
  return false
}

interface PageSpec {
  file: string
  rootClass: string
  headerClass: string
  headerSel: string
  panelSel: string
  skeletonSel: string
  skeletonToken: string
  emptyHint: string
  business: string[]
}

const specs: PageSpec[] = [
  {
    file: 'PortalScriptDesignerApp.vue',
    rootClass: 'view-shell modern-page ai-page',
    headerClass: 'script-hero ai-page ai-panel',
    headerSel: '.ai-page .script-hero',
    panelSel: '.ai-page .script-workbench',
    skeletonSel: '.ai-skeleton-line',
    skeletonToken: '--ai-skeleton-base',
    emptyHint: '门户脚本',
    business: ['ScriptWorkbench', '/api/portal/assemble/designer/script/list/manager', '/api/portal/assemble/designer/scriptversion/list/script/'],
  },
  {
    file: 'PortalWidgetDesignerApp.vue',
    rootClass: 'crud-view modern-page ai-page',
    headerClass: 'view-header glass-card ai-page ai-panel',
    headerSel: '.ai-page .view-header',
    panelSel: '.ai-page .content-panel',
    skeletonSel: '.skel',
    skeletonToken: '--ai-skeleton-base',
    emptyHint: '新建',
    business: ['/api/portal/assemble/designer/widget/list', '/api/portal/assemble/designer/widget/create', 'deleteItem'],
  },
  {
    file: 'ServiceInvokeDesignerApp.vue',
    rootClass: 'crud-view modern-page ai-page',
    headerClass: 'view-header glass-card ai-page ai-panel',
    headerSel: '.ai-page .view-header',
    panelSel: '.ai-page .content-panel',
    skeletonSel: '.skel',
    skeletonToken: '--ai-skeleton-base',
    emptyHint: '新建',
    business: ['/api/program_center/invoke/list', '/api/program_center/invoke/create', 'runInvoke'],
  },
  {
    file: 'FindDesignerApp.vue',
    rootClass: 'editor-view modern-page ai-page',
    headerClass: 'view-header glass-card ai-page ai-panel',
    headerSel: '.ai-page .view-header',
    panelSel: '.ai-page .editor-layout',
    skeletonSel: '.ai-skeleton-line',
    skeletonToken: '--ai-skeleton-base',
    emptyHint: '新建',
    business: ['/api/query/assemble/designer/list/all', '/api/query/assemble/designer/save/', 'selectItem'],
  },
  {
    file: 'PdfViewerApp.vue',
    rootClass: 'viewer-view modern-page ai-page',
    headerClass: 'view-header glass-card ai-page ai-panel',
    headerSel: '.ai-page .view-header',
    panelSel: '.ai-page .viewer-panel',
    skeletonSel: '.ai-skeleton-line',
    skeletonToken: '--ai-skeleton-base',
    emptyHint: '输入文件ID',
    business: ['/api/attachment/download/', '/api/file/download/', 'loadPdf'],
  },
  {
    file: 'CommonApp.vue',
    rootClass: 'crud-view modern-page ai-page',
    headerClass: 'view-header glass-card ai-page ai-panel',
    headerSel: '.ai-page .view-header',
    panelSel: '.ai-page .content-panel',
    skeletonSel: '.skel',
    skeletonToken: '--ai-skeleton-base',
    emptyHint: '新建',
    business: ['/api/general/assemble/control/list', '/api/general/assemble/control/create', 'fmtTime'],
  },
]

describe('R306 C 组六个 admin 设计器 AI 令牌深度消费', () => {
  for (const spec of specs) {
    it(spec.file + ' 落地双 ai-page、玻璃/渐变/栅格/骨架/空态且保留业务端点', () => {
      const src = read(spec.file)
      const tpl = templateOf(src)
      const style = styleOf(src)

      // DOM aiPage≥2：根与头部都是真实 ai-page 元素（真浏览器另做 DOM 断言）。
      expect(tpl).toContain('class="' + spec.rootClass + '"')
      expect(tpl).toContain('class="' + spec.headerClass + '"')
      expect((tpl.match(/ai-page/g) ?? []).length).toBeGreaterThanOrEqual(2)

      // 玻璃面板 + 品牌渐变 + 8px 栅格 + spring 动效，逐选择器扫描真实规则。
      expect(hasAiRule(style, spec.headerSel, '--ai-gradient-surface')).toBe(true)
      expect(hasAiRule(style, spec.headerSel, '--ai-glass-border')).toBe(true)
      expect(hasAiRule(style, spec.panelSel, '--ai-glass-bg')).toBe(true)
      expect(hasAiRule(style, '.ai-page .btn-primary', '--ai-gradient-brand')).toBe(true)
      expect(hasAiRule(style, '.ai-page', '--ai-grid-2')).toBe(true)
      expect(style).toContain('var(--ai-motion-fast)')
      expect(style).toContain('var(--ai-ease-spring)')

      // 骨架令牌下沉（替换裸 Loading/spinner 的视觉主线）。
      expect(hasAiRule(style, spec.skeletonSel, spec.skeletonToken)).toBe(true)
      expect(style).toContain('ai-shimmer')

      // 空态 AI 引导与业务端点/函数保留，禁止改接口。
      expect(tpl).toContain(spec.emptyHint)
      for (const token of spec.business) expect(src).toContain(token)
      expect(aiVarCount(src)).toBeGreaterThanOrEqual(5)
    })
  }
})
