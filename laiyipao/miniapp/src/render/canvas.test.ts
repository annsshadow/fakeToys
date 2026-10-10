/**
 * 渲染层无头冒烟测试。
 *
 * 为什么需要这个：真实浏览器验证受限于"标签页隐藏时 rAF 被冻结、
 * 定时器被节流到 ~1Hz"，无法可靠观察战斗推进；而截图又要求可见窗口。
 * 因此这里用打桩的 CanvasRenderingContext2D 直接驱动渲染层，
 * 验证的是**代码路径正确性**（不抛异常、调用了预期的绘制指令），
 * 而非像素级视觉验证。
 *
 * 打桩记录所有调用，从而能断言"确实画了背景/敌人/弹丸/地形/HUD"。
 */
import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { BattleRenderer } from './canvas'
import { BattleEngine, TICK_MS } from '../game/engine'
import { defaultAttacker } from '../game/damage'
import type { Enemy, EnemyDef, FloatText, GeneratedLevel, SkillDef } from '../game/types'
import { ELEMENT_COLOR, type Element } from '../game/elements'

/** 打桩的 2D 上下文：记录调用，不做任何实际绘制。 */
class StubContext {
  calls: string[] = []
  /** 每次 fill / fillRect 时的 fillStyle 快照：用于断言走了哪个着色分支 */
  fillStyles: any[] = []
  /** 每次 stroke / strokeRect 时的 strokeStyle 快照 */
  strokeStyles: any[] = []
  /** 每次 fillText 时的 fillStyle 快照（弹窗/飘字的颜色在这里设置） */
  textStyles: any[] = []
  fillStyle: any = '#000'
  strokeStyle: any = '#000'
  lineWidth = 1
  globalAlpha = 1
  font = ''
  textAlign: any = 'left'

  save() { this.calls.push('save') }
  restore() { this.calls.push('restore') }
  translate() {}
  scale() {}
  rotate() { this.calls.push('rotate') }
  setTransform() {}
  clearRect() { this.calls.push('clearRect') }
  fillRect(...a: any[]) {
    this.calls.push(`fillRect:${a[0]},${a[1]},${a[2]},${a[3]}`)
    this.fillStyles.push(this.fillStyle)
  }
  strokeRect() {
    this.calls.push('strokeRect')
    this.strokeStyles.push(this.strokeStyle)
  }
  beginPath() { this.calls.push('beginPath') }
  closePath() { this.calls.push('closePath') }
  moveTo() { this.calls.push('moveTo') }
  lineTo() { this.calls.push('lineTo') }
  arc() { this.calls.push('arc') }
  rect() {}
  fill() {
    this.calls.push('fill')
    this.fillStyles.push(this.fillStyle)
  }
  stroke() {
    this.calls.push('stroke')
    this.strokeStyles.push(this.strokeStyle)
  }
  fillText(t: string) {
    this.calls.push(`fillText:${t}`)
    this.textStyles.push(this.fillStyle)
  }
  // 形参必须显式声明（哪怕不用），否则测试里 spread 调用它会被 TS
  // 判为「非元组的 spread 调用」而报错。
  createLinearGradient(_x0: number, _y0: number, _x1: number, _y1: number) {
    return { addColorStop: () => {} }
  }
  measureText(t: string) { return { width: t.length * 6 } }
}

/** 打桩 canvas。返回类型必须显式声明 —— 否则 __ctx.calls 会退化成 any，
 *  级联出大量隐式 any 报错。 */
interface StubCanvas {
  width: number
  height: number
  style: Record<string, string>
  getContext: (type: string) => unknown
  __ctx: StubContext
}

function makeCanvas(): StubCanvas {
  const ctx = new StubContext()
  return {
    width: 750,
    height: 1334,
    style: {},
    getContext: () => ctx,
    __ctx: ctx,
  }
}

const zeroResist = { fire: 0, ice: 0, lightning: 0, corrosion: 0, kinetic: 0 }

