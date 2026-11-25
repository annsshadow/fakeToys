// R36：Wiki 合成技能机制表的**纯显示辅助**。
// 合成技能与基础技能同为 SeedSkill：格式化一律复用 @/skillMechanicsText，
// 不写第二套；AI 结论由公开 /config 的 composite_skills 确定性聚合。
// 诚实边界：这些只是配置基值展示，不参与结算 / replay_hash / score。

import {
  MECHANISM_EMPTY,
  kindText,
  type SkillMechanicsRow,
} from "@/skillMechanicsText";

export { MECHANISM_EMPTY };
export type CompositeMechanicsRow = SkillMechanicsRow;

/** 合法有限正整数 → N 级解锁；0 / 缺失 / 非法一律归空态。 */
export function compositeUnlockText(n: unknown): string {
  if (typeof n !== "number" || !Number.isInteger(n) || n <= 0) return MECHANISM_EMPTY;
  return `${n} 级解锁`;
}

/** 依据：与服务端 SeedSkill / SeedCompositeSkills 同源的公开配置。 */
export const COMPOSITE_AI_BASIS =
  "依据：公开 /api/v1/config 的 composite_skills（与服务端 SeedSkill / SeedCompositeSkills 同源）；格式化仅展示，不改结算";

/** 红线：配置基值不是权威战斗数据，实战/回放以引擎与 replay_hash 为准。 */
export const COMPOSITE_AI_HONESTY =
  "辅助展示非权威战斗数据，展示为配置基值，实际效果以引擎结算为准；权威回放以 replay_hash 为准";

/** CTA：指向 Wiki 已有的合成技能锚点，具体 href 由页面 urlFor 生成。 */
export const COMPOSITE_AI_CTA = "查看合成技能 →";

/** 由 cfg.composite_skills 确定性聚合 AI 结论，不依赖选中行、不手填。 */
export function buildCompositeMechanicsAi(rows: readonly CompositeMechanicsRow[]): string {
  let active = 0;
  let multi = 0;
  for (const row of rows) {
    if (kindText(row.kind) === "主动") active += 1;
    const p = row.pierce;
    const a = row.aoe_radius;
    const c = row.chain;
    const positive = (v: unknown) => typeof v === "number" && Number.isInteger(v) && v > 0;
    if (positive(p) || positive(a) || positive(c)) multi += 1;
  }
  return `共 ${rows.length} 个合成技能：主动 ${active} 个，多目标 ${multi} 个`;
}
