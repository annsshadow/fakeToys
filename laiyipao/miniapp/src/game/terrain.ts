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

  /**
   * 旋转风障：持续改变区域内弹丸飞行方向。
   *
   * ⚠️ 这里**不用 `Math.cos` / `Math.sin`**。
   *
   * 超越函数的精度是**实现定义**的：ECMA-262 只要求
   * "实现应近似计算"，不要求与某个参考实现逐位一致。
   * 不同 V8 版本、不同 Node 版本、不同 CPU 架构（x86 的 SSE 路径
   * 与 ARM 的实现）都可能给出相差 1 ulp 的结果，
   * 而 `Math.round` 会把这个 1 ulp 放大成**整整 1 个定点单位**
   * —— 只要真实值恰好落在 x.5 附近，两个引擎就会往不同方向 round。
   *
   * 后果不是"风障偏了一点点"：弹丸速度变了 → 命中位置变了 →
   * 击杀的 tick 变了 → **replayHash 变了** → I-6 把正常对局判成伪造。
   * 这与 README 第 2 条约束说的完全是一回事（那里禁 `Math.random`，
   * 这里该禁 `Math.cos`）。
   *
   * 解法：方向表在**构建期**用浮点算好，然后**烘焙成源码里的字面量**。
   * 运行时只做整数查表，没有任何浮点运算 ——
   * 表是数据（和反应系数表一样），两端读同一份字面量，逐位一致。
   */
  private updateRotorVane(dtMs: number, ctx: TerrainContext): TerrainEffect {
    // 角度也是整数（0..359）。原来是浮点累加，同样违反定点铁律。
    this.angle = (this.angle + Math.floor((this.param * dtMs) / 1000)) % 360
    const ux = BigInt(ROTOR_DIR_1000[this.angle * 2])
    const uy = BigInt(ROTOR_DIR_1000[this.angle * 2 + 1])
    // 幅度 = param × 0.4，方向取自整数表。
    // 用整数乘除避免 Math.round：round(x) 在 x.5 边界上的行为依赖浮点误差，
    // 而这里 x 已经是精确的千分比，直接截断即可（方向表已按千分比存）。
    const mag = BigInt(this.param) * 4n / 10n
    const deflectX = (ux * mag * 2n) / 1000n
    const deflectY = (uy * mag * 2n) / 1000n
    // ⚠️ 作用半径 140 → 260。
    //
    // 实测（balance.probe 的「地形生效率」）：6 个风障的作用范围内
    // **一次弹丸都没进过**（偏转计数恒为 0）。
    //
    // 原因是弹道很"窄"：弹丸从 (60,880) 直线飞向敌人，
    // 在风障所在的 x 处，弹丸的 y 只在 [623, 935] 之间的一个窄带里。
    // 而风障被随机放在 y ∈ [600,900]，与那条窄带错开的概率很高。
    // 半径 140 时只要错开 140 就完全无效。
    //
    // 260 让风障真正成为"覆盖一片区域的偏转场"，
    // 与它作为「风障」的视觉体量相称。
    for (const p of ctx.projectiles) {
      if (ctx.within(this.x, this.y, 260, p.x, p.y)) {
        p.vx += deflectX
        p.vy += deflectY
      }
    }
    return { blocked: false }
  }

  /**
   * 崩塌掩体：受动能伤害累计到阈值即崩塌 → 弹道永久改变。
   *
   * ⚠️ 这里**不做任何充能**。曾经这里每 tick 扫一遍半径 110 内的动能弹丸
   * `charge += 1`，但那条路径在几何上**永远走不到**：
   *   - engine 的 blocksProjectile 判定半径是 **100**，
   *     弹丸进入 110 的瞬间就已经在 100 内被标记 dead 并从 projectiles 里滤掉
   *   - step() 里 updateProjectiles 先于 updateTerrain
   * 所以 updateCollapseWall 永远看不到任何弹丸，
   * 掩体的 charge 恒为 0，param 调到多大都没用。
   *
   * 唯一的充能来源是 onHit（按命中伤害累计），那条路径是通的。
   * 留一个走不到的分支比没有分支更糟：它让人以为机制在工作。
   */
  private updateCollapseWall(_ctx: TerrainContext): TerrainEffect {
    if (this.state === 'collapsed') return { blocked: false }
    return { blocked: true }
  }

  /** 蓄能塔：充能由击杀驱动（见 onKillCharging），这里只做计时展示 */
  private updateChargeTower(dtMs: number): TerrainEffect {
    this.timer = dtMs
    return { blocked: false }
  }

  /**
   * 由命中结算调用：把伤害与元素喂给地形。
   *
   * ⚠️ 必须带位置判定。engine 的调用点是
   *   `for (const t of this.terrains) if (t.onHit(p.element, res.totalDamage))`
   * —— **对所有地形都调一遍，不看弹丸与地形是否挨着**。
   * 于是「站在 x=900 的油桶」会被「打到 x=200 的敌人」的火焰点燃，
   * 机制语义整个错掉：玩家无法通过站位影响地形，只能靠无脑堆同元素。
   *
   * 加了位置判定之后，「先清掉挡在前面的怪、把火引到油桶」才成为可执行的策略 ——
   * 这正是 I-4 想提供的「地形既是解法也是约束」。
   */
  onHit(element: Element, damage: bigint, hitX: bigint, hitY: bigint): boolean {
    if (this.state === 'collapsed') return false
    // 判定半径比 blocksProjectile(100) 略大，保证"擦边命中"也算。
    //
    // ⚠️ this.x / this.y 是**逻辑单位**（number，来自关卡配置），
    // 而 hitX / hitY 是**定点整数**（bigint，来自引擎内部坐标）。
    // 两者直接相减会得到 NaN 之外的无意义结果 —— 而且是静默的。
    // 坐标约定见文件头：配置用逻辑单位，内部实体用定点（×1000）。
    const tx = toFixed(this.x)
    const ty = toFixed(this.y)
    const near = (r: number) => {
      const rr = toFixed(r)
      const dx = hitX - tx
      const dy = hitY - ty
      return dx * dx + dy * dy <= rr * rr
    }
    switch (this.kind) {
      case 'oil_drum': {
        if (element === 'fire' && this.state === 'idle' && near(120)) {
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
        // 动能打进掩体。半径 100 与 blocksProjectile 对齐 ——
        // 弹丸要真的"打到墙上"才算，不该有擦边充能。
        //
        // ⚠️ 任何元素都能充能，动能 ×3 —— 曾经是**只有动能能充能**。
        //
        // 「硬性克制」在这里等于「硬性死锁」：掩体会挡掉弹丸，
        // 而如果它只能被动能打破，那么不带动能的构筑**永远打不开弹道**。
        // 默认构筑（内容表前 4 个主动技能）恰好是 fire/fire/fire/ice，
        // 一个动能都没有 —— 于是第 22/33/34 关对默认构筑是不可通关的。
        //
        // 改成「动能快 3 倍、其余也能砸」之后：
        //   - 带动能：仍是明显更优解（3 倍充能速度），战术意图保留
        //   - 不带动能：能砸开，只是慢，**关卡不会变成死局**
        //
        // 一般原则：地形机制可以有**偏好**，不能有**唯一解**。
        if (near(100)) {
          const mul = element === 'kinetic' ? 3 : 1
          this.charge += Number(damage / 100n) * mul
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
/**
 * ROTOR_DIR_1000 是旋转风障的单位方向表：**整数千分比**的方向向量。
 *
 * 扁平存放，下标 `角度*2` 是 x（cos）、`角度*2+1` 是 y（sin），
 * 角度范围 0..359 度。共 720 个数，
 * 是**构建期**用浮点算好后烘焙进源码的字面量。
 *
 * ⚠️ 为什么不留浮点表：ECMA-262 只要求 `Math.cos` 是"实现近似的"，
 * 不要求跨实现逐位一致。不同 V8 版本、不同 CPU 架构可能差 1 ulp，
 * 而 1 ulp 在 `Math.round` 的 x.5 边界上会被放大成整整 1 个单位。
 * 风障改的是弹丸速度 → 命中位置变 → 击杀 tick 变 → replayHash 变
 * → I-6 把正常对局判成伪造。
 *
 * 烘焙成字面量后，运行时只有整数查表与整数乘除，**零浮点运算**。
 * 表是数据（性质同反应系数表），两端读同一份字面量，逐位一致。
 * 若将来要改表，必须两端同步改，并重跑 I-6 端到端。
 *
 * 校验：每项的模长应为 1000（允许 ±1 的四舍五入误差），
 * 由 terrain.test.ts 的「方向表是单位向量」守住。
 */
export const ROTOR_DIR_1000: ReadonlyArray<number> = [
  1000, 0, 1000, 17, 999, 35, 999, 52, 998, 70, 996, 87, 995, 105, 993, 122, 990, 139, 988, 156,
  985, 174, 982, 191, 978, 208, 974, 225, 970, 242, 966, 259, 961, 276, 956, 292, 951, 309, 946, 326,
  940, 342, 934, 358, 927, 375, 921, 391, 914, 407, 906, 423, 899, 438, 891, 454, 883, 469, 875, 485,
  866, 500, 857, 515, 848, 530, 839, 545, 829, 559, 819, 574, 809, 588, 799, 602, 788, 616, 777, 629,
  766, 643, 755, 656, 743, 669, 731, 682, 719, 695, 707, 707, 695, 719, 682, 731, 669, 743, 656, 755,
  643, 766, 629, 777, 616, 788, 602, 799, 588, 809, 574, 819, 559, 829, 545, 839, 530, 848, 515, 857,
  500, 866, 485, 875, 469, 883, 454, 891, 438, 899, 423, 906, 407, 914, 391, 921, 375, 927, 358, 934,
  342, 940, 326, 946, 309, 951, 292, 956, 276, 961, 259, 966, 242, 970, 225, 974, 208, 978, 191, 982,
  174, 985, 156, 988, 139, 990, 122, 993, 105, 995, 87, 996, 70, 998, 52, 999, 35, 999, 17, 1000,
  0, 1000, -17, 1000, -35, 999, -52, 999, -70, 998, -87, 996, -105, 995, -122, 993, -139, 990, -156, 988,
  -174, 985, -191, 982, -208, 978, -225, 974, -242, 970, -259, 966, -276, 961, -292, 956, -309, 951, -326, 946,
  -342, 940, -358, 934, -375, 927, -391, 921, -407, 914, -423, 906, -438, 899, -454, 891, -469, 883, -485, 875,
  -500, 866, -515, 857, -530, 848, -545, 839, -559, 829, -574, 819, -588, 809, -602, 799, -616, 788, -629, 777,
  -643, 766, -656, 755, -669, 743, -682, 731, -695, 719, -707, 707, -719, 695, -731, 682, -743, 669, -755, 656,
  -766, 643, -777, 629, -788, 616, -799, 602, -809, 588, -819, 574, -829, 559, -839, 545, -848, 530, -857, 515,
  -866, 500, -875, 485, -883, 469, -891, 454, -899, 438, -906, 423, -914, 407, -921, 391, -927, 375, -934, 358,
  -940, 342, -946, 326, -951, 309, -956, 292, -961, 276, -966, 259, -970, 242, -974, 225, -978, 208, -982, 191,
  -985, 174, -988, 156, -990, 139, -993, 122, -995, 105, -996, 87, -998, 70, -999, 52, -999, 35, -1000, 17,
  -1000, 0, -1000, -17, -999, -35, -999, -52, -998, -70, -996, -87, -995, -105, -993, -122, -990, -139, -988, -156,
  -985, -174, -982, -191, -978, -208, -974, -225, -970, -242, -966, -259, -961, -276, -956, -292, -951, -309, -946, -326,
  -940, -342, -934, -358, -927, -375, -921, -391, -914, -407, -906, -423, -899, -438, -891, -454, -883, -469, -875, -485,
  -866, -500, -857, -515, -848, -530, -839, -545, -829, -559, -819, -574, -809, -588, -799, -602, -788, -616, -777, -629,
  -766, -643, -755, -656, -743, -669, -731, -682, -719, -695, -707, -707, -695, -719, -682, -731, -669, -743, -656, -755,
  -643, -766, -629, -777, -616, -788, -602, -799, -588, -809, -574, -819, -559, -829, -545, -839, -530, -848, -515, -857,
  -500, -866, -485, -875, -469, -883, -454, -891, -438, -899, -423, -906, -407, -914, -391, -921, -375, -927, -358, -934,
  -342, -940, -326, -946, -309, -951, -292, -956, -276, -961, -259, -966, -242, -970, -225, -974, -208, -978, -191, -982,
  -174, -985, -156, -988, -139, -990, -122, -993, -105, -995, -87, -996, -70, -998, -52, -999, -35, -999, -17, -1000,
  0, -1000, 17, -1000, 35, -999, 52, -999, 70, -998, 87, -996, 105, -995, 122, -993, 139, -990, 156, -988,
  174, -985, 191, -982, 208, -978, 225, -974, 242, -970, 259, -966, 276, -961, 292, -956, 309, -951, 326, -946,
  342, -940, 358, -934, 375, -927, 391, -921, 407, -914, 423, -906, 438, -899, 454, -891, 469, -883, 485, -875,
  500, -866, 515, -857, 530, -848, 545, -839, 559, -829, 574, -819, 588, -809, 602, -799, 616, -788, 629, -777,
  643, -766, 656, -755, 669, -743, 682, -731, 695, -719, 707, -707, 719, -695, 731, -682, 743, -669, 755, -656,
  766, -643, 777, -629, 788, -616, 799, -602, 809, -588, 819, -574, 829, -559, 839, -545, 848, -530, 857, -515,
  866, -500, 875, -485, 883, -469, 891, -454, 899, -438, 906, -423, 914, -407, 921, -391, 927, -375, 934, -358,
  940, -342, 946, -326, 951, -309, 956, -292, 961, -276, 966, -259, 970, -242, 974, -225, 978, -208, 982, -191,
  985, -174, 988, -156, 990, -139, 993, -122, 995, -105, 996, -87, 998, -70, 999, -52, 999, -35, 1000, -17,
]
