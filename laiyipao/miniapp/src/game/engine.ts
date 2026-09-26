/**
 * 战斗引擎 —— 固定 60Hz 步进 + 确定性 + 事件队列。
 *
 * 三条铁律（违反其一 I-6 立即失效）：
 *  1. 全部数学用 bigint 定点整数（fixed.ts）
 *  2. 全部随机走 BattleRng（lcg.ts），禁止 Math.random
 *  3. 固定步长推进，不依赖真实时间（渲染层可丢帧，但逻辑不可变速）
 *
 * 表现层通过 drainEvents() 取事件做动画，不直接读引擎内部状态。
 */

import { BattleRng, fnv1a64, hex16 } from './lcg'
import {
  PERMILLE,
  ELEMENT_PER_STACK_BASE,
  mulDiv,
  maxBig,
  clampInt64,
  applyArmor,
  MAX_RESIST,
} from './fixed'
import {
  ELEMENT_ORDER,
  REACTIONS,
  REACTION_ORDER,
  type Element,
  type ReactionKey,
} from './elements'
import {
  Defender,
  resolveHit,
  applyElementStacks,
  type Attacker,
  type HitResult,
} from './damage'
import {
  HeatSystem,
  CardDeck,
  rollWaveCards,
  type Card,
  type EquippedSkill,
  type AttributeEffect,
  type MechanicEffect,
  ACTIVE_SLOTS,
} from './heatmap'
import { Terrain, BASE_X, BASE_Y, FIELD_W, FIELD_H, toFixed } from './terrain'
import type {
  Enemy,
  EnemyDef,
  FloatText,
  GeneratedLevel,
  Projectile,
  ReplayEvent,
  ReplayEventType,
  SkillDef,
} from './types'

export const TICK_HZ = 20
/**
 * 每 tick 的毫秒数。
 *
 * ⚠️ 必须是整数：BigInt(TICK_MS) 在 TICK_MS 为小数时会直接抛
 * RangeError，而 1000/60 = 16.666… 正是一个小数。
 *
 * 固定步长常用做法是把 tick 间隔取整：这里取 50ms（20Hz 逻辑步），
 * 渲染层做插值。宁可步长粗一点，也不要引入浮点时间 ——
 * 浮点时间会让回放哈希在跨设备时漂移。
 */
export const TICK_MS = 50

/**
 * 漏怪推进伤害的分母：漏怪伤害 = base_hp / LeakDamageDivisor。
 *
 * ⚠️ 曾经的实现是拿**敌人自己的血量**当漏怪伤害
 * （pplyArmor(e.maxHp, ...)），于是血量调到 8 倍时漏一只就打死满血防线
 * （base_hp=1000、漏怪伤害 560~2080）。这让平衡空间被压成一条窄缝：
 * 血量低时全清、成长收益为 0；血量高时"漏一只即败"、同样没有成长空间。
 *
 * 取 10 的依据：base_hp=1000 时漏 10 只才归零，
 * 配合"漏怪会扣分"（击杀分拿不到）已经形成足够压力，
 * 不需要让单次失误直接终结战局。
 */
export const LeakDamageDivisor = 10n

/** TICK_MS 的 bigint 形式，供定点运算直接使用 */
export const TICK_BIG = BigInt(TICK_MS)

/**
 * 坐标系约定（重要）：
 *  - 逻辑空间 0..FIELD_W（1000），与地形/关卡配置一致，服务端也用它
 *  - 引擎内部所有坐标是**定点整数**（逻辑值 × 1000）
 *
 * ⚠️ 混用这两者会让敌人瞬间穿过全场 —— 逻辑上 960 单位的距离，
 * 若误当成 960 个定点单位（实际 0.96 逻辑单位），移动速度会放大 1000 倍。
 * 所有从逻辑空间转换的地方必须走 terrain.toFixed()。
 */
export { toFixed } from './terrain'

export type BattlePhase = 'prepare' | 'wave' | 'card_select' | 'won' | 'lost'

export interface BattleConfig {
  level: GeneratedLevel
  enemies: Map<number, EnemyDef>
  skills: Map<number, SkillDef>
  equipped: EquippedSkill[]
  attacker: Attacker
  seed: number | bigint
}

/** 引擎向外抛的事件 */
export type BattleEvent =
  | { type: 'fire'; slot: number; skillId: number }
  | { type: 'hit'; x: bigint; y: bigint; damage: bigint; crit: boolean }
  | {
      type: 'reaction'
      reaction: ReactionKey
      x: bigint
      y: bigint
      damage: bigint
      resisted: boolean
    }
  | { type: 'kill'; x: bigint; y: bigint; boss: boolean }
  | { type: 'leak'; damage: bigint }
  | { type: 'wave_start'; index: number }
  | { type: 'wave_clear'; index: number }
  | { type: 'overheat' }
  | { type: 'card_offer'; cards: Card[] }
  | { type: 'card_taken'; card: Card }
  | { type: 'card_discarded'; card: Card }
  | { type: 'terrain'; kind: string; x: number; y: number }
  | { type: 'won'; score: number; stars: number }
  | { type: 'lost'; score: number }

/** 局内可变的攻方加成（由属性卡/机制卡累积） */
export interface Buffs {
  attackPermille: bigint
  elementCoefPermille: bigint
  critPermille: bigint
  heatCapBonus: bigint
  elementCapBonus: bigint
  armorPermille: bigint
  pierceBonus: number
  chainBonus: number
  aoeBonus: number
  freeDiscard: number
  overheatGuard: number
}

function newBuffs(): Buffs {
  return {
    attackPermille: 0n,
    elementCoefPermille: 0n,
    critPermille: 0n,
    heatCapBonus: 0n,
    elementCapBonus: 0n,
    armorPermille: 0n,
    pierceBonus: 0,
    chainBonus: 0,
    aoeBonus: 0,
    freeDiscard: 0,
    overheatGuard: 0,
  }
}

export class BattleEngine {
  readonly cfg: BattleConfig
  readonly rng: BattleRng
  readonly heat = new HeatSystem()
  readonly deck = new CardDeck()

  phase: BattlePhase = 'prepare'
  tick: number = 0
  elapsedMs: number = 0
  /** 当前波次序号（0 基）。UI 显示"第 N 波"用。 */
  waveIndex = 0

  baseHp: bigint
  baseHpMax: bigint
  baseArmorPermille: bigint

  enemies: Enemy[] = []
  projectiles: Projectile[] = []
  floats: FloatText[] = []
  terrains: Terrain[] = []

  buffs: Buffs = newBuffs()
  skills: EquippedSkill[]

  // 统计
  shots = 0
  hits = 0
  kills = 0
  leaked = 0
  reactionsCount = 0
  score = 0
  heatMax = 0

