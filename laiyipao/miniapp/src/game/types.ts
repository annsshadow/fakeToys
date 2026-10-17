/**
 * 战斗实体与配置类型。
 * 与服务端 domain 的 JSON 结构对应（后端经 /api/v1/config 下发）。
 */

import type { Element, ReactionKey } from './elements'

export type EnemyCategory = 'normal' | 'ranged' | 'flying' | 'special' | 'elite' | 'boss'

/**
 * 敌人定义。对应服务端 domain.SeedEnemy 的 json tag。
 *
 * ⚠️ 全部 snake_case：Go struct 若漏写 json tag 会按字段名原文（PascalCase）
 * 序列化，两边对不上时客户端读到的每个字段都是 undefined，
 * 表现为「战斗里一个敌人都没有」。这个坑真实发生过。
 */
export interface EnemyDef {
  id: number
  code: string
  name: string
  category: EnemyCategory
  hp: number
  /** 定点 ×1000，像素/秒 */
  speed: number
  armor: number
  shield_hp: number
  attack: number
  attack_range: number
  /** 攻击间隔毫秒 */
  attack_interval: number
  fly_height: number
  burrow: boolean
  is_boss: boolean
  /** 五维抗性，千分比 -500..500 */
  resist: Record<string, number>
  descr: string
}

export interface SkillDef {
  id: number
  code: string
  name: string
  family: string
  element: Element
  kind: 'active' | 'passive'
  descr: string
  base_damage: number
  heat_cost: number
  cooldown_ms: number
  pierce: number
  aoe_radius: number
  apply_element: string
  apply_stacks: number
  projectile_speed: number
  chain: number
  unlock_level: number
}

export interface Spawn {
  enemy_id: number
  count: number
  interval: number
  delay: number
}

export interface Wave {
  wave_index: number
  spawns: Spawn[]
}

export type TerrainKind =
  | 'oil_drum'
  | 'tidal_gate'
  | 'rotor_vane'
  | 'collapse_wall'
  | 'charge_tower'

export interface TerrainPlacement {
  kind: TerrainKind
  x: number
  y: number
  param: number
}

/**
 * 关卡**摘要**：`/config` 的 `levels` 就是这个形状 —— 比完整关卡少 `waves`。
 *
 * ⚠️ 与 `GeneratedLevel` 是**两个类型**不是同一个类型的可选字段。
 * 理由：`waves` 一旦写成可选（`waves?: Wave[]`），引擎里那三处
 * `cfg.level.waves` 的读取就会因为「忘了判空」而在运行时炸；
 * 而在类型层面分开，**编译期**就会把「谁需要完整关卡」逼出来。
 *
 * 跨端对拍：Go 侧 `domain.LevelSummary`，由
 * `TestLevelSummaryCoversEveryGeneratedLevelField` 反射逐字段钉住。
 */
export interface LevelSummary {
  id: number
  chapter: number
  name: string
  seed: string
  base_hp: number
  wave_count: number
  difficulty: number
  energy_cost: number
  element_cap: number
  armor_permille: number
  max_reaction_tier: number
  /**
   * 理论满分（1000‰）。star_targets 是它按 [600, 850, 980]‰ 取的三个档。
   *
   * ⚠️ 必须与 star_targets 分开下发：服务端结算裁剪的上界锚定在**这个值**上，
   * 而不是 star_targets[2]。早期版本拿 3 星门槛当裁剪上限（= 理论满分的 75%），
   * 于是每一次干净通关的分数都被裁掉（实测关 1：本地 19842 → 结算 14656），
   * 而且 Clamped 在几乎每次正常胜利上都为真，把这个本该用于发现伪造的信号变成了噪声。
   */
  max_score: number
  is_boss: boolean
  star_targets: number[]
  terrain: TerrainPlacement[]
}

/**
 * 关卡**完整**形状：摘要 + 波次排布。
 *
 * 三条下发路径给的是它：`/battle/token`、`/battle/:id/replay`、`/levels/:id`。
 * **引擎只认这个类型** —— 战斗局必须能拿到 waves 才能开局。
 */
export interface GeneratedLevel extends LevelSummary {
  waves: Wave[]
}

export interface MasteryNode {
  id: number
  family: string
  layer: number
  slot: number
  name: string
  kind: string
  value: number
  prereq_family: string
  prereq_layer: number
}

export interface MasteryFamily {
  family: string
  name: string
  nodes: MasteryNode[]
}

