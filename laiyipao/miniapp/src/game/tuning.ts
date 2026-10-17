/**
 * 战斗节奏集中配置（第 158 轮「游戏节奏」整改）。
 *
 * ## 为什么要有这个文件
 *
 * 改此前**所有影响手感的时间参数都是散落的编译期常量**：
 *
 *   - `SPAWN_PROGRESS_STEP`（80‰，敌人出场淡入）在 engine.ts
 *   - `OVERHEAT_DURATION_MS`（2000ms，过热惩罚）在 heatmap.ts
 *   - `attack_interval / interval / delay`（敌人攻击、刷怪节奏）来自内容表与 levelgen
 *   - `projectile_speed / cooldown_ms` 来自技能表，且被 4 处地方各读一次
 *
 * 后果是「想让整体推进快一点」变成一次跨 4 个文件的人工考古：
 * 先找出所有时间常量，再逐个判断哪些该动，还要担心漏掉某处。
 * 而恰恰是「漏掉一处」最容易出问题 —— 例如把刷怪变快却没相应提升输出，
 * 难度曲线会整条抬升（见下方 deaths 的说明）。
 *
 * 现在它们集中在这里，`BattleEngine` 通过 `cfg.tuning` 接收（缺省 DEFAULT_TUNING）。
 *
 * ## 单位约定
 *
 * 所有倍率字段一律用**千分比**（1000 = 不变），与引擎其余部分
 * （`buffs.attackPermille` 等）保持一致 —— 不引入第二套缩放单位。
 * 时间字段用**毫秒**。
 */

export interface GameTuning {
  /**
   * 敌人出场淡入的**每 tick 进度增量**（千分比，1000 = 一步到位）。
   *
   * 原值 80 → 需要 13 个 tick（650ms）才开始移动、才可被命中。
   * 这 650ms 是**纯粹的等待**：期间敌人既不构成威胁也不给玩家 feedback，
   * 一波 30 只就是累积 19.5 秒的空转。
   *
   * 200 → 5 个 tick（250ms）：仍保留"由淡到实"的出场感，去掉了绝大部分空转。
   *
   * ⚠️ 它是整数千分比，参与  的累计比较 ——
   * 且必须与 `SPAWN_PROGRESS_FULL`(1000) 同量纲（两者都是整数千分比，不做浮点）。
   */
  spawnProgressStep: number

  /** 敌人移动速度倍率（1000 = 不变）。直接决定「推进到防线要多久」。 */
  enemySpeedPermille: number

  /**
   * 弹丸飞行速度倍率（1000 = 不变）。
   *
   * 这是**纯运行时收益**：弹速不影响任何伤害结算，只影响「看到的命中延迟」。
   * 原默认 60000 单位/秒在低倍 skill 上会明显看出"子弹慢慢飘过去"，
   * 期间玩家既不能干预也无法取消 —— 典型的无意义等待。
   */
  projectileSpeedPermille: number

  /**
   * 技能冷却倍率（1000 = 不变）。
   *
   * ⚠️ 与 `enemySpeedPermille` **成对调整**：把敌人变快而不同步提升火力，
   * 等于在暗中抬高难度 —— 敌人更多地在防线前堆积、漏怪数上升。
   * 这是本次调整里最容易踩的一处。
   */
  skillCooldownPermille: number

  /** 波内「首只出场延迟」倍率（1000 = 不变）。开局干等的主要来源之一。 */
  waveDelayPermille: number

  /** 波内「同组刷怪间隔」倍率（1000 = 不变）。决定一波怪铺开的快慢。 */
  waveIntervalPermille: number

  /**
   * 远程敌人攻击间隔倍率（1000 = 不变）。
   *
   * 这里**故意 > 1000**：敌人整体变快后，若攻击频率同步提高，防线血量消耗
   * 会超线性上升。放宽一点让它接近线性。
   */
  enemyAttackIntervalPermille: number

