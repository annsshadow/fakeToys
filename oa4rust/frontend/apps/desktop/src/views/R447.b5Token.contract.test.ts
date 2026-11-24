// @vitest-environment node
import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'

const PAGES = [
  ['RoleManager.vue', 'role-view'],
  ['ThreeMemberApp.vue', 'crud-view'],
  ['CmsDictDesignerApp.vue', 'crud-view'],
  ['ControlPanelApp.vue', 'nav-view'],
  ['ConfigDesignerApp.vue', 'editor-view'],
  ['LogViewerApp.vue', 'crud-view'],
] as const

const scopedBlock = (source: string): string => {
  const marker = '.ai-page-b5.ai-page-b5, .ai-page-b5 .ai-page-b5 {'
  const at = source.indexOf(marker)
  if (at < 0) return ''
  const open = source.indexOf('{', at)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === '{') depth++
    else if (source[i] === '}') {
      depth--
      if (depth === 0) return source.slice(open, i + 1)
    }
  }
  return ''
}

describe('R447 R300 B5 六页 scoped 根容器渐变令牌修复', () => {
  for (const [name, base] of PAGES) {
    const source = readFileSync(new URL(name, import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1'), 'utf-8')
    const block = scopedBlock(source)

    it(name + ': scoped 根容器使用正确令牌 --ai-gradient-surface', () => {
      expect(block).toMatch(/background:\s*var\(--ai-gradient-surface\);/)
    })

    it(name + ': scoped 根容器不再误引 --ai-surface-gradient', () => {
      expect(block).not.toContain('--ai-surface-gradient')
      expect(source).not.toContain('--ai-surface-gradient')
    })

    it(name + ': 保持 R300 独立 B5 体系（ai-page-b5 + data-testid + 原基类，无三标记追加）', () => {
      expect(source).toMatch(new RegExp('class="' + base + '( [^"]+)* ai-page-b5"'))
      expect(source).toMatch(/data-testid="[a-z-]+-page"/)
      expect(source).toContain('.ai-page-b5.ai-page-b5, .ai-page-b5 .ai-page-b5 {')
      expect(block).toContain('border: 1px solid var(--ai-glass-border)')
      expect(block).toContain('box-shadow: var(--ai-shadow-2)')
      const root = source.slice(source.indexOf('<template'), source.indexOf('</template>'))
      expect(root).not.toMatch(/class="[^"]*\bai-panel\b/)
    })
  }
})
