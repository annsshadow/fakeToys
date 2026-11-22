// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
﻿// @vitest-environment jsdom
/**
 * 首页（index.vue）组件测试。
 *
 * mock 掉 @/api/client 与全局 uni，把页面放进真实 Pinia + jsdom 里挂载，
 * 驱动模板三态（登录失败 / 配置加载中 / 正常）与全部交互分支：
 * onMounted 自动登录、onShow 刷新档案、菜单与主行动的导航参数。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { mount, flushPromises, type VueWrapper } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import Index from './index.vue'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { makeConfig } from '../../test/fixtures'

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
  onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  // 公告（2026-10-23）：首页现在会拉一次公告。
  // 不给 mock 的话 `api.fetchAnnouncements` 是 undefined，
  // 首页在 onMounted 里会抛 —— 而抛在异步里，症状是「公告区不出现」，
  // 不会让任何既有断言变红（典型的静默失效）。
  fetchAnnouncements: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

let um: ReturnType<typeof installUniMock>
let wrapper: VueWrapper<any>
let pinia: ReturnType<typeof createPinia>

function mountPage() {
  pinia = createPinia()
  setActivePinia(pinia)
  wrapper = mount(Index, { global: { plugins: [pinia] } })
  return wrapper
}

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '游客7', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: makeConfig(), serverTimeMs: 0 })
  mockApi.fetchWallet.mockResolvedValue({ coin: 10, gem: 20, energy: 30, keys: 40, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({
    user_id: 7,
    build: null,
    build_rating: { total: 42, element_coverage: 5, reaction_coverage: 7, mastery_done: 8, equipment_synergy: 6, mechanic_depth: 4, weaknesses: ['缺少冰系'] },
    power: 9,
  } as any)
  // 缺省：没有公告（绝大多数断言与公告无关，不该被它干扰）
  mockApi.fetchAnnouncements.mockResolvedValue({ items: [] })
})

afterEach(() => {
  wrapper?.unmount()
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('index.vue 首页', () => {
  it('挂载时未登录 → 自动登录并渲染正常态（资源栏 / 五维 / 菜单 / 元素说明）', async () => {
    mountPage()
    await flushPromises()
    const store = useGameStore()
    expect(store.loggedIn).toBe(true)
    expect(wrapper.text()).toContain('游客7')
    expect(wrapper.text()).toContain('游客账号')
    // 2026-11-17 v2：资源栏从「金 10」纯文本改为图标胶囊，断言改为胶囊值序列。
    // 第六批 R56：补上第四种货币「钥匙」，顺序为 金/钻/体/钥匙。
    const pillValues = wrapper.findAll('.res-pill .mono').map((el) => el.text())
    expect(pillValues).toEqual(['10', '20', '30', '40'])
    // buildRating 已下发：五维与短板建议出现
    expect(wrapper.text()).toContain('构筑评分')
    expect(wrapper.text()).toContain('5/5')
    expect(wrapper.text()).toContain('搭配建议')
    expect(wrapper.text()).toContain('缺少冰系')
    // 第 138 轮：五维必须全（此前漏 mechanic_depth 只有 4 条）
    expect(wrapper.findAll('.dim-row').length).toBe(5)
    expect(wrapper.text()).toContain('机制深度')
    expect(wrapper.text()).toContain('4/8') // mechanic_depth=4 → 4/8
    // 装备契合分母是 18（服务端上界），不是误写的 6
    expect(wrapper.text()).toContain('6/18')
    // 功能入口：不写死个数与位置（第 151 轮加商城后，写死 7 的断言
    // 在这里无关地红了 —— 它盯的是排版，不是行为）。
    const labels = wrapper.findAll('.grid-item .grid-label').map((el) => el.text())
    expect(labels).toContain('商城')
    expect(labels).toContain('专精')
    expect(labels).toContain('背包')
    // 5 种元素说明
    expect(wrapper.findAll('.elem-chip').length).toBe(5)
  })

  it('登录失败 → 错误卡片 + 重试按钮；点击重试重新登录', async () => {
    mockApi.guestLogin.mockRejectedValue(new Error('无法连接服务器'))
    mountPage()
    await flushPromises()
    expect(wrapper.text()).toContain('无法连接服务器')
    expect(wrapper.text()).toContain('cd server && go run ./cmd/api')

    mockApi.guestLogin.mockResolvedValue({
      access_token: 'a',
      refresh_token: 'r',
      expires_at: '',
      user: { id: 1, nickname: 'n', avatar_url: '', is_guest: true, status: 1 },
    })
    await wrapper.find('.btn-primary').trigger('click')
    await flushPromises()
    // 重试成功后回到正常态
    expect(wrapper.text()).toContain('构筑评分')
    expect(mockApi.guestLogin).toHaveBeenCalledTimes(2)
  })

  it('configLoading 中 → 显示加载文案（v-else-if 分支）', async () => {
    // 先把 store 置为「已登录 + 配置加载中」再挂载，避免 onMounted 的
    // 登录流程在 flush 时把 configLoading 复位成 false
    pinia = createPinia()
    setActivePinia(pinia)
    const store = useGameStore()
    store.loggedIn = true
    store.configLoading = true
    wrapper = mount(Index, { global: { plugins: [pinia] } })
    await flushPromises()
    expect(wrapper.text()).toContain('正在加载游戏配置…')
  })

  it('账号登录（isGuest=false）→ 顶栏显示「账号登录」（isGuest 三元分支）', async () => {
    pinia = createPinia()
    setActivePinia(pinia)
    const store = useGameStore()
    store.loggedIn = true
    store.isGuest = false
    store.nickname = '微信玩家'
    wrapper = mount(Index, { global: { plugins: [pinia] } })
    await flushPromises()
    // ⚠️ 原文案是「微信账号」，但微信通道未接通，而 isGuest=false 现在还
    // 多了一种来源：**账号口令登录**。继续写「微信账号」等于把口令账号
    // 说成微信账号 —— 事实不对，且「我明明用密码登的」这种困惑极难自查。
    expect(wrapper.text()).toContain('账号登录')
    expect(wrapper.text()).toContain('微信玩家')
  })

  it('buildRating 缺失 → 五维显示默认 0 且没有短板建议（?? 分支）', async () => {
    mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
    mountPage()
    await flushPromises()
    expect(wrapper.text()).toContain('0/5')
    expect(wrapper.text()).toContain('0/7')
    expect(wrapper.text()).toContain('0/8')
    expect(wrapper.find('.weakness-box').exists()).toBe(false)
  })

  it('equipment_synergy 超过 18 → 进度条宽度夹到 100%', async () => {
    mockApi.fetchMe.mockResolvedValue({
      user_id: 7,
      build: null,
      build_rating: { total: 1, element_coverage: 0, reaction_coverage: 0, mastery_done: 0, equipment_synergy: 99, mechanic_depth: 0, weaknesses: [] },
      power: 9,
    } as any)
    mountPage()
    await flushPromises()
    const fills = wrapper.findAll('.bar-fill')
    expect(fills.length).toBe(5) // 第 138 轮起五维齐全（原 4）
    // 第 4 维（索引 3）是装备契合：99/18 → 550% → 夹到 100%
    expect(fills[3]!.attributes('style')).toContain('width: 100%')
  })

  it('构筑评级（G141）用加权满分做分母，未加权上界之和 46 会让评级恒为 S', async () => {
    // 服务端 total = Σ(维度×权重)（power.go ComputeBuildRating:350-354），
    // 默认权重 12/15/10/12/8 → 加权满分 5*12+7*15+8*10+18*12+8*8 = 525。
    // index.vue 此前用未加权上界之和 5+7+8+18+8=46 做分母：
    // 250/525≈48% 应为 B，250/46≈543%→夹到 100% 变成 S —— 评级完全失真。
    mockApi.fetchConfigIfVersion.mockResolvedValue({
      status: 'modified',
      config: makeConfig({
        rating_weights: {
          element_coverage: 12,
          reaction_coverage: 15,
          mastery_done: 10,
          equipment_synergy: 12,
          mechanic_depth: 8,
        },
      }),
      serverTimeMs: 0,
    })
    mockApi.fetchMe.mockResolvedValue({
      user_id: 7,
      build: null,
      build_rating: {
        total: 250,
        element_coverage: 5,
        reaction_coverage: 7,
        mastery_done: 8,
        equipment_synergy: 6,
        mechanic_depth: 4,
        weaknesses: [],
      },
      power: 9,
    } as any)
    mountPage()
    await flushPromises()
    // 用户可见行为：徽章须是 B，而不是被爆表分母夹成 S。
    expect(wrapper.find('.rating-grade').text()).toBe('B')
  })

  it('点击功能入口 → uni.navigateTo 携带对应路径', async () => {
    mountPage()
    await flushPromises()
    const item = wrapper.findAll('.grid-item').find((el) => el.text().includes('专精'))!
    await item.trigger('click')
    expect(um.mock.navigateTo).toHaveBeenCalledWith({ url: '/pages/mastery/mastery' })
  })

  it('R7 新玩家首屏给出明确目标与下一步：引导卡指向第 1 关', async () => {
    mountPage()
    await flushPromises()
    const store = useGameStore()
    store.setMaxStage(0)
    await flushPromises()
    const guide = wrapper.find('.first-run-guide')
    expect(guide.exists()).toBe(true)
    expect(guide.text()).toContain('新玩家先打第 1 关')
    expect(guide.text()).toContain('守住防线')
    await guide.trigger('click')
    expect(um.mock.navigateTo).toHaveBeenCalledWith({ url: '/pages/stage/stage?level=1' })
  })

  it('点击主行动卡 → 跳转下一关（unlockedLevel = maxStage + 1）', async () => {
    mountPage()
    await flushPromises()
    const store = useGameStore()
    store.setMaxStage(5)
    await flushPromises()
    await wrapper.find('.action-card').trigger('click')
    expect(um.mock.navigateTo).toHaveBeenCalledWith({ url: '/pages/stage/stage?level=6' })
  })

  it('onShow 且已登录 → 刷新玩家档案', async () => {
    mountPage()
    await flushPromises()
    expect(mockApi.fetchWallet).toHaveBeenCalledTimes(1)
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()
    expect(mockApi.fetchWallet).toHaveBeenCalledTimes(2)
  })

  it('onShow 但未登录 → 不发档案请求', async () => {
    mockApi.guestLogin.mockImplementation(() => new Promise(() => {})) // 挂起，loggedIn 停留 false
    mountPage()
    await flushPromises()
    expect(mockApi.fetchWallet).not.toHaveBeenCalled()
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()
    expect(mockApi.fetchWallet).not.toHaveBeenCalled()
  })

  // ── 公告（2026-10-23）────────────────────────────────────────────
  // 这组断言盯的是「闭环」：服务端有公告表与后台，玩家端此前零入口。
  // 只断言「渲染了标题」是不够的 —— 还得断言**没有公告时不画空卡片**，
  // 否则「永远画一张空卡片」也能让第一条绿。

  it('有公告 → 渲染标题与正文，默认折叠，点击展开', async () => {
    mockApi.fetchAnnouncements.mockResolvedValue({
      items: [
        { id: 2, title: '双倍掉落', body: '周末两天所有关卡金币翻倍', published_at: '2026-10-23T02:00:00Z' },
        { id: 1, title: '开服公告', body: '欢迎来到来一炮', published_at: '2026-10-20T02:00:00Z' },
      ],
    })
    mountPage()
    await flushPromises()

    expect(wrapper.text()).toContain('双倍掉落')
    expect(wrapper.text()).toContain('周末两天所有关卡金币翻倍')
    expect(wrapper.text()).toContain('全部 2 条 →')
    // 默认折叠：正文带 clamped 类（视觉收起，内容不截断）
    const bodies = wrapper.findAll('.ann-body')
    expect(bodies.length).toBe(2)
    expect(bodies[0].classes()).toContain('clamped')
    // 点击第一条 → 展开
    await wrapper.findAll('.ann-item')[0].trigger('click')
    expect(wrapper.findAll('.ann-body')[0].classes()).not.toContain('clamped')
    // 再点一次 → 收回
    await wrapper.findAll('.ann-item')[0].trigger('click')
    expect(wrapper.findAll('.ann-body')[0].classes()).toContain('clamped')
  })

  it('无公告 → 不渲染空卡片（公告区整块消失）', async () => {
    mockApi.fetchAnnouncements.mockResolvedValue({ items: [] })
    mountPage()
    await flushPromises()
    expect(wrapper.findAll('.ann-item').length).toBe(0)
    expect(wrapper.text()).not.toContain('全部 0 条')
    // 首页主体仍正常
    expect(wrapper.text()).toContain('构筑评分')
  })

  it('首页公告“全部 N 条”显示真实总数，且首页只展示最新 3 条', async () => {
    // 运营发了 6 条公告；server 按 limit 截断（公告页用 limit=20 视为全量）。
    // 首页此前用 fetchAnnouncements(3) 拉 3 条，却把 announcements.length
    // 当“全部”条数 —— 6 条公告在首页显示成“全部 3 条”，一句用户可见的假话。
    const all = Array.from({ length: 6 }, (_, i) => ({
      id: i + 1,
      title: `公告${i + 1}`,
      body: '正文',
      published_at: '2026-11-17T00:00:00Z',
    }))
    mockApi.fetchAnnouncements.mockImplementation((limit?: number) =>
      Promise.resolve({ items: all.slice(0, limit ?? 20) }),
    )
    mountPage()
    await flushPromises()

    // 首页仍只铺最新 3 条（首屏不被挤爆）
    expect(wrapper.findAll('.ann-item').length).toBe(3)
    // “全部 N 条”必须是真实总数 6，而不是首页展示的 3 条
    expect(wrapper.text()).toContain('全部 6 条 →')
  })

  // ── 分享（2026-10-23）────────────────────────────────────────────
  // 判据是**行为**：微信真正用的就是回调的返回值。
  // 只断言「注册了回调」不够 —— 卡片内容错了只有分享出去的人看得见。

  it('分享 → 返回首页标题与路径', async () => {
    mountPage()
    await flushPromises()
    const payload = triggerUniHook(wrapper.vm, 'onShareAppMessage', { from: 'menu' }) as {
      title: string
      path: string
    }
    expect(payload.title).toBe('来一炮 · 塔防 + 元素反应')
    expect(payload.path).toBe('/pages/index/index')
  })

  it('公告请求在途时再次 onShow → 不并发第二次请求', async () => {
    // 首次请求挂起，模拟弱网
    let resolveFirst: (v: any) => void = () => {}
    mockApi.fetchAnnouncements.mockImplementation(
      () => new Promise((r) => (resolveFirst = r)) as any,
    )
    mountPage()
    await flushPromises()
    expect(mockApi.fetchAnnouncements).toHaveBeenCalledTimes(1)

    // 切后台再回来：annLoadedOnce 已是 true，但请求还没回来
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()
    expect(mockApi.fetchAnnouncements).toHaveBeenCalledTimes(1) // in-flight 去重

    resolveFirst({ items: [] })
    await flushPromises()
  })

  it('公告拉取失败 → 保留已展示内容（一次抖动不该把公告抹掉）', async () => {
    mockApi.fetchAnnouncements.mockResolvedValue({
      items: [{ id: 1, title: '维护公告', body: '今晚停机', published_at: null }],
    })
    mountPage()
    await flushPromises()
    expect(wrapper.text()).toContain('维护公告')

    // 第二次刷新失败
    mockApi.fetchAnnouncements.mockRejectedValue(new Error('network down'))
    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()

    // 内容仍在 —— 清空会让玩家以为运营撤了公告
    expect(wrapper.text()).toContain('维护公告')
  })

  it('公告拉取失败 → 首页不受影响，且不显示假数据', async () => {
    mockApi.fetchAnnouncements.mockRejectedValue(new Error('network down'))
    mountPage()
    await flushPromises()
    expect(wrapper.findAll('.ann-item').length).toBe(0)
    expect(wrapper.text()).toContain('构筑评分')
    // 失败不能变成「0 条公告」这种看起来正常的状态
    expect(wrapper.text()).not.toContain('全部 0 条')
  })

  // ── R42 资源变动高亮（X 批）────────────────────────────────────────
  it('R42 钱包数值变化时对应资源胶囊高亮', async () => {
    mountPage()
    await flushPromises()
    // 首次载入钱包也会触发一次高亮（可接受：像「资源到账」），这里只关心后续变化

    const store = useGameStore()
    store.wallet.coin = 999
    await flushPromises()
    const bumped = wrapper.findAll('.res-pill.res-bump')
    expect(bumped.length).toBe(1)
    expect(bumped[0]!.text()).toContain('999')
  })

  // ── Y4 R61–R64 ─────────────────────────────────────────────────────
  it('R61 元素 chip 可点下钻到反应图鉴', async () => {
    mountPage()
    await flushPromises()
    const chip = wrapper.findAll('.elem-chip')[0]!
    expect(chip.classes()).toContain('tap')
    await chip.trigger('click')
    expect(um.mock.navigateTo).toHaveBeenCalledWith({ url: '/pages/reactions/reactions' })
  })

  it('R62 已通关关数有进度提示', async () => {
    mountPage()
    await flushPromises()
    const store = useGameStore()
    store.setMaxStage(7)
    await flushPromises()
    expect(wrapper.find('.action-progress').text()).toContain('已通关 7 关')
  })

  it('R64 公告展开态箭头翻转', async () => {
    mockApi.fetchAnnouncements.mockResolvedValue({
      items: [{ id: 1, title: '公告标题', body: '正文', published_at: '2026-11-17T00:00:00Z' }],
    })
    mountPage()
    await flushPromises()
    const toggle = wrapper.find('.ann-toggle')
    expect(toggle.exists()).toBe(true)
    expect(toggle.classes()).not.toContain('open')
    await wrapper.find('.ann-item').trigger('click')
    await flushPromises()
    expect(wrapper.find('.ann-toggle').classes()).toContain('open')
  })
})

// ==================== 构筑评分空维补导（2026-11-17 100 轮打磨 R14）====================

describe('index.vue 构筑评分空维补导（R14）', () => {
  it('0 分维度给出可点补导入口，点击后导航到对应页面', async () => {
    // 只有装备维度有分，其余四维为 0 —— 四条死灰条必须变成四个入口
    mockApi.fetchMe.mockResolvedValue({
      user_id: 7,
      build: null,
      build_rating: {
        total: 1, element_coverage: 0, reaction_coverage: 0,
        mastery_done: 0, equipment_synergy: 5, mechanic_depth: 0, weaknesses: [],
      },
      power: 9,
    } as any)
    mountPage()
    await flushPromises()

    // 4 条 0 分维度 → 4 个补导入口（有分的那条不出入口）
    const hints = wrapper.findAll('.dim-hint')
    expect(hints.length).toBe(4)
    expect(wrapper.text()).toContain('去背包配元素')
    expect(wrapper.text()).toContain('看反应搭配')
    expect(wrapper.text()).toContain('去点亮专精')

    // 点第 1 条 → 导航到背包
    await hints[0]!.trigger('click')
    expect(um.mock.navigateTo).toHaveBeenCalledWith({ url: '/pages/bag/bag' })
  })

  it('五维全满时不出任何补导入口', async () => {
    mockApi.fetchMe.mockResolvedValue({
      user_id: 7,
      build: null,
      build_rating: {
        total: 99, element_coverage: 5, reaction_coverage: 7,
        mastery_done: 8, equipment_synergy: 18, mechanic_depth: 8, weaknesses: [],
      },
      power: 9,
    } as any)
    mountPage()
    await flushPromises()
    expect(wrapper.findAll('.dim-hint').length).toBe(0)
    expect(wrapper.findAll('.dim-empty-hint').length).toBe(0)
  })
})

// ==================== 第七批 G61 构筑五维雷达图 ====================
describe('index.vue 构筑五维雷达图（第七批 G61）', () => {
  it('G61 渲染五边形雷达：三圈网格 + 五轴 + 实际多边形 + 五标签', async () => {
    mockApi.fetchMe.mockResolvedValue({
      user_id: 7, build: null,
      build_rating: {
        total: 50, element_coverage: 3, reaction_coverage: 4,
        mastery_done: 2, equipment_synergy: 9, mechanic_depth: 5, weaknesses: [],
      },
      power: 9,
    } as any)
    mountPage()
    await flushPromises()
    // 3 圈参考网格
    expect(wrapper.findAll('.radar-ring').length).toBe(3)
    // 5 条轴线
    expect(wrapper.findAll('.radar-axis').length).toBe(5)
    // 5 个顶点标签
    expect(wrapper.findAll('.radar-label').length).toBe(5)
    // 实际多边形存在且 points 含 5 组坐标
    const shape = wrapper.find('.radar-shape')
    expect(shape.exists()).toBe(true)
    const pts = shape.attributes('points') ?? ''
    expect(pts.split(' ').length).toBe(5)
  })

  it('G61 雷达多边形随五维数值变化（偏科形状可辨）', async () => {
    mockApi.fetchMe.mockResolvedValue({
      user_id: 7, build: null,
      build_rating: {
        total: 5, element_coverage: 5, reaction_coverage: 0,
        mastery_done: 0, equipment_synergy: 0, mechanic_depth: 0, weaknesses: [],
      },
      power: 9,
    } as any)
    mountPage()
    await flushPromises()
    const pts = (wrapper.find('.radar-shape').attributes('points') ?? '').split(' ')
    // 第 1 维（元素覆盖满 5/5）顶点应落在半径最大处 ≈ (100, 20)
    expect(pts[0]).toBe('100,20')
    // 第 2 维（反应链 0）顶点应落在中心 (100, 90)
    expect(pts[1]).toBe('100,90')
  })
})
