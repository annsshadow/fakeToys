// @vitest-environment jsdom
/**
 * R24 H5 实时元素层 HUD：渲染侧 RED→GREEN 测试。
 * 消费引擎只读 elementLayerHud() 快照，验证敌人元素层、反应预判、空态与 AI 四判据。
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { flushPromises } from '@vue/test-utils'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { mountPage } from '../../test/page'
import { triggerUniHook } from '../../test/uni-app-stub'
import { makeConfig, makeLevel } from '../../test/fixtures'
import { __resetSfxForTest } from '@/audio/sfx'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(), fetchConfigIfVersion: vi.fn(), fetchWallet: vi.fn(), fetchMe: vi.fn(),
  fetchLoadout: vi.fn(), saveLoadout: vi.fn(), setTokenInternal: vi.fn(), hasUsableToken: vi.fn(() => false),
  onAuthFailure: vi.fn(() => () => {}), loadProfile: vi.fn(() => null), saveProfile: vi.fn(), login: vi.fn(),
  saveLastUsername: vi.fn(), startBattle: vi.fn(), settleBattle: vi.fn(), abandonBattle: vi.fn(),
  myBattleHistory: vi.fn(), myBattleDetail: vi.fn(),
}))
const mockApi = vi.mocked(api, true)

const TICK = 50
class FakeEngine {
  static instances: FakeEngine[] = []
  options: any
  phase = 'running'
  stepCount = 0
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
  snapshot: any[] = []
  events: unknown[] = []
  constructor(options: any) { this.options = options; FakeEngine.instances.push(this) }
  start() {}
  elementLayerHud() { return this.snapshot.map((r: any) => ({ ...r, stacks: { ...r.stacks } })) }
  step() { this.stepCount++; if (this.stepCount >= 1 && this.phase === 'running') this.phase = 'card_select'; return this.events as any[] }
  takeCard() {} discardCard() {} skipCards() {}
  settleInput() { return { kills: 0, leaked: 0, reactions: 0, replay_hash: 'h' } }
  replayHash() { return 'h' }
}
class FakeRenderer {
  static instances: FakeRenderer[] = []
  loop: ((dt: number) => void) | null = null
  constructor(_canvas: any, _engine: any) { FakeRenderer.instances.push(this) }
  setViewport() {} setSpeed() {} stop() {} pause() {} resume() {} handleEvents() {}
  start(cb: (dt: number) => void) { this.loop = cb }
}
vi.mock('@/game/engine', () => ({ BattleEngine: FakeEngine, TICK_MS: TICK }))
vi.mock('@/audio/battle-sfx', () => ({ playBattleEvents: vi.fn() }))
vi.mock('@/game/replay', () => ({
  equippedFromSnapshot: vi.fn(() => [{
    skillId: 1, name: '技能', element: 'fire', kind: 'active', heatCost: 10n,
    cooldownMs: 500, pierce: 0, aoeRadius: 0, baseDamage: 100n, applyElement: 'fire',
    applyStacks: 1n, projectileSpeed: 60000, chain: 0, slot: 0, cooldownRemaining: 0,
  }]),
}))
vi.mock('@/render/canvas', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@/render/canvas')>()
  return { ...actual, BattleRenderer: FakeRenderer }
})

let um: ReturnType<typeof installUniMock>
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
    status: 'modified', config: makeConfig({ levels: [makeLevel({ id: 1, chapter: 1 })] }), serverTimeMs: 0,
  })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 30, keys: 4, tokens: {} })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.fetchLoadout.mockResolvedValue({ skills: [{ skillId: 1, slot: 0 }], active_slots: 5 } as any)
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
afterEach(() => { vi.unstubAllGlobals(); vi.clearAllMocks() })

async function open(snapshot: any[]) {
  vi.resetModules()
  const Battle = (await import('./battle.vue')).default
  const { wrapper } = mountPage(Battle)
  triggerUniHook(wrapper.vm, 'onLoad', { level: '1' })
  await flushPromises()
  const eng = FakeEngine.instances[0]!
  eng.snapshot = snapshot
  FakeRenderer.instances[0]!.loop!(TICK)
  await flushPromises()
  return { wrapper, eng }
}

describe('battle.vue R24 实时元素层 HUD', () => {
  it('敌人行带 data-r24-uid；焰层敌人预判补冰=蒸汽爆发、补电=过热且不含超导', async () => {
    const { wrapper } = await open([
      { uid: 101, defId: 1, name: '游荡者', hp: 80, maxHp: 100, stacks: { fire: 2 }, elementCap: 3 },
    ])
    const hud = wrapper.find('.r24-element-hud')
    expect(hud.exists()).toBe(true)
    const row = wrapper.find('.r24-enemy-row[data-r24-uid="101"]')
    expect(row.exists()).toBe(true)
    expect(row.text()).toContain('游荡者')
    expect(row.text()).toContain('焰')
    const hints = wrapper.findAll('.r24-reaction-hint')
    const text = hints.map(h => h.text()).join(' | ')
    expect(text).toContain('已附着 焰')
    expect(text).toContain('再补 冰')
    expect(text).toContain('蒸汽爆发')
    expect(text).toContain('再补 电')
    expect(text).toContain('过热')
    expect(text).not.toContain('超导')
    wrapper.unmount()
  })

  it('无敌人或无可预判反应时逐字空态「当前场上无可预判的元素反应」', async () => {
    const { wrapper } = await open([])
    expect(wrapper.find('.r24-hud-empty').text()).toBe('当前场上无可预判的元素反应')
    wrapper.unmount()
    const { wrapper: w2 } = await open([
      { uid: 202, defId: 2, name: '全元素怪', hp: 50, maxHp: 100,
        stacks: { fire: 1, ice: 1, lightning: 1, corrosion: 1, kinetic: 1 }, elementCap: 5 },
    ])
    expect(w2.find('.r24-hud-empty').text()).toBe('当前场上无可预判的元素反应')
    expect(w2.findAll('.r24-reaction-hint').length).toBe(0)
    w2.unmount()
  })

  it('AI 四判据 + 诚实红线：AI 标签/真实结论/逐字依据/CTA/红线齐备', async () => {
    const { wrapper } = await open([
      { uid: 303, defId: 3, name: '游荡者', hp: 80, maxHp: 100, stacks: { ice: 1 }, elementCap: 3 },
    ])
    const note = wrapper.find('.r24-ai-note')
    expect(note.exists()).toBe(true)
    expect(note.text()).toMatch(/AI/)
    expect(note.text()).toContain('再补 电')
    expect(note.text()).toContain('超导')
    expect(note.text()).toContain('依据：敌人 stacks 元素层 × REACTION_PAIRS(源自引擎 lookupReaction) 派生；只读快照不进 replay_hash')
    expect(note.text()).toContain('去释放对应元素技能')
    expect(note.text()).toContain('辅助预判非伤害承诺，实际触发取决于释放顺序/命中/层数，权威以 replay_hash 为准')
    wrapper.unmount()
  })

  it('tick 后刷新响应式：快照变化在下一次 step 后反映到 HUD（含 fallbackTick 路径）', async () => {
    const { wrapper, eng } = await open([])
    expect(wrapper.find('.r24-hud-empty').exists()).toBe(true)
    eng.snapshot = [{ uid: 404, defId: 4, name: '游荡者', hp: 40, maxHp: 100, stacks: { fire: 1 }, elementCap: 3 }]
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()
    expect(wrapper.find('.r24-enemy-row[data-r24-uid="404"]').exists()).toBe(true)
    const source = readFileSync(join(__dirname, './battle.vue'), 'utf-8')
    const fallback = source.slice(source.indexOf('function fallbackTick'), source.indexOf('/** 按当前倍速'))
    expect(fallback).toContain('refreshElementHud(eng)')
    wrapper.unmount()
  })
})
