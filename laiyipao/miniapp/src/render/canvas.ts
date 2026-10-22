/**
 * Canvas 2D 渲染层。
 *
 * 职责边界（重要）：
 *  - 引擎（game/engine.ts）只管状态与事件，零绘制代码
 *  - 渲染层只读状态 + 消费事件，**绝不能修改引擎状态**
 *
 * 这样做的收益：同一份内核能同时跑在微信小程序 canvas、H5 canvas、
 * 以及 Node 里的无头测试中；改画面不影响战斗逻辑，反之亦然。
 *
 * 美术：100% Canvas 程序化几何图形，零外部素材（原创性要求，见 GAME_DESIGN §10）。
 */

import { TICK_MS, type BattleEngine } from '../game/engine'
import type { BattleEvent } from '../game/engine'
import { ELEMENT_COLOR, ELEMENT_ORDER, REACTIONS, type Element } from '../game/elements'
import { TERRAIN_NAME } from '../game/terrain'
import { BASE_X, BASE_Y, FIELD_H, FIELD_W } from '../game/terrain'
import type { Enemy, Projectile } from '../game/types'

/**
 * 性能常量（第 158 轮渲染性能整改）。
 *
 * ## 为什么集中在文件头
 *
 * 渲染层此前把「渐变、颜色、格子数」这些成本项散在各 draw 方法里，
 * 于是每次改一处都要回到绘制代码里去数调用次数。集中之后，
 * 下面这些值的调整不再需要碰任何绘制逻辑。
 */

/** 背景网格的格子数（逻辑空间被均分的行/列数） */
const GRID_DIVISIONS = 10

/**
 * 战斗倍速档位（2026-10-22，测试便利）。
 *
 * ## 为什么倍速只能动「墙钟」，不能动 TICK_MS
 *
 * 引擎是定步长的：`step()` 一次推进 `TICK_MS`（50ms）的模拟时间，
 * 事件序列、命中帧、回放哈希全部由「第几次 step」决定。
 * 若把 TICK_MS 改成 50/8 = 6.25ms 来提速，那是**改模拟步长** ——
 * 冷却、移动、伤害结算的离散化都变了，同一份输入会跑出另一条事件序列，
 * I-6 的「任何人可重放验真」当场失效。
 *
 * 所以倍速只改「每墙钟秒跑多少次 step」：TICK_MS 恒为 50，
 * 8× 就是每秒 160 步。模拟轨迹与 1× 逐 tick 完全一致，
 * 只是压缩了等待时间。
 */
export const BATTLE_SPEED_STEPS = [1, 2, 4, 8] as const

/** 倍速上限。改这里之前先读 `BATTLE_SPEED_STEPS` 上方那段。 */
export const MAX_BATTLE_SPEED = 8

/**
 * 把外部传入的倍速钳到合法区间。
 *
 * 非有限值 / NaN / 0 / 负数一律回落到 1 —— 倍速是**加速**，
 * 「0.5× 慢放」不在需求里，静默当成 1× 比让定时器除出 Infinity 好。
 */
export function normalizeSpeed(mult: number): number {
  if (!Number.isFinite(mult) || mult <= 1) return 1
  return Math.min(mult, MAX_BATTLE_SPEED)
}

/**
 * 逻辑时钟的实际间隔（ms）。8× ⇒ 6.25ms。
 *
 * ⚠️ 平台下限：浏览器把定时器钳到 ~4ms，微信小程序基础库的
 * setInterval 粒度更粗（实测 ~10ms 起）。因此 8× 在 H5 上能跑满，
 * 在真机上可能只到 3~5× —— 它是**测试辅助**，不承诺精确倍率。
 */
export function logicIntervalMs(speed: number): number {
  return Math.max(1, TICK_MS / normalizeSpeed(speed))
}

/**
 * 剔除余量（逻辑单位）。敌人在这一步被视为半径 ~30 的圆，
 * 留一点外扩避免边缘上的敌人在完全离开画面前被闪断。
 */
const CULL_MARGIN = 48

// ---------- 离屏图层与精灵缓存 ----------

/**
 * 取一块可离屏绘制的画布。
 *
 * ## 为什么要它
 *
 * 静态内容（背景渐变 + 网格 + 暗角）原本**每帧重画一遍**：
 * 2 个渐变对象 + 18 次 `stroke` + 3 次全屏 `fillRect`。
 * 这些内容从开局到结算**一个像素都不会变**，是全帧最典型的重复劳动。
 * 烘焙到离屏图层后每帧只剩 1 次 `drawImage`。
 *
 * ## 平台差异
 *
 * H5 有 `document`，微信小程序要 `wx.createOffscreenCanvas`。
 * 两者拿到的 ctx API 同构（这正是本文件能在两端跑的前提），
 * 所以在 Individually 适配后对上层是同一个东西。
 *
 * ⚠️ **拿不到就返回 null，调用方必须走降级路径** ——
 * 无头测试环境（Node）既没有 document 也没有 uni，
 * 而「渲染层在这种环境下崩掉」是不可接受的。
 */
function acquireLayer(
  width: number,
  height: number,
): { canvas: unknown; ctx: CanvasRenderingContext2D } | null {
  try {
    const g = globalThis as Record<string, any>
    const makers: Array<() => any> = [
      () => g.uni?.createOffscreenCanvas?.({ type: '2d', width, height }),
      () => g.wx?.createOffscreenCanvas?.({ type: '2d', width, height }),
    ]
    if (typeof g.document?.createElement === 'function') {
      makers.push(() => {
        const c = g.document.createElement('canvas')
        c.width = width
        c.height = height
        return c
      })
    }
    for (const make of makers) {
      const canvas = make()
      if (!canvas) continue
      const ctx = canvas.getContext?.('2d')
      if (ctx) return { canvas, ctx: ctx as CanvasRenderingContext2D }
    }
  } catch {
    // 任何异常都降级 —— 渲染层不能因为拿不到加速手段就不画画
  }
  return null
}

/**
 * 径向光晕精灵的缓存。
 *
 * ## 为什么预烘焙
 *
 * 径向渐变（`createRadialGradient`）在弹丸光晕、油桶火焰、蓄能塔、
 * 旋翼上被**每帧重建**，且 4 处里有 3 处的形状/半径是常量 ——
 * 每次调用都在 Canvas 实现侧分配一个渐变对象，再由光栅化器重算色轮。
 *
 * 这些光晕只由「颜色 + 半径」决定，而二者都来自一张很短的取值表
 * （5 种元素色 + 4 种地形色）。烘焙一次、之后每帧只 `drawImage`。
 *
 * @param color 中心色（CSS 色串）
 * @param radius 精灵外半径（**像素**）
 * @param innerRatio 实心核心占半径的比例，0 = 纯渐变
 */
