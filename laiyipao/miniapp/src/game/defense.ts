/**
 * 防线值守的客户端模拟（I-5 的客户端闭环）。
 *
 * I-5 的设计取舍：**服务端不跑战斗引擎**。
 * 挑战者在自己的设备上用与 P1 完全相同的引擎模拟攻方，
 * 攻方由对方的防线快照驱动。跑完上报结果，服务端只校验：
 *   1. 挑战次数未超限（每日 3 次）
 *   2. 对方 24h 护盾是否过期
 *   3. 资源转移的守恒（扣了对方就必须加自己）
 *
 * 这样取舍的好处是不必维护两套引擎保持同步；
 * 代价是**服务端无法独立验证胜负** —— 所以后台的「防线值守」页
 * 对接近 100% / 0% 的异常胜率做告警（见 DefensesView.vue）。
 *
 * 快照由服务端在下发 candidates 时一并给出（见 service/progression.go），
 * 拿不到快照就无法模拟，只能盲报 —— 那样等于没有防线机制。
 */

import { BattleEngine } from './engine'
import type { ScoreRules } from './score'
import { type Attacker } from './damage'
import type { EquippedSkill } from './heatmap'
import type { Element } from './elements'
import type { EnemyDef, GeneratedLevel, SkillDef } from './types'
import { fnv1a64, hex16 } from './lcg'

/** 防线快照（server DefenseView.Snapshot） */
export interface DefenseSnapshot {
  /** 技能 id 列表 */
  skills: number[]
  equipment: number[]
  mastery_nodes: number[]
  /** 工程装置：slow_belt / block_wall / tesla_grid */
  works: string[]
  /** 已有元素的并集（服务端算好，省得客户端重算） */
  elements: string[]
  mastery: number[]
  rating: {
    element_coverage?: number
    reaction_coverage?: number
    mastery_done?: number
    equipment_synergy?: number
    total?: number
  }
}

export interface DefenseView {
  id: number
  owner_id: number
  owner_name: string
  name: string
  power: number
  element_coverage: number
  mastery_done: number
  wins: number
  losses: number
  shielded_until?: string
  snapshot?: DefenseSnapshot
  snapshot_hash?: string
  can_challenge: boolean
  challenge_blocked?: string
}

export interface ChallengeDeps {
  /** 我方（攻方）的技能，挑战时由我的构筑驱动 */
  myEquipped: EquippedSkill[]
  myAttacker: Attacker
  /** 引擎需要的关卡与内容表 —— 挑战统一用第 1 关作为标准战场 */
  level: GeneratedLevel
  enemies: Map<number, EnemyDef>
  skills: Map<number, SkillDef>
  /**
   * 分数规则，从服务端 /config 的 `score_rules` 转来（`scoreRulesFromServer`）。
   * 缺省时引擎用 `DEFAULT_SCORE_RULES`。
   *
   * ⚠️ 防线挑战的分数必须与结算**同口径**，否则玩家看到的
   * 「我打赢了防线」与服务端算出来的对不上。
   */
  scoreRules?: ScoreRules
  /**
   * 种子覆盖（可选）。
   *
   * 默认按「对方防线 id + 当前分钟」派生，这样同一对组合在一分钟内
   * 结果固定、跨分钟变化，避免反复挑战得到完全相同的战局。
   *
   * ⚠️ 但这会让**测试依赖运行时刻** —— 跨分钟跑就会得到不同战局，
   * 断言变成 flaky。因此种子必须可注入，测试一律显式传固定值。
   */
  seedOverride?: bigint
}

export interface ChallengeOutcome {
  won: boolean
  /** 上报给服务端 ChallengeInput 的字段 */
  report: {
    seed: string
    won: boolean
    duration_ms: number
    hp_left_pct: number
    replay_hash: string
  }
  /** 本地统计（用于展示） */
  stats: {
    kills: number
    leaked: number
    waves: number
    score: number
    durationMs: number
    hpLeftPct: number
  }
  error?: string
}

