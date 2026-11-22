// R10 §1I.2：养成前后可信差值只认两次真实结算，不按配置推算。
import type { BattleDetail } from '@/api/client'

export interface GrowthRow { label: string; before: number; after: number; delta: number }
export interface GrowthCompareView {
  available: boolean
  text: string
  levelId: number
  rows: GrowthRow[]
  hashes: string[]
  basis: string
}

const FIELDS: { key: keyof BattleDetail; label: string }[] = [
  { key: 'score', label: '分数' },
  { key: 'stars', label: '星级' },
  { key: 'kills', label: '击杀' },
  { key: 'leaked', label: '漏怪' },
  { key: 'reactions', label: '反应数' },
  { key: 'hp_left', label: '剩余血量' },
  { key: 'wave_reached', label: '到达波次' },
]

const num = (v: unknown): number => (typeof v === 'number' && Number.isFinite(v) ? v : Number(v) || 0)
const stamp = (v: unknown): number => {
  const t = typeof v === 'number' ? v : Date.parse(String(v ?? ''))
  return Number.isFinite(t) ? t : 0
}

/** 同一关、同一难度、两次真实结算（各带 replay_hash）才给差值；否则诚实空态。 */
export function normalizeGrowthCompare(list: BattleDetail[]): GrowthCompareView {
  const empty: GrowthCompareView = {
    available: false,
    text: '还缺养成前那局战报，先打一局同关',
    levelId: 0,
    rows: [],
    hashes: [],
    basis: '需要同一关两次真实结算，各自带 replay_hash 才能核对；构筑变更点未上报。',
  }
  const valid = (list ?? []).filter((b) => b && typeof b.level_id === 'number' && b.replay_hash && stamp(b.created_at) > 0)
  if (valid.length < 2) return empty
  const chronological = [...valid].sort((a, b) => stamp(a.created_at) - stamp(b.created_at))
  // 相邻两次（按 created_at）同关结算，避免拿不同关卡或隔着别的构筑的局硬凑。
  let pair: [BattleDetail, BattleDetail] | null = null
  for (let i = chronological.length - 1; i > 0; i--) {
    if (chronological[i].level_id === chronological[i - 1].level_id) {
      pair = [chronological[i - 1], chronological[i]]
      break
    }
  }
  if (!pair) return empty
  const [before, after] = pair
  const rows: GrowthRow[] = FIELDS.filter(({ key }) => typeof before[key] === 'number' && typeof after[key] === 'number')
    .map(({ key, label }) => {
      const a = num(before[key]); const b = num(after[key])
      return { label, before: a, after: b, delta: b - a }
    })
  return {
    available: true,
    text: '同一关第 ' + before.level_id + ' 关 · 两次真实结算的可信差值（构筑变更点未上报）',
    levelId: before.level_id,
    rows,
    hashes: [before.replay_hash, after.replay_hash],
    basis: '两次结算 replay_hash：' + before.replay_hash + ' / ' + after.replay_hash + ' · 同关同难度 · 构筑变更点未上报',
  }
}

/** 取同一关最近一局真实结算作为「养成前」基准；缺则诚实空态。 */
export function findGrowthBaseline(list: BattleDetail[], levelId: number): BattleDetail | null {
  const matches = (list ?? []).filter((b) => b && b.level_id === levelId && b.replay_hash)
  return matches.sort((a, b) => stamp(b.created_at) - stamp(a.created_at))[0] ?? null
}
