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
  /** step() 返回的事件（音效接线的判据：页面必须把它们交给 playBattleEvents） */
  events: unknown[] = []
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
    return this.events as any[]
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
  /** 当前倍速（真实渲染器会按它重建逻辑时钟） */
  speed = 1
  /** 暂停态（2026-10-23）。真实渲染器会停掉两条时钟，替身只记状态。 */
  paused = false
  constructor(canvas: any, engine: any) {
    this.canvas = canvas
    this.engine = engine
    FakeRenderer.instances.push(this)
  }
  setViewport(w: number, h: number) {
    this.viewport = [w, h]
  }
  setSpeed(mult: number) {
    this.speed = mult
  }
  start(cb: (dt: number) => void) {
    this.started = true
    this.loop = cb
  }
  stop() {
    this.stopped = true
    this.paused = false
  }
  pause() {
    this.paused = true
  }
  resume() {
    this.paused = false
  }
  handleEvents(_events: unknown[]) {}
}

vi.mock('@/game/engine', () => ({ BattleEngine: FakeEngine, TICK_MS: TICK }))
// 音效映射本身由 audio/battle-sfx.test.ts 覆盖；这里只守「页面有没有接线」。
vi.mock('@/audio/battle-sfx', () => ({ playBattleEvents: vi.fn() }))
vi.mock('@/game/replay', () => ({ equippedFromSnapshot: vi.fn() }))
// 渲染层只替掉 BattleRenderer（渲染细节由 canvas.test.ts 覆盖），
// 倍速换算（BATTLE_SPEED_STEPS / normalizeSpeed / logicIntervalMs）**用真货** ——
// 在这里再抄一份公式，等于让「页面按 6.25ms 建时钟」变成自证，
// 真实换算写错了也测不出来。
vi.mock('@/render/canvas', async (importOriginal) => {
  const actual = await importOriginal<typeof import('@/render/canvas')>()
  return { ...actual, BattleRenderer: FakeRenderer }
})

