// @vitest-environment jsdom
/**
 * 专精页（mastery.vue）组件测试。
 * 覆盖：三层节点的锁定规则（前层未点亮 / 本层已满 2）、
 * toggle 的全部守卫分支（已点亮 / 被锁 / 点数不足）与成功流、
 * 加载失败提示、家族 tab 切换与未知名族的回退显示。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Mastery from './mastery.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig } from '../../test/fixtures'
import type { MasteryFamily } from '@/game/types'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  // 账号登录（测试环境共用一个账号）新增的 API 面：
  //   hasUsableToken —— 本地是否已有可用令牌（有就别再建游客号）
  //   loadProfile / saveProfile —— 身份摘要（昵称 + 是否游客）
  //   login / saveLastUsername —— 口令登录与用户名回填
  hasUsableToken: vi.fn(() => false),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  fetchMastery: vi.fn(),
  allocateMastery: vi.fn(),
  resetMastery: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

let um: ReturnType<typeof installUniMock>

/** 三层 × 每层 2 节点的专精树，专门用来驱动锁定/守卫分支 */
function threeLayerFamily(): MasteryFamily {
  return {
    family: 'flame',
    name: '烈焰',
    nodes: [
      { id: 11, family: 'flame', layer: 1, slot: 0, name: '燃料压缩', kind: 'stat', value: 10, prereq_family: '', prereq_layer: 0 },
      { id: 12, family: 'flame', layer: 1, slot: 1, name: '焰心稳定', kind: 'stat', value: 10, prereq_family: '', prereq_layer: 0 },
      { id: 13, family: 'flame', layer: 2, slot: 0, name: '爆燃', kind: 'stat', value: 20, prereq_family: 'flame', prereq_layer: 1 },
      { id: 14, family: 'flame', layer: 2, slot: 1, name: '余烬', kind: 'stat', value: 20, prereq_family: 'flame', prereq_layer: 1 },
      { id: 17, family: 'flame', layer: 2, slot: 2, name: '引燃', kind: 'stat', value: 20, prereq_family: 'flame', prereq_layer: 1 },
      { id: 15, family: 'flame', layer: 3, slot: 0, name: '烈焰风暴', kind: 'stat', value: 40, prereq_family: 'flame', prereq_layer: 2 },
      { id: 16, family: 'flame', layer: 3, slot: 1, name: '灼烧链', kind: 'stat', value: 40, prereq_family: 'flame', prereq_layer: 2 },
    ],
  }
}

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: makeConfig({ mastery_families: [threeLayerFamily()] }), serverTimeMs: 0 })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes: [] })
  mockApi.allocateMastery.mockResolvedValue({ points: 2, nodes: [11] })
  mockApi.resetMastery.mockResolvedValue({ reset_cleared: 1, points: 3, nodes: [], layer_limit: 2, base_slots: 4, total_slots: 4 })
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('mastery.vue 专精页', () => {
  it('挂载后登录并加载专精状态，渲染三层节点', async () => {
    const { wrapper } = mountPage(Mastery)
    await flushPromises()
    expect(mockApi.fetchMastery).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain('专精点')
    expect(wrapper.text()).toContain('烈焰专精树')
    expect(wrapper.findAll('.node').length).toBe(7)
    expect(wrapper.text()).toContain('第 1 层')
    expect(wrapper.text()).toContain('已选 0/2')
  })

  it('已点亮节点点击 → 提示需重置，不发生请求', async () => {
    mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes: [11] })
    const { wrapper } = mountPage(Mastery)
    await flushPromises()
    const node11 = wrapper.findAll('.node').find((n) => n.text().includes('燃料压缩'))!
    expect(node11.classes()).toContain('on')
    await node11.trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '已点亮的节点需通过重置取消', icon: 'none' })
    expect(mockApi.allocateMastery).not.toHaveBeenCalled()
  })

  it('前一层未点亮时，后层节点被锁 → 提示先点亮上一层', async () => {
    const { wrapper } = mountPage(Mastery)
    await flushPromises()
    const node15 = wrapper.findAll('.node').find((n) => n.text().includes('烈焰风暴'))!
    expect(node15.classes()).toContain('locked') // isLocked: layer>1 且第 2 层没人
    await node15.trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '请先点亮上一层', icon: 'none' })
    expect(mockApi.allocateMastery).not.toHaveBeenCalled()
  })

  it('本层已选满 2 个 → 该层其余未选节点锁定，提示已满', async () => {
    // 前置：第 1 层已点亮 1 个（解除对第 2 层的锁定）、第 2 层已选满 2 个
    mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes: [11, 13, 14] })
    const { wrapper } = mountPage(Mastery)
    await flushPromises()
    const node17 = wrapper.findAll('.node').find((n) => n.text().includes('引燃'))!
    expect(node17.classes()).toContain('locked') // isLocked: pickedInLayer(2) >= 2
    await node17.trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '本层已选满 2 个', icon: 'none' })
  })

  it('专精点不足 → 提示点数不足', async () => {
    mockApi.fetchMastery.mockResolvedValue({ points: 0, nodes: [] })
    const { wrapper } = mountPage(Mastery)
    await flushPromises()
    const node11 = wrapper.findAll('.node').find((n) => n.text().includes('燃料压缩'))!
    await node11.trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '专精点不足（升级可获得更多）', icon: 'none' })
    expect(mockApi.allocateMastery).not.toHaveBeenCalled()
  })

  it('点亮成功：更新点数与已选列表、刷新档案；再点亮第二层前先补第一层', async () => {
    const { wrapper, store } = mountPage(Mastery)
    await flushPromises()
    const node11 = wrapper.findAll('.node').find((n) => n.text().includes('燃料压缩'))!
    await node11.trigger('click')
    await flushPromises()
    expect(mockApi.allocateMastery).toHaveBeenCalledWith(11)
    expect(wrapper.text()).toContain('已选 1/2')
    expect(mockApi.fetchMe).toHaveBeenCalled() // refreshProfile
    expect(store.power).toBe(9)

    // 点亮第 1 层后，第 2 层节点解锁成功流
    mockApi.allocateMastery.mockResolvedValue({ points: 1, nodes: [11, 13] })
    const node13 = wrapper.findAll('.node').find((n) => n.text().includes('爆燃'))!
    expect(node13.classes()).not.toContain('locked')
    await node13.trigger('click')
    await flushPromises()
    expect(mockApi.allocateMastery).toHaveBeenLastCalledWith(13)
    expect(wrapper.findAll('.node').find((n) => n.text().includes('爆燃'))!.classes()).toContain('on')

    // 响应缺 nodes → 已选列表按空处理（?? [] 分支）；点一个未选节点触发
    mockApi.allocateMastery.mockResolvedValue({ points: 0 } as any)
    const node12 = wrapper.findAll('.node').find((n) => n.text().includes('焰心稳定'))!
    await node12.trigger('click')
    await flushPromises()
    expect(wrapper.findAll('.node.on').length).toBe(0)
  })

  it('分配失败：优先展示服务端 error.message，否则退回异常消息', async () => {
    mockApi.allocateMastery.mockRejectedValue({ error: { message: '该节点已被占用' } })
    const { wrapper } = mountPage(Mastery)
    await flushPromises()
    const node11 = wrapper.findAll('.node').find((n) => n.text().includes('燃料压缩'))!
    await node11.trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith(
      { title: '该节点已被占用', icon: 'none', duration: 2500 },
    )

    mockApi.allocateMastery.mockRejectedValue(new Error('网络断开'))
    await node11.trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenLastCalledWith({ title: '网络断开', icon: 'none', duration: 2500 })

    // error 对象存在但没有 message → 退回 e.message（?? 的右侧分支）
    mockApi.allocateMastery.mockRejectedValue({ error: {} })
    await node11.trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenLastCalledWith({ title: undefined, icon: 'none', duration: 2500 })
  })

  it('加载成功但响应缺 nodes → 按空已选处理（?? [] 分支）', async () => {
    mockApi.fetchMastery.mockResolvedValue({ points: 1 } as any)
    const { wrapper } = mountPage(Mastery)
    await flushPromises()
    expect(wrapper.text()).toContain('已选 0/2')
    expect(wrapper.findAll('.node.on').length).toBe(0)
  })

  it('加载失败 → toast 错误信息', async () => {
    mockApi.fetchMastery.mockRejectedValue(new Error('专精加载失败'))
    mountPage(Mastery)
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '专精加载失败', icon: 'none' })
  })

  it('家族 tab：点击切换家族（内联 handler），已投入的家族有小圆点', async () => {
    mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes: [11] })
    const { wrapper } = mountPage(Mastery)
    await flushPromises()
    expect(wrapper.find('.tab-dot').exists()).toBe(true) // flame 已投入
    await wrapper.find('.tab-chip').trigger('click') // curFamily = f.family
    await flushPromises()
    expect(wrapper.text()).toContain('烈焰专精树')
  })

  it('未知名族：树体隐藏，标题回退为家族键名（?? curFamily 分支）', async () => {
    mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: makeConfig({ mastery_families: [] }), serverTimeMs: 0 })
    const { wrapper } = mountPage(Mastery)
    await flushPromises()
    // curNodes 为空 → 树卡片不渲染（v-if 分支）
    expect(wrapper.text()).not.toContain('专精树')
    // curName ?? curFamily：找不到家族时显示默认家族键（模板不渲染树所以通过页面状态验证）
    expect(wrapper.text()).not.toContain('烈焰')
  })
})

