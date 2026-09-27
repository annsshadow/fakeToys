// @vitest-environment jsdom
/**
 * 战斗页（battle.vue）组件测试 —— 只测页面自身逻辑。
 *
 * mock 边界：
 *  - @/game/engine → FakeEngine（战斗内核由 game/ 的专项测试覆盖）
 *  - @/game/replay → equippedFromSnapshot（构造拼装已由 replay 侧测试覆盖）
 *  - @/render/canvas → FakeRenderer（渲染层由 canvas.test.ts 覆盖）
 *
 * 页面自身要验证的关键行为：
 *  - 必须等登录完成再申请开局凭证（否则可能带上一个账号的 token）
 *  - 技能槽位以服务端 build 为准（equippedFromSnapshot，绝不本地拼）
 *  - canvas 获取：H5 的 <uni-canvas> 没有 getContext，必须向下找真实 canvas
 *  - 拿不到 canvas → 定时器降级，战斗逻辑照常推进
 *  - 结算的幂等（settled 单飞）与失败兜底
 */
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import { flushPromises } from '@vue/test-utils'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { mountPage } from '../../test/page'
import { makeConfig, makeLevel } from '../../test/fixtures'

vi.mock('@/api/client', () => ({
  guestLogin: vi.fn(),
  fetchConfig: vi.fn(),
  fetchWallet: vi.fn(),
  fetchMe: vi.fn(),
  fetchLoadout: vi.fn(),
  saveLoadout: vi.fn(),
  setTokenInternal: vi.fn(),
  startBattle: vi.fn(),
  settleBattle: vi.fn(),
}))

const TICK = 50

class FakeEngine {
  static instances: FakeEngine[] = []
  options: any
  phase = 'running'
  stepCount = 0
  /** 第 N 次 step 进入选牌（模拟真实引擎波次结束发牌） */
  cardAtStep = 1
  /** 第 N 次 step 转 won（驱动结算流程）；Infinity 表示不结束 */
  winAtStep = 2
  /** 终局相位：默认 won，可切 lost 驱动"防线失守"展示分支 */
  endPhase: 'won' | 'lost' = 'won'
  baseHp = 800n
  baseHpMax = 1000n
  waveIndex = 0
  kills = 1
  leaked = 2
  reactionsCount = 3
  score = 100
  cfg = { level: { wave_count: 3 } }
  deck = {
    hand: [
      { id: 'c1', name: '燃烧弹', kind: 'skill', rarity: 'rare', descr: 'd1', element: 'fire' },
      { id: 'c2', name: '铁壁', kind: 'attribute', rarity: 'common', descr: 'd2', element: '' },
      { id: 'c3', name: '毒雾', kind: 'mechanic', rarity: 'epic', descr: 'd3', element: 'ice' },
    ],
    discardsLeft: 2,
  }
  taken: string[] = []
  discarded: string[] = []
  skipped = 0
  settleToken = 0

  constructor(options: any) {
    this.options = options
    FakeEngine.instances.push(this)
  }
  start() {}
  step() {
    this.stepCount++
    // 与真实引擎一致：phase 在 step() 内部以原始对象身份被改写
    if (this.stepCount >= this.cardAtStep && this.phase === 'running') {
      this.phase = 'card_select'
    }
    if (this.stepCount >= this.winAtStep) this.phase = this.endPhase
    return []
  }
  takeCard(id: string) {
    this.taken.push(id)
  }
  discardCard(id: string) {
    this.discarded.push(id)
  }
  skipCards() {
    this.skipped++
  }
  settleInput(tokenId: number) {
    this.settleToken = tokenId
    return { kills: 1, leaked: 2, reactions: 3, replay_hash: 'replayhash' }
  }
  replayHash() {
    return 'fallbackhash'
  }
}

class FakeRenderer {
  static instances: FakeRenderer[] = []
  canvas: any
  engine: any
  loop: ((dt: number) => void) | null = null
  viewport: [number, number] | null = null
  started = false
  stopped = false
  constructor(canvas: any, engine: any) {
    this.canvas = canvas
    this.engine = engine
    FakeRenderer.instances.push(this)
  }
  setViewport(w: number, h: number) {
    this.viewport = [w, h]
  }
  start(cb: (dt: number) => void) {
    this.started = true
    this.loop = cb
  }
  stop() {
    this.stopped = true
  }
  handleEvents(_events: unknown[]) {}
}

