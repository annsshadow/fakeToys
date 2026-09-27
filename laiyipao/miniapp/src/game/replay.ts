/**
 * 回放重放（I-6 的客户端闭环）。
 *
 * 这个文件回答一个问题：**「这局分数是真的吗？」**
 *
 * 机制：
 *   战斗完全确定性 —— 给定「关卡 + 随机种子 + 玩家构筑」，引擎产出
 *   完全相同的事件序列，因而算出完全相同的回放哈希。
 *   任何人拿到 battle_id 就能向服务端换回这三样东西，本地重跑一遍，
 *   算出的哈希与服务端记录的不一致，就证明上报的分数被篡改。
 *
 * ⚠️ 这不是「服务端验算」。服务端不跑战斗引擎（见 I-5 的取舍说明），
 * 验证权在客户端。这样做的好处是：作弊者必须伪造**整条事件序列**，
 * 而不能只改一个数字。代价是：若客户端引擎与服务端公式漂移，
 * 正常对局会被误判为伪造 —— 所以两端公式一致性由
 * server/testdata/formula_vectors.json 锁死（见 consistency.test.ts）。
 *
 * ⚠️ 构筑缺失是致命的。回放哈希依赖「用哪些技能、什么养成属性」，
 * 只给关卡和种子算不出正确哈希。服务端必须在 build_snapshot 里
 * 下发真实构筑（见 service/game.go 的 SettleBattle）。
 */

import { BattleEngine, TICK_MS } from './engine'
import { type Attacker } from './damage'
import type { EquippedSkill } from './heatmap'
import type { Element } from './elements'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'

/** 服务端 /battle/{id}/replay 的响应。 */
export interface ReplayInfo {
  battle_id: number
  level_id: number
  /** 字符串形态的 int64 —— 服务端刻意不用 number，见 economy.go 的注释 */
  seed: string
  level: GeneratedLevel & { seed_str?: string }
  expected_hash: string
  created_at: string
  /**
   * 结算时冻结的构筑快照。结构见 build_snapshot：
   *   { skills: { "<id>": {id,name,element,kind,level,slot,...} },
   *     elements: string[], equipment: ..., mastery_nodes: ...,
   *     attacker: {...}, settle_input: {...} }
   *
   * 注意：玩家侧响应里 settle_input 已被服务端剥离（防伪造战报蓝本），
   * 它只存在于 battle_records 表中。
   */
  build: BuildSnapshot
  /**
   * 每波选中的手牌索引（-1 = 跳过）。
   *
   * ⚠️ 选牌会改变后续战斗，缺它就复现不出原局。
   * 空数组表示旧记录没有该字段 —— 此时无法复现，只能看哈希是否巧合相符。
   */
  card_picks?: number[]
}

/** 构筑快照里单个技能的形态（对齐 loadSkillsAndSlots） */
export interface SnapshotSkill {
  id: number
  name: string
  family: string
  element: string
  kind: string
  level: number
  /** -1 表示未放入槽位 */
  slot: number
}

/** 攻方属性（服务端权威下发，见 server/internal/service/attacker.go） */
export interface SnapshotAttacker {
  attack: number
  crit_permille: number
  crit_multiplier_permille: number
  reaction_mult_permille: number
  element_cap: number
  reaction_tier: number
  element_coef_permille: number
  /**
   * 下面三项由本轮补上（专精 heat_cap/armor/mechanic + 全部装备与宝石）。
   *
   * ⚠️ 标成**可选**是有意的：老战报的 build_snapshot 里没有它们。
   * 可选 + 读取处 `?? 0` 才能让老战报照常重放（缺失等价于"无加成"）。
   *
   * 标成必填会让"读老战报"变成类型错误；标成必填但读处不兜默认值，
   * 则重放时得到 NaN/0 混用 —— 两种都更糟。
   */
  heat_cap_permille?: number
  armor_permille?: number
  mechanic_permille?: number
}

