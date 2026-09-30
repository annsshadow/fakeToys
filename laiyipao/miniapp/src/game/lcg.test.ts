import { describe, it, expect } from 'vitest'
import { BattleRng, LCG, fnv1a64, hex16, LCG_KNOWN_VECTORS } from './lcg'
describe('LCG 跨端契约', () => {
  it('必须与 Go 侧 TestLCGKnownVectors 产出逐位相同的序列', () => {
    const l = new LCG(12345)
    for (let i = 0; i < LCG_KNOWN_VECTORS.length; i++) {
      expect(l.next(), `第 ${i + 1} 个值`).toBe(LCG_KNOWN_VECTORS[i])
    }
  })

  it('BigInt 实现与硬编码向量的负向验证（防止向量被误改）', () => {
    // 这些值由 Go uint64 实测产出；若 TS 算出别的，说明两端已漂移
    expect(LCG_KNOWN_VECTORS.map(String)).toEqual([
      '6917529027818027904',
      '1269031133',
      '9223372037807892195',
    ])
  })

  it('intn 落在 [0,n) 内', () => {
    for (const seed of [0, 1, 42, 12345]) {
      const l = new LCG(seed)
      for (let i = 0; i < 500; i++) {
        const v = l.intn(10)
        expect(v).toBeGreaterThanOrEqual(0)
        expect(v).toBeLessThan(10)
      }
    }
  })

  it('n<=0 时 intn 返回 0', () => {
    const l = new LCG(7)
    for (const n of [0, -1, -100]) {
      expect(l.intn(n)).toBe(0)
    }
  })

  it('permille 落在 [0,1000) 内且逐位确定（千分比量纲的滚点）', () => {
    // LCG.permille 是「千分比量纲随机数」的独立出口 —— 它从不被
    // intn/roll 间接执行（roll 走的是 intn(1000)）。若没有测试触碰，
    // 有人把 `next() % 1000n` 误改成 `next() % 10000n` 也不会红，
    // 而所有以 permille 为量纲的调用方会瞬间越界。
    const a = new LCG(2024)
    const b = new LCG(2024)
    for (let i = 0; i < 500; i++) {
      const v = a.permille()
      expect(v).toBeGreaterThanOrEqual(0n)
      expect(v).toBeLessThan(1000n)
      expect(v).toBeTypeOf('bigint')
      expect(v).toBe(b.permille())
    }
  })

  it('同种子两次产生相同序列（确定性）', () => {
    const a = new LCG(999)
    const b = new LCG(999)
    for (let i = 0; i < 20; i++) expect(a.next()).toBe(b.next())
  })
})

describe('FNV-1a 64', () => {
  it('空输入产出 Go 侧的初值', () => {
    expect(fnv1a64('')).toBe(0xcbf29ce484222325n)
  })

  it('"a" 的哈希与 Go hash/fnv 一致', () => {
    // Go: fnv.New64a(); h.Write([]byte("a")) => 0xaf63dc4c8601ec8c
    expect(fnv1a64('a')).toBe(0xaf63dc4c8601ec8cn)
  })

  it('"foobar" 的哈希与 Go 一致', () => {
    // Go: fnv.New64a(); h.Write([]byte("foobar")) => 0x85944171f73967e8
    expect(fnv1a64('foobar')).toBe(0x85944171f73967e8n)
  })

  it('Uint8Array 输入与同内容字符串产出相同哈希（两个入口同一算法）', () => {
    // fnv1a64 有 string 与 Uint8Array 两个入口。防线快照摘要传的是
    // 拼接字符串，回放哈希侧曾计划传字节 —— 若两个入口分叉，
    // 同一份数据会算出两个哈希，跨端对齐直接失效。
    // 顺手钉住字节入口本身的绝对值（非空字节确实参与了迭代）。
    const bytes = new TextEncoder().encode('foobar')
    expect(fnv1a64(bytes)).toBe(fnv1a64('foobar'))
    expect(fnv1a64(new Uint8Array([0x66, 0x6f, 0x6f]))).toBe(fnv1a64('foo'))
  })

  it('hex16 补足 16 位小写十六进制', () => {
    expect(hex16(0n)).toBe('0000000000000000')
    expect(hex16(0xcbf29ce484222325n)).toBe('cbf29ce484222325')
    expect(hex16(0x1n)).toBe('0000000000000001')
  })
})

describe('BattleRng', () => {
  it('roll 落在 0..9999', () => {
    const r = new BattleRng(42)
    for (let i = 0; i < 500; i++) {
      const v = r.roll()
      expect(v).toBeGreaterThanOrEqual(0)
      expect(v).toBeLessThan(10000)
    }
  })

  it('chance 按千分比判定', () => {
    const r = new BattleRng(7)
    let hits = 0
    for (let i = 0; i < 1000; i++) if (r.chance(1000n)) hits++
    // 100% 概率必须恒真
    expect(hits).toBe(1000)

    const r2 = new BattleRng(7)
    let never = 0
    for (let i = 0; i < 1000; i++) if (r2.chance(0n)) never++
    expect(never).toBe(0)
  })

  it('pick 从数组取元素', () => {
    const r = new BattleRng(1)
    const arr = ['a', 'b', 'c'] as const
    for (let i = 0; i < 100; i++) expect(arr).toContain(r.pick(arr))
  })
})