vi.mock('@/game/engine', () => ({ BattleEngine: FakeEngine, TICK_MS: TICK }))
vi.mock('@/game/replay', () => ({ equippedFromSnapshot: vi.fn() }))
vi.mock('@/render/canvas', () => ({ BattleRenderer: FakeRenderer }))

const mockApi = vi.mocked(api, true)
const replayModule = await import('@/game/replay')
const equippedFromSnapshotMock = vi.mocked(replayModule.equippedFromSnapshot)

let um: ReturnType<typeof installUniMock>

const EQUIPPED = [
  {
    skillId: 1, name: '燃烧弹', element: 'fire', kind: 'active' as const,
    heatCost: 20n, cooldownMs: 800, pierce: 0, aoeRadius: 60, baseDamage: 100n,
    applyElement: 'fire', applyStacks: 1n, projectileSpeed: 60000, chain: 0, slot: 0,
    cooldownRemaining: 0,
  },
]

const startBattleResp = {
  token_id: 77,
  seed: '12345',
  level: makeLevel(),
  build: { skills: [{ skillId: 1, slot: 0 }], active_slots: 5 },
}

/** fields 回调收到的 node（页面会就地设置 width/height，供 dpr 断言） */
let lastNode: any = null

/** createSelectorQuery 链式 mock：fields 回调给 node（或 null） */
function mockQueryNode(node: any) {
  lastNode = node
  um.mock.createSelectorQuery.mockImplementation(() => {
    const q: any = {}
    q.in = () => q
    q.select = () => q
    q.fields = (_opts: unknown, cb: (res: any) => void) => {
      cb(node ? { node, width: 375, height: 667 } : null)
      return q
    }
    q.exec = () => {}
    return q
  })
}

/** 每个用例动态导入页面：battle.vue 的 settled 是模块级状态，必须隔离 */
async function loadPage() {
  vi.resetModules()
  return (await import('./battle.vue')).default
}

beforeEach(() => {
  um = installUniMock()
  FakeEngine.instances.length = 0
  FakeRenderer.instances.length = 0
  mockApi.guestLogin.mockResolvedValue({
    access_token: 'a',
    refresh_token: 'r',
    expires_at: '',
    user: { id: 7, nickname: '我', avatar_url: '', is_guest: true, status: 1 },
  })
  mockApi.fetchConfig.mockResolvedValue(makeConfig())
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 30, keys: 4 })
  mockApi.fetchMe.mockResolvedValue({ user_id: 7, build: null, build_rating: null, power: 9 } as any)
  mockApi.startBattle.mockResolvedValue(startBattleResp as any)
  mockApi.settleBattle.mockResolvedValue({
    win: true,
    stars: 3,
    score: 555,
    loot: { coin: 10, gem: 20, keys: 2, energy: 3, other: 9 },
    clamped: true,
    clamp_note: '服务端已裁剪',
    new_max_stage: 4,
  } as any)
  equippedFromSnapshotMock.mockReturnValue(EQUIPPED as any)
  mockQueryNode({ getContext: () => ({}) })
})

afterEach(() => {
  vi.unstubAllGlobals()
  vi.clearAllMocks()
})

async function setupDone(wrapper: any) {
  triggerUniHook(wrapper.vm, 'onLoad', { level: '3' })
  await flushPromises()
}

