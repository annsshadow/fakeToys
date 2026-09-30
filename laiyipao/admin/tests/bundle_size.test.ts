import { describe, it, expect } from 'vitest'
import { readFileSync, existsSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

/**
 * 「element-plus 保持按需引入」的守卫。
 *
 * ## 为什么需要它
 *
 * 第 47 轮把后台从**整包注册**改成**按需引入**：
 *
 * | | 改动前 | 改动后 |
 * |---|---|---|
 * | 产物总计 | 1981 KB | **1101 KB**（−44.4%） |
 * | element-plus | 901 KB（单 chunk，占 45.5%） | **286 KB**（15 个 chunk，−68.2%） |
 * | gzip 后总计 | — | 401 KB |
 *
 * 依赖是 `devDependencies` 里**早就装了、但一直没接进配置**的
 * `unplugin-vue-components` + `unplugin-auto-import`。
 *
 * 体积很容易被无声地退回去：任何人写一行
 * `app.use(ElementPlus)` 「修一下组件不显示的问题」，
 * 或者为了分块好看在 `manualChunks` 里把 `element-plus` 圈起来，
 * 就会把 600+ KB 直接加回去，而**没有任何测试会红**。
 *
 * ## ⚠️ 必须先剥掉注释再看
 *
 * 这正是第 35 轮踩过的坑：守卫匹配到**自己写的解释性注释**，
 * 于是「把代码改对」反而会被判失败 ——
 * 那会激励下一个人把守卫删掉，而不是修代码。
 *
 * 本文件里就有活例子：`main.ts` 的注释里写着
 * 「原先这里不再 `app.use(ElementPlus)`」，
 * 而 `vite.config.ts` 的注释里解释着「不能再这么圈」。
 * **不剥注释的话，这两条守卫会永远红。**
 */

/** 剥掉整行 `//` 注释与 `/* ... *\/` 块注释，保留字符串字面量。 */
function stripComments(src: string): string {
  let out = ''
  let i = 0
  let inStr: string | null = null
  let inLine = false
  let inBlock = false
  while (i < src.length) {
    const c = src[i]
    const next = src[i + 1]
    if (inLine) {
      if (c === '\n') { inLine = false; out += c }
      i++
      continue
    }
    if (inBlock) {
      if (c === '*' && next === '/') { inBlock = false; i += 2; continue }
      if (c === '\n') out += c
      i++
      continue
    }
    if (inStr) {
      out += c
      if (c === '\\') { out += next ?? ''; i += 2; continue }
      if (c === inStr) inStr = null
      i++
      continue
    }
    if (c === "'" || c === '"' || c === '`') { inStr = c; out += c; i++; continue }
    if (c === '/' && next === '/') { inLine = true; i += 2; continue }
    if (c === '/' && next === '*') { inBlock = true; i += 2; continue }
    out += c
    i++
  }
  return out
}

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const read = (rel: string) => {
  const p = join(root, rel)
  if (!existsSync(p)) {
    // 守卫找不到被测文件时必须明确失败，而不是静默跳过 ——
    // 否则「文件被改名 / 挪走」会变成一条永远绿的守卫。
    throw new Error(`守卫找不到 ${rel}（工作目录 ${root}）—— 文件被改名或挪走了？`)
  }
  return readFileSync(p, 'utf8')
}

const mainCode = stripComments(read('src/main.ts'))
const viteCode = stripComments(read('vite.config.ts'))
const vitestCode = stripComments(read('vitest.config.ts'))
const appCode = stripComments(read('src/App.vue'))

describe('element-plus 按需引入（体积守卫）', () => {
  it('main.ts 没有整包注册 ElementPlus', () => {
    expect(
      mainCode,
      'main.ts 又出现了 `app.use(ElementPlus)` —— 这会把 element-plus 全量打进产物' +
        '（实测 901 KB，占 45.5%）。按需引入由 vite.config.ts 的 unplugin-vue-components 负责。',
    ).not.toMatch(/app\s*\.\s*use\s*\(\s*ElementPlus/)
    // 命名导入（ElMessage 等）是可以的：它们可被 tree-shake
    expect(mainCode).not.toMatch(/^import\s+ElementPlus\s+from/m)
  })

  it('main.ts 没有引整包样式表', () => {
    // `element-plus/dist/index.css` 是全量样式；按需模式下样式由 resolver 逐组件引入。
    expect(
      mainCode,
      'main.ts 又引了 `element-plus/dist/index.css` —— 那是全量样式，' +
        '按需模式下每个组件的样式由 ElementPlusResolver 单独引入。',
    ).not.toMatch(/element-plus\/dist\/index\.css/)
  })

  it('vite.config.ts 没有把 element-plus 圈进 manualChunks', () => {
    // 这是**第二个**根因：手写 manualChunks 会强行把整个库圈成一个 chunk，
    // **绕过**按需引入的裁剪。上面那两条守卫全绿、产物却又涨回 900 KB，
    // 就是这一行造成的 —— 所以必须单独守。
    expect(
      viteCode,
      'vite.config.ts 的 manualChunks 里出现了 element-plus —— ' +
        '手写分块会绕过按需裁剪，前面所有优化都会被抵消。',
    ).not.toMatch(/element-plus/)
  })

  it('vite.config.ts 确实接上了按需引入插件，且顺序在 vue() 之后', () => {
    // 顺序判据不是洁癖：Components 靠分析**编译后**的 render 函数收集组件，
    // 排在 vue() 前面就什么也收集不到 —— 表现为「构建成功但组件全丢」，
    // 是一种不会让任何构建报错的静默失败。
    expect(viteCode, 'vite.config.ts 里没有 unplugin-vue-components').toMatch(
      /unplugin-vue-components/,
    )
    const vueAt = viteCode.indexOf('vue()')
    const compAt = viteCode.indexOf('Components(')
    expect(vueAt, 'vite.config.ts 里找不到 vue()').toBeGreaterThan(-1)
    expect(compAt, 'vite.config.ts 里找不到 Components(').toBeGreaterThan(-1)
    expect(
      compAt,
      'Components() 必须排在 vue() **之后** —— 排在前面会收集不到任何组件，' +
        '表现为「构建通过但页面全白」，而且不会有任何报错。',
    ).toBeGreaterThan(vueAt)
  })

  it('测试侧也接了解析器（否则构建与测试用两套组件解析）', () => {
    expect(
      vitestCode,
      'vitest.config.ts 里没有 unplugin-vue-components —— ' +
        'main.ts 不再整包注册之后，测试会解析不到 el-* 组件。',
    ).toMatch(/unplugin-vue-components/)
  })

  it('⚠️ 语言包仍然接在 el-config-provider 上（这条丢了不产生任何测试失败）', () => {
    // 去掉整包注册后，语言包失去了 `app.use(ElementPlus, { locale })` 这个注入点。
    // 忘了接 `<el-config-provider :locale>` 的话：
    //   - 构建正常、测试全绿、页面能开；
    //   - 只是 `el-pagination` 从「共 0 条 / 前往 1 页」变成「Total 0 / Go to 1」。
    //
    // 这是一类**不红但错了**的退化，所以单独守。
    expect(
      appCode,
      'src/App.vue 里没有 el-config-provider —— 按需模式下语言包无处注入，' +
        '分页/日期选择器文案会静默变英文。',
    ).toMatch(/el-config-provider/)
    expect(appCode, 'el-config-provider 没有绑定 locale').toMatch(/:locale\s*=/)
    expect(appCode, 'App.vue 没有引入中文语言包').toMatch(/lang\/zh-cn/)
  })

  it('⚠️ src/components.d.ts 存在（它一没，类型检查器就变瞎）', () => {
    // 这条是本轮**最反直觉**的一条。
    //
    // 改按需引入之前，`components.d.ts` 不存在，于是 `el-table` 是**未知组件**、
    // 插槽行是 `any` —— `vue-tsc` 报的错是 **0 条**。
    // 加上它之后，同一份代码报出 **22 条** TS2345。
    //
    // 也就是说：那个「0 错误」不是干净，是**盲**。
    // 一旦有人把这个文件删掉（它是构建生成的，`git clean` 一下就没了），
    // 类型检查会**静默退回到全盲**，而没有任何测试会红。
    const p = join(root, 'src/components.d.ts')
    expect(
      existsSync(p),
      'src/components.d.ts 不存在 —— 它由 vite 构建生成、但需要提交。' +
        '没有它，vue-tsc 认不出 el-* 组件，插槽行退化成 any，' +
        '类型错误会全部静默消失（实测 22 → 0 条）。',
    ).toBe(true)
    const dts = readFileSync(p, 'utf8')
    // 它必须真的声明了组件，而不是一个空壳
    expect(
      (dts.match(/typeof import\('element-plus/g) || []).length,
      'components.d.ts 里没有组件声明 —— 它可能是个空壳，同样会让类型检查变盲',
    ).toBeGreaterThan(10)
  })

  it('README「后台构建体积」一节的数字与 size-report.mjs 的预算一致', () => {
    // 为什么需要这条：第 47~49 三轮得到的首屏数字，
    // 如果只躺在提交信息里，README 上看不到，而**预算**在脚本里。
    // 两者一旦漂移，就会出现「文档说 161、脚本按 220 放行」这种情况 ——
    // 没人会发现，直到某天首屏真的涨上去。
    //
    // 这与 `server/internal/domain/readme_drift_test.go`（守迁移数）
    // 同一个思路：README 里的数字必须**对着真源核**，
    // 而不是等着某个人想起来更新。
    // ⚠️ 路径是 `../README.md`：那份 README 在 `laiyipao/` 下，不在 `admin/` 下。
    // 我第一版写成 `read('README.md')`，守卫立刻报「找不到文件」——
    // 正是它被设计成**响亮失败**而不是静默跳过的原因。
    const readme = read('../README.md')
    const script = read('scripts/size-report.mjs')

    const raw = script.match(/firstLoadRawKB:\s*(\d+)/)
    const gz = script.match(/firstLoadGzipKB:\s*(\d+)/)
    expect(raw, 'size-report.mjs 里读不到 firstLoadRawKB —— 预算被改名或删了').not.toBeNull()
    expect(gz, 'size-report.mjs 里读不到 firstLoadGzipKB').not.toBeNull()
    const budgetRaw = Number(raw![1])
    const budgetGz = Number(gz![1])

    const start = readme.indexOf('## 后台构建体积')
    expect(start, 'README 里没有「## 后台构建体积」这一节').toBeGreaterThan(-1)
    const sec = readme.slice(start, start + 1200)

    expect(
      sec,
      `README 的「后台构建体积」一节里没有出现脚本当前的预算 ${budgetRaw} —— 改预算时忘了同步 README`,
    ).toContain(String(budgetRaw))
    expect(sec, `README 里没有出现 gzip 预算 ${budgetGz}`).toContain(String(budgetGz))

    const m = sec.match(/首屏合计[^\d]*(\d+) KB/)
    expect(m, 'README 里找不到「首屏合计」的实测值').not.toBeNull()
    const measured = Number(m![1])
    expect(
      measured,
      `README 记的首屏 ${measured} KB 已经 >= 预算 ${budgetRaw} KB —— 预算失效，该重新评估而不是继续放行`,
    ).toBeLessThan(budgetRaw)
  })

  it('记录本轮实测基线（供下次对比；改动后请更新这里）', () => {
    const baseline = { totalKB: 1101, elementKB: 286, gzipKB: 401 }
    // 只做「数据完整」的自检，不做阈值判断 ——
    // 体积会随功能增长，阈值一刀切会逼着人放宽标准而不是优化。
    // 真正的守门在上面 6 条：这 6 条红了就说明优化机制被拆了。
    expect(baseline.elementKB).toBeLessThan(baseline.totalKB)
    expect(baseline.gzipKB).toBeLessThan(baseline.totalKB)
    // eslint-disable-next-line no-console
    console.log(
      `  基线：总计 ${baseline.totalKB} KB / element ${baseline.elementKB} KB / gzip ${baseline.gzipKB} KB` +
        `（改动前 1981 / 901 / —）`,
    )
  })
})
