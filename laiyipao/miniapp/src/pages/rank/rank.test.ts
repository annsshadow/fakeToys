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

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfig: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  fetchLeaderboard: vi.fn(),
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
  mockApi.fetchConfig.mockResolvedValue({} as any)
  mockApi.fetchLeaderboard.mockResolvedValue({
    items: [
      { user_id: 1, nickname: '第一名', score: 100 },
      { user_id: 7, nickname: '', score: 50 },
      { user_id: 2, nickname: '第三名', score: 30 },
    ],
  })
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
})
