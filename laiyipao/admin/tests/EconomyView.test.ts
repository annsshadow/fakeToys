/**
 * EconomyView.vue 测试。
 *
 * 经济页核心是通胀监控（净流入聚合）与商城项编辑的价格/文本分支。
 * netFlow 的累加（首现 ?? 0 与续加）、flowTone 三档、
 * patchShop 对 price/limit/stock 转 Number 而 name 保留字符串的分支逐一锁定。
 */
import { flushPromises, mount } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const fetchEconomy = vi.hoisted(() => vi.fn())
const updateShopItem = vi.hoisted(() => vi.fn())
const box = vi.hoisted(() => ({ prompt: vi.fn(), confirm: vi.fn() }))

vi.mock('@/api', () => ({ fetchEconomy, updateShopItem }))
vi.mock('element-plus', async (importOriginal) => {
  const mod = await importOriginal<typeof import('element-plus')>()
  return { ...mod, ElMessageBox: box }
})

import EconomyView from '@/views/EconomyView.vue'

type Row = Record<string, unknown>

async function mountEconomy(flows: Row[] = [], shop: Row[] = []) {
  fetchEconomy.mockResolvedValue({ flows, shop })
  const wrapper = mount(EconomyView, { global: { plugins: [ElementPlus] } })
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  vi.clearAllMocks()
})

describe('EconomyView 通胀监控', () => {
  it('净流入聚合：同币种累加、四种货币卡齐全、数字字符串参与计算、缺字段兜底 0', async () => {
    // 注意：netFlow 用裸 Number() 累加，无 NaN 保护（amount 为非数字时整列变 NaN）；
    // 数字字符串 '150' 能正常解析。NaN 场景是已知鲁班口，不在本断言内。
    const wrapper = await mountEconomy(
      [
        { currency: 'coin', amount: '150', cnt: 3, reason: 'battle_loot', updated_at: '2026-01-01T00:00:00Z' },
        { currency: 'coin', amount: -200, cnt: 0, reason: 'mystery_reason' },
        { currency: 'gem', amount: 500, cnt: 2, reason: 'shop' },
        { currency: 'energy', amount: 0, cnt: 0, reason: 'signin' },
        { cnt: 1 }, // currency/amount 缺失 → ?? 兜底空币种 0
      ],
      [],
    )
    const text = wrapper.text()
    // coin: 150 - 200 + 0(非法) = -50；gem: 500 → +500；energy: 0；keys: 无数据 → 0
    expect(text).toContain('-50')
    expect(text).toContain('+500')
    // 四张卡都渲染（keys 兜底 0）
    for (const label of ['金币净流入', '钻石净流入', '体力净流入', '钥匙净流入']) {
      expect(text).toContain(label)
    }
    // gem > 0 提示贬值；coin < 0 / energy 0 / keys 无 → 健康
    expect(text).toContain('在贬值，考虑回收')
    expect(text).toContain('健康')
    wrapper.unmount()
  })

  it('flowTone 三档直测：正→warning，零→info，负→success；num 的 NaN/字符串分支', async () => {
    const wrapper = await mountEconomy()
    const vm = wrapper.vm as unknown as {
      flowTone: (n: number) => string
      num: (o: Row, k: string) => number
    }
    expect(vm.flowTone(1)).toBe('warning')
    expect(vm.flowTone(0)).toBe('info')
    expect(vm.flowTone(-1)).toBe('success')
    // num：数字字符串解析、NaN 与 null 回退 0
    expect(vm.num({ a: '8' }, 'a')).toBe(8)
    expect(vm.num({ a: 'abc' }, 'a')).toBe(0)
    expect(vm.num({ a: null }, 'a')).toBe(0)
    wrapper.unmount()
  })

  it('操作列按钮（改价/限购/改名）逐个点击：prompt 结果按字段类型解析提交', async () => {
    const row: Row = { id: 1, name: '体力瓶', currency: 'coin', price: 100, limit: 0, on_sale: 1 }
    updateShopItem.mockResolvedValue({ item: {} })
    box.prompt
      .mockResolvedValueOnce({ value: '120' }) // 改价 → Number
      .mockResolvedValueOnce({ value: '3' }) // 限购 → Number
      .mockResolvedValueOnce({ value: '超级体力瓶' }) // 改名 → 字符串
    const wrapper = await mountEconomy([], [row])
    const buttons = wrapper.findAll('.el-table__row')[0].findAll('button')
    expect(buttons.map((b) => b.text())).toEqual(['改价', '限购', '改名'])

    await buttons[0].trigger('click')
    await flushPromises()
    expect(updateShopItem).toHaveBeenLastCalledWith(1, { price: 120 })

    await buttons[1].trigger('click')
    await flushPromises()
    expect(updateShopItem).toHaveBeenLastCalledWith(1, { limit: 3 })

    await buttons[2].trigger('click')
    await flushPromises()
    expect(updateShopItem).toHaveBeenLastCalledWith(1, { name: '超级体力瓶' })
    wrapper.unmount()
  })

  it('流水表：来源中文映射与未知回退、人均分母为零显示 —', async () => {
    const wrapper = await mountEconomy(
      [
        { currency: 'coin', amount: '150', cnt: 3, reason: 'battle_loot' },
        { currency: 'gem', amount: 500, cnt: 2, reason: 'mystery_reason' },
        { currency: 'energy', amount: 0, cnt: 0, reason: 'signin' },
      ],
      [],
    )
    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].text()).toContain('战斗掉落')
    expect(rows[0].text()).toContain('50.0') // 150/3
    expect(rows[1].text()).toContain('mystery_reason') // 未知来源回退原文
    expect(rows[1].text()).toContain('250.0') // 500/2
    expect(rows[2].text()).toContain('—') // 0 笔 → 无人均
    wrapper.unmount()
  })
})

