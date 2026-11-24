import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'

// R445：9 个"已有 modern-page ai-page 但缺 B5 标记"的管理页，
// 根 div 应升级为 modern-page ai-page-b5 ai-panel，保留原基类。
// 注意：QueryViewApp/QueryExplorerApp/QueryViewDesignerApp 首个 <div> 不是目标，
// 故用全文正则锚定"含原基类 + modern-page ai-page-b5 + ai-panel 的同一 class 属性"，
// 不复用 R444 的 rootClass() 取首个 div 写法。
const viewsDir = fileURLToPath(new URL('./', import.meta.url))

const targets = [
  { file: 'FormApp.vue', base: 'mod-view' },
  { file: 'QueryManager.vue', base: 'query-view' },
  { file: 'QueryViewApp.vue', base: 'mod-view' },
  { file: 'QueryDesigner.vue', base: 'query-designer' },
  { file: 'QueryExplorerApp.vue', base: 'crud-view' },
  { file: 'PortalDesigner.vue', base: 'designer-shell' },
  { file: 'PortalDictDesignerApp.vue', base: 'crud-view' },
  { file: 'PortalPageDesignerApp.vue', base: 'crud-view' },
  { file: 'QueryViewDesignerApp.vue', base: 'designer-shell' },
]

describe('R445 B5 marker on 9 modern-page ai-page views', () => {
  for (const { file, base } of targets) {
    const tpl = readFileSync(viewsDir + file, 'utf-8')

    describe(file, () => {
      it('保留 modern-page 基类', () => {
        expect(tpl).toMatch(/class="[^"]*modern-page[^"]*"/)
      })

      it('根容器 class 含 ai-page-b5（替换裸 ai-page）', () => {
        expect(tpl).toMatch(/class="[^"]*modern-page ai-page-b5[^"]*"/)
      })

      it('同一 class 属性内原基类 + modern-page ai-page-b5 + ai-panel 齐备', () => {
        // 全文精确锚定：规避三页首个 div 非目标的陷阱，且确保 ai-panel 落在根容器
        const anchored = new RegExp(
          'class="[^"]*' + base + '[^"]*modern-page ai-page-b5[^"]*ai-panel[^"]*"'
        )
        expect(tpl).toMatch(anchored)
      })

      it('保留原基类 ' + base, () => {
        expect(tpl).toContain('class="' + base + ' modern-page')
      })
    })
  }
})
