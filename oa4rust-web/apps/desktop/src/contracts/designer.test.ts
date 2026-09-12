import { describe, expect, it } from 'vitest'
import {
  designerPaths,
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
    expect(designerPaths.tableExecute('a/b')).toBe('/jaxrs/query/assemble/designer/table/a%2Fb/execute')
    expect(designerPaths.viewList('sales north')).toBe('/jaxrs/query/assemble/designer/view/list/query/sales%20north')
  })
})