function makeRadialSprite(
  color: string,
  radius: number,
  innerRatio = 0,
): { canvas: unknown; ctx: CanvasRenderingContext2D } | null {
  const size = Math.max(2, Math.ceil(radius * 2))
  const layer = acquireLayer(size, size)
  if (!layer) return null
  const { ctx } = layer
  const c = size / 2
  const grad = ctx.createRadialGradient(c, c, Math.max(0.5, radius * innerRatio), c, c, radius)
  grad.addColorStop(0, color)
  grad.addColorStop(1, 'rgba(0,0,0,0)')
  ctx.fillStyle = grad
  ctx.fillRect(0, 0, size, size)
  return layer
}

/** 逻辑坐标 → 画布坐标 */
export interface Viewport {
  width: number
  height: number
  scale: number
  offsetX: number
  offsetY: number
}

export function makeViewport(width: number, height: number): Viewport {
  // 等比缩放并居中，逻辑空间是 1000×1000
  const scale = Math.min(width / FIELD_W, height / FIELD_H)
  return {
    width,
    height,
    scale,
    offsetX: (width - FIELD_W * scale) / 2,
    offsetY: (height - FIELD_H * scale) / 2,
  }
}

export function toCanvasX(vp: Viewport, fixedX: bigint): number {
  return vp.offsetX + Number(fixedX) / 1000 * vp.scale
}

export function toCanvasY(vp: Viewport, fixedY: bigint): number {
  return vp.offsetY + Number(fixedY) / 1000 * vp.scale
}

export class BattleRenderer {
  private ctx: CanvasRenderingContext2D
  private vp: Viewport
  private engine: BattleEngine
  /** 屏幕震动（BOSS 登场、爆炸） */
  private shake = 0
  /** 反应文字弹窗 */
  private popups: Array<{ text: string; color: string; lifeMs: number; x: number; y: number }> = []
  private rafId: number | null = null
  /** 逻辑时钟（setInterval）。与页面可见性无关，战斗不会因切后台停摆。 */
  private logicTimer: number | null = null
  /** 绘制时钟在无 rAF 环境下的降级定时器 */
  private drawTimer: number | null = null
  private lastTs = 0
  private running = false
  /** 逻辑时钟倍速（1 = 实时）。只影响墙钟，见 `BATTLE_SPEED_STEPS`。 */
  private speed = 1
  /** `start()` 收到的逻辑回调 —— 换挡重启定时器时要复用它。 */
  private loopFn: ((dtMs: number) => void) | null = null

  /** 传给构造函数的画布。只用来推算设备像素比（dpr），不参与绘制。 */
  private readonly canvasRef: HTMLCanvasElement | { width: number; height: number }

  /**
   * 静态背景层（渐变 + 网格 + 暗角）的离屏缓存。
   *
   * `null` 表示「本环境拿不到离屏画布，已放弃加速」—— 一旦判定为
   * 拿不到就不再重试（避免每帧都白白执行一次 acquireLayer 的异常路径）。
   */
  private bgLayer: { canvas: unknown; ctx: CanvasRenderingContext2D } | null = null
  private bgLayerFailed = false
  /** 背景层对应的视口指纹：`宽×高×dpr`。变了才重建。 */
  private bgKey = ''

  /**
   * 渐变精灵缓存（key → 精灵）。
   *
   * 见 `makeRadialSprite`：这些都是「每帧都在重建的不变量」。
   * 用一次性 Map 而不是散落的字段，是为了让「新增一种光晕」不用再
   * 加一个成员 —— 那是这类优化最容易重新退化的地方。
   */
  private sprites = new Map<string, { canvas: unknown } | null>()

  /** 字串缓存：`fontPx -> string`。设置 ctx.font 会触发一次字体解析，
   *  而它的参数只取决于 viewport.scale —— 没必要每帧每个字都重算一遍。 */
  private fontCache = new Map<number, string>()

  constructor(
    canvas: HTMLCanvasElement | { width: number; height: number },
    engine: BattleEngine,
  ) {
    this.engine = engine
    this.canvasRef = canvas
    if ('getContext' in canvas) {
      const c = canvas as HTMLCanvasElement
      // 小程序 canvas 2d 与 H5 canvas 的 API 同构
      this.ctx = (c as unknown as { getContext(t: string): CanvasRenderingContext2D }).getContext(
        '2d',
      )!
      this.vp = makeViewport(c.width, c.height)
    } else {
      throw new Error('BattleRenderer 需要 canvas 元素')
    }
  }

  setViewport(width: number, height: number): void {
    this.vp = makeViewport(width, height)
    this.fontCache.clear()
  }

  /** 设备像素比：实际画布像素 ÷ CSS 视口。拿不到时按 1 处理。 */
  private get dpr(): number {
    const cssW = this.vp.width
    if (cssW <= 0) return 1
    const r = this.canvasRef.width / cssW
    return Number.isFinite(r) && r > 0 ? r : 1
  }

  /** 取精灵；同参数只烘焙一次。拿不到离屏画布时返回 null（调用方降级）。 */
  private sprite(key: string, make: () => { canvas: unknown } | null): { canvas: unknown } | null {
    if (this.sprites.has(key)) return this.sprites.get(key)!
    const s = make()
    this.sprites.set(key, s)
    return s
  }

  /** 缓存 `bold {px}px sans-serif` 这类字串，避免每个字都让 ctx 重新解析字体 */
  private fontOf(px: number, bold = false): string {
    const key = Math.round(px * 100) * (bold ? 2 : 1)
    let f = this.fontCache.get(key)
    if (f === undefined) {
      f = `${bold ? 'bold ' : ''}${px}px sans-serif`
      this.fontCache.set(key, f)
    }
    return f
  }