const ENEMIES: EnemyDef[] = [
  {
    id: 1, code: 'wanderer', name: '游荡者', category: 'normal',
    hp: 100, speed: 40000, armor: 0, shield_hp: 0, attack: 10, attack_range: 0,
    attack_interval: 0, fly_height: 0, burrow: false, is_boss: false,
    resist: zeroResist, descr: '',
  },
  {
    id: 3, code: 'ironjaw', name: '铁颚精英', category: 'boss',
    hp: 3000, speed: 20000, armor: 200, shield_hp: 800, attack: 30, attack_range: 300,
    attack_interval: 2000, fly_height: 0, burrow: false, is_boss: true,
    resist: zeroResist, descr: '',
  },
]

const SKILLS: SkillDef[] = [
  {
    id: 1, code: 'ember', name: '燃烧弹', family: 'flame', element: 'fire' as Element,
    kind: 'active', descr: '', base_damage: 100, heat_cost: 20, cooldown_ms: 800,
    pierce: 0, aoe_radius: 60, apply_element: 'fire', apply_stacks: 1,
    projectile_speed: 60000, chain: 0, unlock_level: 1,
  },
]

function mkLevel(over: Partial<GeneratedLevel> = {}): GeneratedLevel {
  return {
    id: 1, chapter: 1, name: '渲染测试', seed: '1', base_hp: 1000, wave_count: 1,
    difficulty: 1000, energy_cost: 6, element_cap: 3, armor_permille: 0,
    max_reaction_tier: 2, is_boss: false, star_targets: [100, 200, 300],
    // 理论满分：结算裁剪的上界锚定在它上面（不是 star_targets[2]）
    max_score: 500,
    terrain: [{ kind: 'oil_drum', x: 600, y: 600, param: 50 }],
    waves: [{ wave_index: 0, spawns: [{ enemy_id: 1, count: 3, interval: 50, delay: 0 }] }],
    ...over,
  }
}

function mkEngine(level = mkLevel()): BattleEngine {
  return new BattleEngine({
    level,
    enemies: new Map(ENEMIES.map((e) => [e.id, e])),
    skills: new Map(SKILLS.map((s) => [s.id, s])),
    equipped: [
      {
        skillId: 1, name: '燃烧弹', element: 'fire' as Element, kind: 'active' as const,
        heatCost: 20n, cooldownMs: 800, pierce: 0, aoeRadius: 60, baseDamage: 100n,
        applyElement: 'fire' as Element, applyStacks: 1n, projectileSpeed: 60000,
        chain: 0, slot: 0, cooldownRemaining: 0,
      },
    ],
    attacker: defaultAttacker(),
    seed: 4242,
  })
}