  elementsUsed: Record<string, number> = {}
  reactionsUsed: Record<string, number> = {}
  terrainUsed: string[] = []

  private events: BattleEvent[] = []
  private replay: ReplayEvent[] = []
  private uidSeq = 1
  private spawnQueue: Array<{ enemyId: number; atTick: number; uid: number }> = []

  /**
   * 本关总怪数（开波前即可确定，与随机无关）。
   *
   * 存在的原因：胜负判定必须与服务端口径一致。
   * 服务端 battle.go 的 `res.Win = result=="win" && kills >= MaxKillsFor(level)`，
   * 而引擎原先的「spawnQueue 空 && enemies 空」判定会把**漏掉的敌人**
   * 也算成清空（漏怪时 e.dead=true，随后被 filter 出 enemies 数组）。
   * 于是玩家撑住防线并清空全部波次、但过程中漏了 1 只怪时：
   * 客户端播通关动画、报 result='win'，服务端回 res.Win=false。
   * 体验上是「我明明打完了，为什么结算说没赢」。
   */
  readonly totalEnemies: number

  /**
   * 无攻击力敌人漏进防线时的推进伤害。
   *
   * 取 base_hp 的 1/LeakDamageDivisor。Divisor 越大越宽容 ——
   * base_hp=1000、Divisor=10 时漏 10 只才归零，
   * 给玩家「失误几次还能救」的空间，而不是「漏一只就结束」。
   *
   * ⚠️ 它必须**只与关卡血量挂钩、��敌人血量无关**，
   * 否则调敌人血量会连带把生存难度也改了，
   * 两个变量纠缠在一起就没法单独调平衡。
   */
  private get breachDamage(): bigint {
    const d = BigInt(LeakDamageDivisor)
    return this.baseHpMax > 0n ? this.baseHpMax / d : 100n
  }

  constructor(cfg: BattleConfig) {
    this.cfg = cfg
    this.rng = new BattleRng(cfg.seed)
    this.skills = cfg.equipped.map((s) => ({ ...s, cooldownRemaining: 0 }))

    this.baseHp = BigInt(cfg.level.base_hp)
    this.baseHpMax = BigInt(cfg.level.base_hp)
    this.baseArmorPermille = BigInt(cfg.level.armor_permille || 0)

    let total = 0
    for (const w of cfg.level.waves ?? []) {
      for (const sp of w.spawns) total += sp.count
    }
    this.totalEnemies = total

    for (const t of cfg.level.terrain ?? []) {
      this.terrains.push(new Terrain(t))
    }
  }

  // ---------- 生命周期 ----------

  /** 开始第一波 */
  start(): void {
    this.phase = 'wave'
    this.beginWave(0)
  }

  /**
   * 每波选中的手牌索引（-1 = 整波跳过）。
   *
   * ⚠️ 这是 I-6 重放闭环的最后一环：选牌会改变后续战斗
   * （技能升格 / 属性加成 / 机制词条），第三方拿不到这个序列
   * 就复现不出原局，验真会把正常对局判成伪造。
   *
   * 索引而非卡牌 id：同一关卡 + 同一种子 → 同样的手牌顺序 → 同样的索引。
   */
  cardPicks: number[] = []

  /**
   * 重放脚本：外部注入的选牌序列。
   *
   * 注入后引擎在进入 card_select 时**自动**按脚本决策，
   * 不再等待玩家输入 —— 这正是重放需要的无人值守模式。
   */
  private replayScript: number[] | null = null
  private replayScriptPos = 0

  /** 注入重放脚本。传 null 回到交互模式 */
  setReplayScript(picks: number[] | null): void {
    this.replayScript = picks
    this.replayScriptPos = 0
  }

  /**
   * 是否处于重放脚本模式。
   *
   * 外部驱动循环（如 replay.ts 的 runToEnd）必须先问这个：
   * 脚本模式下**不能**自己调 skipCards()，否则会抢在引擎的
   * 脚本决策之前把手牌丢掉 —— 表现为"脚本看起来完全没生效"。
   */
  hasReplayScript(): boolean {
    return this.replayScript !== null
  }

  /** 取走并清空事件队列（渲染层调用） */
  drainEvents(): BattleEvent[] {
    const out = this.events
    this.events = []
    return out
  }

  private emit(e: BattleEvent): void {
    this.events.push(e)
  }

  private record(t: number, type: ReplayEventType, a: number, b = 0, c = 0): void {
    this.replay.push({ t, type, a, b, c })
  }

  /**
   * 计算本局回放哈希（I-6）。
   *
   * 哈希覆盖**全部影响结果的输入**，不只是"恰好发生的事件"：
   *   前缀 = 关卡 id + 攻方系数 + 技能槽配置
   *   主体 = 事件序列
   *
   * ⚠️ 为什么必须带前缀：若只哈希事件流，当一场战斗全是漏怪（kills=0）
   * 时事件里没有任何伤害数值 —— 此时改动攻击力、元素系数都不改变哈希，
   * 作弊者就能"换一套属性"而不被验真发现。前缀把这类改动纳入证伪范围。
   *
   * 注：种子不进前缀。种子的影响已经完整体现在事件流里
   * （刷怪洗牌、暴击滚点、命中次序），重复计入反而会让两端更难对齐。
   */
  replayHash(): string {
    const a = this.currentAttacker()
    const skills = this.skills
      .map((s) => `${s.slot}:${s.skillId}:${s.baseDamage}:${s.applyStacks}:${s.heatCost}`)
      .sort()
      .join(',')
    const prefix =
      `L${this.cfg.level.id}` +
      `|A${a.attack}.${a.critPermille}.${a.critMultiplierPermille}` +
      `.${a.reactionMultPermille}.${a.elementCap}.${a.reactionTier}.${a.elementCoefPermille}` +
      `|S${skills}`

    // 事件序列编码成紧凑文本，保证跨端一致
    const parts: string[] = [prefix]
    for (const e of this.replay) {
      parts.push(`${e.t}|${e.type}|${e.a}|${e.b}|${e.c}`)
    }
    return hex16(fnv1a64(parts.join(';')))
  }

  get replayEventCount(): number {
    return this.replay.length
  }

  // ---------- 波次 ----------

