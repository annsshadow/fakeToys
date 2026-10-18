/**
 * 渲染性能的回归绊线（单位：每帧 draw call / 每帧渐变对象数）。
 *
 * ## 为什么测这两个量
 *
 * Canvas 2D 在中低端设备上的开销几乎全部落在这两处：
 *
 *  1. **draw call** —— 每次 `fill/stroke/fillText` 都是一次状态机提交，
 *     小程序真机上单次提交的固定开销远大于顶点数本身。
 *     「同屏 N 个敌人 → 每个 6~10 次提交」是这类游戏的典型瓶颈。
 *  2. **渐变对象** —— `createLinearGradient` / `createRadialGradient`
 *     每次调用都在** Canvas 实现内部分配**一个渐变对象（ native side，
 *     不受 JS GC 控制，在部分实现里甚至要到下一次 GC 才回收）。
 *     把它们放进每帧循环 = 每帧 N 次 native 分配。
 *
 * 这两项都不依赖机器主频、不受后台负载影响，可以在 CI 里稳定断言 ——
 * 与 `game/perf_baseline.test.ts` 里那条墙钟绊线的定位不同：
 * **这里是计数器，是确定性的；那边墙钟是不确定的。**
 *
 * ## ⚠️ 本文件只统计**调用次数**，不测真实像素光栅化耗时
 *
 * 它能抓到「有人在 draw 里加了一层循环」「每帧多造 200 个渐变」，
 * 抓不到「某个渐变的半径开太大」—— 真实帧率仍需在真机上复测。
 *
 * ## 真机复测帧率的做法
 *
 * 1. `npm run dev:mp-weixin`，用微信开发者工具打开，跑一场后期关卡
 *    （弹幕密度最高的那种，默认构筑的怠速状态测不出东西）。
 * 2. 开发者工具 → 调试器 → Performance / 或小程序「开发调试」面板里的
 *    帧率浮层，读 **每帧耗时与掉帧次数**，重点看**最坏帧**而不是均值：
 *    卡顿的体感来自最坏帧。
 * 3. 对照量：本文件打印的 `每帧 draw call` 与 `每帧渐变对象`。
 *    真机上若某个场景的掉帧与 draw call 峰值在时间轴上重合，
 *    就说明瓶颈还在绘制提交侧；若 draw call 平稳而掉帧仍在，
 *    就要去看 GC（长按 GC 尖峰）而不是继续画。
 * 4. 预算可用环境变量临时放宽，便于本地对比：
 *    `PERF_DRAW_BUDGET=... PERF_STRESS_DRAW_BUDGET=... npx vitest run src/render/perf_frame.test.ts`
 *
 * ## 结算口径
 *
 * 「中低端设备帧率平稳」这个目标，本文件只能证明到
 * **「每帧的提交量与分配量是常量级、且随对象数线性增长」** 为止，
 * 不能证明真机一定满帧 —— 那需要上面的第 2 步。
 */
import { describe, it, expect, beforeAll, afterAll } from 'vitest'
import fixture from '@vectors/smoke_levels.json'
import { BattleRenderer } from './canvas'
import { BattleEngine, TICK_MS } from '../game/engine'
import { equippedFromSnapshot, type BuildSnapshot } from '../game/replay'
import type { Element } from '../game/elements'
import { ACTIVE_SLOTS } from '../game/heatmap'
import { defaultAttacker } from '../game/damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from '../game/types'

/**
 * 计数型打桩上下文：只累加调用次数与分类计数，不做任何绘制。
 *
 * ⚠️ 与 `canvas.test.ts` 的 StubContext 不同：那里用 `calls: string[]`
 * 记录明细以备断言走哪个分支（测试量小）；这里要跑上万帧，
 * push 进数组会让测试自己成为内存瓶颈 —— 必须只留计数器。
 */
class CountingContext {
  /** 提交类调用（真机上的 draw call） */
  fill = 0
  fillRect = 0
  stroke = 0
  strokeRect = 0
  fillText = 0
  strokeText = 0
  /** 建流 / 路径 */
  beginPath = 0
  moveTo = 0
  lineTo = 0
  arc = 0
  /** 渐变对象（native 侧分配，最贵的一项） */
  linearGradient = 0
  radialGradient = 0
  /** 位图提交（`drawImage`） */
  drawImage = 0

  fillStyle: unknown = '#000'
  strokeStyle: unknown = '#000'
  lineWidth = 1
  globalAlpha = 1
  font = ''
  textAlign: unknown = 'left'

  save() {}
  restore() {}
  translate() {}
  scale() {}
  rotate() {}
  setTransform() {}
  clearRect() {}
  fillRectStub() {}
  beginPathStub() {}

