import { request } from '@/api/client'
import { applyEquipmentImpact, normalizeImpact, type R7ImpactView } from '@/pages/_shared/r7ImpactApply'
import type { R6Evidence } from '@/pages/_shared/r6Recommendation'

/**
 * R14 §A/C：服务端权威 CTA。客户端只回传 element/reaction/source 白名单，
 * 绝不拼 id/replace；候选 id、替换 id、收益全部来自 /me/advice/action 响应。
 */
export type R14AdviceSource = 'battle' | 'reactions'
export interface R14AdviceQuery { element: string; reaction?: string; source: R14AdviceSource }
export interface R14AdviceEmpty { code: string; headline: string; message: string; cta: string; route: string }
export interface R14AdviceAction {
  badge: string
  element: string
  expectedReaction: string
  basedOn: string[]
  kind: 'skill' | 'equipment' | null
  candidateId: number
  replacedId: number
  candidateName: string
  replacedName: string
  params: { kind: string; id: number; replace: number } | null
  previewRoute: string
  evidence: R6Evidence[]
  impact: R7ImpactView | null
  status: 'ready' | 'empty'
  empty: R14AdviceEmpty | null
}

const isObj = (v: unknown): v is Record<string, unknown> => typeof v === 'object' && v !== null && !Array.isArray(v)
const str = (v: unknown, fallback = ''): string => typeof v === 'string' ? v : fallback
const num = (v: unknown): number => (typeof v === 'number' && Number.isFinite(v) ? v : 0)
const strings = (v: unknown): string[] => Array.isArray(v) ? v.filter((x): x is string => typeof x === 'string' && x.length > 0) : []

const EMPTY_CODES = ['NO_OWNED_CANDIDATE', 'NO_HISTORY', 'NOT_ENOUGH_DATA', 'CONFIG_MISSING', 'INVALID_ELEMENT', 'NOT_AUTHENTICATED']

function emptyOf(raw: Record<string, unknown>): R14AdviceEmpty | null {
  const e = isObj(raw.empty_reason) ? raw.empty_reason : null
  if (!e) return null
  const code = str(e.code)
  if (!code || !EMPTY_CODES.includes(code)) return null
  return {
    code,
    headline: str(e.headline, 'AI 暂时没有足够数据给出建议'),
    message: str(e.message, '当前没有可执行的候选，先打一场战斗或补齐 owning 目录。'),
    cta: str(e.cta, '去打一场战斗 →'),
    route: str(e.route, '/pages/stage/stage'),
  }
}

/** 把 advice 顶层 impact 交给已验证的 R7 归一器，不复制收益算法、不编数字。 */
function impactOf(raw: Record<string, unknown>, kind: 'skill' | 'equipment', basedOn: string[]): R7ImpactView | null {
  const cfg = basedOn.find((b) => b.indexOf('/config') >= 0) || str(raw.config_version, '未标注')
  const envelope = {
    schema_version: 'rec-v1',
    context: kind,
    impact: raw.impact,
    insights: [{ basis: basedOn }],
    evidence: Array.isArray(raw.evidence) ? raw.evidence : [],
    input_snapshot: { config_version: cfg },
  }
  return normalizeImpact(envelope, kind)
}

/**
 * 严格归一：任何空态都不得带非零候选 id；params 必须与顶层候选一一对应。
 * 校验失败返回 null——页面据此不渲染按钮，绝不退回本地猜测 id。
 */
