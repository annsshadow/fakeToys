/**
 * 可交互地形（I-4）。
 *
 * 5 类地形，约 41% 的关卡使用。地形既是解法（用火区烧整队、
 * 崩塌打开射界）也是约束（掩体会挡自己的弹），无法纯靠堆伤害通关。
 *
 * 坐标约定：地形配置用**逻辑单位**（0..1000），引擎内部实体坐标用**定点整数**（×1000）。
 * 两者不可混用 —— 混用会让判定范围差 1000 倍。
 */

import type { TerrainKind, TerrainPlacement, Enemy, Projectile } from './types'
import type { Element } from './elements'

export const FIELD_W = 1000
export const FIELD_H = 1000
/** 防线所在 x（敌人从右向左推进） */
export const BASE_X = 60
export const BASE_Y = 880

/** 逻辑单位 → 定点整数 */
export function toFixed(logical: number): bigint {
  return BigInt(Math.round(logical * 1000))
}

/** TerrainEffect 是地形对当前帧的影响。 */
export interface TerrainEffect {
  /** 是否阻挡该区域（潮汐闸关闭 / 掩体未崩塌） */
  blocked: boolean
}

/**
 * 已经告警过的未知地形类型。
 *
 * 只为避免同一个坏数据每 tick 打一条 console.warn ——
 * 一场战斗 12000 tick 就能刷爆控制台，把真正的错误淹掉。
 */
const warnedUnknownKind = new Set<unknown>()

export type TerrainState =
  | 'idle'
  | 'burning'
  | 'collapsed'
  | 'charging'
  | 'open'
  | 'closed'

/** TerrainContext 是地形能操作的引擎对象。 */
export interface TerrainContext {
  enemies: Enemy[]
  projectiles: Projectile[]
  /** 火区每 tick 伤害 */
  terrainTick: bigint
  /** 逻辑坐标半径内是否有实体 */
  within(terrainX: number, terrainY: number, r: number, x: bigint, y: bigint): boolean
  enemiesInRadius(x: number, y: number, r: number): Enemy[]
  onKill(e: Enemy): void
  onTerrainTrigger(kind: string, t: Terrain): void
}

export class Terrain {
  kind: TerrainKind
  x: number
  y: number
  param: number
  state: TerrainState = 'idle'
  /** 燃烧剩余毫秒 / 崩塌进度 / 蓄能进度 */
  timer = 0
  charge = 0
  /** 旋转风障当前角度（度） */
  angle = 0
  /** 本局是否被触发过（计入 terrain_used） */
  triggered = false

  constructor(p: TerrainPlacement) {
    this.kind = p.kind
    this.x = p.x
    this.y = p.y
    this.param = p.param
  }

  update(dtMs: number, ctx: TerrainContext): TerrainEffect {
    switch (this.kind) {
      case 'oil_drum':
        return this.updateOilDrum(dtMs, ctx)
      case 'tidal_gate':
        return this.updateTidalGate(dtMs)
      case 'rotor_vane':
        return this.updateRotorVane(dtMs, ctx)
      case 'collapse_wall':
        return this.updateCollapseWall(ctx)
      case 'charge_tower':
        return this.updateChargeTower(dtMs)
      default:
        // ⚠️ 这个 default 不是"防御性编程"，是必需的。
        //
        // TerrainKind 在类型上是字面量联合，所以 5 个 case 已覆盖全集、
        // TypeScript 不会报"可能隐式返回 undefined"。但它来自**网络数据**：
        // 服务端 DB 里的关卡行被手改、灰度中的新地形类型下发到旧客户端、
        // 或 JSON 缺字段导致 kind 变成 undefined —— 任何一种都会让
        // 匹配落到函数末尾的隐式 return undefined，
        // 紧接着 engine 里 `eff.blocked` 立刻抛
        // TypeError: Cannot read properties of undefined。
        //
        // 也就是说：一个未知的地形 id 会让整场战斗崩掉，而不是被忽略。
        // 未知地形应当退化成"什么都不做"，这也是渲染层 canvas.ts 的既有行为
        // （那里 switch 同样没有 default，但不绘制就不崩）。
        if (!warnedUnknownKind.has(this.kind)) {
          warnedUnknownKind.add(this.kind)
          console.warn(
            `[terrain] 未知地形类型 ${JSON.stringify(this.kind)}，已按无效果处理。` +
              `客户端与服务端的地形表可能不同步。`,
          )
        }
        return { blocked: false }
    }
  }

  /** 油桶：受焰元素命中达阈值即引燃 → 爆炸 + 生成 8s 火区 */
  private updateOilDrum(dtMs: number, ctx: TerrainContext): TerrainEffect {
    if (this.state !== 'burning') return { blocked: false }
    this.timer -= dtMs
    if (this.timer <= 0) {
      this.state = 'idle'
      this.timer = 0
      return { blocked: false }
    }
    // 火区持续伤害
    for (const e of ctx.enemiesInRadius(this.x, this.y, 90)) {
      e.hp -= ctx.terrainTick
      e.hitFlashMs = 120
      const cur = e.stacks.get('fire') ?? 0n
      e.stacks.set('fire', cur < 4n ? cur + 1n : cur)
      if (e.hp <= 0n) {
        e.hp = 0n
        e.dead = true
        ctx.onKill(e)
      }
    }
    return { blocked: false }
  }

