// @vitest-environment jsdom
/**
 * R22 H5 选卡反应协同预判：渲染侧 RED→GREEN 测试。
 * 复用 battle.test.ts 的 FakeEngine/FakeRenderer/mock 边界，
 * 验证「已装备元素 × 手牌元素 → 新增/强化反应」纯展示卡。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { triggerUniHook } from '../../test/uni-app-stub'
import { makeConfig, makeLevel } from '../../test/fixtures'
import { __resetSfxForTest } from '@/audio/sfx'

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
  startBattle: vi.fn(),
  settleBattle: vi.fn(),
  abandonBattle: vi.fn(),
  myBattleHistory: vi.fn(),
  myBattleDetail: vi.fn(),
}))
const mockApi = vi.mocked(api, true)

const TICK = 50
class FakeEngine {
  static instances: FakeEngine[] = []
  options: any
  phase = 'running'
  stepCount = 0
  winAtStep = Infinity
  endPhase: 'won' | 'lost' = 'won'
  baseHp = 800n
  baseHpMax = 1000n
  waveIndex = 0
  kills = 0
  leaked = 0
  reactionsCount = 0
  cardPicks: number[] = []
  score = 0
  elapsedMs = 0
  cfg = { level: { wave_count: 3, star_targets: [100, 200, 300] } }
  deck: any = { hand: [], discardsLeft: 2, size: 10 }
  taken: string[] = []
  discarded: string[] = []
  skipped = 0
  events: unknown[] = []
  constructor(options: any) { this.options = options; FakeEngine.instances.push(this) }
  start() {}
  step() {
    this.stepCount++
    if (this.stepCount >= 1 && this.phase === 'running') this.phase = 'card_select'
    return this.events as any[]
  }
  takeCard(id: string) { this.taken.push(id) }
  discardCard(id: string) { this.discarded.push(id) }
  skipCards() { this.skipped++ }
  settleInput() { return { kills: 0, leaked: 0, reactions: 0, replay_hash: 'h' } }
  replayHash() { return 'h' }
}
class FakeRenderer {
  static instances: FakeRenderer[] = []
  loop: ((dt: number) => void) | null = null
  constructor(_canvas: any, _engine: any) { FakeRenderer.instances.push(this) }
  setViewport() {}
  setSpeed() {}
  start(cb: (dt: number) => void) { this.loop = cb }
  stop() {}
  pause() {}
  resume() {}
  handleEvents() {}
}
vi.mock('@/game/engine', () => ({ BattleEngine: FakeEngine, TICK_MS: TICK }))
vi.mock('@/audio/battle-sfx', () => ({ playBattleEvents: vi.fn() }))
vi.mock('@/game/replay', () => ({ equippedFromSnapshot: vi.fn() }))
vi.mock('@/render/canvas', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@/render/canvas')>()
  return { ...actual, BattleRenderer: FakeRenderer }
})

import { equippedFromSnapshot } from '@/game/replay'
const equippedMock = vi.mocked(equippedFromSnapshot, true)

let um: ReturnType<typeof installUniMock>
function eq(element: string) {
  return [{ skillId: 1, name: '技能', element, kind: 'active' as const, heatCost: 10n,
    cooldownMs: 500, pierce: 0, aoeRadius: 0, baseDamage: 100n, applyElement: element,
    applyStacks: 1n, projectileSpeed: 60000, chain: 0, slot: 0, cooldownRemaining: 0 }]
}

beforeEach(() => {
  um = installUniMock()
  __resetSfxForTest()
  FakeEngine.instances.length = 0
  FakeRenderer.instances.length = 0
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a', refresh_token: 'r', expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfigIfVersion.mockResolvedValue({
    status: 'modified',
    config: makeConfig({ levels: [makeLevel({ id: 1, chapter: 1 })] }),
    serverTimeMs: 0,
  })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 30, keys: 4, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.startBattle.mockResolvedValue({
    token_id: 77, seed: '12345', level: makeLevel({ id: 1, chapter: 1 }),
    build: { skills: [{ skillId: 1, slot: 0 }], active_slots: 5 },
  } as any)
  mockApi.settleBattle.mockResolvedValue({ win: true, stars: 3, score: 100, loot: {}, clamped: false } as any)
  um.mock.createSelectorQuery.mockImplementation(() => {
    const q: any = {}
    q.in = () => q
    q.select = () => q
    q.fields = (_o: unknown, cb: (r: any) => void) => {
      cb({ node: { getContext: () => ({}) }, width: 375, height: 667 })
      return q
    }
    q.exec = () => {}
    return q
  })
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

async function open(equippedElements: string[], hand: any[]) {
  // 同一 it 内多次 open 时，实例数组会累积；必须重置，否则下一次拿到上一局的引擎/渲染器。
  FakeEngine.instances.length = 0
  FakeRenderer.instances.length = 0
  equippedMock.mockReturnValue(equippedElements.flatMap(e => eq(e)) as any)
  vi.resetModules()
  const Battle = (await import('./battle.vue')).default
  const { wrapper } = mountPage(Battle)
  triggerUniHook(wrapper.vm, 'onLoad', { level: '1' })
  await flushPromises()
  const eng = FakeEngine.instances[0]!
  eng.deck.hand = hand
  eng.winAtStep = Infinity
  FakeRenderer.instances[0]!.loop!(TICK)
  await flushPromises()
  return wrapper
}

describe('battle.vue R22 选卡反应协同预判', () => {
  it('焰+冰已装备、手牌电卡：卡面 .r22-synergy-card 标出新增过热/超导，data-r22-reaction 用 ReactionKey', async () => {
    const wrapper = await open(['fire', 'ice'], [
      { id: 'c1', name: '闪电卡', kind: 'skill', rarity: 'rare', descr: 'd', element: 'lightning' },
      { id: 'c2', name: '铁壁', kind: 'attribute', rarity: 'common', descr: 'd', element: '' },
    ])
    const cards = wrapper.findAll('.hand-card')
    expect(cards.length).toBe(2)
    const c1 = cards[0]!
    expect(c1.find('.r22-synergy-card').exists()).toBe(true)
    const tags = c1.findAll('.r22-synergy-tag')
    const keys = tags.map(t => t.attributes('data-r22-reaction'))
    expect(keys).toContain('overheat')
    expect(keys).toContain('superconduct')
    expect(c1.text()).toContain('AI · 新增【过热】等2种反应')
    expect(c1.text()).toContain('可搭配 焰·冰')
    wrapper.unmount()
  })

  it('无装备元素 → 逐字空态「当前构筑还没有元素技能，无法预判反应」；无元素卡 → 逐字空态', async () => {
    const wrapper = await open([''], [
      { id: 'c1', name: '闪电卡', kind: 'skill', rarity: 'rare', descr: 'd', element: 'lightning' },
      { id: 'c2', name: '铁壁', kind: 'attribute', rarity: 'common', descr: 'd', element: '' },
    ])
    const cards = wrapper.findAll('.hand-card')
    expect(cards[0]!.find('.r22-synergy-empty').text()).toBe('当前构筑还没有元素技能，无法预判反应')
    const wrapper2 = await open(['fire'], [
      { id: 'c1', name: '闪电卡', kind: 'skill', rarity: 'rare', descr: 'd', element: 'lightning' },
      { id: 'c2', name: '铁壁', kind: 'attribute', rarity: 'common', descr: 'd', element: '' },
    ])
    expect(wrapper2.findAll('.hand-card')[1]!.find('.r22-synergy-empty').text()).toBe('该卡无元素，无法预判反应（属性/机制卡）')
    wrapper.unmount()
    wrapper2.unmount()
  })

  it('AI四判据：.r22-ai-note 含可见AI/真实结论/逐字依据行/可执行CTA，并含诚实红线', async () => {
    const wrapper = await open(['fire', 'ice'], [
      { id: 'c1', name: '闪电卡', kind: 'skill', rarity: 'rare', descr: 'd', element: 'lightning' },
      { id: 'c2', name: '铁壁', kind: 'attribute', rarity: 'common', descr: 'd', element: '' },
    ])
    const note = wrapper.find('.r22-ai-note')
    expect(note.exists()).toBe(true)
    expect(note.text()).toMatch(/AI/)
    expect(note.text()).toContain('新增【过热】等2种反应')
    expect(note.text()).toContain('依据：已装备元素 × 手牌元素，经 REACTION_PAIRS(源自引擎 lookupReaction)派生')
    expect(note.text()).toContain('去装备/选它')
    expect(note.text()).toContain('辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准')
    wrapper.unmount()
  })
})
