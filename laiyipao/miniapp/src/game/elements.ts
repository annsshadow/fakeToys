/**
 * 元素与反应链（I-1 的数据定义）。
 *
 * ⚠️ 本文件与 server/internal/domain/elements.go 必须逐行等价。
 * 元素顺序（ELEMENT_ORDER）影响覆盖率计算与哈希，改动必须同步两处。
 */

export type Element = 'fire' | 'ice' | 'lightning' | 'corrosion' | 'kinetic'

/** 固定顺序，覆盖率计算与哈希都依赖它 */
export const ELEMENT_ORDER: readonly Element[] = [
  'fire',
  'ice',
  'lightning',
  'corrosion',
  'kinetic',
] as const

export const ELEMENT_NAME: Record<Element, string> = {
  fire: '焰',
  ice: '冰',
  lightning: '电',
  corrosion: '毒',
  kinetic: '动能',
}

/** 元素主题色，与后台 CSS 变量保持一致 */
export const ELEMENT_COLOR: Record<Element, string> = {
  fire: '#ff6b35',
  ice: '#58a6ff',
  lightning: '#ffd33d',
  corrosion: '#7ee787',
  kinetic: '#c9a7ff',
}

export type ReactionKey =
  | 'steam_burst'
  | 'overheat'
  | 'burn_cloud'
  | 'superconduct'
  | 'flash_freeze'
  | 'corrosion_spread'
  | 'armor_break'

/** 固定顺序 */
export const REACTION_ORDER: readonly ReactionKey[] = [
  'steam_burst',
  'overheat',
  'burn_cloud',
  'superconduct',
  'flash_freeze',
  'corrosion_spread',
  'armor_break',
] as const

export interface ReactionSpec {
  key: ReactionKey
  name: string
  /** 反应基础系数（千分比） */
  baseCoef: number
  /** 反应伤害中由攻击力贡献的比例（千分比）—— 硬性 ≤ 300 */
  attackWeightPct: number
  /** 附加状态时长（毫秒），0 表示无 */
  statusDurationMs: number
  /** 溅射半径（像素），0 表示单体 */
  aoeRadius: number
  /** 是否驱散护盾 */
  dispelShield: boolean
  /** 对目标的受击伤害放大（千分比），0 表示无 */
  amplifyPct: number
  /** 表现层用：简短说明 */
  descr: string
}

/**
 * # ⚠️ 第 83 轮：这张表里有 **4 条反应只有反应伤害**，没有特殊效果
 *
 * 逐字段核过消费面（`resolveHit` + `BattleEngine.hitEnemy`）：
 *
 * | 字段         | 消费者                                | 状态 |
 * |--------------|---------------------------------------|------|
 * | baseCoef     | `damage.ts` 反应伤害                  | ✓    |
 * | attackWeightPct | `damage.ts` 攻方贡献比例             | ✓    |
 * | statusDurationMs | `damage.ts` → 引擎写 `frozenMs`/`stunnedMs` | ✓ |
 * | dispelShield | `damage.ts`                          | ✓    |
 * | amplifyPct   | `engine.ts`（**且以 `statusDurationMs > 0` 为前提**） | ✓ |
 * | aoeRadius    | **只有 `render/canvas.ts` 的屏幕震动** | ✗ **从未影响任何伤害** |
 *
 * 所以：
 *
 * - `steam_burst` 的「范围伤害」不存在（驱散护盾是真的）
 * - `overheat` 的「爆炸」不存在（眩晕是真的）
 * - `burn_cloud` 的「持续火区」不存在 —— 它整条都是空的
 * - `corrosion_spread` 的「层数传播」不存在
 * - `armor_break` 的「击退」与「削甲」都不存在
 *
 * 本轮把它们改成只描述**已实现**的效果，并把 `aoeRadius` 全部置 0
 * （字段保留：它已经在 `/config` 的公开 JSON 契约里，删字段是破坏性变更）。
 *
 * 「要不要给它们实现」是**产品决策**，已记入 README 已知边界第 19 条。
 */