  /** 消费一帧事件（由外部在固定步长循环中调用） */
  handleEvents(events: BattleEvent[]): void {
    for (const e of events) {
      switch (e.type) {
        case 'reaction': {
          const spec = REACTIONS[e.reaction]
          this.popups.push({
            text: spec.name,
            color: e.resisted ? '#8b949e' : '#7ee787',
            lifeMs: 1200,
            x: Number(e.x) / 1000,
            y: Number(e.y) / 1000,
          })
          // ⚠️ 第 83 轮：删掉 `if (spec.aoeRadius > 0) this.shake = 4`。
          //
          // 那是 `aoeRadius` 在整个仓库里**唯一**的消费者 ——
          // 而它做的事是「屏幕震动」。
          //
          // 问题不在于震动本身，而在于它是一个**假信号**：
          // 玩家从震动推断「这里炸到了」，而实际上溅射从未被实现，
          // 一个敌人都没被打到。于是这条反应「看起来生效了但其实没有」，
          // 比完全没有反馈更让人困惑。
          //
          // 同理 `spec` 现在只用于 `spec.name`（弹字），
          // 所以这个 case 里不再需要它。
          void spec
          break
        }
        case 'kill':
          if (e.boss) this.shake = 14
          break
        case 'leak':
          this.shake = Math.max(this.shake, 8)
          break
        case 'wave_start':
          this.popups.push({ text: `第 ${e.index + 1} 波`, color: '#58a6ff', lifeMs: 1400, x: 500, y: 220 })
          break
        case 'terrain':
          this.popups.push({
            text: TERRAIN_NAME[e.kind as keyof typeof TERRAIN_NAME] ?? e.kind,
            color: '#c9a7ff',
            lifeMs: 1200,
            x: e.x,
            y: e.y,
          })
          this.shake = Math.max(this.shake, 6)
          break
        case 'overheat':
          this.popups.push({ text: '过热！', color: '#ff6b35', lifeMs: 1200, x: 500, y: 300 })
          this.shake = Math.max(this.shake, 6)
          break
        case 'won':
          this.popups.push({ text: `通关 ${e.stars} 星`, color: '#7ee787', lifeMs: 3000, x: 500, y: 300 })
          break
        case 'lost':
          this.popups.push({ text: '防线失守', color: '#ff6b35', lifeMs: 3000, x: 500, y: 300 })
          break
      }
    }
    if (this.popups.length > 8) this.popups.splice(0, this.popups.length - 8)
  }

  /** 绘制一帧 */
  draw(dtMs: number): void {
    const ctx = this.ctx

    if (this.shake > 0) this.shake = Math.max(0, this.shake - dtMs * 0.02)
    const sx = this.shake > 0 ? (Math.random() - 0.5) * this.shake : 0
    const sy = this.shake > 0 ? (Math.random() - 0.5) * this.shake : 0

    ctx.save()
    ctx.translate(sx, sy)

    this.drawBackground(ctx)
    this.drawTerrain(ctx)
    this.drawBase(ctx)
    // ⚠️ 第 158 轮：绘制前先按视口剔除。
    //
    // 逻辑空间（1000×1000）与画布视口并不重合 ——
    // 战场是等比缩放后**居中**的，窄屏上左右两侧有大片留白，
    // 而敌人一路走到被击穿/漏怪才会消失。加锁 Margin 之后，
    // 视野外的对象（飞出场地的弹丸、尚未进入画面的东西）不再产生 draw call。
    for (const p of this.engine.projectiles) {
      if (this.visible(toCanvasX(this.vp, p.x), toCanvasY(this.vp, p.y))) this.drawProjectile(ctx, p)
    }
    for (const e of this.engine.enemies) {
      if (this.visible(toCanvasX(this.vp, e.x), toCanvasY(this.vp, e.y))) this.drawEnemy(ctx, e)
    }
    this.drawFloats(ctx)
    this.drawPopups(ctx, dtMs)
    this.drawHud(ctx)

    ctx.restore()
  }

  /** 该画布坐标点是否可能可见（含 CULL_MARGIN 的余量） */
  private visible(x: number, y: number): boolean {
    const m = CULL_MARGIN * this.vp.scale
    return x >= -m && x <= this.vp.width + m && y >= -m && y <= this.vp.height + m
  }

  private drawBackground(ctx: CanvasRenderingContext2D): void {
    // 静态层可用 → 每帧 1 次 drawImage 取代「2 个渐变 + 18 次 stroke + 3 次全屏填充」
    const layer = this.ensureBackgroundLayer()
    if (layer) {
      ctx.drawImage(layer as never, 0, 0, this.vp.width, this.vp.height)
      return
    }
    this.paintBackground(ctx)
  }

  /**
   * 取背景静态层，必要时烘焙一次。
   *
   * 触发重建的条件只有「视口指纹变了」—— 也就是首帧与旋屏，
   * 一场战斗里通常只发生一次。
   */
  private ensureBackgroundLayer(): unknown | null {
    if (this.bgLayerFailed) return null
    const dpr = this.dpr
    const key = `${this.vp.width}x${this.vp.height}x${dpr}`
    if (this.bgLayer && this.bgKey === key) return this.bgLayer.canvas

    const pw = Math.max(1, Math.round(this.vp.width * dpr))
    const ph = Math.max(1, Math.round(this.vp.height * dpr))
    const layer = acquireLayer(pw, ph)
    if (!layer) {
      // 环境不支持离屏画布：记住这件事，之后每帧直接绘制，不再重试
      this.bgLayerFailed = true
      return null
    }
    // 层内用设备像素，而 paintBackground 写的是 CSS 坐标 —— 用 scale 对齐。
    // 主 ctx 上的 drawImage 再把层缩回 CSS 尺寸，视觉与其它绘制路径一致。
    layer.ctx.save()
    layer.ctx.scale(dpr, dpr)
    this.paintBackground(layer.ctx)
    layer.ctx.restore()

    this.bgLayer = layer
    this.bgKey = key
    return layer.canvas
  }

  /** 背景的一次性绘制。永远成立（无论走离屏缓存还是直接降级到这里）。 */
  private paintBackground(ctx: CanvasRenderingContext2D): void {
    const { width, height } = this.vp
    const grad = ctx.createLinearGradient(0, 0, 0, height)
    grad.addColorStop(0, '#0d1117')
    grad.addColorStop(1, '#161b22')
    ctx.fillStyle = grad
    ctx.fillRect(0, 0, width, height)

    // 地面网格：帮助玩家判断敌人推进距离
    //
    // ⚠️ 第 158 轮：18 条线原来是 18 次 `beginPath + moveTo + lineTo + stroke`
    // = **18 次 draw call**，而它们画的是同一件事（同色、同线宽）。
    // 合成单个 path 后只提交一次 —— 18 → 1。
    ctx.strokeStyle = 'rgba(48,54,61,0.5)'
    ctx.lineWidth = 1
    ctx.beginPath()
    for (let i = 1; i < GRID_DIVISIONS; i++) {
      const x = this.vp.offsetX + (FIELD_W / GRID_DIVISIONS) * i * this.vp.scale
      ctx.moveTo(x, this.vp.offsetY)
      ctx.lineTo(x, this.vp.offsetY + FIELD_H * this.vp.scale)
    }
    for (let i = 1; i < GRID_DIVISIONS; i++) {
      const y = this.vp.offsetY + (FIELD_H / GRID_DIVISIONS) * i * this.vp.scale
      ctx.moveTo(this.vp.offsetX, y)
      ctx.lineTo(this.vp.offsetX + FIELD_W * this.vp.scale, y)
    }
    ctx.stroke()

    // 边缘暗角（vignette）：聚焦中心战场，与网格一起营造纵深
    this.drawVignette(ctx)
  }

