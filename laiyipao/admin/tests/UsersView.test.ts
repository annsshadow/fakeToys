/**
 * UsersView.vue 测试。
 *
 * 用户页是运营权限最高、风险最大的页面：封禁与发资源都直接改动玩家资产。
 * 分页边界（在第一页点上一页/在最后一页点下一页必须是无操作）、
 * 封禁原因必填、发放格式解析（格式错/未知货币）都是必须锁死的行为。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { AdminUser } from '@/api'

const fetchUsers = vi.hoisted(() => vi.fn())
const banUser = vi.hoisted(() => vi.fn())
const unbanUser = vi.hoisted(() => vi.fn())
const grantUser = vi.hoisted(() => vi.fn())
const box = vi.hoisted(() => ({ prompt: vi.fn(), confirm: vi.fn() }))

vi.mock('@/api', () => ({ fetchUsers, banUser, unbanUser, grantUser }))
vi.mock('element-plus', async (importOriginal) => {
  const mod = await importOriginal<typeof import('element-plus')>()
  return { ...mod, ElMessageBox: box }
})

import UsersView from '@/views/UsersView.vue'

function userFixture(overrides: Partial<AdminUser> = {}): AdminUser {
  return {
    id: 1,
    nickname: '玩家甲',
    is_guest: false,
    status: 1,
    max_stage: 10,
    power: 3000,
    coin: 1000,
    gem: 50,
    last_login_at: '2026-01-02T03:04:05Z',
    created_at: '2025-12-01T00:00:00Z',
    ...overrides,
  }
}

async function mountUsers(items: AdminUser[] = [], total = items.length) {
  fetchUsers.mockResolvedValue({ items, total })
  const wrapper = mount(UsersView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return wrapper
}

type Vm = {
  statusOf: (u: AdminUser) => { text: string; type: string }
  suspicious: (u: AdminUser) => boolean
  offset: number
  limit: number
  keyword: string
  load: () => Promise<void>
  prevPage: () => void
  nextPage: () => void
  onBan: (u: AdminUser) => Promise<void>
  onUnban: (u: AdminUser) => Promise<void>
  onGrant: (u: AdminUser) => Promise<void>
}

beforeEach(() => {
  vi.clearAllMocks()
})

describe('UsersView 列表渲染', () => {
  it('状态/游客/未命名/需核查标记/时间格式化如实渲染', async () => {
    const wrapper = await mountUsers([
      userFixture({ id: 1, status: 1, power: 6000, max_stage: 1 }),
      userFixture({ id: 2, status: 2, nickname: '', is_guest: true, last_login_at: '' }),
      userFixture({ id: 3, power: 6000, max_stage: 3 }),
      userFixture({ id: 4, power: 100, max_stage: 1 }),
    ])
    const text = wrapper.text()
    // 状态两种
    expect(text).toContain('正常')
    expect(text).toContain('封禁')
    // 未命名与游客标记
    expect(text).toContain('（未命名）')
    expect(text).toContain('游客')
    // 仅 power>5000 && max_stage<=1 的玩家被标记需核查
    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].text()).toContain('需核查')
    expect(rows[1].text()).not.toContain('需核查')
    expect(rows[2].text()).not.toContain('需核查')
    expect(rows[3].text()).not.toContain('需核查')
    // 空时间显示 —，正常时间 T 换空格并截断秒
    expect(text).toContain('—')
    expect(text).toContain('2026-01-02 03:04:05')
    wrapper.unmount()
  })

  it('statusOf / suspicious 的分支语义直测', async () => {
    const wrapper = await mountUsers()
    const vm = wrapper.vm as unknown as Vm
    expect(vm.statusOf(userFixture({ status: 1 }))).toEqual({ text: '正常', type: 'success' })
    expect(vm.statusOf(userFixture({ status: 2 }))).toEqual({ text: '封禁', type: 'danger' })
    expect(vm.suspicious(userFixture({ power: 5001, max_stage: 1 }))).toBe(true)
    expect(vm.suspicious(userFixture({ power: 5001, max_stage: 2 }))).toBe(false)
    expect(vm.suspicious(userFixture({ power: 5000, max_stage: 1 }))).toBe(false)
    wrapper.unmount()
  })

  it('加载失败：弹出错误消息', async () => {
    fetchUsers.mockRejectedValueOnce(new Error('用户库超时'))
    const wrapper = mount(UsersView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    expect(document.body.textContent).toContain('用户加载失败：用户库超时')
    wrapper.unmount()
  })
})

describe('UsersView 分页', () => {
  it('下一页翻页并带新 offset 请求；到最后一页后按钮无操作', async () => {
    const wrapper = await mountUsers([userFixture()], 120)
    const vm = wrapper.vm as unknown as Vm

    vm.nextPage()
    await flushPromises()
    expect(vm.offset).toBe(50)
    expect(fetchUsers).toHaveBeenLastCalledWith({ keyword: undefined, limit: 50, offset: 50 })

    vm.nextPage()
    await flushPromises()
    expect(vm.offset).toBe(100)

    // offset 100 + limit 50 = 150 >= 120 → 无操作
    vm.nextPage()
    expect(vm.offset).toBe(100)
    wrapper.unmount()
  })

  it('上一页翻页；在第一页时无操作', async () => {
    const wrapper = await mountUsers([userFixture()], 120)
    const vm = wrapper.vm as unknown as Vm

    // 第一页无操作
    vm.prevPage()
    expect(vm.offset).toBe(0)

    vm.offset = 100
    vm.prevPage()
    await flushPromises()
    expect(vm.offset).toBe(50)
    expect(fetchUsers).toHaveBeenLastCalledWith({ keyword: undefined, limit: 50, offset: 50 })

    vm.prevPage()
    await flushPromises()
    expect(vm.offset).toBe(0)
    // offset 0 已是下界 → Math.max 保护，不再翻负
    vm.prevPage()
    expect(vm.offset).toBe(0)
    wrapper.unmount()
  })

  it('搜索按钮重置 offset 到第一页并携带关键词', async () => {
    const wrapper = await mountUsers([userFixture()], 120)
    const vm = wrapper.vm as unknown as Vm
    vm.offset = 50
    vm.keyword = '龙'

    await wrapper.findAll('button').find((b) => b.text() === '查询')!.trigger('click')
    await flushPromises()
    expect(vm.offset).toBe(0)
    expect(fetchUsers).toHaveBeenLastCalledWith({ keyword: '龙', limit: 50, offset: 0 })
    wrapper.unmount()
  })

  it('搜索框回车等效查询（keyup.enter 重置 offset）', async () => {
    const wrapper = await mountUsers([userFixture()], 120)
    const vm = wrapper.vm as unknown as Vm
    vm.offset = 50

    const input = wrapper.find('.toolbar input')
    await input.setValue('火')
    await input.trigger('keyup.enter')
    await flushPromises()
    expect(vm.offset).toBe(0)
    expect(fetchUsers).toHaveBeenLastCalledWith({ keyword: '火', limit: 50, offset: 0 })
    wrapper.unmount()
  })
})

describe('UsersView 封禁 / 解封', () => {
  it('封禁成功：原因 trim 后提交，行状态合并，提示已封禁', async () => {
    box.prompt.mockResolvedValueOnce({ value: '  恶意刷分  ' })
    const row = userFixture({ status: 1 })
    banUser.mockResolvedValueOnce({ user: { ...row, status: 2 } })
    const wrapper = await mountUsers([row])
    const vm = wrapper.vm as unknown as Vm
    await vm.onBan(row)
    await flushPromises()

    expect(banUser).toHaveBeenCalledWith(1, '恶意刷分')
    expect(row.status).toBe(2)
    expect(document.body.textContent).toContain('已封禁')
    wrapper.unmount()
  })

  it('封禁原因全空白：不提交（审计日志不允许空原因）', async () => {
    box.prompt.mockResolvedValueOnce({ value: '   ' })
    const wrapper = await mountUsers([userFixture()])
    const vm = wrapper.vm as unknown as Vm
    await vm.onBan(userFixture())
    await flushPromises()
    expect(banUser).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('封禁弹窗取消：不提交', async () => {
    box.prompt.mockRejectedValueOnce('cancel')
    const wrapper = await mountUsers([userFixture()])
    const vm = wrapper.vm as unknown as Vm
    await vm.onBan(userFixture())
    await flushPromises()
    expect(banUser).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('操作列按钮逐个点击：发资源 / 封禁（正常用户）/ 解封（封禁用户）', async () => {
    const normal = userFixture({ id: 1, status: 1 })
    const banned = userFixture({ id: 2, status: 2 })
    fetchUsers.mockResolvedValue({ items: [normal, banned], total: 2 })
    grantUser.mockResolvedValueOnce({ wallet: { coin: 11000, gem: 50 } })
    banUser.mockResolvedValueOnce({ user: { ...normal, status: 2 } })
    unbanUser.mockResolvedValueOnce({ user: { ...banned, status: 1 } })
    box.prompt
      .mockResolvedValueOnce({ value: 'coin 10000' })
      .mockResolvedValueOnce({ value: '恶意刷分' })

    const wrapper = mount(UsersView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    const rows = wrapper.findAll('.el-table__row')

    // 第 1 行（正常用户）：发资源 → 封禁
    const normalBtns = rows[0].findAll('button')
    expect(normalBtns.map((b) => b.text())).toEqual(['发资源', '封禁'])
    await normalBtns[0].trigger('click')
    await flushPromises()
    expect(grantUser).toHaveBeenCalledWith(1, 'coin', 10000)
    await rows[0].findAll('button')[1].trigger('click')
    await flushPromises()
    expect(banUser).toHaveBeenCalledWith(1, '恶意刷分')

    // 第 2 行（封禁用户）：解封
    const bannedBtns = rows[1].findAll('button')
    expect(bannedBtns.map((b) => b.text())).toEqual(['发资源', '解封'])
    await bannedBtns[1].trigger('click')
    await flushPromises()
    expect(unbanUser).toHaveBeenCalledWith(2)
    wrapper.unmount()
  })

  it('解封成功：状态合并回正常，提示已解封', async () => {
    const row = userFixture({ status: 2 })
    unbanUser.mockResolvedValueOnce({ user: { ...row, status: 1 } })
    const wrapper = await mountUsers([row])
    const vm = wrapper.vm as unknown as Vm
    await vm.onUnban(row)
    await flushPromises()
    expect(unbanUser).toHaveBeenCalledWith(1)
    expect(row.status).toBe(1)
    expect(document.body.textContent).toContain('已解封')
    wrapper.unmount()
  })
})

describe('UsersView 发放资源', () => {
  it('格式正确：按货币 key 与数量提交，行内余额刷新', async () => {
    // ⚠️ 现有实现校验的是货币 key（coin/gem/energy/keys），而弹窗提示文案写的是
    // 「金币 10000」这种中文标签 —— 两者不一致（疑似产品 bug，已在测试报告标记）。
    // 这里锁定实现契约：key 才能通过。
    box.prompt.mockResolvedValueOnce({ value: 'coin 10000' })
    const row = userFixture()
    grantUser.mockResolvedValueOnce({ wallet: { coin: 11000, gem: 60 } })
    const wrapper = await mountUsers([row])
    const vm = wrapper.vm as unknown as Vm
    await vm.onGrant(row)
    await flushPromises()

    expect(grantUser).toHaveBeenCalledWith(1, 'coin', 10000)
    expect(row.coin).toBe(11000)
    expect(row.gem).toBe(60)
    expect(document.body.textContent).toContain('已发放')
    wrapper.unmount()
  })

  it('按弹窗提示输入中文标签「金币 10000」会被拒为未知货币（文案与解析不一致的行为快照）', async () => {
    box.prompt.mockResolvedValueOnce({ value: '金币 10000' })
    const wrapper = await mountUsers([userFixture()])
    const vm = wrapper.vm as unknown as Vm
    await vm.onGrant(userFixture())
    await flushPromises()
    expect(document.body.textContent).toContain('未知货币：金币')
    expect(grantUser).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('格式错误（缺数量）：提示格式并拒绝提交', async () => {
    box.prompt.mockResolvedValueOnce({ value: '金币abc' })
    const wrapper = await mountUsers([userFixture()])
    const vm = wrapper.vm as unknown as Vm
    await vm.onGrant(userFixture())
    await flushPromises()
    expect(document.body.textContent).toContain('格式错误，应为「货币名 数量」')
    expect(grantUser).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('未知货币：提示并拒绝提交（防止给不存在币种发钱）', async () => {
    box.prompt.mockResolvedValueOnce({ value: '元宝 5' })
    const wrapper = await mountUsers([userFixture()])
    const vm = wrapper.vm as unknown as Vm
    await vm.onGrant(userFixture())
    await flushPromises()
    expect(document.body.textContent).toContain('未知货币：元宝')
    expect(grantUser).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('负数数量也允许提交（回收场景），弹窗取消则不提交', async () => {
    box.prompt.mockResolvedValueOnce({ value: 'gem -500' })
    grantUser.mockResolvedValueOnce({ wallet: { coin: 1000, gem: 0 } })
    const row = userFixture()
    const wrapper = await mountUsers([row])
    const vm = wrapper.vm as unknown as Vm
    await vm.onGrant(row)
    await flushPromises()
    expect(grantUser).toHaveBeenCalledWith(1, 'gem', -500)
    wrapper.unmount()

    box.prompt.mockRejectedValueOnce('cancel')
    const wrapper2 = await mountUsers([userFixture()])
    await (wrapper2.vm as unknown as Vm).onGrant(userFixture())
    await flushPromises()
    expect(grantUser).toHaveBeenCalledTimes(1) // 只有上一个用例那一次
    wrapper2.unmount()
  })
})