export interface BuildSnapshot {
  skills: Record<string, SnapshotSkill>
  /**
   * 已解锁技能覆盖的**元素名**（fire/ice/lightning/...），不是技能 id。
   *
   * ⚠️ 同名的 `skill_ids` 是服务端的误命名（它装的就是这个元素名数组），
   * 而 `/me/loadout` 响应里的 `skill_ids` 是真的技能 id 数组。
   * 重放不需要这个字段（技能来自 `skills`），它只用于展示元素覆盖。
   * 读的时候优先用本字段，老战报只有 skill_ids 时回退。
   */
  elements?: string[]
  /** @deprecated 误命名，实际是元素名数组。保留仅为兼容历史战报。 */
  skill_ids?: string[]
  equipment: unknown
  mastery_nodes: unknown
  /**
   * 攻方属性。**重放必须用它，不能本地推算** ——
   * 缺了它算出的哈希与服务端记录必然不同（I-6 失效）。
   */
  attacker?: SnapshotAttacker
  settle_input?: Record<string, unknown>
}

export interface ReplayDeps {
  level: GeneratedLevel
  enemies: Map<number, EnemyDef>
  skills: Map<number, SkillDef>
}

/** 重放结果 */
export interface ReplayOutcome {
  /** 本地重放算出的哈希 */
  computedHash: string
  /** 与服务端记录的是否一致 */
  matched: boolean
  /** 本地重放的统计（用于人工判断"差在哪"） */
  stats: {
    kills: number
    leaked: number
    score: number
    waves: number
    durationMs: number
  }
  /** 构筑缺失等前置问题 */
  error?: string
}

/**
 * 把构筑快照映射成引擎需要的 EquippedSkill 列表。
 *
 * ⚠️ 槽位顺序必须与首次对战时**完全一致**，否则哈希必然不同。
 * 这里按 snapshot 里的 slot 升序排，slot = -1 的技能视为未装备（跳过）。
 */
export function equippedFromSnapshot(build: BuildSnapshot, skills: Map<number, SkillDef>): EquippedSkill[] {
  const list = Object.values(build?.skills ?? {})
    .filter((s) => s && typeof s.id === 'number')
    .sort((a, b) => a.slot - b.slot || a.id - b.id)

  const out: EquippedSkill[] = []
  for (const s of list) {
    if (s.slot < 0) continue // 未放入槽位
    const def = skills.get(s.id)
    if (!def) continue // 技能已被下架，跳过而不是报错
    out.push({
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
      slot: s.slot,
      cooldownRemaining: 0,
    })
  }
  return out
}

/**
 * 由构筑快照还原攻方属性。
 *
 * ⚠️ 绝不能本地推算。回放哈希由「关卡 + 种子 + 攻方属性 + 技能」共同决定，
 * 两端各算一套的话，哈希永远对不上。
 *
 * 缺少该字段时明确返回 error，而不是给出一个必然不匹配的哈希
 * 然后假装"验过了" —— 后者会让 I-6 变成制造假警报的噪音源。
 */
function attackerFromSnapshot(build: BuildSnapshot): { att: Attacker } | { error: string } {
  const a = build?.attacker
  if (!a || typeof a.attack !== 'number') {
    return {
      error:
        '构筑快照不含 attacker 字段，无法还原攻方属性。' +
        '（服务端需在 build_snapshot 里下发攻击力/元素系数等系数，见 service/attacker.go）',
    }
  }
  return {
    att: {
      attack: BigInt(a.attack),
      critPermille: BigInt(a.crit_permille ?? 50),
      critMultiplierPermille: BigInt(a.crit_multiplier_permille ?? 1500),
      reactionMultPermille: BigInt(a.reaction_mult_permille ?? 1000),
      elementCap: BigInt(a.element_cap ?? 3),
      reactionTier: BigInt(a.reaction_tier ?? 1),
      elementCoefPermille: BigInt(a.element_coef_permille ?? 1000),
      // ⚠️ 这三项缺省 0，是**有意的**：老战报的 build_snapshot 里没有它们
      // （那时它们根本不存在），而"缺失 = 0"正好等价于"没有装备/宝石/专精"。
      // 填任何非 0 的缺省都会让老战报重放出的哈希与当年记录的不符。
      //
      // 反过来，服务端不下发这三项时玩家会**丢掉全部装备与专精加成**
      // 却毫无提示 —— 所以服务端侧由 TestAttackerViewCoversEveryField
      // 守死"Attacker 每个字段都必须出现在 AttackerView 里"。
      heatCapPermille: BigInt(a.heat_cap_permille ?? 0),
      armorPermille: BigInt(a.armor_permille ?? 0),
      mechanicPermille: BigInt(a.mechanic_permille ?? 0),
    },
  }
}