  private beginWave(index: number): void {
    this.waveIndex = index
    this.phase = 'wave'
    this.deck.newWave()
    this.deck.discardsLeft += this.buffs.freeDiscard

    const wave = this.cfg.level.waves[index]
    if (!wave) {
      // ⚠️ 空关卡不是胜利。
      // 原先这里是 finish(true)：waves=[] 的关卡 start() 立刻判胜，
      // 而服务端 MaxKillsFor(gl)=0 让 `kills(0) >= 0` 成立，照样发掉落。
      // 畸形关卡因此变成"秒胜 + 发钱"。
      // 当前 levelgen 必填 >=5 波，所以不可达；但 level 是网络数据，
      // 缺失字段时这个分支正是入口。
      this.finish(false)
      return
    }
    this.spawnQueue = []
    for (const group of wave.spawns) {
      // ⚠️ delay/interval 的单位是**毫秒**（levelgen.go 生成时按 ms，
      // 字段注释也写明「毫秒」），而 atTick 是 tick 数（每 tick 50ms）。
      // 直接相加等于把整关节奏放慢 50 倍：
      // 实测第 1 关第一波 delay=1012ms 被当成 1012 tick = 50.6 秒才出第一只怪，
      // 全关 39 只怪要刷 18.7 分钟，期间玩家在空场干等。
      //
      // 这里统一折算成 tick，并向上取整 —— 向上取整保证
      // 「间隔 N 毫秒」不会因为截断变成「间隔 0 毫秒」而让同组怪同时出现。
      const delayTicks = Math.ceil(group.delay / TICK_MS)
      const intervalTicks = Math.ceil(group.interval / TICK_MS)
      for (let i = 0; i < group.count; i++) {
        this.spawnQueue.push({
          enemyId: group.enemy_id,
          atTick: this.tick + delayTicks + i * intervalTicks,
          uid: this.uidSeq++,
        })
      }
    }
    // 确定性洗牌：同一波内的出场顺序由 PRNG 决定
    for (let i = this.spawnQueue.length - 1; i > 0; i--) {
      const j = this.rng.intn(i + 1)
      const tmp = this.spawnQueue[i]
      this.spawnQueue[i] = this.spawnQueue[j]
      this.spawnQueue[j] = tmp
    }

    this.emit({ type: 'wave_start', index })
    this.record(this.tick, 'wave', index)
  }

  private spawnEnemy(enemyId: number, uid: number): void {
    const def = this.cfg.enemies.get(enemyId)
    if (!def) return
    const resist = new Map<Element, bigint>()
    for (const e of ELEMENT_ORDER) {
      // 字段名与服务端 json tag 对齐（snake_case）
      const v = def.resist?.[e] ?? 0
      resist.set(e, clampInt64(BigInt(v), -MAX_RESIST, MAX_RESIST))
    }
    const e: Enemy = {
      uid,
      defId: enemyId,
      name: def.name,
      category: def.category,
      x: toFixed(FIELD_W - 40),
      y: toFixed(600 + this.rng.intn(340)),
      hp: BigInt(def.hp),
      maxHp: BigInt(def.hp),
      shield: BigInt(def.shield_hp),
      // ⚠️ 这里**只能**是敌人自身的护甲，不能加 buffs.armorPermille。
      //
      // buffs.armorPermille 来自卡牌「加固工事：防线护甲 +10%」，
      // 玩家预期是**减少漏怪时的伤害**。但它曾被加到敌人的护甲上，
      // 于是效果完全反向：一张写着"防线护甲"的卡让玩家变弱 ——
      // 玩家所有伤害打敌人时先被减 10%，而漏怪伤害一分不减免。
      //
      // 正确的消费点是漏怪结算的两处 applyArmor（见 updateEnemies），
      // 那里用的才是 this.baseArmorPermille。
      armorPermille: BigInt(def.armor || 0),
      flyHeight: def.fly_height,
      burrow: def.burrow,
      isBoss: def.is_boss,
      resist,
      stacks: new Map(),
      applyElement: (el: Element, add: bigint, cap: bigint) => {
        const cur = e.stacks.get(el) ?? 0n
        const next = cur + add > cap ? cap : cur + add
        if (next !== cur) e.stacks.set(el, next)
      },
      speed: BigInt(def.speed),
      attack: BigInt(def.attack),
      attackRange: BigInt(def.attack_range),
      attackInterval: def.attack_interval,
      attackCooldown: def.attack_interval,
      frozenMs: 0,
      stunnedMs: 0,
      slowedMs: 0,
      amplifyPermille: 0n,
      knockback: 0n,
      dead: false,
      spawnProgress: 0,
      hitFlashMs: 0,
    }
    this.enemies.push(e)
  }

  /**
   * 推进敌人位置。抽出来是为了让引擎内部调用与测试调用走同一份逻辑 ——
   * 之前 updateEnemies 在 tick 开头就做了位移，导致测试无法单独验证。
   */
  stepEnemyMotion(e: Enemy, frozenAll: boolean): void {
    if (e.dead) return
    if (e.spawnProgress < 1) {
      e.spawnProgress = Math.min(1, e.spawnProgress + 0.08)
      return
    }
    if (e.hitFlashMs > 0) e.hitFlashMs -= TICK_MS
    if (e.frozenMs > 0) e.frozenMs -= TICK_MS
    if (e.stunnedMs > 0) e.stunnedMs -= TICK_MS
    if (e.slowedMs > 0) e.slowedMs -= TICK_MS

    const frozen = e.frozenMs > 0 || e.stunnedMs > 0 || frozenAll
    if (frozen) return

    if (e.knockback !== 0n) {
      e.x += e.knockback
      e.knockback = 0n
    } else {
      const slowFactor = e.slowedMs > 0 ? 500n : 1000n
      const step = mulDiv(e.speed, slowFactor, 1000n) * TICK_BIG / 1000n
      e.x -= step
    }
  }

  // ---------- 主循环 ----------

  /** 推进一个固定步长。返回本步产生的事件 */
  step(): BattleEvent[] {
    if (this.phase === 'won' || this.phase === 'lost') return this.drainEvents()

    // ⚠️ 重放决策必须在 tick++ **之前**执行，否则会多消耗一个 tick。
    // tick 会写进每条 replay 事件（record(tick, ...)），
    // 多一格会让后续所有事件时间戳整体偏移 → 哈希必然不同。
    // 原局里玩家点牌同样不推进 tick，两者必须严格一致。
    if (this.phase === 'card_select' && this.replayScript !== null) {
      this.applyReplayDecision()
      // 决策可能已推进到下一波（deck 空时 takeCard 会调 beginWave），
      // 也可能仍是 card_select（脚本要求再选一张）。两种情况都继续本 tick。
    }

    this.tick++
    this.elapsedMs += TICK_MS

    this.heat.update(TICK_MS)
    this.heat.tickCooldown(TICK_MS, this.skills)

    if (this.phase === 'wave') {
      this.updateSpawns()
      this.updateEnemies()
      this.updateAutoFire()
      this.updateProjectiles()
      this.updateTerrain()
      this.updateFloats()
      this.checkWaveEnd()
    }

    this.heatMax = Number(this.heat.maxHeatThisBattle)

    return this.drainEvents()
  }

