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
import { DEFAULT_SCORE_RULES, damageScore, killScore, type ScoreRules } from './score'
import {
  PERMILLE,
  ELEMENT_PER_STACK_BASE,
  mulDiv,
  maxBig,
  minBig,
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

/**
 * 回放前缀的 **S 段**（技能槽配置）—— 抽成纯函数。
 *
 * 格式：每槽一条 `slot:skillId:baseDamage:applyStacks:heatCost`，按**字符串**升序，逗号连接。
 *
 * ## 为什么要抽出来（第 56 轮）
 *
 * 服务端要独立重算这个串来校验上报（见 `server/internal/domain/replayskills.go`）。
 * 两端格式一旦漂移，合法玩家的每一局都会被判成作弊 —— 而且是那种
 * 「客户端、服务端、e2e 全绿，只有真机玩家被拒」的漂移，极难定位。
 *
 * 所以这个串进了 `server/testdata/formula_vectors.json`：
 * Go 与 TS 两侧的测试读**同一个**字面期望值。
 *
 * 顺带一提：`.sort()` 无参数时按 UTF-16 码元比较，所以 `10:...` 排在 `2:...` 前面。
 * Go 侧必须用 `sort.Strings` 才等价 —— 写成按槽位数值排会锁死 10 槽以上的玩家。
 */
export function replaySkillsSegmentOf(
  skills: readonly Pick<EquippedSkill, 'slot' | 'skillId' | 'baseDamage' | 'applyStacks' | 'heatCost'>[]
): string {
  return skills
    .map((s) => `${s.slot}:${s.skillId}:${s.baseDamage}:${s.applyStacks}:${s.heatCost}`)
    .sort()
    .join(',')
}

export const TICK_HZ = 20

/**
 * `card_picks` 里「本波弃过一张」的编码（第 66 轮）。
 *
 * 一波内的记录值形如 `-(PICK_DISCARD_BASE + handIdx)`：
 * 弃过牌并取第 0 张 → -3，第 1 张 → -4，第 2 张 → -5。
 *
 * -1 保留给「整波跳过」（既有语义，不改）。
 *
 * ⚠️⚠️ 编码**必须恒为负**，否则与普通手牌下标撞车。
 *
 * 第一版写成 `PICK_DISCARD_BASE - handIdx`（base=2），于是
 * 「弃过牌并取第 0 张」= 2 —— 而 2 也是合法的普通下标（取第 2 张）。
 * 两者的 record / 事件流完全不同，却编码成同一个值。
 *
 * 实测症状很隐蔽：`cardPicks` 记成 `[2,2,2,2]`，看起来完全正常
 * （全是合法下标），但重放按「取第 2 张」解释，于是每波少一次弃牌
 * → 事件流少 4 条 → 哈希失配。
 *
 * **教训**：编码负数下标时，「编码值」与「合法取值域」必须**互不相交**。
 * 这个 bug 与 README 记的「第 57 轮 slot=10 排在 slot=2 前面」同源：
 * 都是编码与取值域撞车，而单测（全绿）看不出来，因为撞车只在**跨侧**发生。
 *
 * 下界：handIdx ∈ [0, 2]（每波固定 3 张牌）→ 最负 -5。
 * **服务端 `card_picks` 的下界校验必须同步放宽到 -5。**
 */
export const PICK_DISCARD_BASE = 3

/**
 * `card_picks` 里「本波弃过牌但整波跳过」的编码（第 66 轮）。
 *
 * 形如 `-(PICK_DISCARD_SKIP_BASE + handIdx)`：
 * 弃一张后跳过第 0 张 → -4，第 1 张 → -5，第 2 张 → -6。
 *
 * 为什么需要它：`-1`（整波跳过）无法表达「弃过牌再跳过」。
 * 实测那种打法下原局与重放的事件流差一条 `record(..., 1)`，
 * 热量也差 1 → replayHash 不同 → I-6 判伪造。
 * 而「弃一张后跳过」是完全合法的操作（discardCard 与 skipCards 都只判
 * `phase === 'card_select'`）。
 *
 * ⚠️ 它必须与 `-(PICK_DISCARD_BASE + handIdx)` 取值域**不相交**：
 *   弃+取：-(3+0..2) = -3..-5
 *   弃+跳：-(6+0..2) = -6..-8
 * 不相交，靠的是两个 BASE 相差 3（大于单波最大手牌数 2）。
 * 若哪天 `rollWaveCards` 改成每波 4 张，这两个区间就会开始重叠 ——
 * 那时必须把 BASE 拉开，而不是改 `handIdx` 的上限。
 */
export const PICK_DISCARD_SKIP_BASE = 6

/**
 * `card_picks` 条目的合法取值下界（第 66 轮）。
 *
 * 导出给边界与契约测试共用，避免三处各写一份魔数。
 */
export const PICK_MIN = -(PICK_DISCARD_SKIP_BASE + 2) // = -8
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

/**
 * 输出位漏怪代价的分母（第 54 轮定稿）：`底血 × attack / LeakAttackerDivisor`。
 *
 * ## 为什么要改这条公式
 *
 * 内容表里 `attack` 是**平铺值**（8/10/28/35/45/60），而底血从 1000 涨到 14000。
 * 于是「有攻击力的敌人」只掉 6~60 点，相对底血几乎为零；
 * 而零攻击力的杂兵走 `breachDamage`，代价恒为底血的 1/10。
 *
 * **一只 boss 漏进防线的代价只有杂兵的 1/4 ~ 1/40**，没有哪条设计原则支持这个不对称。
 * 后果是护甲（只减漏怪伤害）吃满封顶 750‰ 也只多买 **3.66%** 血量 ——
 * 护甲、装备护甲、专精 armor 节点、宝石 armor 词条**全部买不到东西**。
 *
 * 改成随底血等比缩放后，**每关的相对代价一致**，护甲第一次在每一关都有分量。
 *
 * ## 分母怎么定的（1200 / 2400 / 3600 / 6000 / 12000 实测扫描，100 关 × 2 护甲档）
 *
 * | 分母 | 通关 | 护甲收益 | 掉血占比 | 前 30 关失败 |
 * |---|---|---|---|---|
 * | 基线(无缩放) | 100/100 | 3.66% | 5.33% | 无 |
 * | 1200 | 96/100 | 1.86% | 13.81% | 无 |
 * | 2400 | 96/100 | 1.60% | 13.50% | 无 |
 * | **3600** | **99/100** | **6.81%** | 12.23% | **无** |
 * | 6000 | 100/100 | 5.70% | 8.43% | 无 |
 * | 12000 | 100/100 | 3.48% | 5.11% | 无 |
 *
 * 取 **3600**：护甲收益比基线高 **86%**，前 30 关**无进度墙**，
 * 只有第 96 关失败（第 54 轮实测）。
 *
 * 更小的分母会让早期关卡被压垮：
 * | | 第 1 关（底血 1000） | 第 100 关（底血 14000） |
 * |---|---|---|
 * | 基线 boss 漏一次 | 60（6.0% 底血） | 60（0.4% 底血） |
 * | 平铺 ×5 | 300（**30%**） | 300（2.1%） |
 *
 * **平铺放大对早期关卡的伤害最大，与难度曲线完全相反** —— 入口关卡被压垮、
 * 终盘反而相对变轻。这是第 53 轮那次失败的核心教训。
 *
 * ## 与内容表的关系
 *
 * 隐含「最强敌人攻击力 = 60」（chalkqueen）。改内容表 `attack` 上限时
 * **必须同步这个分母**，否则最强输出位的相对代价会漂移，且不会有任何测试报错。
 */
export const LeakAttackerDivisor = 3600n

/**
 * 出场进度：**整数千分比** 0..SPAWN_PROGRESS_FULL。
 *
 * ⚠️ 敌人曾经用 0..1 的浮点记录出场进度、每 tick 累加 0.08，
 * 这直接违反项目的定点整数铁律（README 六个工程约束第 2 条）。
 *
 * 为什么它不只是"风格问题"：这个值决定敌人**何时开始移动、何时可被命中**，
 * 所以它会改变后续每一个 tick 的事件序列，进而改变 replayHash。
 * 而 IEEE 754 浮点加法的中间精度**没有跨实现保证** ——
 * 不同 V8 版本、不同 CPU 架构（x86 的 SSE 路径与 ARM 的实现不同）可能差 1 ulp。
 * 差 1 ulp 就可能让某个边界判定在两端落在不同一侧，
 * 于是 I-6 把这类关卡的正常对局判成伪造。
 *
 * 取 FULL=1000 / STEP=80 是为了**逐位等价**于原来的行为：
 * 80/1000 = 0.08，13 个 tick 达到满（12 个 tick 时 960 < 1000）。
 * 换成别的数值会让"敌人何时可被击中"的时刻整体平移，
 * 存量战报的哈希会失配。
 */
export const SPAWN_PROGRESS_FULL = 1000

/** 每 tick 的出场进度增量（80‰，等价于原来的 0.08）。 */
export const SPAWN_PROGRESS_STEP = 80

/**
 * MAX_BATTLE_TICKS 是战斗的绝对 tick 上限（30000 tick = 1500s = 25 分钟）。
 *
 * 到达即强制判负并结束，防止引擎因任何死锁而**永久挂死**。
 *
 * ⚠️ 实测踩过：第 22/33/34 关因「掩体挡弹丸 → 弹丸打不到敌人 →
 * 掩体永远打不破」的循环依赖，引擎永久停在 wave 阶段 ——
 * 12000 tick 只杀 2~17 只怪，命中率 1%，战斗不会自然结束。
 * 真机上那就是玩家盯着一个永远不结算的画面，体力也拿不回来。
 *
 * 那个具体缺陷已修，但**这类缺陷（循环依赖 / 几何互斥）无法靠读代码杜绝** ——
 * 本项目已经栽过三次（掩体充能路径几何互斥、刷怪毫秒当 tick、
 * 热量死区导致战斗停摆）。所以留一道兜底。
 *
 * 预算依据：实测最慢的合法关卡是第 100 关 594s = 11880 tick，
 * 30000 是它的 2.5 倍余量 —— 合法对局**不可能**触碰这个上限，
 * 因此它只会在真正死锁时触发。
 *
 * 与服务端的一致性：服务端对 duration_ms 的上界是 1800s（54000 tick），
 * 比这里宽 1.8 倍。宁可服务端更宽松 —— 否则会出现
 * "客户端因停滞结束了、服务端却认为时长非法"的不一致。
 */
export const MAX_BATTLE_TICKS = 30000

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
  /**
   * 可用的主动技能槽位数。
   *
   * ⚠️ 缺省必须等于 `ACTIVE_SLOTS`（4），**不得改变**。
   *
   * 缺省值决定重放哈希：绝大多数战报是 4 槽的，
   * 缺省一旦不是 4，历史战报全部重放不出原哈希。
   *
   * 只有玩家在专精树点了「额外插槽」（`MasteryEffect.ExtraSlots`）时
   * 才会 > 4，而那由服务端 `/mastery` 的 `total_slots` 下发。
   */
  activeSlots?: number
  /**
   * 分数规则。缺省用 DEFAULT_SCORE_RULES。
   *
   * ⚠️ 真实对局应当**从服务端 /config 的 score_rules 下发**，
   * 而不是用这里的默认值 —— 因为 star_targets 是服务端算的，
   * 客户端用自己的一份就会与门槛算法漂移（见 score.ts 的注释）。
   */
  scoreRules?: ScoreRules
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
  /** 「隔热护罩」免疫了一次过热（与真正的 overheat 区分开） */
  | { type: 'card_heated' }
  /** 战斗到达绝对 tick 上限被强制结束（引擎停滞兜底） */
  | { type: 'stalemate' }
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

  /**
   * **开战前**构筑的 S 段，在构造那一刻冻结（第 64 轮）。
   *
   * ⚠️ 必须是字符串快照，不能靠「持有 skills 数组的旧引用」：
   * `applyCard` 走的是 `this.skills = this.skills.map(...)` —— 整体替换数组，
   * 元素是 `{...s, baseDamage: 新值}` 这样的**新对象**。
   * 所以旧引用指向的旧数组不会跟着变，但新数组的内容已经是升格后的 ——
   * 只要上报时读 `this.skills` 就一定会带上局内状态。
   */
  private readonly buildSnapshotSkills: string

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

  /** 当前可用的主动槽位数。见 BattleConfig.activeSlots 的缺省约定。 */
  readonly activeSlots: number

  /** 分数规则。缺省 DEFAULT_SCORE_RULES，实际对局应从服务端下发。 */
  readonly scoreRules: ScoreRules

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
    // 上报用的 S 段在这里冻结 —— 早于任何 applyCard（第 64 轮）。
    this.buildSnapshotSkills = replaySkillsSegmentOf(this.skills)

    this.baseHp = BigInt(cfg.level.base_hp)
    this.baseHpMax = BigInt(cfg.level.base_hp)
    this.baseArmorPermille = BigInt(cfg.level.armor_permille || 0)

    let total = 0
    for (const w of cfg.level.waves ?? []) {
      for (const sp of w.spawns) total += sp.count
    }
    this.totalEnemies = total
    // ⚠️ 缺省必须是 ACTIVE_SLOTS —— 改这个默认值会让所有 4 槽战报的
    // 重放哈希失配（绝大多数战报都是 4 槽的）。
    //
    // 只有玩家在专精树点了「额外插槽」（MasteryEffect.ExtraSlots）时才会 > 4，
    // 那个值由服务端 `/mastery` 的 `total_slots` 下发。
    this.activeSlots = cfg.activeSlots ?? ACTIVE_SLOTS
    this.scoreRules = cfg.scoreRules ?? DEFAULT_SCORE_RULES

    // 热量上限加成（专精 heat_cap + 宝石 gem_heat）。
    //
    // ⚠️ 此前**从未从攻方读入**，于是「热量上限」专精节点（16 个）
    // 与散热石宝石完全无效 —— 它们只进 I-7 评分。
    //
    // 放在构造器而不是每次读 cap 时加：capBonus 是 HeatMeter 的可变状态，
    // 而 heat_cap 攻方字段是本局常量，构造时注入一次最省。
    this.heat.capBonus = cfg.attacker.heatCapPermille

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
  /**
   * 回放前缀里的 **`S` 段**：本局装载的技能槽配置。
   *
   * 格式：每槽一条 `slot:skillId:baseDamage:applyStacks:heatCost`，按字符串升序，逗号连接。
   *
   * ## 为什么把它单独提出来（第 56 轮）
   *
   * `replayHash` 整体是 **无法**被服务端廉价校验的：
   * 它是 `fnv1a64(前缀;事件1;事件2;…)` 的单向哈希，
   * 而前缀里的 `A` 段取自 `currentAttacker()` —— 那个值含 `this.buffs.*`，
   * 是**战斗中选卡产生的加成**，服务端在结算时不知道。
   *
   * 但 `S` 段**不含 buff**，它完全由构筑决定：槽位、技能 id、
   * 底伤（技能等级已烘进去）、叠层、热量。
   * 服务端同样有这些数据，于是可以独立重算并比对。
   *
   * 它能抓住：
   * - 伪造 `base_damage`（即假报技能等级 —— 等级必须烘进底伤）
   * - 上报一套与 `user_skill_slots` 不同的技能
   * - 槽位错位
   *
   * 抓不到的：伪造 `A` 段（攻方系数）—— 那需要模拟，已记入 README 已知边界。
   *
   * ⚠️⚠️ 这里**只**服务于 `replayHash()`，不用于上报（第 64 轮修正）。
   *
   * 原注释写的是「这个字符串同时用于哈希与上报，必须是同一个来源」——
   * 那条假设是**错的**，且它把一个把绝大多数玩家判成作弊的缺陷正当化了。
   *
   * 两个用途需要的是**相反**的口径：
   *
   *   | 用途   | 该含什么                     | 为什么 |
   *   |--------|------------------------------|--------|
   *   | 哈希   | **含**局内状态（取牌升格后） | 否则取牌不改变哈希，「同种子不同操作得同哈希」成立，验真说谎 |
   *   | 上报   | **不含**局内状态（开战前构筑） | 服务端按 DB 重算，DB 里没有局内升格这回事 |
   *
   * 之前两处都调本方法（读 `this.skills`，会被 `applyCard` 就地升格），
   * 于是取过一张技能卡的正常对局上报 `0:1:120:2:20`，
   * 而服务端重算出 `0:1:100:1:20` → `ErrReplaySkillsMismatch` → 422。
   * `rollWaveCards` 固定把技能卡放在手牌下标 0，5 波各取 1 张时
   * 随机选命中技能卡的概率 ≈ 1-(2/3)^5 ≈ **86%**。
   */
  replaySkillsSegment(): string {
    return replaySkillsSegmentOf(this.skills)
  }

  /**
   * **上报给服务端的** S 段：开战前的构筑，不含任何局内升格（第 64 轮）。
   *
   * 与 `replaySkillsSegment()` 读同一个数组，但必须**在构造时冻结** ——
   * `applyCard` 会 `this.skills = this.skills.map(...)` 整体替换数组元素，
   * 所以持有旧数组的引用并不能免疫升格，必须在构造那一刻算出字符串。
   *
   * 守卫：`replay_skills_build_snapshot.test.ts`。
   */
  buildSnapshotSegment(): string {
    return this.buildSnapshotSkills
  }

  replayHash(): string {
    const a = this.currentAttacker()
    const skills = this.replaySkillsSegment()
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
    // ⚠️ 第 66 轮：每波重置「本波弃过牌」标记 ——
    // 编码只对**当前波**的 card_picks 项有意义。
    this.discardedThisWave = false

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
      armorShredMs: 0,
      armorShredPermille: 0n,
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
    // `e.dead` 守卫的 true 分支**不可达**：唯一调用点 updateEnemies 在循环里
    // 已先 `if (e.dead) continue`（见上方 updateEnemies），传进来的 e 必非 dead。
    // 保留是防御未来新增调用点，当前无法触达。
    /* v8 ignore next -- 见上：调用点已过滤 dead，此守卫真分支不可达 */
    if (e.dead) return
    if (e.spawnProgress < SPAWN_PROGRESS_FULL) {
      e.spawnProgress = Math.min(SPAWN_PROGRESS_FULL, e.spawnProgress + SPAWN_PROGRESS_STEP)
      return
    }
    if (e.hitFlashMs > 0) e.hitFlashMs -= TICK_MS
    if (e.frozenMs > 0) e.frozenMs -= TICK_MS
    if (e.stunnedMs > 0) e.stunnedMs -= TICK_MS
    if (e.slowedMs > 0) e.slowedMs -= TICK_MS
    if (e.armorShredMs > 0) e.armorShredMs -= TICK_MS

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

    // ⚠️ 待结束的本波在这里收尾（第 66 轮）。
    //
    // 位置关键：必须在 `applyReplayDecision` **之前**。
    // 否则同一 tick 内 `applyReplayDecision` 会被再调一次，
    // 消费 script 的下一项，把下一波的决策当成本波的第二张牌
    // （实测 picks 记成 [-3,-3,-1,-1]，事件流多一条原局没有的 402/902/0）。
    if (this.cardSelectDone) {
      this.cardSelectDone = false
      this.beginWave(this.waveIndex + 1)
    }

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

    // 停滞兜底：放在 tick++ 之后、任何战斗逻辑之前。
    // 位置很关键 —— 必须在 phase 检查之后（本函数开头已 return 掉终局），
    // 且必须在移动/开火之前，这样"到上限"这件事本身是这次 tick 的第一个事实。
    if (this.checkStalemate()) return this.drainEvents()

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
            const dmg = this.leakDamageFor(e)
            this.baseHp -= dmg
            this.leaked++
            this.emit({ type: 'leak', damage: dmg })
            // ⚠️ 第 70 轮：`a` 从 `uidOf(dmg)` 改成 `e.uid`。
            //
            // 两条漏怪路径此前**语义不一致**：
            //   射程内扣血  →  `record(tick, 'leak', uidOf(dmg))`
            //   抵达防线    →  `record(tick, 'leak', e.uid)`
            //
            // `uidOf(v) = Number(v % 100000n)` —— 传进去的是**伤害值**，
            // 于是 `a` 变成了「漏怪伤害 mod 100000」，与任何 uid 无关。
            //
            // 判定它是接线错误而非约定的证据：`uidOf` 全仓**只有一个调用点**，
            // 函数名说「取 uid」而实参是伤害值 —— 名字与实参对不上，
            // 说明写的时候想的是别的东西。
            //
            // 后果：
            //  1. **语义错位** —— 这条事件不记录是哪只怪漏的，
            //     事后无法从回放定位责任目标
            //  2. **信息损失** —— 伤害相差 100000 的两次漏怪在哈希里
            //     不可区分（`leakDamageFor` 的量级是
            //     `baseHpMax × attack / 3600`，当前几百到几千，
            //     尚未跨过 10^5，但随 base_hp 增长会跨过）
            //
            // 现在 `a` 统一是敌人 uid，伤害值放进 `b`
            // —— `b` 此前在 leak 事件上恒为 0，没被别的类型占用。
            this.record(this.tick, 'leak', e.uid, Number(dmg))
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
        // ⚠️ 第 70 轮：`b` 补上伤害值，与射程内那条对齐。
        //
        // 两条路径此前一处写 uid 一处写伤害，且都没有记录另一项 ——
        // 也就是说无论走哪条路，都**丢掉了**一个信息。
        // 现在约定：`a` = 敌人 uid，`b` = 漏掉的伤害值。
        this.record(this.tick, 'leak', e.uid, Number(dmg))
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
      this.enterOverheat()
      return
    }

    for (const s of this.skills) {
      if (s.slot >= this.activeSlots) continue // 被动槽不主动释放
      if (s.cooldownRemaining > 0) continue
      if (this.heat.heat + s.heatCost > this.heat.cap) continue

      const target = this.pickTarget(s)
      if (!target) continue

      // tryCast 失败的分支**不可达**：上面第 725 行已 `if (heat + heatCost > cap) continue`
      // 预过滤，其判据与 tryCast 内部拒绝条件（heat+cost>cap）逐字相同，且两行之间
      // pickTarget 不改热量。故走到这里 tryCast 必然成功（并原子地累加热量）。
      // 保留 tryCast 的返回值检查是"预检 + 原子提交"的双保险，但当前 continue 无法触达。
      /* v8 ignore next -- 见上：725 行已预过滤同一判据，tryCast 必成功，此 continue 不可达 */
      if (!this.heat.tryCast(s.heatCost)) continue
      s.cooldownRemaining = s.cooldownMs
      this.fire(s, target)

      // ⚠️ 必须在开火后检查过热 —— 热量打满的那一刻正是"用尽最后一点资源"的时刻。
      // 若在开火前检查，玩家会看到"我还没出手就过热了"。
      //
      // 这个调用一旦漏掉，热量会永久卡在上限、再也放不出技能（衰减虽在，
      // 但不会触发过热状态），战斗直接停摆 —— 属于静默失效。
      if (this.heat.checkOverheat()) {
        this.enterOverheat(this.skillIndexOf(s))
        break // 一次性触发，避免同帧多个技能重复发事件
      }
    }
  }

  /**
   * 进入过热状态。
   *
   * 这里是「隔热护罩」（overheat_guard 机制卡）的**唯一**消费点。
   *
   * ⚠️ 该卡此前只写不读：`buffs.overheatGuard += m.value` 之后再无任何读取点，
   * 所以玩家拿到一张写着「过热时免疫一次过热」的卡，
   * 观察不到任何变化 —— 但它会通过 mechanicIndex 参与 replayHash，
   * 也就是说**哈希记录了一个不产生任何效果的选择**。
   *
   * 5 张机制卡里 pierce_bonus / chain_bonus / aoe_bonus / free_discard
   * 都有真实消费点，只有这一张是空转。
   *
   * 触发时：消耗一层护盾、**不进入过热**、且不弹 overheat 事件
   * （没真的过热，弹事件会让 UI 显示"过热"而实际没过热）。
   */
  private enterOverheat(skillIndex = -1): void {
    if (this.buffs.overheatGuard > 0) {
      this.buffs.overheatGuard--
      // 护盾生效：清空热量当作"扛过去了"，但不进入过热状态。
      // 清空是必须的 —— 否则热量仍停在死区里，下一 tick 又会触发兜底，
      // 形成"每 tick 消耗一层护盾"的空转。
      this.heat.absorbOverheat()
      this.emit({ type: 'card_heated' })
      this.record(this.tick, 'guard', 0)
      return
    }
    this.heat.forceOverheat()
    this.emit({ type: 'overheat' })
    this.record(this.tick, 'overheat', skillIndex)
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
    return this.leakDamageFor(e)
  }

  /**
   * 一次漏怪对防线造成的伤害（已计护甲）。
   *
   * 「射程内扣血」与「抵达扣血」**必须用同一个函数** ——
   * 两处各写一份公式时，护甲的效果会被两条路径分摊，
   * 而 leaked 又把两类混在一起计数，于是任何按关卡平均的测量都不可信
   * （第 50 轮就栽在这里：把 19 次漏怪按关卡平均分类，得出过错的归因）。
   *
   * - 有攻击力：底血 × attack / LeakAttackerDivisor
   * - 无攻击力：底血 / LeakDamageDivisor（杂兵兜底，原本就有）
   */
  private leakDamageFor(e: Enemy): bigint {
    if (e.attack <= 0n) return applyArmor(this.breachDamage, this.defenseArmorPermille())
    // 先乘后除，避免整数截断把小数吃掉
    const scaled = (this.baseHpMax * e.attack) / LeakAttackerDivisor
    return applyArmor(scaled, this.defenseArmorPermille())
  }

  /**
   * 防线的有效护甲 = 关卡基础护甲 + 卡牌加成 + **攻方护甲加成**。
   *
   * ⚠️ 卡牌加成（buffs.armorPermille）只在这里生效。
   * 它曾经被加到 `spawnEnemy` 里敌人的护甲上，效果完全反向 ——
   * 写着「防线护甲 +10%」的卡让玩家变弱。
   *
   * 攻方护甲（attacker.armorPermille）是本轮补上的第三项：
   * 装备的 `BaseArmor`、专精树的 armor 节点、宝石的 gem_armor 词条
   * 全部汇总到这一个字段。此前它们**只进 I-7 构筑评分**，
   * 战斗里完全无效 —— 玩家穿上「钢鳞胸甲」后漏怪伤害一点没少。
   *
   * applyArmor 内部已有 MAX_ARMOR=750‰ 封顶，
   * 所以「关卡 250‰ + 卡 600‰ + 装备 250‰」会收敛到 750‰，那是设计内的 clamp。
   */
  private defenseArmorPermille(): bigint {
    return this.baseArmorPermille + this.buffs.armorPermille + this.cfg.attacker.armorPermille
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
      if (s.slot >= this.activeSlots) continue // 被动槽不消耗热量，不参与判据
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
    const live = this.enemies.filter((e) => !e.dead && e.spawnProgress >= SPAWN_PROGRESS_FULL)
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
      // `p.dead` 守卫的 true 分支**不可达**：每次 updateProjectiles 末尾都
      // `this.projectiles = this.projectiles.filter((p) => !p.dead)`（见函数尾），
      // 故下一 tick 进入本循环时数组里不含 dead 弹丸；本 tick 内每颗只遍历一次。
      /* v8 ignore next -- 见上：末尾已滤除 dead，此守卫真分支不可达 */
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
        // 被崩塌前的掩体阻挡。
        //
        // ⚠️ 这里**必须把弹丸的伤害喂给掩体**，否则掩体永远打不破。
        //
        // 曾经的写法只是 `p.dead = true; break`，于是：
        //
        //   掩体挡住弹丸 → 弹丸销毁 → 走不到 checkProjectileHit
        //   → 掩体唯一的充能源（onHit，只在弹丸命中**敌人**时调用）永不触发
        //   → 掩体永不崩塌 → 弹丸继续被挡          ← 循环依赖
        //
        // 实测后果：第 22/33/34 关**永远打不完**。弹丸轨迹实测显示，
        // 从 (60,880) 射向 (884,638) 的弹丸在 (497,752) 消失，
        // 而那里距掩体 (514,654) 恰好 99.5 < 100 —— 被挡。
        // 命中率掉到 1%，12000 tick 只杀 3 只怪，引擎永久停在 wave 阶段。
        //
        // 而 `onHit` 的元素门槛又让情况更糟：掩体只吃动能，
        // 默认构筑是 fire/fire/fire/ice，**一个动能都没有**。
        //
        // 现在弹丸打在掩体上等同于打在敌人身上（有伤害、有元素），
        // 于是"用动能砸开掩体、打开弹道"这个原本的设计意图才真正成立。
        for (const t of this.terrains) {
          if (t.blocksProjectile() && this.withinTerrain(t.x, t.y, 100, p.x, p.y)) {
            if (t.onHit(p.element, p.damage, p.x, p.y)) {
              this.terrainUsed.push(t.kind)
              this.emit({ type: 'terrain', kind: t.kind, x: t.x, y: t.y })
              this.record(this.tick, 'terrain', terrainIndex(t.kind), t.x, t.y)
            }
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
      if (e.dead || e.spawnProgress < SPAWN_PROGRESS_FULL) continue
      if (p.hitSet.has(e.uid)) continue
      if (!this.hitEnemy(p, e)) continue

      this.hits++
      p.hitSet.add(e.uid)

      // 溅射
      //
      // `this.hits` 只在**主目标**这里自增，applyAoe / 弹射内部**不**加。
      //
      // hits 的语义是「有多少次开火命中了东西」，不是「总共造成了几次伤害结算」。
      // 理由是一条跨端强约束：服务端 ValidateSettle 拒 in.Hits > in.Shots
      // （ErrInvalidHitRate），engine.test.ts 也有 hits <= shots。
      // 一发带 aoe/chain 的弹丸本就能命中多个敌人，每次结算都 hits++ 会让
      // 高 chain + 高 pierce 的构筑稳定产出 hits > shots —— 那是真的 422。
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
          // 弹射目标必须**当场结算伤害**（第 65 轮修复）。
          //
          // 原来是 `p.hitSet.add(next.uid); continue`：
          // continue 只是数组游标前进，不会回头结算 next；
          // 而 next 已被写进 hitSet，下一轮开头的 hitSet.has 会跳过它。
          // 于是弹射只消耗 chainLeft、只位移弹丸，目标一点伤害都吃不到 ——
          // 连锁闪电 / 电弧弹这一整类多目标技能的定位完全失效。
          //
          // 结算顺序：先结算、后写 hitSet（反过来会重复结算）。
          if (this.hitEnemy(p, next, true)) {
            p.hitSet.add(next.uid)
          }
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

  /**
   * 对单个敌人结算一次命中。
   *
   * skipRadius 用于**已经确定要命中**的路径：溅射（applyAoe 已按 aoe_radius
   * 选好目标）与弹射（弹丸直接跳到目标身上，距离恒为 0）。
   *
   * 为什么必须显式跳过（第 65 轮）：原实现无条件执行 withinEnemy，
   * 而 withinEnemy 的半径是 toFixed(28 + flyHeight*0.1) —— 那是**单体命中半径**，
   * 不是溅射半径。于是 applyAoe 按 aoeRadius 正确选出的目标，紧接着被这 28
   * 单位再裁一次：声明的 aoe_radius 被当成 28，所有溅射技能实质失效
   * （只有彼此贴在一起、距命中点不到 28 单位的敌人才会被顺带打到）。
   */
  private hitEnemy(p: Projectile, e: Enemy, skipRadius = false): boolean {
    if (!skipRadius && !this.withinEnemy(e, p.x, p.y)) return false

    // 把敌人的运行时状态包成 Defender，复用与 Go 完全一致的结算。
    // 削甲生效期（armor_break）内折进有效护甲：削到 0 为止，不为负。
    const effArmor =
      e.armorShredMs > 0
        ? (e.armorPermille > e.armorShredPermille
            ? e.armorPermille - e.armorShredPermille
            : 0n)
        : e.armorPermille
    const def = new Defender(e.hp, e.shield, effArmor)
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
    //
    // ⚠️ 第 88 轮：`amplifyPct` 与 `statusDurationMs` 是**两件独立的事**，
    // 原来被一个 `if res.statusDurationMs > 0` 捆在一起。
    //
    // # 缺陷：潜伏耦合 —— 加一条「只有增伤、没有状态」的反应会静默失效
    //
    // 原写法：
    //
    //    if (res.statusDurationMs > 0) {
    //      const spec = REACTIONS[...]
    //      if (spec.amplifyPct > 0) e.amplifyPermille = ...
    //      ...frozenMs / stunnedMs...
    //    }
    //
    // `amplifyPct` 的生效被 `statusDurationMs > 0` 把门，于是：
    //
    //	新增一条 amplify_pct = 300、status_duration_ms = 0 的反应
    //	→ 受击增伤**永远不会生效**，而代码看起来完全正常
    //
    // 为什么这不可能被测出来：**当前 7 条反应里没有任何一条落在这个组合**
    //（superconduct 600/4000、flash_freeze 300/2000，其余全 0）。
    // 所以判据「amplify 生效」在今天与「不生效」观察不到差别 ——
    // 它是一个**只有在改动之后才会显形**的洞。
    //
    // 这与第 80/85 轮记的同一个陷阱：
    // **只要输入落不到分界线上，关于分界线的断言都是空的。**
    //
    // # 修法
    //
    // 两个字段各管各的，各有自己的判据。
    // ⚠️ 查表本身**必须**留在 `res.reaction` 非空的守卫里。
    //
    // 我第一版把查表提到条件之外，于是每次命中都执行
    // `REACTIONS[res.reaction as ReactionKey]` —— 而无反应时
    // `res.reaction` 是**空串**，`REACTIONS['']` 是 undefined，
    // 紧接着读 `.amplifyPct` 就抛：
    //
    //	TypeError: Cannot read properties of undefined (reading 'amplifyPct')
    //
    // 既有测试（`replay_discard.test.ts` 等 4 个文件、35 个用例）**当场抓到**。
    // 那条 `if (res.statusDurationMs > 0)` 除了把门 amplify，
    // **顺带**把「无反应时不要查表」也挡住了 —— 一个副作用式的守卫。
    //
    // 拆耦合时必须把它显式补回来，否则就是拿一个偶发崩溃换另一个潜伏洞。
    const reactKey = res.reaction as ReactionKey
    const reactSpec = reactKey ? REACTIONS[reactKey] : undefined
    if (reactSpec && reactSpec.amplifyPct > 0) {
      e.amplifyPermille = BigInt(reactSpec.amplifyPct)
    }
    if (res.statusDurationMs > 0) {
      const react = res.reaction
      const spec = reactSpec ?? REACTIONS[react as ReactionKey]
      if (react === 'flash_freeze' || react === 'superconduct') e.frozenMs = spec.statusDurationMs
      if (react === 'overheat') e.stunnedMs = spec.statusDurationMs
      if (react === 'armor_break' && spec.armorShredPermille > 0) {
        // 削甲：statusDurationMs 内目标护甲被削 spec.armorShredPermille 千分比
        // （hitEnemy 构建 Defender 时折进有效护甲）；
        // 击退：一次性右推（stepEnemyMotion 的 knockback 分支消费后清零）。
        e.armorShredMs = spec.statusDurationMs
        e.armorShredPermille = BigInt(spec.armorShredPermille)
        e.knockback += BigInt(spec.knockback)
      }
    }

    this.score += Number(damageScore(this.scoreRules, res.totalDamage))
    this.pushFloat(
      e.x,
      e.y,
      String(Number(damageScore(this.scoreRules, res.totalDamage))),
      res.crit ? '#ffd33d' : '#e6edf3',
      res.crit ? 20 : 16,
    )
    this.emit({ type: 'hit', x: p.x, y: p.y, damage: res.totalDamage, crit: res.crit })
    this.record(this.tick, 'hit', e.uid, Number(damageScore(this.scoreRules, res.totalDamage)), res.crit ? 1 : 0)

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

    // ⚠️ steam_burst 的溅射范围伤害。
    //
    // # 设计与 I-6
    //
    // `replay_hash` 仅覆盖 S 段（构筑快照），服务端**不**重算事件流
    // （battle_collections.go：replay_hash 只封长 ≤64，store as-opaque）。
    // 所以溅射事件只需「本局自身哈希稳定」，而非「服务端可独立验证」 ——
    // 它不影响 replay_skills 段，因而不影响当前 I-6 校验。
    //
    // # 口径
    //
    // - 伤害 = reactionDmg >> 2（定点）。来源是**本次命中的
    //   `res.reactionDamage`**——双端已通过 `resolveHit` 逐位一致
    //   （formula_vectors.json 锁定），所以溅射伤害也一致。
    // - 源体 e 不计（自身已结算）。
    // - 半径 = 反应表 `aoeRadius`（像素，×1000 转定点）。
    // - 邻居伤害走 `resolveHit` 复用护甲/护盾/抗性，但**不再触发反应**
    //   （forceReaction 为空且邻居身上挂的已有元素 + 技能元素不再配对生成
    //   新反应 —— 否则 steam_burst 就会在溅射目标上**再次**爆开，
    //   形成指数级连锁。`res.reaction == ''` 守住这一点）。
    // - 目标集合确定：`this.enemies` 数组顺序 + 距离 ≤ 半径 + 存活。
    //   两者皆由种子决定（刷怪洗牌 / 坐标由 levelgen 产出），故同种子
    //   同行。`dist2` 用 bigint 避免浮点，不存在跨端漂移。
    if (res.reaction === 'steam_burst' && res.reactionDamage > 0n && reactSpec !== undefined && reactSpec.aoeRadius > 0) {
      this.triggerSteamBurstAoe(e, res.reactionDamage, att)
    }
    // 地形联动。地形自身负责判定弹丸是否经过它（updateXxx 内部已做空间判定），
    // 因此这里只需传入元素与伤害，不需要坐标。
    for (const t of this.terrains) {
      // 传入命中位置：地形要判断"这一发是否真的打在地形上"，
      // 否则站在远处的油桶会被任意位置的火焰点燃（见 Terrain.onHit 注释）。
      if (t.onHit(p.element, res.totalDamage, p.x, p.y)) {
        this.terrainUsed.push(t.kind)
        this.emit({ type: 'terrain', kind: t.kind, x: t.x, y: t.y })
        this.record(this.tick, 'terrain', terrainIndex(t.kind), t.x, t.y)
      }
    }

    if (res.killed) {
      e.dead = true
      this.kills++
      this.score += Number(killScore(this.scoreRules, e.isBoss))
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

  /**
   * steam_burst 的溅射范围伤害。
   *
   * 对「已命中触发 steam_burst 的敌人」周围 `aoeRadius` 内、存活的其他敌人，
   * 造成 `reactionDmg >> 2`（25%）的二次伤害。
   *
   * # 二次伤害为何不用 `hitEnemy`
   *
   * `hitEnemy` 是投射体命中专用：它从 `Projectile` 里读伤害/元素/滚点，
   * 并在结算后**施加元素层数** (`applyElementStacks`)。溅射如果复用它会：
   *   - 把技能元素再次施加到邻居身上（触发第二次 steam_burst / overheat）；
   *   - 消耗 `this.rng.roll`（rng 顺序偏移 → 整局事件流改动 → 哈希变化）。
   *
   * 所以这里只复用 `resolveHit` 的**装配 + 结算**部分（护甲/护盾/抗性)，
   * 不施加元素、不掷骰、不触发反应 —— 二次伤害纯粹是「命中减去护甲/护盾」。
   * 两端公式一致（formula_vectors.json），I-6 不受影响。
   *
   * # 确定性
   *
   * - 先判死亡：`e.dead` 已结算过的敌人跳过（不重复结算同一个死体）。
   * - 距离 `≤ rr`（含边界）、`this.enemies` 数组顺序。
   *   刷怪洗牌与坐标均由 levelgen 的 LCG 产出，所以同种子同行；
   *   `dist2`/`toFixed` 全定点，无浮点。
   */
  private triggerSteamBurstAoe(origin: Enemy, reactionDmg: bigint, att: Attacker): void {
    const spec = REACTIONS['steam_burst']
    const rr = toFixed(spec.aoeRadius)
    
    //
    // 2026-10-10 实现 steam_burst 范围伤害：aoeRadius 恢复为 120（像素）。
    // 引擎 `hitEnemy` 在 steam_burst 反应触发后、以命中敌人为圆心、半径
    // aoeRadius 搜索邻居，造成 reactionDmg >> 2（25%）的二次伤害。
    // I-6 安全：溅射伤害来自 resolveHit（formula_vectors.json 锁定），
    // 不进入 replay_skills 段，因而不影响 I-6 的 S-段校验。
    const splash = reactionDmg >> 2n
    if (splash <= 0n) return
    for (const e of this.enemies) {
      if (e.dead || e.uid === origin.uid) continue
      if (dist2(origin.x, origin.y, e.x, e.y) > rr * rr) continue

      const def = new Defender(e.hp, e.shield, e.armorShredMs > 0
        ? (e.armorPermille > e.armorShredPermille ? e.armorPermille - e.armorShredPermille : 0n)
        : e.armorPermille)
      for (const [el, v] of e.resist) def.resist.set(el, v)
      for (const [el, v] of e.stacks) def.stacks.set(el, v)

      // 二次伤害：复用 direct+damage 结算，不触发反应（input.skillElement=''）。
      const res = resolveHit(att, def, {
        skillDamage: splash,
        skillElement: '',
        roll: 9999, // 不暴击（crit 作用于 direct+element，不影响 splash 本身），固定滚点防范
      })
      e.hp = def.hp
      e.shield = def.shield
      e.stacks = def.stacks

      if (res.totalDamage > 0n) {
        this.score += Number(damageScore(this.scoreRules, res.totalDamage))
        this.emit({ type: 'hit', x: e.x, y: e.y, damage: res.totalDamage, crit: false })
        this.record(this.tick, 'hit', e.uid, Number(damageScore(this.scoreRules, res.totalDamage)), 0)
      }
      if (res.killed) {
        e.dead = true
        this.kills++
        this.score += Number(killScore(this.scoreRules, e.isBoss))
        this.emit({ type: 'kill', x: e.x, y: e.y, boss: e.isBoss })
        this.record(this.tick, 'kill', e.uid, e.isBoss ? 1 : 0)
        const dom = dominantOf(e) ?? 'fire'
        for (const t of this.terrains) {
          if (t.onKillCharging(dom, att.elementCap, this.terrainCtx())) {
            if (!this.terrainUsed.includes(t.kind)) this.terrainUsed.push(t.kind)
            this.emit({ type: 'terrain', kind: t.kind, x: t.x, y: t.y })
            this.record(this.tick, 'terrain', terrainIndex(t.kind), t.x, t.y)
          }
        }
      }
    }
  }

  private applyAoe(p: Projectile, origin: Enemy, radius: number): void {
    const rr = toFixed(radius)
    for (const e of this.enemies) {
      if (e.dead || e.uid === origin.uid) continue
      // 溅射目标也必须记进 hitSet。原实现不记，于是「穿透 + 溅射」组合下
      // 同一敌人会被反复结算：每穿透一个主目标就再跑一次 applyAoe。
      // 内容表里「电磁栅栏」Pierce 12 / AoeRadius 50 就是这个组合。
      if (p.hitSet.has(e.uid)) continue
      if (dist2(origin.x, origin.y, e.x, e.y) > rr * rr) continue
      if (this.hitEnemy(p, e, true)) {
        p.hitSet.add(e.uid)
      }
    }
  }

  private currentAttacker(): Attacker {
    const base = this.cfg.attacker
    const lv = this.cfg.level
    return {
      attack: base.attack + this.buffs.attackPermille,
      // ⚠️ 第 73 轮：这里夹一层 `min(PERMILLE)`。
      //
      // 消费侧的判据是 `roll >= PERMILLE - critPermille`
      // （damage.ts:348，与 Go 的 damage.go:359 同式）。
      // 当 critPermille > 1000 时右边变成**负数**，而 roll 是 0..9999 ——
      // 于是判据恒真，**每一发都暴击**。
      //
      // # 为什么能超过 1000
      //
      // `Attacker.CritPermille` 的文档写着「**0..1000 约定**」
      // （Go 侧 damage.go:63 同样写着），但**两端都不执行它**：
      //
      //   装备侧  loadout_attacker.go 把词缀和夹到 MaxLoadoutCritPermille=500
      //   基础值  defaultAttacker() = 50
      //           → base.critPermille ≤ 550
      //   局内卡  applyAttribute 的 crit 卡 value: 80n，**无夹取**
      //
      // 每波恰好 1 张属性卡（`rollWaveCards` 的固定构成），
      // 最难关卡 10 波（levelgen.go 的 chapter 6），
      // 6 张 crit 卡 = 550 + 480 = **1030 > 1000**。
      //
      // 概率不高（每波 1/6 命中，最多 10 波，P(≥6) ≈ 0.2%），
      // 但这是**确定性**的：种子给定后必然如此。
      //
      // # 为什么是夹而不是「报错」
      //
      // 这是客户端的局内计算，服务端**不重算 replay_hash**
      // （只校验长度，battle_collections.go:318），
      // 所以夹取不会造成跨端哈希分歧。
      //
      // 而「报错」在这里无从谈起 —— 玩家无法选择抽到哪张卡，
      // 卡池是种子决定的。夹取是唯一能把越界值变成合法值的地方。
      //
      // # 为什么只夹 crit，不夹 attack / elementCoefPermille
      //
      // 那两个是**线性**消费（`d = skillDamage * (1000 + attack) / 1000`），
      // 越界只是数值变大，没有行为**悬崖**。
      // crit 是唯一有悬崖的：`permille - critPermille` 会变号。
      critPermille: minBig(base.critPermille + this.buffs.critPermille, PERMILLE),
      critMultiplierPermille: base.critMultiplierPermille,
      reactionMultPermille: base.reactionMultPermille,
      elementCap: (lv.element_cap ? BigInt(lv.element_cap) : base.elementCap) + this.buffs.elementCapBonus,
      reactionTier: BigInt(lv.max_reaction_tier || 1),
      elementCoefPermille: base.elementCoefPermille + this.buffs.elementCoefPermille,
      // 这三项**原样透传**，不与局内 Buffs 相加 ——
      // 因为它们的消费者不在伤害公式里：
      //   armorPermille    → defenseArmorPermille()（防线护甲，不走 currentAttacker）
      //   heatCapPermille  → 构造器注入 heat.capBonus
      //   mechanicPermille → applyMechanic 放大卡面数值
      //
      // 在这里相加会让它们**既*被计入*攻方又*被单独消费*** 一次，
      // 而 currentAttacker 的返回值也参与重放哈希 —— 多加一次就哈希不符。
      heatCapPermille: base.heatCapPermille,
      armorPermille: base.armorPermille,
      mechanicPermille: base.mechanicPermille,
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
      // ⚠️ 第 77 轮：元素层数上限必须**动态**取，不能写常量。
      //
      // 它是 `lv.element_cap` + 局内 `elementCapBonus` 的合成值
      // （与 `currentAttacker()` 里那一项同源），玩家还能靠
      // 「元素容器」卡（element_cap +1）抬高。
      //
      // 用 `currentAttacker()` 而不是各处重算：那是唯一一处
      // 已经把这个合成值算对的地方，重算就是**第二份实现**。
      elementCap: this.currentAttacker().elementCap,
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
        this.score += Number(killScore(this.scoreRules, e.isBoss))
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
    // ⚠️ 第 66 轮：本波弃过牌时，把「弃过」编码进记录，
    // 否则重放不会复现那次 refundHeat 与 record 事件（哈希必然失配）。
    this.recordPick(this.discardedThisWave ? -(PICK_DISCARD_BASE + handIdx) : handIdx)
    // ⚠️ 第 66 轮：走 finishCardSelect 而不是直接 beginWave ——
    // 直接 beginWave 会让 `wave` 事件落在取牌那一 tick，而原局的
    // wave 事件在下一 tick（实测 DIFF@61 orig=402 repl=401）。
    if (this.deck.size === 0) this.finishCardSelect()
    return card
  }

  /**
   * 记录本波选中的手牌下标（-1 = 整波跳过）。
   *
   * 语义：手牌在本次 offer 中的下标（0/1/2），对应 applyReplayDecision 的解释。
   *
   * ⚠️ 第 66 轮：新增「本波弃过一张」的标记。
   *
   * 原来每波只记第一次选择，注释写「玩家若先弃牌后取牌，记的是取的那次」。
   * **索引语义确实是对的**（弃一张后取到的下标仍指向同一张卡），
   * 但**弃牌的副作用无人复现**：
   *   - `discardCard` 会 `refundHeat(1)`（降低热量、延后过热）
   *   - 以及 `record(..., 'card', ..., 1)` 写进回放事件流
   *
   * 而重放侧 `applyReplayDecision` 只按脚本取牌，既不回充热量也不写那条事件
   * → 事件流少一条 → **replayHash 必然不同** → I-6 把这局判成伪造。
   *
   * 「先弃后取」是 `discardCard` 与 `takeCard` 都允许的合法操作
   * （两者只判 `phase === 'card_select'`），所以这不是理论漏洞。
   *
   * # 为什么不改协议去表达「弃牌张数」
   *
   * `card_picks` 是 `number[]`，服务端校验 `len <= WaveCount` 且每项 `>= -1`。
   * 改成变长序列要动跨端契约与历史战报兼容性；而弃牌每波**最多 1 次**
   * （`DISCARD_PER_WAVE = 1`），所以「有没有弃过」这一个 bit 就够。
   *
   * 编码：`-2 - handIdx` 表示「本波先弃过一张，然后取第 handIdx 张」。
   * handIdx 是**弃牌之后**的手牌下标（弃牌移除的是更靠前的一张），
   * 与 `takeCard` 内部取下标的时机一致。
   *
   * ⚠️ 下界必须是 -2 - (handMax-1)。当前每波 3 张牌（rollWaveCards 固定
   * 产 skill/attribute/mechanic 各 1），handIdx ∈ [0,2] → 最负 -4。
   * 服务端 `p < -1` 的拒绝对本编码仍然成立（-2..-4 全被拒）。
   * 所以**必须同步放宽服务端下界到 -4**，否则正常对局会被 422。
   */
  private recordPick(handIdx: number): void {
    if (this.cardPicks.length <= this.waveIndex) {
      this.cardPicks.push(handIdx)
    }
  }

  /**
 * 结束本波选牌：清空剩余手牌并进入下一波（第 66 轮新增）。
 *
 * # ⚠️ 这里**不能**为剩余手牌写 record 事件
 *
 * 第一版写了 `record(tick, 'card', cardIndex(c), 1)`，理由是
 * 「它们确实被丢弃了，应该记进事件流」。实测这是**错的**：
 *
 *   原局 ORIGCARD=401/30/1 401/506/0 **402/902/0** ...
 *   重放 REPLCARD=401/30/1 401/506/0 **401/902/1** ...
 *                                      ↑ tick 差 1 格、flag 差 1
 *
 * 根因：**原局根本没有「丢弃剩余手牌」这个动作**。
 * 玩家取 1 张就离开选牌界面，剩下那 2 张一直躺在 `deck.hand` 里，
 * 直到下一波的 `deck.setHand(cards)` 直接覆盖 —— 全程没有任何事件。
 *
 * 所以重放侧也不该造这个事件。`FIRSTDIFF@61` 精确定位到这一条：
 * 事件流一多，后续所有 tick 全部偏移 → replayHash 不同。
 *
 * # 为什么还需要这个函数（而不是继续用 `deck.size === 0`）
 *
 * 原来只有「拿光最后一张牌」才进下一波。玩家「取 1 张就走」的正常路径
 * 走不到那里，于是 `phase` 停在 `card_select`，
 * 重放侧下一 tick 又进 `applyReplayDecision` 消费脚本下一项 → 波次错位
 * （实测 `discardsLeft` 显示 wave 2 被进入两次）。
 *
 * 这里把「取完就结束本波」显式化，用 `deck.drop` 静默清空
 * （不消耗弃牌次数、不写事件），`skipCards` 与弃牌路径共用。
 */
private finishCardSelect(): void {
    for (const c of [...this.deck.hand]) {
      this.deck.drop(c.id)
    }
    this.beginWave(this.waveIndex + 1)
  }

  /**
   * 结束本波选牌，但**下一 tick** 才推进到下一波（第 66 轮）。
   *
   * # 为什么需要这个「延后一 tick」的变体
   *
   * 原局里玩家「弃 1 张、拿 1 张、剩下 2 张不动」，此时 `deck.size !== 0`，
   * `takeCard` 里的 `if (this.deck.size === 0) beginWave(...)` 不成立 ——
   * 于是 phase 停在 `card_select`，**下一 tick** 玩家离开界面时才推进。
   *
   * 实测证据（弃+取，每波一张）：
   *
   *   CARD_O: 401/30/1 401/506/0  [tick 728] 40/1 ...
   *   CARD_R: 401/30/1 401/506/0  [tick 727] 40/1 ...
   *                                          ^ wave 事件早了一格
   *
   *  ⚠️ 写这段注释时踩过一个坑：`... 506/0 **728**-/40/1 ...` 里的
   * `**` 与紧随的 `/` 组成了注释闭合序列，把块注释提前结束 ——
   * 表现是 esbuild 报 `Expected ";" but found "/"`，
   * 而报错行号落在**注释内部**，看起来完全无辜。
   // 在注释里写事件流样例时，避免让 `**` 紧跟 `/`。
   *
   * 而「弃牌后跳过整波」那条路径不同 —— `skipCards` 是**同 tick** 推进的
   * （原局 `401:wave` 连着出现两次）。
   *
   * 两条路径的 tick 语义本来就不同，必须分别表达；
   * 把它们统一成一种只会让其中一条失配。
   *
   * # 为什么重放侧必须走这条延后路径
   *
   * `applyReplayDecision` 在 `return` 之后，phase 仍是 `card_select`。
   * 若不在下一 tick 立刻收尾，`applyReplayDecision` 会在**同一 tick**
   * 再被调用一次（`step()` 开头无条件检查），消费 script 的下一项 ——
   * 把下一波的决策当成本波的第二张牌。实测 picks 记成 `[-3,-3,-1,-1]`，
   * 事件流多出 `402/902/0` 这条原局没有的记录。
   *
   * 所以：重放侧必须在**返回后**由 `step()` 在下一 tick 开头收尾，
   * 且收尾时机要与原局「玩家离开界面」那一步对齐。
   */
private finishCardSelectNextTick(): void {
    for (const c of [...this.deck.hand]) {
      this.deck.drop(c.id)
    }
    this.cardSelectDone = true
  }

  /** 本波选牌已结束，等待下一 tick 推进到下一波（第 66 轮）。 */
  private cardSelectDone = false

  /** 本波是否已经弃过牌（第 66 轮新增）。 */
  private discardedThisWave = false

  /**
   * 弃牌。
   *
   * ⚠️ 第 66 轮：弃牌会**改写本波的 card_picks 记录**，
   * 让重放能复现这次弃牌（回充热量 + 那条 record 事件）。
   * 见 `recordPick` 的注释。
   */
  discardCard(id: string): Card | null {
    if (this.phase !== 'card_select') return null
    const card = this.deck.hand.find((c) => c.id === id)
    if (!card) return null
    const r = this.deck.discard(id)
    if (!r.ok) return null
    this.heat.refundHeat(r.refund)
    this.emit({ type: 'card_discarded', card })
    this.record(this.tick, 'card', cardIndex(card), 1)
    this.discardedThisWave = true
    if (this.deck.size === 0) this.beginWave(this.waveIndex + 1)
    return card
  }

  /** 放弃全部手牌，直接进入下一波。不消耗弃牌次数，也不返还热量 */
  skipCards(): void {
    if (this.phase !== 'card_select') return
    // -1 表示整波跳过（重放时按此原样复现）
    //
    // ⚠️ 第 66 轮：弃过牌再跳过时不能记 -1 ——
    // 那样重放侧不知道要复现那次 discard 的 record 事件与热量变化。
    // 记 `-(PICK_DISCARD_SKIP_BASE + handIdx)`，handIdx 取弃牌后的第一张
    // （那正是「跳过时手牌里还剩什么」的信息，重放侧只需丢掉它）。
    const encoded = this.discardedThisWave ? -PICK_DISCARD_SKIP_BASE : -1
    this.recordPick(encoded)
    this.finishCardSelect()
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
      // `script === null` 的 true 分支**不可达**：applyReplayDecision 仅在
      // step() 里 `phase==='card_select' && this.replayScript !== null` 时被调用，
      // 且循环内无处把 replayScript 置空。保留是类型收窄（null 排除）用。
      /* v8 ignore next -- 见上：调用点已保证 replayScript!==null，此真分支不可达 */
      if (script === null) return

      if (this.replayScriptPos >= script.length) {
        this.skipCards()
        continue
      }
      const want = script[this.replayScriptPos++]
      // ⚠️ 第 66 轮：负值编码表示「本波弃过牌」。
      //   -1                  整波跳过（未弃牌）
      //   -(3+handIdx)        弃一张，取第 handIdx 张   [-3..-5]
      //   -(6+handIdx)        弃一张，整波跳过           [-6..-8]
      //
      // 原实现把任何 `want < 0` 都当「整波跳过」，于是「先弃后取」
      // 与「弃后跳过」两种正常打法都被重放成「整波跳过」——
      // 少了那次 refundHeat 与那条 record 事件，replayHash 必然不同，
      // I-6 把这局判成伪造。
      if (want <= -PICK_DISCARD_SKIP_BASE) {
        // 弃一张后跳过：先复现 discard 的三件事，再走 skipCards。
        const toDrop = this.deck.hand[0]
        if (toDrop) this.discardCard(toDrop.id)
        // recordPick 已被 discard 路径影响，这里显式写 -1 之外的语义：
        // skipCards 会再 recordPick 一次，但 `length <= waveIndex` 保证只写一项。
        this.skipCards()
        return
      }
      if (want <= -PICK_DISCARD_BASE) {
        // 弃掉第一张手牌。discardCard 会做原局同样的三件事：
        // 扣次数 + refundHeat + record(..., 1)。
        const toDrop = this.deck.hand[0]
        if (toDrop) this.discardCard(toDrop.id)
        // 编码是 -(PICK_DISCARD_BASE + handIdx)，所以 handIdx = -(want + PICK_DISCARD_BASE)
        const rest = -(want + PICK_DISCARD_BASE)
        const target = this.deck.hand[rest]
        if (!target) {
          this.skipCards()
          continue
        }
        this.takeCard(target.id)
        // ⚠️⚠️ 这里必须 `return`，**不能** `continue`（第 66 轮）。
        //
        // 一个 card_picks 项就是**一整波**的决策，原局里玩家
        // 「弃 1 张、拿 1 张、剩下 2 张不动」就结束了 ——
        // 剩下那 2 张在**下一 tick** 玩家离开界面时才被丢掉。
        //
        // 若写成 continue，while 会在同一 tick 里消费 script 的下一项，
        // 把**下一波**的决策当成本波的第二张牌。实测 trace：
        //   want=-3 pos=1 wave=0 hand=3
        //   want=-3 pos=2 wave=0 hand=1   ← 本波只剩 1 张牌，本该结束
        // 结果第 3、4 波错位，cardPicks 记成 [-3,-3,-1,-1]，事件流少 7 条。
        //
        // 用 `finishCardSelectNextTick` 而不是 `finishCardSelect`：
        // 原局这条路径不立即进下一波（`deck.size` 非 0），
        // 立即推进会让 `wave` 事件早一格（实测 orig=728 repl=727）。
        this.finishCardSelectNextTick()
        return
      }
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
    } else if (card.kind === 'skill' && card.skillId !== undefined) {
      // ⚠️ 判据用 `!== undefined` 而不是真值判断：
      // skillId === 0 曾经落进最后的隐式 else，卡被 consume 掉、
      // emit 了 card_taken，但什么也没发生 —— 玩家看到"获得技能卡"却没反应。
      // 内容表当前 id 从 1 开始所以不触发，但 id 空间没有"必须非 0"的保证。
      //
      // 技能卡：若已在槽内则升格，否则装入**空槽**
      const existing = this.skills.find((s) => s.skillId === card.skillId)
      if (existing) {
        // 升格：更高伤害 + 更高层数 + 更高穿透
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
        if (def) {
          // 找**真正空着**的主动槽。
          //
          // ⚠️ 原来是 `findIndex(s => s.slot < ACTIVE_SLOTS && s.slot >= 0)`，
          // 而 skills 数组里每个元素就代表一个已占用的槽 ——
          // 于是这个表达式命中的永远是**第一个已占用**的槽，
          // 随后 `s.slot === slot ? {...新技能} : s` 直接把它替换掉。
          //
          // 后果：抽到任何**新**技能都会顶掉槽 0 的老技能，
          // 槽位数永远不增加。实测已装 [1,2] 时抽到技能 9：
          //   before = [1,2] slots=[0,1] → after = [9,2] slots=[0,1]
          // 技能 1 被销毁，而玩家的三选一白白消耗了一次机会。
          // （只有抽到**已装备**技能的卡才会"升格"，所以前期是净损失。）
          const activeSlots = new Set(
            this.skills.filter((s) => s.slot < this.activeSlots).map((s) => s.slot),
          )
          let free = -1
          for (let i = 0; i < this.activeSlots; i++) {
            if (!activeSlots.has(i)) {
              free = i
              break
            }
          }
          if (free < 0) {
            // 主动槽已满：升格一个伤害最高的技能（不消耗玩家的选牌机会）
            //
            // 槽满时最合理的处理不是"顶掉随机一个"，
            // 而是明确地告诉玩家"没位置了"—— 所以这里做的是
            // 「强化已有技能里伤害最高的那一个」，
            // 至少不会让玩家的构筑凭空少一个技能。
            const best = this.skills
              .filter((s) => s.slot < this.activeSlots)
              .reduce<(typeof this.skills)[number] | null>(
                (acc, s) => (acc === null || s.baseDamage > acc.baseDamage ? s : acc),
                null,
              )
            if (best) {
              this.skills = this.skills.map((s) =>
                s.slot === best.slot
                  ? {
                      ...s,
                      baseDamage: s.baseDamage + s.baseDamage / 5n,
                      applyStacks: s.applyStacks + 1n,
                      pierce: s.pierce + 1,
                    }
                  : s,
              )
            }
            return
          }
          this.skills = [
            ...this.skills,
            {
              skillId: def.id,
              name: def.name,
              element: def.element as Element,
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
              slot: free,
              cooldownRemaining: 0,
            },
          ]
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
        // ⚠️ 这里**只**写 heat.capBonus，不要在 Buffs 里再存一份。
        //
        // 而真正被 `HeatMeter.cap`（`get cap() { return HEAT_MAX + this.capBonus }`）
        // 读走的是后者 —— 前者成了**只写不读的镜像字段**。
        //
        // 危害不是"多占一点内存"，而是**陷阱**：
        // 实际没人维护的值（比如从别处加了 Buffs 却没同步 HeatMeter），
        // 而全绿的测试不会提示任何异常。
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
    // 机制卡强度加成（专精树第 3 层槽 1「机制改造」，8 个节点）。
    //
    // ⚠️ 这个 kind 此前在服务端 `EvaluateMastery` 的 switch 里**连 case 都没有**，
    // 于是 8 系 × 1 个节点 = 8 个真实节点完全惰性 ——
    // 玩家花点数点出「机制改造」，战斗里什么都不变。
    //
    // 语义：把卡面数值整体放大 (1000 + v)/1000。
    // 选"放大卡面数值"而不是"抽到好卡的概率更高"，是因为前者是确定性的
    // （重放时能逐位复现），后者需要改抽牌随机序列，
    // 而卡牌是客户端选的、改序列会与 I-6 的 CardPicks 闭环冲突。
    // 全程 bigint 运算（README 工程约束 8：禁止实数运算）。
    //
    // MechanicEffect.value 是 number（小整数计数），攻方是 bigint，
    // 所以先 Math.trunc 成整数再进 bigint 域 —— 内容表的机制卡面值
    // 全是整数，trunc 不会丢精度；真出现小数时是内容表错了，
    // 静默四舍五入会让"卡面显示 3、实际生效 2"这类问题更难查。
    const v = Number(
      mulDiv(
        BigInt(Math.trunc(m.value)),
        PERMILLE + this.cfg.attacker.mechanicPermille,
        PERMILLE,
      ),
    )
    switch (m.kind) {
      case 'pierce_bonus':
        this.buffs.pierceBonus += v
        break
      case 'chain_bonus':
        this.buffs.chainBonus += v
        break
      case 'aoe_bonus':
        this.buffs.aoeBonus += v
        break
      case 'free_discard':
        this.buffs.freeDiscard += v
        this.deck.discardsLeft += v
        break
      case 'overheat_guard':
        this.buffs.overheatGuard += v
        break
    }
  }

  private finish(win: boolean, stalemate = false): void {
    this.phase = win ? 'won' : 'lost'
    const stars = this.calcStars()
    if (win) {
      this.emit({ type: 'won', score: this.score, stars })
    } else {
      this.emit({ type: 'lost', score: this.score })
    }
    // 停滞必须**记进回放**：它是一个真实发生过的终局状态，
    // 而重放方要能复现"这局是被上限截断的"而不是"这局还在跑"。
    // 不记就等于哈希不覆盖这个结局 —— 同一场战斗，
    // 一次跑到上限结束、一次靠玩家操作结束，哈希会不同。
    if (stalemate) {
      this.record(this.tick, 'stalemate', this.tick)
    }
  }

  /**
   * 停滞检测：到达绝对 tick 上限就强制结束。
   *
   * ⚠️ 这不是"防玩家卡住"的兜底，而是**防引擎挂死**的兜底。
   *
   * 实测踩过：第 22/33/34 关因为「掩体挡弹丸 → 弹丸打不到敌人 →
   * 掩体永远打不破」的循环依赖，引擎**永久停在 wave 阶段** ——
   * 12000 tick 只杀 2~17 只怪，命中率掉到 1%，战斗不会自然结束。
   * 在真机上那意味着玩家盯着一个永远不结算的画面。
   *
   * 那个具体缺陷已修（见 updateProjectiles 里的注释），但：
   *   - 循环依赖/互斥这类缺陷很难靠"读代码看出��"杜绝
   *   - 任何未来的内容改动都可能再造一个
   *   - 挂死的代价（玩家永久卡在一局、无法退出、体力不返还）远高于误判
   *
   * 所以这里加一道**兜底**：超过预算就判负结束。
   *
   * 预算取 30000 tick = 1500s = 25 分钟：
   *   实测最慢的合法关卡是第 100 关 594s（11880 tick），
   *   30000 是它的 2.5 倍余量 —— 合法对局**不可能**触碰。
   *   而服务端对 duration_ms 的上界（1800s）比它更宽，
   *   避免出现"客户端结束了、服务端却认为时长非法"的不一致。
   */
  private checkStalemate(): boolean {
    // 终局守卫的 true 分支**不可达**：step() 开头已 `if (won||lost) return`（见 step），
    // checkStalemate 只在其后被调用，运行到这里时 phase 必非终局。是与 step 重复的
    // 双保险，保留但当前无法触达。
    /* v8 ignore next -- 见上：step 已拦终局，此守卫真分支不可达 */
    if (this.phase === 'won' || this.phase === 'lost') return false
    if (this.tick < MAX_BATTLE_TICKS) return false
    this.emit({ type: 'stalemate' })
    this.finish(false, true)
    return true
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
      /**
       * 第 56 轮新增：前缀里的 S 段（技能槽配置）。
       *
       * 服务端用 `user_skill_slots` + `user_skills.level` + 内容表重算它并比对。
       * 这不需要任何战斗模拟 —— 与 replay_hash 整体不同，那个算不出来。
       *
       * 抓的是：伪造 base_damage（假报技能等级）、上报另一套技能、槽位错位。
       *
       * ⚠️ 第 64 轮：这里用 `buildSnapshotSegment()`（开战前冻结），
       * **不是** `replaySkillsSegment()`（读 this.skills，含局内取牌升格）。
       * 服务端比对的是 DB，而 DB 里没有局内升格这回事 ——
       * 用后者会让取过技能卡的正常对局被 422 判成作弊（约 86% 的真实对局）。
       */
      replay_skills: this.buildSnapshotSegment(),
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
  // 三处防御性回退（`skillId ?? 0`、`effect ? :0`、`mechanic ? :0`）的落空分支**不可达**：
  // cardIndex 只被 record() 用于牌局中的真实手牌，而手牌全部由 CardDeck 造出——
  // skill 卡必带 skillId、attribute 卡必带 effect、mechanic 卡必带 mechanic（见 heatmap.ts）。
  // 缺字段的卡无从经公开 API 进入本函数。分派用的 kind 判定本身由 take/discard/skip 用例覆盖，
  // 这里整体豁免只影响这些永不触达的空值回退。
  /* v8 ignore next 3 -- 见上：卡片恒为良构，三处空值回退分支不可达 */
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

/**
 * ⚠️ 第 70 轮删除：`uidOf(v) = Number(v % 100000n)`。
 *
 * 它唯一的调用点是射程内漏怪那一条，传入的是**伤害值**：
 * `record(tick, 'leak', uidOf(dmg))` —— 于是 `a` 变成了
 * 「漏怪伤害 mod 100000」，与敌人 uid 无关。
 *
 * 函数名说「取 uid」而实参是伤害值，说明写的时候想的是别的东西；
 * 且抵达防线那条路径写的是 `e.uid` —— 两条路径语义不一致。
 *
 * 留着它会让人以为「取 uid」是个通用工具而去复用它。
 * 需要「把大整数压进 number」时应该显式写 `Number(v % 100000n)`，
 * 让「为什么要取模」摆在现场。
 */
