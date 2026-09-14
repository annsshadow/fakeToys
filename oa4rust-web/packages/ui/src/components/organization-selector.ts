import { type ApiResponse, api } from '@oa4rust/sdk'

export const ORGANIZATION_SELECTOR_TYPES = ['unit', 'person', 'identity'] as const

export type OrganizationSelectorType = (typeof ORGANIZATION_SELECTOR_TYPES)[number]
export type OrganizationSelectorMode = 'single' | 'multiple'

export interface OrganizationSelectorItem {
  id: string
  name: string
  type: OrganizationSelectorType
  parentId?: string
  unitId?: string
  personId?: string
  mobile?: string
  email?: string
}

interface OrganizationSearchRecord {
  id?: unknown
  unique?: unknown
  name?: unknown
  distinguishedName?: unknown
  parentId?: unknown
  parent_id?: unknown
  unitId?: unknown
  unit_id?: unknown
  personId?: unknown
  person_id?: unknown
  mobile?: unknown
  email?: unknown
}

export type OrganizationSelectorSearch = (
  type: OrganizationSelectorType,
  keyword: string,
) => Promise<OrganizationSelectorItem[]>

const SEARCH_ROOT = '/jaxrs/organization/assemble/control'

export function organizationSelectorSearchPath(type: OrganizationSelectorType): string {
  return `${SEARCH_ROOT}/${type}/list/like`
}

function stringField(value: unknown): string | undefined {
  return typeof value === 'string' && value.length > 0 ? value : undefined
}

export function normalizeOrganizationSelectorItem(
  type: OrganizationSelectorType,
  record: OrganizationSearchRecord,
): OrganizationSelectorItem | null {
  const id = stringField(record.id) ?? stringField(record.unique)
  const name = stringField(record.name) ?? stringField(record.distinguishedName)
  if (!id || !name) return null

  return {
    id,
    name,
    type,
    parentId: stringField(record.parentId) ?? stringField(record.parent_id),
    unitId: stringField(record.unitId) ?? stringField(record.unit_id),
    personId: stringField(record.personId) ?? stringField(record.person_id),
    mobile: stringField(record.mobile),
    email: stringField(record.email),
  }
}

export async function searchOrganizationSelector(
  type: OrganizationSelectorType,
  keyword: string,
): Promise<OrganizationSelectorItem[]> {
  const response = await api.put<OrganizationSearchRecord[]>(organizationSelectorSearchPath(type), {
    key: keyword.trim(),
  })

  return normalizeOrganizationSearchResponse(type, response)
}

export function normalizeOrganizationSearchResponse(
  type: OrganizationSelectorType,
  response: Pick<ApiResponse<OrganizationSearchRecord[]>, 'data'>,
): OrganizationSelectorItem[] {
  if (!Array.isArray(response.data)) return []

  return response.data.flatMap((record) => {
    const item = normalizeOrganizationSelectorItem(type, record)
    return item ? [item] : []
  })
}