  private updateSpawns(): void {
    for (let i = this.spawnQueue.length - 1; i >= 0; i--) {
      if (this.spawnQueue[i].atTick <= this.tick) {
        this.spawnEnemy(this.spawnQueue[i].enemyId, this.spawnQueue[i].uid)
        this.spawnQueue.splice(i, 1)
      }
    }
  }

  private updateEnemies(): void {
    const frozenAll = this.heat.overheated
    for (const e of this.enemies) {
      if (e.dead) continue
      this.stepEnemyMotion(e, frozenAll)

      // 远程敌人攻击
      //
      // ⚠️ `e.x > BASE_X` 这个条件是必需的，不是优化。
      // 下面的「抵达防线」分支已经在处理 e.x <= BASE_X 的敌人。
      // 曾经两个 if 顺序执行、没有互斥：敌人越过防线后
      // `dist = e.x - BASE_X` 变成负数，`<= attackRange` 必然成立，
      // 于是同一个 tick 里既远程打一次、又按抵达扣一次 ——
      // 双倍伤害、leaked 计 2、发出两条 leak 事件。
      // 内容表里远程敌人（attack_range 150~600）大量存在，
      // 每个抵达时都触发一次，防线血量被额外扣掉接近一倍。
      if (!e.burrow && e.attack > 0n && e.attackRange > 0n && e.x > toFixed(BASE_X)) {
        e.attackCooldown -= TICK_MS
        if (e.attackCooldown <= 0) {
          e.attackCooldown = e.attackInterval
          const dist = e.x - toFixed(BASE_X)
          if (dist <= toFixed(Number(e.attackRange))) {
            const dmg = applyArmor(e.attack, this.defenseArmorPermille())
            this.baseHp -= dmg
            this.leaked++
            this.emit({ type: 'leak', damage: dmg })
            this.record(this.tick, 'leak', uidOf(dmg))
            if (this.baseHp <= 0n) {
              this.baseHp = 0n
              this.finish(false)
              return
            }
          }
        }
      }

      // 抵达防线
      if (e.x <= toFixed(BASE_X)) {
        e.dead = true
        const dmg = this.leakDamage(e)
        this.baseHp -= dmg
        this.leaked++
        this.emit({ type: 'leak', damage: dmg })
        this.record(this.tick, 'leak', e.uid)
        if (this.baseHp <= 0n) {
          this.baseHp = 0n
          this.finish(false)
          return
        }
      }
    }
    this.enemies = this.enemies.filter((e) => !e.dead)
  }

  /**
   * 自动索敌开火：玩家只做微调，不做逐发操作。
   *
   * 热量检查放在索敌**之后**：先确认有目标再扣热量，
   * 否则一波末尾敌人还没刷完就会白白积热并过热，
   * 玩家会看到"我没出手却过热了"这种无法理解的死法。
   */
  private updateAutoFire(): void {
    if (this.heat.overheated) return

    // ⚠️ 死区兜底：必须在遍历技能**之前**做一次。
    //
    // 死区怎么形成的：tryCast 允许热量正好到 cap（`heat + cost > cap` 才拒），
    // 而 checkOverheat 要求 heat >= cap。于是当
    //   cap - 最便宜技能热量 < heat < cap
    // 时，没有任何技能能放（都超 cap），可 checkOverheat 又不触发（没到 cap）。
    // heat 停在这个区间里出不来，战斗永久停摆。
    //
    // 实测后果（第 1 关、4 个技能 20/18/22/20 热量、seed 12345）：
    // 385 秒只开出 5 发、0 杀 10 漏、0 星 —— 游戏完全不可玩。
    //
    // 修法：进自动循环前，若「任何技能都放不出」就强制走一次过热。
    // 语义上也说得通 —— 热量已经高到连最便宜的技能都用不起了，
    // 这在 I-2 的框架里就是「过热」，玩家的正确反应本就是停手。
    //
    // 注意必须在 `pickTarget` 之前：无目标的空场里推进热量到死区
    // 同样需要能被过热解救，否则玩家在等刷怪时也会卡死。
    if (this.heatStuck()) {
      this.heat.forceOverheat()
      this.emit({ type: 'overheat' })
      this.record(this.tick, 'overheat', -1)
      return
    }

    for (const s of this.skills) {
      if (s.slot >= ACTIVE_SLOTS) continue // 被动槽不主动释放
      if (s.cooldownRemaining > 0) continue
      if (this.heat.heat + s.heatCost > this.heat.cap) continue

      const target = this.pickTarget(s)
      if (!target) continue

      if (!this.heat.tryCast(s.heatCost)) continue
      s.cooldownRemaining = s.cooldownMs
      this.fire(s, target)

      // ⚠️ 必须在开火后检查过热 —— 热量打满的那一刻正是"用尽最后一点资源"的时刻。
      // 若在开火前检查，玩家会看到"我还没出手就过热了"。
      //
      // 这个调用一旦漏掉，热量会永久卡在上限、再也放不出技能（衰减虽在，
      // 但不会触发过热状态），战斗直接停摆 —— 属于静默失效。
      if (this.heat.checkOverheat()) {
        this.emit({ type: 'overheat' })
        this.record(this.tick, 'overheat', this.skillIndexOf(s))
        break // 一次性触发，避免同帧多个技能重复发事件
      }
    }
  }

  /**
   * 漏进防线一只敌人造成的伤害。
   *
   * ⚠️ 这里曾经是 `e.attack > 0 ? e.attack : applyArmor(e.maxHp, ...)`，
   * 也就是**拿敌人的血量当漏怪伤害**。后果不是"数值偏大"，而是：
   *
   *   血量 ×8  →  漏怪伤害 560~2080  →  base_hp(1000) 漏一只就归零
   *   血量 ×16 →  漏一只必死
   *
   * 于是平衡空间被压成一条极窄的缝：血量低到玩家能全清时，
   * 分数被击杀分锁死、所有成长维度收益为 0；血量一高就变成
   * "漏一只即败"，成长维度同样没有空间（只剩"从失败到成功"的跳变）。
   * 实测 (弹速, 血量) 网格里 18 个组合没有一个落在健康区间。
   *
   * 正确的语义：漏怪的代价由**推进本身**决定，而不是由这只怪有多硬决定。
   * 有攻击力的敌人按攻击力算；无攻击力的杂兵给一个固定的推进伤害 ——
   * 大致是"漏掉它相当于丢掉 base_hp 的一个固定比例"，
   * 于是「漏得多」是渐进惩罚（可调优的难度曲线），
   * 而不是「血量一改就变成一击必杀」的悬崖。
   */
  private leakDamage(e: Enemy): bigint {
    if (e.attack > 0n) return applyArmor(e.attack, this.defenseArmorPermille())
    return applyArmor(this.breachDamage, this.defenseArmorPermille())
  }