  fillRectCall() { this.fillRect++ }
  fillCall() { this.fill++ }
  strokeCall() { this.stroke++ }
  strokeRectCall() { this.strokeRect++ }
  fillTextCall() { this.fillText++ }
  strokeTextCall() { this.strokeText++ }
  beginPathCall() { this.beginPath++ }
  drawImageCall() { this.drawImage++ }
  moveToCall() { this.moveTo++ }
  lineToCall() { this.lineTo++ }
  arcCall() { this.arc++ }
  rect() {}
  closePath() {}
  measureText(t: string) { return { width: t.length * 6 } }

  createLinearGradient() {
    this.linearGradient++
    return { addColorStop: () => {} }
  }
  createRadialGradient() {
    this.radialGradient++
    return { addColorStop: () => {} }
  }

  /** draw call 合计 */
  get drawCalls(): number {
    return (
      this.fill + this.fillRect + this.stroke + this.strokeRect +
      this.fillText + this.strokeText + this.drawImage
    )
  }
  /** 渐变对象合计 */
  get gradients(): number {
    return this.linearGradient + this.radialGradient
  }
}

/**
 * 上面那堆 `xxxCall()` 是为了让 CountingContext 保持「属性即计数器」的形态
 * 又不与 CanvasRenderingContext2D 的方法名冲突。真正的适配在这里发生：
 * 用 `as unknown as CanvasRenderingContext2D` 让渲染层把它当上下文用，
 * 方法名由下面这组赋值对齐。
 */
function makeCtx(): { ctx: CanvasRenderingContext2D; c: CountingContext } {
  const c = new CountingContext()
  const bridge = {
    get fillStyle() { return c.fillStyle },
    set fillStyle(v) { c.fillStyle = v },
    get strokeStyle() { return c.strokeStyle },
    set strokeStyle(v) { c.strokeStyle = v },
    get lineWidth() { return c.lineWidth },
    set lineWidth(v) { c.lineWidth = v },
    get globalAlpha() { return c.globalAlpha },
    set globalAlpha(v) { c.globalAlpha = v },
    get font() { return c.font },
    set font(v) { c.font = v },
    get textAlign() { return c.textAlign },
    set textAlign(v) { c.textAlign = v },
    save: () => c.save(),
    restore: () => c.restore(),
    translate: () => c.translate(),
    scale: () => c.scale(),
    rotate: () => c.rotate(),
    setTransform: () => c.setTransform(),
    clearRect: () => c.clearRect(),
    fillRect: () => c.fillRectCall(),
    strokeRect: () => c.strokeRectCall(),
    beginPath: () => c.beginPathCall(),
    closePath: () => c.closePath(),
    moveTo: () => c.moveToCall(),
    lineTo: () => c.lineToCall(),
    arc: () => c.arcCall(),
    rect: () => c.rect(),
    fill: () => c.fillCall(),
    stroke: () => c.strokeCall(),
    fillText: () => c.fillTextCall(),
    strokeText: () => c.strokeTextCall(),
    measureText: (t: string) => c.measureText(t),
    createLinearGradient: () => c.createLinearGradient(),
    createRadialGradient: () => c.createRadialGradient(),
    drawImage: () => c.drawImageCall(),
  }
  return { ctx: bridge as unknown as CanvasRenderingContext2D, c }
}

/** 打桩 canvas，满足 BattleRenderer 构造时对 getContext 的探测 */
function makeCanvas(ctx: CanvasRenderingContext2D) {
  return {
    width: 750,
    height: 1334,
    style: {} as Record<string, string>,
    getContext: () => ctx,
  }
}

const levels = (fixture.levels as unknown as GeneratedLevel[]).slice().sort((a, b) => a.id - b.id)
const enemies = new Map<number, EnemyDef>(
  (fixture.enemies as unknown as EnemyDef[]).map((e) => [e.id, e]),
)
const skills = new Map<number, SkillDef>(
  [
    ...(fixture.skills as unknown as SkillDef[]),
    ...(fixture.composite_skills as unknown as SkillDef[]),
  ].map((s) => [s.id, s]),
)

function buildSnapshot(): BuildSnapshot {
  const active = [...skills.values()]
    .filter((s) => s.kind === 'active')
    .sort((a, b) => a.id - b.id)
    .slice(0, ACTIVE_SLOTS)
  const map: Record<string, unknown> = {}
  active.forEach((s, i) => {
    map[String(s.id)] = {
      id: s.id, name: s.name, family: 't', element: s.element, kind: s.kind, slot: i, level: 1,
    }
  })
  return { skills: map, attacker: { attack: 0 }, active_slots: ACTIVE_SLOTS } as unknown as BuildSnapshot
}