describe('BattleRenderer 无头渲染', () => {
  let canvas: StubCanvas

  beforeEach(() => {
    canvas = makeCanvas()
  })

  /** BattleRenderer 的构造签名收 HTMLCanvasElement；桩只需结构兼容即可 */
  function makeRenderer(engine: BattleEngine): BattleRenderer {
    return new BattleRenderer(canvas as unknown as HTMLCanvasElement, engine)
  }

  it('构造后能取到 2D 上下文并计算视口', () => {
    const e = mkEngine()
    const r = makeRenderer(e)
    r.setViewport(375, 667)
    // 视口由 CSS 尺寸决定，不应被 dpr 后的像素尺寸影响
    expect(r).toBeTruthy()
    canvas.__ctx.calls.length = 0
  })

  it('draw() 不抛异常，且画了背景与 HUD', () => {
    const e = mkEngine()
    const r = makeRenderer(e)
    e.start()
    for (let i = 0; i < 60; i++) r.handleEvents(e.step())

    canvas.__ctx.calls.length = 0
    expect(() => r.draw(TICK_MS)).not.toThrow()

    const calls: string[] = canvas.__ctx.calls
    // fillRect 带坐标参数，判定用前缀匹配
    expect(calls.some((c) => c.startsWith('fillRect'))).toBe(true) // 背景与血条
    expect(calls.some((c) => c.startsWith('fillText:'))).toBe(true) // HUD 文字
  })

  it('敌人存活时会绘制敌人形状与血条', () => {
    const e = mkEngine()
    const r = makeRenderer(e)
    e.start()
    for (let i = 0; i < 60; i++) e.step()
    expect(e.enemies.length).toBeGreaterThan(0)

    canvas.__ctx.calls.length = 0
    r.draw(TICK_MS)
    const calls = canvas.__ctx.calls
    expect(calls).toContain('arc') // 普通敌人是圆形
    expect(calls).toContain('beginPath')
  })

  it('BOSS 存活时绘制八边形（moveTo + lineTo 多次）', () => {
    const e = mkEngine(
      mkLevel({
        waves: [{ wave_index: 0, spawns: [{ enemy_id: 3, count: 1, interval: 0, delay: 0 }] }],
      }),
    )
    const r = makeRenderer(e)
    e.start()
    for (let i = 0; i < 40; i++) e.step()
    expect(e.enemies.some((x) => x.isBoss)).toBe(true)

    canvas.__ctx.calls.length = 0
    r.draw(TICK_MS)
    // 八边形 = 8 次 lineTo
    const lineToCount = canvas.__ctx.calls.filter((c: string) => c === 'lineTo').length
    expect(lineToCount).toBeGreaterThanOrEqual(8)
  })

  it('地形会被绘制（油桶是圆）', () => {
    const e = mkEngine()
    const r = makeRenderer(e)
    e.start()
    e.step()
    canvas.__ctx.calls.length = 0
    r.draw(TICK_MS)
    expect(canvas.__ctx.calls).toContain('arc')
  })

  it('弹丸存在时会绘制拖尾（createLinearGradient）', () => {
    const e = mkEngine()
    const r = makeRenderer(e)
    e.start()
    // 推进到有弹丸在飞
    for (let i = 0; i < 200 && e.projectiles.length === 0; i++) e.step()

    let sawGradient = false
    const origCreate = canvas.__ctx.createLinearGradient.bind(canvas.__ctx)
    canvas.__ctx.createLinearGradient = (x0, y0, x1, y1) => {
      sawGradient = true
      return origCreate(x0, y0, x1, y1)
    }
    r.draw(TICK_MS)
    if (e.projectiles.length > 0) {
      expect(sawGradient).toBe(true)
    }
  })

  it('反应事件会生成弹字', () => {
    const e = mkEngine()
    const r = makeRenderer(e)
    e.start()
    for (let i = 0; i < 400; i++) r.handleEvents(e.step())

    canvas.__ctx.calls.length = 0
    r.draw(TICK_MS)
    // 弹字会出现在 fillText 中
    const texts: string[] = canvas.__ctx.calls
      .filter((c) => c.startsWith('fillText:'))
      .map((c) => c.slice(10))
    expect(texts.length).toBeGreaterThan(0)
  })

  it('start 可重复调用而不叠加定时器，stop 后不再推进', async () => {
    const e = mkEngine()
    const r = makeRenderer(e)
    e.start()
    let steps = 0
    r.start(() => { steps++ })
    r.start(() => { steps++ })
    await new Promise((res) => setTimeout(res, TICK_MS * 5))
    const afterStart = steps
    expect(afterStart).toBeGreaterThan(0)
    // 5 个 tick 窗口内不应因叠加而异常高频
    expect(afterStart).toBeLessThanOrEqual(8)

    r.stop()
    const afterStop = steps
    await new Promise((res) => setTimeout(res, TICK_MS * 3))
    expect(steps).toBe(afterStop) // stop 后不再推进
  })

  it('视口随尺寸变化而缩放（横屏/竖屏都应可用）', () => {
    const e = mkEngine()
    const r = makeRenderer(e)
    for (const [w, h] of [[375, 667], [667, 375], [414, 896]] as const) {
      r.setViewport(w, h)
      expect(() => r.draw(TICK_MS)).not.toThrow()
    }
  })

  it('rAF 帧循环每帧重绘并重注册下一帧 —— 链路断裂的症状是画面停在首帧', () => {
    const e = mkEngine()
    const r = makeRenderer(e)
    e.start()

    // 生产（H5 与小程序）走的是 rAF 路径；node 测试环境没有 rAF，
    // 渲染器降级到 drawTimer，所以上面的用例守不住这半条路。
    // 这里装一个可手动驱动的假 rAF，守的是「帧循环本身」：
    //   1. 每帧都要真的 draw（否则画面静止、逻辑照跑）
    //   2. 每帧末尾都要重新注册下一帧（canvas.ts 的
    //      `this.rafId = requestAnimationFrame(frame)`）
    // 第 2 条此前的守卫是零：那行一旦被删或挪到 draw 之前，
    // 无头测试（直接调 draw）与 drawTimer 路径全部照绿，
    // 而真机表现是「战斗在打、画面冻在第一帧」——HUD 靠独立的
    // logicTimer 照样更新，极具迷惑性。
    const pending: Array<(ts: number) => void> = []
    let nextId = 1
    const g = globalThis as unknown as {
      requestAnimationFrame?: unknown
      cancelAnimationFrame?: unknown
    }
    const prevRaf = g.requestAnimationFrame
    const prevCaf = g.cancelAnimationFrame
    g.requestAnimationFrame = (cb: (ts: number) => void) => {
      pending.push(cb)
      return nextId++
    }
    g.cancelAnimationFrame = () => {}

    try {
      r.start(() => {})
      // start() 应当注册首帧
      expect(pending.length).toBe(1)

      const before = canvas.__ctx.calls.length
      // 手动驱动 5 帧。ts 从 1*16 起给非零递增值，
      // 避开 lastTs=0 的初始化帧（dt 会是 0）。
      for (let i = 1; i <= 5; i++) {
        const cb = pending.shift()
        expect(cb, `第 ${i} 帧没有待执行回调 —— 上一帧漏了重新注册`).toBeTruthy()
        cb!(i * 16)
        expect(
          pending.length,
          `第 ${i} 帧执行后没有重新注册下一帧 —— 画面将在此冻结`,
        ).toBeGreaterThanOrEqual(1)
      }

      // 5 帧每帧都产出绘制指令（背景填充至少一次/帧）
      const fills = canvas.__ctx.calls
        .slice(before)
        .filter((c) => c.startsWith('fillRect')).length
      expect(fills).toBeGreaterThanOrEqual(5)
    } finally {
      r.stop()
      g.requestAnimationFrame = prevRaf
      g.cancelAnimationFrame = prevCaf
    }
  })
})

