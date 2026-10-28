/**
 * UsersView.vue 测试。
 *
 * 用户页是运营权限最高、风险最大的页面：封禁与发资源都直接改动玩家资产。
 * 分页边界（在第一页点上一页/在最后一页点下一页必须是无操作）、
 * 封禁原因必填、发放格式解析（格式错/未知货币）都是必须锁死的行为。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { defineComponent, h, KeepAlive, nextTick, ref } from 'vue'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { AdminUserDetailResp, AdminUserView } from '@/api'

const fetchUsers = vi.hoisted(() => vi.fn())
const fetchUserDetail = vi.hoisted(() => vi.fn())
const banUser = vi.hoisted(() => vi.fn())
const unbanUser = vi.hoisted(() => vi.fn())
const grantUser = vi.hoisted(() => vi.fn())
const box = vi.hoisted(() => ({ prompt: vi.fn(), confirm: vi.fn() }))

vi.mock('@/api', () => ({ fetchUsers, fetchUserDetail, banUser, unbanUser, grantUser }))
vi.mock('element-plus', async (importOriginal) => {
  const mod = await importOriginal<typeof import('element-plus')>()
  return { ...mod, ElMessageBox: box }
})

import UsersView from '@/views/UsersView.vue'

function userFixture(overrides: Partial<AdminUserView> = {}): AdminUserView {
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
    // 默认「验过且一致」；覆盖 {verify_checked:0,verify_mismatched:0} 即「未验真」
    verify_checked: 5,
    verify_mismatched: 0,
    ...overrides,
  }
}

async function mountUsers(items: AdminUserView[] = [], total = items.length) {
  fetchUsers.mockResolvedValue({ items, total })
  const wrapper = mount(UsersView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return wrapper
}

function detailFixture(overrides: Partial<AdminUserDetailResp> = {}): AdminUserDetailResp {
  return {
    user: userFixture({
      id: 7,
      nickname: '详情玩家',
      max_stage: 12,
      power: 4321,
      verify_checked: 9,
      verify_mismatched: 2,
    }),
    wallet: { coin: 12345, gem: 67, energy: 89, keys: 3 },
    mastery_nodes: [11, 12],
    equipped_skills: [1, 0, 3],
    equipped_equipment: [21, 22],
    recent_battles: [
      {
        id: 100,
        user_id: 7,
        nickname: '详情玩家',
        level_id: 12,
        result: 'win',
        stars: 3,
        score: 9999,
        kills: 42,
        leaked: 0,
        wave_reached: 10,
        duration_ms: 60000,
        reactions: 5,
        heat_max: 88,
        replay_hash: 'hash',
        created_at: '2026-01-02T03:04:05Z',
        total_enemies: 42,
        verify_checked: 1,
        verify_mismatched: 0,
      },
    ],
    verifications: [
      {
        id: 1,
        battle_id: 100,
        verifier_id: null,
        admin_verifier_id: null,
        expected_hash: 'e1',
        actual_hash: 'a1',
        matched: true,
        created_at: '2026-01-02T03:04:05Z',
      },
      {
        id: 2,
        battle_id: 101,
        verifier_id: 3,
        admin_verifier_id: null,
        expected_hash: 'e2',
        actual_hash: 'a2',
        matched: false,
        created_at: '2026-01-02T03:04:06Z',
      },
    ],
    flow_count: 17,
    ...overrides,
  }
}

function deferred<T>() {
  let resolve!: (value: T) => void
  const promise = new Promise<T>((res) => {
    resolve = res
  })
  return { promise, resolve }
}

type Vm = {
  statusOf: (u: AdminUserView) => { text: string; type: string }
  suspicious: (u: AdminUserView) => boolean
  offset: number
  limit: number
  keyword: string
  detail: AdminUserDetailResp | null
  detailLoading: boolean
  load: () => Promise<void>
  openDetail: (u: AdminUserView) => Promise<void>
  prevPage: () => void
  nextPage: () => void
  onBan: (u: AdminUserView) => Promise<void>
  onUnban: (u: AdminUserView) => Promise<void>
  onGrant: (u: AdminUserView) => Promise<void>
}

beforeEach(() => {
  // reset 而不是 clear：clear 只清调用记录，不清 once 队列；
  // 前序用例若提前断言失败，未消费的 mockResolvedValueOnce 会串到后续用例。
  vi.resetAllMocks()
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
    // 空时间显示 —；正常时间渲染成查看者本地时区（第 124 轮：服务端下发 UTC）
    expect(text).toContain('—')
    const d = new Date('2026-01-02T03:04:05Z')
    const p = (n: number) => String(n).padStart(2, '0')
    expect(text).toContain(
      `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())} ${p(d.getHours())}:${p(d.getMinutes())}:${p(d.getSeconds())}`,
    )
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
    expect(normalBtns.map((b) => b.text())).toEqual(['详情', '发资源', '封禁'])
    await normalBtns.find((b) => b.text() === '发资源')!.trigger('click')
    await flushPromises()
    expect(grantUser).toHaveBeenCalledWith(1, 'coin', 10000)
    await rows[0].findAll('button').find((b) => b.text() === '封禁')!.trigger('click')
    await flushPromises()
    expect(banUser).toHaveBeenCalledWith(1, '恶意刷分')

    // 第 2 行（封禁用户）：解封
    const bannedBtns = rows[1].findAll('button')
    expect(bannedBtns.map((b) => b.text())).toEqual(['详情', '发资源', '解封'])
    await bannedBtns.find((b) => b.text() === '解封')!.trigger('click')
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

  it('负数数量 = 回收（保留既有能力），文案必须说「回收」而不是「发放」（第 109 轮）', async () => {
    box.prompt.mockResolvedValueOnce({ value: 'gem -500' })
    grantUser.mockResolvedValueOnce({ wallet: { coin: 1000, gem: 0 } })
    const row = userFixture()
    const wrapper = await mountUsers([row])
    const vm = wrapper.vm as unknown as Vm
    await vm.onGrant(row)
    await flushPromises()
    expect(grantUser).toHaveBeenCalledWith(1, 'gem', -500)
    // 判据落在文案上：运营在界面上必须看得见「这是回收」。
    // 服务端同轮把符号写进了审计事件与流水 reason，
    // UI 说「已发放」会让整个符号链路在最后一环断裂。
    // ⚠️ document.body 会累积同文件前序用例的 ElMessage，
    // 所以只断言**本用例自己的**「钻石 500」组合，不能全局 not.toContain。
    const text = document.body.textContent as string
    expect(text).toContain('已回收 钻石 500')
    expect(text).not.toContain('已发放 钻石 500')
    wrapper.unmount()

    box.prompt.mockRejectedValueOnce('cancel')
    const wrapper2 = await mountUsers([userFixture()])
    await (wrapper2.vm as unknown as Vm).onGrant(userFixture())
    await flushPromises()
    expect(grantUser).toHaveBeenCalledTimes(1) // 只有上一个用例那一次
    wrapper2.unmount()
  })
})

describe('UsersView 验真信号三态', () => {
  /**
   * 判据是**文案**而不是颜色。
   *
   * 颜色（el-tag 的 type）落在渲染后的 class 上，很脆；
   * 而运营真正要读到的信息是「未验真」和「不匹配 3/5」这两个说法本身。
   */
  it('未验真（0/0）标为「未验真」而不是「一致」', async () => {
    // 这一条是本组最关键的：**0/0 是「未知」，不是「干净」**。
    // 如果这里显示成「一致」，那么最不可信的用户看起来最干净。
    const wrapper = await mountUsers([
      userFixture({ id: 1, nickname: '未验真甲', verify_checked: 0, verify_mismatched: 0 }),
    ])
    expect(wrapper.text()).toContain('未验真')
    expect(wrapper.text()).not.toContain('一致 ')
  })

  it('验过且一致显示次数', async () => {
    const wrapper = await mountUsers([
      userFixture({ id: 2, nickname: '干净乙', verify_checked: 8, verify_mismatched: 0 }),
    ])
    expect(wrapper.text()).toContain('一致 8')
  })

  it('有不匹配时显示「不匹配 m/n」', async () => {
    const wrapper = await mountUsers([
      userFixture({ id: 3, nickname: '可疑丙', verify_checked: 5, verify_mismatched: 3 }),
    ])
    expect(wrapper.text()).toContain('不匹配 3/5')
  })

  it('三个状态互不串味（一次挂三个用户）', async () => {
    const wrapper = await mountUsers([
      userFixture({ id: 1, nickname: 'A', verify_checked: 0, verify_mismatched: 0 }),
      userFixture({ id: 2, nickname: 'B', verify_checked: 4, verify_mismatched: 0 }),
      userFixture({ id: 3, nickname: 'C', verify_checked: 4, verify_mismatched: 1 }),
    ])
    const t = wrapper.text()
    expect(t).toContain('未验真')
    expect(t).toContain('一致 4')
    expect(t).toContain('不匹配 1/4')
  })

  it('字段缺失（旧版后端）时按「未验真」处理，不显示 undefined', async () => {
    // 后台连旧版服务端时这两个字段不存在。此时必须显示「未验真」，
    // 而不是 `undefined` —— 页面上的 "undefined" 会让运营以为出了 bug。
    const legacy = userFixture({ id: 9, nickname: '旧版' })
    delete (legacy as Partial<AdminUserView>).verify_checked
    delete (legacy as Partial<AdminUserView>).verify_mismatched
    const wrapper = await mountUsers([legacy])
    expect(wrapper.text()).toContain('未验真')
    expect(wrapper.text()).not.toContain('undefined')
  })
})