function mkEngine(lv: GeneratedLevel): BattleEngine {
  return new BattleEngine({
    level: lv,
    enemies,
    skills,
    equipped: equippedFromSnapshot(buildSnapshot(), skills),
    attacker: defaultAttacker(),
    seed: 12345,
    activeSlots: ACTIVE_SLOTS,
  })
}

/** 绘制帧率：rAF 的目标间隔（60fps） */
const FRAME_MS = 1000 / 60

/**
 * 最小离屏画布工厂。
 *
 * ## 为什么探针必须提供它
 *
 * `canvas.ts` 的静态背景层与光晕精灵都依赖**离屏画布**
 * （H5 走 `document.createElement`、小程序走 `wx.createOffscreenCanvas`）。
 * 而本文件的 test.environment 是 `node` —— 没有 `document`。
 *
 * 不注入的话测到的永远是**降级路径**，也就是优化前的行为，
 * 数字会好看成「几乎没优化」。真机上跑的恰恰是被优化的那条路。
 * 所以这里补一层假的 `document.createElement('canvas')`，
 * 它返回的 ctx 与打桩主上下文 API 同构（这正是渲染层能在两端跑的前提）。
 */
function makeOffscreen(width: number, height: number) {
  const noop = () => {}
  const ctx = {
    fillStyle: '', strokeStyle: '', lineWidth: 1, globalAlpha: 1, font: '', textAlign: '',
    save: noop, restore: noop, translate: noop, scale: noop, rotate: noop, setTransform: noop,
    clearRect: noop, fillRect: noop, strokeRect: noop, beginPath: noop, closePath: noop,
    moveTo: noop, lineTo: noop, arc: noop, rect: noop, fill: noop, stroke: noop,
    fillText: noop, strokeText: noop, drawImage: noop,
    measureText: (t: string) => ({ width: t.length * 6 }),
    createLinearGradient: () => ({ addColorStop: noop }),
    createRadialGradient: () => ({ addColorStop: noop }),
  }
  return { width, height, style: {} as Record<string, string>, getContext: () => ctx }
}

/**
 * 满载压力引擎。
 *
 * ## 为什么不用 fixtures 里的真实关卡
 *
 * 真实关卡的同屏峰值只有 **4 敌 / 9 弹**（见上一组测试的实测输出）——
 * 那是「4 技能 × 800ms 冷却」这一默认构筑的必然结果，
 * 用它来衡量上限等于拿怠速状态去标定跑车的极速。
 *
 * 而questers 现场报告的是**中低端设备上掉帧**，那恰恰发生在
 * 后期构筑（冷却缩减、多弹道、穿透+弹射）把弹幕堆起来的时刻 ——
 * 玩家实际会遇到的最坏情况，fixture 里根本不存在。
 *
 * 所以这里手搓一个可控的高负载场：
 *   - 敌人：血极厚、走得极慢 → 保证长时间停留在场上不被清掉
 *   - 技能：冷却极短、热量为 0、弹速慢 → 让大量弹丸长时间在飞
 */
function mkStressEngine(): BattleEngine {
  const tankDef: EnemyDef = {
    id: 9001, code: 'stress_tank', name: '压力怪', category: 'normal',
    hp: 50_000_000, speed: 150, armor: 0, shield_hp: 0,
    attack: 0, attack_range: 0, attack_interval: 0,
    fly_height: 0, burrow: false, is_boss: false,
    resist: { fire: 0, ice: 0, lightning: 0, corrosion: 0, kinetic: 0 },
    descr: '',
  }
  const rapidDef: SkillDef = {
    id: 9001, code: 'stress_rapid', name: '压力弹', family: 't', element: 'fire' as Element,
    kind: 'active', descr: '', base_damage: 1, heat_cost: 0, cooldown_ms: 50,
    pierce: 0, aoe_radius: 0, apply_element: 'fire', apply_stacks: 1,
    // 慢到要飞很久才出界 → 同屏弹丸数持续堆高
    projectile_speed: 12_000, chain: 0, unlock_level: 1,
  }
  const lv: GeneratedLevel = {
    id: 9001, chapter: 1, name: '压力测试', seed: '1', base_hp: 100_000_000, wave_count: 1,
    difficulty: 1000, energy_cost: 0, element_cap: 3, armor_permille: 0,
    max_reaction_tier: 2, is_boss: false, star_targets: [100, 200, 300], max_score: 500,
    terrain: [],
    // 一波 30 只、每 tick 出一只、伤害打到几千万血上——同屏自然堆满
    waves: [{ wave_index: 0, spawns: [{ enemy_id: 9001, count: 30, interval: 50, delay: 0 }] }],
  }
  const em = new Map<number, EnemyDef>([[9001, tankDef]])
  const sm = new Map<number, SkillDef>([[9001, rapidDef]])
  return new BattleEngine({
    level: lv,
    enemies: em,
    skills: sm,
    equipped: equippedFromSnapshot(
      { skills: { '9001': { id: 9001, name: '压力弹', family: 't', element: 'fire', kind: 'active', slot: 0, level: 1 } }, attacker: { attack: 0 }, active_slots: ACTIVE_SLOTS } as unknown as BuildSnapshot,
      sm,
    ),
    attacker: defaultAttacker(),
    seed: 12345,
    activeSlots: ACTIVE_SLOTS,
  })
}