export const REACTIONS: Record<ReactionKey, ReactionSpec> = {
  steam_burst: {
    key: 'steam_burst',
    name: '蒸汽爆发',
    baseCoef: 60,
    attackWeightPct: 300,
    statusDurationMs: 0,
    // ⚠️ 第 83 轮：120 → 0。**溅射从未被实现**。
    //
    // `resolveHit` 只消费 `baseCoef` / `attackWeightPct` / `statusDurationMs` /
    // `dispelShield`（+ 引擎侧读 `amplifyPct`）—— `aoeRadius` 一个字都没读。
    //
    // 而它此前唯一的消费者是 `render/canvas.ts` 的**屏幕震动**：
    // `if (spec.aoeRadius > 0) this.shake = …`
    // 也就是说游戏**在视觉上谎称发生了爆炸**，而实际什么都没发生。
    // 玩家从震动推断「炸到了」，于是这条反应看起来「有时不灵」。
    aoeRadius: 0,
    dispelShield: true,
    amplifyPct: 0,
    // ⚠️ 文案只描述**已实现**的效果（见 elements.ts 顶部的诚实标注）。
    descr: '驱散护盾并造成反应伤害',
  },
  overheat: {
    key: 'overheat',
    name: '过热',
    baseCoef: 55,
    attackWeightPct: 300,
    statusDurationMs: 1500,
    aoeRadius: 0, // ⚠️ 第 83 轮：90 → 0，「爆炸」从未被实现（理由同 steam_burst）
    dispelShield: false,
    amplifyPct: 0,
    descr: '眩晕 1.5 秒并造成反应伤害',
  },
  burn_cloud: {
    key: 'burn_cloud',
    name: '燃烧云',
    // ⚠️ 第 83 轮：这条反应的**整条文案都是未实现的**。
    //
    // `statusDurationMs = 0` → 没有燃烧状态；
    // `aoeRadius = 100` → 溅射从未被消费；
    // `dispelShield = false` / `amplifyPct = 0` → 没有别的。
    //
    // 也就是说 `burn_cloud` 现在的**唯一**实现是「一个系数较低的应伤害」
    // （baseCoef 40，是七条里第二低的）。
    //
    // 诚实的文案就该这么说。写成「生成持续火区」是在文档站上对一个
    // 不存在的机制做广告。
    //
    // 「要不要给它实现火区」是**产品决策**（伤害量？是否计分？
    // 是否需要服务端重算以免 I-6 失配？），不在这轮的猜范围内。
    // 已记入 README 已知边界。
    baseCoef: 40,
    attackWeightPct: 250,
    statusDurationMs: 0,
    aoeRadius: 0, // [mutation] 100
    dispelShield: false,
    amplifyPct: 0,
    descr: '造成反应伤害',
  },
  superconduct: {
    key: 'superconduct',
    name: '超导',
    baseCoef: 50,
    attackWeightPct: 250,
    statusDurationMs: 4000,
    aoeRadius: 0,
    dispelShield: false,
    amplifyPct: 600,
    descr: '受击伤害 +60%',
  },
  flash_freeze: {
    key: 'flash_freeze',
    name: '急速冻结',
    baseCoef: 45,
    attackWeightPct: 250,
    statusDurationMs: 2000,
    aoeRadius: 0,
    dispelShield: false,
    amplifyPct: 300,
    descr: '冻结并提高受击伤害',
  },
  corrosion_spread: {
    key: 'corrosion_spread',
    name: '腐蚀扩散',
    baseCoef: 35,
    attackWeightPct: 200,
    statusDurationMs: 0,
    aoeRadius: 0, // ⚠️ 第 83 轮：150 → 0，「传播」从未被实现
    dispelShield: false,
    amplifyPct: 0,
    descr: '造成反应伤害',
  },
  armor_break: {
    key: 'armor_break',
    name: '破甲击退',
    // ⚠️ 第 83 轮：「击退」与「削减护甲」**都没有实现**。
    //
    // `Enemy.knockback` 字段存在，但没有任何代码写它；
    // 护甲削减也没有消费者（`applyArmor` 只读 `def.armorPermille`，
    // 而 `armorPermille` 不由反应改写）。
    //
    // 所以这条反应现在**只有反应伤害**（baseCoef 30，七条里最低）。
    // 名字里的「破甲」「击退」也在承诺不存在的东西 —— 但改名要动
    // 两端契约与已存档的战报语义，**留给产品决策**，这轮只改文案。
    baseCoef: 30,
    attackWeightPct: 300,
    statusDurationMs: 0,
    aoeRadius: 0,
    dispelShield: false,
    amplifyPct: 0,
    descr: '造成反应伤害',
  },
}

/**
 * 反应查表：已附着元素 A + 新施加元素 B → 反应。
 *
 * 反应不可交换：谁先谁后决定结果。动能不能作为"已附着元素"
 * 触发新反应（表中 kinetic 的键不含空），它只能后手打出破甲。
 */
const REACTION_TABLE: Record<Element, Partial<Record<Element, ReactionKey>>> = {
  fire: {
    ice: 'steam_burst',
    lightning: 'overheat',
    corrosion: 'burn_cloud',
    kinetic: 'armor_break',
  },
  ice: {
    fire: 'steam_burst',
    lightning: 'superconduct',
    corrosion: 'flash_freeze',
    kinetic: 'armor_break',
  },
  lightning: {
    fire: 'overheat',
    ice: 'superconduct',
    corrosion: 'corrosion_spread',
    kinetic: 'armor_break',
  },
  corrosion: {
    fire: 'burn_cloud',
    ice: 'flash_freeze',
    lightning: 'corrosion_spread',
    kinetic: 'armor_break',
  },
  kinetic: {
    fire: 'armor_break',
    ice: 'armor_break',
    lightning: 'armor_break',
    corrosion: 'armor_break',
  },
}

/** 返回"已附着元素 + 新施加元素"触发的反应；无反应返回 null */
export function lookupReaction(
  existing: Element | '' | undefined,
  incoming: Element | '' | undefined,
): ReactionKey | null {
  if (!existing || !incoming || existing === incoming) return null
  const byIncoming = REACTION_TABLE[incoming]
  if (!byIncoming) return null
  return byIncoming[existing] ?? null
}

/** 该元素搭配能触发的全部反应（用于构筑评分） */
export function reachableReactions(elements: Set<Element>): Set<ReactionKey> {
  const out = new Set<ReactionKey>()
  for (const existing of ELEMENT_ORDER) {
    if (!elements.has(existing)) continue
    for (const incoming of ELEMENT_ORDER) {
      // incoming 也必须在搭配里：反应需要先后两种元素**都能打出**，
      // 只带一种元素的构筑触发不了任何反应。不限定 incoming 会让
      // 单元素也被算出反应，构筑评分虚高。
      if (!elements.has(incoming)) continue
      const k = lookupReaction(existing, incoming)
      if (k) out.add(k)
    }
  }
  return out
}

export function isElement(s: string): s is Element {
  return (ELEMENT_ORDER as readonly string[]).includes(s)
}
