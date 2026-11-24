// R31：Wiki 反应机制字段的**纯显示辅助**。
// 把服务端 /config 下发的三个机制字段转成玩家可读文案；零/缺失/非法值
// 统一返回 MECHANISM_EMPTY，绝不输出 "0" / "NaN" / "undefined" / 空串。
// 诚实边界：这些只是既有机制参数的展示，不参与结算 / replay_hash / score。

export const MECHANISM_EMPTY = "—"

/** 正数（有限数）原样返回，其余（0/null/undefined/负数/NaN/∞）返回 null。 */
function positive(v: number | null | undefined): number | null {
  if (v == null || typeof v !== "number" || !Number.isFinite(v) || v <= 0) return null
  return v
}

/** spread_stacks：层数传播量，正整数显示「传播 N 层」。 */
export function spreadStacksText(v: number | null | undefined): string {
  const n = positive(v)
  if (n === null || !Number.isInteger(n)) return MECHANISM_EMPTY
  return `传播 ${n} 层`
}

/** armor_shred_permille：削甲比例，单位是千分比，直接显示 N‰（150 → 150‰，即 15%）。 */
export function armorShredText(v: number | null | undefined): string {
  const n = positive(v)
  if (n === null) return MECHANISM_EMPTY
  return `${n}‰`
}

/** knockback：击退位移，服务端定点 ×1000，显示像素 = 值 / 1000（4000 → 4px）。 */
export function knockbackText(v: number | null | undefined): string {
  const n = positive(v)
  if (n === null) return MECHANISM_EMPTY
  const px = Math.round((n / 1000) * 1000) / 1000
  return `${px}px`
}