/** 战斗中的活动实体 */
export interface Enemy {
  uid: number
  defId: number
  name: string
  category: EnemyCategory
  /** 定点坐标（×1000） */
  x: bigint
  y: bigint
  hp: bigint
  maxHp: bigint
  shield: bigint
  armorPermille: bigint
  flyHeight: number
  burrow: boolean
  isBoss: boolean
  resist: Map<Element, bigint>
  stacks: Map<Element, bigint>
  /** 施加元素层数（受 elementCap 约束）。terrain.ts 的蓄能塔用它给全场挂元素。 */
  applyElement(e: Element, add: bigint, cap: bigint): void
  /** 移动速度（定点） */
  speed: bigint
  /** 攻击力（扣防线血量用） */
  attack: bigint
  /** 射程（**逻辑单位**的 bigint，内容 = def.attack_range） */
  attackRange: bigint
  /**
   * 射程的**定点形式**（= `toFixed(attack_range)`），出生即恒定。
   *
   * 与上一个是同一件事的两种量纲：定点坐标系里的距离比较需要它，
   * 而 `updateEnemies` 原本每 tick 都在做一次 `toFixed(Number(attackRange))`
   * 的往返转换。派生缓存，不参与任何玩法输入。
   */
  attackRangeFixed: bigint
  attackInterval: number
  attackCooldown: number
  /** 状态效果剩余毫秒 */
  frozenMs: number
  stunnedMs: number
  slowedMs: number
  /** 受击伤害放大（千分比，来自超导/急速冻结） */
  amplifyPermille: bigint
  /** 击退位移（定点） */
  knockback: bigint
  /** 削甲剩余毫秒（armor_break 反应：护甲被削减的持续时间） */
  armorShredMs: number
  /** 当前削甲量（千分比，armorShredMs>0 时生效；hitEnemy 构建 Defender 时折进有效护甲） */
  armorShredPermille: bigint
  dead: boolean
  /** 出生动画进度 0..1 */
  spawnProgress: number
  hitFlashMs: number
  /**
   * 单体命中判定半径的**平方（定点）**，出生即恒定。
   *
   * = `toFixed(28 + flyHeight * 0.1)²`
   *
   * 为什么字段化（第 158 轮性能整改）：`withinEnemy` 是全引擎调用次数
   * 最多的判定（每个弹丸 × 每个敌人 × 每 tick），而它依赖的半径只由
   * 出生时就固定的 `flyHeight` 决定 —— 每 tick 重算等于把同一个值
   * 计算上千遍，每次还新建一个 bigint。
   *
   * ⚠️ 它是**派生缓存**，不是新的玩法输入：
   * 必须与 `flyHeight` 一致，只能在 `spawnEnemy` 里由 flyHeight 算出。
   */
  hitRadius2: bigint
}

export interface Projectile {
  uid: number
  skillId: number
  element: Element
  x: bigint
  y: bigint
  vx: bigint
  vy: bigint
  damage: bigint
  applyStacks: bigint
  pierceLeft: number
  chainLeft: number
  /** 已命中的目标 uid，避免同一次穿透重复命中 */
  hitSet: Set<number>
  aoeRadius: number
  dead: boolean
  /** 表现用：拖尾长度 */
  trailLen: number
  slot: number
}

export interface FloatText {
  x: bigint
  y: bigint
  text: string
  color: string
  lifeMs: number
  maxLifeMs: number
  size: number
}

/** 回放事件（I-6 的哈希输入） */
export type ReplayEventType =
  | 'fire'
  | 'hit'
  | 'reaction'
  | 'kill'
  | 'leak'
  | 'wave'
  | 'overheat'
  | 'card'
  | 'terrain'
  /**
   * 「隔热护罩」免疫了一次过热。
   *
   * ⚠️ 必须与 'overheat' 分开成一个事件类型。
   * 用同一个类型的话，重放时无法区分"真的过热了"与"被护盾扛过去了"，
   * 而两者对战斗的影响完全不同（后者不进 overheated 状态）。
   * 记进哈希也是必要的：护盾消耗是战果的一部分，
   * 不记就等于"哈希记录了一个玩家看得见、却不影响判定的选择"。
   */
  | 'guard'
  /**
   * 战斗到达绝对 tick 上限被强制结束（引擎停滞兜底）。
   *
   * ⚠️ 必须与 'overheat' 一样**记进回放**：它是一个真实发生过的终局，
   * 而「这局被上限截断」与「这局自然结束」对重放方是两种不同的历史。
   * 不记就等于哈希不覆盖这个结局。
   */
  | 'stalemate'

export interface ReplayEvent {
  t: number
  type: ReplayEventType
  /** 紧凑数值：敌 uid / 技能 id / 反应序号 / 波次 */
  a: number
  /**
   * 第二载荷。逐 type 的语义（第 70 轮补全）：
   *
   * - `leak`：漏掉的伤害值（`a` 是**敌人 uid**）
   * - `card`：1 = 丢弃 / 0 = 取牌
   * - `wave`：1（恒定）
   * - 其他：恒为 0
   *
   * ⚠️ 此前 `leak` 的两条路径一处把伤害写进 `a`、一处写 uid，
   * 无论走哪条都丢掉一个信息。现在约定 `a` = uid、`b` = 伤害。
   */
  b: number
  c: number
}

export type ReactionName = ReactionKey