/** 单局 tick 上限，与 replay.ts 保持一致 */
const MAX_TICKS = 60 * 60 * 10

function runToEnd(engine: BattleEngine, maxTicks: number): number {
  let ticks = 0
  // 循环的 false 退出分支与底部 return 均**构造性不可达**：
  // maxTicks 唯一取值 MAX_TICKS(36000)，而 BattleEngine 的停滞兜底 checkStalemate
  // 在 tick 到 MAX_BATTLE_TICKS(30000) 时就强制 finish(lost)，引擎最迟 ~30000 步终局，
  // 循环必从内部 won/lost 提前 return —— `ticks<maxTicks` 变 false 与走到底部 return
  // 永远轮不到（36000>30000 的安全余量）。这是"引擎万一不自行终止"的安全阀，无公开 API 可达。
  while (ticks < maxTicks) {
    const phase: string = engine.phase
    if (phase === 'won' || phase === 'lost') return ticks
    // 防线值守不允许中途取牌（玩家不在场），统一跳过
    if (phase === 'card_select') engine.skipCards()
    engine.step()
    ticks++
    /* v8 ignore next -- 见上：循环 false 退出分支不可达（36000>30000） */
  }
  /* v8 ignore next 2 -- 见上：耗尽 maxTicks 的收尾 return 与函数末返回点不可达 */
  return ticks
}

/**
 * 工程装置对攻方的增益。
 *
 * 装置是防线快照的一部分，本质是攻方的"预设强化"。
 * 这里把装置折算成攻方属性加成 —— 与 P1 战斗里属性卡的机制同源，
 * 保证两边走的是同一套数值路径。
 */
function applyWorks(att: Attacker, works: string[]): Attacker {
  const a = { ...att }
  // `?? []` 的空数组回退分支**不可达**：applyWorks 是模块私有，唯一调用点
  // runChallenge 传入的已是 `view.snapshot?.works ?? []`（在调用前就兜过底），
  // 到这里 works 恒为数组。保留这层守卫是防御 future 调用点，但当前无法触达，故豁免其分支。
  /* v8 ignore next -- 见上：works 恒为数组，`?? []` 回退分支不可达 */
  for (const code of works ?? []) {
    switch (code) {
      case 'slow_belt':
        // 减速带：削弱推进速度的等效手段 = 提高我方元素系数
        a.elementCoefPermille += 100n
        break
      case 'block_wall':
        // 壁垒：等效提高防线护甲，表现为提高元素层数上限
        a.elementCap += 1n
        break
      case 'tesla_grid':
        // 特斯拉栅格：提高反应伤害倍率
        a.reactionMultPermille += 200n
        break
    }
  }
  return a
}

/**
 * 校验对方快照能否被还原。
 *
 * ⚠️ 这一步不能省：拿不到完整快照就直接跑，得到的胜负毫无意义，
 * 却会被服务端记账（影响对方战绩）—— 那是在污染别人的数据。
 */
export function validateSnapshot(view: DefenseView): { ok: true } | { ok: false; reason: string } {
  const snap = view.snapshot
  if (!snap) return { ok: false, reason: '未获取到对方防线快照，无法模拟挑战' }
  if (!Array.isArray(snap.skills) || snap.skills.length === 0) {
    return { ok: false, reason: '对方快照里没有任何技能，无法模拟' }
  }
  if (!view.snapshot_hash) {
    return { ok: false, reason: '对方快照缺少哈希，无法确认挑战的是同一份构筑' }
  }
  if (isShieldActive(view.shielded_until, Date.now())) {
    return { ok: false, reason: '对方护盾尚未过期' }
  }
  return { ok: true }
}

/**
 * 发起一次防线挑战。
 *
 * 注意语义：我是攻方，挑战对方构筑。胜负由"我把对方防线打穿"决定 ——
 * 与 P1 的规则一致（守住 = 胜利），只是攻守互换。
 */
