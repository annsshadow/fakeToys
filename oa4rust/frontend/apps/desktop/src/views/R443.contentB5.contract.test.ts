// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const DIR = new URL('./', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')
// 3 个真实可达的纯裸 mod-view 管理页 + 其原始根容器 class（R303 Exactly 模式：保留原标记）
const PAGES: Array<[string, string]> = [
  ['CategoryApp', 'mod-view'],
  ['ProgramCenterApp', 'mod-view'],
  ['ServerApp', 'mod-view'],
]

function rootClass(name: string): string {
  const src = readFileSync(DIR + name + '.vue', 'utf8')
  const m = src.match(/<div\s+class="([^"]+)"/)
  return m ? m[1] : ''
}

describe('R443 纯裸管理页 B5 统一视觉标记', () => {
  for (const [name, original] of PAGES) {
    describe(name, () => {
      it('根容器含 modern-page', () => {
        expect(rootClass(name)).toContain('modern-page')
      })
      it('根容器含 ai-page-b5', () => {
        expect(rootClass(name)).toContain('ai-page-b5')
      })
      it('根容器含 ai-panel', () => {
        expect(rootClass(name)).toContain('ai-panel')
      })
      it('保留原根容器标记 ' + original, () => {
        expect(rootClass(name)).toContain(original)
      })
    })
  }
})