/**
 * 2026-10-21（P5-2）：破坏性操作的**二次确认**。
 *
 * 此前：
 *   - 解封：一次点击直接生效（而封禁是要填理由的 —— 不对称）
 *   - 发/回收资源：输入完直接提交，一次手滑 `coin -1000000`
 *     会直接回收玩家 100 万金币，而反馈只是一条几秒后消失的 toast
 *
 * 判据是**可观测后果**：确认框被调用；取消则**不发请求**。
 */
describe('UsersView 破坏性操作需要二次确认（P5-2）', () => {
  beforeEach(() => {
    vi.resetAllMocks()
    fetchUsers.mockResolvedValue({ items: [userFixture({ id: 1, status: 2 })], total: 1 })
  })

  it('解封：先弹确认框；取消则不调用 unbanUser', async () => {
    unbanUser.mockResolvedValueOnce({ user: userFixture({ id: 1, status: 1 }) })
    box.confirm.mockRejectedValueOnce('cancel')

    const wrapper = mount(UsersView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    const rows = wrapper.findAll('.el-table__row')
    const btn = rows[0].findAll('button').find((b) => b.text() === '解封')
    expect(btn, '找不到「解封」按钮').toBeTruthy()
    await btn!.trigger('click')
    await flushPromises()

    expect(box.confirm).toHaveBeenCalled()
    expect(unbanUser).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('解封：确认后才调用 unbanUser', async () => {
    unbanUser.mockResolvedValueOnce({ user: userFixture({ id: 1, status: 1 }) })
    box.confirm.mockResolvedValueOnce('ok')

    const wrapper = mount(UsersView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    const rows = wrapper.findAll('.el-table__row')
    const btn = rows[0].findAll('button').find((b) => b.text() === '解封')
    await btn!.trigger('click')
    await flushPromises()

    expect(box.confirm).toHaveBeenCalled()
    expect(unbanUser).toHaveBeenCalledWith(1)
    wrapper.unmount()
  })

  it('回收资源：确认框里写清回收数量；取消则不调用 grantUser', async () => {
    box.prompt.mockResolvedValueOnce({ value: 'coin -1000000' })
    box.confirm.mockRejectedValueOnce('cancel')

    const wrapper = mount(UsersView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    const rows = wrapper.findAll('.el-table__row')
    const btn = rows[0].findAll('button').find((b) => b.text() === '发资源')
    expect(btn, '找不到「发资源」按钮').toBeTruthy()
    await btn!.trigger('click')
    await flushPromises()

    expect(box.confirm).toHaveBeenCalled()
    // 确认框的文案必须说清**回收多少**（含数字），否则运营看不到那个负号
    const msg = String(box.confirm.mock.calls[0]?.[0] ?? '')
    expect(msg).toContain('1000000')
    expect(grantUser).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('发放资源：确认后才调用 grantUser', async () => {
    grantUser.mockResolvedValueOnce({ wallet: { coin: 11000, gem: 50 } })
    box.prompt.mockResolvedValueOnce({ value: 'coin 10000' })
    box.confirm.mockResolvedValueOnce('ok')

    const wrapper = mount(UsersView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    const rows = wrapper.findAll('.el-table__row')
    const btn = rows[0].findAll('button').find((b) => b.text() === '发资源')
    await btn!.trigger('click')
    await flushPromises()

    expect(box.confirm).toHaveBeenCalled()
    expect(grantUser).toHaveBeenCalledWith(1, 'coin', 10000)
    wrapper.unmount()
  })
})

describe('UsersView 详情抽屉（P5-7）', () => {
  it('点击「详情」加载并渲染玩家、钱包、构筑、战报与验真记录', async () => {
    const row = userFixture({ id: 7, nickname: '详情玩家' })
    fetchUserDetail.mockResolvedValueOnce(detailFixture())
    const wrapper = await mountUsers([row])

    const btn = wrapper.findAll('.el-table__row')[0].findAll('button').find((b) => b.text() === '详情')!
    await btn.trigger('click')
    await flushPromises()

    expect(fetchUserDetail).toHaveBeenCalledWith(7)
    const vm = wrapper.vm as unknown as Vm
    expect(vm.detail?.user.id).toBe(7)
    expect(vm.detailLoading).toBe(false)

    // el-drawer 默认 append-to-body=false，内容仍在组件树内；
    // 而 mount 默认挂在游离节点上，所以断言用 wrapper.text() 而不是 body。
    const text = wrapper.text()
    expect(text).toContain('玩家详情')
    expect(text).toContain('详情玩家')
    expect(text).toContain('12345')
    expect(text).toContain('67')
    expect(text).toContain('89')
    expect(text).toContain('17')
    // 构筑：0 技能显示「空」，装备与专精按 #/顿号拼接
    expect(text).toContain('#1 / 空 / #3')
    expect(text).toContain('#21、#22')
    expect(text).toContain('11、12')
    // 战报与验真两态
    expect(text).toContain('win')
    expect(text).toContain('9999')
    expect(text).toContain('一致')
    expect(text).toContain('不匹配')
    wrapper.unmount()
  })

  it('详情加载失败：抽屉保留、提示错误、loading 复位', async () => {
    fetchUserDetail.mockRejectedValueOnce(new Error('详情库超时'))
    const wrapper = await mountUsers([userFixture({ id: 7 })])

    await (wrapper.vm as unknown as Vm).openDetail(userFixture({ id: 7 }))
    await flushPromises()

    const vm = wrapper.vm as unknown as Vm
    expect(vm.detail).toBeNull()
    expect(vm.detailLoading).toBe(false)
    expect(document.body.textContent).toContain('玩家详情加载失败：详情库超时')
    wrapper.unmount()
  })

  it('慢响应乱序：只接受最新一次详情请求（序号守卫）', async () => {
    const first = deferred<AdminUserDetailResp>()
    const second = deferred<AdminUserDetailResp>()
    fetchUserDetail.mockReturnValueOnce(first.promise).mockReturnValueOnce(second.promise)
    const wrapper = await mountUsers([userFixture({ id: 1 }), userFixture({ id: 2 })])
    const vm = wrapper.vm as unknown as Vm

    const p1 = vm.openDetail(userFixture({ id: 1, nickname: '旧请求' }))
    const p2 = vm.openDetail(userFixture({ id: 2, nickname: '新请求' }))

    // 后发先至：新请求先回，旧请求后回，旧数据不能覆盖新数据。
    second.resolve(detailFixture({ user: userFixture({ id: 2, nickname: '新请求' }) }))
    await p2
    first.resolve(detailFixture({ user: userFixture({ id: 1, nickname: '旧请求' }) }))
    await p1

    expect(vm.detail?.user.id).toBe(2)
    expect(vm.detail?.user.nickname).toBe('新请求')
    expect(vm.detailLoading).toBe(false)
    wrapper.unmount()
  })
})

describe('UsersView 操作失败提示（P2-2）', () => {
  it('封禁失败（非取消）：提示服务端原因，不静默吞掉 403', async () => {
    box.prompt.mockResolvedValueOnce({ value: '恶意刷分' })
    banUser.mockRejectedValueOnce(new Error('403 只读账号'))
    const wrapper = await mountUsers([userFixture()])

    await (wrapper.vm as unknown as Vm).onBan(userFixture())
    await flushPromises()

    expect(document.body.textContent).toContain('封禁失败：403 只读账号')
    wrapper.unmount()
  })

  it('解封失败：提示服务端原因', async () => {
    box.confirm.mockResolvedValueOnce('ok')
    unbanUser.mockRejectedValueOnce(new Error('用户不存在'))
    const wrapper = await mountUsers([userFixture({ status: 2 })])

    await (wrapper.vm as unknown as Vm).onUnban(userFixture({ status: 2 }))
    await flushPromises()

    expect(box.confirm).toHaveBeenCalled()
    expect(unbanUser).toHaveBeenCalledWith(1)
    expect(document.body.textContent).toContain('解封失败：用户不存在')
    wrapper.unmount()
  })

  it('发放失败（确认之后）：提示服务端原因', async () => {
    box.prompt.mockResolvedValueOnce({ value: 'coin 100' })
    box.confirm.mockResolvedValueOnce('ok')
    grantUser.mockRejectedValueOnce(new Error('钱包服务不可用'))
    const wrapper = await mountUsers([userFixture()])

    await (wrapper.vm as unknown as Vm).onGrant(userFixture())
    await flushPromises()

    expect(grantUser).toHaveBeenCalledWith(1, 'coin', 100)
    expect(document.body.textContent).toContain('发放失败：钱包服务不可用')
    wrapper.unmount()
  })
})

describe('UsersView keep-alive 刷新（P5-9）', () => {
  it('首次激活不重复加载，离开再回来才刷新', async () => {
    fetchUsers.mockResolvedValue({ items: [userFixture()], total: 1 })
    const show = ref(true)
    const Other = defineComponent({ render: () => h('div', 'other') })
    const Host = defineComponent({
      setup() {
        return () =>
          h(KeepAlive, null, { default: () => (show.value ? h(UsersView) : h(Other)) })
      },
    })

    const wrapper = mount(Host, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    expect(fetchUsers).toHaveBeenCalledTimes(1)

    show.value = false
    await nextTick()
    show.value = true
    await nextTick()
    await flushPromises()
    expect(fetchUsers).toHaveBeenCalledTimes(2)
    wrapper.unmount()
  })
})

describe('UsersView 分页/搜索边界补充', () => {
  it('上一页在 offset 小于一页时归零（不出现负 offset）', async () => {
    const wrapper = await mountUsers([userFixture()], 120)
    const vm = wrapper.vm as unknown as Vm
    vm.offset = 25
    vm.prevPage()
    await flushPromises()
    expect(vm.offset).toBe(0)
    wrapper.unmount()
  })

  it('搜索框清空：重置 offset 并重新加载', async () => {
    const wrapper = await mountUsers([userFixture()], 120)
    const vm = wrapper.vm as unknown as Vm
    vm.offset = 50
    vm.keyword = '龙'

    wrapper.findComponent({ name: 'ElInput' }).vm.$emit('clear')
    await flushPromises()
    expect(vm.offset).toBe(0)
    expect(fetchUsers).toHaveBeenLastCalledWith({ keyword: '龙', limit: 50, offset: 0 })
    wrapper.unmount()
  })
})
