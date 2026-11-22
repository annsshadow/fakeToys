import { myBattleDetail, myBattleHistory, type BattleDetail } from '@/api/client'

/** R7 §1F.4：只从战报接口取数，不重算分数、星级，也不写死示例数值。 */
export interface BattleCausalView {
  hasHistory: boolean
  battleId: number
  score: number
  stars: number
  leaked: number
  reactions: number
  topElements: { name: string; count: number }[]
  topReactions: { name: string; count: number }[]
  text: string
  source: string
}

const ELEMENT_LABEL: Record<string, string> = {
  fire: '焰', ice: '冰', lightning: '电', corrosion: '毒', kinetic: '动能',
}

const num = (v: unknown): number => (typeof v === 'number' && Number.isFinite(v) ? v : 0)

function topPairs(map: Record<string, unknown>): { name: string; count: number }[] {
  return Object.entries(map)
    .map(([key, value]) => ({ name: key, count: num(value) }))
    .filter((x) => x.count > 0)
    .sort((a, b) => b.count - a.count)
    .slice(0, 3)
}

function labelPair(key: string): string {
  return key.replace(/[+"\-]/g, ' + ').split(' + ').map((k) => ELEMENT_LABEL[k] ?? k).join('+')
}

export function normalizeBattleCausal(detail: BattleDetail | null): BattleCausalView {
  if (!detail) {
    return {
      hasHistory: false, battleId: 0, score: 0, stars: 0, leaked: 0, reactions: 0,
      topElements: [], topReactions: [],
      text: '没有战报：先打一场战斗，才能解释「本局靠什么拿分」。',
      source: 'battle_records',
    }
  }
  const topElements = topPairs(detail.elements_used ?? {}).map((x) => ({ name: ELEMENT_LABEL[x.name] ?? x.name, count: x.count }))
  const topReactions = topPairs(detail.reactions_used ?? {}).map((x) => ({ name: labelPair(x.name), count: x.count }))
  const elementText = topElements.map((x) => x.name + ' ' + x.count + ' 次').join('、')
  const reactionText = topReactions.map((x) => x.name + ' ' + x.count + ' 次').join('、')
  return {
    hasHistory: true,
    battleId: detail.id,
    score: num(detail.score),
    stars: num(detail.stars),
    leaked: num(detail.leaked),
    reactions: num(detail.reactions),
    topElements,
    topReactions,
    text: '本局靠' + (elementText || '出战元素（战报未细分）') + '推进，触发 ' + num(detail.reactions) +
      ' 次反应（' + (reactionText || '战报未细分') + '），拿到 ' + num(detail.score) + ' 分、' + num(detail.stars) + ' 星。',
    source: 'battle_records/战报 #' + detail.id + ' · ' + String(detail.replay_hash || '').slice(0, 8),
  }
}

/** 取最近一场真实战报详情；无历史/失败都返回 null，页面写诚实空态。 */
export async function fetchLatestBattleCausal(): Promise<BattleCausalView> {
  try {
    const list = await myBattleHistory(1)
    const first = list.items?.[0]
    if (!first?.id) return normalizeBattleCausal(null)
    const detail = await myBattleDetail(first.id)
    return normalizeBattleCausal(detail)
  } catch {
    return normalizeBattleCausal(null)
  }
}
