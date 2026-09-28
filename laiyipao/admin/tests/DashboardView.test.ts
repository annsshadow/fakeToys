/**
 * DashboardView.vue 测试。
 *
 * 看板的三类真实风险都在这里：
 * 1. KPI 派生逻辑（验真一致率、胜率的分母保护）算错会误导运营决策；
 * 2. echarts 初始化的三处 ref 判空与 ??= 复用逻辑（内存泄漏/重复初始化）；
 * 3. 加载竞态：请求未返回时用户已离开页面 → refs 已置空，renderCharts 不能炸。
 * echarts 以替身注入（jsdom 无 canvas），但组件逻辑原样执行。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { DashboardResp } from '@/api'

const fetchDashboard = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchDashboard }))

const chartStub = { setOption: vi.fn(), dispose: vi.fn() }
const echartsInit = vi.hoisted(() => vi.fn())
vi.mock('echarts/core', () => ({ use: vi.fn(), init: echartsInit }))

import DashboardView from '@/views/DashboardView.vue'

function dashboardFixture(overrides: Partial<DashboardResp> = {}): DashboardResp {
  return {
    users: { total: 1200, guests: 300, wechat: 900, banned: 5, new_today: 12 },
    battles: { total: 0, today: 3, wins: 0, avg_duration_ms: 9000 },
    progression: { avg_max_stage: 42.5, avg_power: 8800, stage_100_clears: 2 },
    economy: { coin_in: 100, coin_out: 40, gem_in: 7, gem_out: 3, orders_paid: 9, orders_pending: 1 },
    verification: { checked: 1000, matched: 990, mismatched: 10 },
    reaction_usage: [
      { reaction: 'steam_burst', count: 30 },
      { reaction: 'overheat', count: 10 },
      { reaction: 'custom_rx', count: 20 },
    ],
    daily_active: [
      { date: '2026-01-01', dau: 100 },
      { date: '2026-01-02', dau: 140 },
    ],
    stage_funnel: [{ level_id: 1, attempts: 500, clears: 300 }],
    ...overrides,
  }
}

async function mountDashboard() {
  const wrapper = mount(DashboardView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  await new Promise((r) => setTimeout(r, 60)) // load() 内部有 30ms 图表渲染延迟
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  vi.clearAllMocks()
  echartsInit.mockReturnValue(chartStub)
  fetchDashboard.mockResolvedValue(dashboardFixture())
})

describe('DashboardView KPI 渲染', () => {
  it('加载成功：KPI 数值与副标题如实渲染', async () => {
    const wrapper = await mountDashboard()
    expect(wrapper.text()).toContain('数据看板')
    expect(wrapper.text()).toContain('1,200')
    expect(wrapper.text()).toContain('游客 300 · 微信 900 · 今日新增 12')
    expect(wrapper.text()).toContain('待支付 1')
    expect(wrapper.text()).toContain('100 关通关人次 2')
  })

  it('战斗总数为 0 时胜率分母被保护为 1，而不是 NaN', async () => {
    // fixture 中 battles.total = 0
    const wrapper = await mountDashboard()
    expect(wrapper.text()).toContain('0.0%')
    expect(wrapper.text()).not.toContain('NaN')
  })

  it('验真一致率：mismatched > 0 → danger 红色标签', async () => {
    const wrapper = await mountDashboard()
    expect(wrapper.text()).toContain('99.0%')
    expect(wrapper.find('.kpi-value .el-tag--danger').exists()).toBe(true)
  })

  it('验真一致率：mismatched = 0 → success 标签', async () => {
    fetchDashboard.mockResolvedValue(
      dashboardFixture({ verification: { checked: 500, matched: 500, mismatched: 0 } }),
    )
    const wrapper = await mountDashboard()
    expect(wrapper.text()).toContain('100.0%')
    expect(wrapper.find('.kpi-value .el-tag--success').exists()).toBe(true)
  })

  it('验真一致率：checked = 0 → 显示「—」且标签为 info', async () => {
    fetchDashboard.mockResolvedValue(
      dashboardFixture({ verification: { checked: 0, matched: 0, mismatched: 0 } }),
    )
    const wrapper = await mountDashboard()
    expect(wrapper.text()).toContain('—')
    expect(wrapper.find('.kpi-value .el-tag--info').exists()).toBe(true)
  })

  it('反应使用分布为空时展示引导告警；有数据时不展示', async () => {
    const withData = await mountDashboard()
    expect(withData.text()).not.toContain('尚无反应触发记录')
    withData.unmount()

    fetchDashboard.mockResolvedValue(dashboardFixture({ reaction_usage: [] }))
    const empty = await mountDashboard()
    expect(empty.text()).toContain('尚无反应触发记录')
  })
})

describe('DashboardView 图表渲染', () => {
  it('三张图各初始化一次并 setOption；反应图按次数升序、未知反应回退原始 key', async () => {
    const wrapper = await mountDashboard()
    expect(echartsInit).toHaveBeenCalledTimes(3)
    expect(chartStub.setOption).toHaveBeenCalledTimes(3)

    const opts = chartStub.setOption.mock.calls.map((c) => c[0])
    // DAU 折线图
    expect(opts[0].xAxis.data).toEqual(['2026-01-01', '2026-01-02'])
    expect(opts[0].series[0].data).toEqual([100, 140])
    // 漏斗柱状图
    expect(opts[1].xAxis.data).toEqual(['第1关'])
    expect(opts[1].series.map((s: { data: number[] }) => s.data)).toEqual([[500], [300]])
    // 反应图：按 count 升序 → overheat(10), custom_rx(20), steam_burst(30)；
    // custom_rx 不在中文映射表 → 回退原始 key
    expect(opts[2].yAxis.data).toEqual(['过热', 'custom_rx', '蒸汽爆发'])
    expect(opts[2].series[0].data).toEqual([10, 20, 30])
    wrapper.unmount()
  })

  it('刷新数据时复用已有图表实例（??= 不重复 init）', async () => {
    const wrapper = await mountDashboard()
    expect(echartsInit).toHaveBeenCalledTimes(3)

    fetchDashboard.mockResolvedValue(dashboardFixture({ users: { ...dashboardFixture().users, total: 2000 } }))
    await (wrapper.vm as unknown as { load: () => Promise<void> }).load()
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()

    expect(echartsInit).toHaveBeenCalledTimes(3) // 没有第二次 init
    expect(chartStub.setOption).toHaveBeenCalledTimes(6) // 但重新 setOption
    expect(wrapper.text()).toContain('2,000')
    wrapper.unmount()
  })

  it('接口返回 null 时 renderCharts 的空数据护栏直接返回，不初始化图表', async () => {
    fetchDashboard.mockResolvedValueOnce(null)
    const wrapper = await mountDashboard()
    expect(echartsInit).not.toHaveBeenCalled()
    expect(wrapper.find('.kpi-value').exists()).toBe(false)
    wrapper.unmount()
  })

  it('加载期间离开页面：refs 已置空，renderCharts 判空跳过、不炸不泄漏', async () => {
    let resolveFetch!: (v: DashboardResp) => void
    fetchDashboard.mockReturnValueOnce(new Promise<DashboardResp>((r) => (resolveFetch = r)))

    const wrapper = mount(DashboardView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    wrapper.unmount() // 请求还挂着就离开页面

    resolveFetch(dashboardFixture())
    await flushPromises()
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()

    expect(echartsInit).not.toHaveBeenCalled()
  })
})

describe('DashboardView 加载失败', () => {
  it('请求失败：展示错误告警与后端提示，KPI 区域不渲染', async () => {
    fetchDashboard.mockRejectedValueOnce(new Error('connect ECONNREFUSED'))
    const wrapper = await mountDashboard()
    expect(document.body.textContent).toContain('看板数据加载失败：connect ECONNREFUSED')
    expect(wrapper.text()).toContain('后端不可达：connect ECONNREFUSED')
    expect(wrapper.text()).toContain('请先启动 server')
    expect(wrapper.find('.kpi-value').exists()).toBe(false)
    expect(echartsInit).not.toHaveBeenCalled()
  })
})
