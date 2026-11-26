// R26 选卡阶段下一波敌情简报：纯派生，不碰引擎状态/结算/replay_hash。
// 数据来源仅为 eng.cfg.level.waves 与 store.enemyMap，调用方传入。

export interface NextWaveBriefRow {
  enemy_id: number
  name: string
  count: number
  threat: string
  unknown: boolean
}

export interface NextWaveBrief {
  waveIndex: number
  rows: NextWaveBriefRow[]
  empty: boolean
  ai: string
}

type SpawnLike = { enemy_id: number; count: number }
type WaveLike = { wave_index?: number; spawns?: unknown }
type EnemyDefLike = {
  id: number
  name: string
  is_boss?: boolean
  fly_height?: number
  burrow?: boolean
  attack_range?: number
}

/** 威胁标签优先级与 stage.vue enemyThreatTag 完全一致 */
function threatOf(def: EnemyDefLike | undefined): string {
  if (!def) return ''
  if (def.is_boss) return '[首领]'
  if ((def.fly_height ?? 0) > 0) return '[飞行]'
  if (def.burrow) return '[潜地]'
  if ((def.attack_range ?? 0) > 0) return '[远程]'
  return ''
}

/**
 * 取 waves[waveIndex + 1]，按 enemy_id 聚合 count，派生只读简报。
 * waveIndex 为引擎当前波次下标（0 起）；测试可用 -1 指向 waves[0]。
 */
export function buildNextWaveBrief(
  waves: unknown,
  waveIndex: number,
  enemyDefs: ReadonlyMap<number, EnemyDefLike> | Readonly<Record<number, EnemyDefLike>> | undefined | null,
): NextWaveBrief {
  const nextIndex = waveIndex + 1
  const emptyBrief = (): NextWaveBrief => ({ waveIndex: nextIndex, rows: [], empty: true, ai: '' })

  if (!Array.isArray(waves)) return emptyBrief()
  const next = (waves as WaveLike[])[nextIndex]
  if (!next || !Array.isArray(next.spawns)) return emptyBrief()

  const counts = new Map<number, number>()
  for (const sp of next.spawns as SpawnLike[]) {
    if (!sp || typeof sp.enemy_id !== 'number' || !Number.isInteger(sp.enemy_id) || sp.enemy_id < 0) continue
    if (typeof sp.count !== 'number' || !Number.isFinite(sp.count) || sp.count <= 0) continue
    counts.set(sp.enemy_id, (counts.get(sp.enemy_id) ?? 0) + sp.count)
  }
  if (counts.size === 0) return emptyBrief()

  const defs = enemyDefs
  const rows: NextWaveBriefRow[] = []
  for (const [enemy_id, count] of counts) {
    const def: EnemyDefLike | undefined = defs instanceof Map
      ? defs.get(enemy_id)
      : (defs as Readonly<Record<number, EnemyDefLike>> | undefined)?.[enemy_id]
    const known = !!def && typeof def.name === 'string' && def.name.length > 0
    rows.push({
      enemy_id,
      name: known ? def!.name : `#${enemy_id}`,
      count,
      threat: known ? threatOf(def) : '',
      unknown: !known,
    })
  }
  rows.sort((a, b) => b.count - a.count || a.enemy_id - b.enemy_id)

  const kinds = rows.length
  const total = rows.reduce((sum, r) => sum + r.count, 0)
  const threatCounts = new Map<string, number>()
  for (const r of rows) if (r.threat) threatCounts.set(r.threat, (threatCounts.get(r.threat) ?? 0) + r.count)
  const threatParts = Array.from(threatCounts.entries()).map(([t, n]) => `${t}×${n}`).join('、')

  let ai = `下一波 ${kinds} 种敌人共 ${total} 只`
  if (threatParts) ai += `，其中 ${threatParts}，结合本波威胁与手牌元素协同选卡`
  else ai += '，按伤害与元素协同选卡'

  return { waveIndex: nextIndex, rows, empty: false, ai }
}
