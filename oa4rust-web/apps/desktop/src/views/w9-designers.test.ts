import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string) => readFileSync(resolve(import.meta.dirname, name), 'utf8')

describe('W9 designer interactions', () => {
  it('PortalDesigner persists drag/drop widget configuration through the real design save endpoint', () => {
    const source = read('PortalDesigner.vue')
    expect(source).toContain('@dragstart="dragModule = module"')
    expect(source).toContain('@drop.stop="dropWidget(index)"')
    expect(source).toContain('serializePortalContent(widgets.value)')
    expect(source).toContain('designerPaths.portalSave(activeId.value)')
    expect(source).not.toContain('功能开发中')
  })

  it('QueryTableDesigner uses typed columns and the safe backend execute path without raw SQL', () => {
    const source = read('QueryTableDesignerApp.vue')
    expect(source).toContain('tablePayload(form.value.name,form.value.queryFlag,form.value.columns)')
    expect(source).toContain('designerPaths.tableExecute(activeFlag.value)')
    expect(source).not.toContain('v-model="sql"')
    expect(source).not.toContain('/surface/table/list')
  })

  it('QueryViewDesigner exposes only contracted visual capabilities and real simulate/bundle calls', () => {
    const source = read('QueryViewDesignerApp.vue')
    expect(source).toContain('form.definition.filters')
    expect(source).toContain('form.definition.sorts')
    expect(source).toContain('form.definition.paging')
    expect(source).toContain('designerPaths.viewSimulate(activeId.value)')
    expect(source).toContain('designerPaths.viewBundle(activeId.value)')
    expect(source).toContain('当前后端 query designer/surface 无 lookup 路由或存储契约')
    expect(source).not.toContain('crud-view')
  })
})
