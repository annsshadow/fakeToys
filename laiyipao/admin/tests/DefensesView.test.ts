/**
 * DefensesView.vue 测试。
 *
 * 防线值守页的价值全在异常识别：极端胜率（>98% 或 <2%）且样本量 ≥3
 * 才值得人工抽查。filter 的早退（样本不足）与 || 的两侧、
 * winRate/rateTone 的零样本保护都必须被数据覆盖到。
 *
 * ───────────────────────────────────────────────────────────────────────────
 * 2026-10-21：fixture 换成了**服务端真实形状**。
 *
 * 此前 fixture 用 `user_id`（服务端给的是 `owner_id` + `owner_name`），
 * 于是「属主」列在测试里是绿的、在线上是空的 —— 测的从来不是线上那个形状。
 * 现在 fixture 与 `service.AdminDefenseView` 一致，字段名拼错**编译期就会红**。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { createMemoryHistory, createRouter } from 'vue-router'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const fetchDefenses = vi.hoisted(() => vi.fn())
const fetchAdminDefenseChallenges = vi.hoisted(() => vi.fn())

vi.mock('@/api', () => ({
  fetchDefenses,
  fetchAdminDefenseChallenges,
}))

import DefensesView from '@/views/DefensesView.vue'

/** 与 service.AdminDefenseView 一致（2026-10-21）。 */
function defenseFixture(overrides: Record<string, unknown> = {}) {
  return {
    id: 1,
    owner_id: 9,
    owner_name: '铁壁玩家',
    name: '铁壁',
    power: 8000,
    element_coverage: 4,
    mastery_done: 6,
    wins: 50,
    losses: 50,
    shielded_until: '2026-02-01T08:00:00Z',
    snapshot_hash: 'hash0123456789abcdef',
    updated_at: '2026-01-15T12:00:00Z',
    ...overrides,
  }
}

/** 属主列是 el-link（第 50 轮），点击跳转用户页 —— 必须注入真实 router。 */
let lastRouter: ReturnType<typeof createRouter> | null = null

async function makeRouter() {
  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: '/defenses', component: { template: '<div />' } },
      { path: '/users', component: { template: '<div />' } },
    ],
  })
  await router.push('/defenses')
  await router.isReady()
  lastRouter = router
  return router
}

async function mountDefenses(items: Array<Record<string, unknown>> = [], total = items.length) {
  fetchDefenses.mockResolvedValue({ items, total })
  const router = await makeRouter()
  const wrapper = mount(DefensesView, { global: { plugins: [ElementPlus, router] } })
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  vi.clearAllMocks()
})

describe('DefensesView 异常胜率识别', () => {
  it('全 100% / 全 0% 且样本 ≥3 的防线被点名；正常与样本不足的不误伤', async () => {
    const wrapper = await mountDefenses([
      defenseFixture({ id: 1, wins: 100, losses: 0 }), // 100% → 异常
      defenseFixture({ id: 2, wins: 0, losses: 50 }), // 0% → 异常
      defenseFixture({ id: 3, wins: 90, losses: 10 }), // 90% → 正常区间（但色阶 warning）
      defenseFixture({ id: 4, wins: 1, losses: 1 }), // 样本 2 < 3 → 过滤早退
      defenseFixture({ id: 5, wins: 0, losses: 0 }), // 零样本 → 早退
    ])
    expect(wrapper.text()).toContain('2 条防线胜率异常（接近 100% 或 0%）')

    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].find('.el-tag--danger').exists()).toBe(true)
    expect(rows[1].find('.el-tag--danger').exists()).toBe(true)
    expect(rows[2].find('.el-tag--danger').exists()).toBe(false)
    expect(rows[2].find('.el-tag--warning').exists()).toBe(true)
    wrapper.unmount()
  })

  it('无异常防线时不渲染告警条', async () => {
    const wrapper = await mountDefenses([defenseFixture({ wins: 50, losses: 50 })])
    expect(wrapper.text()).not.toContain('条防线胜率异常')
    wrapper.unmount()
  })

  it('winRate / rateTone 的零样本与区间边界', async () => {
    const wrapper = await mountDefenses([
      defenseFixture({ id: 1, wins: 0, losses: 0 }), // 零样本
      defenseFixture({ id: 2, wins: 60, losses: 40 }), // 0.6 → success
      defenseFixture({ id: 3, wins: 76, losses: 24 }), // 0.76 → warning
    ])
    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].text()).toContain('—')
    expect(rows[0].find('.el-tag--info').exists()).toBe(true)
    expect(rows[1].find('.el-tag--success').exists()).toBe(true)
    expect(rows[2].find('.el-tag--warning').exists()).toBe(true)
    wrapper.unmount()
  })
})

