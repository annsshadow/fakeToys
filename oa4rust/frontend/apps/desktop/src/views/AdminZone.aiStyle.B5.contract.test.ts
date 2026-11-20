// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/** R300 B5 契约：管理区共享原语与代表页应用 AI 令牌，业务逻辑/API 保持不变。 */
import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, resolve } from 'node:path'

const here = dirname(fileURLToPath(import.meta.url))
const read = (p: string) => readFileSync(resolve(here, p), 'utf8')

const shared = [
  '../components/admin/AdminLayout.vue',
  '../components/admin/AdminPageHeader.vue',
  '../components/admin/AdminFormSection.vue',
  '../components/admin/AdminTable.vue',
  '../components/admin/AdminToolbar.vue',
  '../components/admin/ModernUI.vue',
]
const pages = [
  'RoleManager.vue',
  'ConfigDesignerApp.vue',
  'LogViewerApp.vue',
  'CmsDictDesignerApp.vue',
  'ThreeMemberApp.vue',
  'ControlPanelApp.vue',
]

function template(src: string): string {
  const start = src.indexOf('<template>')
  let depth = 0
  for (let i = start; i < src.length; i++) {
    if (src.startsWith('<template', i) && !src.startsWith('</template', i)) depth++
    if (src.startsWith('</template>', i)) {
      depth--
      if (depth === 0) return src.slice(start, i + '</template>'.length)
    }
  }
  return src.slice(start)
}

describe('R300 B5 管理区 AI 风格契约', () => {
  it('共享管理原语消费 --ai-* 令牌', () => {
    for (const p of shared) {
      const src = read(p)
      expect(src, p).toMatch(/var\(--ai-[a-z-]+\)/)
    }
  })

  it('共享布局标题与导航使用 AI 渐变/缓动', () => {
    const src = read('../components/admin/AdminLayout.vue')
    expect(src).toContain('var(--ai-gradient-text)')
    expect(src).toContain('var(--ai-ease-spring)')
    expect(src).toContain('ai-b5-shell')
  })

  it('代表管理页接入 AI 页面容器与渐变标题', () => {
    for (const p of pages) {
      const tpl = template(read(p))
      expect(tpl, p).toMatch(/class="[^"]*ai-page-b5[^"]*"/)
      expect(tpl, p).toContain('ai-gradient-text')
    }
  })

  it('代表页保留关键 data-testid 与真实数据绑定', () => {
    const role = template(read('RoleManager.vue'))
    for (const id of ['role-page', 'role-refresh', 'role-create', 'role-members', 'role-edit', 'role-delete']) {
      expect(role).toContain(`data-testid="${id}"`)
    }
    const log = template(read('LogViewerApp.vue'))
    expect(log).toContain('data-testid="log-viewer-page"')
    const cfg = template(read('ConfigDesignerApp.vue'))
    expect(cfg).toContain('data-testid="config-designer-page"')
  })

  it('模板不嵌入 API 路径且不渲染原始 personId', () => {
    for (const p of pages) {
      const tpl = template(read(p))
      expect(tpl, p).not.toContain('/api/')
      expect(tpl, p).not.toContain('{{ item.personId }}')
    }
  })
})
