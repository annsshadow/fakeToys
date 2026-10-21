/**
 * EquipmentView.vue 测试。
 *
 * ───────────────────────────────────────────────────────────────────────────
 * 2026-10-21：fixture 换成了**服务端真实形状**，断言也换成了**真实行为**。
 *
 * 此前这个页面有两处「看起来有、其实没有」：
 *   1. 宝石的「效果」与「说明」两列**同绑 `descr`** —— 宝石真正的效果字段
 *      是 `attr`（domain.SeedGem: id/code/name/attr/descr），所以「效果」列
 *      显示的其实是说明文字。
 *   2. 皮肤的「价格」列绑了 `price` —— domain.SeedSkin **没有 price 字段**
 *      （id/code/name/rarity/passive/descr），所以那一列永远空。
 *
 * 而旧 fixture 连 `price` 都编了出来，于是测试是绿的、线上是空的。
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

/** 与 domain.SeedEquipment 一致。 */
function equipmentFixture(overrides: Record<string, unknown> = {}) {
  return {
    id: 1,
    code: 'wpn_ember',
    name: '焰纹炮',
    slot: 'weapon',
    tier: 2,
    element: 'fire',
    descr: '同系加成',
    base_armor: 30,
    base_bonus_pct: 120,
    unlock_level: 5,
    ...overrides,
  }
}

/** 与 domain.SeedGem 一致：效果字段是 `attr`，不是 `descr`。 */
function gemFixture(overrides: Record<string, unknown> = {}) {
  return {
    id: 10,
    code: 'gem_atk',
    name: '攻击石',
    attr: 'attack',
    descr: '+5% 攻击',
    ...overrides,
  }
}

/** 与 domain.SeedSkin 一致：**没有 price 字段**。 */
function skinFixture(overrides: Record<string, unknown> = {}) {
  return {
    id: 20,
    code: 'skin_ember',
    name: '焰纹·红',
    rarity: 'rare',
    passive: 'fire_aoe',
    descr: '限定皮肤',
    ...overrides,
  }
}

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
        equipmentFixture(),
        equipmentFixture({ id: 2, name: '虚空甲', element: 'void', slot: 'hat', tier: 1, descr: '未知系' }),
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

  /**
   * 2026-10-21：装备的 `base_armor` / `base_bonus_pct` / `unlock_level`
   * 服务端一直返回、界面一直不显示 —— 运营看不到「这件装备到底加多少」。
   */
  it('装备表展示护甲 / 攻击加成 / 解锁等级（P2-3 顺带补齐）', async () => {
    const wrapper = await mountEquipment({ equipment: [equipmentFixture()] })
    const text = wrapper.text()
    expect(text).toContain('30') // base_armor
    expect(text).toContain('120') // base_bonus_pct
    expect(text).toContain('5') // unlock_level
    wrapper.unmount()
  })

  /**
   * 2026-10-21（P2-3）：宝石的「效果」列此前绑的是 `descr`，
   * 于是它显示的是说明文字 —— 两列一模一样。
   * 现在「效果」列读 `attr`。
   */
  it('宝石的「效果」列读 attr，与「说明」列不是同一列（P2-3）', async () => {
    const wrapper = await mountEquipment({ gems: [gemFixture()] })
    const text = wrapper.text()
    expect(text).toContain('attack') // attr（效果）
    expect(text).toContain('+5% 攻击') // descr（说明）
    wrapper.unmount()
  })

  /**
   * 2026-10-21（P2-4）：皮肤的「价格」列绑了不存在的 `price` 字段 → 恒空。
   * 皮肤真实的可用字段是 `rarity` / `passive`。
   */
  it('皮肤表展示稀有度与被动，不再有恒空的价格列（P2-4）', async () => {
    const wrapper = await mountEquipment({ skins: [skinFixture()] })
    const text = wrapper.text()
    expect(text).toContain('焰纹·红')
    expect(text).toContain('rare')
    expect(text).toContain('fire_aoe')
    // 旧列还在的话这里会是空字符串
    expect(wrapper.findAll('.el-table__row')[0].text()).not.toBe('')
    wrapper.unmount()
  })

  it('宝石与皮肤两个 tab 的数据也在挂载时加载完成（切换即显）', async () => {
    const wrapper = await mountEquipment({
      gems: [gemFixture()],
      skins: [skinFixture()],
    })
    const text = wrapper.text()
    expect(text).toContain('攻击石')
    expect(text).toContain('焰纹·红')
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
    expect(document.body.textContent).toContain('内容服务不可用')
    wrapper.unmount()
  })
})