export function runChallenge(view: DefenseView, deps: ChallengeDeps): ChallengeOutcome {
  const fail = (reason: string): ChallengeOutcome => ({
    won: false,
    report: { seed: '0', won: false, duration_ms: 0, hp_left_pct: 100, replay_hash: '0000000000000000' },
    stats: { kills: 0, leaked: 0, waves: 0, score: 0, durationMs: 0, hpLeftPct: 100 },
    error: reason,
  })

  const v = validateSnapshot(view)
  if (!v.ok) return fail(v.reason)

  if (deps.myEquipped.length === 0) return fail('未装备任何技能，请先配置出战技能')

  // 种子：默认按「对方防线 id + 当前分钟」派生（见 ChallengeDeps.seedOverride 的说明）。
  // 测试必须显式注入固定种子，否则断言会随运行时刻漂移。
  const seedBig =
    deps.seedOverride !== undefined
      ? deps.seedOverride & ((1n << 63n) - 1n)
      : (BigInt(view.id) * 1000003n + BigInt(Math.floor(Date.now() / 60000))) &
        ((1n << 63n) - 1n)

  const attacker = applyWorks(deps.myAttacker, view.snapshot?.works ?? [])

  const engine = new BattleEngine({
    level: deps.level,
    enemies: deps.enemies,
    skills: deps.skills,
    equipped: deps.myEquipped,
    attacker,
    seed: seedBig,
    // 分数规则从服务端配置来，而不是用客户端默认值 ——
    // 防线挑战的分数要和结算同口径（见 score.ts 的注释）。
    scoreRules: deps.scoreRules,
  })
  engine.start()
  runToEnd(engine, MAX_TICKS)

  const hpMax = Number(engine.baseHpMax)
  const hpLeftPct = hpMax > 0 ? Math.round((Number(engine.baseHp) / hpMax) * 100) : 0
  const won = engine.phase === 'won'
  const hash = engine.replayHash()

  return {
    won,
    report: {
      // ⚠️ 字符串下发：int64 超过 2^53 后 JSON number 在 JS 侧解析即失精，
      // 服务端会拿它和本地比较。服务端 ChallengeInput.Seed 是 int64。
      seed: seedBig.toString(),
      won,
      duration_ms: Math.round(engine.elapsedMs),
      hp_left_pct: hpLeftPct,
      replay_hash: hash,
    },
    stats: {
      kills: engine.kills,
      leaked: engine.leaked,
      waves: engine.waveIndex + 1,
      score: engine.score,
      durationMs: Math.round(engine.elapsedMs),
      hpLeftPct,
    },
  }
}

/** 快照摘要哈希（本地校验用，与服务端 SnapshotHash 口径不同的轻量版） */
export function snapshotDigest(works: string[], skills: number[]): string {
  const s = [...skills].sort((a, b) => a - b).join(',') + '|' + [...(works ?? [])].sort().join(',')
  return hex16(fnv1a64(s))
}

/** 把快照里的元素列表转成 Element 数组（用于展示） */
export function snapshotElements(view: DefenseView): Element[] {
  const valid: Element[] = ['fire', 'ice', 'lightning', 'corrosion', 'kinetic']
  return (view.snapshot?.elements ?? []).filter((e): e is Element =>
    valid.includes(e as Element),
  )
}

/**
 * 护盾是否**仍在生效**（按时间判断，不是按「字段存在」判断）。
 *
 * ⚠️ 第 134 轮：`shielded_until` 是服务端下发的 RFC3339 时间戳。
 * 「字段非空」≠「护盾未过期」—— 护盾 24h 后 `shielded_until` 仍在库里
 * （是过去的时间），但护盾其实已失效。旧 me.vue 用 `!!shielded_until`
 * 判存在，于是过期护盾在「我的页」仍显示成「护盾开启」。
 * 现统一走时间比较，nowMs 显式传入便于测试（生产传 Date.now()）。
 */
export function isShieldActive(until: string | undefined, nowMs: number): boolean {
  if (!until) return false
  const t = Date.parse(until)
  if (Number.isNaN(t)) return false
  return t > nowMs
}
