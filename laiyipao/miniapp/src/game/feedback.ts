// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R9 战斗反馈的纯归因层。
 *
 * 这些数据只用于展示，不参与引擎推进、不进 record/replay_hash。
 * 所有数值必须来自真实战斗事件或服务端权威 card_picks；
 * 缺字段就写诚实空态，绝不推算或编造。
 */
import { ELEMENT_NAME, REACTIONS, type Element, type ReactionKey } from "./elements"

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
  const scenario = !spec ? "反应详情未上报"
    : spec.aoeRadius > 0 ? `密集波 25% 溅射（半径 ${spec.aoeRadius}）`
    : spec.dispelShield ? "驱散护盾"
    : spec.armorShredPermille > 0 ? "削甲击退"
    : spec.spreadStacks > 0 ? "传播腐蚀层数"
    : spec.amplifyPct > 0 ? "提高目标受击伤害"
    : spec.statusDurationMs > 0 ? `附加状态 ${spec.statusDurationMs}ms`
    : spec.descr
  const conclusion = known ? `${pair}触发${name}，${scenario}` : `元素组合未上报，${name}详情无法归因`
  return {
    name,
    pair,
    cue: `${conclusion}；依据 reaction=${String(reaction)}/damage=${Number(damage)}；点卡凑同元素`,
  }
}
