/**
 * R33 Wiki 敌人战斗机制展示文案。
 *
 * 全部入参为 unknown：服务端配置即使临时缺字段或给脏值，
 * 表格也只能显示破折号，绝不把 NaN / undefined / [object Object] 泄漏给玩家。
 * 这里只做展示格式化，不参与任何战斗计算或结算。
 */

export const MECHANISM_EMPTY = '—'

function isPositiveFinite(v: unknown): v is number {
  return typeof v === 'number' && Number.isFinite(v) && v > 0
}

function isPositiveInteger(v: unknown): v is number {
  return typeof v === 'number' && Number.isInteger(v) && v > 0
}

/** 除以 1000 后展示：整数不带小数，非整数最多一位，Math.round 消除浮点噪声。 */
function perThousandText(v: unknown, unit: string): string {
  if (!isPositiveFinite(v)) return MECHANISM_EMPTY
  const n = Math.round((v / 1000) * 10) / 10
  return `${n}${unit}`
}

/** speed 为定点 x1000 的像素/秒。 */
export function speedText(v: unknown): string {
  return perThousandText(v, '像素/秒')
}

/** attack_interval 为毫秒（引擎会按 tuning 倍率缩放，这里展示配置基值）。 */
export function attackIntervalText(v: unknown): string {
  return perThousandText(v, '秒')
}

/** armor 为千分比，绝不按百分比显示（那会夸大 10 倍）。 */
export function armorText(v: unknown): string {
  return isPositiveInteger(v) ? `${v}‰` : MECHANISM_EMPTY
}

/** 以下四项为引擎逻辑单位原值，不换算、不加单位词。 */
export function shieldHpText(v: unknown): string {
  return isPositiveInteger(v) ? String(v) : MECHANISM_EMPTY
}

export function attackText(v: unknown): string {
  return isPositiveInteger(v) ? String(v) : MECHANISM_EMPTY
}

export function attackRangeText(v: unknown): string {
  return isPositiveInteger(v) ? String(v) : MECHANISM_EMPTY
}

export function flyHeightText(v: unknown): string {
  return isPositiveInteger(v) ? String(v) : MECHANISM_EMPTY
}

/** 仅严格 true 显示「是」；false 与一切非法值都显示空态。 */
export function boolText(v: unknown): string {
  return v === true ? '是' : MECHANISM_EMPTY
}

export const ENEMY_AI_CTA = '查看敌人列表 →'

/** AI 概览只依赖公开配置的最小字段集，便于纯函数测试。 */
export interface EnemyMechanicsRow {
  is_boss: unknown
  category: unknown
  burrow: unknown
  armor: unknown
}

export function buildEnemyMechanicsAi(enemies: readonly EnemyMechanicsRow[]): string {
  const total = enemies.length
  const boss = enemies.filter((e) => e.is_boss === true).length
  const fly = enemies.filter((e) => e.category === 'flying').length
  const burrow = enemies.filter((e) => e.burrow === true).length
  const armor = enemies.filter((e) => armorText(e.armor) !== MECHANISM_EMPTY).length
  return `共 ${total} 种敌人：首领 ${boss} 种，飞行 ${fly} 种，潜地 ${burrow} 种，带护甲 ${armor} 种`
}