  /** 边缘暗角（径向渐变，透明到半透明黑） */
  private drawVignette(ctx: CanvasRenderingContext2D): void {
    const { width, height } = this.vp
    const cx = width / 2
    const cy = height / 2
    const radius = Math.max(width, height) * 0.7
    const grad = ctx.createRadialGradient(cx, cy, radius * 0.3, cx, cy, radius)
    grad.addColorStop(0, 'rgba(0,0,0,0)')
    grad.addColorStop(1, 'rgba(0,0,0,0.45)')
    ctx.fillStyle = grad
    ctx.fillRect(0, 0, width, height)
  }

  private drawBase(ctx: CanvasRenderingContext2D): void {
    const vp = this.vp
    const x = vp.offsetX + BASE_X * vp.scale
    const y = vp.offsetY + BASE_Y * vp.scale
    const w = 46 * vp.scale
    const h = 120 * vp.scale
    const hpPct = Number(this.engine.baseHp) / Number(this.engine.baseHpMax)

    // 防线墙体：纯色 + 左侧高光条（用半透明白叠加，不替换主色，
    // 让"血量分段变色"保持为可断言的纯色）
    const wallColor = hpPct > 0.5 ? '#58a6ff' : hpPct > 0.2 ? '#ffd33d' : '#ff6b35'
    ctx.fillStyle = wallColor
    ctx.fillRect(x - w * 0.2, y - h / 2, w * 0.22, h)
    // 左侧高光（半透明白窄条，叠加在墙上，不改变主色可断言性）
    ctx.fillStyle = 'rgba(255,255,255,0.18)'
    ctx.fillRect(x - w * 0.2, y - h / 2, 2 * vp.scale, h)
    // 底部阴影条（半透明黑，给"金属板"的厚度感）
    ctx.fillStyle = 'rgba(0,0,0,0.35)'
    ctx.fillRect(x - w * 0.2, y + h / 2 - 3 * vp.scale, w * 0.22, 3 * vp.scale)

    // 血条
    const barH = 6
    ctx.fillStyle = 'rgba(0,0,0,0.5)'
    ctx.fillRect(x - w, y - h / 2 - 12, w * 2, barH)
    ctx.fillStyle = hpPct > 0.3 ? '#7ee787' : '#ff6b35'
    ctx.fillRect(x - w, y - h / 2 - 12, w * 2 * hpPct, barH)
  }

  /**
   * 「敌人本体」的整副精灵：形状填充 + 描边 + 左上高光合成一张位图。
   *
   * ## 改前每个敌人每帧：3 次 draw call + 一次路径构建
   *
   * 而这三者**全部是静态的** —— 形状只由 `category` 决定（6 种），
   * 颜色只由「是否 BOSS / 是否飞行 / 是否受击闪白」决定（4 种），
   * 半径只由三者 + `vp.scale` 决定。组合数是十几个量级，
   * 而同屏 30 个敌人 × 60fps 就是每秒 5400 次重复构建。
   *
   * ⚠️ **动态部分仍在原路径**：护盾环、冻结/眩晕覆盖、元素层方块、
   * 血条这几样每帧都在变，不能烘进精灵 —— 它们保留原来的绘制。
   */
  private enemySprite(
    category: string,
    bodyColor: string,
    strokeColor: string,
    rPx: number,
  ): unknown | null {
    const key = `enemy:${category}:${bodyColor}:${strokeColor}:${Math.round(rPx)}`
    return (
      this.sprite(key, () => {
        const pad = 3
        const size = Math.max(2, Math.ceil(rPx * 2 + pad * 2))
        const layer = acquireLayer(size, size)
        if (!layer) return null
        this.paintEnemyBody(layer.ctx, size / 2, size / 2, rPx, category, bodyColor, strokeColor, this.vp.scale)
        return { canvas: layer.canvas }
      })?.canvas ?? null
    )
  }

  /** 敌人本体的即时绘制。「烘焙精灵」与「降级路径」两处共用同一份。 */
  private paintEnemyBody(
    ctx: CanvasRenderingContext2D,
    x: number,
    y: number,
    r: number,
    category: string,
    bodyColor: string,
    strokeColor: string,
    lineWidthPx: number,
  ): void {
    ctx.beginPath()
    switch (category) {
      case 'flying':
        // 三角形（飞行）
        ctx.moveTo(x, y - r)
        ctx.lineTo(x + r, y + r * 0.7)
        ctx.lineTo(x - r, y + r * 0.7)
        ctx.closePath()
        break
      case 'ranged':
        // 菱形（远程）
        ctx.moveTo(x, y - r)
        ctx.lineTo(x + r, y)
        ctx.lineTo(x, y + r)
        ctx.lineTo(x - r, y)
        ctx.closePath()
        break
      case 'boss':
      case 'elite':
        // 八边形（BOSS/精英）
        for (let i = 0; i < 8; i++) {
          const a = (Math.PI * 2 * i) / 8 + Math.PI / 8
          const px = x + Math.cos(a) * r
          const py = y + Math.sin(a) * r
          if (i === 0) ctx.moveTo(px, py)
          else ctx.lineTo(px, py)
        }
        ctx.closePath()
        break
      case 'special':
        // 六边形（特殊）
        for (let i = 0; i < 6; i++) {
          const a = (Math.PI * 2 * i) / 6
          const px = x + Math.cos(a) * r
          const py = y + Math.sin(a) * r
          if (i === 0) ctx.moveTo(px, py)
          else ctx.lineTo(px, py)
        }
        ctx.closePath()
        break
      default:
        // 圆形（普通）
        ctx.arc(x, y, r, 0, Math.PI * 2)
    }

    ctx.fillStyle = bodyColor
    ctx.fill()
    ctx.strokeStyle = strokeColor
    ctx.lineWidth = lineWidthPx
    ctx.stroke()
    // 左上高光（小圆点 + 半透明），给实体感
    ctx.beginPath()
    ctx.arc(x - r * 0.35, y - r * 0.35, r * 0.35, 0, Math.PI * 2)
    ctx.fillStyle = 'rgba(255,255,255,0.18)'
    ctx.fill()
  }