  /** 潮汐闸：周期开合，开时低层敌人可通行 */
  private updateTidalGate(dtMs: number): TerrainEffect {
    const period = Math.max(1, this.param)
    this.timer += dtMs
    while (this.timer >= period) {
      this.timer -= period
      this.state = this.state === 'open' ? 'closed' : 'open'
    }
    return { blocked: this.state === 'closed' }
  }

  /** 旋转风障：持续改变区域内弹丸飞行方向 */
  private updateRotorVane(dtMs: number, ctx: TerrainContext): TerrainEffect {
    this.angle = (this.angle + (this.param * dtMs) / 1000) % 360
    const rad = (this.angle * Math.PI) / 180
    const deflectX = BigInt(Math.round(Math.cos(rad) * this.param * 0.4)) * 2n
    const deflectY = BigInt(Math.round(Math.sin(rad) * this.param * 0.4)) * 2n
    for (const p of ctx.projectiles) {
      if (ctx.within(this.x, this.y, 140, p.x, p.y)) {
        p.vx += deflectX
        p.vy += deflectY
      }
    }
    return { blocked: false }
  }

  /** 崩塌掩体：受动能伤害累计到阈值即崩塌 → 弹道永久改变 */
  private updateCollapseWall(ctx: TerrainContext): TerrainEffect {
    if (this.state === 'collapsed') return { blocked: false }
    for (const p of ctx.projectiles) {
      if (p.element === 'kinetic' && ctx.within(this.x, this.y, 110, p.x, p.y)) {
        this.charge += 1
        if (this.charge >= this.param) {
          this.state = 'collapsed'
          this.triggered = true
          ctx.onTerrainTrigger('collapse_wall', this)
        }
      }
    }
    return { blocked: true }
  }

  /** 蓄能塔：充能由击杀驱动（见 onKillCharging），这里只做计时展示 */
  private updateChargeTower(dtMs: number): TerrainEffect {
    this.timer = dtMs
    return { blocked: false }
  }

  /**
   * 由命中结算调用：把伤害与元素喂给地形。
   * 返回 true 表示本次命中触发了地形效果。
   */
  onHit(element: Element, damage: bigint): boolean {
    if (this.state === 'collapsed') return false
    switch (this.kind) {
      case 'oil_drum': {
        if (element === 'fire' && this.state === 'idle') {
          this.charge += Number(damage / 100n)
          if (this.charge >= this.param) {
            this.state = 'burning'
            this.timer = 8000
            this.charge = this.param
            this.triggered = true
            return true
          }
        }
        return false
      }
      case 'collapse_wall': {
        if (element === 'kinetic') {
          this.charge += Number(damage / 100n)
          if (this.charge >= this.param) {
            this.state = 'collapsed'
            this.triggered = true
            return true
          }
        }
        return false
      }
      default:
        return false
    }
  }

  /** 击杀时充能。蓄能塔满时给全场敌人上同种元素。 */
  onKillCharging(element: Element, cap: bigint, ctx: TerrainContext): boolean {
    if (this.kind !== 'charge_tower') return false
    this.charge += 1
    if (this.charge < this.param) return false
    this.charge = 0
    this.triggered = true
    this.applyChargeToAll(element, ctx, cap)
    return true
  }

  /** 给全场敌人上同种元素（翻盘机制） */
  applyChargeToAll(element: Element, ctx: TerrainContext, cap: bigint): void {
    for (const e of ctx.enemies) {
      e.applyElement(element, 1n, cap)
    }
  }

  /** 是否阻挡弹道（崩塌后的掩体不再阻挡） */
  blocksProjectile(): boolean {
    return this.kind === 'collapse_wall' && this.state !== 'collapsed'
  }

  serialize(): { kind: string; x: number; y: number; state: string; triggered: boolean } {
    return { kind: this.kind, x: this.x, y: this.y, state: this.state, triggered: this.triggered }
  }
}

export const TERRAIN_NAME: Record<TerrainKind, string> = {
  oil_drum: '油桶',
  tidal_gate: '潮汐闸',
  rotor_vane: '旋转风障',
  collapse_wall: '崩塌掩体',
  charge_tower: '蓄能塔',
}

export const TERRAIN_DESCR: Record<TerrainKind, string> = {
  oil_drum: '受到焰元素命中即引燃，爆炸并生成 8 秒火区',
  tidal_gate: '周期开合，改变低层敌人通路',
  rotor_vane: '持续改变 140 半径内弹丸的飞行方向',
  collapse_wall: '动能伤害累计到阈值即崩塌，永久改变弹道',
  charge_tower: '蓄满后给全场敌人上同种元素（翻盘机制）',
}
