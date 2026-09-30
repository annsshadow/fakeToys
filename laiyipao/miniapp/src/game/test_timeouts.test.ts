import { describe, it, expect } from 'vitest'
import { readFileSync, readdirSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

/**
 * 「重活测试必须有显式超时预算」的守卫。
 *
 * ## 这条守卫来自一次真实的 flaky
 *
 * 第 45 轮加了 `armor_shape.test.ts`（扫 9 档护甲 × 前 40 关 + 2×100 关），
 * 5 个 `it` **全部用 vitest 默认的 5000ms 超时**。实测耗时：
 *
 *   剩余血量随护甲严格递增   5847 ms   ← 只有 17% 余量
 *   满封顶不让关卡变难       4258 ms   ← 只有 17% 余量
 *   边际收益崩塌             1312 ms
 *
 * 结果：全量跑时**出现过一次失败**，且**连续 9 次无法复现**（含 `--sequence.shuffle`）。
 *
 * 根因不是逻辑错，是**机器一忙就撞上 5 秒默认上限**。
 * 一个「跑得对、只是慢」的用例在负载下变红 —— 而这正是本项目反复说的
 * 「一条经常误报的守卫等于没有守卫」的**镜像**：
 * 这次不是守卫误报，是**被守卫对象**误报。
 *
 * ## 为什么不是「把阈值调高」而是「必须显式声明」
 *
 * 超时的用途是**抓挂死**，不是**卡性能**。
 * 合法要跑几秒的测试（整局扫描、平衡探测）应该显式声明一个宽松预算，
 * 让「慢」与「挂」区分开。默认 5 秒把两者混在一起了。
 *
 * ## 局限（明说）
 *
 * 判据是**源码文本匹配**（是否含 `new BattleEngine` + 是否遍历全关卡 +
 * 是否带数字超时），所以：
 *
 *  - 判「重活」用的是**同一批正则**，如果有人把写法改成正则认不出的形式，
 *    守卫会以为它不重活而放行；
 *  - 它只保证「有声明」，不保证「声明得够大」。
 *
 * 它防的是「新增重活测试时忘了写预算」这一类手滑，
 * 不是形式化验证。
 */

const here = dirname(fileURLToPath(import.meta.url))

/** vitest 默认单用例超时（ms）。 */
const DEFAULT_TIMEOUT_MS = 5000

interface Scan {
  file: string
  heavy: boolean
  its: number
  withTimeout: number
  slowest: number
  /** 重活是否已经提到 beforeAll 并在那里声明了预算 */
  hoistedToBeforeAll: boolean
}

function scan(): Scan[] {
  const out: Scan[] = []
  for (const name of readdirSync(here)) {
    if (!name.endsWith('.test.ts')) continue
    const src = readFileSync(join(here, name), 'utf8')
    // 「重活」= 真的跑引擎，且遍历全部关卡
    const runsEngine = /new BattleEngine/.test(src)
    const sweepsAll = /(of levels\)|levels\.map\(|for \(const lv of)/.test(src)
    out.push({
      file: name,
      heavy: runsEngine && sweepsAll,
      // `beforeAll(fn, 300_000)` —— 重活被提到钩子里、且预算已声明
      hoistedToBeforeAll: /beforeAll\([\s\S]*?,\s*\d[\d_]*\s*\)/.test(src),
      // 逐个 it(...) 配平括号，看它有没有数字超时
      ...countIts(src),
    })
  }
  return out
}

/**
 * 找下一个**独立的** `it(`。
 *
 * ⚠️ 必须要求词边界。
 * 第一版用 `src.indexOf('it(', i)`，结果数到了 `function sampleHit(` 里的 `it(` ——
 * 而 `sampleHit` 的参数尾部 `(..., 300000)` 又**恰好**符合「带数字超时」的形态，
 * 于是 `withTimeout` 被算成 14 > `its` 6，守卫报出
 * 「6 个用例里只有 14 个声明了」这种自相矛盾的话。
 *
 * > 判据里的 bug 会**伪装成「数据有问题」**，
 * > 而人被引导去怀疑数据 —— 不会去怀疑判据。
 * > 这个 bug 先在**打补丁的脚本**里犯过，随后被复制进了守卫。
 */
function nextIt(src: string, from: number): number {
  let i = from
  while (i < src.length) {
    const idx = src.indexOf('it(', i)
    if (idx < 0) return -1
    const prev = idx === 0 ? '' : src[idx - 1]
    if (!/[A-Za-z0-9_$]/.test(prev)) return idx
    i = idx + 3
  }
  return -1
}

function countIts(src: string): { its: number; withTimeout: number; slowest: number } {
  let its = 0
  let withTimeout = 0
  let slowest = 0
  let i = 0
  while (true) {
    const idx = nextIt(src, i)
    if (idx < 0) break
    let depth = 0
    let j = idx + 2
    let inStr: string | null = null
    for (; j < src.length; j++) {
      const c = src[j]
      if (inStr) {
        if (c === '\\') { j++; continue }
        if (c === inStr) inStr = null
        continue
      }
      if (c === "'" || c === '"' || c === '`') { inStr = c; continue }
      if (c === '(') depth++
      else if (c === ')') {
        depth--
        if (depth === 0) break
      }
    }
    const body = src.slice(idx, j + 1)
    its++
    const m = body.match(/,\s*(\d[\d_]*)\s*\)\s*$/)
    if (m) {
      withTimeout++
      const v = Number(m[1].replace(/_/g, ''))
      if (v > slowest) slowest = v
    }
    i = j + 1
  }
  return { its, withTimeout, slowest }
}

describe('重活测试的超时预算', () => {
  const rows = scan()

  it('扫描器本身能认出重活文件（前提守卫）', () => {
    const heavy = rows.filter((r) => r.heavy)
    expect(heavy.length, '一个重活文件都没认出来 —— 本守卫的前提不成立').toBeGreaterThan(3)
    expect(rows.length).toBeGreaterThan(10)
  })

  it('每个重活文件都显式声明了超时预算（逐个 it，或提到 beforeAll）', () => {
    // 两种合法形态：
    //   a) 每个 `it` 自己带预算（重活在用例体内）；
    //   b) 重活被提到 `beforeAll(fn, 300_000)`，`it` 只做廉价的数组扫描
    //      （`settle_bounds_parity` / `tier_impact` 就是这一种）。
    //
    // 之所以要区分 b：给那种文件里的每个 `it` 都挂 300 秒
    // **看起来满足了守卫，实际把守卫变成了恒真** ——
    // 「全部文件都声明了」就不再传递任何信息。
    const bad: string[] = []
    for (const r of rows.filter((x) => x.heavy)) {
      if (r.withTimeout >= r.its) continue
      if (r.hoistedToBeforeAll) continue
      bad.push(
        `${r.file}：${r.its} 个用例里只有 ${r.withTimeout} 个声明了超时，` +
          `也没有把重活提到带预算的 beforeAll（未声明的用 vitest 默认 ${DEFAULT_TIMEOUT_MS}ms）`,
      )
    }
    expect(
      bad.join('\n'),
      '重活测试必须显式声明超时预算 —— 默认 5 秒会让「跑得对只是慢」的用例在负载下变红，' +
        '而这种 flaky 无法复现、极难定位（armor_shape.test.ts 就这样挂过一次）',
    ).toBe('')
  })

  it('声明的预算不小于 60 秒（5 秒默认值的 12 倍）', () => {
    // 60 秒是实测最慢用例（5.8 秒）的 10 倍余量。
    // 判据是「不许比默认 5 秒只宽一点点」—— 那等于没改。
    const bad: string[] = []
    for (const r of rows.filter((x) => x.heavy)) {
      if (r.withTimeout > 0 && r.slowest < 60_000) {
        bad.push(`${r.file}：最大预算 ${r.slowest}ms < 60000ms`)
      }
    }
    expect(bad.join('\n')).toBe('')
  })

  it('⚠️ 守卫不是恒真：把一个文件的声明拿掉，它必须红', () => {
    // 这条是**给守卫自己**的变异验证。
    //
    // 本项目反复栽在「断言恒真」上：一条永远绿的守卫比没有守卫更糟。
    // 这里用一份**构造出来的**源码样本喂给同一套判据，
    // 证明判据确实会因为「少声明」而判不合格。
    const sampleHeavy = `
      it('a', () => { new BattleEngine({}); for (const lv of levels) {} })
      it('b', () => { new BattleEngine({}); for (const lv of levels) {} })
    `
    const sampleHeavyFixed = `
      beforeAll(() => { for (const lv of levels) {} }, 300000)
      it('a', () => { new BattleEngine({}); for (const lv of levels) {} })
      it('b', () => { new BattleEngine({}); for (const lv of levels) {} })
    `
    const bad = countIts(sampleHeavy)
    const fixed = countIts(sampleHeavyFixed)
    const HOIST = /beforeAll\([\s\S]*?,\s*\d[\d_]*\s*\)/
    // 同一个判据，分别喂两个样本
    const qualifies = (c: { withTimeout: number; its: number }, src: string) =>
      c.withTimeout >= c.its || HOIST.test(src)

    // 原形态：2 个用例、0 个声明、无 beforeAll → 必须判为**不合格**
    expect(bad.its).toBe(2)
    expect(bad.withTimeout).toBe(0)
    expect(qualifies(bad, sampleHeavy)).toBe(false)

    // 修正形态（提到带预算的 beforeAll）→ 必须判为**合格**
    expect(HOIST.test(sampleHeavyFixed)).toBe(true)
    expect(qualifies(fixed, sampleHeavyFixed)).toBe(true)

    // 再补一种：逐个 it 声明预算，也应合格
    const perIt = `
      it('a', () => { new BattleEngine({}); for (const lv of levels) {} }, 300000)
      it('b', () => { new BattleEngine({}); for (const lv of levels) {} }, 300000)
    `
    expect(qualifies(countIts(perIt), perIt)).toBe(true)
  })

  it('记录各重活文件的用例数与预算（诊断用）', () => {
    for (const r of rows.filter((x) => x.heavy)) {
      console.log(
        `  ${r.file.padEnd(32)} 用例 ${String(r.its).padStart(2)}  ` +
          `声明超时 ${String(r.withTimeout).padStart(2)}  最大预算 ${r.slowest}ms`,
      )
    }
    // 前提：至少认出一个「全部声明」的与一个「曾经漏声明」的历史事实
    expect(rows.some((r) => r.heavy && r.withTimeout === r.its && r.its > 0)).toBe(true)
  })
})
