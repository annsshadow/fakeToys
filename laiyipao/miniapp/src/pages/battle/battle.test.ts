// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
﻿// @vitest-environment jsdom
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
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { flushPromises } from '@vue/test-utils'
import * as api from '@/api/client'
import { installUniMock } from '../../test/uni-mock'
import { triggerUniHook } from '../../test/uni-app-stub'
import { mountPage } from '../../test/page'
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
  startBattle: vi.fn(),
  settleBattle: vi.fn(),
  abandonBattle: vi.fn(),
  myBattleHistory: vi.fn(),
  myBattleDetail: vi.fn(),
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
  /** R9 选卡反馈：权威 card_picks（由引擎公开数组透传） */
  cardPicks: number[] = []
  score = 100
  elapsedMs = 0
  cfg: {
    level: {
      wave_count: number
      star_targets?: number[]
      waves?: Array<{ wave_index: number; spawns: Array<{ enemy_id: number; count: number; interval: number; delay: number }> }>
    }
  } = { level: { wave_count: 3 } }
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
  ;(um.mock as any).redirectTo = vi.fn()
  __resetSfxForTest() // 音效解锁态是模块级单例
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
  mockApi.abandonBattle.mockResolvedValue({ refunded_energy: 6, abandoned: true } as any)
  mockApi.myBattleHistory.mockResolvedValue({ items: [], next_cursor: 0 } as any)
  mockApi.myBattleDetail.mockResolvedValue({} as any)
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

  it('R10 §1I.2：结算卡只在两次真实结算时给差值，缺历史显示诚实空态', async () => {
    mockQueryNode(null)
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    await new Promise((r) => setTimeout(r, TICK * 5))
    await flushPromises()
    expect(wrapper.text()).toContain('AI · 养成前后（同一关可信差值）')
    expect(wrapper.text()).toContain('还缺养成前那局战报，先打一局同关')
    expect(wrapper.text()).not.toContain('提升 +')
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

  it('结算失败后仍可放弃退体力：兜底卡提供放弃入口，点击走 abandon 退体力链路', async () => {
    mockApi.settleBattle.mockRejectedValue(new Error('服务器忙'))
    um.mock.showModal.mockImplementation((opts: any) => opts.success?.({ confirm: true }))
    const Battle = await loadPage()
    const { wrapper, store } = mountPage(Battle)
    const refreshWalletSpy = vi.spyOn(store, 'refreshWallet').mockResolvedValue(undefined)
    await setupDone(wrapper)
    FakeEngine.instances[0]!.winAtStep = 1
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    expect(wrapper.text()).toContain('服务器忙')
    // 结算失败 ⇒ 服务端未标记 used_at ⇒ 仍可放弃退体力；兜底卡必须保留入口
    const abandonBtn = wrapper.findAll('.btn').find((b: any) => b.text().includes('放弃'))
    expect(abandonBtn, '结算失败兜底卡必须保留退体力入口').toBeTruthy()
    await abandonBtn!.trigger('click')
    await flushPromises()
    expect(mockApi.abandonBattle).toHaveBeenCalledWith(77)
    expect(refreshWalletSpy).toHaveBeenCalled()
    await new Promise((resolve) => setTimeout(resolve, 850))
  })

  it('0 星且分数未达 1 星门槛时，下一步应指向 1 星而不是空喊冲 3 星', async () => {
    // 2026-11-22 R5 真浏览器实测：第 1 关结算 stars=0 / score=11646，
    // 1★ 门槛 11941，页面却说「再稳一点，冲 3 星拿满钥匙」。
    // 玩家连 1 星都没拿到就被催 3 星，是拿不到可执行信息的误导文案。
    mockApi.settleBattle.mockResolvedValue({
      win: true,
      stars: 0,
      score: 11646,
      loot: {},
      new_max_stage: 0,
    } as any)
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    eng.cfg.level.star_targets = [11941, 16916, 19503]
    eng.winAtStep = 1
    eng.settleInput = () => ({ kills: 39, leaked: 0, reactions: 91, replay_hash: 'zero-star' })
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    const text = wrapper.text()
    expect(text).toContain('1 星')
    expect(text).toContain('11941')
    expect(text).toContain('295')
    expect(text, '尚未拿到 1 星时不能空喊冲 3 星').not.toContain('冲 3 星')
  })

  it('R8 战斗 HUD 实时显示免钳分倒计时并在临近锚点时提示（来自关卡波次与战斗时钟）', async () => {
    // 2026-11-22 真浏览器：第 1 关 75s 结算时分数被钳到 19708，
    // 但战斗全程没有任何提示；玩家只能事后在结算卡看到上限。
    // 服务端是「快于锚点按比例裁分」，所以这是免钳分倒计时，不是保分窗口。
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const liveEngine = (wrapper.vm as any).engine as any
    expect(liveEngine, '页面必须把 engine 暴露给模板').toBeTruthy()
    liveEngine.cfg.level.waves = Array.from({ length: 5 }, (_, i) => ({
      wave_index: i,
      spawns: [{ enemy_id: 1, count: 6, interval: 200, delay: 0 }],
    }))
    liveEngine.elapsedMs = 10_000
    await flushPromises()
    expect(wrapper.text()).toContain('钳分中 · 距满分 35s')
    expect(wrapper.text(), '锚点前仍在钳分区，不能说成保分窗口').not.toContain('保分窗口')
    expect(wrapper.find('.time-budget').classes(), '安全期不得预警').not.toContain('warn')

    liveEngine.elapsedMs = 40_000
    await flushPromises()
    expect(wrapper.text()).toContain('即将免钳分')
    expect(wrapper.find('.time-budget').classes()).toContain('warn')

    liveEngine.elapsedMs = 46_000
    await flushPromises()
    expect(wrapper.text()).toContain('已免钳分')
    expect(wrapper.find('.time-budget').classes()).toContain('clamped')
  })

  it('R7 战斗 HUD 实时显示距下一星差额（门槛来自关卡配置，随分数更新）', async () => {
    // 2026-11-22 R7 实测：关卡目标只在选关页与结算卡出现，战斗中不知道还差多少分。
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    // VTU 会把 setup 的 ref 解包，wrapper.vm.engine 就是响应式的引擎代理；
    // 直接改它才能验证「随得分变化更新」，改 FakeEngine 的原始对象不会触发渲染。
    const liveEngine = (wrapper.vm as any).engine as any
    expect(liveEngine, '页面必须把 engine 暴露给模板').toBeTruthy()
    liveEngine.cfg.level.star_targets = [11941, 16916, 19503]
    liveEngine.score = 10000
    await flushPromises()
    expect(wrapper.text()).toContain('距 1 星 1941')
    liveEngine.score = 12000
    await flushPromises()
    const text = wrapper.text()
    expect(text).toContain('距 2 星 4916')
    expect(text, '跨过 1 星门槛后不应继续显示 1 星差额').not.toContain('距 1 星')
  })

  it('R7 结算归因把超时长钳分上限、实得分、星级门槛和漏怪数串成可执行建议', async () => {
    // 2026-11-22 R7 实测：第 1 关 74s 结算 10909 分，1★ 门槛 11941，
    // 页面只分散显示原始 clamp_note 与分数，玩家不知道「为什么没星、该改什么」。
    mockApi.settleBattle.mockResolvedValue({
      win: true,
      stars: 0,
      score: 10909,
      loot: {},
      new_max_stage: 0,
      clamped: true,
      clamp_note: '得分超时长额度：19726 → 10909（74s 内上限，满分需 60s）',
    } as any)
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    eng.cfg.level.star_targets = [11941, 16916, 19503]
    eng.winAtStep = 1
    eng.settleInput = () => ({ kills: 39, leaked: 0, reactions: 91, replay_hash: 'r7-attribution' })
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    const text = wrapper.text()
    expect(text).toContain('74s 内得分上限 19726')
    expect(text).toContain('实得 10909')
    expect(text).toContain('1 星门槛 11941')
    expect(text).toContain('还差 1032')
    expect(text).toContain('漏怪 0')
    expect(text).toContain('缩短清波时间')
  })

  it('R7 结算归因没有钳分数据时不得编造超时长上限', async () => {
    mockApi.settleBattle.mockResolvedValue({
      win: true,
      stars: 0,
      score: 11646,
      loot: {},
      new_max_stage: 0,
      clamped: false,
    } as any)
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    eng.cfg.level.star_targets = [11941, 16916, 19503]
    eng.winAtStep = 1
    eng.settleInput = () => ({ kills: 39, leaked: 0, reactions: 91, replay_hash: 'r7-no-cap' })
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    const text = wrapper.text()
    expect(text).toContain('实得 11646')
    expect(text).toContain('1 星门槛 11941')
    expect(text, '服务端没给 clamp_note 时不能编造得分上限').not.toContain('得分上限')
  })

  it('反应偏少时的下一步建议必须诚实说明攻击力权重上限，不得说关系不大', async () => {
    // 2026-11-21 R4 真浏览器实测：结算建议写「反应伤害不吃攻击力加成」；
    // t63 §1F 进一步钉住：damage.ts 反应攻击侧最高 300‰，「关系不大」同样是说谎，
    // 会让玩家误洗攻击词条。必须说清元素侧为主、攻击力也有贡献且上限 30%。
    mockApi.settleBattle.mockResolvedValue({
      win: true,
      stars: 1,
      score: 100,
      loot: {},
      new_max_stage: 0,
    } as any)
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    eng.winAtStep = 1
    eng.settleInput = () => ({ kills: 1, leaked: 0, reactions: 1, replay_hash: 'lowrx' })
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    expect(wrapper.text()).toContain('多凑一种元素触发反应链')
    expect(wrapper.text(), '不能把有 30% 贡献说成完全不吃').not.toContain('不吃攻击力加成')
    expect(wrapper.text(), 't63 §1F：30% 不是「关系不大」').not.toContain('关系不大')
    expect(wrapper.text()).toContain('面板攻击力也有贡献')
    expect(wrapper.text()).toContain('30%')
  })

  it('结算被服务端拒绝 → 结果卡不冒充通关，也不展示虚构的星级/奖励/战报入口', async () => {
    // 真实实测（2026-11-21，第 1 关通关）：服务端重算 replay_skills 拒绝后，
    // 页面仍显示「通关」+ 0 星 + C 评级 + 原始报错串，玩家完全不知道战绩没入账。
    mockApi.settleBattle.mockRejectedValue(
      new Error('replay_skills 与构筑不符\n上报 0:1:100:1:20\n重算 0:1:282:1:20'),
    )
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    FakeEngine.instances[0]!.winAtStep = 1
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    const text = wrapper.text()
    expect(text).toContain('战绩未入账')
    expect(text, '结算被拒时绝不能冒充通关').not.toContain('通关')
    expect(text).toContain('本地战绩')
    expect(text).toContain('构筑')
    expect(wrapper.find('.grade-badge').exists(), '失败时不该展示 C 评级').toBe(false)
    expect(wrapper.find('.stars-big').exists(), '失败时不该展示 0 星').toBe(false)
    expect(wrapper.find('.reward-row').exists(), '失败时不该展示奖励').toBe(false)
    expect(text, '没有 battle_id 时不该有战报入口').not.toContain('查看本局战报')
    expect(text, '原始错误串仍要保留，便于排障').toContain('0:1:282:1:20')
    expect(
      wrapper.findAll('.btn').some((b: any) => b.text().includes('放弃并退回体力')),
      '仍必须能退体力',
    ).toBe(true)
  })

  it('R151 令牌行丢失：结算 404 → 重取一次凭证并重报，奖励只发一次', async () => {
    mockApi.settleBattle
      .mockRejectedValueOnce(
        Object.assign(new Error('not found'), { status: 404, code: 'not_found' }),
      )
      .mockResolvedValueOnce({
        win: true,
        stars: 3,
        score: 777,
        loot: { coin: 12 },
        clamped: false,
      } as any)
    mockApi.startBattle
      .mockResolvedValueOnce(startBattleResp as any)
      .mockResolvedValueOnce({ ...startBattleResp, token_id: 88 } as any)

    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    eng.winAtStep = 1
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    // 开局 + 重取 = 2 次；结算 404 + 重报 = 2 次
    expect(mockApi.startBattle).toHaveBeenCalledTimes(2)
    expect(mockApi.settleBattle).toHaveBeenCalledTimes(2)
    // 重报必须带新 token，而不是沿用丢失的那个
    expect(mockApi.settleBattle.mock.calls[1]![0]).toMatchObject({ token_id: 88 })
    // 结果卡正常展示（这一局没有被判掉），奖励按服务端返回展示一次
    expect(wrapper.text()).toContain('通关')
    expect(wrapper.text()).toContain('金币 +12')
    expect(wrapper.text()).not.toContain('not found')
  })

  it('R151 重试仍 404 → 不再重取（最多一次），落兜底战报', async () => {
    mockApi.settleBattle.mockRejectedValue(
      Object.assign(new Error('not found'), { status: 404, code: 'not_found' }),
    )
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    FakeEngine.instances[0]!.winAtStep = 1
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    expect(mockApi.startBattle).toHaveBeenCalledTimes(2)
    expect(mockApi.settleBattle).toHaveBeenCalledTimes(2)
    expect(wrapper.text()).toContain('not found')
  })

  it('R151 非 404 的结算失败不触发重取凭证（不能把真错误当成令牌丢失）', async () => {
    mockApi.settleBattle.mockRejectedValue(
      Object.assign(new Error('校验失败'), { status: 422, code: 'unprocessable' }),
    )
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    FakeEngine.instances[0]!.winAtStep = 1
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    expect(mockApi.startBattle).toHaveBeenCalledTimes(1) // 只有开局
    expect(mockApi.settleBattle).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain('校验失败')
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

  it('手动暂停后切后台 → 保持暂停，暂停原因不被覆盖为「切后台」', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const renderer = FakeRenderer.instances[0]!

    await wrapper.find('.pause-btn').trigger('click')
    expect(renderer.paused).toBe(true)
    expect(wrapper.text()).not.toContain('切到后台时自动暂停了')

    triggerUniHook(wrapper.vm, 'onHide')
    await flushPromises()
    // 已经在暂停中：原因仍是玩家自己点的，遮罩文案不该被改写
    expect(renderer.paused).toBe(true)
    expect(wrapper.text()).not.toContain('切到后台时自动暂停了')
  })

  it('无 canvas 降级路径：暂停停掉本页定时器，点继续后恢复推进', async () => {
    // 这条走的是 `renderer === null` 的分支：`doPause` 里 clearInterval(timerId)、
    // `doResume` 里 startFallbackLoop()。删掉前者，定时器仍在空转；
    // 删掉后者，页面会永远停在「已暂停」遮罩上 —— 而 4 条 FakeRenderer 用例都发现不了。
    mockQueryNode(null)
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    eng.winAtStep = Infinity // 别让它自己结束，否则表就停了

    await new Promise((r) => setTimeout(r, TICK * 4))
    const before = eng.stepCount
    expect(before).toBeGreaterThan(0)

    triggerUniHook(wrapper.vm, 'onHide')
    await flushPromises()
    await new Promise((r) => setTimeout(r, TICK * 4))
    expect(eng.stepCount).toBe(before) // 定时器已停，一步都不走

    await wrapper.find('.pause-go').trigger('click')
    await new Promise((r) => setTimeout(r, TICK * 4))
    expect(eng.stepCount).toBeGreaterThan(before) // 恢复后继续
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

describe('battle.vue 战场图例（2026-11-17）', () => {
  it('帮助 chip 开合图例遮罩；图例覆盖热量/技能槽/全部反应名', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)

    expect(wrapper.find('.legend-overlay').exists()).toBe(false)
    await wrapper.find('.help-chip').trigger('click')
    expect(wrapper.find('.legend-overlay').exists()).toBe(true)
    const legendText = wrapper.find('.legend-box').text()
    // 三块核心解释必须都在：热量档位、技能槽读法、反应链（全部 7 条）
    expect(legendText).toContain('热量条')
    expect(legendText).toContain('技能槽')
    expect(legendText).toContain('蒸汽爆发')
    expect(legendText).toContain('破甲击退')
    expect(legendText).toContain('腐蚀扩散')
    // t63 §1F：图例也不能把攻击侧最高 300‰ 说成「关系不大」，会误导洗词条。
    expect(legendText, 't63 §1F：30% 不是「关系不大」').not.toContain('关系不大')
    expect(legendText).toContain('面板攻击力也有贡献')
    expect(legendText).toContain('30%')
    // 点遮罩空白处关闭
    await wrapper.find('.legend-overlay').trigger('click')
    expect(wrapper.find('.legend-overlay').exists()).toBe(false)
  })

  it('图例关闭按钮与再点帮助 chip 都能收起', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    await wrapper.find('.help-chip').trigger('click')
    await wrapper.find('.legend-close').trigger('click')
    expect(wrapper.find('.legend-overlay').exists()).toBe(false)
    await wrapper.find('.help-chip').trigger('click')
    expect(wrapper.find('.legend-overlay').exists()).toBe(true)
    await wrapper.find('.help-chip').trigger('click')
    expect(wrapper.find('.legend-overlay').exists()).toBe(false)
  })

  it('R3 打开图例会暂停战斗，关闭后自动继续；手动暂停不被图例误恢复', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    const renderer = FakeRenderer.instances[0]!
    eng.winAtStep = Infinity
    renderer.loop!(TICK)
    const before = eng.stepCount
    expect(before).toBeGreaterThan(0)
    expect(renderer.paused).toBe(false)

    // 玩家在战斗中途打开图例读说明：整块遮罩挡住画布，战斗时间不该继续流逝。
    await wrapper.find('.help-chip').trigger('click')
    expect(wrapper.find('.legend-overlay').exists()).toBe(true)
    expect(renderer.paused).toBe(true)
    renderer.loop!(TICK)
    renderer.loop!(TICK)
    expect(eng.stepCount).toBe(before)

    // 关掉图例就该回到战斗，不该再让玩家多点一次「继续战斗」。
    await wrapper.find('.legend-close').trigger('click')
    expect(wrapper.find('.legend-overlay').exists()).toBe(false)
    expect(renderer.paused).toBe(false)
    renderer.loop!(TICK)
    expect(eng.stepCount).toBe(before + 1)

    // 手动暂停时打开图例：关闭后仍保持暂停，不能替玩家擅自恢复。
    await wrapper.find('.pause-btn').trigger('click')
    expect(renderer.paused).toBe(true)
    await wrapper.find('.help-chip').trigger('click')
    expect(wrapper.find('.legend-overlay').exists()).toBe(true)
    await wrapper.find('.legend-close').trigger('click')
    expect(renderer.paused).toBe(true)
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
    // 2026-11-17 v2：图标从 emoji 换成 SVG（Icon 组件），可断言的形态
    // 从文本内容改为 aria-label —— 意图不变：状态可见、随点击切换、落存储。
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)

    const chip = wrapper.find('.mute-chip')
    expect(chip.attributes('aria-label')).toBe('静音')
    await chip.trigger('click')
    expect(wrapper.find('.mute-chip').attributes('aria-label')).toBe('取消静音')
    expect(um.mock.setStorageSync).toHaveBeenCalledWith('lyp_sfx_muted', '1')

    await wrapper.find('.mute-chip').trigger('click')
    expect(wrapper.find('.mute-chip').attributes('aria-label')).toBe('静音')
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

  it('结算**在途** → 退回通用文案（此窗口 settleResult 还是 null，stars/score 取 ?? 0）', async () => {
    // `settled.value = true` 在 settle() 第一行，而 settleResult 要等
    // await settleBattle 返回后才赋值。弱网下这个窗口可达数秒 ——
    // 此时若按「settled 已置位」判断，分享出去就是「第 3 关 0 星 · 0 分」。
    let resolveSettle: (v: any) => void = () => {}
    mockApi.settleBattle.mockImplementation(
      () => new Promise((r) => (resolveSettle = r)) as any,
    )
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    eng.winAtStep = 1
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises() // 结算请求已发出但未返回

    const payload = triggerUniHook(wrapper.vm, 'onShareAppMessage', { from: 'menu' }) as {
      title: string
    }
    expect(payload.title).toBe('来一炮 · 塔防 + 元素反应')
    expect(payload.title).not.toContain('0 星')
    expect(payload.title).not.toContain('0 分')

    // 放行，避免挂起的 Promise 泄漏到后续用例
    resolveSettle({ win: true, stars: 3, score: 555, loot: {} })
    await flushPromises()
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


describe('battle.vue R9 战斗反馈与连连贯性（2026-11-22）', () => {
  it('R9 反应事件在 HUD 留下可读的最近反应记录，而不只是瞬时粒子', async () => {
    // 真浏览器第 1 关 8× 连拍：反应只在 canvas 上闪一下，HUD 没有任何
    // 「刚触发了什么」的可读记录。高层速下玩家根本看不清，事后只能看结算计数。
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const eng = FakeEngine.instances[0]!
    eng.cardAtStep = Number.POSITIVE_INFINITY
    eng.winAtStep = Number.POSITIVE_INFINITY
    eng.events = [
      { type: 'reaction', reaction: 'steam_burst', damage: 1234n, resisted: false, x: 0n, y: 0n },
      { type: 'reaction', reaction: 'overheat', damage: 567n, resisted: false, x: 0n, y: 0n },
    ]
    FakeRenderer.instances[0]!.loop!(TICK)
    await wrapper.vm.$nextTick()

    const feed = wrapper.find('.reaction-feed')
    expect(feed.exists(), '战斗 HUD 应有最近反应记录').toBe(true)
    expect(feed.text()).toContain('蒸汽爆发')
    expect(feed.text()).toContain('1234')
    expect(feed.text()).toContain('过热')
  })

  it('R9 通关后提供下一关入口，直接带着解锁关卡进入选关页', async () => {
    // 真浏览器第 1 关结算后只有「再来一局 / 返回关卡」：返回后选关页仍停在第 1 关，
    // 玩家要自己找到第 2 关。前 3 关连贯性被这个多余步骤打断。
    mockApi.settleBattle.mockResolvedValue({
      win: true, stars: 0, score: 11646, loot: {}, new_max_stage: 2,
    } as any)
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    triggerUniHook(wrapper.vm, 'onLoad', { level: '1' })
    await flushPromises()
    FakeEngine.instances[0]!.winAtStep = 1
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    const next = wrapper.find('.next-level-btn')
    expect(next.exists(), '通关且解锁新关时应给出下一关入口').toBe(true)
    expect(next.text()).toContain('第 2 关')
    await next.trigger('click')
    expect((um.mock as any).redirectTo).toHaveBeenCalledWith({ url: '/pages/stage/stage?level=2' })
  })

  it('R9 结算失败不显示下一关入口，避免谎报解锁', async () => {
    mockApi.settleBattle.mockRejectedValue(new Error('上报失败'))
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    triggerUniHook(wrapper.vm, 'onLoad', { level: '1' })
    await flushPromises()
    FakeEngine.instances[0]!.winAtStep = 1
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    expect(wrapper.text()).toContain('上报失败')
    expect(wrapper.find('.next-level-btn').exists()).toBe(false)
  })

  it('R9 反应记录显示真实元素组合与 AI 因果提示', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    triggerUniHook(wrapper.vm, 'onLoad', { level: '1' })
    await flushPromises()
    const eng = FakeEngine.instances[0]!
    eng.events = [
      { type: 'reaction', reaction: 'overheat', x: 0n, y: 0n, damage: 1234n, resisted: false, existingElement: 'fire', incomingElement: 'lightning' },
      { type: 'reaction', reaction: 'steam_burst', x: 0n, y: 0n, damage: 88n, resisted: true },
    ]
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    const feed = wrapper.find('.reaction-feed')
    expect(feed.exists()).toBe(true)
    expect(feed.text()).toContain('焰+电 → 过热')
    expect(feed.text()).toContain('1234')
    expect(feed.text()).toContain('AI')
    expect(feed.text()).toContain('密集波 25% 溅射')
    expect(feed.text()).toContain('依据 reaction=overheat/damage=1234')
    expect(feed.text()).toContain('点卡凑同元素')
    expect(feed.text()).toContain('元素组合未上报')
    expect(feed.text()).toContain('抗性削减')
  })

  it('R9 选卡面板给出基于 card_picks 的 AI 取舍反馈', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    triggerUniHook(wrapper.vm, 'onLoad', { level: '1' })
    await flushPromises()
    const eng = FakeEngine.instances[0]!
    eng.cardPicks = [0, -1]
    eng.reactionsCount = 7
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    const fb = wrapper.find('.card-choice-feedback')
    expect(fb.exists()).toBe(true)
    expect(fb.text()).toContain('AI')
    expect(fb.text()).toContain('已取卡 1 张')
    expect(fb.text()).toContain('跳过 1 波')
    expect(fb.text()).toContain('反应 7 次')
    expect(fb.text()).toContain('依据 card_picks')
    expect(fb.text()).toContain('点卡装备')
  })

  it('R9 结算反馈用 card_picks 与真实反应伤害占比归因', async () => {
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    triggerUniHook(wrapper.vm, 'onLoad', { level: '1' })
    await flushPromises()
    const eng = FakeEngine.instances[0]!
    eng.cardAtStep = Infinity
    eng.winAtStep = 1
    eng.events = [
      { type: 'hit', x: 0n, y: 0n, damage: 1000n, crit: false },
      { type: 'reaction', reaction: 'overheat', x: 0n, y: 0n, damage: 250n, resisted: false },
    ]
    eng.settleInput = vi.fn(() => ({
      kills: 39, leaked: 0, reactions: 12, replay_hash: 'replayhash',
      card_picks: [0, -1, -7, 2],
    }))
    FakeRenderer.instances[0]!.loop!(TICK)
    await flushPromises()

    const fb = wrapper.find('.settle-card-feedback')
    expect(fb.exists()).toBe(true)
    expect(fb.text()).toContain('AI')
    expect(fb.text()).toContain('取卡 3 张')
    expect(fb.text()).toContain('弃 1 张')
    expect(fb.text()).toContain('跳过 1 波')
    expect(fb.text()).toContain('反应 12 次')
    expect(fb.text()).toContain('反应伤害占比 25%')
    expect(fb.text()).toContain('依据 card_picks')
    expect(fb.text()).toContain('下一局')
  })

  it('R9-BUG-1 移动视口 HUD 胶囊不竖排：nowrap/flex-shrink:0 且窄屏换行', async () => {
    // t93 终审 390×844 真浏览器：波次/击杀/星级/钳分四个胶囊被挤成逐字竖排。
    // 胶囊自身必须不收缩、文字不换行；窄屏靠父行换行，而不是把字挤竖。
    const Battle = await loadPage()
    const { wrapper } = mountPage(Battle)
    await setupDone(wrapper)
    const liveEngine = (wrapper.vm as any).engine as any
    liveEngine.cfg.level.star_targets = [11941]
    liveEngine.cfg.level.waves = Array.from({ length: 5 }, (_, i) => ({
      wave_index: i,
      spawns: [{ enemy_id: 1, count: 6, interval: 200, delay: 0 }],
    }))
    await flushPromises()

    expect(wrapper.findAll('.hud-pill').length, '四项 HUD 都应常驻').toBeGreaterThanOrEqual(4)
    for (const frag of ['波次', '击杀', '距 1 星', '钳分中']) {
      expect(wrapper.text()).toContain(frag)
    }

    const css = readFileSync(resolve(__dirname, 'battle.vue'), 'utf8')
    expect(css, 'HUD 行窄屏必须换行').toMatch(/\.hud-row\s*\{[^}]*flex-wrap:\s*wrap/)
    expect(css, '胶囊不得被 flex 压成竖条').toMatch(/\.hud-pill\s*\{[^}]*flex-shrink:\s*0/)
    expect(css, '胶囊文字不得逐字换行').toMatch(/\.hud-pill\s*\{[^}]*white-space:\s*nowrap/)
    expect(css, '钳分文案同样不得竖排').toMatch(/\.time-budget-text\s*\{[^}]*white-space:\s*nowrap/)
    expect(css, '移动视口应有专门收缩布局').toMatch(/@media\s*\(max-width:\s*430px\)/)
    expect(css, '窄屏下 spacer 不应把胶囊挤出可视宽度').toMatch(/@media\s*\(max-width:\s*430px\)[\s\S]*\.hud-spacer\s*\{\s*display:\s*none/)
  })
})
