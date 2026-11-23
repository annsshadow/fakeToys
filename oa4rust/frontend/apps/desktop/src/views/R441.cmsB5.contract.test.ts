// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const DIR = new URL('./', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')
// 7 个真实可达的 CMS 缺口管理页 + 其原始根容器 class（R303 Exactly 模式：保留原标记）
const PAGES: Array<[string, string]> = [
  ['CmsColumnApp', 'crud-view'],
  ['CmsModuleApp', 'crud-view'],
  ['CmsColumnManagerApp', 'crud-view'],
  ['CmsFormDesignerApp', 'crud-view'],
  ['CmsScriptDesignerApp', 'mod-view'],
  ['CmsViewDesignerApp', 'crud-view'],
  ['CmsXformApp', 'crud-view'],
]

function rootClass(name: string): string {
  const src = readFileSync(DIR + name + '.vue', 'utf8')
  const m = src.match(/<div\s+class="([^"]+)"/)
  return m ? m[1] : ''
}

describe('R441 CMS 缺口管理页 B5 统一视觉标记', () => {
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