  /**
   * 防线的有效护甲 = 关卡基础护甲 + 卡牌加成。
   *
   * ⚠️ 卡牌加成（buffs.armorPermille）只在这里生效。
   * 它曾经被加到 `spawnEnemy` 里敌人的护甲上，效果完全反向 ——
   * 写着「防线护甲 +10%」的卡让玩家变弱。专精树的 armor 节点与
   * 壁垒石装备走的是同一条路径，所以整条「防线护甲」成长线都是反的。
   *
   * applyArmor 内部已有 MAX_ARMOR=750‰ 封顶，
   * 所以「关卡 250‰ + 叠 6 张卡 600‰」会收敛到 750‰，那是设计内的 clamp。
   */
  private defenseArmorPermille(): bigint {
    return this.baseArmorPermille + this.buffs.armorPermille
  }

  /**
   * 是否已陷入「放不出任何技能」的死区。
   *
   * 判据：存在至少一个已装备的主动技能（否则不算被困），
   * 且它的热量需求放不进当前剩余额度。
   * 冷却中的技能不参与判据 —— 冷却会自己结束，不构成永久困住。
   */
  private heatStuck(): boolean {
    const costs: bigint[] = []
    for (const s of this.skills) {
      if (s.slot >= ACTIVE_SLOTS) continue // 被动槽不消耗热量，不参与判据
      costs.push(s.heatCost)
    }
    return this.heat.isStuckFor(costs)
  }

  /** 在技能列表里找下标，找不到返回 -1（仅用于事件记录） */
  private skillIndexOf(s: EquippedSkill): number {
    return this.skills.findIndex((x) => x.skillId === s.skillId)
  }

  /** 索敌优先级：BOSS/精英 > 最近目标；优先能命中的（溅射/穿透/高伤） */
  private pickTarget(s: EquippedSkill): Enemy | null {
    const live = this.enemies.filter((e) => !e.dead && e.spawnProgress >= 1)
    if (live.length === 0) return null
    // 飞行/钻地敌人只能被溅射或穿透命中，优先安排给对应技能
    const evasive = live.filter((e) => e.flyHeight > 0 || e.burrow)
    if ((s.aoeRadius > 0 || s.pierce > 0) && evasive.length > 0) {
      return evasive.reduce((a, b) => (a.x <= b.x ? a : b))
    }
    // 其余取最靠近防线的
    return live.reduce((a, b) => (a.x <= b.x ? a : b))
  }

  private fire(s: EquippedSkill, target: Enemy): void {
    this.shots++
    const speed = BigInt(s.projectileSpeed || 60000)
    const dx = target.x - toFixed(BASE_X)
    const dy = target.y - toFixed(BASE_Y)
    const len = maxBig(1n, sqrtApprox(dx * dx + dy * dy))
    // 归一化方向（定点，保持确定性）
    const nx = (dx * speed) / len
    const ny = (dy * speed) / len

    this.projectiles.push({
      uid: this.uidSeq++,
      skillId: s.skillId,
      element: s.element,
      x: toFixed(BASE_X),
      y: toFixed(BASE_Y),
      vx: nx,
      vy: ny,
      damage: s.baseDamage,
      applyStacks: s.applyStacks,
      pierceLeft: s.pierce + this.buffs.pierceBonus,
      chainLeft: s.chain + this.buffs.chainBonus,
      hitSet: new Set(),
      aoeRadius: s.aoeRadius + this.buffs.aoeBonus,
      dead: false,
      trailLen: 0,
      slot: s.slot,
    })
    this.emit({ type: 'fire', slot: s.slot, skillId: s.skillId })
    this.record(this.tick, 'fire', s.skillId, s.slot)
  }

  private updateProjectiles(): void {
    // 每 tick 分若干等分子步进，降低高速弹丸穿透漏判
    const SUB_STEPS = 2
    const tickBig = BigInt(TICK_MS)
    for (const p of this.projectiles) {
      if (p.dead) continue
      for (let s = 0; s < SUB_STEPS; s++) {
        p.x += p.vx / tickBig
        p.y += p.vy / tickBig
        p.trailLen = p.trailLen + 1

        // 越界（用定点边界）
        if (p.x < 0n || p.x > toFixed(FIELD_W) || p.y < 0n || p.y > toFixed(FIELD_H)) {
          p.dead = true
          break
        }
        // 被崩塌前的掩体阻挡
        for (const t of this.terrains) {
          if (t.blocksProjectile() && this.withinTerrain(t.x, t.y, 100, p.x, p.y)) {
            p.dead = true
            break
          }
        }
        if (p.dead) break

        this.checkProjectileHit(p)
        if (p.dead) break
      }
    }
    this.projectiles = this.projectiles.filter((p) => !p.dead)
  }

  private checkProjectileHit(p: Projectile): void {
    for (const e of this.enemies) {
      if (e.dead || e.spawnProgress < 1) continue
      if (p.hitSet.has(e.uid)) continue
      if (!this.hitEnemy(p, e)) continue

      this.hits++
      p.hitSet.add(e.uid)

      // 溅射
      if (p.aoeRadius > 0) {
        this.applyAoe(p, e, p.aoeRadius)
      }
      // 弹射
      if (p.chainLeft > 0) {
        const next = this.enemies
          .filter((x) => !x.dead && !p.hitSet.has(x.uid))
          .reduce<Enemy | null>((best, x) => {
            if (!best) return x
            const d1 = dist2(p.x, p.y, x.x, x.y)
            const d2 = dist2(p.x, p.y, best.x, best.y)
            return d1 < d2 ? x : best
          }, null)
        if (next) {
          p.chainLeft--
          p.x = next.x
          p.y = next.y
          p.hitSet.add(next.uid)
          continue
        }
      }
      // 穿透
      if (p.pierceLeft > 0) {
        p.pierceLeft--
        continue
      }
      p.dead = true
      return
    }
  }

