import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

/**
 * 三种任务周期（daily / weekly / achievement）**都必须**在客户端有入口（第 94 轮）。
 *
 * # 缺陷：3 条周任务与 5 条成就在客户端完全够不着
 *
 * `signin.vue` 的任务面板原先写死：
 *
 * ```ts
 * const res = await api.fetchTasks('daily')
 * ```
 *
 * 于是：
 *
 * | scope | 任务数 | 客户端入口 |
 * |---|---|---|
 * | `daily` | 4 | ✅ |
 * | `weekly` | 3 | ❌ **没有** |
 * | `achievement` | 5 | ❌ **没有** |
 *
 * 服务端 `/tasks?scope=weekly` 与 `?scope=achievement` 都实现了，
 * `client.ts` 里也有 `fetchTasks(scope)` —— 只是**没有任何页面调用它们**。
 *
 * 这意味着第 76 轮修的「周任务永远领不到」在服务端是对的，
 * 但玩家在客户端**看不到也领不到** —— 那个修复对真实玩家不可见。
 *
 * 与第 87 轮同族：`descr` 字段被下发、被注释声称「文档站直接展示」，
 * 实际文档站没有那一列。**「服务端有」不等于「玩家够得着」。**
 *
 * # 为什么用源码扫描而不是渲染断言
 *
 * 渲染断言能验证「切到每周后请求了 `?scope=weekly`」，
 * 但判据需要 mock + 挂载 + 点击，而**它只能证明这一条路径**。
 *
 * 真正要守的性质是「三种 scope 一个都不漏」，而 scope 的**全集**
 * 由服务端的 `tasks.scope` 定义 —— 那是个会增长的值。
 * 所以判据是：**客户端出现的 scope 集合 ⊇ 服务端种子里的 scope 集合**。
 */
/**
 * stripTsComments 剥掉行注释与块注释。
 *
 * ⚠️ 这是本项目**第三次**为同一个原因加它：
 *
 *   - 第 83 轮：`canvas.ts` 的说明注释里两次提到 `spec.aoeRadius`
 *   - 第 88 轮：修复说明里引用了「修复前的写法」，判据在注释里找到目标
 *   - 第 94 轮：本文件下方那段说明注释里写着 ``fetchTasks('daily')``，
 *     于是「不得有硬编码 daily」那条断言**命中了注释**
 *
 * 三次都是同一个形状：**文本扫描分不清注释与代码**。
 * 它之所以重复发生，是因为每次都在新文件里重写判据 ——
 * **「剥注释」应该是共用工具而不是各写一份。**
 *
 * ⚠️ 这是文本级剥离，对字符串字面量里的 `//` 会误伤。
 * 那种写法本身可疑，而误伤的代价是「有人要来解释这条为什么红了」。
 */
