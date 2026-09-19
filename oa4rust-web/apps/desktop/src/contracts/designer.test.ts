import { describe, expect, it } from 'vitest'
import {
  designerPaths,
  extractList,
  moveItem,
  type PortalWidget,
  parsePortalContent,
  parseViewDefinition,
  serializePortalContent,
  serializeViewDefinition,
  tablePayload,
  viewRuntimePayload,
} from './designer'

const widgets: PortalWidget[] = [
  { id: 'w1', type: 'text', title: '介绍', width: 2, config: { text: 'hello' } },
  { id: 'w2', type: 'table', title: '列表', width: 4, config: { source: 'orders' } },
]

describe('designer contracts', () => {
  it('serializes the portal payload as the backend-required content string and round-trips widgets', () => {
    const content = serializePortalContent(widgets)
    expect(typeof content).toBe('string')
    expect(JSON.parse(content)).toEqual({ version: 1, widgets })
    expect(parsePortalContent({ components: JSON.parse(content) }).widgets).toEqual(widgets)
  })

  it('reorders portal widgets without mutating the original array', () => {
    const reordered = moveItem(widgets, 0, 1)
    expect(reordered.map((widget) => widget.id)).toEqual(['w2', 'w1'])
    expect(widgets.map((widget) => widget.id)).toEqual(['w1', 'w2'])
  })

  it('keeps QueryTable create/edit payload typed and never accepts raw SQL', () => {
    const payload = tablePayload(' Orders ', ' sales ', [{ name: 'total', type: 'decimal', nullable: false }])
    expect(payload).toEqual({
      name: 'Orders',
      queryFlag: 'sales',
      columns: [{ name: 'total', type: 'decimal', nullable: false }],
    })
    expect(payload).not.toHaveProperty('sql')
  })

  it('serializes view filter, sort, and paging and maps runtime filter/count fields', () => {
    const definition = serializeViewDefinition({
      version: 1,
      sql: ' SELECT id FROM orders ',
      filters: [{ field: 'status', operator: 'eq', value: 'open' }],
      sorts: [{ field: 'created_at', direction: 'desc' }],
      paging: { page: 0, size: 999 },
    })
    expect(definition.sql).toBe('SELECT id FROM orders')
    expect(definition.paging).toEqual({ page: 1, size: 500 })
    expect(parseViewDefinition(JSON.stringify(definition))).toEqual(definition)
    expect(viewRuntimePayload(definition)).toEqual({
      filterList: definition.filters,
      parameter: {},
      count: 500,
    })
  })

  it('encodes identifiers in every dynamic backend path', () => {
    expect(designerPaths.tableExecute('a/b')).toBe('/api/query/assemble/designer/table/a%2Fb/execute')
    expect(designerPaths.viewList('sales north')).toBe('/api/query/assemble/designer/view/list/query/sales%20north')
  })

  it('encodes identifiers in the remaining portal/table/view path builders', () => {
    // 路径注入防护：任何 id 段都必须 encodeURIComponent，逐构造器钉死。
    const bad = 'a/b c'
    const enc = 'a%2Fb%20c'
    expect(designerPaths.portalGet(bad)).toBe(`/api/portal/assemble/designer/get/${enc}`)
    expect(designerPaths.portalSave(bad)).toBe(`/api/portal/assemble/designer/save/${enc}`)
    expect(designerPaths.tableGet(bad)).toBe(`/api/query/assemble/designer/table/${enc}`)
    expect(designerPaths.tableSave(bad)).toBe(`/api/query/assemble/designer/table/${enc}`)
    expect(designerPaths.viewGet(bad)).toBe(`/api/query/assemble/designer/view/${enc}`)
    expect(designerPaths.viewSave(bad)).toBe(`/api/query/assemble/designer/view/${enc}`)
    expect(designerPaths.viewDelete(bad)).toBe(`/api/query/assemble/designer/view/${enc}`)
    expect(designerPaths.viewSimulate(bad)).toBe(`/api/query/assemble/designer/view/${enc}/simulate`)
    expect(designerPaths.viewBundle(bad)).toBe(`/api/query/assemble/designer/view/${enc}/bundle`)
  })

  it('extractList unwraps both bare arrays and {data} envelopes, degrading to []', () => {
    expect(extractList(['a'])).toEqual(['a'])
    expect(extractList({ data: [1, 2] })).toEqual([1, 2])
    expect(extractList({ other: [] })).toEqual([])
    expect(extractList(null)).toEqual([])
  })

  it('moveItem returns an unmutated copy for out-of-bounds or no-op moves', () => {
    const items = [1, 2, 3]
    expect(moveItem(items, 5, 0)).toEqual([1, 2, 3])
    expect(moveItem(items, -1, 0)).toEqual([1, 2, 3])
    expect(moveItem(items, 1, 1)).toEqual([1, 2, 3])
    expect(items).toEqual([1, 2, 3])
  })

  it('parsePortalContent degrades to an empty design for broken or widget-less input', () => {
    expect(parsePortalContent('not json')).toEqual({ version: 1, widgets: [] })
    expect(parsePortalContent({ components: { widgets: 'nope' } })).toEqual({ version: 1, widgets: [] })
  })

  it('parseViewDefinition degrades to the empty view for null-ish or broken JSON', () => {
    const empty = { version: 1, sql: '', filters: [], sorts: [], paging: { page: 1, size: 20 } }
    expect(parseViewDefinition('null')).toEqual(empty)
    expect(parseViewDefinition('{broken')).toEqual(empty)
  })
})