export function normalizeAdviceAction(payload: unknown, expectedSource?: R14AdviceSource): R14AdviceAction | null {
  if (!isObj(payload) || payload.schema_version !== 'advice-action-v1') return null
  const element = str(payload.element)
  // reaction 入参与 expected_reaction 仅用于依据展示（SPEC A.1 可选），不参与鉴权；
  // 图鉴页按元素查询时服务端可回空串，不能因此吞掉权威候选。
  const expectedReaction = str(payload.expected_reaction)
  if (!element) return null
  const basedOn = strings(payload.based_on)
  if (!basedOn.length) return null
  if (expectedSource && payload.source && payload.source !== expectedSource) return null
  const badge = str(payload.badge, 'AI')
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
  const previewRoute = str(payload.preview_route, '')
  const empty = emptyOf(payload)
  const candidateId = num(payload.candidate_id)
  const replacedId = num(payload.replaced_id)
  const params = isObj(payload.params) ? payload.params : null

  if (empty) {
    // 防作弊红线：空态绝不允许携带可写参数（服务端 code 与 id=0 必须一致）。
    if (candidateId !== 0 || replacedId !== 0 || params !== null) return null
    return { badge, element, expectedReaction, basedOn, kind: null, candidateId: 0, replacedId: 0,
      candidateName: '', replacedName: '', params: null, previewRoute, evidence, impact: null,
      status: 'empty', empty }
  }

  const kind = payload.kind
  if (kind !== 'skill' && kind !== 'equipment') return null
  if (!(candidateId > 0) || !(replacedId > 0) || candidateId === replacedId) return null
  if (!params || str(params.kind) !== kind || num(params.id) !== candidateId || num(params.replace) !== replacedId) return null
  if (previewRoute && !previewRoute.startsWith('/pages/')) return null
  return {
    badge, element, expectedReaction, basedOn, kind, candidateId, replacedId,
    candidateName: str(payload.candidate_name, '推荐项'),
    replacedName: str(payload.replaced_name, '当前项'),
    params: { kind, id: candidateId, replace: replacedId },
    previewRoute, evidence, impact: impactOf(payload, kind, basedOn),
    status: 'ready', empty: null,
  }
}

/** 只带白名单 query；请求失败只回 null，页面显示诚实空态，不本地伪造候选。 */
export async function fetchAdviceAction(query: R14AdviceQuery): Promise<R14AdviceAction | null> {
  try {
    let path = '/me/advice/action?element=' + encodeURIComponent(query.element) + '&source=' + encodeURIComponent(query.source)
    if (query.reaction) path += '&reaction=' + encodeURIComponent(query.reaction)
    const raw = await request<unknown>(path)
    return normalizeAdviceAction(raw, query.source)
  } catch {
    return null
  }
}

export interface R14ApplyStore {
  loadout: number[]
  equipment: Array<{ id: number; slot: string; equipped: boolean }>
  loadLoadout(): Promise<void>
  persistLoadout(ids: number[]): Promise<boolean>
  loadEquipment(rethrow?: boolean): Promise<void>
}
export interface R14ApplyResult { ok: boolean; error: string; doneName: string }

/** 只在第二步确认时调用；写入前后都读服务端权威配置，失败保留原配置。 */
export async function applyAdviceAction(action: R14AdviceAction, store: R14ApplyStore): Promise<R14ApplyResult> {
  if (action.status !== 'ready' || !action.params || !action.kind) {
    return { ok: false, error: '服务端没有可执行的候选，未写入任何配置。', doneName: '' }
  }
  try {
    if (action.kind === 'skill') {
      await store.loadLoadout()
      const current = (store.loadout || []).slice(0, 4)
      while (current.length < 4) current.push(0)
      const slot = current.indexOf(action.replacedId)
      if (slot < 0) return { ok: false, error: '当前出战槽位已变化，未写入任何配置。请重试', doneName: '' }
      if (current.includes(action.candidateId)) {
        return { ok: false, error: '推荐技能已在出战配置中，未重复写入。请重试', doneName: '' }
      }
      current[slot] = action.candidateId
      const ok = await store.persistLoadout(current)
      if (!ok) return { ok: false, error: '写入失败，原出战配置未改动。请重试', doneName: '' }
      return { ok: true, error: '', doneName: action.candidateName }
    }
    await store.loadEquipment()
    const candidate = (store.equipment || []).find((e) => e.id === action.candidateId)
    if (!candidate) return { ok: false, error: '推荐装备尚未拥有，未写入任何配置。请重试', doneName: '' }
    const current = (store.equipment || []).map((e) => ({ id: e.id, slot: e.slot, equipped: e.equipped }))
    await applyEquipmentImpact(current, action.candidateId, action.replacedId, candidate.slot)
    await store.loadEquipment()
    return { ok: true, error: '', doneName: action.candidateName }
  } catch (e) {
    return { ok: false, error: '写入失败：' + ((e as Error)?.message || '未知错误') + '，原配置未改动。请重试', doneName: '' }
  }
}