interface FrameStat {
  drawCalls: number
  gradients: number
  enemies: number
  projectiles: number
}

/**
 * 以「60fps 绘制 + 20Hz 逻辑」的真实节律跑满一关，逐帧采样。
 *
 * ⚠️ 为什么不用 `renderer.start()`：那会真的起 setInterval / rAF，
 * 把测试的墙钟时间与这两者的调度绑在一起 —— 既慢又不稳定。
 * 这里手动按帧推进，帧数确定、统计确定。
 */
function sampleEngine(eng: BattleEngine, maxFrames: number): FrameStat[] {
  const { ctx, c } = makeCtx()
  const r = new BattleRenderer(makeCanvas(ctx) as never, eng)
  r.setViewport(375, 667)

  eng.start()
  const frames: FrameStat[] = []
  let acc = 0
  let prevDraw = -1
  let prevGrad = -1

  for (let f = 0; f < maxFrames; f++) {
    // 逻辑：20Hz 固定步长（与 rAF 解耦，见 canvas.ts 的 start 注释）
    acc += FRAME_MS
    while (acc >= TICK_MS) {
      if (eng.phase === 'card_select') eng.skipCards()
      if (eng.phase === 'won' || eng.phase === 'lost') break
      const events = eng.step()
      r.handleEvents(events)
      acc -= TICK_MS
    }
    if (eng.phase === 'won' || eng.phase === 'lost') break

    r.draw(FRAME_MS)

    // 用累计计数器的增量归因到本帧（比在 draw 里插桩更准，且无侵入）
    const d = c.drawCalls
    const g = c.gradients
    frames.push({
      drawCalls: prevDraw < 0 ? d : d - prevDraw,
      gradients: prevGrad < 0 ? g : g - prevGrad,
      enemies: eng.enemies.length,
      projectiles: eng.projectiles.length,
    })
    prevDraw = d
    prevGrad = g
  }
  return frames
}

function sample(lv: GeneratedLevel, maxFrames = 3600): FrameStat[] {
  return sampleEngine(mkEngine(lv), maxFrames)
}

function pct(sorted: number[], p: number): number {
  if (sorted.length === 0) return 0
  const i = Math.min(sorted.length - 1, Math.floor((sorted.length * p) / 100))
  return sorted[i]
}

