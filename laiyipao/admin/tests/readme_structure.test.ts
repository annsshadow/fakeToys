/**
 * README 结构守卫（第 58 轮）。
 *
 * ## 为什么需要它
 *
 * README 里那套「工程约束」条目是几轮优化攒下来的教训 —— 价值很高，
 * 也因此**很容易在插入新条目时被破坏**。第 56~58 三轮里真的坏过两次，
 * 两次都**没有任何报错**：
 *
 *   1. 新条目插到了旧条目的**正文中间**，把一个段落孤立到后面；
 *   2. 条目编号乱序（`12, 13, 14, 16, 17, 15`）。
 *
 * 文档腐烂比代码腐烂更隐蔽 —— 代码腐烂会编译失败，文档腐烂只会让人读到错的东西。
 *
 * ## 这道守卫刻意**不**做什么
 *
 * 最初有一版「标题里不许出现中文计数」。实测把
 * `## 三条被撤回的误判`、`### 三个测量陷阱`、`### 测试空洞的四种形态`
 * 全部判成违规 —— 而这些标题里的数字**就是内容本身**。
 *
 * 为此写禁令等于写一个错的守卫，比没有守卫更糟（本项目栽过：
 * 「分数容差取 2% 而非严格 >=」那条守卫如果写成死等就会永远红、被当成 flaky 忽略）。
 *
 * 所以收窄成只拦**真正会腐烂的形状**：计数 + 「约束/条目」，
 * 即「这里一共有几条约束」这种一加条目就不对的标签。
 * 那个真实的案例（`## 十四个关键工程约束`）已经改成不带计数的 `## 关键工程约束`。
 */
import { readFileSync } from 'node:fs'
import { join, dirname } from 'node:path'
import { fileURLToPath } from 'node:url'
import { describe, it, expect } from 'vitest'

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..')
const lines = readFileSync(join(root, 'README.md'), 'utf8').split(/\r?\n/)

/** `### 12. xxx` → "12"；`### 9.1 xxx` → "9.1"；其余 → null */
function itemNumber(line: string): string | null {
  const m = /^### (\d+(?:\.\d+)?)\.\s+\S/.exec(line)
  return m ? m[1] : null
}

const isHeading = (line: string) => /^#{1,6}\s+\S/.test(line)

describe('README 结构', () => {
  it('约束条目编号在同一段里连续递增', () => {
    // ⚠️ 必须**分段**检查。
    //
    // README 里有两个各自从 1 开始的编号序列：
    //   「快速开始」的 1~4（起后端/起小程序/起后台/端到端验收）
    //   「关键工程约束」的 1~17
    //
    // 第一版当成一条序列，于是报 `4 -> 1` —— 那是**误报**，
    // 而误报的守卫会被当成噪声忽略，之后真乱序也就不再有人看。
    //
    // 遇到 `##` 二级标题就开新段：段内必须严格 +1，段与段之间不比较。
    const runs: Array<{ start: number; nums: number[] }> = []
    let cur: { start: number; nums: number[] } | null = null

    lines.forEach((line, i) => {
      if (/^##\s+\S/.test(line)) {
        cur = { start: i + 1, nums: [] }
        runs.push(cur)
        return
      }
      const n = itemNumber(line)
      if (n === null) return
      if (!cur) {
        cur = { start: i + 1, nums: [] }
        runs.push(cur)
      }
      // 子条目（9.1）不参与顶层序列
      if (Number.isInteger(Number(n))) cur.nums.push(Number(n))
    })

    const bad: string[] = []
    for (const run of runs) {
      for (let i = 1; i < run.nums.length; i++) {
        if (run.nums[i] !== run.nums[i - 1] + 1) {
          bad.push(`L${run.start} 起的一段：${run.nums[i - 1]} -> ${run.nums[i]}`)
        }
      }
    }
    expect(bad).toEqual([])
    // 至少要有两段、且最长那段超过 10 条 —— 否则守卫自己没生效
    expect(runs.length).toBeGreaterThanOrEqual(2)
    expect(Math.max(...runs.map((r) => r.nums.length))).toBeGreaterThan(10)
  })

  it('每个标题前面都有空行（否则渲染会与上一段粘连）', () => {
    const bad: string[] = []
    lines.forEach((line, i) => {
      if (i === 0) return
      if (isHeading(line) && lines[i - 1].trim() !== '') {
        bad.push(`L${i + 1}: ${line.slice(0, 48)}`)
      }
    })
    expect(bad).toEqual([])
  })

  it('没有「这里一共有 N 条约束」这种会腐烂的标题标签', () => {
    // 只拦「计数 + 约束/条目」这一种形状。
    // 「三条被撤回的误判」这种数字是内容，不在管辖范围内 —— 见文件头说明。
    const rot = /^#{2,3}\s+\S*[一二三四五六七八九十百]+\s*(?:个|条)?\s*(?:关键)?(?:工程)?(?:约束|条目)/
    const bad: string[] = []
    lines.forEach((line, i) => {
      if (/^#{2,3}\s/.test(line) && rot.test(line)) {
        bad.push(`L${i + 1}: ${line.slice(0, 48)}（条目一加就不对，去掉计数）`)
      }
    })
    expect(bad).toEqual([])
  })

  it('没有重复的长正文行 —— 抓「段落被插进来的内容劈开后留下副本」', () => {
    // 第 57 轮真的插坏过一次：把新小节插进第 16 条的正文中间，
    // 导致「「没上报」与「上报了但不对」要分开」这句在正文里出现**两次** ——
    // 一次在正确位置，一次落单在后面。
    //
    // ⚠️ 我先写过一版「检测段落被劈开」的判据（正文行前面是空行、后面是标题），
    // 变异验证时它**没抓住**复现出来的同一处断裂 —— 那是装饰品，已删。
    // 换成实测可靠的信号：重复的长正文行。
    //
    // 阈值 16 字是量出来的：当前 README 里 >=12 字有 1 处重复
    //（`pnpm install`，代码块里的命令），>=16 字 **0 处**。
    const MIN = 16
    const isStructural = (l: string) =>
      l === undefined ||
      l.trim() === '' ||
      isHeading(l) ||
      /^\s*([-*+]|\d+\.)\s/.test(l) ||
      /^\s*\|/.test(l) ||
      /^\s*```/.test(l) ||
      /^\s*>/.test(l) ||
      l.trim() === '---'

    const seen = new Map<string, number>()
    const dups: string[] = []
    lines.forEach((l, i) => {
      const t2 = l.trim()
      if (isStructural(l) || t2.length < MIN) return
      const prev = seen.get(t2)
      if (prev !== undefined) {
        dups.push(`L${prev} 与 L${i + 1} 重复：${t2.slice(0, 48)}`)
      } else {
        seen.set(t2, i + 1)
      }
    })
    expect(dups).toEqual([])
    // 守卫本身要生效：至少扫到足够多的行，否则「0 重复」是因为什么都没扫到
    expect(seen.size).toBeGreaterThan(100)
  })
})