// ==================== 分支覆盖：假引擎直驱 ====================
// 上面一组用真实引擎验证「整体能跑通」；下面一组用手工构造的引擎状态
// 直驱每个绘制分支 —— 冻结/眩晕/护盾/元素层数/地形种类等状态在真实对局
// 里受随机性支配，难以稳定复现，只有手搓状态才能逐分支断言。

/** 只填 BattleRenderer.draw 读取的字段的最小敌人（其余字段给中性值） */
function fakeEnemy(over: Partial<Enemy> = {}): Enemy {
  return {
    uid: 1,
    defId: 1,
    name: '假敌',
    category: 'normal',
    x: 100000n,
    y: 100000n,
    hp: 50n,
    maxHp: 100n,
    shield: 0n,
    armorPermille: 0n,
    flyHeight: 0,
    burrow: false,
    isBoss: false,
    resist: new Map(),
    stacks: new Map(),
    applyElement: () => {},
    speed: 1n,
    attack: 1n,
    attackRange: 0n,
    attackInterval: 0,
    attackCooldown: 0,
    frozenMs: 0,
    stunnedMs: 0,
    slowedMs: 0,
    amplifyPermille: 0n,
    knockback: 0n,
    armorShredMs: 0,
    armorShredPermille: 0n,
    dead: false,
    spawnProgress: 1,
    hitFlashMs: 0,
    ...over,
  }
}

/** 只填 draw 系列读取的字段的最小引擎（真实引擎的对局随机性在这里不需要） */
function fakeEngine(
  over: Partial<{
    enemies: Enemy[]
    projectiles: unknown[]
    terrains: Array<Record<string, unknown>>
    floats: FloatText[]
    heat: { cap: bigint; heat: bigint; overheated: boolean; overheatRemaining: number }
    skills: Array<Record<string, unknown>>
    baseHp: bigint
    baseHpMax: bigint
  }> = {},
): BattleEngine {
  return {
    enemies: [],
    projectiles: [],
    terrains: [],
    floats: [],
    heat: { cap: 100n, heat: 20n, overheated: false, overheatRemaining: 0 },
    skills: [],
    baseHp: 900n,
    baseHpMax: 1000n,
    ...over,
  } as unknown as BattleEngine
}

