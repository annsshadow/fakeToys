import { api } from '@oa4rust/sdk'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import {
  normalizeOrganizationSearchResponse,
  normalizeOrganizationSelectorItem,
  organizationSelectorSearchPath,
  searchOrganizationSelector,
} from './organization-selector'

vi.mock('@oa4rust/sdk', () => ({
  api: { put: vi.fn() },
}))

const put = vi.mocked(api.put)

describe('organization selector API contract', () => {
  beforeEach(() => {
    put.mockReset()
  })

  it.each(['unit', 'person', 'identity'] as const)(
    'uses the authenticated SDK and PUT list/like contract for %s searches',
    async (type) => {
      put.mockResolvedValue({ data: [], success: true })

      await searchOrganizationSelector(type, '  张三  ')

      expect(organizationSelectorSearchPath(type)).toBe(`/jaxrs/organization/assemble/control/${type}/list/like`)
      expect(put).toHaveBeenCalledWith(organizationSelectorSearchPath(type), { key: '张三' })
    },
  )

  it('normalizes unit, person and identity rows without losing their entity type', () => {
    expect(
      normalizeOrganizationSelectorItem('unit', {
        id: 'unit-1',
        name: '研发部',
        parent_id: 'unit-root',
      }),
    ).toEqual({
      id: 'unit-1',
      name: '研发部',
      type: 'unit',
      parentId: 'unit-root',
      unitId: undefined,
      personId: undefined,
      mobile: undefined,
      email: undefined,
    })
    expect(
      normalizeOrganizationSelectorItem('person', {
        id: 'person-1',
        name: '张三',
        unitId: 'unit-1',
        mobile: '13800000000',
      }),
    ).toMatchObject({ id: 'person-1', name: '张三', type: 'person', unitId: 'unit-1' })
    expect(
      normalizeOrganizationSelectorItem('identity', {
        id: 'identity-1',
        name: '研发经理',
        unit_id: 'unit-1',
        person_id: 'person-1',
      }),
    ).toMatchObject({
      id: 'identity-1',
      name: '研发经理',
      type: 'identity',
      unitId: 'unit-1',
      personId: 'person-1',
    })
  })

  it('drops malformed API records so v-model values always carry stable identity', () => {
    expect(
      normalizeOrganizationSearchResponse('person', {
        data: [{ id: 'person-1', name: '张三' }, { id: 'missing-name' }, { name: 'missing-id' }],
      }),
    ).toEqual([
      {
        id: 'person-1',
        name: '张三',
        type: 'person',
        parentId: undefined,
        unitId: undefined,
        personId: undefined,
        mobile: undefined,
        email: undefined,
      },
    ])
  })
})