const mockApi = vi.mocked(api, true)
const battleSfxModule = await import('@/audio/battle-sfx')
const playBattleEventsMock = vi.mocked(battleSfxModule.playBattleEvents)
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
  mockApi.fetchConfigIfVersion.mockResolvedValue({ status: 'modified', config: makeConfig(), serverTimeMs: 0 })
  mockApi.fetchWallet.mockResolvedValue({ coin: 1, gem: 2, energy: 30, keys: 4, tokens: {} })
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
    // 第 133 轮：分数规则必须由 battle.vue 从服务端 config 传入（不再是引擎缺省）。
    // 若接线退化（不再传 scoreRules），这里是 undefined → 红。
    expect(eng.options.scoreRules).toBeDefined()

    const renderer = FakeRenderer.instances[0]!
    expect(renderer.started).toBe(true)
    expect(renderer.viewport).toEqual([375, 667]) // 视口用 CSS 像素
    // canvas 按 dpr 放大（pixelRatio 2 × 375）
    expect(lastNode.width).toBe(750)
    expect(lastNode.height).toBe(1334)
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
  })

  it('第 137 轮：onUnload 之后 getCanvas 才 resolve，渲染器不在已卸载页面启动（防泄漏）', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)

    // 可控的 canvas 查询：fields 回调先挂起，直到测试手动 resolve。
    let resolveNode: (res: any) => void = () => {}
    um.mock.createSelectorQuery.mockImplementation(() => {
      const q: any = {}
      q.in = () => q
      q.select = () => q
      q.fields = (_o: unknown, cb: (res: any) => void) => {
        resolveNode = cb
        return q
      }
      q.exec = () => {}
      return q
    })

    // 触发 onLoad → setup() 跑过 login/startBattle，停在 await getCanvas()
    triggerUniHook(wrapper.vm, 'onLoad', { level: '3' })
    await flushPromises()
    expect(FakeEngine.instances.length).toBe(1) // 引擎已建（在 getCanvas 之前）

    // getCanvas 仍挂起时退出页面（onUnload 时 renderer 还是 null，无从可停）
    triggerUniHook(wrapper.vm, 'onUnload')
    await flushPromises()

    // 此刻才让 getCanvas resolve（模拟「晚到的 canvas 节点」）
    resolveNode({ node: { getContext: () => ({}) }, width: 375, height: 667 })
    await flushPromises()

    // 修前：getCanvas 完成后会 new BattleRenderer + r.start() → 已卸载页面上泄漏。
    // 修后：pageUnloaded=true → setup 直接 return，渲染器绝不创建。
    expect(FakeRenderer.instances.length).toBe(0)
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
    // 没有 canvas 时战斗逻辑照跑，只是没有渲染层
    expect(FakeRenderer.instances.length).toBe(0)

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
    // 选牌引导（口语化说明）：标题 + 「点卡上的「弃」…还能回 1 点热量」的提示行
    expect(wrapper.text()).toContain('这一波给你 3 张卡，点一张就装备上')
    expect(wrapper.text()).toContain('点卡上的「弃」丢掉，还能回 1 点热量')
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

    await wrapper.findAll('.btn').find((b: any) => b.text().includes('直接下一波'))!.trigger('click')
    expect(eng.skipped).toBe(1)
  })

  it('节点既无 getContext 也无 querySelector → 视为拿不到 canvas，回退降级', async () => {
    mockQueryNode({}) // 既不是 canvas，也不是 uni-canvas 包装
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    expect(FakeRenderer.instances.length).toBe(0)
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
    expect(wrapper.text()).not.toContain('这一波给你 3 张卡')
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

describe('battle.vue 暂停（2026-10-23）', () => {
  // 加暂停的动因是「切后台回来战斗已经推进了一段」：
  // 逻辑时钟是 setInterval，与页面可见性无关（当初为后台也能跑逻辑刻意做的），
  // 于是「切后台会怎样」取决于平台对后台定时器的处置 —— 未定义。
  //
  // 断言分三层：
  //   1. UI：按钮能暂停，遮罩出现，点继续能恢复；
  //   2. 引擎：暂停期间**一次 step 都没有**（多一道守卫，不只依赖定时器停掉）；
  //   3. 切后台：onHide 自动暂停，onShow **不自动恢复**。

  it('点暂停 → 渲染器 pause、遮罩出现；点继续 → resume、遮罩消失', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const renderer = FakeRenderer.instances[0]!
    expect(renderer.paused).toBe(false)

    await wrapper.find('.pause-btn').trigger('click')
    expect(renderer.paused).toBe(true)
    expect(wrapper.find('.pause-overlay').exists()).toBe(true)
    expect(wrapper.text()).toContain('已暂停')
    // 暂停时「暂停」按钮自己消失（避免重复点）
    expect(wrapper.find('.pause-btn').exists()).toBe(false)

    await wrapper.find('.pause-go').trigger('click')
    expect(renderer.paused).toBe(false)
    expect(wrapper.find('.pause-overlay').exists()).toBe(false)
  })

  it('暂停期间即使有人调了帧回调，引擎也不推进（不依赖定时器真的停掉）', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    const renderer = FakeRenderer.instances[0]!

    // 先跑一步确认链路通
    renderer.loop!(TICK)
    const before = eng.stepCount
    expect(before).toBeGreaterThan(0)

    await wrapper.find('.pause-btn').trigger('click')
    // 模拟「平台在暂停后仍补发了一次帧回调」：这是真实存在的场景
    renderer.loop!(TICK)
    renderer.loop!(TICK)
    expect(eng.stepCount).toBe(before) // 一步都不能走

    await wrapper.find('.pause-go').trigger('click')
    renderer.loop!(TICK)
    expect(eng.stepCount).toBe(before + 1) // 恢复后正常推进
  })

  it('onHide 自动暂停；onShow 不自动恢复（仍需玩家点继续）', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const renderer = FakeRenderer.instances[0]!

    triggerUniHook(wrapper.vm, 'onHide')
    await flushPromises() // 钩子是同步的，DOM 重渲染是异步的
    expect(renderer.paused).toBe(true)
    expect(wrapper.find('.pause-overlay').exists()).toBe(true)
    // 遮罩上说明「是切后台导致的」，而不是无来由地暂停
    expect(wrapper.text()).toContain('切到后台时自动暂停了')

    triggerUniHook(wrapper.vm, 'onShow')
    await flushPromises()
    // 关键：回来**不自动继续** —— 自动继续会让玩家发现自己离开的几秒里
    // 防线被打掉一半，那是比「等你点继续」糟糕得多的体感
    expect(renderer.paused).toBe(true)
    expect(wrapper.find('.pause-overlay').exists()).toBe(true)

    await wrapper.find('.pause-go').trigger('click')
    expect(renderer.paused).toBe(false)
  })

  it('已结算后切后台不暂停（没什么可暂停的）', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    const renderer = FakeRenderer.instances[0]!
    // 推到 won 触发结算
    eng.winAtStep = 1
    renderer.loop!(TICK)
    await flushPromises()
    expect(mockApi.settleBattle).toHaveBeenCalled()

    triggerUniHook(wrapper.vm, 'onHide')
    await flushPromises()
    expect(renderer.paused).toBe(false)
    expect(wrapper.find('.pause-overlay').exists()).toBe(false)
  })
})

