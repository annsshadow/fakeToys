/**
 * P5-6：readonly 账号不该看见写按钮。
 *
 * 此前 `GET /admin/me` 的封装 `adminMe()` **从未被调用** ——
 * 于是 `role` 从未被读取，readonly 账号看得见全部写按钮，
 * 只有点下去吃一个 403 才知道自己不能写。
 * 而那个 403 出现在**动作已经发生之后**（「已封禁」之后才发现没权限）。
 *
 * 判据：身份是 readonly 时，写按钮**不渲染**。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { provide } from 'vue'

const fetchUsers = vi.hoisted(() => vi.fn())
const adminMe = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchUsers, adminMe }))

import UsersView from '@/views/UsersView.vue'
import { ADMIN_ROLE_KEY } from '@/utils/adminRole'
import { computed, readonly, ref } from 'vue'

function userFixture(overrides: Record<string, unknown> = {}) {
  return {
    id: 1,
    nickname: '测试用户',
    is_guest: true,
    status: 1,
    max_stage: 3,
    power: 100,
    coin: 10,
    gem: 2,
    last_login_at: '2026-01-02T03:04:05Z',
    created_at: '2026-01-01T00:00:00Z',
    verify_checked: 0,
    verify_mismatched: 0,
    ...overrides,
  }
}

async function mountUsers(role: 'admin' | 'readonly') {
  fetchUsers.mockResolvedValue({ items: [userFixture()], total: 1 })
  const roleRef = ref(role)
  const wrapper = mount(UsersView, {
    global: {
      plugins: [ElementPlus],
      provide: {
        [ADMIN_ROLE_KEY as symbol]: {
          role: readonly(roleRef),
          name: readonly(ref('ops')),
          isReadonly: computed(() => roleRef.value === 'readonly'),
        },
      },
    },
  })
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  vi.clearAllMocks()
})

describe('UsersView 只读账号看不到写按钮（P5-6）', () => {
  it('role=admin：发资源 / 封禁 按钮都在', async () => {
    const wrapper = await mountUsers('admin')
    const row = wrapper.find('.el-table__row')
    expect(row.text()).toContain('发资源')
    expect(row.text()).toContain('封禁')
    wrapper.unmount()
  })

  it('role=readonly：发资源 / 封禁 按钮都不渲染', async () => {
    const wrapper = await mountUsers('readonly')
    const row = wrapper.find('.el-table__row')
    expect(row.text()).not.toContain('发资源')
    expect(row.text()).not.toContain('封禁')
    // 读操作仍然可用
    expect(row.text()).toContain('测试用户')
    wrapper.unmount()
  })
})