describe('EconomyView 商城编辑', () => {
  it('改价：数值输入转 Number 提交并合并返回', async () => {
    box.prompt.mockResolvedValueOnce({ value: '120' })
    const row: Row = { id: 1, name: '体力瓶', currency: 'coin', price: 100, limit: 0, on_sale: 1 }
    updateShopItem.mockResolvedValueOnce({ item: { price: 120 } })
    const wrapper = await mountEconomy([], [row])
    const vm = wrapper.vm as unknown as {
      patchShop: (r: Row, field: string, label: string) => Promise<void>
    }
    await vm.patchShop(row, 'price', '价格')
    await flushPromises()

    expect(updateShopItem).toHaveBeenCalledWith(1, { price: 120 })
    expect(row.price).toBe(120)
    expect(document.body.textContent).toContain('已保存')
    wrapper.unmount()
  })

  it('限购与库存走同一个数值分支；改名保留字符串', async () => {
    const row: Row = { id: 2, name: '改名前', price: 1 }
    updateShopItem.mockResolvedValue({ item: {} })
    box.prompt
      .mockResolvedValueOnce({ value: '3' }) // limit
      .mockResolvedValueOnce({ value: '9' }) // stock
      .mockResolvedValueOnce({ value: '新名字' }) // name
    const wrapper = await mountEconomy([], [row])
    const vm = wrapper.vm as unknown as {
      patchShop: (r: Row, field: string, label: string) => Promise<void>
    }
    await vm.patchShop(row, 'limit', '限购次数')
    expect(updateShopItem).toHaveBeenLastCalledWith(2, { limit: 3 })
    await vm.patchShop(row, 'stock', '库存')
    expect(updateShopItem).toHaveBeenLastCalledWith(2, { stock: 9 })
    await vm.patchShop(row, 'name', '名称')
    expect(updateShopItem).toHaveBeenLastCalledWith(2, { name: '新名字' })
    wrapper.unmount()
  })

  it('弹窗取消：不提交', async () => {
    box.prompt.mockRejectedValueOnce('cancel')
    const wrapper = await mountEconomy([], [{ id: 1, name: '体力瓶' }])
    const vm = wrapper.vm as unknown as {
      patchShop: (r: Row, field: string, label: string) => Promise<void>
    }
    await vm.patchShop({ id: 1, name: '体力瓶' }, 'price', '价格')
    await flushPromises()
    expect(updateShopItem).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('商城表渲染：限购 0 显示「不限」、上架状态双色、null 名称显示空', async () => {
    const wrapper = await mountEconomy(
      [],
      [
        { id: 1, name: '体力瓶', currency: 'coin', price: 100, limit: 0, on_sale: 1 },
        { id: 2, name: null, currency: 'gem', price: '20', limit: 5, on_sale: 0 },
      ],
    )
    const text = wrapper.text()
    expect(text).toContain('不限')
    expect(text).toContain('5')
    expect(text).not.toContain('null')
    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].find('.el-tag--success').exists()).toBe(true)
    expect(rows[1].find('.el-tag--info').exists()).toBe(true)
    wrapper.unmount()
  })

  it('加载失败：弹出错误消息', async () => {
    fetchEconomy.mockRejectedValueOnce(new Error('经济服务不可用'))
    const wrapper = mount(EconomyView, { global: { plugins: [ElementPlus] } })
    await flushPromises()
    expect(document.body.textContent).toContain('经济数据加载失败：经济服务不可用')
    wrapper.unmount()
  })
})
