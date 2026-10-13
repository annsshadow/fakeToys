// @vitest-environment jsdom
/**
 * 商城页（shop.vue）的行为测试。
 *
 * # 为什么这页的测试重心在「买完之后刷新什么」
 *
 * 商城最容易出的两类错都不是「买不动」，而是：
 *
 *	1. 买成功了，但界面还显示旧余额 / 旧购买次数
 *	   → 玩家在限次边界上再点一次，被服务端拒绝，体验像「买了又没买」
 *	2. 买失败了（钱不够 / 已售罄），但界面仍显示还能买
 *	   → 玩家反复点，得到的全是同一个错误
 *
 * 所以断言全部围绕「买完/失败后必须重拉」而不是「点了会发请求」。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Shop from './shop.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  fetchShop: vi.fn(),
  buyItem: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

let um: ReturnType<typeof installUniMock>

/** 一件钻石商品：买了会掉钻石，所以余额变化是可断言的。 */
function shopList() {
  return [
    {
      id: 9, code: 'shop_gem', name: '随机宝石', category: 'gem',
      price: { gem: 60 }, payload: { gem_instance: 1 },
      limit_per_day: 3, bought_today: 0,
    },
    {
      id: 8, code: 'shop_gem_wash', name: '洗练石', category: 'boost',
      price: { coin: 30000 }, payload: { gem_wash_token: 1 },
      limit_per_day: 3, bought_today: 3,
    },
    {
      id: 5, code: 'shop_retry', name: '续战令', category: 'boost',
      price: { coin: 5000 }, payload: { revive_token: 1 },
      limit_per_day: 5, bought_today: 0,
    },
  ]
}

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified', config: { skills: [], gems: [] } as any, serverTimeMs: 0,
  })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skill_ids: [] })
  mockApi.fetchShop.mockResolvedValue(shopList() as any)
  mockApi.fetchWallet.mockResolvedValue({ coin: 52000, gem: 290, energy: 3, keys: 4, tokens: {} })
  mockApi.buyItem.mockResolvedValue({} as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('shop.vue 商城页', () => {
  it('列出商品并写清「花费 → 产出」，玩家能判断值不值', async () => {
    const { wrapper } = mountPage(Shop)
    await flushPromises()
    const text = wrapper.text()
    expect(text).toContain('随机宝石')
    // 宝石不是货币，产出必须写清是「一颗实体」而不是「1 宝石」
    expect(text).toContain('随机宝石 1 颗')
    // 三种道具都列出来 —— 它们此前唯一的发放途径就是这个页面
    expect(text).toContain('洗练石')
    expect(text).toContain('续战令')
    expect(text).toContain('30000 金币')
    expect(text).toContain('60 钻石')
  })

  it('购买成功后重拉商品与钱包：余额与已买次数都得以服务端为准', async () => {
    const { wrapper } = mountPage(Shop)
    await flushPromises()
    expect(wrapper.text()).toContain('290')

    // 买完之后服务端说：宝石那件已买 1 次、钻石扣到 230
    mockApi.fetchShop.mockResolvedValue(
      shopList().map((x) => (x.id === 9 ? { ...x, bought_today: 1 } : x)) as any,
    )
    mockApi.fetchWallet.mockResolvedValue({ coin: 52000, gem: 230, energy: 3, keys: 4, tokens: {} })
    await wrapper.findAll('.shop-row').at(0)!.find('.link')!.trigger('click')
    await flushPromises()
    await flushPromises()

    expect(mockApi.buyItem).toHaveBeenCalledWith(9)
    expect(mockApi.fetchShop.mock.calls.length).toBe(2)
    // 余额跟着服务端走 —— 本地减 60 在限次/退款场景下会与真实值不符
    expect(wrapper.text()).toContain('230')
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '购买成功', icon: 'none' })
  })

  it('限次买满的商品显示「已售罄」且点击不发请求', async () => {
    const { wrapper } = mountPage(Shop)
    await flushPromises()
    expect(wrapper.text()).toContain('今日已买满')

    // 洗练石那行（bought_today=3, limit=3）
    const row = wrapper.findAll('.shop-row').at(1)!
    const link = row.find('.link')!
    expect(link.text()).toBe('已售罄')
    await link.trigger('click')
    await flushPromises()
    expect(mockApi.buyItem).not.toHaveBeenCalled()
  })

  it('不限次商品（limit_per_day=0）不会因为 bought_today=0 被误判成售罄', async () => {
    mockApi.fetchShop.mockResolvedValue([
      {
        id: 6, code: 'shop_mastery_reset', name: '专精点重置', category: 'boost',
        price: { gem: 80 }, payload: { mastery_reset: 1 },
        limit_per_day: 0, bought_today: 0,
      },
    ] as any)
    const { wrapper } = mountPage(Shop)
    await flushPromises()
    expect(wrapper.text()).not.toContain('已售罄')
    expect(wrapper.text()).toContain('购买')
  })

  it('购买失败 → toast 原因并重拉（余额/限次可能已被别的请求改变）', async () => {
    mockApi.buyItem.mockRejectedValue(new Error('金币不足'))
    const { wrapper } = mountPage(Shop)
    await flushPromises()
    await wrapper.findAll('.shop-row').at(2)!.find('.link')!.trigger('click')
    await flushPromises()
    await flushPromises()

    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '金币不足', icon: 'none' })
    // 失败也要重拉：页面上那份 bought_today 此刻是陈旧的
    expect(mockApi.fetchShop.mock.calls.length).toBe(2)
  })

  it('拉不到商城 → 页面留下错误原因，不显示成「商城空」', async () => {
    mockApi.fetchShop.mockRejectedValue(new Error('网络炸了'))
    const { wrapper } = mountPage(Shop)
    await flushPromises()
    // 「空」与「拉不到」必须长得不一样：前者让玩家以为自己错过了什么
    expect(wrapper.text()).not.toContain('商城暂无可售商品')
    expect(wrapper.text()).toContain('加载失败')
    expect(wrapper.text()).toContain('网络炸了')
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '网络炸了', icon: 'none' })
  })

  it('上一笔购买未完成时，点另一件也不会再发请求（防重复扣币）', async () => {
    let resolveBuy: (v: any) => void = () => {}
    mockApi.buyItem.mockImplementation(() => new Promise((r) => { resolveBuy = r }))
    const { wrapper } = mountPage(Shop)
    await flushPromises()

    // 点宝石（id=9），请求挂着不返回
    await wrapper.findAll('.shop-row').at(0)!.find('.link')!.trigger('click')
    await flushPromises()
    expect(wrapper.text()).toContain('购买中')

    // 此时点**另一行**——它的按钮没被 v-if 挡住，所以这一步真正打到
    // `buyingId !== 0` 那道守卫上（同一点两次会被 v-if 拦掉，测不到守卫）。
    await wrapper.findAll('.shop-row').at(2)!.find('.link')!.trigger('click')
    await flushPromises()
    expect(mockApi.buyItem).toHaveBeenCalledTimes(1)

    resolveBuy({})
    await flushPromises()
    expect(mockApi.buyItem).toHaveBeenCalledTimes(1)
    expect(mockApi.buyItem).toHaveBeenCalledWith(9)
  })

  it('钱包拉不到时仍能下单：价格写在每一行里', async () => {
    mockApi.fetchWallet.mockRejectedValue(new Error('钱包炸了'))
    const { wrapper } = mountPage(Shop)
    await flushPromises()
    expect(wrapper.text()).toContain('30000 金币')
    await wrapper.findAll('.shop-row').at(0)!.find('.link')!.trigger('click')
    await flushPromises()
    expect(mockApi.buyItem).toHaveBeenCalledWith(9)
  })
})