  /** 对单个敌人结算一次命中 */
  private hitEnemy(p: Projectile, e: Enemy): boolean {
    if (!this.withinEnemy(e, p.x, p.y)) return false

    // 把敌人的运行时状态包成 Defender，复用与 Go 完全一致的结算
    const def = new Defender(e.hp, e.shield, e.armorPermille)
    for (const [el, v] of e.resist) def.resist.set(el, v)
    for (const [el, v] of e.stacks) def.stacks.set(el, v)
    // 受击伤害放大（超导/急速冻结）
    if (e.amplifyPermille > 0n) {
      for (const el of ELEMENT_ORDER) {
        const cur = def.resistOf(el)
        def.resist.set(el, cur - e.amplifyPermille / 10n)
      }
    }

    const att = this.currentAttacker()
    const input = {
      skillDamage: p.damage,
      skillElement: p.element,
      applyStacks: p.applyStacks,
      roll: this.rng.roll(),
    }
    const res: HitResult = resolveHit(att, def, input)
    // 施加元素必须在伤害结算之后，否则会自我触发反应
    applyElementStacks(att, def, input)

    // 写回敌人状态
    e.hp = def.hp
    e.shield = def.shield
    e.stacks = def.stacks
    e.hitFlashMs = 140
    if (res.dispelShield && e.shield === 0n) {
      this.pushFloat(e.x, e.y, '护盾驱散', '#58a6ff', 14)
    }
    if (res.statusDurationMs > 0) {
      const spec = REACTIONS[res.reaction as ReactionKey]
      if (spec.amplifyPct > 0) {
        e.amplifyPermille = BigInt(spec.amplifyPct)
      }
      const react = res.reaction
      if (react === 'flash_freeze' || react === 'superconduct') e.frozenMs = spec.statusDurationMs
      if (react === 'overheat') e.stunnedMs = spec.statusDurationMs
    }

    this.score += Number(res.totalDamage / 100n)
    this.pushFloat(
      e.x,
      e.y,
      String(Number(res.totalDamage / 100n)),
      res.crit ? '#ffd33d' : '#e6edf3',
      res.crit ? 20 : 16,
    )
    this.emit({ type: 'hit', x: p.x, y: p.y, damage: res.totalDamage, crit: res.crit })
    this.record(this.tick, 'hit', e.uid, Number(res.totalDamage / 100n), res.crit ? 1 : 0)

    // 元素使用统计
    if (p.element) {
      this.elementsUsed[p.element] = (this.elementsUsed[p.element] ?? 0) + 1
    }

    // 反应统计
    if (res.reaction) {
      this.reactionsCount++
      this.reactionsUsed[res.reaction] = (this.reactionsUsed[res.reaction] ?? 0) + 1
      const resisted = res.resistAppliedPermille < PERMILLE
      this.emit({
        type: 'reaction',
        reaction: res.reaction,
        x: e.x,
        y: e.y,
        damage: res.reactionDamage,
        resisted,
      })
      this.pushFloat(
        e.x,
        e.y - 40n,
        REACTIONS[res.reaction].name,
        resisted ? '#8b949e' : '#7ee787',
        15,
      )
      this.record(this.tick, 'reaction', e.uid, reactionIndex(res.reaction))
    }

    // 地形联动。地形自身负责判定弹丸是否经过它（updateXxx 内部已做空间判定），
    // 因此这里只需传入元素与伤害，不需要坐标。
    for (const t of this.terrains) {
      if (t.onHit(p.element, res.totalDamage)) {
        this.terrainUsed.push(t.kind)
        this.emit({ type: 'terrain', kind: t.kind, x: t.x, y: t.y })
        this.record(this.tick, 'terrain', terrainIndex(t.kind), t.x, t.y)
      }
    }

    if (res.killed) {
      e.dead = true
      this.kills++
      this.score += e.isBoss ? 5000 : 500
      this.emit({ type: 'kill', x: e.x, y: e.y, boss: e.isBoss })
      this.record(this.tick, 'kill', e.uid, e.isBoss ? 1 : 0)
      // 蓄能塔充能：满则给全场敌人上该敌人身上的主元素
      const dom = dominantOf(e) ?? 'fire'
      for (const t of this.terrains) {
        if (t.onKillCharging(dom, this.currentAttacker().elementCap, this.terrainCtx())) {
          if (!this.terrainUsed.includes(t.kind)) this.terrainUsed.push(t.kind)
          this.emit({ type: 'terrain', kind: t.kind, x: t.x, y: t.y })
          this.record(this.tick, 'terrain', terrainIndex(t.kind), t.x, t.y)
        }
      }
    }
    return true
  }

  private applyAoe(p: Projectile, origin: Enemy, radius: number): void {
    const rr = toFixed(radius)
    for (const e of this.enemies) {
      if (e.dead || e.uid === origin.uid) continue
      if (dist2(origin.x, origin.y, e.x, e.y) > rr * rr) continue
      this.hitEnemy(p, e)
    }
  }

  private currentAttacker(): Attacker {
    const base = this.cfg.attacker
    const lv = this.cfg.level
    return {
      attack: base.attack + this.buffs.attackPermille,
      critPermille: base.critPermille + this.buffs.critPermille,
      critMultiplierPermille: base.critMultiplierPermille,
      reactionMultPermille: base.reactionMultPermille,
      elementCap: (lv.element_cap ? BigInt(lv.element_cap) : base.elementCap) + this.buffs.elementCapBonus,
      reactionTier: BigInt(lv.max_reaction_tier || 1),
      elementCoefPermille: base.elementCoefPermille + this.buffs.elementCoefPermille,
    }
  }

  private updateTerrain(): void {
    const ctx = this.terrainCtx()
    for (const t of this.terrains) {
      const eff = t.update(TICK_MS, ctx)
      if (eff.blocked) {
        // 潮汐闸关闭：低层敌人无法通过，被挡在闸门右侧
        for (const e of this.enemies) {
          if (!e.burrow && Math.abs(Number(e.y - toFixed(t.y)) / 1000) < 60) {
            e.x = maxBig(e.x, toFixed(t.x))
          }
        }
      }
    }
  }

  private terrainCtx() {
    return {
      enemies: this.enemies,
      projectiles: this.projectiles,
      terrainTick: ELEMENT_PER_STACK_BASE / 8n,
      within: (tx: number, ty: number, r: number, x: bigint, y: bigint) => {
        const rr = toFixed(r)
        return dist2(toFixed(tx), toFixed(ty), x, y) <= rr * rr
      },
      enemiesInRadius: (x: number, y: number, r: number) => {
        const rr = toFixed(r)
        return this.enemies.filter((e) => !e.dead && dist2(toFixed(x), toFixed(y), e.x, e.y) <= rr * rr)
      },
      onKill: (e: Enemy) => {
        // ⚠️ 击杀分必须在这里记，与 hitEnemy 里的那条口径一致。
        // 原先只 kills++ 不加分，于是「用油桶火区烧死」与「用弹丸打死」
        // 同样一只怪差 500 分（BOSS 差 5000）—— 分数不再只取决于战果，
        // 还取决于敌人怎么死，直接影响上报的 score 与星级判定。
        this.kills++
        this.score += e.isBoss ? 5000 : 500
        this.emit({ type: 'kill', x: e.x, y: e.y, boss: e.isBoss })
        this.record(this.tick, 'kill', e.uid, e.isBoss ? 1 : 0)
      },
      onTerrainTrigger: (kind: string, t: Terrain) => {
        this.terrainUsed.push(kind)
        this.emit({ type: 'terrain', kind, x: t.x, y: t.y })
        this.record(this.tick, 'terrain', terrainIndex(kind), t.x, t.y)
      },
    }
  }