  private drawEnemy(ctx: CanvasRenderingContext2D, e: Enemy): void {
    const vp = this.vp
    const x = toCanvasX(vp, e.x)
    const y = toCanvasY(vp, e.y)
    if (e.spawnProgress < 1) {
      ctx.globalAlpha = e.spawnProgress
    }
    if (e.flyHeight > 0) {
      ctx.globalAlpha *= 0.9
    }

    const r = (e.isBoss ? 26 : e.category === 'normal' ? 14 : 18) * vp.scale

    const bodyColor = e.hitFlashMs > 0 ? '#ffffff' : e.isBoss ? '#8b2f4a' : e.category === 'flying' ? '#3d5a80' : '#4a6b52'
    // 元素色描边 + 高光，让不同 category 的敌人在颜色上更容易区分
    // 非 BOSS 的描边用背景色（深色），与测试「#0d1117 必须出现」保持一致；
    // BOSS 仍保留警示橙描边（测试断言 #ff6b35 必须出现）
    const strokeColor = e.isBoss ? '#ff6b35' : '#0d1117'
    const lineWidth = 2 * vp.scale * 0.5

    // 主体：形状 + 描边 + 高光整副烘焙成位图，3 次提交 → 1 次 drawImage
    const body = this.enemySprite(e.category, bodyColor, strokeColor, r)
    if (body) {
      const size = Math.max(2, Math.ceil(r * 2 + 6))
      ctx.drawImage(body as never, x - size / 2, y - size / 2, size, size)
    } else {
      this.paintEnemyBody(ctx, x, y, r, e.category, bodyColor, strokeColor, lineWidth)
    }

    // 护盾环
    if (e.shield > 0n) {
      ctx.beginPath()
      ctx.arc(x, y, r + 4 * vp.scale, 0, Math.PI * 2)
      ctx.strokeStyle = '#58a6ff'
      ctx.lineWidth = 2 * vp.scale * 0.4
      ctx.stroke()
    }

    // 状态标记：冻结/眩晕
    if (e.frozenMs > 0) {
      ctx.fillStyle = 'rgba(88,166,255,0.35)'
      ctx.beginPath()
      ctx.arc(x, y, r, 0, Math.PI * 2)
      ctx.fill()
    }
    if (e.stunnedMs > 0) {
      ctx.fillStyle = 'rgba(255,211,61,0.3)'
      ctx.beginPath()
      ctx.arc(x, y, r, 0, Math.PI * 2)
      ctx.fill()
    }

    // 元素层数：五色小方块，直接显示敌人身上挂了什么 —— 这是抗性玩法的可读性关键
    //
    // ⚠️ 第 158 轮两处修改：
    //  1. `els` 数组原本是**函数内的字面量** —— 每个敌人每帧新建一次，
    //     而它是个常量。提到模块级。
    //  2. 整个循环用 `e.stacks.size > 0` 先行拦截 ——
    //     场面上绝大多数敌人在绝大多数帧上身上是**没有层数的**，
    //     却要为「确认没有」付出 5 次 Map.get。
    if (e.stacks.size > 0) {
      let slot = 0
      for (const el of ELEMENT_ORDER) {
        const s = e.stacks.get(el) ?? 0n
        for (let i = 0n; i < s && i < 4n; i++) {
          const sx = x - r + slot * 6 * vp.scale
          const sy = y + r + 3 * vp.scale
          ctx.fillStyle = ELEMENT_COLOR[el]
          ctx.fillRect(sx, sy, 4 * vp.scale, 4 * vp.scale)
          slot++
        }
      }
    }

    // 血条
    //
    // ⚠️ 第 158 轮：**满血的敌人不画血条**（省 2 次提交）。
    // 场上绝大多数敌人在绝大多数帧是未受伤的，而一条填满的血条
    // 提供的信息量为零 —— 反而把画面糊满无用细节。受伤才显示，
    // 顺带让「正在掉血的目标」在视觉上更容易被注意到。
    if (e.hp < e.maxHp) {
      const bw = r * 2
      const hpPct = Number(e.hp) / Number(e.maxHp)
      ctx.fillStyle = 'rgba(0,0,0,0.5)'
      ctx.fillRect(x - bw / 2, y - r - 8 * vp.scale, bw, 3 * vp.scale)
      ctx.fillStyle = e.isBoss ? '#ff6b35' : '#7ee787'
      ctx.fillRect(x - bw / 2, y - r - 8 * vp.scale, bw * hpPct, 3 * vp.scale)
    }

    ctx.globalAlpha = 1
  }

  /**
   * 「弹体 + 外圈光晕」的整颗精灵。
   *
   * ## 改前每个弹丸每帧
   *
   *   1 个 `createLinearGradient`（拖尾）+ 1 个 `createRadialGradient`（光晕）
   *   + 4 次 draw call（拖尾 / 弹体 fill / 弹体 stroke / 光晕）
   *
   * 两个渐变都是**每帧重建的 native 对象** —— 战场上 9 颗弹丸 × 60fps
   * 就是每秒 1080 个渐变对象，全部进出 GC。而它们的形状、颜色、半径
   * 全部是常量，只取决于「元素色」这一个 enum（5 种取值）。
   *
   * 烘焙后每个弹丸每帧：**1 次 drawImage（代替光晕+弹体 3 次提交）
   * + 1 段拖尾描边**，渐变分配归零。
   *
   * ⚠️ 精灵尺寸依赖 `vp.scale`，所以 key 里必须带像素半径 ——
   * 旋屏后 scale 变化会自动烘焙新的一档，旧的仍在 Map 里（数量级 ≤ 个位数）。
   */
  private projectileSprite(element: Element, rPx: number): unknown | null {
    const key = `proj:${element}:${Math.round(rPx)}`
    return (
      this.sprite(key, () => {
        const color = ELEMENT_COLOR[element]
        const outer = rPx * 2.4
        const size = Math.max(2, Math.ceil(outer * 2) + 2)
        const layer = acquireLayer(size, size)
        if (!layer) return null
        const c = size / 2
        // 光晕（alpha 0.25 与原实现一致）
        const grad = layer.ctx.createRadialGradient(c, c, rPx * 0.5, c, c, outer)
        grad.addColorStop(0, color)
        grad.addColorStop(1, 'rgba(0,0,0,0)')
        layer.ctx.globalAlpha = 0.25
        layer.ctx.fillStyle = grad
        layer.ctx.fillRect(0, 0, size, size)
        layer.ctx.globalAlpha = 1
        // 弹体（实心圆 + 白描边）
        layer.ctx.beginPath()
        layer.ctx.arc(c, c, rPx, 0, Math.PI * 2)
        layer.ctx.fillStyle = color
        layer.ctx.fill()
        layer.ctx.strokeStyle = '#ffffff'
        layer.ctx.lineWidth = 1
        layer.ctx.stroke()
        return { canvas: layer.canvas }
      })?.canvas ?? null
    )
  }

