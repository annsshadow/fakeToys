/**
 * 技能升级规则 —— 客户端侧的唯一定义点。
 *
 * ## 之前 `user_skills.level` 是一列永远为 1 的数据
 *
 * 被读进 build 快照、从无写入、也没有任何升级端点。
 * 而服务端 `HitInput.SkillDamage` 的注释曾声称伤害公式含「等级系数」——
 * 一个不存在的因子。那条注释比功能先到了两年。
 *
 * ## 接线要三件事同时成立，缺一不可
 *
 *  1. **有升级路径**（`POST /me/skills/:id/upgrade`）
 *  2. **等级进公式**（本文件 `skillBaseDamageAtLevel`）
 *  3. **两端一致**（level 已在 build 快照里，实战与重放走同一个函数）
 *
 * 只做 2 而不做 1，等级恒为 1、系数恒等于 1 ——
 * 功能「看起来接好了」但完全无效。这是本项目反复栽过的形状
 * （README「已知边界」里的「只写不读」「纯装饰」两条）。
 *
 * ## 唯一真源在 Go 侧
 *
 * - Go：`server/internal/domain/skillrules.go` 的 `DefaultSkillRules()`
 * - 下发：`/config` 的 `skill_rules`、契约夹具的 `skill_rules`
 * - 本文件的 `DEFAULT_SKILL_RULES` 只是**离线兜底**（config 未到达时），
 *   由 `TestSkillRulesMatchServerContract` 断言两者逐字段一致
 *
 * 照 `score.ts` 的先例：分数规则当初散落两端 5 处，
 * 漂移时表现为「两端各自自洽、星级却对不上」，没有任何测试会发现。
 * 同一个坑不踩第二次。
 */
export interface SkillRules {
  /** 等级上限 */
  maxLevel: number
  /** 每升一级的伤害加成（千分比） */
  coefPermille: number
  /** 费用基数；实际费用 = baseCost × 当前等级 */
  baseCost: number
}

/**
 * DEFAULT_SKILL_RULES 必须与 `domain.DefaultSkillRules()` 逐字段一致
 * —— 由 `skill.test.ts` 的 TestSkillRulesMatchServerContract 强制。
 */
export const DEFAULT_SKILL_RULES: SkillRules = {
  maxLevel: 10,
  coefPermille: 50,
  baseCost: 100,
}

/** 把服务端下发的 snake_case 结构转成 SkillRules。 */
export function skillRulesFromServer(raw: {
  max_level: number
  coef_permille: number
  base_cost: number
}): SkillRules {
  return {
    maxLevel: raw.max_level,
    coefPermille: raw.coef_permille,
    baseCost: raw.base_cost,
  }
}

/**
 * clampLevel 把任意输入夹到合法等级区间。
 *
 * 与 Go 侧 `SkillRules.ClampLevel` 行为一致。
 *
 * 存在的理由：`build.skills[].level` 直通数据库。
 * 如果服务端某天漏了夹紧（或旧版本快照里存着越界值），
 * 客户端会算出 4900‰ 的伤害加成 —— 而**没有任何东西会报错**，
 * 表现只是「这个技能伤害高得离谱」，要从战报里反推才看得出来。
 * 与其信任上游，不如让读路径必然产出合法值。
 */
export function clampLevel(rules: SkillRules, level: number): number {
  if (!Number.isFinite(level) || level < 1) return 1
  const l = Math.floor(level)
  if (rules.maxLevel > 0 && l > rules.maxLevel) return rules.maxLevel
  return l
}

/**
 * coefPermilleAt 返回该等级的伤害系数（千分比，1000 = 无加成）。
 *
 * ⚠️ **level 1 → 1000‰ 是精确的恒等变换，不是近似。**
 *
 *     coef(1) = 1000 + (1-1)*50 = 1000
 *     damage = base * 1000 / 1000 = base      // 整除，无余数
 *
 * 这条性质是**兼容性要求**，不是巧合：
 * 升级功能上线前所有 `user_skills.level` 恒为 1，
 * 所以所有**存量战报的重放哈希必须逐位不变**。
 *
 * 哈希为什么会在意：`replayHash()` 把 `baseDamage` 拼进锚点
 * （`engine.ts` 的 `${s.slot}:${s.skillId}:${s.baseDamage}:…`）。
 * 若基线取成 1000‰ 以外的值，每一个历史战报都会静默失配 ——
 * 而失配表现为「验真失败」，看起来像作弊，
 * 实际是上线了一个数学常数。
 *
 * 满级（10）= 1000 + 9*50 = 1450‰，即 +45% 伤害。
 */
export function coefPermilleAt(rules: SkillRules, level: number): number {
  return 1000 + (clampLevel(rules, level) - 1) * rules.coefPermille
}

/**
 * skillBaseDamageAtLevel 返回该等级下的技能基础伤害。
 *
 * ## 刻意「烘进 baseDamage」而不是给 EquippedSkill 加 level 字段
 *
 * 两个方案都能算对伤害，但只有一个能保住 I-6：
 *
 * - 烘进 `baseDamage`（本方案）：等级自动进入 `replayHash()` 的锚点，
 *   于是**不同等级的构筑哈希不同**。验真时「哈希不符」能立刻指出
 *   「你重放用的等级和当时不一样」。
 * - 加 `level` 字段在 `fire()` 里乘：等级**不进哈希**。
 *   两个不同等级的构筑可能算出同一份哈希，
 *   验真通过但伤害对不上 —— 哈希在说谎。
 *
 * 与 `activeSlots` 缺省必须是 `ACTIVE_SLOTS = 4` 同一个道理：
 * 凡是能改变战斗结果的输入，都必须落在哈希锚点里，
 * 否则 I-6 的「验真」就退化成「验一个玩家可以随便填的东西」。
 *
 * ## 整除方向
 *
 * `(base * coef) / 1000` 而不是 `base * coef / 1000n` 之外的其他写法，
 * 是因为伤害是 bigint 而系数是 int；先乘后除保证**不丢精度**。
 * 反过来（先除后乘）会在系数 < 1000 时向下取整到 0。
 */
export function skillBaseDamageAtLevel(
  rules: SkillRules,
  baseDamage: bigint,
  level: number,
): bigint {
  const coef = BigInt(coefPermilleAt(rules, level))
  if (coef === 1000n) return baseDamage // 恒等快路径：等级 1 逐位不变
  return (baseDamage * coef) / 1000n
}

/**
 * upgradeCostFrom 返回从 level 升到 level+1 的金币花费。
 *
 * 满级返回 0 —— 调用方**必须**自己先判 `isMaxLevel`，
 * 否则「满级还扣 0 费成功」会让玩家以为升级生效了。
 * 返回 0 只是为了让「不需要花钱」这个信号在两端一致。
 */
export function upgradeCostFrom(rules: SkillRules, level: number): number {
  const l = clampLevel(rules, level)
  if (rules.maxLevel > 0 && l >= rules.maxLevel) return 0
  return rules.baseCost * l
}

/** 是否已满级。 */
export function isMaxLevel(rules: SkillRules, level: number): boolean {
  return clampLevel(rules, level) >= rules.maxLevel
}
