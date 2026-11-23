// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R9 战斗反馈的纯归因层。
 *
 * 这些数据只用于展示，不参与引擎推进、不进 record/replay_hash。
 * 所有数值必须来自真实战斗事件或服务端权威 card_picks；
 * 缺字段就写诚实空态，绝不推算或编造。
 */
import { ELEMENT_NAME, ELEMENT_ORDER, REACTIONS, REACTION_ORDER, type Element, type ReactionKey } from "./elements"
import { HEAT_MAX } from "./heatmap"
import { REACTION_PAIRS } from "../utils/reactionPairs"

export interface CardPickSummary {
  taken: number
  discarded: number
  skipped: number
  total: number
}

// 与 engine.ts 的 PICK_CODE_BASE / encodeDiscardCode 枚举表逐位一致，并由 feedback.test.ts
// 对全部 [-22..-3] 编码做跨函数比对。这里不 import engine，避免页面测试对引擎 mock 的耦合。
const PICK_CODE_BASE = 3
const PICK_CODE_MAX = 19

function popcount3(mask: number): number {
  return (mask & 1) + ((mask >> 1) & 1) + ((mask >> 2) & 1)
}

function decodePick(pick: number): { mask: number; take: number } | null {
  let code = -pick - PICK_CODE_BASE
  if (!Number.isInteger(code) || code < 0 || code > PICK_CODE_MAX) return null
  for (let mask = 0; mask < 8; mask++) {
    const n = 4 - popcount3(mask)
    if (code < n) return { mask, take: code === n - 1 ? -1 : code }
    code -= n
  }
  return null
}

/** 解码权威 card_picks：非负=取原手牌，-1=跳过，其余为 encodeDiscardCode 的负值。 */
export function summarizeCardPicks(picks: readonly number[]): CardPickSummary {
  const s: CardPickSummary = { taken: 0, discarded: 0, skipped: 0, total: picks.length }
  for (const pick of picks) {
    if (!Number.isInteger(pick)) continue
    if (pick >= 0) { s.taken++; continue }
    if (pick === -1) { s.skipped++; continue }
    const decoded = decodePick(pick)
    if (!decoded) continue
    if (decoded.take >= 0) s.taken++
    s.discarded += popcount3(decoded.mask)
  }
  return s
}

export interface DamageShare {
  reactionDamage: number
  totalDamage: number
  ratioPct: number | null
}

/**
 * hit.damage 已经是本次命中的总伤害（含反应部分，见 engine.hitEnemy），
 * reaction.damage 是其中反应侧的金额；两者不能相加当分母，否则重复计数。
 */
export function damageShareFromEvents(events: readonly ({ type?: string; damage?: bigint | number } & Record<string, unknown>)[]): DamageShare {
  let reactionDamage = 0
  let totalDamage = 0
  for (const e of events) {
    const amount = Number(e.damage ?? 0)
    if (!Number.isFinite(amount) || amount <= 0) continue
    if (e.type === "reaction") reactionDamage += amount
    else if (e.type === "hit") totalDamage += amount
  }
  return {
    reactionDamage,
    totalDamage,
    ratioPct: totalDamage > 0 ? Math.round(reactionDamage * 1000 / totalDamage) / 10 : null,
  }
}

export interface ReactionCue {
  name: string
  pair: string
  cue: string
}

/** 由引擎事件中的真实元素与反应键生成一句可归因提示（AI 标识由模板负责）。 */
export function reactionCue(
  reaction: ReactionKey,
  existing: Element | "" | null | undefined,
  incoming: Element | "" | null | undefined,
  damage: bigint | number = 0n,
): ReactionCue {
  const spec = REACTIONS[reaction]
  const name = spec?.name ?? String(reaction)
  const known = Boolean(existing && incoming)
  const pair = known ? `${ELEMENT_NAME[existing as Element]}+${ELEMENT_NAME[incoming as Element]}` : "元素组合未上报"
  const scenario = !spec ? "反应详情未上报" : reactionMechanism(reaction)
  const conclusion = known ? `${pair}触发${name}，${scenario}` : `元素组合未上报，${name}详情无法归因`
  return {
    name,
    pair,
    cue: `${conclusion}；依据 reaction=${String(reaction)}/damage=${Number(damage)}；点卡凑同元素`,
  }
}

