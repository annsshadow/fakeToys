/**
 * SkillsView.vue 测试。
 *
 * 技能页的核心是「热量效率」派生指标：它决定运营如何发现数值失衡的技能。
 * dpsPerHeat 的除零保护（heat_cost<=0、cooldown=0）与 heatTone 四档色阶、
 * 前端过滤组合、逐字段编辑弹窗三态都要锁住。
 */
import { flushPromises, mount } from '@vue/test-utils'
import { ElSelect } from 'element-plus'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import type { AdminSkill } from '@/api'

const fetchSkills = vi.hoisted(() => vi.fn())
const updateSkill = vi.hoisted(() => vi.fn())
const box = vi.hoisted(() => ({ prompt: vi.fn(), confirm: vi.fn() }))

vi.mock('@/api', () => ({ fetchSkills, updateSkill }))
vi.mock('element-plus', async (importOriginal) => {
  const mod = await importOriginal<typeof import('element-plus')>()
  return { ...mod, ElMessageBox: box }
})

import SkillsView from '@/views/SkillsView.vue'

function skillFixture(overrides: Partial<AdminSkill> = {}): AdminSkill {
  return {
    id: 1,
    code: 'fire_bolt',
    name: '火弹',
    family: 'flame',
    element: 'fire',
    kind: 'active',
    base_damage: 100,
    heat_cost: 10,
    cooldown_ms: 500,
    pierce: 0,
    aoe_radius: 0,
    apply_element: 'fire',
    apply_stacks: 2,
    unlock_level: 1,
    descr: '发射火弹',
    ...overrides,
  }
}

