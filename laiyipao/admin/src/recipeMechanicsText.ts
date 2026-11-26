// R38：Wiki 合成配方（recipes）的**纯显示辅助**。
// 口径只对齐公开 /config 的 SeedRecipe 四字段与 SeedSkill 名称，绝不参与熔炼结算。
// MECHANISM_EMPTY 从 @/enemyMechanicsText 同源复用，禁止第二份字面量。
export { MECHANISM_EMPTY } from '@/enemyMechanicsText'
import { MECHANISM_EMPTY } from '@/enemyMechanicsText'

/** 全部格式化入参一律 unknown；脏值统一空态，禁止 NaN/undefined 泄漏。 */
function isPositiveInteger(v: unknown): v is number {
  return typeof v === 'number' && Number.isFinite(v) && Number.isInteger(v) && v > 0
}

interface RecipeLike {
  output?: unknown
  a?: unknown
  b?: unknown
  out_tier?: unknown
}

function asRecipe(v: unknown): RecipeLike | null {
  return v && typeof v === 'object' && !Array.isArray(v) ? (v as RecipeLike) : null
}

function unknownSkill(id: number): string {
  return `（未知技能#${id}）`
}

/**
 * 从 Map / Record / 函数三种映射里查技能名。
 * 映射抛错也只回退「未知技能#N」，不把异常文本泄漏给玩家。
 */
function skillNameOf(id: number, skillNameById: unknown): string {
  try {
    let name: unknown
    if (skillNameById instanceof Map) name = skillNameById.get(id)
    else if (typeof skillNameById === 'function') name = (skillNameById as (id: number) => unknown)(id)
    else if (skillNameById && typeof skillNameById === 'object') name = (skillNameById as Record<string, unknown>)[String(id)]
    return typeof name === 'string' && name.length > 0 ? name : unknownSkill(id)
  } catch {
    return unknownSkill(id)
  }
}

/**
 * 材料 A + 材料 B 的逐字技能名。
 * a/b 非正整数时返回空态；未知 id 逐字标注，绝不猜测。
 */
export function recipeMaterialsText(r: unknown, skillNameById: unknown): string {
  const recipe = asRecipe(r)
  if (!recipe || !isPositiveInteger(recipe.a) || !isPositiveInteger(recipe.b)) return MECHANISM_EMPTY
  // name 一定来自纯格式化；未知 id 逐字显示，不给玩家造一个不存在的技能名。
  return `${skillNameOf(recipe.a, skillNameById)} + ${skillNameOf(recipe.b, skillNameById)}`
}

/** out_tier 仅有限正整数有效。 */
export function tierText(v: unknown): string {
  return isPositiveInteger(v) ? `${v} 阶` : MECHANISM_EMPTY
}

/** 只读数组保护；三个入参都不修改，两次同输入严格相等。 */
function readonlyArray(v: unknown): readonly unknown[] {
  return Array.isArray(v) ? v : []
}

/**
 * AI 结论：共 {n} 条配方：二阶 {t2} 条、三阶 {t3} 条。
 * 只统计 output/a/b/out_tier 均合法正整数的配方；t2/t3 严格等于 2/3。
 * skills/compositeSkills 仅用于名称映射候选，不参与计数。
 */
export function buildRecipeMechanicsAi(
  recipes: unknown,
  skills: unknown,
  compositeSkills: unknown,
): string {
  void skills
  void compositeSkills
  let total = 0
  let tier2 = 0
  let tier3 = 0
  for (const item of readonlyArray(recipes)) {
    const recipe = asRecipe(item)
    if (!recipe || !isPositiveInteger(recipe.output) || !isPositiveInteger(recipe.a) || !isPositiveInteger(recipe.b) || !isPositiveInteger(recipe.out_tier)) continue
    total += 1
    if (recipe.out_tier === 2) tier2 += 1
    if (recipe.out_tier === 3) tier3 += 1
  }
  return `共 ${total} 条配方：二阶 ${tier2} 条、三阶 ${tier3} 条`
}

export const RECIPE_AI_BASIS = '依据：公开 /api/v1/config 的 recipes（与服务端 SeedRecipe / SeedRecipes 同源）；仅展示配方，不改熔炼结算'
export const RECIPE_AI_HONESTY = '能否熔炼以游戏内材料拥有状态为准；配置基值非权威战斗数据，实际效果以引擎结算为准，权威回放以 replay_hash 为准'
export const RECIPE_AI_CTA = '查看合成配方 →'
