/**
 * EconomyView.vue 测试。
 *
 * 经济页核心是通胀监控（净流入聚合）与商城项编辑的价格/文本分支。
 * netFlow 的累加（首现 ?? 0 与续加）、flowTone 三档、
 * patchShop 对 price/limit/stock 转 Number 而 name 保留字符串的分支逐一锁定。
 *
 * ───────────────────────────────────────────────────────────────────────────
 * 2026-10-21：fixture 换成了**服务端真实形状**（`service.AdminEconomyResp` /
 * `service.AdminShopItemView`），并且补了「上架/下架」这条新入口的用例。
 *
 * 此前 `num()` 这个「什么都往里塞」的兜底被测得很细（数字字符串、NaN、null），
 * 而它服务的其实是**服务端根本不会发的形状**。现在类型是真的，
 * 兜底可以删了 —— 判据也随之换成「真实字段算对了没有」。
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

/** 与 service.AdminShopItemView 一致。 */
function shopFixture(overrides: Row = {}): Row {
  return {
    id: 1,
    code: 'energy_bottle',
    name: '体力瓶',
    category: 'consumable',
    price: { coin: 100 },
    payload: { energy: 5 },
    limit_per_day: 0,
    sort_order: 1,
    enabled: true,
    ...overrides,
  }
}

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
  it('净流入聚合：同币种累加、四种货币卡齐全', async () => {
    // netFlow 读服务端 AdminEconomy 流水行的 `delta`（SUM）与 `currency`。
    // ⚠️ 第 112 轮前读的是 `amount`（不存在的字段）→ 恒 0，通胀监控形同虚设。
    const wrapper = await mountEconomy(
      [
        { currency: 'coin', delta: 150, count: 3, reason: 'battle_loot' },
        { currency: 'coin', delta: -200, count: 0, reason: 'mystery_reason' },
        { currency: 'gem', delta: 500, count: 2, reason: 'shop' },
        { currency: 'energy', delta: 0, count: 0, reason: 'signin' },
      ],
      [],
    )
    const text = wrapper.text()
    // coin: 150 - 200 = -50；gem: +500；energy: 0；keys: 无数据 → 0
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

  // 第 112 轮守卫：净流入 KPI 必须真的等于 Σ delta。
  // 若有人把字段读回 `amount`（修前 bug），delta=500 的 gem 卡会显示 0/健康，
  // 这条断言直接红。
  it('净流入 KPI = Σ delta（而非恒 0）', async () => {
    const wrapper = await mountEconomy(
      [{ currency: 'coin', delta: 300, count: 1, reason: 'battle_loot' }],
      [],
    )
    const coinCard = wrapper.text()
    expect(coinCard).toContain('+300')
    expect(coinCard).toContain('在贬值，考虑回收') // 300 > 0
    wrapper.unmount()
  })

  it('flowTone 三档直测：正→warning，零→info，负→success', async () => {
    const wrapper = await mountEconomy()
    const vm = wrapper.vm as unknown as { flowTone: (n: number) => string }
    expect(vm.flowTone(1)).toBe('warning')
    expect(vm.flowTone(0)).toBe('info')
    expect(vm.flowTone(-1)).toBe('success')
    wrapper.unmount()
  })

  it('操作列按钮（改价/限购/改名/上下架）逐个点击：按服务端形状解析提交（第 113 轮）', async () => {
    const row = shopFixture()
    updateShopItem.mockResolvedValue({ item: {} })
    box.prompt
      .mockResolvedValueOnce({ value: 'coin:120' }) // 改价 → 对象
      .mockResolvedValueOnce({ value: '3' }) // 限购 → 非负整数
      .mockResolvedValueOnce({ value: '超级体力瓶' }) // 改名 → 字符串
    const wrapper = await mountEconomy([], [row])
    const buttons = wrapper.findAll('.el-table__row')[0].findAll('button')
    expect(buttons.map((b) => b.text())).toEqual(['改价', '限购', '改名', '下架'])

    await buttons[0].trigger('click')
    await flushPromises()
    expect(updateShopItem).toHaveBeenLastCalledWith(1, { price: { coin: 120 } })

    await buttons[1].trigger('click')
    await flushPromises()
    expect(updateShopItem).toHaveBeenLastCalledWith(1, { limit_per_day: 3 })

    await buttons[2].trigger('click')
    await flushPromises()
    expect(updateShopItem).toHaveBeenLastCalledWith(1, { name: '超级体力瓶' })
    wrapper.unmount()
  })

  it('流水表：来源中文映射与未知回退、人均分母为零显示 —（第 112 轮：读 delta/count）', async () => {
    const wrapper = await mountEconomy(
      [
        { currency: 'coin', delta: 150, count: 3, reason: 'battle_loot' },
        { currency: 'gem', delta: 500, count: 2, reason: 'mystery_reason' },
        { currency: 'energy', delta: 0, count: 0, reason: 'signin' },
      ],
      [],
    )
    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].text()).toContain('战斗掉落')
    expect(rows[0].text()).toContain('50.0') // 150/3 人均单笔
    expect(rows[0].text()).toContain('150') // 总量列 = delta
    expect(rows[0].text()).toContain('3') // 笔数 = count
    expect(rows[1].text()).toContain('mystery_reason') // 未知来源回退原文
    expect(rows[1].text()).toContain('250.0') // 500/2
    expect(rows[2].text()).toContain('—') // 0 笔 → 无人均
    wrapper.unmount()
  })
})

