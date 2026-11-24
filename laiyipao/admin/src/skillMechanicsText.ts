// R32：Wiki 技能作战机制字段的**纯显示辅助**。
// 把服务端 /config 下发的 kind/cooldown_ms/pierce/aoe_radius/chain 转成玩家可读文案；
// 0 / 缺失 / 非法值统一返回 MECHANISM_EMPTY，绝不输出 "0" / "NaN" / "undefined" / 空串。
// 诚实边界：这些只是既有机制参数的展示，不参与结算 / replay_hash / score。

export const MECHANISM_EMPTY = "—"

/** 仅有限正整数返回该数，其余（0/负/小数/NaN/∞/非数字）返回 null。 */
function positiveInt(v: unknown): number | null {
  if (typeof v !== "number" || !Number.isInteger(v) || v <= 0) return null
  return v
}

/** kind：active → 主动，passive → 被动，其它/空/非字符串 → 未知类型（不伪造空值）。 */
export function kindText(kind: unknown): string {
  if (kind === "active") return "主动"
  if (kind === "passive") return "被动"
  return "未知类型"
}

/** cooldown_ms：毫秒转秒；整数秒「N秒」，非整数最多一位小数去尾随0；0/非法 → —。 */
export function cooldownText(ms: unknown): string {
  if (typeof ms !== "number" || !Number.isFinite(ms) || ms <= 0) return MECHANISM_EMPTY
  const sec = ms / 1000
  if (Number.isInteger(sec)) return sec + "秒"
  const one = Math.round(sec * 10) / 10
  return one + "秒"
}

/** pierce：正整数 → 穿透 N，其余 → —。 */
export function pierceText(n: unknown): string {
  const v = positiveInt(n)
  return v === null ? MECHANISM_EMPTY : `穿透 ${v}`
}

/** aoe_radius：像素半径整数，正整数 → 范围 N（不做换算/不追加数字），其余 → —。 */
export function aoeRadiusText(n: unknown): string {
  const v = positiveInt(n)
  return v === null ? MECHANISM_EMPTY : `范围 ${v}`
}

/** chain：弹射次数整数，正整数 → 连锁 N，其余 → —。 */
export function chainText(n: unknown): string {
  const v = positiveInt(n)
  return v === null ? MECHANISM_EMPTY : `连锁 ${v}`
}

/** AI 结论所需的最小行形状（只读展示，不参与任何结算）。 */
export interface SkillMechanicsRow {
  kind: unknown
  pierce: unknown
  aoe_radius: unknown
  chain: unknown
}

/** AI 跳转合成技能的固定 CTA（指向 Wiki 已有 #composite 锚点）。 */
export const SKILL_AI_CTA = "查看合成技能 →"

/** 由 cfg.skills 确定性聚合出 AI 结论，不依赖「当前选中行」。 */
export function buildSkillMechanicsAi(skills: readonly SkillMechanicsRow[]): string {
  let active = 0
  let passive = 0
  let multi = 0
  for (const row of skills) {
    const kind = kindText(row.kind)
    if (kind === "主动") active += 1
    if (kind === "被动") passive += 1
    const p = positiveInt(row.pierce)
    const a = positiveInt(row.aoe_radius)
    const c = positiveInt(row.chain)
    if (p !== null || a !== null || c !== null) multi += 1
  }
  return `共 ${skills.length} 个技能：主动 ${active} 个，被动 ${passive} 个，多目标 ${multi} 个`
}
