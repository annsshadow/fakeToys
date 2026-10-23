// @vitest-environment jsdom
/**
 * 排行榜页（rank.vue）组件测试。
 * 覆盖：三个榜单 tab 的描述与切换、加载三态（加载中 / 空 / 列表）、
 * 加载失败 toast、榜单条目的名次与"是我"高亮、挂载时自动登录后拉榜。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Rank from './rank.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { triggerUniHook } from '../../test/uni-app-stub'

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
  fetchLeaderboard: vi.fn(),
  // 我的排名（2026-10-23）：与榜单并发拉取，缺 mock 会让 load() 直接抛。
  fetchMyRank: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

let um: ReturnType<typeof installUniMock>

beforeEach(() => {
  um = installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: {} as any, serverTimeMs: 0 })
  mockApi.fetchLeaderboard.mockResolvedValue({
    items: [
      { user_id: 1, nickname: '第一名', score: 100 },
      { user_id: 7, nickname: '', score: 50 },
      { user_id: 2, nickname: '第三名', score: 30 },
    ],
  })
  // 缺省：有成绩、居中
  mockApi.fetchMyRank.mockResolvedValue({ kind: 'power', rank: 2, total: 3, score: 50 })
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

describe('rank.vue 排行榜', () => {
  it('挂载自动登录并加载默认（power）榜单，渲染名次与"是我"高亮', async () => {
    const { wrapper, store } = mountPage(Rank)
    await flushPromises()
    expect(mockApi.fetchLeaderboard).toHaveBeenCalledWith('power')
    expect(wrapper.text()).toContain('按累计成长值排名。')
    const rows = wrapper.findAll('.rank-row')
    expect(rows.length).toBe(3)
    // 空昵称回退为"玩家"
    expect(wrapper.text()).toContain('玩家')
    // user_id 与当前用户一致 → 高亮
    expect(rows[1]!.classes()).toContain('me')
    expect(store.loggedIn).toBe(true)
  })

  it('切换三个 tab：描述文案与请求参数随之变化（switch 分支全覆盖）', async () => {
    const { wrapper } = mountPage(Rank)
    await flushPromises()
    const tabs = wrapper.findAll('.tab')

    await tabs[1]!.trigger('click')
    expect(mockApi.fetchLeaderboard).toHaveBeenLastCalledWith('stage')
    expect(wrapper.text()).toContain('按最高通关关卡排名。')

    await tabs[2]!.trigger('click')
    expect(mockApi.fetchLeaderboard).toHaveBeenLastCalledWith('efficiency')
    expect(wrapper.text()).toContain('低养成高技巧')

    await tabs[0]!.trigger('click')
    expect(mockApi.fetchLeaderboard).toHaveBeenLastCalledWith('power')
    expect(wrapper.text()).toContain('按累计成长值排名。')
  })

  it('第 136 轮：陈旧响应不覆盖更新的 tab（快速切 tab 的竞态）', async () => {
    // 手动控制两个在途请求的完成顺序：让「更早发起」的 stage 请求
    // 比「更晚发起」的 efficiency 请求后到。修前 items 会被 stage 覆盖，
    // 修后（seq 守卫）stage 响应被丢弃，列表保持 efficiency。
    let resolveStage: (v: any) => void = () => {}
    let resolveEff: (v: any) => void = () => {}
    const stagePending = new Promise((r) => (resolveStage = r))
    const effPending = new Promise((r) => (resolveEff = r))
    mockApi.fetchLeaderboard.mockImplementation((type?: string) => {
      if (type === 'stage') return stagePending as unknown as Promise<any>
      if (type === 'efficiency') return effPending as unknown as Promise<any>
      return Promise.resolve({ items: [] })
    })

    const { wrapper } = mountPage(Rank)
    await flushPromises() // 挂载默认 power（已 resolve {items:[]}）
    const tabs = wrapper.findAll('.tab')
    await tabs[1]!.trigger('click') // 发起 stage（seq 2，在途）
    await tabs[2]!.trigger('click') // 发起 efficiency（seq 3，在途）

    // 先让「新」的 efficiency 完成，再让「旧」的 stage 后到
    resolveEff({ items: [{ user_id: 100, nickname: 'FRESH_EFF', score: 1 }] })
    await flushPromises()
    expect(wrapper.text()).toContain('FRESH_EFF')

    resolveStage({ items: [{ user_id: 200, nickname: 'STALE_STAGE', score: 2 }] })
    await flushPromises()
    // 陈旧 stage 响应被丢弃：列表仍是 efficiency 的 FRESH_EFF，而非 STALE_STAGE
    expect(wrapper.text()).toContain('FRESH_EFF')
    expect(wrapper.text()).not.toContain('STALE_STAGE')
  })

  it('加载中 → 显示加载文案', async () => {
    mockApi.fetchLeaderboard.mockImplementation(() => new Promise(() => {}))
    const { wrapper } = mountPage(Rank)
    await flushPromises()
    expect(wrapper.text()).toContain('加载中…')
  })

  it('榜单为空 → 显示"暂无数据"（items 缺失时 ?? [] 回退）', async () => {
    mockApi.fetchLeaderboard.mockResolvedValue({} as any)
    const { wrapper } = mountPage(Rank)
    await flushPromises()
    expect(wrapper.text()).toContain('暂无数据')
  })

  it('加载失败 → toast 提示并清空列表', async () => {
    mockApi.fetchLeaderboard.mockRejectedValue(new Error('网络错误'))
    const { wrapper } = mountPage(Rank)
    await flushPromises()
    expect(um.mock.showToast).toHaveBeenCalledWith({ title: '网络错误', icon: 'none' })
    expect(wrapper.text()).toContain('暂无数据')
  })

  // ── 我的排名（2026-10-23）──────────────────────────────────────────
  // 补的是「榜单看得见但用不上」：只回前 50 条，未进前 50 的人找不到自己。

  it('有成绩 → 显示「第 N / 共 M 人」与我的分数', async () => {
    mockApi.fetchMyRank.mockResolvedValue({ kind: 'power', rank: 12, total: 340, score: 9876 })
    const { wrapper } = mountPage(Rank)
    await flushPromises()
    expect(wrapper.text()).toContain('我的排名')
    expect(wrapper.text()).toContain('第 12 / 共 340 人')
    expect(wrapper.text()).toContain('9876')
  })

  it('无成绩（rank=0）→ 显示「暂无成绩」，绝不显示「第 0 名」', async () => {
    mockApi.fetchMyRank.mockResolvedValue({ kind: 'stage', rank: 0, total: 0, score: 0 })
    const { wrapper } = mountPage(Rank)
    await flushPromises()
    expect(wrapper.text()).toContain('暂无成绩')
    expect(wrapper.text()).not.toContain('第 0')
    // 无成绩时给出「怎么才有成绩」的提示，而不是一句话打发。
    // ⚠️ 提示按**当前 tab** 给（战力榜 → 积累成长值），不按服务端回的 kind ——
    // 服务端 kind 只是回显，玩家看的是他正在看的那个榜。
    expect(wrapper.text()).toContain('积累成长值')
  })

  it('我的排名失败 → 榜单照常显示（次要信息不拖垮主内容）', async () => {
    mockApi.fetchMyRank.mockRejectedValue(new Error('boom'))
    const { wrapper } = mountPage(Rank)
    await flushPromises()
    expect(wrapper.findAll('.rank-row').length).toBe(3)
    expect(wrapper.text()).not.toContain('我的排名')
    expect(wrapper.text()).not.toContain('第 0')
  })

  it('切 tab 时先清掉旧榜名次（不显示上一个榜的数字）', async () => {
    mockApi.fetchMyRank.mockResolvedValue({ kind: 'power', rank: 5, total: 9, score: 100 })
    const { wrapper } = mountPage(Rank)
    await flushPromises()
    expect(wrapper.text()).toContain('第 5 / 共 9 人')

    // 切到 stage，让新请求挂起 —— 期间必须看不到 power 的第 5 名
    mockApi.fetchLeaderboard.mockImplementation(() => new Promise(() => {}))
    mockApi.fetchMyRank.mockImplementation(() => new Promise(() => {}))
    await wrapper.findAll('.tab')[1]!.trigger('click')
    await flushPromises()
    expect(wrapper.text()).not.toContain('第 5 / 共 9 人')
  })

  it('深链 ?type=stage → 直接选中关卡榜', async () => {
    const { wrapper } = mountPage(Rank)
    // uni-app 的真实顺序是 onLoad → onShow → mounted。替身里 onMounted 的
    // 异步体在第一个 await 处让出，所以在 flush 之前触发 onLoad 与真实顺序等价。
    triggerUniHook(wrapper.vm, 'onLoad', { type: 'stage' })
    await flushPromises()
    // ⚠️ 必须同时断言**次数**：只断言 `toHaveBeenCalledWith('stage')` 时，
    // 「onMounted 先按 power 拉一次、onLoad 再补一次 stage」这种实现照样绿，
    // 而那正是「首次用错类型 + 多发一次请求」的形态。
    expect(mockApi.fetchLeaderboard).toHaveBeenCalledTimes(1)
    expect(mockApi.fetchLeaderboard).toHaveBeenCalledWith('stage')
    expect(wrapper.text()).toContain('按最高通关关卡排名。')
  })

  it('深链 ?type=不存在 → 回落到默认榜（分享链接可能来自旧版本）', async () => {
    const { wrapper } = mountPage(Rank)
    triggerUniHook(wrapper.vm, 'onLoad', { type: 'nope' })
    await flushPromises()
    expect(mockApi.fetchLeaderboard).toHaveBeenCalledTimes(1)
    expect(mockApi.fetchLeaderboard).toHaveBeenCalledWith('power')
  })

  // ── 分享（2026-10-23）────────────────────────────────────────────
  it('分享 → 标题带当前榜与名次，路径带 type', async () => {
    mockApi.fetchMyRank.mockResolvedValue({ kind: 'power', rank: 12, total: 340, score: 9876 })
    const { wrapper } = mountPage(Rank)
    await flushPromises()
    const payload = captureShare(wrapper)
    expect(payload.title).toBe('我在战力榜排第 12')
    expect(payload.path).toBe('/pages/rank/rank?type=power')
  })

  it('分享（无名次）→ 不出现「第 0」', async () => {
    mockApi.fetchMyRank.mockResolvedValue({ kind: 'power', rank: 0, total: 0, score: 0 })
    const { wrapper } = mountPage(Rank)
    await flushPromises()
    const payload = captureShare(wrapper)
    expect(payload.title).not.toContain('第 0')
    expect(payload.title).toContain('战力榜')
  })
})

/**
 * 触发页面注册的 onShareAppMessage，返回它给出的 payload。
 *
 * 这是**行为**断言：微信真正用的就是回调的返回值。
 * 只断言「注册了回调」是不够的 —— 卡片内容错了只有分享出去的人看得见。
 */
function captureShare(wrapper: any): { title: string; path: string } {
  const out = triggerUniHook(wrapper.vm, 'onShareAppMessage', { from: 'menu' })
  if (!out || typeof out !== 'object') {
    throw new Error('onShareAppMessage 没有返回分享对象（微信会拿不到卡片内容）')
  }
  return out as { title: string; path: string }
}