  private drawProjectile(ctx: CanvasRenderingContext2D, p: Projectile): void {
    const vp = this.vp
    const x = toCanvasX(vp, p.x)
    const y = toCanvasY(vp, p.y)
    const r = 5 * vp.scale
    const color = ELEMENT_COLOR[p.element]

    // 拖尾：用纯色 + alpha **取代每帧一条 createLinearGradient**。
    //
    // 拖尾只有 14 逻辑单位长（≈几个像素），在这个尺度上「渐变到透明」
    // 与「同色半透明」肉眼几乎不可分，但它省掉的是每帧每颗弹丸
    // 一次 native 渐变分配。
    //
    // Math.hypot → 开方手写：hypot 内部要做溢出保护的重标定，
    // 而这里的量级已知（速度分量是定点速度/tick，远不到溢出）。
    const vx = Number(p.vx)
    const vy = Number(p.vy)
    const vlen = Math.max(1, Math.sqrt(vx * vx + vy * vy))
    const tl = 14 * vp.scale
    const tailX = x - (vx / vlen) * tl
    const tailY = y - (vy / vlen) * tl
    ctx.globalAlpha = 0.55
    ctx.strokeStyle = color
    ctx.lineWidth = r * 1.4
    ctx.beginPath()
    ctx.moveTo(x, y)
    ctx.lineTo(tailX, tailY)
    ctx.stroke()
    ctx.globalAlpha = 1

    // 弹体 + 光晕：合并成一张预烘焙精灵，3 次提交 → 1 次 drawImage
    const sprite = this.projectileSprite(p.element, r)
    if (sprite) {
      const size = Math.max(2, Math.ceil(r * 2.4 * 2) + 2)
      ctx.drawImage(sprite as never, x - size / 2, y - size / 2, size, size)
      return
    }

    // 降级路径（拿不到离屏画布时）：保持原来的画法
    ctx.beginPath()
    ctx.arc(x, y, r, 0, Math.PI * 2)
    ctx.fillStyle = color
    ctx.fill()
    ctx.strokeStyle = '#ffffff'
    ctx.lineWidth = 1
    ctx.stroke()
    const glowGrad = ctx.createRadialGradient(x, y, r * 0.5, x, y, r * 2.4)
    glowGrad.addColorStop(0, color)
    glowGrad.addColorStop(1, 'rgba(0,0,0,0)')
    ctx.fillStyle = glowGrad
    ctx.globalAlpha = 0.25
    ctx.beginPath()
    ctx.arc(x, y, r * 2.4, 0, Math.PI * 2)
    ctx.fill()
    ctx.globalAlpha = 1
  }