  private withinTerrain(tx: number, ty: number, r: number, x: bigint, y: bigint): boolean {
    // tx/ty/r 是逻辑单位，x/y 是定点；半径平方需要 ×1000² 才同量纲
    return dist2(toFixed(tx), toFixed(ty), x, y) <= toFixed(r) * toFixed(r)
  }

  private withinEnemy(e: Enemy, x: bigint, y: bigint): boolean {
    // 飞行与钻地：只有溅射/穿透能命中
    const r = 28 + e.flyHeight * 0.1
    const rr = toFixed(r)
    return dist2(e.x, e.y, x, y) <= rr * rr
  }

  private pushFloat(x: bigint, y: bigint, text: string, color: string, size: number): void {
    this.floats.push({ x, y, text, color, lifeMs: 900, maxLifeMs: 900, size })
    if (this.floats.length > 60) this.floats.splice(0, this.floats.length - 60)
  }

  private updateFloats(): void {
    for (const f of this.floats) {
      f.lifeMs -= TICK_MS
      f.y -= 30n
    }
    this.floats = this.floats.filter((f) => f.lifeMs > 0)
  }

  // ---------- 波次结束 / 结算 ----------

  private checkWaveEnd(): void {
    if (this.spawnQueue.length > 0) return
    if (this.enemies.length > 0) return
    this.emit({ type: 'wave_clear', index: this.waveIndex })
    this.record(this.tick, 'wave', this.waveIndex, 1)

    const next = this.waveIndex + 1
    if (next >= this.cfg.level.waves.length) {
      // 通关语义：**清空全部波次且防线未破**。
      //
      // 这里曾经是 finish(true)，后来一度改成 kills >= totalEnemies
      // 以对齐服务端 —— 但那个口径是错的：
      // 漏怪的敌人已经扣了 base_hp（那就是「漏怪容忍度」的设计载体，
      // 见 stepEnemy / 抵达防线分支），若血还够却因为「漏过」而直接判负，
      // 等于同一件事惩罚两次且第二次更严。实测第 1 关默认构筑
      // kills=32 / leaked=7，于是永远无法通关。
      //
      // 现在两端统一为「清空即胜」：
      //   引擎：波次队列空 + 场上无敌人 + 防线未破
      //   服务端：kills + leaked >= 总怪数（两者都表示"这只怪已不再构成威胁"）
      //
      // totalEnemies > 0 是为了排除空关卡秒胜（见 beginWave 里的说明）。
      this.finish(this.totalEnemies > 0)
      return
    }
    this.phase = 'card_select'
    const cards = rollWaveCards(
      this.rng,
      this.cfg.equipped.map((s) => ({
        id: s.skillId,
        name: s.name,
        element: s.element,
        kind: s.kind,
      })),
    )
    this.deck.setHand(cards)
    this.emit({ type: 'card_offer', cards })
  }

  /** 取牌（玩家选择） */
  takeCard(id: string): Card | null {
    if (this.phase !== 'card_select') return null
    // 索引必须在 take 之前取：take 后该卡已不在 hand 里。
    //
    // ⚠️ 记录的是**手牌下标**（0/1/2），不是 cardIndex()。
    // 重放脚本按手牌下标解释，两者语义必须一致 ——
    // 混用会让重放选中完全不同的牌，且不会有任何报错。
    const handIdx = this.deck.hand.findIndex((c) => c.id === id)
    const card = this.deck.take(id)
    if (!card) return null
    this.applyCard(card)
    this.emit({ type: 'card_taken', card })
    this.record(this.tick, 'card', cardIndex(card))
    this.recordPick(handIdx)
    if (this.deck.size === 0) this.beginWave(this.waveIndex + 1)
    return card
  }

  /**
   * 记录本波选中的手牌下标（-1 = 整波跳过）。
   *
   * 语义：手牌在本次 offer 中的下标（0/1/2），对应 applyReplayDecision 的解释。
   * 每波只记第一次有效选择 —— 玩家若先弃牌后取牌，记的是取的那次。
   */
  private recordPick(handIdx: number): void {
    if (this.cardPicks.length <= this.waveIndex) {
      this.cardPicks.push(handIdx)
    }
  }

  /** 弃牌 */
  discardCard(id: string): Card | null {
    if (this.phase !== 'card_select') return null
    const card = this.deck.hand.find((c) => c.id === id)
    if (!card) return null
    const r = this.deck.discard(id)
    if (!r.ok) return null
    this.heat.refundHeat(r.refund)
    this.emit({ type: 'card_discarded', card })
    this.record(this.tick, 'card', cardIndex(card), 1)
    if (this.deck.size === 0) this.beginWave(this.waveIndex + 1)
    return card
  }

  /** 放弃全部手牌，直接进入下一波。不消耗弃牌次数，也不返还热量 */
  skipCards(): void {
    if (this.phase !== 'card_select') return
    for (const c of [...this.deck.hand]) {
      this.deck.drop(c.id)
      this.emit({ type: 'card_discarded', card: c })
      this.record(this.tick, 'card', cardIndex(c), 1)
    }
    // -1 表示整波跳过（重放时按此原样复现）
    this.recordPick(-1)
    this.beginWave(this.waveIndex + 1)
  }

  /**
   * 按重放脚本执行选牌决策。
   *
   * ⚠️ 必须在**单个 tick 内把本波所有决策做完**，而不是每个 tick 做一个。
   *
   * 原因：原局里玩家的操作是离散的、不占 tick 的 ——
   * 「取 1 张，再跳过剩下 2 张」发生在同一个 tick 间隔内。
   * 若重放把每次决策摊到不同 tick，后续所有事件的 tick 都会偏移，
   * 哈希必然不同（record 里的 tick 是哈希输入）。
   *
   * 脚本耗尽或越界时回退为"跳过整波"，保证重放一定能跑到底
   * （宁可哈希不符，也不要中途卡死让用户看到空白页）。
   */
  private applyReplayDecision(): void {
    // guard 兜底：正常一轮 while 就会离开 card_select（skipCards → beginWave），
    // 但若 beginWave 因波次耗尽直接 finish，仍需确保不无限循环。
    let guard = 0
    while (this.phase === 'card_select' && guard++ < 16) {
      const script = this.replayScript
      if (script === null) return

      if (this.replayScriptPos >= script.length) {
        this.skipCards()
        continue
      }
      const want = script[this.replayScriptPos++]
      // -1 或越界 → 整波跳过
      if (want < 0 || want >= this.deck.hand.length) {
        this.skipCards()
        continue
      }
      // 手牌顺序可能与原局不同（不同 seed 的洗牌结果），
      // 因此按**索引**取，而不是按卡牌 id —— 索引才是脚本记录的语义。
      const target = this.deck.hand[want]
      if (!target) {
        this.skipCards()
        continue
      }
      this.takeCard(target.id)
      // takeCard 后手牌可能已空（已进下一波），也可能还有剩余
      // （玩家在原局取了 1 张又跳过其余）—— 两种都由 while 继续处理。
    }
  }