describe('渲染性能绊线', () => {
  // ⚠️ 全局副作用必须在块结束后还原，否则会漏到同进程里的其它测试文件：
  // 「有没有 document」正是渲染层决定走缓存还是走降级的依据。
  let prevDoc: unknown
  beforeAll(() => {
    prevDoc = (globalThis as Record<string, unknown>).document
    ;(globalThis as Record<string, unknown>).document = {
      createElement: (tag: string) => (tag === 'canvas' ? makeOffscreen(1, 1) : {}),
    }
  })
  afterAll(() => {
    if (prevDoc === undefined) {
      delete (globalThis as Record<string, unknown>).document
    } else {
      ;(globalThis as Record<string, unknown>).document = prevDoc
    }
  })

  it('满载场景（数十敌人 + 数十弹丸）的每帧成本不超出预算', () => {
    const frames = sampleEngine(mkStressEngine(), 900)
    const draws = frames.map((f) => f.drawCalls).sort((a, b) => a - b)
    const grads = frames.map((f) => f.gradients)
    const maxDraw = draws[draws.length - 1]
    const maxGrad = Math.max(...grads)
    const peakEnemies = Math.max(...frames.map((f) => f.enemies))
    const peakProj = Math.max(...frames.map((f) => f.projectiles))

    console.log(`  满载同屏峰值        敌 ${peakEnemies} / 弹丸 ${peakProj}`)
    console.log(`  每帧 draw call      平均 ${(draws.reduce((a, b) => a + b, 0) / draws.length).toFixed(1)} / 最大 ${maxDraw}`)
    console.log(`  每帧渐变对象        最大 ${maxGrad}`)

    // 前提守卫：这个场景的意义在于「真的堆到高负载」。
    // 若哪天 fixtures 或技能冷却改动让它退化成个位数，
    // 这条测试就只是在测空场 —— 必须先自证负载够。
    expect(peakEnemies).toBeGreaterThanOrEqual(20)
    expect(peakProj).toBeGreaterThanOrEqual(8)

    expect(maxGrad).toBe(0)
    expect(maxDraw).toBeLessThanOrEqual(STRESS_DRAW_BUDGET)
  }, 300_000)

  it('最坏关卡的每帧 draw call 与渐变对象数保持在预算内', () => {
    // 第 100 关是 fixture 里敌人数最多的一关（实测 84 敌），是最坏负载。
    const lv = levels.find((l) => l.id === 100) ?? levels[levels.length - 1]
    const frames = sample(lv)
    const draws = frames.map((f) => f.drawCalls).sort((a, b) => a - b)
    const grads = frames.map((f) => f.gradients).sort((a, b) => a - b)
    const avgDraw = draws.reduce((a, b) => a + b, 0) / Math.max(1, draws.length)
    const p99Draw = pct(draws, 99)
    const maxDraw = draws[draws.length - 1]
    const avgGrad = grads.reduce((a, b) => a + b, 0) / Math.max(1, grads.length)
    const maxGrad = grads[grads.length - 1]
    const peakEnemies = Math.max(...frames.map((f) => f.enemies))
    const peakProj = Math.max(...frames.map((f) => f.projectiles))

    console.log(`  关卡                L${lv.id}（${frames.length} 帧）`)
    console.log(`  同屏峰值            敌 ${peakEnemies} / 弹丸 ${peakProj}`)
    console.log(`  每帧 draw call      平均 ${avgDraw.toFixed(1)} / p99 ${p99Draw} / 最大 ${maxDraw}`)
    console.log(`  每帧渐变对象        平均 ${avgGrad.toFixed(1)} / 最大 ${maxGrad}`)

    // 前提守卫：没有帧可统计时下面所有断言都会退化成 0 <= X，**恒真**。
    // 「断言恒真」在本项目反复出事 —— 必须先确认真的测到了东西。
    expect(frames.length).toBeGreaterThan(0)
    expect(maxDraw).toBeGreaterThan(0)

    // 绊线取当前实测值的宽松上界，目的是抓数量级劣化（比如有人在 draw
    // 里加了对每个敌人的嵌套循环），而不是抓百分之几十的波动。
    expect(maxDraw).toBeLessThanOrEqual(DRAW_CALL_BUDGET)
    expect(maxGrad).toBeLessThanOrEqual(GRADIENT_BUDGET)
  }, 300_000)
})

/**
 * 每帧 draw call 预算（真实最坏关卡）。
 *
 * 定标（第 158 轮实测，L100 同屏 敌 4 + 弹丸 9）：
 *   - 优化前：平均 92.8 / 最大 **130**
 *   - 优化后：平均 54.1 / 最大 **88**
 *
 * 预算取 120 —— 相对实测留出一档余量，但足以拦住任何数量级的劣化。
 */
const DRAW_CALL_BUDGET = Number(process.env.PERF_DRAW_BUDGET ?? 120)

/**
 * 每帧渐变对象预算。
 *
 * 渐变是 native 侧分配，且**与对象数成正比**（优化前每个弹丸 2 个）。
 * 目标是把静态渐变全部一次性烘焙成精灵，让动态部分恒为 0。
 *
 * ⚠️ 这个断言是**硬约束**：任何 `createLinearGradient` / `createRadialGradient`
 * 回到每帧循环里都会让它变红 —— 这正是第 158 轮要根治的那个问题。
 */
const GRADIENT_BUDGET = Number(process.env.PERF_GRADIENT_BUDGET ?? 0)

/**
 * 满载场景的每帧 draw call 预算。
 *
 * 定标（第 158 轮实测，同一压测场景：敌 30 + 弹丸 288）：
 *   - 优化前：平均 730.4 / 最大 **1337**，其中渐变对象峰值 **578/帧**
 *   - 优化后：平均 320.9 / 最大 **622**，渐变 **0**
 *
 * 预算取 700（相对实测峰值约 12% 余量）。它抓的是「有人在 per-object
 * 绘制里加了一层」这类改动 —— 那会让这个数字立刻跳到 1300 附近。
 */
const STRESS_DRAW_BUDGET = Number(process.env.PERF_STRESS_DRAW_BUDGET ?? 700)