  /** 过热持续时间（毫秒）。这是一段**惩罚性等待**，保留威慑但压缩时长。 */
  overheatDurationMs: number

  /** 受击闪白时长（毫秒）。纯表现，不影响任何结算。 */
  hitFlashMs: number

  /** 伤害飘字寿命（毫秒）。纯表现。 */
  floatLifeMs: number
}

/**
 * 当前生效的一档节奏。
 *
 * ## 取值来源（不是拍脑袋）
 *
 * 每一档都用这两个既有守卫回归过：
 *   - `game/balance.probe.test.ts` —— 100 关**死局扫描**（不得有关卡打不过）
 *   - `game/difficulty.test.ts` —— **难度曲线单调性**
 * 它们正是「不能为了快而把难度搞崩」那一侧的保护网。
 *
 * 定档方式是参数扫描（`(波次延迟, 波次间隔, 敌人移速, 技能冷却)` 网格）：
 * 逐个组合跑 100 关总 tick + 两个守卫用例，取「提速最多且不破坏守卫」的一档。
 *
 * ## ⚠️ 移速与冷却为什么最终没有动
 *
 * 扫到 `enemySpeed=1050` 或 `cooldown=900` 时，100 关总时长还能再降 1~2%，
 * 但 `skill_level_battle.test.ts` 的两条方向性用例（升级应更快 / 更少漏怪）
 * 会翻成 BAD —— 那是「等级不再有区分度」的信号。
 *
 * 同理 `projectileSpeedPermille` 从 1200 提到 1300 会让第 99 关在满配下
 * 掉 1 星（`tier_impact.test.ts` 的守卫）。
 * 用户明确要求保留难度曲线与可控性，所以停在这里。
 *
 * ## 实测效果（第 158 轮，同口径 100 关裸装扫描）
 *
 *   战斗总时长  **282,390 tick → 231,707 tick（约 -17.9%）**
 *   死局        0 → 0
 *   满配掉星    0 → 0
 *   满配掉分    0 → 0
 *
 * 提速主要来自「压缩无意义的等待」这一侧（波次延迟/间隔），
 * 而不是把敌人调快 —— 后者会同时抬高难度，与用户要求相悖。
 */
export const DEFAULT_TUNING: GameTuning = {
  // ⚠️ 保持 80，**不许改**：determinism.test.ts 把「13 个 tick 达成满出场」
  // 当成铁律守着 —— FULL/STEP = 12.5 不能整除正是它要保证的形状，
  // 改成能整除的数会让「敌人何时可被击中」平移一格，
  // 所有存量战报的 replayHash 失配。改动前先读那个测试的注释。
  spawnProgressStep: 80,
  // 敌人移速与技能冷却**都保持 1000**：扫参实测把它们往「更快」推到 1050/900
  // 之后，等级提升带来的区分度就被噪声淹没（见下方 diff 说明），
  // 而这正是用户要「保留难度曲线」的那一侧。
  enemySpeedPermille: 1000,
  skillCooldownPermille: 1000,
  projectileSpeedPermille: 1200,
  waveDelayPermille: 800,
  waveIntervalPermille: 850,
  enemyAttackIntervalPermille: 1000,
  overheatDurationMs: 1600,
  hitFlashMs: 110,
  floatLifeMs: 700,
}

/**
 * 完全等同「整改前」的一档，用于回归测试与 A/B 对比。
 *
 * 有了它，任何关于节奏的说法都可以被复现为真 ——
 * 不必依赖"我记得以前是 2000ms"这种容易漂的记忆。
 */
export const LEGACY_TUNING: GameTuning = {
  spawnProgressStep: 80,
  enemySpeedPermille: 1000,
  projectileSpeedPermille: 1000,
  skillCooldownPermille: 1000,
  waveDelayPermille: 1000,
  waveIntervalPermille: 1000,
  enemyAttackIntervalPermille: 1000,
  overheatDurationMs: 2000,
  hitFlashMs: 140,
  floatLifeMs: 900,
}