describe('EconomyView 商城编辑', () => {
  it('改价：「coin:120, gem:5」解析成 map 提交（第 113 轮：不再发裸数字）', async () => {
    const row = shopFixture()
    updateShopItem.mockResolvedValueOnce({ item: { id: 1, updated: ['price'] } })
    box.prompt.mockResolvedValueOnce({ value: 'coin:120, gem:5' })
    const wrapper = await mountEconomy([], [row])
    const vm = wrapper.vm as unknown as {
      patchShop: (r: Row, field: string, label: string) => Promise<void>
    }
    await vm.patchShop(row, 'price', '价格')
    await flushPromises()

    // 关键：price 必须提交成对象（服务端 JSONB 是 map[string]int，
    // 玩家 Buy 按 map 读；裸数字落库会让该商品购买链路永久 500）
    expect(updateShopItem).toHaveBeenCalledWith(1, { price: { coin: 120, gem: 5 } })
    // 保存后整页重拉（响应只有 {id,updated}，旧数据 assign 没意义）
    expect(fetchEconomy).toHaveBeenCalledTimes(2)
    expect(document.body.textContent).toContain('已保存')
    wrapper.unmount()
  })

  it('改价：裸数字/未知货币/负数全部拒绝提交', async () => {
    const row = shopFixture()
    const wrapper = await mountEconomy([], [row])
    const vm = wrapper.vm as unknown as {
      patchShop: (r: Row, field: string, label: string) => Promise<void>
    }
    for (const bad of ['120', 'cino:5', 'coin:-3']) {
      box.prompt.mockResolvedValueOnce({ value: bad })
      await vm.patchShop(row, 'price', '价格')
      await flushPromises()
      expect(document.body.textContent).toContain('价格格式应为')
    }
    expect(updateShopItem).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('限购走 limit_per_day 键（服务端白名单），非法值拒绝；改名保留字符串', async () => {
    const row = shopFixture({ id: 2, name: '改名前' })
    updateShopItem.mockResolvedValue({ item: {} })
    box.prompt
      .mockResolvedValueOnce({ value: '3' }) // limit_per_day
      .mockResolvedValueOnce({ value: 'abc' }) // 非法
      .mockResolvedValueOnce({ value: '新名字' }) // name
    const wrapper = await mountEconomy([], [row])
    const vm = wrapper.vm as unknown as {
      patchShop: (r: Row, field: string, label: string) => Promise<void>
    }
    await vm.patchShop(row, 'limit_per_day', '限购次数')
    await flushPromises()
    expect(updateShopItem).toHaveBeenLastCalledWith(2, { limit_per_day: 3 })

    await vm.patchShop(row, 'limit_per_day', '限购次数')
    await flushPromises()
    expect(document.body.textContent).toContain('限购须为非负整数')
    expect(updateShopItem).toHaveBeenCalledTimes(1) // 非法值没提交

    await vm.patchShop(row, 'name', '名称')
    await flushPromises()
    expect(updateShopItem).toHaveBeenLastCalledWith(2, { name: '新名字' })
    wrapper.unmount()
  })

  it('弹窗取消：不提交', async () => {
    box.prompt.mockRejectedValueOnce('cancel')
    const wrapper = await mountEconomy([], [shopFixture()])
    const vm = wrapper.vm as unknown as {
      patchShop: (r: Row, field: string, label: string) => Promise<void>
    }
    await vm.patchShop(shopFixture(), 'price', '价格')
    await flushPromises()
    expect(updateShopItem).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('商城表渲染：价格 map / 限购 limit_per_day / 上架 enabled（第 112 轮服务端真实形状）', async () => {
    const wrapper = await mountEconomy(
      [],
      [
        shopFixture(),
        shopFixture({ id: 2, name: '', price: { gem: 20, coin: 5 }, limit_per_day: 5, enabled: false }),
      ],
    )
    const text = wrapper.text()
    // 价格 map 渲染成「金币:100」「钻石:20、金币:5」，不再是 num(object)=0
    expect(text).toContain('金币:100')
    expect(text).toContain('钻石:20')
    expect(text).toContain('金币:5')
    // 限购：0 → 不限；5 → 5
    expect(text).toContain('不限')
    expect(text).toContain('5')
    expect(text).not.toContain('null')
    const rows = wrapper.findAll('.el-table__row')
    expect(rows[0].find('.el-tag--success').exists()).toBe(true) // enabled=true 上架
    expect(rows[1].find('.el-tag--info').exists()).toBe(true) // enabled=false 未上架
    wrapper.unmount()
  })

  /**
   * 2026-10-21（P5-5）：上架/下架是**新增**的入口 ——
   * 服务端 `PUT /admin/shop/:id` 早就支持 `enabled`，后台一直没有入口，
   * 于是「临时下架一个出问题的商品」运营根本做不到。
   *
   * 下架是破坏性动作（玩家立刻买不到），所以必须二次确认：
   * 取消 → 不提交；确认 → 提交 `enabled: false`。
   */
  it('下架商品：先二次确认，确认后提交 enabled=false', async () => {
    const row = shopFixture({ enabled: true })
    updateShopItem.mockResolvedValue({ item: { id: 1, updated: ['enabled'] } })
    box.confirm.mockResolvedValue('ok')
    const wrapper = await mountEconomy([], [row])
    const btn = wrapper.findAll('.el-table__row')[0].findAll('button').find((b) => b.text() === '下架')
    expect(btn, '找不到「下架」按钮').toBeTruthy()
    await btn!.trigger('click')
    await flushPromises()
    expect(box.confirm).toHaveBeenCalled()
    expect(updateShopItem).toHaveBeenCalledWith(1, { enabled: false })
    wrapper.unmount()
  })

  it('下架商品：确认框取消则不提交', async () => {
    const row = shopFixture({ enabled: true })
    box.confirm.mockRejectedValue('cancel')
    const wrapper = await mountEconomy([], [row])
    const btn = wrapper.findAll('.el-table__row')[0].findAll('button').find((b) => b.text() === '下架')
    await btn!.trigger('click')
    await flushPromises()
    expect(updateShopItem).not.toHaveBeenCalled()
    wrapper.unmount()
  })

  it('未上架商品显示「上架」按钮，提交 enabled=true', async () => {
    const row = shopFixture({ enabled: false })
    updateShopItem.mockResolvedValue({ item: { id: 1, updated: ['enabled'] } })
    box.confirm.mockResolvedValue('ok')
    const wrapper = await mountEconomy([], [row])
    const btn = wrapper.findAll('.el-table__row')[0].findAll('button').find((b) => b.text() === '上架')
    expect(btn, '找不到「上架」按钮').toBeTruthy()
    await btn!.trigger('click')
    await flushPromises()
    expect(updateShopItem).toHaveBeenCalledWith(1, { enabled: true })
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