  private drawTerrain(ctx: CanvasRenderingContext2D): void {
    const vp = this.vp
    ctx.textAlign = 'center'
    ctx.lineWidth = Math.max(1, vp.scale)
    for (const t of this.engine.terrains) {
      const x = vp.offsetX + t.x * vp.scale
      const y = vp.offsetY + t.y * vp.scale
      const r = 16 * vp.scale

      switch (t.kind) {
        case 'oil_drum': {
          // 燃烧状态带火焰动画（光晕脉动）
          const pulse = t.state === 'burning' ? (Math.sin(t.timer / 200) + 1) * 0.5 : 0
          ctx.beginPath()
          ctx.arc(x, y, r, 0, Math.PI * 2)
          ctx.fillStyle = t.state === 'burning' ? '#ff6b35' : '#6b5b3f'
          ctx.fill()
          ctx.strokeStyle = '#0d1117'
          ctx.lineWidth = 1.5
          ctx.stroke()
          if (t.state === 'burning') {
            // 火焰光晕：同样是每帧重建的径向渐变，形状/颜色恒定 → 精灵化
            const flame = this.sprite(`flame:${Math.round(r * 2)}`, () =>
              makeRadialSprite('rgba(255,107,53,0.45)', r * 2),
            )
            if (flame) {
              ctx.globalAlpha = 0.5 + pulse * 0.3
              ctx.drawImage(flame.canvas as never, x - r * 2, y - r * 2, r * 4, r * 4)
              ctx.globalAlpha = 1
            } else {
              const glow = ctx.createRadialGradient(x, y, r * 0.3, x, y, r * 2)
              glow.addColorStop(0, 'rgba(255,107,53,0.45)')
              glow.addColorStop(1, 'rgba(255,107,53,0)')
              ctx.globalAlpha = 0.5 + pulse * 0.3
              ctx.fillStyle = glow
              ctx.beginPath()
              ctx.arc(x, y, r * 2, 0, Math.PI * 2)
              ctx.fill()
              ctx.globalAlpha = 1
            }
          }
          break
        }
        case 'tidal_gate': {
          // 闸门：纯色 + 开/闭状态不同（保持可断言的纯色，
          // 叠加光晕用径向渐变给质感）
          ctx.fillStyle = t.state === 'closed' ? '#58a6ff' : 'rgba(88,166,255,0.25)'
          ctx.fillRect(x - r, y - r, r * 2, r * 0.4)
          // 关闭时闸门上的光晕：同样是恒定径向渐变 → 精灵化
          if (t.state === 'closed') {
            const g = this.sprite(`gate:${Math.round(r * 1.2)}`, () =>
              makeRadialSprite('rgba(88,166,255,0.5)', r * 1.2),
            )
            if (g) {
              ctx.drawImage(g.canvas as never, x - r * 1.2, y - r * 0.3 - r * 1.2, r * 2.4, r * 2.4)
            } else {
              const gateGlow = ctx.createRadialGradient(x, y - r * 0.3, 2, x, y - r * 0.3, r * 1.2)
              gateGlow.addColorStop(0, 'rgba(88,166,255,0.5)')
              gateGlow.addColorStop(1, 'rgba(88,166,255,0)')
              ctx.fillStyle = gateGlow
              ctx.fillRect(x - r, y - r, r * 2, r * 0.4)
            }
          }
          break
        }
        case 'rotor_vane': {
          ctx.save()
          ctx.translate(x, y)
          ctx.rotate((t.angle * Math.PI) / 180)
          ctx.strokeStyle = '#c9a7ff'
          ctx.lineWidth = 3
          for (let i = 0; i < 3; i++) {
            const a = (Math.PI * 2 * i) / 3
            ctx.beginPath()
            ctx.moveTo(0, 0)
            ctx.lineTo(Math.cos(a) * r, Math.sin(a) * r)
            ctx.stroke()
          }
          // 中心光晕：恒定的紫色径向渐变 → 精灵化
          const vane = this.sprite(`vane:${Math.round(r * 0.6)}`, () =>
            makeRadialSprite('rgba(201,167,255,0.4)', r * 0.6),
          )
          if (vane) {
            ctx.drawImage(vane.canvas as never, -r * 0.6, -r * 0.6, r * 1.2, r * 1.2)
          } else {
            const rotorGlow = ctx.createRadialGradient(0, 0, 2, 0, 0, r * 0.6)
            rotorGlow.addColorStop(0, 'rgba(201,167,255,0.4)')
            rotorGlow.addColorStop(1, 'rgba(201,167,255,0)')
            ctx.fillStyle = rotorGlow
            ctx.beginPath()
            ctx.arc(0, 0, r * 0.6, 0, Math.PI * 2)
            ctx.fill()
          }
          ctx.restore()
          break
        }
        case 'collapse_wall': {
          // 掩体带纵向纹理（模拟石块）
          ctx.fillStyle = t.state === 'collapsed' ? 'rgba(48,54,61,0.3)' : '#6e7681'
          ctx.fillRect(x - r, y - r * 0.7, r * 2, r * 1.4)
          if (t.state !== 'collapsed') {
            // 石块分缝
            ctx.strokeStyle = 'rgba(0,0,0,0.35)'
            ctx.lineWidth = 1
            for (let i = 1; i < 4; i++) {
              const fy = y - r * 0.7 + (r * 1.4 * i) / 4
              ctx.beginPath()
              ctx.moveTo(x - r, fy)
              ctx.lineTo(x + r, fy)
              ctx.stroke()
            }
            ctx.beginPath()
            ctx.moveTo(x, y - r * 0.7)
            ctx.lineTo(x, y + r * 0.7)
            ctx.stroke()
          }
          break
        }
        case 'charge_tower': {
          // 蓄能塔带脉冲光晕（蓄能越满越亮）—— 形状恒定，只有 alpha 随进度变
          const pct = Math.min(1, t.charge / Math.max(1, t.param))
          const tower = this.sprite(`tower:${Math.round(r * 2)}`, () =>
            makeRadialSprite('rgba(126,231,135,0.5)', r * 2),
          )
          if (tower) {
            ctx.globalAlpha = 0.25 + pct * 0.45
            ctx.drawImage(tower.canvas as never, x - r * 2, y - r * 2, r * 4, r * 4)
            ctx.globalAlpha = 1
          } else {
            const towerGlow = ctx.createRadialGradient(x, y, r * 0.3, x, y, r * 2)
            towerGlow.addColorStop(0, 'rgba(126,231,135,0.5)')
            towerGlow.addColorStop(1, 'rgba(126,231,135,0)')
            ctx.globalAlpha = 0.25 + pct * 0.45
            ctx.fillStyle = towerGlow
            ctx.beginPath()
            ctx.arc(x, y, r * 2, 0, Math.PI * 2)
            ctx.fill()
            ctx.globalAlpha = 1
          }

          ctx.beginPath()
          ctx.arc(x, y, r * 0.8, 0, Math.PI * 2)
          ctx.fillStyle = '#7ee787'
          ctx.fill()
          // 蓄能进度（环形，宽度随蓄能增加）
          ctx.strokeStyle = '#0d1117'
          ctx.lineWidth = 4 + pct * 4
          ctx.beginPath()
          ctx.arc(x, y, r * 1.1, -Math.PI / 2, -Math.PI / 2 + Math.PI * 2 * pct)
          ctx.stroke()
          break
        }
      }
    }
  }

  private drawFloats(ctx: CanvasRenderingContext2D): void {
    const vp = this.vp
    ctx.textAlign = 'center'
    let fontPx = -1
    for (const f of this.engine.floats) {
      const alpha = Math.max(0, f.lifeMs / f.maxLifeMs)
      ctx.globalAlpha = alpha
      ctx.fillStyle = f.color
      // 字体串随 size 变，而 size 在一帧里高度重复 —— 用缩放后的像素值去重
      const px = f.size * vp.scale
      if (px !== fontPx) {
        ctx.font = this.fontOf(px)
        fontPx = px
      }
      ctx.fillText(f.text, toCanvasX(vp, f.x), toCanvasY(vp, f.y))
    }
    ctx.globalAlpha = 1
  }

  private drawPopups(ctx: CanvasRenderingContext2D, dtMs: number): void {
    const vp = this.vp
    ctx.textAlign = 'center'
    ctx.font = this.fontOf(22 * vp.scale, true)
    ctx.strokeStyle = 'rgba(0,0,0,0.6)'
    ctx.lineWidth = 3

    // ⚠️ 第 158 轮：改前是「倒序遍历 + `splice(i,1)`」——
    // splice 在循环里每次都要把右侧元素整体左移，而这里每帧都可能删元素。
    // 改成**双写指针压缩**：一趟过去既完成更新绘制又完成剔除，
    // 数组对象本身也不再被反复重建。
    let w = 0
    for (let i = 0; i < this.popups.length; i++) {
      const p = this.popups[i]
      p.lifeMs -= dtMs
      if (p.lifeMs <= 0) continue
      this.popups[w++] = p

      const alpha = Math.min(1, p.lifeMs / 400)
      ctx.globalAlpha = alpha
      ctx.fillStyle = p.color
      // 描边 + 阴影：让弹字在深色背景上更清晰
      const px = vp.offsetX + p.x * vp.scale
      const py = vp.offsetY + p.y * vp.scale
      ctx.strokeText(p.text, px, py)
      ctx.fillText(p.text, px, py)
    }
    this.popups.length = w
    ctx.globalAlpha = 1
  }