async function mountSkills(items: AdminSkill[] = [], recipes: Array<Record<string, number>> = []) {
  fetchSkills.mockResolvedValue({ items, recipes })
  const wrapper = mount(SkillsView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return wrapper
}

type Vm = {
  dpsPerHeat: (s: AdminSkill) => number
  heatTone: (s: AdminSkill) => string
  recipeText: (r: Record<string, number>) => string
  activeElement: string
  activeKind: string
  skills: AdminSkill[]
  patch: (row: AdminSkill, field: keyof AdminSkill, label: string) => Promise<void>
  onEditDescr: (row: AdminSkill) => Promise<void>
}

beforeEach(() => {
  vi.clearAllMocks()
})

describe('SkillsView 热量效率派生', () => {
  it('dpsPerHeat：常规技能 = 每秒射数 × 伤害 ÷ 热量', async () => {
    const wrapper = await mountSkills()
    const vm = wrapper.vm as unknown as Vm
    // (100 * (1000/500)) / 10 = 20
    expect(vm.dpsPerHeat(skillFixture())).toBe(20)
    wrapper.unmount()
  })

  it('dpsPerHeat：零热量技能直接返回伤害（除零保护）', async () => {
    const wrapper = await mountSkills()
    const vm = wrapper.vm as unknown as Vm
    expect(vm.dpsPerHeat(skillFixture({ heat_cost: 0, base_damage: 42 }))).toBe(42)
    expect(vm.dpsPerHeat(skillFixture({ heat_cost: -5, base_damage: 42 }))).toBe(42)
    wrapper.unmount()
  })

  it('dpsPerHeat：零冷却按 1ms 计算（Math.max 保护）', async () => {
    const wrapper = await mountSkills()
    const vm = wrapper.vm as unknown as Vm
    // (10 * (1000/1)) / 10 = 1000
    expect(vm.dpsPerHeat(skillFixture({ cooldown_ms: 0, base_damage: 10 }))).toBe(1000)
    wrapper.unmount()
  })

  it('heatTone 四档色阶按效率分界（8/4/2）', async () => {
    const wrapper = await mountSkills()
    const vm = wrapper.vm as unknown as Vm
    const toneOf = (damage: number) => vm.heatTone(skillFixture({ base_damage: damage, heat_cost: 10, cooldown_ms: 500 }))
    // (40*2)/10=8 → success；(30*2)/10=6 → info；(15*2)/10=3 → warning；(5*2)/10=1 → danger
    expect(toneOf(40)).toBe('success')
    expect(toneOf(30)).toBe('info')
    expect(toneOf(15)).toBe('warning')
    expect(toneOf(5)).toBe('danger')
    wrapper.unmount()
  })

  it('recipeText 输出与 Go 侧 SeedRecipe json tag 对齐的 a/b/output/out_tier', async () => {
    const wrapper = await mountSkills([], [{ a: 1, b: 2, output: 9, out_tier: 3 }])
    const vm = wrapper.vm as unknown as Vm
    expect(vm.recipeText({ a: 1, b: 2, output: 9, out_tier: 3 })).toBe('#1 + #2 → #9（T3）')
    await flushPromises()
    expect(wrapper.text()).toContain('#1 + #2 → #9（T3）')
    wrapper.unmount()
  })
})

describe('SkillsView 列表与过滤', () => {
  it('系别与元素未知值回退原始 key；已知值用中文映射', async () => {
    const wrapper = await mountSkills([
      skillFixture({ id: 1, family: 'flame', element: 'fire' }),
      skillFixture({ id: 2, family: 'alien', element: 'void', name: '虚空' }),
    ])
    const text = wrapper.text()
    expect(text).toContain('焰系')
    expect(text).toContain('焰')
    expect(text).toContain('alien')
    expect(text).toContain('void')
    wrapper.unmount()
  })

  it('元素 + 类型双过滤：&& 短路逻辑的四种组合', async () => {
    const wrapper = await mountSkills([
      skillFixture({ id: 1, element: 'fire', kind: 'active' }),
      skillFixture({ id: 2, element: 'fire', kind: 'passive' }),
      skillFixture({ id: 3, element: 'ice', kind: 'active', name: '冰锥' }),
    ])
    const vm = wrapper.vm as unknown as Vm

    // 都不过滤 → 3
    vm.activeElement = ''
    vm.activeKind = ''
    await flushPromises()
    expect(wrapper.findAll('.el-table__row')).toHaveLength(3)

    // 只按元素 → 2
    vm.activeElement = 'fire'
    await flushPromises()
    expect(wrapper.findAll('.el-table__row')).toHaveLength(2)

    // 元素 + 类型 → 1
    vm.activeKind = 'passive'
    await flushPromises()
    expect(wrapper.findAll('.el-table__row')).toHaveLength(1)

    // 只按类型 → 1
    vm.activeElement = ''
    await flushPromises()
    expect(wrapper.findAll('.el-table__row')).toHaveLength(1)
    wrapper.unmount()
  })

  it('工具栏真实交互：点击「焰」单选与类型下拉联动过滤（v-model 处理器真实执行）', async () => {
    const wrapper = await mountSkills([
      skillFixture({ id: 1, element: 'fire', kind: 'active' }),
      skillFixture({ id: 2, element: 'fire', kind: 'passive', name: '火甲' }),
      skillFixture({ id: 3, element: 'ice', kind: 'active', name: '冰锥' }),
    ])

    // 点击元素单选按钮「焰」（radio 组件靠内部 input 的 change 驱动 v-model）
    const fireLabel = wrapper.findAll('.el-radio-button').find((b) => b.text() === '焰')!
    await fireLabel.find('input').setValue(true)
    await flushPromises()
    expect(wrapper.findAll('.el-table__row')).toHaveLength(2)

    // 类型下拉选中「主动」
    const select = wrapper.findComponent(ElSelect)
    await select.vm.$emit('update:modelValue', 'active')
    await flushPromises()
    const rows = wrapper.findAll('.el-table__row')
    expect(rows).toHaveLength(1)
    expect(rows[0].text()).toContain('火弹')
    wrapper.unmount()
  })

  it('操作列三个按钮（伤害/热量/描述）逐个点击走真实编辑流程', async () => {
    const row = skillFixture({ id: 7, base_damage: 100, heat_cost: 10 })
    fetchSkills.mockResolvedValue({ items: [row], recipes: [] })
    updateSkill
      .mockResolvedValueOnce({ skill: { ...row, base_damage: 250 } })
      .mockResolvedValueOnce({ skill: { ...row, heat_cost: 20 } })
      .mockResolvedValueOnce({ skill: { ...row, descr: '重塑后的描述' } })
    box.prompt
      .mockResolvedValueOnce({ value: '250' })
      .mockResolvedValueOnce({ value: '20' })
      .mockResolvedValueOnce({ value: '重塑后的描述' })

    const wrapper = mount(SkillsView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    const buttons = wrapper.findAll('.el-table__row')[0].findAll('button')
    expect(buttons.map((b) => b.text())).toEqual(['伤害', '热量', '描述'])

    await buttons[0].trigger('click')
    await flushPromises()
    expect(updateSkill).toHaveBeenLastCalledWith(7, { base_damage: 250 })

    await buttons[1].trigger('click')
    await flushPromises()
    expect(updateSkill).toHaveBeenLastCalledWith(7, { heat_cost: 20 })

    await buttons[2].trigger('click')
    await flushPromises()
    expect(updateSkill).toHaveBeenLastCalledWith(7, { descr: '重塑后的描述' })
    wrapper.unmount()
  })

  it('加载失败：弹出错误消息', async () => {
    fetchSkills.mockRejectedValueOnce(new Error('技能表损坏'))
    const wrapper = mount(SkillsView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    expect(document.body.textContent).toContain('技能加载失败：技能表损坏')
    wrapper.unmount()
  })
})

describe('SkillsView 数值编辑', () => {
  it('patch 确认：按字段名提交 Number 值并合并返回', async () => {
    box.prompt.mockResolvedValueOnce({ value: '250' })
    const row = skillFixture()
    updateSkill.mockResolvedValueOnce({ skill: { ...row, base_damage: 250 } })
    const wrapper = await mountSkills([row])
    const vm = wrapper.vm as unknown as Vm
    await vm.patch(row, 'base_damage', '基础伤害')
    await flushPromises()

    expect(updateSkill).toHaveBeenCalledWith(1, { base_damage: 250 })
    expect(row.base_damage).toBe(250)
    expect(document.body.textContent).toContain('已保存')
    wrapper.unmount()
  })

  it('patch 取消：不提交', async () => {
    box.prompt.mockRejectedValueOnce('cancel')
    const wrapper = await mountSkills([skillFixture()])
    const vm = wrapper.vm as unknown as Vm
    await vm.patch(skillFixture(), 'heat_cost', '热量消耗')
    await flushPromises()
    expect(updateSkill).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('描述编辑成功：提交文本值并合并返回', async () => {
    const row = skillFixture()
    box.prompt.mockResolvedValueOnce({ value: '新描述' })
    updateSkill.mockResolvedValueOnce({ skill: { ...row, descr: '新描述' } })
    const wrapper = await mountSkills([row])
    const vm = wrapper.vm as unknown as Vm
    await vm.onEditDescr(row)
    await flushPromises()
    expect(updateSkill).toHaveBeenCalledWith(1, { descr: '新描述' })
    expect(row.descr).toBe('新描述')
    wrapper.unmount()
  })

  it('描述编辑取消：不提交', async () => {
    box.prompt.mockRejectedValueOnce('cancel')
    const wrapper = await mountSkills([skillFixture()])
    const vm = wrapper.vm as unknown as Vm
    await vm.onEditDescr(skillFixture())
    await flushPromises()
    expect(updateSkill).not.toHaveBeenCalled()
    wrapper.unmount()
  })
})