function stripTsComments(src: string): string {
  return (
    src
      // 块注释
      .replace(/\/\*[\s\S]*?\*\//g, ' ')
      // 行注释
      .replace(/^\s*\/\/.*$/gm, ' ')
      // ⚠️ **模板里的 HTML 注释**（.vue 的 <template> 段落）
      //
      // 第 94 轮实测踩到：我把「此前写死 fetchTasks('daily')」这段说明
      // 写在模板的 `<!-- -->` 里，而前两条规则都剥不掉它 ——
      // 于是「不得有硬编码 daily」那条断言命中的**全是注释**。
      //
      // 三种注释形式都要剥，缺一种就会出现「判据在读注释」的假阴性。
      .replace(/<!--[\s\S]*?-->/g, ' ')
  )
}

describe('三种任务周期都要有客户端入口（第 94 轮）', () => {
  // ⚠️ 必须剥注释 —— 见 stripTsComments 的说明（本项目第三次为同一原因加它）。
  const src = stripTsComments(readFileSync(resolve(__dirname, 'signin.vue'), 'utf-8'))

  it('任务面板的拉取用当前 scope，而不是写死 daily', () => {
    expect(
      /fetchTasks\(\s*scope\.value\s*\)/.test(src),
      '任务面板写死了 fetchTasks(\'daily\') —— weekly 与 achievement 够不着',
    ).toBe(true)
    expect(
      /fetchTasks\(\s*'daily'\s*\)/.test(src),
      "任务面板里仍有 fetchTasks('daily') 硬编码",
    ).toBe(false)
  })

  it('SCOPES 覆盖服务端的三种 scope', () => {
    const m = src.match(/const SCOPES = \[([\s\S]*?)\] as const/)
    expect(m, '找不到 SCOPES 定义').not.toBeNull()
    const keys = [...(m![1].matchAll(/key:\s*'(\w+)'/g))].map((x) => x[1])
    // ⚠️ 这份清单与 server/internal/seeder/seed.go 的 scope 取值一一对应。
    // 少一个就有一批任务在界面上消失，且**没有任何报错**。
    for (const want of ['daily', 'weekly', 'achievement']) {
      expect(keys, `SCOPES 里没有 '${want}' —— 那批任务在客户端消失且无任何报错`).toContain(
        want,
      )
    }
    expect(keys.length, `SCOPES 有 ${keys.length} 项`).toBe(3)
  })

  it('切换周期会重新拉取（scope 是响应式来源）', () => {
    expect(src, '找不到 switchScope').toContain('function switchScope')
    const body = src.slice(src.indexOf('function switchScope'))
    expect(body.slice(0, 400), 'switchScope 没有重新拉取').toContain('load()')
    // scope 必须是 ref —— 否则模板点它也没用
    expect(
      /const scope = ref<ScopeKey>\('daily'\)/.test(src),
      'scope 必须是 ref —— 模板里点它才能触发刷新',
    ).toBe(true)
  })

  it('三个周期标签都渲染出来了（不只是定义了常量）', () => {
    expect(
      /v-for="s in SCOPES"/.test(src),
      '模板里没有遍历 SCOPES —— 常量定义了但界面上没有切换入口',
    ).toBe(true)
  })
})

/**
 * 服务端 `tasks.scope` 的取值集合必须与客户端 `SCOPES` 一致。
 *
 * 这条放在 miniapp 侧是因为它是**跨端契约**的客户端那一半；
 * 服务端那一半由 `internal/seeder` 的种子数据定义。
 *
 * ⚠️ 判据是「集合相等」而不是「客户端 ⊇ 服务端」——
 * 因为**多出来**的 scope 同样是错的：界面会给出一个永远空列表的标签页。
 */
describe('客户端 SCOPES 与服务端 seed 的 scope 集合一致', () => {
  it('服务端 seed 用到的 scope 恰好是这三个', () => {
    const seed = readFileSync(
      resolve(__dirname, '../../../../server/internal/seeder/seed.go'),
      'utf-8',
    )
    const found = new Set(
      [...seed.matchAll(/"(daily|weekly|achievement)"/g)].map((m) => m[1]),
    )
    expect([...found].sort(), '服务端 scope 取值变了').toEqual([
      'achievement',
      'daily',
      'weekly',
    ])

    const ui = stripTsComments(
      readFileSync(resolve(__dirname, 'signin.vue'), 'utf-8'),
    )
    const m = ui.match(/const SCOPES = \[([\s\S]*?)\] as const/)!
    const keys = [...(m[1].matchAll(/key:\s*'(\w+)'/g))].map((x) => x[1]).sort()
    expect(keys, '客户端 SCOPES 与服务端 scope 集合不一致').toEqual([
      'achievement',
      'daily',
      'weekly',
    ])
  })
})

/**
 * 剥离器自身的守卫：三种注释里的字面量都**不得**被扫到。
 *
 * ⚠️ 这条不是预防性设计 —— 第 94 轮的三次失败里有两次正是
 * 「判据在注释里找到了目标」。
 *
 * 而「剥离器只覆盖两种注释形式」这件事**不会以断言的形式出现**：
 * 它表现为「某个守卫莫名其妙地红了 / 莫名其妙地绿了」。
 * 所以必须显式钉住三种形式。
 */
describe('stripTsComments 覆盖三种注释形式', () => {
  const src = [
    "// fetchTasks('daily') 行注释",
    "/* fetchTasks('daily') 块注释 */",
    "<!-- fetchTasks('daily') 模板注释 -->",
    "const x = 1",
  ].join('\n')

  it('三种注释里的字面量都被剥掉', () => {
    expect(stripTsComments(src)).not.toContain('fetchTasks')
  })

  it('真代码留着', () => {
    expect(stripTsComments(src)).toContain('const x = 1')
  })
})
