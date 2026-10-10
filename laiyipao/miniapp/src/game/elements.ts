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
  /**
   * 削甲量（千分比，0 表示无）。armor_break 专用：
   * 触发后目标护甲在 `statusDurationMs` 内被削减本值。
   * 消费者：`engine.ts` 的 `hitEnemy`（构建 Defender 时折进有效护甲）。
   */
  armorShredPermille: number
  /**
   * 击退位移（定点 ×1000，0 表示无）。armor_break 专用：
   * 触发时目标一次性向右（远离防线）位移本值。
   * 消费者：`engine.ts` 的 `hitEnemy`（写入 `Enemy.knockback`）。
   */
  knockback: number
  /** 表现层用：简短说明 */
  descr: string
}

/**
 * # ⚠️ 这张表里有 **3 条反应只有反应伤害**，没有特殊效果
 *
 * 逐字段核过消费面（`resolveHit` + `BattleEngine.hitEnemy`）：
 *
 * | 字段               | 消费者                                | 状态 |
 * |-------------------|---------------------------------------|------|
 * | baseCoef          | `damage.ts` 反应伤害                  | ✓    |
 * | attackWeightPct   | `damage.ts` 攻方贡献比例              | ✓    |
 * | statusDurationMs  | `damage.ts` → 引擎写 `frozenMs`/`stunnedMs`/`armorShredMs` | ✓ |
 * | dispelShield      | `damage.ts`                          | ✓    |
 * | amplifyPct        | `engine.ts`（**且以 `statusDurationMs > 0` 为前提**） | ✓ |
 * | armorShredPermille| `engine.ts`（削甲，armor_break）      | ✓    |
 * | knockback         | `engine.ts`（击退位移，armor_break）  | ✓    |
 * | aoeRadius         | **只有 `render/canvas.ts` 的屏幕震动** | ✗ **从未影响任何伤害** |
 *
 * 所以：
 *
 * - `steam_burst` 的「范围伤害」不存在（驱散护盾是真的）
 * - `overheat` 的「爆炸」不存在（眩晕是真的）
 * - `burn_cloud` 的「持续火区」不存在 —— 它整条都是空的
 * - `corrosion_spread` 的「层数传播」不存在
 * - `armor_break` 的「击退」与「削甲」**已实现**（armorShredPermille / knockback，
 *   见下方该条注释），不再是空壳
 *
 * 第 83 轮把未实现项的文案改成只描述**已实现**的效果，并把 `aoeRadius` 全部置 0
 * （字段保留：它已经在 `/config` 的公开 JSON 契约里，删字段是破坏性变更）。
 *
 * 「要不要给 steam_burst / overheat / burn_cloud / corrosion_spread 实现
 * 溅射 / 爆炸 / 火区 / 传播」仍是**产品决策**，已记入 README 已知边界第 19 条。
 */

export const REACTIONS: Record<ReactionKey, ReactionSpec> = {
  steam_burst: {
    key: 'steam_burst',
    name: '蒸汽爆发',
    baseCoef: 60,
    attackWeightPct: 300,
    statusDurationMs: 0,
    // ⚠️ 第 83 轮：120 → 0。**溅射从未被实现** —— 渲染层的屏幕震动谎称发生了
    // 爆炸，实际 0 半径，气冷逻辑上什么都没发生。
    //
    // 2026-10-10 实现 steam_burst 范围伤害：aoeRadius 恢复为 120（像素）。
    // 引擎 `hitEnemy` 在 steam_burst 反应触发后、以命中敌人为圆心、半径
    // aoeRadius 搜索邻居，造成 reactionDmg >> 2（25%）的二次伤害。
    // 口径与 `reaction_specs.json`（服务端 canonical）逐行一致，
    // 由 `reaction_contract.test.ts` 双向锁定。
    //
    // I-6 安全性：服务端 replay_hash 只封 S 段（build_snapshot），
    // 不重算事件流（battle_collections.go：replay_hash 仅封长 ≤64），
    // 所以溅射事件只需客户端局内哈希稳定 —— `aoeRadius` 改动不影响
    // replay_skills 段，因而不影响 I-6 的 S-段校验。
    // `aoeRadius` 此前唯一的消费者是 `render/canvas.ts` 屏幕震动，
    // 它原本谎称「120 像素范围爆炸」却不造成任何伤害。
    aoeRadius: 120,
    dispelShield: true,
    amplifyPct: 0,
    armorShredPermille: 0,
    knockback: 0,
    // 文案承诺已实现的效果（第 83 轮诚实修订），加上本轮接上的范围伤害。
    // 与 server/testdata/reaction_specs.json 的 descr 逐字一致（contract test 钉住）。
    descr: '驱散护盾并造成反应伤害（触发时以自身为圆心，半径 120 像素内的敌人受到 25% 范围伤害）',
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
    armorShredPermille: 0,
    knockback: 0,
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
    armorShredPermille: 0,
    knockback: 0,
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
    armorShredPermille: 0,
    knockback: 0,
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
    armorShredPermille: 0,
    knockback: 0,
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
    armorShredPermille: 0,
    knockback: 0,
    descr: '造成反应伤害',
  },
  armor_break: {
    key: 'armor_break',
    name: '破甲击退',
    // 第 83 轮：「击退」与「削减护甲」当时**都没有实现**，只有反应伤害，
    // 文案已改成诚实描述。
    //
    // 本轮（产品决策落地）：把「击退 + 削甲」真正实现出来。
    //
    //  1. **削甲**：`armorShredPermille = 150`（15%），持续 `statusDurationMs`
    //     = 4000ms。触发后 4 秒内，目标的有效护甲被削减 150‰，
    //     引擎在构建 Defender 时折进 `effectiveArmor`（见 engine.ts hitEnemy）。
    //  2. **击退**：`knockback = 4000`（定点 ×1000 = 4px），触发时一次性把
    //     目标向右（远离防线）推 4px，写入 `Enemy.knockback`，
    //     由 `stepEnemyMotion` 的击退分支消费并清零（一次性，非持续）。
    //
    // 这两个值都落在 0..合理区间：削甲 150‰ 相对敌人护甲（普通 0..150，
    // BOSS 200..300）是「显著但不破坏」的量级；击退 4px 相对敌人每 tick
    // 推进 2..4px 是「轻微后移」，不会把敌人推出命中范围。
    baseCoef: 30,
    attackWeightPct: 300,
    statusDurationMs: 4000,
    aoeRadius: 0,
    dispelShield: false,
    amplifyPct: 0,
    armorShredPermille: 150,
    knockback: 4000,
    descr: '击退目标并削减其护甲 4 秒（150‰），同时造成反应伤害',
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