  /** HUD：热量条 + 技能槽 + 槽位提示。绘制在逻辑空间之外（屏幕固定位置） */
  private drawHud(ctx: CanvasRenderingContext2D): void {
    const vp = this.vp
    const heat = this.engine.heat
    const barW = vp.width * 0.6
    const barH = 10
    const x = (vp.width - barW) / 2
    const y = vp.height - 46

    // 热量条（纯色填充 + 圆角遮罩，关键色保持可断言的纯色）
    ctx.fillStyle = 'rgba(0,0,0,0.55)'
    ctx.fillRect(x, y, barW, barH)
    const cap = Number(heat.cap)
    const pct = cap > 0 ? Number(heat.heat) / cap : 0
    const heatColor = heat.overheated ? '#ff6b35' : pct > 0.8 ? '#ffd33d' : '#58a6ff'
    ctx.fillStyle = heatColor
    ctx.fillRect(x, y, barW * Math.min(1, pct), barH)
    // 热量条顶部高光（半透明白），增强立体感
    ctx.fillStyle = 'rgba(255,255,255,0.15)'
    ctx.fillRect(x, y, barW * Math.min(1, pct), 2)
    ctx.strokeStyle = '#30363d'
    ctx.lineWidth = 1
    ctx.strokeRect(x, y, barW, barH)

    ctx.fillStyle = '#8b949e'
    ctx.font = '11px sans-serif'
    ctx.textAlign = 'left'
    ctx.fillText(`热量 ${Number(heat.heat)}/${cap}`, x, y - 4)
    ctx.textAlign = 'right'
    if (heat.overheated) {
      ctx.fillStyle = '#ff6b35'
      ctx.fillText(`过热 ${Math.ceil(heat.overheatRemaining / 100) / 10}s`, x + barW, y - 4)
    } else {
      ctx.fillText(`元素层数 ${this.engine.skills.length} 技能`, x + barW, y - 4)
    }

    // 技能槽（图标为几何图形）
    const slotSize = 30
    const gap = 6
    const totalW = this.engine.skills.length * slotSize + (this.engine.skills.length - 1) * gap
    let sx = (vp.width - totalW) / 2
    const sy = vp.height - 30
    for (const s of this.engine.skills) {
      ctx.fillStyle = 'rgba(22,27,34,0.85)'
      ctx.fillRect(sx, sy, slotSize, slotSize)
      ctx.strokeStyle = ELEMENT_COLOR[s.element]
      ctx.lineWidth = 1.5
      ctx.strokeRect(sx, sy, slotSize, slotSize)
      // 冷却遮罩
      if (s.cooldownRemaining > 0) {
        const cd = s.cooldownRemaining / Math.max(1, s.cooldownMs)
        ctx.fillStyle = 'rgba(0,0,0,0.55)'
        ctx.fillRect(sx, sy + slotSize * (1 - cd), slotSize, slotSize * cd)
      }
      // 元素色块
      ctx.fillStyle = ELEMENT_COLOR[s.element]
      ctx.fillRect(sx + 8, sy + 8, slotSize - 16, slotSize - 16)
      sx += slotSize + gap
    }
  }

  /**
   * 启动循环。
   *
   * ⚠️ 关键设计：逻辑与绘制必须由**两个独立的时钟**驱动，不能都挂在 rAF 上。
   *
   * rAF 与页面可见性绑定：标签页切到后台 / 小程序切后台时，rAF 会被完全冻结
   * （实测 document.hidden 时 rAF 回调次数为 0，而 setInterval 正常）。
   * 若把战斗逻辑挂在 rAF 上，页面不可见时战斗就彻底停摆 —— 无头环境、
   * 后台运行、真机切后台都会出问题。
   *
   * 因此：
   *   - 逻辑：setInterval(TICK_MS) 固定步长，与可见性无关
   *   - 绘制：rAF 跟随屏幕刷新率，页面不可见时自然暂停（省电，且无可见之处）
   *
   * 这样即使绘制降到 1fps，战斗的时间流仍完全一致 —— I-6 的回放哈希不会漂移。
   */
  start(loop: (dtMs: number) => void): void {
    if (this.running) return
    this.running = true
    this.loopFn = loop

    // 1) 逻辑时钟：固定步长（间隔随倍速缩放，步长不变）
    this.startLogicClock()

    // 2) 绘制时钟：跟随刷新率。
    // rAF 在无浏览器环境（Node 测试、部分小程序基础库）下可能不存在，
    // 此时降级为定时器绘制，保证逻辑与绘制都不会因缺少 rAF 而崩溃。
    if (typeof requestAnimationFrame === 'function') {
      const frame = (ts: number) => {
        if (!this.running) return
        if (this.lastTs === 0) this.lastTs = ts
        const dt = Math.min(ts - this.lastTs, TICK_MS * 4)
        this.lastTs = ts
        this.draw(dt)
        this.rafId = requestAnimationFrame(frame)
      }
      this.lastTs = 0
      this.rafId = requestAnimationFrame(frame)
    } else {
      this.drawTimer = setInterval(() => {
        if (this.running) this.draw(TICK_MS)
      }, TICK_MS) as unknown as number
    }
  }

  /**
   * 启动逻辑时钟（间隔 = `logicIntervalMs(倍速)`）。
   *
   * 倍速走「缩短定时器间隔」而不是「每个 tick 多跑几步」：
   * 后者会变成「每 50ms 突然推进 8 步、然后静止 50ms」，
   * 画面上的运动是一顿一顿的（20 次/秒的跳变）；前者是平滑的。
   */
  private startLogicClock(): void {
    this.logicTimer = setInterval(() => {
      if (!this.running) return
      this.loopFn?.(TICK_MS)
    }, logicIntervalMs(this.speed)) as unknown as number
  }

  private stopLogicClock(): void {
    if (this.logicTimer !== null) {
      clearInterval(this.logicTimer as unknown as ReturnType<typeof setInterval>)
      this.logicTimer = null
    }
  }

  /**
   * 换挡（2026-10-22）。
   *
   * 允许在运行中调用：先停旧定时器再按新间隔重建。
   * ⚠️ 不先停就重建会**留下两个时钟**，倍速会变成平方级 ——
   * 而且症状只是「怎么突然这么快」，看不出是定时器叠加。
   *
   * 未 start() 时只记下档位，等 start() 按新档位建时钟。
   */
  setSpeed(mult: number): void {
    const next = normalizeSpeed(mult)
    if (next === this.speed) return
    this.speed = next
    if (!this.running) return
    this.stopLogicClock()
    this.startLogicClock()
  }

  /** 当前倍速（供页面回显）。 */
  getSpeed(): number {
    return this.speed
  }

  stop(): void {
    this.running = false
    if (this.rafId !== null) {
      cancelAnimationFrame(this.rafId)
      this.rafId = null
    }
    this.stopLogicClock()
    if (this.drawTimer !== null) {
      clearInterval(this.drawTimer as unknown as ReturnType<typeof setInterval>)
      this.drawTimer = null
    }
  }
}