describe('DefensesView 渲染细节', () => {
  /**
   * 2026-10-21（P2-2）：这两列此前**恒空**，而测试是绿的 ——
   * 因为 fixture 用了前端自己编的字段名。
   *
   * 现在断言的是「服务端给的字段真的显示出来了」：
   * 属主读 `owner_id`/`owner_name`，快照哈希读 `snapshot_hash`。
   */
  it('属主与快照哈希两列不再是空的（P2-2）', async () => {
    const wrapper = await mountDefenses([defenseFixture()])
    const text = wrapper.text()
    expect(text).toContain('铁壁玩家') // owner_name
    expect(text).toContain('9') // owner_id
    expect(text).toContain('hash0123456789ab…') // snapshot_hash 截断 16 位
    wrapper.unmount()
  })

  it('防线名 / 战力 / 元素覆盖 / 专精点 都取真实字段', async () => {
    const wrapper = await mountDefenses([defenseFixture()])
    const text = wrapper.text()
    expect(text).toContain('铁壁')
    expect(text).toContain('8000')
    expect(text).toContain('4/5')
    expect(text).toContain('6')
    wrapper.unmount()
  })

  it('空名回退「（未命名）」；空快照哈希显示「—」', async () => {
    const wrapper = await mountDefenses([
      defenseFixture({ id: 1, name: '', snapshot_hash: '' }),
    ])
    const text = wrapper.text()
    expect(text).toContain('（未命名）')
    expect(text).toContain('—')
    wrapper.unmount()
  })

  it('刷新按钮重新拉取（带 limit/offset）', async () => {
    const wrapper = await mountDefenses()
    expect(fetchDefenses).toHaveBeenCalledTimes(1)
    await wrapper.findAll('button').find((b) => b.text() === '刷新')!.trigger('click')
    await flushPromises()
    expect(fetchDefenses).toHaveBeenCalledTimes(2)
    // 2026-10-21（P2-6）：防线列表分页了 —— limit/offset 一起发。
    // 此前固定 limit=100 且**没有** offset，于是第 101 条起不可达。
    // 2026-10-31（新一轮第 19 轮）：第三个参数是筛选对象（无筛选时全 undefined）。
    expect(fetchDefenses).toHaveBeenLastCalledWith(50, 0, { keyword: undefined, owner_id: undefined })
    wrapper.unmount()
  })

  it('加载失败：弹出错误消息', async () => {
    fetchDefenses.mockRejectedValueOnce(new Error('防线服务超时'))
    const router = await makeRouter()
    const wrapper = mount(DefensesView, { global: { plugins: [ElementPlus, router] } })
    await flushPromises()
    expect(document.body.textContent).toContain('防线加载失败：防线服务超时')
    wrapper.unmount()
  })
})

describe('DefensesView 属主跳转用户详情（第 50 轮）', () => {
  it('点击属主昵称：跳转 /users 并携带 owner_id', async () => {
    const wrapper = await mountDefenses([defenseFixture({ owner_id: 9, owner_name: '铁壁玩家' })])
    const link = wrapper.find('.el-table .el-link')
    expect(link.exists(), '找不到属主列链接').toBe(true)
    expect(link.text()).toBe('铁壁玩家')

    await link.trigger('click')
    await flushPromises()

    expect(lastRouter?.currentRoute.value.path).toBe('/users')
    expect(lastRouter?.currentRoute.value.query.user_id).toBe('9')
    wrapper.unmount()
  })

  it('无属主昵称时显示「—」，不渲染可点击链接', async () => {
    const wrapper = await mountDefenses([defenseFixture({ owner_name: '' })])
    expect(wrapper.find('.el-table .el-link').exists()).toBe(false)
    expect(wrapper.text()).toContain('—')
    wrapper.unmount()
  })
})

describe('DefensesView 挑战记录挑战者跳转用户详情（第 65 轮）', () => {
  function challengeFixture(overrides: Record<string, unknown> = {}): Record<string, unknown> {
    return {
      id: 1,
      defense_id: 1,
      defense_name: '铁壁',
      challenger_id: 7,
      challenger_nickname: '挑战者甲',
      won: false,
      duration_ms: 12000,
      hp_left_pct: 40,
      loot: { coin: 100 },
      created_at: '2026-01-01T00:00:00Z',
      ...overrides,
    }
  }

  async function openChallenges(wrapper: Awaited<ReturnType<typeof mountDefenses>>) {
    const vm = wrapper.vm as unknown as { openChallenges: () => void }
    vm.openChallenges()
    await flushPromises()
    return wrapper
  }

  it('点击挑战者昵称：跳转 /users 并携带 challenger_id', async () => {
    fetchAdminDefenseChallenges.mockResolvedValue({ items: [challengeFixture()], total: 1 })
    const wrapper = await mountDefenses([defenseFixture()])
    await openChallenges(wrapper)

    const link = wrapper.find('.el-dialog .el-link')
    expect(link.exists(), '找不到挑战者列链接').toBe(true)
    expect(link.text()).toBe('挑战者甲')

    await link.trigger('click')
    await flushPromises()

    expect(lastRouter?.currentRoute.value.path).toBe('/users')
    expect(lastRouter?.currentRoute.value.query.user_id).toBe('7')
    wrapper.unmount()
  })

  it('无昵称的挑战者显示「—」，不渲染可点击链接', async () => {
    fetchAdminDefenseChallenges.mockResolvedValue({
      items: [challengeFixture({ challenger_nickname: '' })],
      total: 1,
    })
    const wrapper = await mountDefenses([defenseFixture()])
    await openChallenges(wrapper)

    expect(wrapper.find('.el-dialog .el-link').exists()).toBe(false)
    expect(wrapper.text()).toContain('—')
    wrapper.unmount()
  })
})
