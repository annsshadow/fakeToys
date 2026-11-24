/** R27 波次推进只读快照的原始四字段（来自 BattleEngine.waveProgressHud()）。 */
export interface WaveProgressRaw {
  waveIndex: number
  waveTotal: number
  onField: number
  pending: number
}

/** R27 波次推进展示派生结果。只读派生，不进 replay_hash。 */
export interface WaveProgressView extends WaveProgressRaw {
  resolved: number
  clearedPct: number
  empty: boolean
  ai: string
}

const clamp = (n: number, lo: number, hi: number): number => Math.max(lo, Math.min(hi, n))

/**
 * 把引擎只读快照收敛为可展示的波次进度。
 * - 非法数据 / 本波无敌人 → empty=true，不抛异常、不编造。
 * - resolved = 已处理（击败 + 漏怪），不拆分、不反推击杀；Boss 按 1 只计入。
 * - 纯展示派生：不写引擎、不参与结算与 replay_hash。
 */
export function buildWaveProgress(raw: unknown): WaveProgressView {
  const empty = (): WaveProgressView => ({
    waveIndex: 0, waveTotal: 0, onField: 0, pending: 0,
    resolved: 0, clearedPct: 0, empty: true, ai: '',
  })
  if (!raw || typeof raw !== 'object') return empty()
  const r = raw as Partial<WaveProgressRaw>
  if (typeof r.waveIndex !== 'number' || !Number.isInteger(r.waveIndex) || r.waveIndex < 0) return empty()
  if (typeof r.waveTotal !== 'number' || !Number.isFinite(r.waveTotal)) return empty()
  if (typeof r.onField !== 'number' || !Number.isFinite(r.onField)) return empty()
  if (typeof r.pending !== 'number' || !Number.isFinite(r.pending)) return empty()
  const total = Math.max(0, Math.floor(r.waveTotal))
  const onField = clamp(Math.floor(r.onField), 0, total)
  const pending = clamp(Math.floor(r.pending), 0, total)
  const resolved = clamp(total - onField - pending, 0, total)
  const pct = total > 0 ? Math.round((resolved / total) * 1000) / 10 : 0
  const clearedPct = clamp(pct, 0, 100)
  if (total === 0) return empty()
  const ai = `本波共 ${total} 只，已处理 ${resolved} 只，场上 ${onField} 只、待出场 ${pending} 只，清理进度 ${clearedPct.toFixed(1)}%`
  return {
    waveIndex: r.waveIndex,
    waveTotal: total,
    onField,
    pending,
    resolved,
    clearedPct,
    empty: false,
    ai,
  }
}
