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
import { ELEMENT_COLOR, REACTIONS, type Element } from '../game/elements'
import { TERRAIN_NAME } from '../game/terrain'
import { BASE_X, BASE_Y, FIELD_H, FIELD_W } from '../game/terrain'
import type { Enemy, Projectile } from '../game/types'

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

  constructor(
    canvas: HTMLCanvasElement | { width: number; height: number },
    engine: BattleEngine,
  ) {
    this.engine = engine
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
    for (const p of this.engine.projectiles) this.drawProjectile(ctx, p)
    for (const e of this.engine.enemies) this.drawEnemy(ctx, e)
    this.drawFloats(ctx)
    this.drawPopups(ctx, dtMs)
    this.drawHud(ctx)

    ctx.restore()
  }

  private drawBackground(ctx: CanvasRenderingContext2D): void {
    const { width, height } = this.vp
    const grad = ctx.createLinearGradient(0, 0, 0, height)
    grad.addColorStop(0, '#0d1117')
    grad.addColorStop(1, '#161b22')
    ctx.fillStyle = grad
    ctx.fillRect(0, 0, width, height)

    // 地面网格：帮助玩家判断敌人推进距离
    ctx.strokeStyle = 'rgba(48,54,61,0.5)'
    ctx.lineWidth = 1
    for (let i = 1; i < 10; i++) {
      const x = this.vp.offsetX + (FIELD_W / 10) * i * this.vp.scale
      ctx.beginPath()
      ctx.moveTo(x, this.vp.offsetY)
      ctx.lineTo(x, this.vp.offsetY + FIELD_H * this.vp.scale)
      ctx.stroke()
    }
    for (let i = 1; i < 10; i++) {
      const y = this.vp.offsetY + (FIELD_H / 10) * i * this.vp.scale
      ctx.beginPath()
      ctx.moveTo(this.vp.offsetX, y)
      ctx.lineTo(this.vp.offsetX + FIELD_W * this.vp.scale, y)
      ctx.stroke()
    }
  }

  private drawBase(ctx: CanvasRenderingContext2D): void {
    const vp = this.vp
    const x = vp.offsetX + BASE_X * vp.scale
    const y = vp.offsetY + BASE_Y * vp.scale
    const w = 46 * vp.scale
    const h = 120 * vp.scale
    const hpPct = Number(this.engine.baseHp) / Number(this.engine.baseHpMax)

    // 防线墙体
    ctx.fillStyle = hpPct > 0.5 ? '#58a6ff' : hpPct > 0.2 ? '#ffd33d' : '#ff6b35'
    ctx.fillRect(x - w * 0.2, y - h / 2, w * 0.22, h)

    // 血条
    const barH = 6
    ctx.fillStyle = 'rgba(0,0,0,0.5)'
    ctx.fillRect(x - w, y - h / 2 - 12, w * 2, barH)
    ctx.fillStyle = hpPct > 0.3 ? '#7ee787' : '#ff6b35'
    ctx.fillRect(x - w, y - h / 2 - 12, w * 2 * hpPct, barH)
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

    // 主体：用敌人 category 决定形状（程序化几何，无外部素材）
    ctx.beginPath()
    switch (e.category) {
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

    const bodyColor = e.hitFlashMs > 0 ? '#ffffff' : e.isBoss ? '#8b2f4a' : e.category === 'flying' ? '#3d5a80' : '#4a6b52'
    ctx.fillStyle = bodyColor
    ctx.fill()
    ctx.strokeStyle = e.isBoss ? '#ff6b35' : '#0d1117'
    ctx.lineWidth = 2 * vp.scale * 0.5
    ctx.stroke()

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
    const els: Element[] = ['fire', 'ice', 'lightning', 'corrosion', 'kinetic']
    let slot = 0
    for (const el of els) {
      const s = e.stacks.get(el) ?? 0n
      for (let i = 0n; i < s && i < 4n; i++) {
        const sx = x - r + slot * 6 * vp.scale
        const sy = y + r + 3 * vp.scale
        ctx.fillStyle = ELEMENT_COLOR[el]
        ctx.fillRect(sx, sy, 4 * vp.scale, 4 * vp.scale)
        slot++
      }
    }

    // 血条
    const bw = r * 2
    const hpPct = Number(e.hp) / Number(e.maxHp)
    ctx.fillStyle = 'rgba(0,0,0,0.5)'
    ctx.fillRect(x - bw / 2, y - r - 8 * vp.scale, bw, 3 * vp.scale)
    ctx.fillStyle = e.isBoss ? '#ff6b35' : '#7ee787'
    ctx.fillRect(x - bw / 2, y - r - 8 * vp.scale, bw * hpPct, 3 * vp.scale)

    ctx.globalAlpha = 1
  }

  private drawProjectile(ctx: CanvasRenderingContext2D, p: Projectile): void {
    const vp = this.vp
    const x = toCanvasX(vp, p.x)
    const y = toCanvasY(vp, p.y)
    const r = 5 * vp.scale
    const color = ELEMENT_COLOR[p.element]

    // 拖尾
    const vlen = Math.max(1, Math.hypot(Number(p.vx), Number(p.vy)))
    const tl = 14 * vp.scale
    const tailX = x - (Number(p.vx) / vlen) * tl
    const tailY = y - (Number(p.vy) / vlen) * tl
    const grad = ctx.createLinearGradient(x, y, tailX, tailY)
    grad.addColorStop(0, color)
    grad.addColorStop(1, 'rgba(0,0,0,0)')
    ctx.strokeStyle = grad
    ctx.lineWidth = r * 1.4
    ctx.beginPath()
    ctx.moveTo(x, y)
    ctx.lineTo(tailX, tailY)
    ctx.stroke()

    // 弹体
    ctx.beginPath()
    ctx.arc(x, y, r, 0, Math.PI * 2)
    ctx.fillStyle = color
    ctx.fill()
    ctx.strokeStyle = '#ffffff'
    ctx.lineWidth = 1
    ctx.stroke()
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
          ctx.beginPath()
          ctx.arc(x, y, r, 0, Math.PI * 2)
          ctx.fillStyle = t.state === 'burning' ? '#ff6b35' : '#6b5b3f'
          ctx.fill()
          ctx.strokeStyle = '#0d1117'
          ctx.lineWidth = 1.5
          ctx.stroke()
          break
        }
        case 'tidal_gate': {
          ctx.fillStyle = t.state === 'closed' ? '#58a6ff' : 'rgba(88,166,255,0.25)'
          ctx.fillRect(x - r, y - r, r * 2, r * 0.4)
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
          ctx.restore()
          break
        }
        case 'collapse_wall': {
          ctx.fillStyle = t.state === 'collapsed' ? 'rgba(48,54,61,0.3)' : '#6e7681'
          ctx.fillRect(x - r, y - r * 0.7, r * 2, r * 1.4)
          break
        }
        case 'charge_tower': {
          ctx.beginPath()
          ctx.arc(x, y, r * 0.8, 0, Math.PI * 2)
          ctx.fillStyle = '#7ee787'
          ctx.fill()
          // 蓄能进度
          const pct = Math.min(1, t.charge / Math.max(1, t.param))
          ctx.strokeStyle = '#0d1117'
          ctx.lineWidth = 4
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
    for (const f of this.engine.floats) {
      const alpha = Math.max(0, f.lifeMs / f.maxLifeMs)
      ctx.globalAlpha = alpha
      ctx.fillStyle = f.color
      ctx.font = `${f.size}px sans-serif`
      ctx.fillText(f.text, toCanvasX(vp, f.x), toCanvasY(vp, f.y))
    }
    ctx.globalAlpha = 1
  }

  private drawPopups(ctx: CanvasRenderingContext2D, dtMs: number): void {
    const vp = this.vp
    ctx.textAlign = 'center'
    for (let i = this.popups.length - 1; i >= 0; i--) {
      const p = this.popups[i]
      p.lifeMs -= dtMs
      if (p.lifeMs <= 0) {
        this.popups.splice(i, 1)
        continue
      }
      const alpha = Math.min(1, p.lifeMs / 400)
      ctx.globalAlpha = alpha
      ctx.fillStyle = p.color
      ctx.font = `bold ${22 * vp.scale}px sans-serif`
      ctx.fillText(p.text, vp.offsetX + p.x * vp.scale, vp.offsetY + p.y * vp.scale)
    }
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

    // 热量条
    ctx.fillStyle = 'rgba(0,0,0,0.55)'
    ctx.fillRect(x, y, barW, barH)
    const cap = Number(heat.cap)
    const pct = cap > 0 ? Number(heat.heat) / cap : 0
    ctx.fillStyle = heat.overheated ? '#ff6b35' : pct > 0.8 ? '#ffd33d' : '#58a6ff'
    ctx.fillRect(x, y, barW * Math.min(1, pct), barH)
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

    // 1) 逻辑时钟：固定步长
    this.logicTimer = setInterval(() => {
      if (!this.running) return
      loop(TICK_MS)
    }, TICK_MS) as unknown as number

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

  stop(): void {
    this.running = false
    if (this.rafId !== null) {
      cancelAnimationFrame(this.rafId)
      this.rafId = null
    }
    if (this.logicTimer !== null) {
      clearInterval(this.logicTimer as unknown as ReturnType<typeof setInterval>)
      this.logicTimer = null
    }
    if (this.drawTimer !== null) {
      clearInterval(this.drawTimer as unknown as ReturnType<typeof setInterval>)
      this.drawTimer = null
    }
  }
}
