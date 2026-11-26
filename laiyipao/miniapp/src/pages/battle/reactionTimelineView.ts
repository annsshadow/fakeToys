/**
 * R29：完整反应时间线抽屉的只读展示模型（纯函数）。
 *
 * 数据来自 BattleEngine.reactionTimeline() / reactionTimelineTruncated() 的只读旁路，
 * 反应名来自服务端 config.reactions。全程只读派生，不写引擎权威状态、不进 replay_hash。
 * 合法性口径照抄 battles.vue 的 R15 战报时间线，绝不输出伪造数字。
 */

export interface R29TimelineRow {
  index: number
  wave: number
  tick: number
  reaction: string
  reactionName: string
  enemy_uid: number
  damage: number
}

export interface R29TimelineView {
  rows: R29TimelineRow[]
  empty: boolean
  truncated: boolean
}

export const R29_EMPTY_TEXT = '本局暂无逐次反应记录'
export const R29_TRUNCATED_TEXT = '已截断，仅显示前 200 条'
export const R29_BASIS = '依据：引擎 reactionTimeline() 只读旁路（前 200 条，不进 replay_hash）；反应名以服务端 config.reactions 为准'
export const R29_HONESTY = '辅助展示非权威判据，权威以 replay_hash 为准'
export const R29_UNKNOWN_SUFFIX = '（未知反应）'

interface TimelineEventLike {
  wave?: unknown
  tick?: unknown
  reaction?: unknown
  enemy_uid?: unknown
  damage?: unknown
}

interface ReactionDefLike {
  key: string
  name: string
}

interface ValidEvent {
  wave: number
  tick: number
  reaction: string
  enemy_uid: number
  damage: number
  order: number
}

const isInt = (n: unknown): n is number => typeof n === 'number' && Number.isInteger(n)
const isFiniteNumber = (n: unknown): n is number => typeof n === 'number' && Number.isFinite(n)

/** 从只读时间线派生展示行：过滤非法记录、按 wave/tick 稳定排序、映射服务端反应名。 */
export function buildReactionTimelineView(
  events: TimelineEventLike[] | null | undefined,
  truncated: boolean,
  reactions: ReactionDefLike[] | null | undefined,
): R29TimelineView {
  const nameMap = new Map<string, string>()
  if (Array.isArray(reactions)) {
    for (const def of reactions) {
      if (def && typeof def.key === 'string' && typeof def.name === 'string' && !nameMap.has(def.key)) {
        nameMap.set(def.key, def.name)
      }
    }
  }

  const valid: ValidEvent[] = []
  if (Array.isArray(events)) {
    for (let i = 0; i < events.length; i++) {
      const e = events[i]
      if (!e || typeof e !== 'object') continue
      if (!isInt(e.wave) || e.wave < 1) continue
      if (!isInt(e.tick) || e.tick < 0) continue
      if (typeof e.reaction !== 'string' || e.reaction.length === 0) continue
      if (!isInt(e.enemy_uid) || e.enemy_uid < 1) continue
      if (!isFiniteNumber(e.damage)) continue
      valid.push({
        wave: e.wave,
        tick: e.tick,
        reaction: e.reaction,
        enemy_uid: e.enemy_uid,
        damage: e.damage,
        order: i,
      })
    }
  }

  // 稳定排序：wave 升序 → tick 升序 → 同刻保持原输入次序
  valid.sort((a, b) => a.wave - b.wave || a.tick - b.tick || a.order - b.order)

  const rows: R29TimelineRow[] = valid.map((v, idx) => ({
    index: idx,
    wave: v.wave,
    tick: v.tick,
    reaction: v.reaction,
    reactionName: nameMap.has(v.reaction) ? nameMap.get(v.reaction)! : v.reaction + R29_UNKNOWN_SUFFIX,
    enemy_uid: v.enemy_uid,
    damage: v.damage,
  }))

  return { rows, empty: rows.length === 0, truncated: Boolean(truncated) }
}