describe('battle.vue 音效接线（2026-10-23）', () => {
  // 映射规则由 audio/battle-sfx.test.ts 覆盖。这里只守一件更基础的事：
  // 页面**真的把事件交给了音效层**。
  // 不守这条的话，最可能的失效是「音效模块写好了、测试全绿、页面没接」——
  // 而那正是本仓库「有写无读」那类缺陷。

  it('每帧的事件都交给 playBattleEvents', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    const renderer = FakeRenderer.instances[0]!

    eng.events = [{ type: 'fire', slot: 0, skillId: 1 }]
    renderer.loop!(TICK)
    expect(playBattleEventsMock).toHaveBeenCalledWith([{ type: 'fire', slot: 0, skillId: 1 }])
  })

  it('静音开关：点一下写入存储并换图标，再点一下取消', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)

    const chip = wrapper.find('.mute-chip')
    expect(chip.text()).toBe('🔊')
    await chip.trigger('click')
    expect(wrapper.find('.mute-chip').text()).toBe('🔇')
    expect(um.mock.setStorageSync).toHaveBeenCalledWith('lyp_sfx_muted', '1')

    await wrapper.find('.mute-chip').trigger('click')
    expect(wrapper.find('.mute-chip').text()).toBe('🔊')
    expect(um.mock.setStorageSync).toHaveBeenLastCalledWith('lyp_sfx_muted', '0')
  })
})

describe('battle.vue 分享（2026-10-23）', () => {
  it('未结算 → 通用文案，不出现分数（此时分数还没有权威值）', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const payload = triggerUniHook(wrapper.vm, 'onShareAppMessage', { from: 'menu' }) as {
      title: string
      path: string
    }
    expect(payload.title).toBe('来一炮 · 塔防 + 元素反应')
    expect(payload.title).not.toContain('分')
    expect(payload.path).toBe('/pages/index/index')
  })

  it('已结算 → 标题带关卡/星级/分数', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    eng.winAtStep = 1
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()
    const payload = triggerUniHook(wrapper.vm, 'onShareAppMessage', { from: 'menu' }) as {
      title: string
    }
    expect(payload.title).toContain('第 3 关')
    expect(payload.title).toContain('3 星')
    expect(payload.title).toContain('555 分')
  })

  it('结算失败 → 退回通用文案（失败时 stars 恒为 0，那更假）', async () => {
    mockApi.settleBattle.mockRejectedValue(new Error('结算失败'))
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    eng.winAtStep = 1
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()
    const payload = triggerUniHook(wrapper.vm, 'onShareAppMessage', { from: 'menu' }) as {
      title: string
    }
    expect(payload.title).toBe('来一炮 · 塔防 + 元素反应')
    expect(payload.title).not.toContain('0 星')
  })
})

describe('battle.vue 倍速档位', () => {
  it('档位渲染为 1/2/4/8×，点选后交给渲染器并持久化', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const renderer = FakeRenderer.instances[0]!
    // 建逻辑时钟前就把持久化档位交进去（默认 1×）
    expect(renderer.speed).toBe(1)

    const chips = wrapper.findAll('.speed-chip')
    expect(chips.map((c) => c.text())).toEqual(['1×', '2×', '4×', '8×'])
    // 当前档位高亮
    expect(chips[0]!.classes()).toContain('on')

    await chips[3]!.trigger('click')
    expect(renderer.speed).toBe(8)
    expect(chips[3]!.classes()).toContain('on')
    expect(chips[0]!.classes()).not.toContain('on')
    expect(um.mock.setStorageSync).toHaveBeenCalledWith('lyp_battle_speed', '8')

    // 点已选中的档位 → 幂等，不再写存储
    um.mock.setStorageSync.mockClear()
    await chips[3]!.trigger('click')
    expect(um.mock.setStorageSync).not.toHaveBeenCalled()
  })

  it('持久化的 8× 在开局时就生效（不必每次重开都点一遍）', async () => {
    um.storage.set('lyp_battle_speed', '8')
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    expect(FakeRenderer.instances[0]!.speed).toBe(8)
    expect(wrapper.findAll('.speed-chip')[3]!.classes()).toContain('on')
  })

  it('存储里是脏值（空串 / 非数字 / 0）→ 回落到 1×', async () => {
    um.storage.set('lyp_battle_speed', 'NaN')
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    expect(FakeRenderer.instances[0]!.speed).toBe(1)
  })

  it('无 canvas 的降级路径同样换挡：定时器间隔按倍速缩短', async () => {
    mockQueryNode(null)
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    // 只观察间隔，不依赖定时器真的触发（否则就是一条 flaky 的计时测试）
    const spy = vi.spyOn(globalThis, 'setInterval')
    await setupDone(wrapper)
    expect(spy.mock.calls.map((c) => Number(c[1]))).toContain(TICK) // 1× → 50ms

    spy.mockClear()
    await wrapper.findAll('.speed-chip')[3]!.trigger('click') // 8×
    expect(spy.mock.calls.map((c) => Number(c[1]))).toEqual([TICK / 8]) // 6.25ms

    spy.mockRestore()
    triggerUniHook(wrapper.vm, 'onUnload')
  })
})


