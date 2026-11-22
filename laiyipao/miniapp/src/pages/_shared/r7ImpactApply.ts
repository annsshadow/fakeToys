import { request, saveEquippedEquipment } from '@/api/client'
import { emptyRecommendation, normalizeRecommendation, type R6Card, type R6Evidence } from '@/pages/_shared/r6Recommendation'

/**
 * R7 §1F.4：推荐卡上的「应用推荐」只能打开对比预览，确认后才写。
 * 所有收益数字来自 GET /me/recommendations/impact 的顶层 impact，
 * 不使用 insights[].payload（服务端不那里放 impact），也不做本地收益估算。
 */
export type R7ImpactKind = 'skill' | 'equipment'

export interface R7ImpactChanges {
  attackPermilleDelta: number
  armorPermilleDelta: number
  damagePerWaveDelta: number
  damagePerWavePct: number
  powerDelta: number
  ratingTotalDelta: number
  starRateObserved: number | null
  starProbabilityEstimate: null
}

export interface R7ImpactSide {
  attackPermille: number
  armorPermille: number
  estimatedDamagePerWave: number
  power: number
  ratingTotal: number
}

export interface R7ImpactView {
  kind: R7ImpactKind
  candidateId: number
  replacedId: number
  candidateName: string
  replacedName: string
  baseline: R7ImpactSide
  candidate: R7ImpactSide
  changes: R7ImpactChanges
  method: string
  starRateNote: string
  evidence: R6Evidence[]
  basis: string[]
  configVersion: string
  battleCount: number
  windowDays: number
  generatedAt: string
}

export interface R7ImpactResult {
  card: R6Card | null
  impact: R7ImpactView | null
  error: string
}

const isObj = (v: unknown): v is Record<string, unknown> => typeof v === 'object' && v !== null && !Array.isArray(v)
const str = (v: unknown, fallback = ''): string => typeof v === 'string' ? v : fallback
const num = (v: unknown): number => (typeof v === 'number' && Number.isFinite(v) ? v : 0)
const nullableNum = (v: unknown): number | null => (typeof v === 'number' && Number.isFinite(v) ? v : null)
const emptyCodes = ['NO_HISTORY', 'NOT_ENOUGH_DATA', 'CONFIG_MISSING']

function emptyCodeOf(payload: Record<string, unknown>): string {
  const reason = isObj(payload.empty_reason) ? payload.empty_reason : null
  const code = str(reason?.code)
  return emptyCodes.includes(code) ? code : ''
}

function sideOf(raw: unknown): R7ImpactSide {
  const s = isObj(raw) ? raw : {}
  return {
    attackPermille: num(s.attack_permille),
    armorPermille: num(s.armor_permille),
    estimatedDamagePerWave: num(s.estimated_damage_per_wave),
    power: num(s.power),
    ratingTotal: num(s.rating_total),
  }
}

/** 响应不合法时返回 null；空历史也返回 null（空态由 normalizeImpactCard 负责）。 */
export function normalizeImpact(payload: unknown, expectedKind: R7ImpactKind): R7ImpactView | null {
  if (!isObj(payload) || payload.schema_version !== 'rec-v1') return null
  if (payload.context && payload.context !== expectedKind) return null
  if (emptyCodeOf(payload)) return null
  const raw = payload.impact
  if (!isObj(raw) || raw.kind !== expectedKind) return null
  const changesRaw = isObj(raw.changes) ? raw.changes : {}
  // 服务端明确禁止把换装映射到星级概率；这里只接受 null，防止伪造数值混入 UI。
  if (changesRaw.star_probability_estimate !== null) return null

  const snap = isObj(payload.input_snapshot) ? payload.input_snapshot : {}
  const insights = Array.isArray(payload.insights) ? payload.insights : []
  const first = insights.find(isObj)
  const basis = first && Array.isArray(first.basis)
    ? first.basis.filter((b): b is string => typeof b === 'string' && b.length > 0).slice(0, 4)
    : []
  const evidence: R6Evidence[] = (Array.isArray(payload.evidence) ? payload.evidence : [])
    .filter(isObj)
    .slice(0, 3)
    .map((e) => ({
      metric: str(e.metric, '指标'),
      value: typeof e.value === 'number' || typeof e.value === 'string' ? e.value : '—',
      unit: str(e.unit),
      source: str(e.source, 'unknown'),
      window: str(e.window, '—'),
    }))

  return {
    kind: expectedKind,
    candidateId: num(raw.candidate_id),
    replacedId: num(raw.replaced_id),
    candidateName: str(raw.candidate_name, '推荐项'),
    replacedName: str(raw.replaced_name, '当前项'),
    baseline: sideOf(raw.baseline),
    candidate: sideOf(raw.candidate),
    changes: {
      attackPermilleDelta: num(changesRaw.attack_permille_delta),
      armorPermilleDelta: num(changesRaw.armor_permille_delta),
      damagePerWaveDelta: num(changesRaw.damage_per_wave_delta),
      damagePerWavePct: num(changesRaw.damage_per_wave_pct),
      powerDelta: num(changesRaw.power_delta),
      ratingTotalDelta: num(changesRaw.rating_total_delta),
      starRateObserved: nullableNum(changesRaw.star_rate_observed),
      starProbabilityEstimate: null,
    },
    method: str(raw.method, '收益口径未由服务端提供'),
    starRateNote: str(raw.star_rate_note, '星级概率无可信模型，未做推测'),
    evidence,
    basis,
    configVersion: str(snap.config_version, '未标注'),
    battleCount: num(snap.battle_count),
    windowDays: num(snap.window_days),
    generatedAt: str(payload.generated_at, ''),
  }
}

/** 推荐卡正文仍走 R6 的统一归一器，空历史映射为诚实空态；不构造本地收益数字。 */
export function normalizeImpactCard(payload: unknown, context: R7ImpactKind): R6Card | null {
  const card = normalizeRecommendation(payload, context)
  if (card) return card
  if (!isObj(payload) || payload.schema_version !== 'rec-v1') return null
  const code = emptyCodeOf(payload)
  const reason = isObj(payload.empty_reason) ? payload.empty_reason : null
  return code ? emptyRecommendation(code, str(reason?.message) || undefined) : null
}

/** 查询真实收益；失败只回错误，绝不生成假数字。 */
export async function fetchRecommendationImpact(
  kind: R7ImpactKind,
  candidateId: number,
  replacedId: number,
): Promise<R7ImpactResult> {
  try {
    const raw = await request<unknown>(
      '/me/recommendations/impact?kind=' + kind + '&id=' + candidateId + '&replace=' + replacedId,
    )
    return { card: normalizeImpactCard(raw, kind), impact: normalizeImpact(raw, kind), error: '' }
  } catch (e) {
    return { card: null, impact: null, error: (e as Error)?.message || '真实收益预估请求失败' }
  }
}

interface EquippedSlot { id: number; slot: string; equipped: boolean }

/** 确认后整组替换：先摘掉被替换项与同槽旧件，再放入已拥有候选，服务端事务校验。 */
export function applyEquipmentImpact(
  current: EquippedSlot[],
  candidateId: number,
  replacedId: number,
  candidateSlot: string,
) {
  const next = current
    .filter((e) => e.equipped && e.id !== replacedId && e.slot !== candidateSlot)
    .map((e) => e.id)
  if (!next.includes(candidateId)) next.push(candidateId)
  return saveEquippedEquipment(next)
}
