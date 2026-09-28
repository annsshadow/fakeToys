/**
 * BattlesView.vue 测试。
 *
 * 战报页的守恒检查（kills+leaked）、反应滥用标记（reactions/kills>30）、
 * 以及详情抽屉里 JSON 字符串的容错解析（parseObj 四分支）是数据可信度的关键。
 */
import { flushPromises, mount } from '@vue/test-utils'
import { ElDrawer, ElInputNumber } from 'element-plus'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { AdminBattle } from '@/api'

const fetchBattles = vi.hoisted(() => vi.fn())
const fetchBattleDetail = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchBattles, fetchBattleDetail }))

import BattlesView from '@/views/BattlesView.vue'

function battleFixture(overrides: Partial<AdminBattle> = {}): AdminBattle {
  return {
    id: 1,
    user_id: 9,
    level_id: 3,
    result: 'win',
    stars: 3,
    score: 88000,
    kills: 10,
    leaked: 2,
    wave_reached: 5,
    duration_ms: 12345,
    reactions: 5,
    heat_max: 90,
    replay_hash: 'abcd1234abcd1234abcd1234abcd1234',
    created_at: '2026-01-02T03:04:05Z',
    ...overrides,
  }
}

async function mountBattles(items: AdminBattle[] = [], total = items.length) {
  fetchBattles.mockResolvedValue({ items, total })
  const wrapper = mount(BattlesView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  vi.clearAllMocks()
})

describe('BattlesView 列表渲染', () => {
  it('胜负/星级/守恒/时长/时间如实渲染；零击杀零漏怪标记「无接触」', async () => {
    const wrapper = await mountBattles([
      battleFixture({ id: 1 }),
      battleFixture({ id: 2, result: 'lose', stars: 0, kills: 0, leaked: 0 }),
    ])
    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].text()).toContain('胜')
    expect(rows[0].text()).toContain('★★★')
    expect(rows[0].text()).toContain('守恒')
    expect(rows[0].text()).toContain('12.3s')
    expect(rows[0].text()).toContain('2026-01-02 03:04:05')
    expect(rows[1].text()).toContain('负')
    expect(rows[1].text()).not.toContain('★')
    expect(rows[1].text()).toContain('无接触')
    wrapper.unmount()
  })

  it('反应滥用标记：reactions/kills > 30 才标红；任一前置为 0 不标记', async () => {
    const wrapper = await mountBattles([
      battleFixture({ id: 1, kills: 1, reactions: 40 }), // 40 > 30 → 滥用
      battleFixture({ id: 2, kills: 1, reactions: 30 }), // 边界：30 不大于 30
      battleFixture({ id: 3, kills: 0, reactions: 40 }), // kills=0 → 不标记
      battleFixture({ id: 4, kills: 5, reactions: 0 }), // reactions=0 → 不标记
    ])
    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].find('.danger').exists()).toBe(true)
    expect(rows[1].find('.danger').exists()).toBe(false)
    expect(rows[2].find('.danger').exists()).toBe(false)
    expect(rows[3].find('.danger').exists()).toBe(false)
    wrapper.unmount()
  })

  it('筛选按钮：默认参数与设置过滤后的参数（输入框 v-model 双向生效）', async () => {
    const wrapper = await mountBattles()
    await wrapper.findAll('button').find((b) => b.text() === '筛选')!.trigger('click')
    await flushPromises()
    expect(fetchBattles).toHaveBeenLastCalledWith({ user_id: undefined, level_id: undefined, limit: 50 })

    // 通过组件事件驱动两个 el-input-number 的 v-model（与真实输入等价）
    const nums = wrapper.findAllComponents(ElInputNumber)
    expect(nums).toHaveLength(2)
    await nums[0].vm.$emit('update:modelValue', 5)
    await nums[1].vm.$emit('update:modelValue', 7)
    await flushPromises()
    await wrapper.findAll('button').find((b) => b.text() === '筛选')!.trigger('click')
    await flushPromises()
    expect(fetchBattles).toHaveBeenLastCalledWith({ user_id: 5, level_id: 7, limit: 50 })
    wrapper.unmount()
  })

  it('加载失败：弹出错误消息', async () => {
    fetchBattles.mockRejectedValueOnce(new Error('战报库离线'))
    const wrapper = mount(BattlesView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    expect(document.body.textContent).toContain('战报加载失败：战报库离线')
    wrapper.unmount()
  })
})