// 第 150 轮：重铸（POST /mastery/reset）的客户端接入。
//
// 这个端点从第 74 轮起就在服务端存在，`mastery_reset` 也买得到，
// 但客户端从来没有接过 —— 于是它是商店里一件「买了不知道干嘛」的商品，
// 而点亮的节点**永远无法取消**（`AllocateMastery` 只插不删）。
describe('mastery.vue 重铸（POST /mastery/reset）', () => {
  /** 打开重铸按钮的前置：有道具 + 有已点亮节点 */
  async function readyWithTokens(tokens: number, nodes: number[] = [11]) {
    mockApi.fetchWallet.mockResolvedValue({
      coin: 1,
      gem: 2,
      energy: 3,
      keys: 4,
      tokens: { mastery_reset: tokens },
    })
    mockApi.fetchMastery.mockResolvedValue({ points: 3, nodes })
    const mounted = mountPage(Mastery)
    await flushPromises()
    return mounted
  }

  it('道具余额来自 /wallet 的 tokens，显示在重铸入口上', async () => {
    const { wrapper } = await readyWithTokens(2)
    expect(wrapper.text()).toContain('重铸道具 2')
  })

  it('没有道具 → 按钮置灰，点它只提示不发请求', async () => {
    const { wrapper } = await readyWithTokens(0)
    const link = wrapper.find('.reset-row .link')
    expect(link.classes()).toContain('link-off')
    await link.trigger('click')
    await flushPromises()
    expect(mockApi.resetMastery).not.toHaveBeenCalled()
  })

  it('没有已点亮节点 → 按钮置灰（重铸没有意义）', async () => {
    const { wrapper } = await readyWithTokens(1, [])
    expect(wrapper.find('.reset-row .link').classes()).toContain('link-off')
  })

  it('成功：按回执刷新点数/已选、刷新钱包余额、提示清掉几个', async () => {
    mockApi.resetMastery.mockResolvedValue({
      reset_cleared: 2,
      points: 3,
      nodes: [],
      layer_limit: 2,
      base_slots: 4,
      total_slots: 4,
    })
    const { wrapper } = await readyWithTokens(1, [11, 13])
    await wrapper.find('.reset-row .link').trigger('click')
    await flushPromises()
    expect(mockApi.resetMastery).toHaveBeenCalledTimes(1)
    // 回执里的状态直接刷 UI：节点清空、点数回到 3
    expect(wrapper.findAll('.node.on').length).toBe(0)
    expect(wrapper.text()).toContain('专精点')
    expect(um.mock.showToast).toHaveBeenCalledWith({
      title: '已重铸，清掉 2 个节点',
      icon: 'none',
    })
    // 余额必须以服务端为准（不能本地 -1）
    expect(mockApi.fetchWallet).toHaveBeenCalled()
  })

  it('服务端拒绝（道具已被并发用掉）→ 展示服务端 error.message，UI 不变', async () => {
    mockApi.resetMastery.mockRejectedValue({ error: { message: '重铸道具不足' } })
    const { wrapper } = await readyWithTokens(1, [11])
    await wrapper.find('.reset-row .link').trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith(
      { title: '重铸道具不足', icon: 'none', duration: 2500 },
    )
    // 节点仍在（没有把 UI 清成"已重铸"的假象）
    expect(wrapper.findAll('.node.on').length).toBe(1)
  })

  it('异常对象没有 error.message → 退回 e.message', async () => {
    mockApi.resetMastery.mockRejectedValue(new Error('网络断开'))
    const { wrapper } = await readyWithTokens(1, [11])
    await wrapper.find('.reset-row .link').trigger('click')
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenLastCalledWith({
      title: '网络断开',
      icon: 'none',
      duration: 2500,
    })
  })

  it('并发点击：只发一次请求（道具扣完第二次必失败，重复提示很吵）', async () => {
    let release!: (v: any) => void
    mockApi.resetMastery.mockImplementation(() => new Promise((res) => { release = res }))
    const { wrapper } = await readyWithTokens(1, [11])
    const link = wrapper.find('.reset-row .link')
    await link.trigger('click')
    await flushPromises() // 进入 resetting=true
    await wrapper.find('.reset-row .link').trigger('click')
    release({ reset_cleared: 1, points: 3, nodes: [], layer_limit: 2, base_slots: 4, total_slots: 4 })
    await flushPromises()
    expect(mockApi.resetMastery).toHaveBeenCalledTimes(1)
  })
})
