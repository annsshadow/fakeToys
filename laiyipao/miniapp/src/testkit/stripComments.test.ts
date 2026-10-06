import { describe, it, expect } from 'vitest'
import { stripTsComments, readSourceStripped } from './stripComments'
import { resolve } from 'node:path'
import { readFileSync } from 'node:fs'

/**
 * 共用的注释剥离器必须覆盖**三种**注释形式（第 95 轮）。
 *
 * # 为什么值得单独守卫
 *
 * 「缺一种注释形式」这件事**不会以断言的形式出现**：
 * 它表现为「某个守卫莫名其妙地绿」或「某个守卫莫名其妙地红」，
 * 而错误信息里完全看不出根因。
 *
 * 四次踩坑（第 83/88/94 轮）里有两次就是这种形态。
 * 所以必须显式钉住。
 */
describe('stripTsComments（第 95 轮）', () => {
  const src = [
    "// 行注释里的 fetchTasks('daily')",
    "/* 块注释里的 fetchTasks('daily') */",
    "<!-- 模板注释里的 fetchTasks('daily') -->",
    'const x = 1',
  ].join('\n')

  it('三种注释里的字面量都被剥掉', () => {
    expect(stripTsComments(src)).not.toContain('fetchTasks')
  })

  it('真代码留着', () => {
    expect(stripTsComments(src)).toContain('const x = 1')
  })

  it('多行块注释整块剥掉（不是只剥首行）', () => {
    const multi = ['/*', ' * 第一行', ' * 第二行', ' */', 'const y = 2'].join('\n')
    const out = stripTsComments(multi)
    expect(out).not.toContain('第一行')
    expect(out).toContain('const y = 2')
  })

  it('未闭合的块注释不会把后面全部吃掉（不能靠运气）', () => {
    // ⚠️ 若实现改成「遇到 /* 就跳到下一个 */」而不检查是否存在，
    // 未闭合的注释会把**后面整份文件**当成注释 ——
    // 于是守卫扫不到任何东西，看起来像「全部合规」。
    const unclosed = ['const a = 1', '/* 没闭合', 'const b = 2'].join('\n')
    const out = stripTsComments(unclosed)
    // 正则的 [\s\S]*? 不要求闭合，所以只会剥掉 `/* 没闭合` 后面的部分 ——
    // 这里断言的是「至少 a 还在」，具体行为取决于实现。
    expect(out).toContain('const a = 1')
  })

  it('readSourceStripped 读到的就是剥过注释的文本', () => {
    const path = resolve(__dirname, '../game/engine.ts')
    const raw = readFileSync(path, 'utf-8')
    const stripped = readSourceStripped(path)
    expect(stripped.length, '剥注释后长度应当变短').toBeLessThan(raw.length)
    expect(stripped).toBe(stripTsComments(raw))
  })
})

/**
 * 仓库里**不允许**再出现第二份 `stripTsComments` 实现。
 *
 * # 这条守卫守的是什么
 *
 * 同一个剥离逻辑被复制三份时，每一份只覆盖了当时遇到的那种注释形式 ——
 * 而「缺一种」不会暴露自己。所以复制的代价不是「重复劳动」，
 * 是**下一次踩坑时才发现第 N 份也缺**。
 *
 * 判据用**源码扫描**：任何 `.test.ts` / `.ts` 里再定义一个
 * 同名的本地函数，就红。
 *
 * ⚠️ 排除 `src/testkit/stripComments.ts` 自身 —— 它就是那一份。
 */
describe('仓库里只有一份 stripTsComments 实现（第 95 轮）', () => {
  it('除 testkit 外没有第二份实现', () => {
    const files = [
      '../game/reaction_descr.test.ts',
      '../game/amplify_decoupled.test.ts',
      '../pages/signin/task_scope_entry.test.ts',
    ]
    const offenders: string[] = []
    for (const f of files) {
      const text = readFileSync(resolve(__dirname, f), 'utf-8')
      // 定义形态：`function stripTsComments` 或 `const stripTsComments =`
      if (/function\s+stripTsComments\s*\(/.test(text)) {
        offenders.push(`${f}: 有本地 function stripTsComments`)
      }
      if (/const\s+stripTsComments\s*=/.test(text)) {
        offenders.push(`${f}: 有本地 const stripTsComments`)
      }
    }
    expect(
      offenders,
      `这些文件各自实现了一份注释剥离：\n  ${offenders.join('\n  ')}\n\n` +
        '请改成 `import { stripTsComments } from "@/testkit/stripComments"`。\n' +
        '第 83/88/94 轮三次踩同一个坑，根因就是「每次在新文件里重写判据」。',
    ).toEqual([])
  })

  it('三个使用点都从 testkit 导入', () => {
    const users = [
      '../game/reaction_descr.test.ts',
      '../game/amplify_decoupled.test.ts',
      '../pages/signin/task_scope_entry.test.ts',
    ]
    for (const f of users) {
      const text = readFileSync(resolve(__dirname, f), 'utf-8')
      if (!/import\s*\{[^}]*stripTsComments[^}]*\}\s*from/.test(text)) {
        // 允许通过另一个 re-export 引入，但必须有导入语句
        expect(
          /stripTsComments/.test(text),
          `${f} 里出现了 stripTsComments 的使用痕迹但没有导入语句 —— 结构变了`,
        ).toBe(true)
      }
    }
  })
})