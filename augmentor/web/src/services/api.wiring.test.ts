/**
 * 「服务层封装是否真的接到了页面上」的守门测试
 *
 * 起因：#11 给 8 个后端能力补了 `services/api.ts` 封装。**加了封装却忘了在页面里
 * 引用**是这类改动最典型的漏法，而且类型检查、lint、覆盖率三者**都发现不了** ——
 * 函数确实被导出了，覆盖率还算 100%。所以这里做一次静态引用扫描。
 *
 * 判据：`api.ts` 导出的每个函数名，必须在 `pages/` + `components/` + `App.tsx`
 * 的源码里以标识符形式出现过 —— **且不在 import 语句、也不在注释里**
 * （见 `stripImports` / `stripComments`）。这两条都是被缺陷注入逼出来的：
 * 不剥 import 拦不住「import 了没用」，不剥注释拦不住「函数删了、注释留着」。
 *
 * 局限：这是**文本匹配**，不是调用图分析。只能确认「被引用了」，不能确认
 * 「被调用了」（比如写在字符串里、或被 `if (false)` 包住）。它拦的是
 * 「完全没接线」「只 import 没用」「只在注释里提到」这三档，不是全部。
 */
import { describe, expect, it } from 'vitest'

import * as api from './api'

/**
 * 「有封装但页面还没用上」的白名单 —— **L211 起为空**
 *
 * 历史上这里挂着 5 个函数（getCheckpoints / visualizeData / getVersion /
 * getVersionData / getVersionHistory）。它们全部已在对应页面（增强页断点列表、
 * 分析页图表文件、版本页详情/数据/操作历史）真实接线，白名单清空。
 *
 * 保留这个常量而不是删掉判据：将来任何新增的未接线封装仍会被最后一条用例
 * 当场拦下 —— 只检查新增函数的话，这个守门会随时间失效。
 */
const KNOWN_UNWIRED: string[] = []

/**
 * 去掉 import 语句后再扫描
 *
 * 不做这一步的话，「import 了但根本没调用」会骗过扫描：函数名仍留在 import
 * 列表里。而这恰恰是本守门要拦的形态之一（另一形态是「连 import 都没有」）。
 *
 * 注：ESLint 的 `no-unused-vars` 也能拦「import 了没用」，但本测试不该依赖
 * 另一个门 —— 两个门各自独立成立才有意义。
 */
const stripImports = (source: string) =>
  source.replace(/^\s*import\b[\s\S]*?from\s+['"][^'"]+['"]\s*$/gm, '')

/**
 * 去掉注释后再扫描
 *
 * 不剥离注释的话这个守门会被自己骗过：解释「为什么改用 `getDemoData()`」的
 * 那段注释里就带着函数名，函数删掉、注释留着，扫描照样通过。
 *
 * 只剥离**整行** `//` 注释与整块 C 风格块注释，不做完整的词法分析：
 * 行尾注释（`foo() // bar`）剥不掉，字符串里的 `//`（如 `'http://x'`）也不会
 * 被误伤。这是个刻意的折中 —— 残留会让守门偏松（可能漏），误伤会让它偏紧
 * （会假红）。偏紧会立刻暴露，偏松则靠下面的缺陷注入兜住。
 */
const stripComments = (source: string) =>
  source.replace(/\/\*[\s\S]*?\*\//g, '').replace(/^[ \t]*\/\/.*$/gm, '')

/**
 * 用 `?raw` 取源码文本
 *
 * 不能直接 import 这些 `.tsx`：那会真的执行页面模块（antd 组件在 jsdom 下
 * 需要 matchMedia / ResizeObserver 等一堆 shim），而本测试只关心「名字出现过没有」。
 *
 * **页面测试文件不算「接线」**（L211 补）：`../pages/*.tsx` 的 glob 会把同目录的
 * `*.test.tsx` 一起收进语料面，而组件测试的 `vi.mock` 替身表天然逐条写出服务函数名
 * ——「页面没接，但测试 mock 了」会骗过扫描。L211 的缺陷注入实证：把 `getVersionHistory`
 * 从页面源码整体摘掉，守门照样 4 passed。接线只认页面/组件/App 的真实源码。
 */
const rawUiSources = {
  ...import.meta.glob('../pages/*.tsx', { query: '?raw', import: 'default', eager: true }),
  ...import.meta.glob('../components/*.tsx', { query: '?raw', import: 'default', eager: true }),
  ...import.meta.glob('../*.tsx', { query: '?raw', import: 'default', eager: true }),
} as Record<string, string>

const uiSources = Object.fromEntries(
  Object.entries(rawUiSources).filter(([name]) => !name.endsWith('.test.tsx'))
)

describe('服务层与页面的接线', () => {
  it('页面源码确实被读到了（否则下面的断言会空转通过）', () => {
    const names = Object.keys(uiSources)

    // 反向元断言：glob 模式写错时 `uiSources` 会是空对象，
    // 于是「没有未接线函数」恒成立，测试变成摆设。
    expect(names).toContain('../App.tsx')
    expect(names.filter(name => name.startsWith('../pages/')).length).toBeGreaterThanOrEqual(10)
    expect(names.filter(name => name.startsWith('../components/')).length).toBeGreaterThanOrEqual(4)
  })

  it('语料面不含任何 *.test.tsx（L211：测试替身表不算接线）', () => {
    // 若 glob 面再次被扩回含测试文件，本用例先红，指向上一条的过滤为什么存在。
    expect(Object.keys(uiSources).filter(name => name.endsWith('.test.tsx'))).toEqual([])
    expect(Object.keys(rawUiSources).some(name => name.endsWith('.test.tsx'))).toBe(true)
  })

  it('import 语句本身不算「已接线」（否则本守门形同虚设）', () => {
    const stripped = stripImports("import { foo, bar } from './x'\n\nconst y = foo()\n")

    expect(stripped).not.toContain('foo, bar')
    expect(stripped).toContain('const y = foo()')
  })

  it('注释里提到函数名也不算「已接线」', () => {
    const stripped = stripComments(
      '/** 说明：这里改用 `getDemoData()` 了 */\n// const z = getDemoData()\nconst y = 1\n'
    )

    expect(stripped).not.toContain('getDemoData')
    expect(stripped).toContain('const y = 1')
  })

  it('除白名单外，api.ts 导出的每个函数都在页面里被引用', () => {
    const apiNames = Object.keys(api).filter(
      name => typeof (api as Record<string, unknown>)[name] === 'function'
    )

    // 同样要防空转：命名空间枚举失败（长度 0）时下面的差集也是空的
    expect(apiNames.length).toBeGreaterThanOrEqual(40)

    const haystack = Object.values(uiSources).map(stripComments).map(stripImports).join('\n')
    const unwired = apiNames.filter(
      name => !KNOWN_UNWIRED.includes(name) && !new RegExp(`\\b${name}\\b`).test(haystack)
    )

    expect(unwired).toEqual([])
  })
})
