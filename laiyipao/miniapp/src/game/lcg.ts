/**
 * 确定性伪随机数发生器（I-6 的根基）。
 *
 * 选用 PCG-XSH-RR 的 LCG 变体而非 mulberry32，是因为它只用 64 位整数运算，
 * Go 用 uint64、TS 用 BigInt，两端能产出**逐位相同**的序列 ——
 * 这已由 Node 实测验证（见 server/internal/domain/levelgen_test.go:TestLCGKnownVectors）。
 *
 * ⚠️ 严禁使用 Math.random()。所有随机必须经本模块，否则回放哈希不可复现。
 *
 * 溢出说明：JS 的 BigInt 不会自动环绕 2^64，因此每次运算后显式取模。
 */

const MULT = 6364136223846793005n
const INC = 1442695040888963407n
const INIT_XOR = 0x2545f4914f6cdd1dn

export class LCG {
  private state: bigint

  constructor(seed: number | bigint) {
    this.state = BigInt.asUintN(64, BigInt(seed) ^ INIT_XOR)
  }

  /** 返回下一个伪随机数（无符号 64 位） */
  next(): bigint {
    const old = this.state
    this.state = BigInt.asUintN(64, old * MULT + INC)
    // PCG-XSH-RR 输出置换
    const xorshifted = BigInt.asUintN(32, ((old >> 18n) ^ old) >> 27n)
    const rot = old >> 59n
    if (rot === 0n) return xorshifted
    return BigInt.asUintN(64, (xorshifted >> rot) | (xorshifted << ((64n - rot) & 63n)))
  }

  /** 返回 [0, n) 内的整数。n <= 0 时返回 0 */
  intn(n: number): number {
    if (n <= 0) return 0
    return Number(this.next() % BigInt(n))
  }

  /** 返回 [0, 1) 之间的定点数（千分比） */
  permille(): bigint {
    return this.next() % 1000n
  }
}

/** 跨端契约向量：seed=12345 的前三个输出 */
export const LCG_KNOWN_VECTORS = [
  6917529027818027904n,
  1269031133n,
  9223372037807892195n,
] as const

/**
 * FNV-1a 64 位哈希（与 Go 的 hash/fnv 一致）。
 * 用于回放哈希与防线快照摘要。
 */
export function fnv1a64(data: Uint8Array | string): bigint {
  const bytes = typeof data === 'string' ? new TextEncoder().encode(data) : data
  let hash = 0xcbf29ce484222325n
  const prime = 0x100000001b3n
  for (const b of bytes) {
    hash = BigInt.asUintN(64, (hash ^ BigInt(b)) * prime)
  }
  return hash
}

/** 把 uint64 格式化为 16 位小写十六进制（与 Go 的 Hex16 一致） */
export function hex16(v: bigint): string {
  return BigInt.asUintN(64, v).toString(16).padStart(16, '0')
}

/** 战斗内使用的 PRNG 包装：只暴露战斗需要的接口 */
export class BattleRng {
  private lcg: LCG

  constructor(seed: number | bigint) {
    this.lcg = new LCG(seed)
  }

  /** 暴击判定用的 0..9999 滚点 */
  /**
   * 暴击判定用的滚点。**量纲是千分比（0..999）**。
   *
   * ⚠️ 曾经是 `intn(10000)`（万分比 0..9999），而判定阈值
   * `roll >= 1000 - critPermille` 是千分比 —— 两个量纲混用，
   * 于是 P(crit) = (10000 - (1000 - critPermille)) / 10000，
   * 即 critPermille=50（标称 5%）实际产生 **90.5%** 暴击率，
   * 10 倍偏差。
   *
   * 危害不只是数字错：所有 critPermille 取值都落在 90%+ 的区间，
   * 于是「提高暴击率」这个成长维度**边际收益被压平** ——
   * 实测 critPermille 从 0 提到 1000（满暴），第 1 关分数只从
   * 15542 变到 15545（+0.02%）。专精树的 crit 节点与 gem_crit 全是空节点。
   *
   * 统一到千分比而不是改判定公式：BattleRng 里已经有
   * `intn(1000) < permille` 的正确用法（第 88 行），
   * 让 roll 与所有其他千分比参数同量纲，从根上消除这类混淆。
   * 契约向量里的 crit_permille 恒为 0，暴击路径不参与那些向量，
   * 所以本改动不影响既有期望值；分布由 TestCritRateDistribution 守住。
   */
  roll(): number {
    return this.lcg.intn(1000)
  }

  /** 0..1 */
  chance(permille: bigint): boolean {
    return BigInt(this.lcg.intn(1000)) < permille
  }

  intn(n: number): number {
    return this.lcg.intn(n)
  }

  /** 洗练宝石词条数量 */
  pick<T>(arr: readonly T[]): T {
    return arr[this.lcg.intn(arr.length)]
  }
}
