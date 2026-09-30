/**
 * EquipmentView.vue 测试。
 *
 * 三个目录表（装备/宝石/皮肤）字段随版本增长，视图层用 str() 容错取值：
 * null/undefined 的名称要渲染成空串而不是 "null"，未知元素/部位回退原文。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const fetchEquipment = vi.hoisted(() => vi.fn())
vi.mock('@/api', () => ({ fetchEquipment }))

import EquipmentView from '@/views/EquipmentView.vue'

beforeEach(() => {
  vi.clearAllMocks()
})

async function mountEquipment(data?: {
  equipment?: Array<Record<string, unknown>>
  gems?: Array<Record<string, unknown>>
  skins?: Array<Record<string, unknown>>
}) {
  fetchEquipment.mockResolvedValue({
    equipment: data?.equipment ?? [],
    gems: data?.gems ?? [],
    skins: data?.skins ?? [],
  })
  const wrapper = mount(EquipmentView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return wrapper
}

describe('EquipmentView 列表渲染', () => {
  it('装备表：已知元素/部位用中文映射与配色，未知值回退原文', async () => {
    const wrapper = await mountEquipment({
      equipment: [
        { id: 1, name: '焰纹炮', element: 'fire', slot: 'weapon', tier: 2, descr: '同系加成' },
        { id: 2, name: '虚空甲', element: 'void', slot: 'hat', tier: 1, descr: '未知系' },
      ],
    })
    const text = wrapper.text()
    expect(text).toContain('焰纹炮')
    expect(text).toContain('焰')
    expect(text).toContain('武器')
    // 未知元素/部位：显示原始值而不是 undefined
    expect(text).toContain('void')
    expect(text).toContain('hat')
    wrapper.unmount()
  })

  it('str() 对 null / undefined / 数值的三种容错', async () => {
    const wrapper = await mountEquipment({
      equipment: [
        { id: 3, name: null, element: 'ice', slot: 'armor', tier: 1, descr: 'x' },
        { id: 4, tier: 3 },
      ],
    })
    const vm = wrapper.vm as unknown as {
      str: (o: Record<string, unknown>, k: string) => string
      equipment: Array<Record<string, unknown>>
    }
    expect(vm.str(vm.equipment[0], 'name')).toBe('') // null → ''
    expect(vm.str(vm.equipment[1], 'name')).toBe('') // undefined → ''
    expect(vm.str(vm.equipment[1], 'tier')).toBe('3') // 数值 → 字符串
    // 渲染层面：null 名称显示为空（不是 "null"）
    expect(wrapper.text()).not.toContain('null')
    wrapper.unmount()
  })

  it('宝石与皮肤两个 tab 的数据也在挂载时加载完成（切换即显）', async () => {
    const wrapper = await mountEquipment({
      gems: [{ id: 10, name: '攻击石', descr: '+5% 攻击' }],
      skins: [{ id: 20, name: '焰纹·红', descr: '限定皮肤', price: 300 }],
    })
    const text = wrapper.text()
    expect(text).toContain('攻击石')
    expect(text).toContain('焰纹·红')
    expect(text).toContain('300')
    wrapper.unmount()
  })

  it('tab 切换：点击「宝石」页签后 activeTab 更新', async () => {
    const wrapper = await mountEquipment()
    const tab = wrapper.findAll('.el-tabs__item').find((t) => t.text() === '宝石')
    await tab!.trigger('click')
    await flushPromises()
    const vm = wrapper.vm as unknown as { activeTab: string }
    expect(vm.activeTab).toBe('gems')
    wrapper.unmount()
  })

  it('加载失败：弹出错误消息', async () => {
    fetchEquipment.mockRejectedValueOnce(new Error('内容服务不可用'))
    const wrapper = mount(EquipmentView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    expect(document.body.textContent).toContain('内容加载失败：内容服务不可用')
    wrapper.unmount()
  })
})
