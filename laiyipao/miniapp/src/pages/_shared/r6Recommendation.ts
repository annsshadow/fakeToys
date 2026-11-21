import { request } from '@/api/client'

/** R6 §1E.2 用户侧推荐接口允许的 context（本 H5 任务只消费其中三页）。 */
export type R6Context = 'equipment' | 'reactions' | 'skills'

export interface R6Evidence {
  metric: string
  value: number | string
  unit: string
  source: string
  window: string
}

export interface R6Cta {
  label: string
  route: string
  params?: Record<string, string | number>
}

export interface R6Card {
  status: 'api' | 'local' | 'empty'
  context: string
  badge: 'AI'
  headline: string
  engineLabel: string
  evidence: R6Evidence[]
  basis: string[]
  cta: R6Cta
  emptyCode?: string
  generatedAt?: string
}

export interface R6LocalInput {
  context: R6Context
  headline: string
  basis: string[]
  evidence?: R6Evidence[]
  ctaLabel: string
  ctaRoute: string
  ctaParams?: Record<string, string | number>
}

const isObj = (v: unknown): v is Record<string, unknown> => typeof v === 'object' && v !== null && !Array.isArray(v)
const str = (v: unknown, fallback = ''): string => (typeof v === 'string' ? v : fallback)
const allowedConfidence = new Set([null, 'rule_based', 'statistical'])

/** 拒绝假置信度/伪模型名；不认识的 name 一律降级为 rules，不宣称 GPT/LLM。 */
function modelLabel(model: unknown): string {
  if (!isObj(model)) return '规则引擎 · rules rec-v1'
  const raw = str(model.name, 'rules').toLowerCase()
  const name = raw === 'stats' || raw === 'statistical' ? 'stats' : 'rules'
  const version = str(model.version, 'rec-v1')
  return (model.kind === 'statistical' || name === 'stats' ? '统计模型' : '规则引擎') + ' · ' + name + ' ' + version
}

/** §1A.4 / §1E.2：数据不足时诚实空态，写清缺什么与下一步，不回退写死推荐。 */
export function emptyRecommendation(code: string, message?: string): R6Card {
  const known: Record<string, string> = {
    NO_HISTORY: '还没有可参照的战斗历史',
    NOT_ENOUGH_DATA: '战斗样本还不足以判断搭配优劣',
    CONFIG_MISSING: '配置版本尚未下发，无法比较当前配装',
  }
  const reason = message || known[code] || '推荐所需数据尚未就绪'
  return {
    status: 'empty',
    context: 'recommendation',
    badge: 'AI',
    headline: 'AI 暂时没有足够数据给出建议',
    engineLabel: '数据不足 · ' + code,
    evidence: [],
    basis: ['缺什么：' + reason, '下一步：先打一场战斗，或补齐配置后回到本页刷新'],
    cta: { label: '去打一场战斗 →', route: '/pages/stage/stage' },
    emptyCode: code,
  }
}

/** 接口失败时的本地兜底：必须显式标注「本地规则引擎」，不冒充个性化推荐。 */
export function localFallbackCard(input: R6LocalInput): R6Card {
  return {
    status: 'local',
    context: input.context,
    badge: 'AI',
    headline: input.headline,
    engineLabel: '本地规则引擎 · 当前页面数据',
    evidence: input.evidence ?? [],
    basis: input.basis,
    cta: { label: input.ctaLabel, route: input.ctaRoute, params: input.ctaParams },
  }
}

/** 把服务端 rec-v1 响应归一为推荐卡；字段不合格时返回 null，调用方走本地兜底。 */
export function normalizeRecommendation(payload: unknown, context: string): R6Card | null {
  if (!isObj(payload) || payload.schema_version !== 'rec-v1') return null
  if (payload.context && payload.context !== context) return null
  const model = isObj(payload.model) ? payload.model : null
  if (!model || typeof model.name !== 'string' || typeof model.version !== 'string') return null

  const empty = isObj(payload.empty_reason) ? payload.empty_reason : null
  const code = str(empty?.code)
  if (code && ['NO_HISTORY', 'NOT_ENOUGH_DATA', 'CONFIG_MISSING'].includes(code)) {
    return emptyRecommendation(code, str(empty?.message) || undefined)
  }

  const insights = Array.isArray(payload.insights) ? payload.insights : []
  if (!insights.length) return null
  const insight = insights.find((i): i is Record<string, unknown> => isObj(i) && typeof i.headline === 'string' && isObj(i.cta) && typeof i.cta.route === 'string')
  if (!insight) return null
  const confidence = (insight as Record<string, unknown>).confidence
  if (!allowedConfidence.has(confidence as null | string)) return null

  const ctaRaw = insight.cta as Record<string, unknown>
  const params: Record<string, string | number> = {}
  if (isObj(ctaRaw.params)) {
    for (const [k, v] of Object.entries(ctaRaw.params)) {
      if (typeof v === 'string' || typeof v === 'number') params[k] = v
    }
  }
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

  const basis = (Array.isArray(insight.basis) ? insight.basis : [])
    .filter((b): b is string => typeof b === 'string' && b.length > 0)
    .slice(0, 4)
  const snap = isObj(payload.input_snapshot) ? payload.input_snapshot : null
  if (snap && typeof snap.window_days === 'number') {
    basis.push('统计窗口 ' + snap.window_days + ' 天' + (typeof snap.battle_count === 'number' ? ' / ' + snap.battle_count + ' 场' : ''))
  }
  if (snap && typeof snap.config_version === 'string' && snap.config_version) {
    basis.push('配置版本 ' + snap.config_version)
  }
  const insightModel = isObj(insight.model) ? insight.model : model
  const engineLabel = modelLabel(insightModel)

  return {
    status: 'api',
    context,
    badge: 'AI',
    headline: str(insight.headline),
    engineLabel,
    evidence,
    basis: basis.length ? basis : ['依据来自推荐接口 evidence[] 与输入快照'],
    cta: { label: str(ctaRaw.label, '查看建议'), route: str(ctaRaw.route), params },
    generatedAt: typeof payload.generated_at === 'string' ? payload.generated_at : undefined,
  }
}

/** GET /me/recommendations；网络/鉴权/字段异常统一返回 null，由页面挂本地兜底。 */
export async function fetchRecommendations(context: R6Context): Promise<R6Card | null> {
  try {
    const raw = await request<unknown>('/me/recommendations?context=' + encodeURIComponent(context))
    return normalizeRecommendation(raw, context)
  } catch {
    return null
  }
}
