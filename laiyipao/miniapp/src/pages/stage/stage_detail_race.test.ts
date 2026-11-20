// @vitest-environment jsdom
// 第 69 轮（lybug-r219）：关卡敌人预览竞态回归测试。
// 用户可见症状：快速从关卡 A 切到关卡 B 时，若关卡 A 的 /levels/{id} 响应
// 晚于关卡 B 返回，陈旧的敌人波次会覆盖当前关卡的预览，导致「击杀金币（约 N 只）」
// 与 killCoin 显示成上一关的敌人数量（server 契约本身正确，是 miniapp 消费面漏守卫）。
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import Stage from './stage.vue'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { mountPage } from '../../test/page'
import { makeConfig, makeLevel } from '../../test/fixtures'

const makeDetail = (id: number, count: number) => ({
  id,
  chapter: 1,
  name: `关卡${id}`,
  cost: 6,
  waves: [
    { spawns: [{ enemy_id: 1, count, interval_ms: 500, delay_ms: 0 }] },
    { spawns: [{ enemy_id: 2, count, interval_ms: 600, delay_ms: 0 }] },
  ],
  star_targets: [100, 200, 300],
  recommend_power: 100,
  first_clear_reward: { coin: 0, gem: 0, energy: 0, keys: 0 },
  kind: 'normal',
  level_type: 'normal',
  boss_enemy_id: 0,
  environment: 'none',
  terrain_kind: 'none',
  hazard_kind: 'none',
  enemy_tags: [],
  skill_tags: [],
  element_id: 0,
  enemy_element_ids: [],
})
const detailById: Record<number, ReturnType<typeof makeDetail>> = {
  1: makeDetail(1, 4), // 旧关：8 只敌人
  2: makeDetail(2, 1), // 新关：2 只敌人
}

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false),
  onAuthFailure: vi.fn(() => () => {}),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
  diagnose: vi.fn(),
  fetchMyStars: vi.fn(),
  fetchLevel: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

describe('stage 关卡敌人预览竞态（第 69 轮）', () => {
  beforeEach(() => {
    installUniMock()
    mockApi.guestLogin.mockResolvedValue({
      access_token: 'a',
      refresh_token: 'r',
      expires_at: '',
      user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
    })
    mockApi.fetchConfigIfVersion.mockResolvedValue({
      status: 'modified',
      config: makeConfig({
        levels: [makeLevel({ id: 1, chapter: 1 }), makeLevel({ id: 2, chapter: 1 })],
      }),
      serverTimeMs: 0,
    })
    mockApi.fetchWallet.mockResolvedValue({ coin: 0, gem: 0, energy: 100, keys: 0, tokens: {} })
    mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
    mockApi.diagnose.mockResolvedValue({ stuck: false, stage: 'mid_game' } as any)
    mockApi.fetchMyStars.mockResolvedValue({ stars: {} })
  })

  it('旧关卡的慢响应返回后不得覆盖新关卡的敌人预览与击杀金币', async () => {
    let resolveOld!: (v: ReturnType<typeof makeDetail>) => void
    const oldPending = new Promise<ReturnType<typeof makeDetail>>((resolve) => { resolveOld = resolve })
    mockApi.fetchLevel.mockImplementation((id: any) => {
      if (id === 1) return oldPending as any
      return Promise.resolve(detailById[id as number]) as any
    })

    const { wrapper, store } = mountPage(Stage)
    store.setMaxStage(1) // unlockedLevel = 2，初始 selected 解析为关卡 1（旧请求）
    await triggerUniHook(wrapper.vm, 'onLoad', { level: 1 })
    await flushPromises()
    expect(mockApi.fetchLevel).toHaveBeenCalledWith(1)

    // 用户快速切换到关卡 2：新请求立即返回。
    ;(wrapper.vm as unknown as { enter: (id: number) => void }).enter(2)
    await flushPromises()
    expect(mockApi.fetchLevel).toHaveBeenCalledWith(2)
    expect(wrapper.text()).toContain('击杀金币（约 2 只）')

    // 旧关卡响应迟到 —— 不得覆盖关卡 2 的预览（8 只 / killCoin 160）。
    resolveOld(detailById[1])
    await flushPromises()
    expect(wrapper.text()).toContain('击杀金币（约 2 只）')
    expect(wrapper.text()).not.toContain('击杀金币（约 8 只）')
    expect(wrapper.text()).not.toContain('+160')
  })
})
