// @vitest-environment jsdom
/**
 * 验真页（verify.vue）组件测试。
 * 覆盖：复现信息拉取与校验、构筑/攻方属性的展示分支、
 * 本地重放的成功 / error / 异常三分支、提交验真的成功与失败、
 * 伪造哈希流程、battle_id 的非法输入守卫。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Verify from './verify.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { makeConfig } from '../../test/fixtures'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfig: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  getReplay: vi.fn(),
  verifyReplay: vi.fn(),
}))

vi.mock('@/game/replay', () => ({
  // verify.vue 以 `replay as runReplay` 导入，mock 必须导出原名 replay
  replay: vi.fn(),
  prettyHash: vi.fn((h: string) => `pretty(${h})`),
  prettyDuration: vi.fn((ms: number) => `${ms}ms`),
}))

const mockApi = vi.mocked(api, true)
const replayModule = await import('@/game/replay')
const mockReplay = vi.mocked(replayModule, true)
const runReplayMock = mockReplay.replay as unknown as ReturnType<typeof vi.fn>

// 全局 uni 由 installUniMock 安装（页面 setup 需要它存在，但本文件不直接断言其调用）

const replayInfoFixture = {
  battle_id: 9,
  level_id: 3,
  level: { wave_count: 2 },
  seed: '42',
  build: {
    skills: {
      a: { slot: 1, name: '冰霜弹', element: 'ice' },
      b: null,
      c: { slot: -1, name: '备用', element: 'fire' },
      d: { slot: 0, name: '燃烧弹', element: 'fire' },
    },
    attacker: { attack: 100, crit_permille: 50, element_coef_permille: 400 },
  },
  created_at: '2026-01-01T00:00:00Z',
  expected_hash: 'abcdef0123456789',
}

beforeEach(() => {
  installUniMock()
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfig.mockResolvedValue(makeConfig())
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 3, keys: 4 })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.getReplay.mockResolvedValue(replayInfoFixture as any)
  mockApi.verifyReplay.mockResolvedValue({
    matched: true,
    actual_hash: 'abc',
    expected_hash: 'abcdef0123456789',
  })
  // 注意：真实 runReplay 是同步函数（毫秒级重放），mock 必须用 returnValue
  runReplayMock.mockReturnValue({
    matched: true,
    computedHash: 'abc',
    stats: { kills: 5, leaked: 1, waves: 2, durationMs: 1234 },
  } as any)
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

async function fetchInfo(wrapper: any, id = '9') {
  const input = wrapper.find('input')
  ;(input.element as HTMLInputElement).value = id
  await input.trigger('input')
  await wrapper.findAll('.btn').find((b: any) => b.text() === '取复现信息')!.trigger('click')
  await flushPromises()
}

describe('verify.vue 验真页', () => {
  it('取复现信息：渲染构筑技能（按槽位排序、过滤空项与未装备）与攻方属性', async () => {
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    expect(mockApi.getReplay).toHaveBeenCalledWith(9)
    expect(wrapper.text()).toContain('复现信息')
    expect(wrapper.text()).toContain('#0 燃烧弹(fire) · #1 冰霜弹(ice)') // slot 排序
    expect(wrapper.text()).not.toContain('备用') // slot -1 被过滤
    expect(wrapper.text()).toContain('攻击 100‰ · 暴击 50‰ · 元素系数 400‰')
    expect(wrapper.text()).toContain('pretty(abcdef01') // prettyHash 包装
    // 本地重放区出现
    expect(wrapper.text()).toContain('本地重放并提交验真')
  })

  it('构筑为空 / 攻方缺失 → 回退文案（?? 与 !a 分支）', async () => {
    mockApi.getReplay.mockResolvedValue({
      ...replayInfoFixture,
      build: { skills: {}, attacker: null },
    } as any)
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    expect(wrapper.text()).toContain('无（全部未装备）')
    expect(wrapper.text()).toContain('缺失 — 无法重放')
  })

  it('非法 battle_id（空 / 0 / 负数）→ 提示且不发请求', async () => {
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    for (const bad of ['', '0', '-3']) {
      const input = wrapper.find('input')
      ;(input.element as HTMLInputElement).value = bad
      await input.trigger('input')
      await wrapper.findAll('.btn').find((b: any) => b.text() === '取复现信息')!.trigger('click')
      await flushPromises()
      expect(wrapper.text()).toContain('请输入有效的 battle_id')
    }
    expect(mockApi.getReplay).not.toHaveBeenCalled()
  })

  it('build 整体缺失 → skills ?? {} 短路 + 攻方缺失文案', async () => {
    mockApi.getReplay.mockResolvedValue({
      ...replayInfoFixture,
      build: undefined,
    } as any)
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    expect(wrapper.text()).toContain('无（全部未装备）')
    expect(wrapper.text()).toContain('缺失 — 无法重放')
  })

  it('重放与验真均不一致（matched=false）→ 红色裁定文案', async () => {
    runReplayMock.mockReturnValue({
      matched: false,
      computedHash: 'deadbeef',
      stats: { kills: 1, leaked: 9, waves: 1, durationMs: 10 },
    } as any)
    mockApi.verifyReplay.mockResolvedValue({
      matched: false,
      actual_hash: 'deadbeef',
      expected_hash: 'abcdef0123456789',
    })
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    await wrapper.findAll('.btn').find((b: any) => b.text().includes('本地重放并提交'))!.trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(wrapper.text()).toContain('不一致 — 该分数无法被复现')
    expect(wrapper.text()).toContain('验真失败 — 分数被判定为不可复现')
  })

  it('取复现信息失败 → 错误信息展示', async () => {
    mockApi.getReplay.mockRejectedValue(new Error('战斗不存在'))
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper, '404')
    expect(wrapper.text()).toContain('战斗不存在')
  })

  it('本地重放成功 → 提交哈希并显示服务端裁定（一致）', async () => {
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    await wrapper.findAll('.btn').find((b: any) => b.text().includes('本地重放并提交'))!.trigger('click')
    // doReplay 内部 setTimeout(30) 后同步重放
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(runReplayMock).toHaveBeenCalledWith(replayInfoFixture, {
      level: replayInfoFixture.level,
      enemies: expect.anything(),
      skills: expect.anything(),
    })
    expect(mockApi.verifyReplay).toHaveBeenCalledWith(9, 'abc')
    expect(wrapper.text()).toContain('一致 — 分数可复现')
    expect(wrapper.text()).toContain('击杀 5')
    expect(wrapper.text()).toContain('1234ms')
    expect(wrapper.text()).toContain('验真通过')
  })

  it('重放结果带 error → 不提交验真，显示无法重放', async () => {
    runReplayMock.mockReturnValue({ error: '快照不完整' } as any)
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    await wrapper.findAll('.btn').find((b: any) => b.text().includes('本地重放并提交'))!.trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(wrapper.text()).toContain('无法重放：快照不完整')
    expect(mockApi.verifyReplay).not.toHaveBeenCalled()
  })

  it('重放抛异常 → 提示重放异常', async () => {
    runReplayMock.mockImplementation(() => { throw new Error('引擎崩溃') })
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    await wrapper.findAll('.btn').find((b: any) => b.text().includes('本地重放并提交'))!.trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(wrapper.text()).toContain('重放异常：引擎崩溃')
  })

  it('提交验真失败 → errMsg 展示', async () => {
    mockApi.verifyReplay.mockRejectedValue(new Error('验真接口失败'))
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    await wrapper.findAll('.btn').find((b: any) => b.text().includes('本地重放并提交'))!.trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(wrapper.text()).toContain('验真接口失败')
  })

  it('伪造哈希：清空 outcome 并提交全零哈希', async () => {
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    await fetchInfo(wrapper)
    // 先制造一个 outcome，再伪造 → outcome 被清空、提交全零哈希
    await wrapper.findAll('.btn').find((b: any) => b.text().includes('本地重放并提交'))!.trigger('click')
    await new Promise((r) => setTimeout(r, 60))
    await flushPromises()
    expect(wrapper.text()).toContain('一致 — 分数可复现')

    await wrapper.findAll('.btn').find((b: any) => b.text().includes('伪造哈希'))!.trigger('click')
    await flushPromises()
    expect(mockApi.verifyReplay).toHaveBeenLastCalledWith(9, '0000000000000000')
    expect(wrapper.text()).not.toContain('一致 — 分数可复现')
  })

  it('无复现信息时直呼 doReplay / doFalsify → 守卫提示（按钮此时不在 UI 上，走绑定调用）', async () => {
    const { wrapper } = mountPage(Verify)
    await flushPromises()
    ;(wrapper.vm as any).doReplay()
    await flushPromises() // errMsg 渲染在 nextTick
    expect(wrapper.text()).toContain('请先取复现信息')
    ;(wrapper.vm as any).doFalsify()
    await flushPromises()
    expect(mockApi.verifyReplay).not.toHaveBeenCalled()
  })

  it('挂载时未登录 → 自动登录', async () => {
    mountPage(Verify)
    await flushPromises()
    expect(mockApi.guestLogin).toHaveBeenCalledTimes(1)
  })
})
