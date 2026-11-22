// R10 §1I.3：宝石佩戴收益只认服务端 /me 权威属性差分，未上报写诚实空态。
import { fetchMe } from '@/api/client'

export interface GemGainRow { key: string; label: string; before: number; after: number; delta: number }
export interface GemGainView { available: boolean; text: string; rows: GemGainRow[]; basis: string }

const ATTACKER_FIELDS: { key: string; label: string }[] = [
  { key: 'attack', label: '攻击' },
  { key: 'crit_permille', label: '暴击率‰' },
  { key: 'crit_multiplier_permille', label: '暴击倍率‰' },
  { key: 'reaction_mult_permille', label: '反应倍率‰' },
  { key: 'element_coef_permille', label: '元素系数‰' },
  { key: 'heat_cap_permille', label: '热量上限‰' },
  { key: 'armor_permille', label: '护甲‰' },
]

/** 只比较服务端 build.attacker 的两份权威快照；缺任一份就是不虚构的诚实空态。 */
export function normalizeGemGain(data: unknown): GemGainView {
  const empty: GemGainView = {
    available: false,
    text: '佩戴收益未上报，先看词条描述',
    rows: [],
    basis: '需要佩戴前后两次服务端 /me 的 build.attacker 属性快照才能差分；静态词条说明不算已生效收益。',
  }
  if (!data || typeof data !== 'object') return empty
  const o = data as Record<string, any>
  const before = o.before?.build?.attacker
  const after = o.after?.build?.attacker
  const beforeUser = o.before?.user_id
  const afterUser = o.after?.user_id
  if (!before || !after || typeof before.attack !== 'number' || typeof after.attack !== 'number') return empty
  if (beforeUser && afterUser && beforeUser !== afterUser) return empty
  const rows: GemGainRow[] = ATTACKER_FIELDS.filter(({ key }) => typeof before[key] === 'number' && typeof after[key] === 'number')
    .map(({ key, label }) => ({ key, label, before: before[key], after: after[key], delta: after[key] - before[key] }))
  return {
    available: true,
    text: '服务端属性差分（佩戴前 → 佩戴后）',
    rows,
    basis: '来源：服务端 /me build.attacker 两次权威快照 · attack / crit / reaction_mult / heat_cap 等字段',
  }
}

/** 本地缓存 key 只含非敏感 ID；不写真值、不写 token。key 形状：lyp:r10:gem-gain:<user_id>:<gem_id> */
export function r10GemGainCacheKey(userId: number, gemId: number): string {
  return 'lyp:r10:gem-gain:' + userId + ':' + gemId
}

/** 佩戴/卸下写成功后由调用方采集：before = 写前快照，after = 写并 refreshProfile 后的 /me 快照。 */
export async function captureGemGain(before: unknown): Promise<Record<string, unknown> | null> {
  try {
    const after = await fetchMe()
    return { before, after }
  } catch {
    return null
  }
}
