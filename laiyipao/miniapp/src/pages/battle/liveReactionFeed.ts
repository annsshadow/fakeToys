/**
 * R28：实时反应 feed 的波次/敌人归因（纯只读派生）。
 * 归因只来自 R15 timeline 记录，绝不拿当前 waveIndex 冒充。
 * 不写引擎任何权威状态，不进 replay_hash。
 */

export const ATTRIBUTION_UNAVAILABLE = '归因不可用'
export const LIVE_FEED_BASIS = '依据：R15 reactionTimeline 逐次事件（wave/enemy_uid/reaction/damage）按事件发射顺序配对；截断或不匹配如实标注归因不可用'
export const LIVE_FEED_HONESTY = '辅助归因非权威判据，权威以 replay_hash 为准'

/** 本次 step 的反应事件（至少含 reaction/damage） */
export interface LiveReactionEvent {
  reaction: string
  damage: bigint | number
}

/** timeline 记录的结构形状（来自引擎只读旁路 ReactionTimelineEvent） */
export interface LiveTimelineRecord {
  wave?: unknown
  tick?: unknown
  reaction?: unknown
  enemy_uid?: unknown
  damage?: unknown
}

export interface ReactionAttribution {
  reaction: string
  damage: number
  wave: number | null
  enemy_uid: number | null
  available: boolean
}

export interface LiveFeedResult {
  rows: ReactionAttribution[]
  matchedCount: number
  unavailableCount: number
}

export interface LiveFeedAi {
  conclusion: string
  basis: string
  honesty: string
}

const isIntAtLeastOne = (v: unknown): v is number =>
  typeof v === 'number' && Number.isInteger(v) && v >= 1

const unavailableRow = (reaction: string, damage: number): ReactionAttribution => ({
  reaction,
  damage,
  wave: null,
  enemy_uid: null,
  available: false,
})

/**
 * 按事件发射顺序从 timeline 尾部逆序消费配对。
 * - 截断优先：truncated=true 时尾部已丢弃，全部不可用。
 * - 命中 reaction + damage 严格相等即消费游标，随后才做合法性校验。
 * - rows 与输入 events 下标一一对应。
 */
export function attributeLiveReactions(
  events: LiveReactionEvent[],
  timeline: LiveTimelineRecord[] | null | undefined,
  truncated: boolean,
): LiveFeedResult {
  const list = Array.isArray(events) ? events : []
  const rows: ReactionAttribution[] = list.map(e =>
    unavailableRow(String(e?.reaction ?? ''), Number(e?.damage ?? 0)),
  )
  if (truncated === true || !Array.isArray(timeline) || timeline.length === 0) {
    return { rows, matchedCount: 0, unavailableCount: rows.length }
  }

  let cursor = timeline.length - 1
  for (let i = list.length - 1; i >= 0; i--) {
    const e = list[i]
    const reaction = e?.reaction
    const damage = Number(e?.damage)
    if (typeof reaction !== 'string' || !reaction || !Number.isFinite(damage)) continue

    let hit = -1
    for (let j = cursor; j >= 0; j--) {
      const rec = timeline[j] as LiveTimelineRecord | undefined
      if (rec && rec.reaction === reaction && Number(rec.damage) === damage) {
        hit = j
        break
      }
    }
    if (hit < 0) continue

    // 命中即消费游标：即使记录非法也占住，绝不被后一事件重复占用。
    cursor = hit - 1
    const rec = timeline[hit] as LiveTimelineRecord
    const wave = rec.wave
    const enemyUid = rec.enemy_uid
    const valid =
      isIntAtLeastOne(wave) &&
      isIntAtLeastOne(enemyUid) &&
      typeof rec.reaction === 'string' && rec.reaction.length > 0 &&
      Number.isFinite(Number(rec.damage))
    if (valid) {
      rows[i] = { reaction, damage, wave: wave as number, enemy_uid: enemyUid as number, available: true }
    }
  }

  const matchedCount = rows.filter(r => r.available).length
  return { rows, matchedCount, unavailableCount: rows.length - matchedCount }
}

/** 可用时逐字「第N波 · #uid」，不可用时逐字归因不可用。 */
export function attributionText(row: ReactionAttribution | null | undefined): string {
  return row?.available && row.wave !== null && row.enemy_uid !== null
    ? `第${row.wave}波 · #${row.enemy_uid}`
    : ATTRIBUTION_UNAVAILABLE
}

/** AI 结论来自真实匹配计数，依据/红线用逐字常量。 */
export function buildLiveFeedAi(result: LiveFeedResult): LiveFeedAi {
  const total = result.rows.length
  const conclusion = total === 0
    ? '最近无反应记录可归因'
    : `已归因 ${result.matchedCount}/${total} 条最近反应`
  return { conclusion, basis: LIVE_FEED_BASIS, honesty: LIVE_FEED_HONESTY }
}
