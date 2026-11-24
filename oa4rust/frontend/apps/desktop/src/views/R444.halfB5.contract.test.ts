// @vitest-environment node
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'

const DIR = new URL("./", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')
const PAGES: Array<[string, string]> = [
  ['ViewApp', 'mod-view'],
  ['FileInfoApp', 'mod-view'],
  ['JPushApp', 'mod-view'],
]
function rootClass(name: string): string {
  const src = readFileSync(DIR + name + '.vue', 'utf8')
  const m = src.match(/<div\s+class="([^"]+)"/)
  return m ? m[1] : ''
}

describe('R444 半B5页视觉统一标记', () => {
  for (const [name, base] of PAGES) {
    describe(name, () => {
      it('根容器含 modern-page', () => { expect(rootClass(name)).toContain('modern-page') })
      it('根容器含 ai-page-b5', () => { expect(rootClass(name)).toContain('ai-page-b5') })
      it('根容器含 ai-panel', () => { expect(rootClass(name)).toContain('ai-panel') })
      it('保留根容器标记 ' + base, () => { expect(rootClass(name)).toContain(base) })
    })
  }
})