  private applyCard(card: Card): void {
    if (card.kind === 'attribute' && card.effect) {
      this.applyAttribute(card.effect)
    } else if (card.kind === 'mechanic' && card.mechanic) {
      this.applyMechanic(card.mechanic)
    } else if (card.kind === 'skill' && card.skillId) {
      // 技能卡：若已在槽内则升格，否则装入空槽
      const existing = this.skills.find((s) => s.skillId === card.skillId)
      if (existing) {
        // 升格：更高反应倍率 + 更高层数
        this.skills = this.skills.map((s) =>
          s.skillId === card.skillId
            ? {
                ...s,
                baseDamage: s.baseDamage + s.baseDamage / 5n,
                applyStacks: s.applyStacks + 1n,
                pierce: s.pierce + 1,
              }
            : s,
        )
      } else {
        const def = this.cfg.skills.get(card.skillId)
        const emptySlot = this.skills.findIndex((s) => s.slot < ACTIVE_SLOTS && s.slot >= 0)
        const slot = emptySlot >= 0 ? this.skills[emptySlot].slot : this.skills.length
        if (def && emptySlot >= 0) {
          this.skills = this.skills.map((s) =>
            s.slot === slot
              ? {
                  ...s,
                  skillId: def.id,
                  name: def.name,
                  element: def.element,
                  kind: def.kind,
                  heatCost: BigInt(def.heat_cost),
                  cooldownMs: def.cooldown_ms,
                  pierce: def.pierce,
                  aoeRadius: def.aoe_radius,
                  baseDamage: BigInt(def.base_damage),
                  applyElement: (def.apply_element || def.element) as Element | '',
                  applyStacks: BigInt(def.apply_stacks),
                  projectileSpeed: def.projectile_speed,
                  chain: def.chain,
                }
              : s,
          )
        }
      }
    }
  }

  private applyAttribute(e: AttributeEffect): void {
    switch (e.kind) {
      case 'attack':
        this.buffs.attackPermille += e.value
        break
      case 'element_coef':
        this.buffs.elementCoefPermille += e.value
        break
      case 'crit':
        this.buffs.critPermille += e.value
        break
      case 'heat_cap':
        this.buffs.heatCapBonus += e.value
        this.heat.capBonus += e.value
        break
      case 'element_cap':
        this.buffs.elementCapBonus += e.value
        break
      case 'armor':
        this.buffs.armorPermille += e.value
        break
    }
  }

  private applyMechanic(m: MechanicEffect): void {
    switch (m.kind) {
      case 'pierce_bonus':
        this.buffs.pierceBonus += m.value
        break
      case 'chain_bonus':
        this.buffs.chainBonus += m.value
        break
      case 'aoe_bonus':
        this.buffs.aoeBonus += m.value
        break
      case 'free_discard':
        this.buffs.freeDiscard += m.value
        this.deck.discardsLeft += m.value
        break
      case 'overheat_guard':
        this.buffs.overheatGuard += m.value
        break
    }
  }

  private finish(win: boolean): void {
    this.phase = win ? 'won' : 'lost'
    const stars = this.calcStars()
    if (win) {
      this.emit({ type: 'won', score: this.score, stars })
    } else {
      this.emit({ type: 'lost', score: this.score })
    }
  }

  /** 星级由客户端算一遍供即时反馈；服务端会重算校验（I-6 的分工） */
  private calcStars(): number {
    const t = this.cfg.level.star_targets
    let s = 0
    for (const v of t) if (this.score >= v) s++
    return s
  }

  /** 结算上报数据（结构与 domain.SettleInput 一致） */
  settleInput(tokenId: number) {
    return {
      token_id: tokenId,
      result: this.phase === 'won' ? 'win' : 'lose',
      stars: this.phase === 'won' ? this.calcStars() : 0,
      score: this.score,
      kills: this.kills,
      leaked: this.leaked,
      hp_left: Number(this.baseHp),
      wave_reached: this.waveIndex + 1,
      duration_ms: Math.round(this.elapsedMs),
      shots: this.shots,
      hits: this.hits,
      reactions: this.reactionsCount,
      heat_max: Number(this.heat.maxHeatThisBattle),
      elements_used: this.elementsUsed,
      reactions_used: this.reactionsUsed,
      terrain_used: [...new Set(this.terrainUsed)],
      /** I-6 重放闭环：选牌决策序列，第三方据此复现原局 */
      card_picks: [...this.cardPicks],
      replay_hash: this.replayHash(),
    }
  }
}

// ---------- 工具 ----------

function dist2(x1: bigint, y1: bigint, x2: bigint, y2: bigint): bigint {
  const dx = x1 - x2
  const dy = y1 - y2
  return dx * dx + dy * dy
}

/** 整数平方根近似（牛顿法，避免 Math.sqrt 的浮点不确定） */
function sqrtApprox(n: bigint): bigint {
  if (n < 2n) return n
  let x = n
  let y = (x + 1n) / 2n
  while (y < x) {
    x = y
    y = (x + n / x) / 2n
  }
  return x
}

function dominantOf(e: Enemy): Element | null {
  let best: Element | null = null
  let bestStacks = 0n
  for (const el of ELEMENT_ORDER) {
    const v = e.stacks.get(el) ?? 0n
    if (v > bestStacks) {
      best = el
      bestStacks = v
    }
  }
  return best
}

function reactionIndex(k: ReactionKey): number {
  return REACTION_ORDER.indexOf(k) + 1
}

function terrainIndex(kind: string): number {
  const order = ['oil_drum', 'tidal_gate', 'rotor_vane', 'collapse_wall', 'charge_tower']
  return order.indexOf(kind) + 1
}

function cardIndex(c: Card): number {
  if (c.kind === 'skill') return (c.skillId ?? 0) * 10
  if (c.kind === 'attribute') return 500 + (c.effect ? attributeIndex(c.effect.kind) : 0)
  return 900 + (c.mechanic ? mechanicIndex(c.mechanic.kind) : 0)
}

function attributeIndex(kind: string): number {
  return ['attack', 'element_coef', 'crit', 'heat_cap', 'element_cap', 'armor'].indexOf(kind) + 1
}

function mechanicIndex(kind: string): number {
  return ['pierce_bonus', 'chain_bonus', 'aoe_bonus', 'free_discard', 'overheat_guard'].indexOf(
    kind,
  ) + 1
}

function uidOf(v: bigint): number {
  return Number(v % 100000n)
}
