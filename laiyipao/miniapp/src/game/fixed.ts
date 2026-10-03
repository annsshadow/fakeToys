/**
 * 定点整数数学（I-6 的硬约束基础）。
 *
 * 为什么不能用浮点：
 * 浮点运算在不同 JS 引擎（V8 / JSCore / QuickJS）与不同设备上，
 * 舍入行为可能不同 → 同一场战斗算出的回放哈希必然不同 → I-6 失效。
 *
 * 因此：**所有伤害、速度、坐标、血量一律用整数 + 千分比定点数**，
 * 随机只走 lcg.ts 的 PRNG，绝不使用 Math.random。
 * 渲染层在最终绘制前才转回浮点。
 *
 * ⚠️ 本文件的 mulDiv / clamp / applyArmor 等函数必须与
 * server/internal/domain/damage.go 逐行等价。改动必须同步两处，
 * 并更新 server/testdata/formula_vectors.json。
 */

/**
 * 千分比基数。
 *
 * ⚠️ 所有常量必须是 bigint（带 n 后缀）。写成普通数字会导致
 * bigint 与 number 混算 —— 比较时 1000 !== 1000n，
 * 且 BigInt(小数) 会直接抛 RangeError。
 */
export const PERMILLE = 1000n

/** 直接伤害权重（千分比） */
export const DIRECT_WEIGHT = 400n

/** 单层元素的基础伤害基准（绝对值，不随攻击力缩放） */
export const ELEMENT_PER_STACK_BASE = 1200n

/** 元素层数持续伤害的除数 */
export const ELEMENT_TICK_DIVISOR = 40n

/** 抗性绝对值上限（千分比，±50%） */
export const MAX_RESIST = 500n

/** 减伤上限（千分比，75%） */
export const MAX_ARMOR = 750n

/** 反通胀红线：反应伤害中攻击力贡献占比上限（千分比） */
export const MAX_REACTION_ATTACK_WEIGHT = 300n

/** bigint 乘法后转回（带溢出保护的 mulDiv） */
export function mulDiv(a: bigint, b: bigint, c: bigint): bigint {
  if (c === 0n) return 0n
  return (a * b) / c
}

export function clampInt64(v: bigint, lo: bigint, hi: bigint): bigint {
  if (v < lo) return lo
  if (v > hi) return hi
  return v
}

export function minBig(a: bigint, b: bigint): bigint {
  return a < b ? a : b
}

export function maxBig(a: bigint, b: bigint): bigint {
  return a > b ? a : b
}

/** 应用护甲减伤，上限 75% */
/**
 * 应用护甲减伤，最高减 75%。
 *
 * ⚠️ 第 71 轮：补上 `dmg <= 0` 早退，与 Go 侧 `damage.go:469` 对齐。
 *
 * Go 侧的注释解释了它为什么在：
 *
 * ```go
 * if dmg <= 0 {
 *     // 负伤害原样返回：负数在结算里表示「反伤/异常」，不该被护甲改写。
 *     return dmg
 * }
 * ```
 *
 * 缺了它两端**不等价**：dmg=-1000、armor=500 时 TS 得 -500、Go 得 -1000。
 *
 * ## 当前不可达，但契约应当先于实现正确
 *
 * 三个调用点都传非负值：
 *   - `damage.ts:261` —— 上一行有 `if (d < 0n) d = 0n` 夹紧
 *   - `engine.ts` 的 `breachDamage` 分支 —— `baseHpMax / LeakDamageDivisor`，非负
 *   - `engine.ts` 的 `e.attack > 0n` 分支
 *
 * 所以这不是**当前**的缺陷，而是「文件头声明的『逐行等价』与实现不符」。
 *
 * ## 为什么仍然要改
 *
 * 1. 文件头写着「本文件的 applyArmor 必须与 damage.go 逐行等价」——
 *    声明与实现不符时，**下一个读声明的人**（人或模型）会按声明推理，
 *    而声明是错的。
 * 2. Go 侧的注释说明负伤害**将来会有语义**（反伤/异常）。
 *    那天 TS 侧会以不同的值参与哈希，而
 *    `formula_vectors.json` 的 damage 用例全是正伤害 —— **抓不到**。
 *
 * 这与 README 记的「跨端一致 ≠ 两端都对」是同一件事：
 * 契约向量只覆盖「两端都传了同一个合法值」时的比对，
 * 分歧恰好在**非法输入路径**上
 * （README 第 9.3 节记的就是 `reactionElement` 的空串分歧）。
 */
export function applyArmor(dmg: bigint, armorPermille: bigint): bigint {
  if (dmg <= 0n) return dmg
  if (armorPermille <= 0n) return dmg
  const capped = armorPermille > MAX_ARMOR ? MAX_ARMOR : armorPermille
  return mulDiv(dmg, PERMILLE - capped, PERMILLE)
}

/** 把定点数转为绘制用浮点（仅渲染层调用） */
export function toFloat(v: bigint, scale = 1000): number {
  return Number(v) / scale
}

/** 把绘制用浮点转回定点整数（仅输入层调用，如拖拽坐标） */
export function fromFloat(v: number, scale = 1000): bigint {
  return BigInt(Math.round(v * scale))
}