/** 反应机制短语与图鉴同源：传播半径不是伤害溅射，必须分开说。 */
function reactionMechanism(reaction: ReactionKey): string {
  const spec = REACTIONS[reaction]
  if (!spec) return "反应详情未上报"
  const parts: string[] = []
  if (spec.dispelShield) parts.push("驱散护盾")
  if (spec.armorShredPermille > 0) parts.push(`削甲 ${spec.armorShredPermille}‰`)
  if (spec.knockback > 0) parts.push("击退")
  if (spec.spreadStacks > 0) parts.push(`传播 ${spec.spreadStacks} 层腐蚀（半径 ${spec.aoeRadius}，不触发新反应）`)
  if (spec.amplifyPct > 0) parts.push(`受击伤害 +${spec.amplifyPct / 10}%`)
  if (spec.statusDurationMs > 0) {
    const sec = `${(spec.statusDurationMs / 1000).toFixed(1)}s`
    const status = spec.descr.match(/^(眩晕|冻结)/)?.[1]
    parts.push(status ? `${status} ${sec}` : `持续 ${sec}`)
  }
  if (spec.aoeRadius > 0 && spec.spreadStacks === 0) parts.push(`密集波 25% 溅射（半径 ${spec.aoeRadius}）`)
  return parts.join("、") || spec.descr
}

export interface ReactionLearningInput {
  reactionsUsed?: Record<string, number> | null
  elementsUsed?: Record<string, number> | null
}

/** 战报反应建议：只消费权威 reactions_used/elements_used，缺数据不编造。 */
export function reactionLearningAdvice(input: ReactionLearningInput): string {
  const used = Object.entries(input.reactionsUsed ?? {})
    .filter(([key, count]) => Number.isFinite(Number(count)) && Number(count) > 0 && Boolean(REACTIONS[key as ReactionKey]))
    .sort((a, b) => Number(b[1]) - Number(a[1]))
  if (!used.length) return "本局未上报反应使用记录，不给元素建议。"
  const [topKey, topCount] = used[0]!
  const topSpec = REACTIONS[topKey as ReactionKey]
  const topPair = REACTION_PAIRS[topKey as ReactionKey]?.[0]
  const topText = topPair
    ? `${ELEMENT_NAME[topPair[0]]}+${ELEMENT_NAME[topPair[1]]}触发${topSpec.name} ×${Number(topCount)}`
    : `${topSpec.name} ×${Number(topCount)}（元素组合未上报）`
  const usedElements = new Set<Element>(
    Object.entries(input.elementsUsed ?? {})
      .filter(([key, count]) => Number(count) > 0 && ELEMENT_ORDER.includes(key as Element))
      .map(([key]) => key as Element),
  )
  const usedKeys = new Set(used.map(([key]) => key))
  let next = "当前元素已覆盖本局已触发反应，下一局可补另一种元素扩大覆盖。"
  for (const key of REACTION_ORDER) {
    if (usedKeys.has(key)) continue
    const pair = (REACTION_PAIRS[key] ?? []).find(([a, b]) => usedElements.has(a) !== usedElements.has(b))
    if (!pair) continue
    const keep = usedElements.has(pair[0]) ? pair[0] : pair[1]
    const add = usedElements.has(pair[0]) ? pair[1] : pair[0]
    next = `下一局补 ${ELEMENT_NAME[add]}（已有 ${ELEMENT_NAME[keep]}），可解锁${REACTIONS[key].name}（${ELEMENT_NAME[pair[0]]}+${ELEMENT_NAME[pair[1]]}）`
    break
  }
  return `反应复盘：${topText}；${next}；依据：reactions_used/elements_used。`
}

export interface DefeatReviewInput {
  wave: number | null
  totalWaves?: number | null
  kills: number | null
  leaked: number | null
  reactions: number | null
  heatMax?: number | null
  cardPicks?: readonly number[] | null
  elementsUsed?: Record<string, number> | null
}

