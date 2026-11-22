// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
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

/**
 * 服务端 `domain/battle.go` 的 `SpeedAnchorPermille`：满分锚点 = 理论清怪秒数 × 9。
 * 客户端要提前告诉玩家还剩多少"保分窗口"，必须用同一个系数；
 * 这里用跨端测试解析 Go 源码钉住，避免两份定义悄悄漂移。
 */
export const SPEED_ANCHOR_PERMILLE = 9000n

/** 进入钳分区前多少秒开始预警（纯 UI 策略，不改变得分规则）。 */
export const TIME_BUDGET_WARN_SEC = 5

export interface ScoreTimeBudget {
  /** 打满分数所需秒数（由关卡波次推导，服务端同款口径） */
  anchorSec: number
  elapsedSec: number
  remainingSec: number
  /** 是否已进入钳分区或临近钳分 */
  warning: boolean
  /** 是否已越过锚点、得分开始被裁剪 */
  clamped: boolean
  text: string
}

interface WaveForBudget {
  spawns?: ReadonlyArray<{ count?: number; interval?: number; delay?: number }>
}

/**
 * 从关卡波次推导剩余保分窗口。
 *
 * 与服务端 `theoreticalClearSec` / `speedAnchorSec` 同口径：
 * 每波取 `delay + (count-1)*interval` 的最大值求和并向下取整到秒，再乘 0.9。
 * 没有波次配置时返回 null —— 不允许拿本地硬编码秒数冒充服务端口径。
 */
export function scoreTimeBudget(
  level: { waves?: ReadonlyArray<WaveForBudget> },
  elapsedMs: number,
): ScoreTimeBudget | null {
  const waves = level?.waves
  if (!waves || !waves.length) return null
  let theoreticalMs = 0
  for (const wave of waves) {
    let waveEndMs = 0
    for (const spawn of wave.spawns ?? []) {
      const count = Math.max(0, Number(spawn.count ?? 0))
      const interval = Math.max(0, Number(spawn.interval ?? 0))
      const delay = Math.max(0, Number(spawn.delay ?? 0))
      const end = delay + Math.max(0, count - 1) * interval
      if (end > waveEndMs) waveEndMs = end
    }
    theoreticalMs += waveEndMs
  }
  const fullSec = Math.floor(theoreticalMs / 1000)
  const anchorSec = Math.floor((fullSec * Number(SPEED_ANCHOR_PERMILLE)) / 1000)
  if (!Number.isFinite(anchorSec) || anchorSec <= 0) return null

  const elapsedSec = Math.max(0, Math.floor(Number(elapsedMs) / 1000))
  const remainingSec = Math.max(0, anchorSec - elapsedSec)
  const clamped = elapsedSec >= anchorSec
  const warning = clamped || remainingSec <= TIME_BUDGET_WARN_SEC
  // ⚠️ 服务端 scoreCapFor 是「快于锚点才裁、到锚点才满分」的负向闸：
  // 锚点之前一直处于钳分区，剩余秒数是「距离免钳分还有多久」，
  // 绝不能写成「保分窗口」——那会让玩家以为慢打会扣分。
  const text = clamped
    ? '已免钳分'
    : warning
      ? `钳分中 · 距满分 ${remainingSec}s（即将免钳分）`
      : `钳分中 · 距满分 ${remainingSec}s`
  return { anchorSec, elapsedSec, remainingSec, warning, clamped, text }
}
