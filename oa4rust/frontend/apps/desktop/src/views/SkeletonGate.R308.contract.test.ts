// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R308：PortalScriptDesignerApp 静态骨架条无条件渲染修正契约。
// t62 基线审计坐实：script-loading ai-skeleton-line 无条件渲染，数据加载后仍显骨架。
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function read(name: string): string {
  return readFileSync(resolve(import.meta.dirname, name), 'utf8')
}
function templateOf(src: string): string {
  return src.slice(src.indexOf('<template>'), src.indexOf('</template>'))
}

describe('R308 skeleton bar gate — PortalScriptDesignerApp', () => {
  const src = read('PortalScriptDesignerApp.vue')
  const tpl = templateOf(src)

  it('no unconditional static skeleton bar in template', () => {
    const barRe = /<div[^>]*class="[^"]*script-loading[^"]*ai-skeleton-line[^"]*"[^>]*>/
    const match = tpl.match(barRe)
    expect(match).toBeNull()
  })

  it('if a skeleton bar remains it must bind a real loading flag', () => {
    if (tpl.includes('script-loading')) {
      expect(tpl).toMatch(/script-loading[^>]*v-if=/)
      expect(src).toMatch(/<script[\s\S]*\bloading\b/)
    } else {
      expect(tpl).not.toContain('script-loading')
      expect(src).not.toMatch(/<div[^>]*class="[^"]*script-loading[^"]*ai-skeleton-line/)
    }
  })

  it('ai-page/ai-panel tokens and ScriptWorkbench business adapter not regressed', () => {
    expect(tpl).toContain('view-shell modern-page ai-page')
    expect(tpl).toContain('script-hero ai-page ai-panel')
    expect(tpl).toContain('script-workbench ai-panel')
    expect(tpl).toContain('<ScriptWorkbench')
    expect(src).toContain('/api/portal/assemble/designer/script/list/manager')
    expect(src).toMatch(/\.ai-page\s*\{[^}]*var\(--ai-grid-2\)/)
  })
})