describe('BattlesView 详情抽屉', () => {
  const detailBase = { ...battleFixture({ id: 88 }), shots: 10, hits: 8 } as AdminBattle & Record<string, unknown>

  it('详情正常打开：回放哈希、发射/命中、元素与反应分布中文标签渲染；关闭抽屉走 v-model', async () => {
    fetchBattleDetail.mockResolvedValueOnce({
      battle: {
        ...detailBase,
        elements_used: '{"fire":3,"kinetic":1,"void":4}', // void 不在映射表 → 回退原始 key
        reactions_used: '{"steam_burst":2,"mega_boom":1}', // mega_boom 同上
        terrain_used: 'oil_drum',
      },
    })
    const wrapper = await mountBattles([battleFixture()])
    await wrapper.findAll('button').find((b) => b.text() === '详情')!.trigger('click')
    await flushPromises()

    expect(fetchBattleDetail).toHaveBeenCalledWith(1)
    const text = wrapper.text()
    expect(text).toContain('战报详情')
    expect(text).toContain('battle_id')
    expect(text).toContain('10 / 8')
    expect(text).toContain('abcd1234abcd1234abcd1234abcd1234')
    expect(text).toContain('焰 × 3')
    expect(text).toContain('动能 × 1')
    expect(text).toContain('void × 4')
    expect(text).toContain('蒸汽爆发 × 2')
    expect(text).toContain('mega_boom × 1')

    // 用户关闭抽屉：v-model 的 onUpdate 处理器真实执行
    const drawer = wrapper.findComponent(ElDrawer)
    await drawer.vm.$emit('update:modelValue', false)
    await flushPromises()
    const vm = wrapper.vm as unknown as { detailDrawer: boolean }
    expect(vm.detailDrawer).toBe(false)
    wrapper.unmount()
  })

  it('服务端修正过的战报：clamped 标记展示，clamp_note 缺省时显示默认文案', async () => {
    fetchBattleDetail.mockResolvedValueOnce({
      battle: { ...detailBase, clamped: true, clamp_note: '' },
    })
    const wrapper = await mountBattles([battleFixture()])
    await wrapper.findAll('button').find((b) => b.text() === '详情')!.trigger('click')
    await flushPromises()
    expect(wrapper.text()).toContain('上报数据越界，已按上限截断')
    wrapper.unmount()
  })

  it('未修正的战报不显示修正标签（v-if 假分支）', async () => {
    fetchBattleDetail.mockResolvedValueOnce({ battle: { ...detailBase } })
    const wrapper = await mountBattles([battleFixture()])
    await wrapper.findAll('button').find((b) => b.text() === '详情')!.trigger('click')
    await flushPromises()
    expect(wrapper.text()).not.toContain('服务端修正')
    wrapper.unmount()
  })

  it('元素/反应 JSON 的容错：非法 JSON 与空值都回退为空分布提示', async () => {
    fetchBattleDetail.mockResolvedValueOnce({
      battle: {
        ...detailBase,
        elements_used: 'not-json{',
        reactions_used: 'also-bad{',
        terrain_used: 0,
      },
    })
    const wrapper = await mountBattles([battleFixture()])
    await wrapper.findAll('button').find((b) => b.text() === '详情')!.trigger('click')
    await flushPromises()

    const text = wrapper.text()
    // 解析失败 → 空分布 → 引导文案
    expect(text).toContain('无记录 —— 这一局完全没触发反应')
    // terrain_used=0：数值走 !parseObj 的真分支，显示占位
    expect(text).toContain('—')
    wrapper.unmount()
  })

  it('详情接口失败：弹错误且抽屉不打开', async () => {
    fetchBattleDetail.mockRejectedValueOnce(new Error('战报不存在'))
    const wrapper = await mountBattles([battleFixture()])
    await wrapper.findAll('button').find((b) => b.text() === '详情')!.trigger('click')
    await flushPromises()
    expect(document.body.textContent).toContain('战报不存在')
    // 接口失败时抽屉保持关闭
    const vm = wrapper.vm as unknown as { detailDrawer: boolean }
    expect(vm.detailDrawer).toBe(false)
    wrapper.unmount()
  })

  it('parseObj 直接验证四个分支：对象透传 / null 回退 / 合法 JSON / 非法 JSON', async () => {
    const wrapper = await mountBattles()
    const vm = wrapper.vm as unknown as { parseObj: (v: unknown) => Record<string, number> }
    expect(vm.parseObj({ fire: 2 })).toEqual({ fire: 2 })
    expect(vm.parseObj(null)).toEqual({})
    expect(vm.parseObj('{"a":1}')).toEqual({ a: 1 })
    expect(vm.parseObj('bad{')).toEqual({})
    wrapper.unmount()
  })
})