describe('battle.vue 战斗页', () => {
  it('开局成功：先登录再申请凭证，引擎参数全部来自服务端，渲染器走帧循环', async () => {
    const Battle = await loadPage()
    const { wrapper, store } = mountPage(Battle)
    await setupDone(wrapper)

    expect(mockApi.guestLogin).toHaveBeenCalled()
    expect(mockApi.startBattle).toHaveBeenCalledWith(3)
    // 槽位以服务端 build 为准（equippedFromSnapshot，绝不本地拼）
    expect(equippedFromSnapshotMock).toHaveBeenCalledWith(startBattleResp.build, store.skillMap)
    expect(store.equippedSkillIds).toEqual([1])

    const eng = FakeEngine.instances[0]!
    expect(eng.options.seed).toBe(12345n) // 字符串 seed → BigInt
    expect(eng.options.level).toBe(startBattleResp.level)
    expect(eng.options.activeSlots).toBe(5)
    expect(eng.options.attacker).toBeDefined()

    const renderer = FakeRenderer.instances[0]!
    expect(renderer.started).toBe(true)
    expect(renderer.viewport).toEqual([375, 667]) // 视口用 CSS 像素
    // canvas 按 dpr 放大（pixelRatio 2 × 375）
    expect(lastNode.width).toBe(750)
    expect(lastNode.height).toBe(1334)
    expect(wrapper.text()).toContain('渲染: 帧循环')
    expect(wrapper.text()).toContain('波次 1 / 3')
    expect(wrapper.text()).toContain('800 / 1000')
  })

  it('H5 的 uni-canvas 包装元素：无 getContext 时向下找真实 canvas', async () => {
    const inner = { getContext: () => ({}) }
    mockQueryNode({ querySelector: (sel: string) => (sel === 'canvas' ? inner : null) })
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    expect(FakeRenderer.instances.length).toBe(1)
    expect(wrapper.text()).toContain('渲染: 帧循环')
  })

  it('未装备任何技能 → 提示去背包配置，不建引擎', async () => {
    equippedFromSnapshotMock.mockReturnValue([] as any)
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    expect(wrapper.text()).toContain('未装备任何技能')
    expect(FakeEngine.instances.length).toBe(0)
    // 引擎未建立 → HUD 占位文案（engine 为 null 的计算属性分支）
    expect(wrapper.text()).toContain('波次 —')
    expect(wrapper.text()).toContain('—')
    expect(wrapper.text()).toContain('击杀 0')
  })

  it('startBattle 失败 → startError 展示；登录失败同理', async () => {
    mockApi.startBattle.mockRejectedValue(new Error('体力不足'))
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    expect(wrapper.text()).toContain('无法开始战斗')
    expect(wrapper.text()).toContain('体力不足')

    // 登录失败：startError 取 loginError
    mockApi.startBattle.mockClear()
    mockApi.guestLogin.mockRejectedValue(new Error('登录失败'))
    const Battle2 = await loadPage()
    const { wrapper: w2 } = mountPage(Battle2)
    triggerUniHook(w2.vm, 'onLoad', { level: '3' })
    await flushPromises()
    expect(w2.text()).toContain('登录失败')
  })

  it('拿不到 canvas → 定时器降级推进，won 后自动结算并清理', async () => {
    mockQueryNode(null)
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    expect(wrapper.text()).toContain('渲染: 降级(无canvas)')

    // 定时器推进 → 引擎 won → settle
    await new Promise((r) => setTimeout(r, TICK * 5))
    await flushPromises()
    expect(mockApi.settleBattle).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain('通关')
    expect(FakeRenderer.instances.length).toBe(0)

    // onUnload：停渲染器、清定时器（不再继续 step）
    const eng = FakeEngine.instances[0]!
    triggerUniHook(wrapper.vm, 'onUnload')
    const steps = eng.stepCount
    await new Promise((r) => setTimeout(r, TICK * 3))
    expect(eng.stepCount).toBe(steps)
  })

  it('帧循环驱动 won → 结算成功：结果卡、奖励、new_max_stage、停帧', async () => {
    const Battle = await loadPage()
    const { wrapper, store } = mountPage(Battle)
    await setupDone(wrapper)

    const eng = FakeEngine.instances[0]!
    const renderer = FakeRenderer.instances[0]!
    renderer.loop!(TICK)
    renderer.loop!(TICK) // 第 2 步 won → settle
    await flushPromises()

    expect(eng.settleToken).toBe(77) // tokenId 传给结算
    expect(mockApi.settleBattle).toHaveBeenCalledTimes(1)
    expect(renderer.stopped).toBe(true)
    expect(store.maxStage).toBe(4) // new_max_stage
    expect(wrapper.text()).toContain('通关')
    expect(wrapper.text()).toContain('服务端已裁剪')
    expect(wrapper.text()).toContain('金币 +10')
    expect(wrapper.text()).toContain('钻石 +20')
    expect(wrapper.text()).toContain('钥匙 +2')
    expect(wrapper.text()).toContain('体力 +3')
    expect(wrapper.text()).toContain('other +9') // 未知货币键 → 原文
    expect(wrapper.text()).toContain('replayhash')

    // settled 单飞：再次 won 不重复结算
    renderer.loop!(TICK)
    await flushPromises()
    expect(mockApi.settleBattle).toHaveBeenCalledTimes(1)

    // 返回关卡按钮
    await wrapper.findAll('.btn').find((b: any) => b.text().includes('返回关卡'))!.trigger('click')
    expect(um.mock.navigateBack).toHaveBeenCalledTimes(1)
  })

  it('结算接口失败 + 战斗失败 → 兜底战报（防线失守 + 错误信息 + 本地哈希）', async () => {
    mockApi.settleBattle.mockRejectedValue(new Error('结算失败'))
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    FakeEngine.instances[0]!.endPhase = 'lost' // 防线被攻破
    FakeRenderer.instances[0]!.loop!(TICK)
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()
    expect(wrapper.text()).toContain('防线失守') // win = eng.phase === 'won' → false
    expect(wrapper.text()).toContain('结算失败')
    expect(wrapper.text()).toContain('fallbackhash')
  })

  it('选牌界面：phase=card_select 时渲染手牌，取卡 / 弃卡 / 跳过', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    eng.winAtStep = Infinity // 不让本局打到终局
    // phase 由 step() 内部改写（与真实引擎一致）—— 此前引擎的原始对象写入
    // 不触发响应式，选牌界面永远出不来（triggerRef 修复后本用例才可能通过）
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()
    expect(wrapper.text()).toContain('选择一张卡')
    // kindLabel 三种分支
    expect(wrapper.text()).toContain('技能')
    expect(wrapper.text()).toContain('属性')
    expect(wrapper.text()).toContain('机制')

    const cards = wrapper.findAll('.hand-card')
    expect(cards.length).toBe(3)
    await cards[0]!.trigger('click')
    expect(eng.taken).toEqual(['c1'])
    await cards[1]!.find('.dim').trigger('click') // 弃
    expect(eng.discarded).toEqual(['c2'])

    await wrapper.findAll('.btn').find((b: any) => b.text().includes('跳过'))!.trigger('click')
    expect(eng.skipped).toBe(1)
  })

  it('节点既无 getContext 也无 querySelector → 视为拿不到 canvas，回退降级', async () => {
    mockQueryNode({}) // 既不是 canvas，也不是 uni-canvas 包装
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    expect(wrapper.text()).toContain('渲染: 降级(无canvas)')
  })

  it('getCanvas 细节：pixelRatio 缺省为 1、fields 未返回尺寸时回退页面尺寸', async () => {
    um.mock.getSystemInfoSync.mockImplementation((() => ({
      platform: 'devtools',
      pixelRatio: 0, // falsy → dpr 回退 1（|| 1 分支）
      windowWidth: 375,
      windowHeight: 667,
    })) as any)
    const node: any = { getContext: () => ({}) }
    um.mock.createSelectorQuery.mockImplementation(() => {
      const q: any = {}
      q.in = () => q
      q.select = () => q
      // fields 回调不带 width/height → 页面回退 canvasW/canvasH
      q.fields = (_opts: unknown, cb: (res: any) => void) => {
        cb({ node })
        return q
      }
      q.exec = () => {}
      return q
    })
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    expect(node.width).toBe(375) // 375 * dpr 1
    expect(node.height).toBe(667)
    expect(FakeRenderer.instances[0]!.viewport).toEqual([375, 667])
    expect(wrapper.text()).toContain('渲染: 帧循环')
  })

  it('登录失败信息为空 → startError 回退固定文案（|| 右侧分支）', async () => {
    mockApi.guestLogin.mockRejectedValue(new Error(''))
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    triggerUniHook(wrapper.vm, 'onLoad', { level: '3' })
    await flushPromises()
    expect(wrapper.text()).toContain('登录失败')
  })

  it('引擎未建立时直呼选牌操作 → 早退（!engine.value 守卫）', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    ;(wrapper.vm as any).takeCard('c1')
    ;(wrapper.vm as any).discardCard('c1')
    ;(wrapper.vm as any).skipCards()
    await flushPromises()
    // 没有引擎、没有崩溃、没有选牌界面
    expect(wrapper.text()).not.toContain('选择一张卡')
  })

  it('onLoad 无 query → 默认第 1 关；onUnload 时渲染器为 null 也安全', async () => {
    mockQueryNode(null)
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    triggerUniHook(wrapper.vm, 'onLoad') // query undefined → levelId 1
    await flushPromises()
    expect(mockApi.startBattle).toHaveBeenCalledWith(1)
    triggerUniHook(wrapper.vm, 'onUnload') // renderer.value 为 null 的分支
  })

  it('onUnload：停止渲染器', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const renderer = FakeRenderer.instances[0]!
    expect(renderer.stopped).toBe(false)
    triggerUniHook(wrapper.vm, 'onUnload')
    expect(renderer.stopped).toBe(true)
  })
})


