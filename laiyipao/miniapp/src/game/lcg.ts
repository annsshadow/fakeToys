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
  roll(): number {
    return this.lcg.intn(10000)
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
