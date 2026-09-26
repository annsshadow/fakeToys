/**
 * 分数规则 —— 客户端侧的唯一定义点。
 *
 * ⚠️ 这里的数字以前散落在 `engine.ts` 的 5 处（加分 ×2、记事件 ×2、
 * 地形充能 ×1）与测试的 2 处。而 `star_targets` 是**服务端算的**：
 *
 *   Go 侧：levelgen 用 `scorePerDamageUnit` 等常量算 star_targets
 *   TS 侧：engine.ts 用自己那份 /100、500/5000 累加实际得分
 *
 * 两处一旦漂移，玩家拿到的星级就与实际表现对不上，而
 * **没有任何测试会发现** —— 因为两端各自都「自洽」。
 * 这与 README 工程约束 4（同名不同义）是同一类问题：
 * 契约不是靠约定维持的，是靠**一个可核对的来源**维持的。
 *
 * 现在：
 *   - 唯一定义在 `DEFAULT_SCORE_RULES`（本文件）
 *   - 服务端在 `/config` 的 `score_rules` 里下发同一份（`domain.DefaultScoreRules`）
 *   - 测试用 `TestScoreRulesMatchServerContract` 断言两边逐位一致
 *
 * 改数值时必须三处同步，而测试会强制这一点。
 */
export interface ScoreRules {
  /** 每多少伤害算 1 分 */
  perDamageUnit: bigint
  /** 击杀普通敌人的分数 */
  onKillNormal: bigint
  /** 击杀 BOSS 的分数 */
  onKillBoss: bigint
  /** 一/二/三星相对理论满分的占比（千分比） */
  starTargetRatio: readonly [bigint, bigint, bigint]
  /** 打满理论满分所需的最短秒数，速率裁剪的锚点 */
  scoreFullAtSec: bigint
}

/**
 * DEFAULT_SCORE_RULES 必须与 server/internal/domain/levelgen.go 的
 * DefaultScoreRules() 逐位一致 —— 由本文件底部的测试强制。
 */
export const DEFAULT_SCORE_RULES: ScoreRules = {
  perDamageUnit: 100n,
  onKillNormal: 500n,
  onKillBoss: 5000n,
  starTargetRatio: [600n, 850n, 980n],
  scoreFullAtSec: 60n,
}

/** 把夹具里的 snake_case 结构转成 ScoreRules。 */
export function scoreRulesFromServer(raw: {
  per_damage_unit: number
  on_kill_normal: number
  on_kill_boss: number
  star_target_ratio: number[]
  score_full_at_sec: number
}): ScoreRules {
  return {
    perDamageUnit: BigInt(raw.per_damage_unit),
    onKillNormal: BigInt(raw.on_kill_normal),
    onKillBoss: BigInt(raw.on_kill_boss),
    starTargetRatio: [
      BigInt(raw.star_target_ratio[0]),
      BigInt(raw.star_target_ratio[1]),
      BigInt(raw.star_target_ratio[2]),
    ],
    scoreFullAtSec: BigInt(raw.score_full_at_sec),
  }
}

/** 累计伤害分。分数体系里唯一由伤害贡献的部分。 */
export function damageScore(rules: ScoreRules, totalDamage: bigint): bigint {
  if (totalDamage <= 0n) return 0n
  return totalDamage / rules.perDamageUnit
}

/** 击杀分。BOSS 与杂兵不同。 */
export function killScore(rules: ScoreRules, isBoss: boolean): bigint {
  return isBoss ? rules.onKillBoss : rules.onKillNormal
}
