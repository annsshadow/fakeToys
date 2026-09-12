export type PortalWidgetType = 'text' | 'table' | 'chart' | 'metric' | 'link'

export interface PortalWidgetConfig {
  text?: string
  source?: string
  target?: string
}

export interface PortalWidget {
  id: string
  type: PortalWidgetType
  title: string
  width: 1 | 2 | 3 | 4
  config: PortalWidgetConfig
}

export interface PortalDesignContent {
  version: 1
  widgets: PortalWidget[]
}

export interface TableColumnDefinition {
  name: string
  type: 'text' | 'integer' | 'decimal' | 'boolean' | 'date' | 'datetime' | 'json'
  nullable: boolean
}

export interface QueryViewFilter {
  field: string
  operator: 'eq' | 'ne' | 'contains' | 'gt' | 'gte' | 'lt' | 'lte'
  value: string
}

export interface QueryViewSort {
  field: string
  direction: 'asc' | 'desc'
}

export interface QueryViewDefinition {
  version: 1
  sql: string
  filters: QueryViewFilter[]
  sorts: QueryViewSort[]
  paging: { page: number; size: number }
}

export const designerPaths = {
  portalList: '/jaxrs/portal/assemble/designer/list',
  portalCreate: '/jaxrs/portal/assemble/designer/create',
  portalGet: (id: string) => `/jaxrs/portal/assemble/designer/get/${encodeURIComponent(id)}`,
  portalSave: (id: string) => `/jaxrs/portal/assemble/designer/save/${encodeURIComponent(id)}`,
  tableList: '/jaxrs/query/assemble/designer/table/list/manage',
  tableGet: (flag: string) => `/jaxrs/query/assemble/designer/table/${encodeURIComponent(flag)}`,
  tableCreate: '/jaxrs/query/assemble/designer/table',
  tableSave: (flag: string) => `/jaxrs/query/assemble/designer/table/${encodeURIComponent(flag)}`,
  tableExecute: (flag: string) => `/jaxrs/query/assemble/designer/table/${encodeURIComponent(flag)}/execute`,
  viewList: (queryFlag: string) =>
    `/jaxrs/query/assemble/designer/view/list/query/${encodeURIComponent(queryFlag)}`,
  viewGet: (id: string) => `/jaxrs/query/assemble/designer/view/${encodeURIComponent(id)}`,
  viewCreate: '/jaxrs/query/assemble/designer/view',
  viewSave: (id: string) => `/jaxrs/query/assemble/designer/view/${encodeURIComponent(id)}`,
  viewDelete: (id: string) => `/jaxrs/query/assemble/designer/view/${encodeURIComponent(id)}`,
  viewSimulate: (id: string) => `/jaxrs/query/assemble/designer/view/${encodeURIComponent(id)}/simulate`,
  viewBundle: (id: string) => `/jaxrs/query/assemble/designer/view/${encodeURIComponent(id)}/bundle`,
}

export function extractList<T>(data: unknown): T[] {
  if (Array.isArray(data)) return data as T[]
  if (data && typeof data === 'object' && Array.isArray((data as { data?: unknown }).data)) {
    return (data as { data: T[] }).data
  }
  return []
}

export function moveItem<T>(items: readonly T[], from: number, to: number): T[] {
  if (from === to || from < 0 || to < 0 || from >= items.length || to >= items.length) return [...items]
  const next = [...items]
  const [item] = next.splice(from, 1)
  if (item !== undefined) next.splice(to, 0, item)
  return next
}

export function serializePortalContent(widgets: readonly PortalWidget[]): string {
  const content: PortalDesignContent = { version: 1, widgets: widgets.map((widget) => ({ ...widget })) }
  return JSON.stringify(content)
}

export function parsePortalContent(raw: unknown): PortalDesignContent {
  const empty: PortalDesignContent = { version: 1, widgets: [] }
  const wrapped = raw && typeof raw === 'object' && 'components' in raw ? (raw as { components: unknown }).components : raw
  try {
    const parsed = typeof wrapped === 'string' ? JSON.parse(wrapped) : wrapped
    if (!parsed || typeof parsed !== 'object' || !Array.isArray((parsed as PortalDesignContent).widgets)) return empty
    return { version: 1, widgets: (parsed as PortalDesignContent).widgets }
  } catch {
    return empty
  }
}

export function tablePayload(name: string, queryFlag: string, columns: readonly TableColumnDefinition[]) {
  return { name: name.trim(), queryFlag: queryFlag.trim(), columns: columns.map((column) => ({ ...column })) }
}

export function serializeViewDefinition(definition: QueryViewDefinition) {
  return {
    version: 1 as const,
    sql: definition.sql.trim(),
    filters: definition.filters.map((filter) => ({ ...filter })),
    sorts: definition.sorts.map((sort) => ({ ...sort })),
    paging: {
      page: Math.max(1, Math.trunc(definition.paging.page) || 1),
      size: Math.min(500, Math.max(1, Math.trunc(definition.paging.size) || 20)),
    },
  }
}

export function parseViewDefinition(raw: unknown): QueryViewDefinition {
  const empty: QueryViewDefinition = { version: 1, sql: '', filters: [], sorts: [], paging: { page: 1, size: 20 } }
  try {
    const parsed = typeof raw === 'string' ? JSON.parse(raw) : raw
    if (!parsed || typeof parsed !== 'object') return empty
    const value = parsed as Partial<QueryViewDefinition>
    return serializeViewDefinition({
      ...empty,
      sql: typeof value.sql === 'string' ? value.sql : '',
      filters: Array.isArray(value.filters) ? value.filters : [],
      sorts: Array.isArray(value.sorts) ? value.sorts : [],
      paging: value.paging ?? empty.paging,
    })
  } catch {
    return empty
  }
}

export function viewRuntimePayload(definition: QueryViewDefinition) {
  const value = serializeViewDefinition(definition)
  return { filterList: value.filters, parameter: {}, count: value.paging.size }
}
