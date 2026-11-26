// R39：Wiki 皮肤图鉴 skins 与结算掉落 drop_rates 的**纯显示辅助**。
// 口径只对齐公开 /api/v1/config 的 SeedSkin 六字段与 DropRates 六键配置基值，
// 不翻译机制型 passive code、不把配置基值写成百分比/千分比、不参与结算。
// MECHANISM_EMPTY 从 @/enemyMechanicsText 同源复用，禁止第二份字面量。
export { MECHANISM_EMPTY } from '@/enemyMechanicsText'
import { MECHANISM_EMPTY } from '@/enemyMechanicsText'

const RARITY_LABEL: Record<string, string> = {
  rare: '稀有',
  epic: '史诗',
  legendary: '传说',
}

function nonEmptyText(v: unknown): string | null {
  return typeof v === 'string' && v.trim().length > 0 ? v : null
}

/** 稀有度三档逐字映射；未知合法档原样，脏值统一空态。 */
export function rarityText(v: unknown): string {
  const s = nonEmptyText(v)
  if (s === null) return MECHANISM_EMPTY
  return RARITY_LABEL[s] ?? s
}

/** 机制型词条 code 原样展示，不翻译、不扩展描述。 */
export function passiveText(v: unknown): string {
  return nonEmptyText(v) ?? MECHANISM_EMPTY
}

/** 仅有限整数合法；0 也是合法配置基值。绝不追加 %/‰/单位。 */
export function dropRatesText(v: unknown): string {
  if (typeof v !== 'number' || !Number.isFinite(v) || !Number.isInteger(v) || v < 0) return MECHANISM_EMPTY
  return String(v)
}

/** 只读输入形状：与服务端 SeedSkin 六字段对齐。 */
export interface SkinMechanicsRow {
  id: number
  code: string
  name: string
  rarity: string
  passive: string
  descr: string
}

const DROP_KEYS = ['coin_base', 'coin_per_star', 'coin_per_kill', 'reaction_bonus', 'keys_on_three_star', 'energy_on_win'] as const

function readonlyArray(v: unknown): readonly unknown[] {
  return Array.isArray(v) ? v : []
}

function asRow(v: unknown): SkinMechanicsRow | null {
  return v && typeof v === 'object' && !Array.isArray(v) ? (v as SkinMechanicsRow) : null
}

/**
 * 确定性聚合：共 {total} 款皮肤：稀有 {rare}、史诗 {epic}、传说 {legendary}；掉落 {present}/6 项齐备。
 * total 统计 id 为正整数的行；三档只统计 rarityText 命中三映射的行；掉落按六键逐项校验。
 */
export function buildSkinDropMechanicsAi(skins: readonly SkinMechanicsRow[], dropRates: unknown): string {
  let total = 0
  let rare = 0
  let epic = 0
  let legendary = 0
  for (const item of readonlyArray(skins)) {
    const row = asRow(item)
    if (!row || typeof row.id !== 'number' || !Number.isInteger(row.id) || row.id <= 0) continue
    total += 1
    if (row.rarity === 'rare') rare += 1
    else if (row.rarity === 'epic') epic += 1
    else if (row.rarity === 'legendary') legendary += 1
  }
  const rates = dropRates && typeof dropRates === 'object' && !Array.isArray(dropRates)
    ? (dropRates as Record<string, unknown>)
    : {}
  let present = 0
  for (const key of DROP_KEYS) {
    if (dropRatesText(rates[key]) !== MECHANISM_EMPTY) present += 1
  }
  return `共 ${total} 款皮肤：稀有 ${rare}、史诗 ${epic}、传说 ${legendary}；掉落 ${present}/6 项齐备`
}

export const SKIN_DROP_AI_BASIS = '依据：公开 /api/v1/config 的 skins（与服务端 SeedSkin / SeedSkins 同源）与 drop_rates（与服务端 DropRates / DefaultDropRates 同源）；仅展示配置基值'
export const SKIN_DROP_AI_HONESTY = '皮肤被动与掉落均为配置基值，实际结算与获得以游戏内为准；权威回放以 replay_hash 为准'
export const SKIN_DROP_AI_CTA = '查看皮肤图鉴 →'