function finite(value: unknown): value is number {
  return typeof value === 'number' && Number.isFinite(value)
}

/**
 * R12 失败复盘纯归因层：只接受真实战报/结算字段，缺字段就留空态或明确「未上报」。
 * 不参与引擎推进，不进 replay_hash。目的是把「为什么输」和「下一局改什么」
 * 从一堆分散数字翻译成一条可执行的结论，而不是空泛鼓励。
 */
export function defeatReview(input: DefeatReviewInput): string {
  const wave = finite(input.wave) ? input.wave : null
  const totalWaves = finite(input.totalWaves) && input.totalWaves! > 0 ? input.totalWaves : null
  const kills = finite(input.kills) ? input.kills : null
  const leaked = finite(input.leaked) ? input.leaked : null
  const reactions = finite(input.reactions) ? input.reactions : null
  if (wave === null && kills === null && leaked === null && reactions === null) {
    return '本局缺少失败复盘所需的波次/击杀/漏怪/反应数据，不做估算。'
  }

  const basis: string[] = []
  const waveText = wave !== null
    ? `第 ${wave}${totalWaves !== null ? `/${totalWaves}` : ''} 波失守`
    : '失守波次未上报'
  if (wave !== null) basis.push('wave')

  let leakText = ''
  if (kills !== null && leaked !== null) {
    const denom = kills + leaked
    const pct = denom > 0 ? Math.round(leaked * 100 / denom) : 0
    leakText = `漏怪 ${leaked}/${denom}（${pct}%），击杀 ${kills}`
    basis.push('kills', 'leaked')
  } else {
    leakText = `漏怪 ${leaked ?? '未上报'}，击杀 ${kills ?? '未上报'}`
    if (kills !== null) basis.push('kills')
    if (leaked !== null) basis.push('leaked')
  }

  const reactionText = reactions !== null ? `反应 ${reactions} 次` : '反应未上报'
  if (reactions !== null) basis.push('reactions')

  let pickText = ''
  let skipped = 0
  if (input.cardPicks) {
    const s = summarizeCardPicks(input.cardPicks)
    skipped = s.skipped
    pickText = s.skipped > 0
      ? `跳过选卡 ${s.skipped} 波`
      : s.total > 0 ? `取卡 ${s.taken} 张、弃 ${s.discarded} 张` : '本局无选卡记录'
    basis.push('card_picks')
  }

  let heatText = ''
  if (finite(input.heatMax)) {
    heatText = `峰值热量 ${input.heatMax}/${Number(HEAT_MAX)}`
    basis.push('heat_max')
  }

  let elementText = ''
  if (input.elementsUsed && typeof input.elementsUsed === 'object') {
    const names = Object.keys(input.elementsUsed)
      .filter((key) => Number(input.elementsUsed?.[key] ?? 0) > 0)
      .map((key) => ELEMENT_NAME[key as Element] ?? key)
    if (names.length === 1) elementText = `仅${names[0]} 1 种元素`
    else if (names.length > 1) elementText = `元素 ${names.join('、')}（${names.length} 种）`
    if (names.length) basis.push('elements_used')
  }

  const cause = leaked !== null && kills !== null && leaked > kills
    ? '漏怪多于击杀，火力/清怪不足'
    : reactions === 0
      ? '反应链未触发，元素搭配未形成收益'
      : skipped >= 2
        ? '多次跳过选卡，构筑补强不足'
        : '防线压力高于本局构筑承受力'
  const action = reactions === 0
    ? '下一局至少取 1 张卡补第二种元素以触发反应；同时优先升主技能或装备提高清怪'
    : skipped > 0
      ? '下一局不要连续跳过选卡，优先取技能/属性卡补强构筑'
      : '下一局优先升主技能或装备，并调整元素搭配提高反应覆盖'

  return [
    waveText, leakText, reactionText, pickText, heatText, elementText,
    cause, action, `依据：${basis.join('/')}`,
  ].filter(Boolean).join('；') + '。'
}