describe('BattleRenderer 分支覆盖（假引擎直驱绘制分支）', () => {
  let canvas: StubCanvas

  beforeEach(() => {
    canvas = makeCanvas()
  })
  afterEach(() => {
    vi.unstubAllGlobals()
  })

  function makeRenderer(engine: BattleEngine): BattleRenderer {
    return new BattleRenderer(canvas as unknown as HTMLCanvasElement, engine)
  }

  function popupTexts(): string[] {
    return canvas.__ctx.calls
      .filter((c) => c.startsWith('fillText:'))
      .map((c) => c.slice('fillText:'.length))
  }

  it('构造函数拒绝没有 getContext 方法的对象', () => {
    expect(() => new BattleRenderer({ width: 10, height: 10 } as any, mkEngine())).toThrow(
      'BattleRenderer 需要 canvas 元素',
    )
  })

  it('handleEvents：每种事件都按类型弹窗/震动，地形未知种类回退原文', () => {
    const r = makeRenderer(mkEngine())
    r.handleEvents([
      { type: 'reaction', reaction: 'steam_burst', x: 1n, y: 1n, damage: 1n, resisted: false },
      { type: 'reaction', reaction: 'superconduct', x: 2n, y: 2n, damage: 1n, resisted: true },
      { type: 'kill', x: 1n, y: 1n, boss: false },
      { type: 'kill', x: 1n, y: 1n, boss: true },
      { type: 'leak', damage: 5n },
      { type: 'wave_start', index: 1 },
      { type: 'terrain', kind: 'oil_drum', x: 1, y: 1 },
      { type: 'terrain', kind: 'not_a_terrain', x: 1, y: 1 },
      { type: 'overheat' },
      { type: 'won', score: 100, stars: 3 },
      { type: 'lost', score: 0 },
    ])
    r.draw(0)
    const texts = popupTexts()
    expect(texts).toContain('蒸汽爆发')
    expect(texts).toContain('第 2 波')
    expect(texts).toContain('油桶')
    expect(texts).toContain('not_a_terrain') // TERRAIN_NAME 未命中 → ?? 原文
    expect(texts).toContain('过热！')
    expect(texts).toContain('通关 3 星')
    expect(texts).toContain('防线失守')
    // 命中/被抗性两种弹窗色都要出现（resisted 分支；弹窗色在 fillText 时设置）
    expect(canvas.__ctx.textStyles).toContain('#7ee787')
    expect(canvas.__ctx.textStyles).toContain('#8b949e')
  })

  it('弹窗上限 8 条：最旧的被丢弃，超限不崩', () => {
    const r = makeRenderer(mkEngine())
    const events = Array.from({ length: 12 }, () => ({
      type: 'reaction' as const,
      reaction: 'steam_burst' as const,
      x: 1n,
      y: 1n,
      damage: 1n,
      resisted: false,
    }))
    r.handleEvents(events)
    r.draw(0)
    expect(popupTexts().filter((t) => t === '蒸汽爆发').length).toBe(8)
  })

  it('已过期弹窗在绘制时被移除（lifeMs <= 0 分支）', () => {
    const r = makeRenderer(mkEngine())
    r.handleEvents([
      { type: 'reaction', reaction: 'steam_burst', x: 1n, y: 1n, damage: 1n, resisted: false },
    ])
    r.draw(2000) // 弹窗 lifeMs 1200 - 2000 → 过期
    canvas.__ctx.calls.length = 0
    r.draw(0)
    expect(popupTexts()).not.toContain('蒸汽爆发')
  })

  it('六种敌人形状：圆 / 三角 / 菱形 / 八边形×2 / 六边形', () => {
    const enemies = [
      fakeEnemy({ uid: 1, category: 'normal' }),
      fakeEnemy({ uid: 2, category: 'flying' }),
      fakeEnemy({ uid: 3, category: 'ranged' }),
      fakeEnemy({ uid: 4, category: 'boss', isBoss: true }),
      fakeEnemy({ uid: 5, category: 'elite' }),
      fakeEnemy({ uid: 6, category: 'special' }),
    ]
    const r = makeRenderer(fakeEngine({ enemies }))
    r.draw(TICK_MS)
    const calls = canvas.__ctx.calls
    expect(calls).toContain('arc') // 普通敌人 = 圆
    // 5 个非普通敌人各自 closePath 一次（boss 与 elite 都走八边形）
    expect(calls.filter((c) => c === 'closePath').length).toBe(5)
    // BOSS 描边警示色，其余深色
    expect(canvas.__ctx.strokeStyles).toContain('#ff6b35')
    expect(canvas.__ctx.strokeStyles).toContain('#0d1117')
  })

  it('敌人状态分支：受击闪白 / 护盾环 / 冻结 / 眩晕 / 出生进度 / 飞行高度 / 元素层数截断', () => {
    const stacks = new Map<Element, bigint>([
      ['fire', 2n],
      ['kinetic', 5n], // 超过 4 层 → 绘制被夹到 4 格
    ])
    const e = fakeEnemy({
      uid: 1,
      hitFlashMs: 100,
      shield: 1n,
      frozenMs: 800,
      stunnedMs: 500,
      spawnProgress: 0.5,
      flyHeight: 10,
      stacks,
    })
    const r = makeRenderer(fakeEngine({ enemies: [e] }))
    r.draw(TICK_MS)
    // 受击闪白覆盖本体色
    expect(canvas.__ctx.fillStyles).toContain('#ffffff')
    // 护盾环
    expect(canvas.__ctx.strokeStyles).toContain('#58a6ff')
    // 冻结 / 眩晕覆盖层
    expect(canvas.__ctx.fillStyles).toContain('rgba(88,166,255,0.35)')
    expect(canvas.__ctx.fillStyles).toContain('rgba(255,211,61,0.3)')
    // 元素层数方块：fire 2 格，kinetic 5 层只画 4 格
    expect(canvas.__ctx.fillStyles.filter((s) => s === ELEMENT_COLOR.fire).length).toBe(2)
    expect(canvas.__ctx.fillStyles.filter((s) => s === ELEMENT_COLOR.kinetic).length).toBe(4)
    // 绘制结束后 globalAlpha 必须复位，否则后续实体全部半透明
    expect(canvas.__ctx.globalAlpha).toBe(1)
  })

  it('五种地形全部画出，各状态着色分支齐全', () => {
    const terrains = [
      { kind: 'oil_drum', x: 100, y: 100, param: 10, state: 'burning', angle: 0, charge: 0 },
      { kind: 'oil_drum', x: 150, y: 100, param: 10, state: 'idle', angle: 0, charge: 0 },
      { kind: 'tidal_gate', x: 200, y: 100, param: 10, state: 'closed', angle: 0, charge: 0 },
      { kind: 'tidal_gate', x: 250, y: 100, param: 10, state: 'open', angle: 0, charge: 0 },
      { kind: 'rotor_vane', x: 300, y: 100, param: 10, state: 'idle', angle: 45, charge: 0 },
      { kind: 'collapse_wall', x: 400, y: 100, param: 10, state: 'collapsed', angle: 0, charge: 0 },
      { kind: 'collapse_wall', x: 450, y: 100, param: 10, state: 'solid', angle: 0, charge: 0 },
      { kind: 'charge_tower', x: 500, y: 100, param: 10, state: 'idle', angle: 0, charge: 5 },
    ]
    const r = makeRenderer(fakeEngine({ terrains }))
    r.draw(TICK_MS)
    expect(canvas.__ctx.fillStyles).toContain('#ff6b35') // 燃烧中的油桶
    expect(canvas.__ctx.fillStyles).toContain('#6b5b3f') // 未引燃
    expect(canvas.__ctx.fillStyles).toContain('#58a6ff') // 关闭的潮汐闸
    expect(canvas.__ctx.fillStyles).toContain('rgba(88,166,255,0.25)') // 开启
    expect(canvas.__ctx.fillStyles).toContain('rgba(48,54,61,0.3)') // 已崩塌
    expect(canvas.__ctx.fillStyles).toContain('#6e7681') // 完好掩体
    expect(canvas.__ctx.fillStyles).toContain('#7ee787') // 蓄能塔
    // 旋转风障每个调用一次 rotate（3 片风叶用 moveTo/lineTo 画）
    expect(canvas.__ctx.calls.filter((c) => c === 'rotate').length).toBe(1)
  })

  it('飘字按剩余寿命绘制（engine.floats 消费）', () => {
    const floats: FloatText[] = [
      { x: 50000n, y: 50000n, text: '-50', color: '#ff6b35', lifeMs: 500, maxLifeMs: 1000, size: 20 },
    ]
    const r = makeRenderer(fakeEngine({ floats }))
    r.draw(TICK_MS)
    expect(canvas.__ctx.calls).toContain('fillText:-50')
  })

  it('HUD：热量高段警示黄、层数文案、冷却中的槽位画遮罩', () => {
    const skills = [
      { element: 'fire' as Element, cooldownRemaining: 400, cooldownMs: 800 },
      { element: 'ice' as Element, cooldownRemaining: 0, cooldownMs: 800 },
    ]
    const r = makeRenderer(
      fakeEngine({
        heat: { cap: 100n, heat: 90n, overheated: false, overheatRemaining: 0 },
        skills,
      }),
    )
    r.draw(TICK_MS)
    const texts = popupTexts()
    expect(texts.some((t) => t.startsWith('热量 90/100'))).toBe(true)
    expect(texts).toContain('元素层数 2 技能')
    expect(canvas.__ctx.fillStyles).toContain('#ffd33d') // pct > 0.8 → 黄
  })

  it('HUD：上限为 0 → pct=0 分支；过热态 → 倒计时文案', () => {
    const r = makeRenderer(
      fakeEngine({ heat: { cap: 0n, heat: 5n, overheated: true, overheatRemaining: 1234 } }),
    )
    r.draw(TICK_MS)
    const texts = popupTexts()
    expect(texts.some((t) => t.startsWith('热量 5/0'))).toBe(true)
    expect(texts.some((t) => t.startsWith('过热 '))).toBe(true)
    expect(canvas.__ctx.fillStyles).toContain('#ff6b35')
  })

  it('防线墙体颜色随血量三段变化（>0.5 / >0.2 / 其余）', () => {
    const cases = [
      [900n, 1000n, '#58a6ff'],
      [400n, 1000n, '#ffd33d'],
      [100n, 1000n, '#ff6b35'],
    ] as const
    for (const [hp, max, color] of cases) {
      canvas = makeCanvas()
      const r = makeRenderer(fakeEngine({ baseHp: hp, baseHpMax: max }))
      r.draw(TICK_MS)
      expect(canvas.__ctx.fillStyles).toContain(color)
    }
  })

  it('start：存在 rAF 时走帧循环（dt 被 Math.min 夹到上限），stop 取消帧回调', () => {
    const frames: Array<(ts: number) => void> = []
    vi.stubGlobal('requestAnimationFrame', (cb: (ts: number) => void) => {
      frames.push(cb)
      return frames.length as unknown as number
    })
    const cancelSpy = vi.fn()
    vi.stubGlobal('cancelAnimationFrame', cancelSpy)

    const r = makeRenderer(fakeEngine())
    r.start(() => {})
    expect(frames.length).toBe(1)
    // 首帧建立 lastTs（dt 被夹到 TICK_MS*4），随后一帧 dt 正常 —— Math.min 两侧分支
    frames[0]!(1000)
    frames[0]!(1000 + TICK_MS)
    expect(frames.length).toBe(3)

    r.stop()
    expect(cancelSpy).toHaveBeenCalledTimes(1)
    const before = frames.length
    frames[frames.length - 1]!(0) // stop 之后迟到的帧回调必须直接返回
    expect(frames.length).toBe(before)
  })

  it('时钟守卫：running 被置 false 后，仍挂着的逻辑/绘制定时器不再推进', async () => {
    // stop() 是同步的"running=false + clearInterval"，正常时序下守卫永远走不到
    // 假分支；这里直接翻转内部 running 模拟"定时器还在排队但已停"的防御场景，
    // 验证的正是这两个守卫存在的意义（与 stop 时序解耦的兜底）。
    const r = makeRenderer(fakeEngine())
    let ticks = 0
    r.start(() => {
      ticks++
    })
    await new Promise((res) => setTimeout(res, TICK_MS * 2 + 20))
    expect(ticks).toBeGreaterThan(0)

    ;(r as unknown as { running: boolean }).running = false
    const before = ticks
    await new Promise((res) => setTimeout(res, TICK_MS * 3))
    expect(ticks).toBe(before)
    r.stop() // 清掉两个定时器，避免污染下一个用例
  })
})
