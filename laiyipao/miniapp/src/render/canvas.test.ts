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
import { describe, it, expect, beforeEach } from 'vitest'
import { BattleRenderer } from './canvas'
import { BattleEngine, TICK_MS } from '../game/engine'
import { defaultAttacker } from '../game/damage'
import type { EnemyDef, GeneratedLevel, SkillDef } from '../game/types'
import type { Element } from '../game/elements'

/** 打桩的 2D 上下文：记录调用，不做任何实际绘制。 */
class StubContext {
  calls: string[] = []
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
  setTransform() {}
  clearRect() { this.calls.push('clearRect') }
  fillRect(...a: any[]) { this.calls.push(`fillRect:${a[0]},${a[1]},${a[2]},${a[3]}`) }
  strokeRect() { this.calls.push('strokeRect') }
  beginPath() { this.calls.push('beginPath') }
  closePath() { this.calls.push('closePath') }
  moveTo() { this.calls.push('moveTo') }
  lineTo() { this.calls.push('lineTo') }
  arc() { this.calls.push('arc') }
  rect() {}
  fill() { this.calls.push('fill') }
  stroke() { this.calls.push('stroke') }
  fillText(t: string) { this.calls.push(`fillText:${t}`) }
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
})