/** 单局推进上限，防止异常关卡把浏览器卡死 */
const MAX_TICKS_PER_REPLAY = 60 * 60 * 10 // 10 分钟游戏时长

/**
 * 把引擎推到 won/lost 或 tick 上限。
 *
 * 抽成独立函数有两个原因：
 *  1. 打断 TS 的字面量窄化 —— 在同一个函数里读 engine.phase 多次会被
 *     控制流分析按上一次的值定型，导致「永假比较」报错。
 *  2. 选牌策略集中在一处。
 *
 * 选牌由引擎自己按脚本处理（见 engine.setReplayScript），
 * 本函数只在**没有脚本**时兜底跳过整波 —— 那对应旧记录
 * （card_picks 为空），此时无法复现原局，只能跑完看哈希是否巧合相符。
 */
function runToEnd(engine: BattleEngine, maxTicks: number): number {
  let ticks = 0
  while (ticks < maxTicks) {
    const phase: string = engine.phase
    if (phase === 'won' || phase === 'lost') return ticks
    // ⚠️ 脚本模式下**不能**外部 skipCards —— 引擎会在 step() 内部
    // 按脚本自动决策。外部抢先丢掉手牌会让脚本彻底失效。
    // 只有无脚本（旧记录缺 card_picks）时才兜底跳过整波。
    if (phase === 'card_select' && !engine.hasReplayScript()) engine.skipCards()
    engine.step()
    ticks++
  }
  return ticks
}

/**
 * 重放一局并比对哈希。
 *
 * 全程同步、无渲染、无定时器 —— 纯状态机推演。
 * 10 分钟的战斗约 12000 tick，毫秒级完成，因此可以同步跑。
 */
export function replay(info: ReplayInfo, deps: ReplayDeps): ReplayOutcome {
  const empty = (err: string): ReplayOutcome => ({
    computedHash: '',
    matched: false,
    stats: { kills: 0, leaked: 0, score: 0, waves: 0, durationMs: 0 },
    error: err,
  })

  if (!info?.level) return empty('缺少关卡数据，无法重放')
  if (!info.build?.skills) {
    return empty('缺少构筑快照（build_snapshot）。回放哈希依赖构筑，缺它算出的哈希必然不符。')
  }

  const equipped = equippedFromSnapshot(info.build, deps.skills)
  if (equipped.length === 0) {
    return empty('构筑快照里没有任何已装备技能（全部 slot < 0），无法重放')
  }

  const derived = attackerFromSnapshot(info.build)
  if ('error' in derived) return empty(derived.error)
  const att = derived.att

  let seed: bigint
  try {
    seed = BigInt(info.seed)
  } catch {
    return empty(`种子格式非法：${info.seed}`)
  }

  const engine = new BattleEngine({
    level: info.level,
    enemies: deps.enemies,
    skills: deps.skills,
    equipped,
    attacker: att,
    seed,
  })
  // 注入选牌脚本：原局玩家点了哪张，重放就点哪张。
  // 缺脚本（空或 undefined）时不注入，引擎走交互模式，
  // 由 runToEnd 兜底跳过整波 —— 对应旧记录无法复现选牌的情形。
  if (Array.isArray(info.card_picks) && info.card_picks.length > 0) {
    engine.setReplayScript(info.card_picks)
  }
  engine.start()
  runToEnd(engine, MAX_TICKS_PER_REPLAY)

  const computed = engine.replayHash()
  return {
    computedHash: computed,
    matched: computed === info.expected_hash,
    stats: {
      kills: engine.kills,
      leaked: engine.leaked,
      score: engine.score,
      waves: engine.waveIndex + 1,
      durationMs: Math.round(engine.elapsedMs),
    },
  }
}

/** 供 UI 显示：把 16 位十六进制哈希按 4 位一组分隔，便于肉眼比对 */
export function prettyHash(hash: string): string {
  if (!hash) return ''
  return (hash.match(/.{1,4}/g) ?? []).join(' ').toUpperCase()
}

/** 供 UI 显示：毫秒转「m:ss」 */
export function prettyDuration(ms: number): string {
  const total = Math.floor(ms / 1000)
  const m = Math.floor(total / 60)
  const s = total % 60
  return `${m}:${String(s).padStart(2, '0')}`
}

export { TICK_MS }
