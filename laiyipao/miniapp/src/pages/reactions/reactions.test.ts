// @vitest-environment jsdom
/**
 * pages/reactions/reactions.vue 测试（P4-2 / P4-3）。
 *
 * 这两条修的是同一个根因：**能力存在但没有入口**。
 *
 *  - `/config.reactions`：服务端把 7 条反应的完整参数都下发下来了，
 *    客户端**一处都没读** → 玩家查不到「冰+雷=什么反应、伤害多少」。
 *  - `reachableReactions(elements)`：引擎里现成的函数，
 *    只有测试在调 → 玩家看不到「我这个构筑能打出哪些反应链」。
 */
import { describe, it, expect, vi, beforeEach } from 'vitest'
import { mount, flushPromises } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import Reactions from './reactions.vue'
import { useGameStore } from '@/store/game'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { makeConfig } from '../../test/fixtures'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfigIfVersion: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  hasUsableToken: vi.fn(() => false),
  loadProfile: vi.fn(() => null),
  saveProfile: vi.fn(),
  login: vi.fn(),
  saveLastUsername: vi.fn(),
}))

const mockApi = vi.mocked(api, true)

let um: ReturnType<typeof installUniMock>

/** 反应表：与服务端 `domain.ReactionSpec` 同形状（少一个字段就会红）。 */
function reactionFixtures() {
  return [
    {
      key: 'steam_burst',
      name: '蒸汽爆发',
      base_coef: 60,
      attack_weight_pct: 300,
      status_duration_ms: 0,
      aoe_radius: 120,
      spread_stacks: 0,
      dispel_shield: true,
      amplify_pct: 0,
      armor_shred_permille: 0,
      knockback: 0,
      descr: '驱散护盾并造成溅射',
    },
    {
      key: 'overheat',
      name: '过热',
      base_coef: 55,
      attack_weight_pct: 300,
      status_duration_ms: 1500,
      aoe_radius: 90,
      spread_stacks: 0,
      dispel_shield: false,
      amplify_pct: 0,
      armor_shred_permille: 0,
      knockback: 0,
      descr: '眩晕并造成溅射',
    },
  ]
}

async function mountReactions(elements: 'fire_ice' | 'fire_only' = 'fire_ice') {
  const pinia = createPinia()
  setActivePinia(pinia)
  const cfg = makeConfig({
    reactions: reactionFixtures() as never,
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified',
    config: cfg,
    serverTimeMs: 0,
  })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 1, energy: 1, keys: 1, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: {}, build_rating: {}, power: 0, max_stage: 1 })
  // 出战槽位：技能 1 = fire，技能 2 = ice（见 fixtures.makeSkill）
  // ⚠️ 字段名是 `skill_ids`（服务端 /me/loadout 的契约），不是 `loadout`。
  mockApi.fetchLoadout.mockResolvedValue({
    skill_ids: elements === 'fire_ice' ? [1, 2] : [1],
  })
  const wrapper = mount(Reactions, { global: { plugins: [pinia] } })
  await flushPromises()
  // store 是异步加载配置的，等它到位
  const store = useGameStore()
  // 配置（反应表）在 config 里 → 必须 loadConfig；
  // 出战槽位（元素）在 loadout 里 → 必须 loadLoadout。
  // 两者都不是 refreshProfile 干的，所以这里显式调。
  await store.loadConfig()
  await store.refreshProfile()
  await store.loadLoadout()
  await flushPromises()
  return wrapper
}

beforeEach(() => {
  um = installUniMock()
  vi.clearAllMocks()
  mockApi.hasUsableToken.mockReturnValue(false)
  mockApi.loadProfile.mockReturnValue(null)
})

describe('reactions.vue 反应图鉴', () => {
  it('展示服务端下发的全部反应与关键参数（P4-2）', async () => {
    const wrapper = await mountReactions()
    const text = wrapper.text()
    expect(text).toContain('蒸汽爆发')
    expect(text).toContain('过热')
    // 参数（这些此前客户端一处都没读）
    expect(text).toContain('系数 60')
    expect(text).toContain('攻击力权重 300%')
    expect(text).toContain('溅射半径 120')
    expect(text).toContain('持续 1.5s')
    expect(text).toContain('驱散护盾')
    // 服务端的说明文案
    expect(text).toContain('驱散护盾并造成溅射')
    wrapper.unmount()
  })

  it('标出「我的构筑可达」的反应（P4-3：reachableReactions 此前零 UI 消费）', async () => {
    const wrapper = await mountReactions('fire_ice')
    // 引擎的 reachableReactions(fire, ice) 命中 steam_burst
    expect(wrapper.findAll('.rxn-hit').length).toBe(1)
    expect(wrapper.text()).toContain('可达')
    wrapper.unmount()
  })

  it('构筑元素凑不出反应时给出引导而不是空白', async () => {
    const wrapper = await mountReactions('fire_only')
    expect(wrapper.findAll('.rxn-hit').length).toBe(0)
    expect(wrapper.text()).toContain('凑不出任何反应链')
    wrapper.unmount()
  })
})
