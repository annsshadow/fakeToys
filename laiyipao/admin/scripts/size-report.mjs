#!/usr/bin/env node
/**
 * 首屏体积报告 —— 用「用户真正等多少」代替「dist 一共多少」。
 *
 * ## 为什么不用 dist 总量
 *
 * 路由是代码分割的。`dist` 总量把**每个懒加载页**的代码都算进去，
 * 于是 534 KB 的 echarts 看起来占了产物的 48% ——
 * 而它其实**只在打开 Dashboard 时才下载**，首屏根本不等它。
 *
 * 用错指标会得出「必须优化 echarts」的结论，而实际上没人等它；
 * 也会**低估**真正的首屏优化（把 901 KB 的 element-plus 从首屏挪走，
 * 收益是 1051 KB，而 dist 总量只反映 520 KB）。
 *
 * 所以这里做两件事：
 *  1. 从 `index.html` 的入口出发，沿**静态** import 传递闭包 → **首屏**
 *  2. 再逐个算路由懒加载块的额外代价 → 谁贵、贵在哪个页面
 *
 * ## 用法
 *
 *   npm run build && npm run size          # 只报告
 *   npm run build && npm run size:check    # 超预算则退出码 1
 *
 * 为什么是独立脚本而**不是**一个 vitest 用例：
 * 它需要先有 `dist`，而 `npm run build` 要 10 秒以上 ——
 * 把构建塞进单元测试会让整个测试套件依赖打包器，
 * 而且在一台刚 clone 还没 build 的机器上必然失败。
 * 结构性的回归（有人又写回 `app.use(ElementPlus)`）由
 * `tests/bundle_size.test.ts` 在源码层守，那是秒级的；
 * 本脚本负责**量**，两件事分工不同。
 */
import { readFileSync, readdirSync, existsSync, statSync } from 'node:fs'
import { join, dirname, relative } from 'node:path'
import { fileURLToPath } from 'node:url'
import { gzipSync } from 'node:zlib'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const dist = join(root, 'dist')
const assets = join(dist, 'assets')

/**
 * 预算。留了余量，让正常加功能不会误报；
 * 但一次「整包注册」级别的回归（实测首屏 161 → 1195 KB）会超出 5 倍，必红。
 *
 * ⚠️ 这两个数字改过两次，每次都是因为**我的测量本身错了**：
 *  1. 第一次拿 dist 总量当首屏（1981 KB），而 dist 里还残留着更早的构建文件；
 *  2. 第二次 CSS 把 27 个文件全加了，实际首屏只 link 1 个（8 KB）。
 * 所以基线必须**由本脚本实测**、改动后**重测**，
 * 不要从提交信息里抄一个数过来 —— 那正是我两次都做错的事。
 */
export const BUDGET = {
  firstLoadRawKB: 220, // 实测 161
  firstLoadGzipKB: 80, // 实测 62
}

if (!existsSync(assets)) {
  console.error('找不到 dist/assets —— 请先 `npm run build`。')
  process.exit(2)
}

/**
 * ⚠️ 陈旧 dist 检查。
 *
 * 本脚本**只读 dist**，不构建。所以忘记 build 时它会拿上一次构建的产物
 * 给出一个**看起来完全正常的错误数字**。
 *
 * 这不是假想：第 47 轮我就是在一份**残留了旧文件**的 dist 上
 * 测出「1981 KB」，重新干净构建后实际是 1621 KB；
 * 第 48 轮跑本脚本时 dist 还是上一轮「回退对比」留下的，
 * 于是它报出 1378 KB 的首屏 —— 而当前代码的首屏是 328 KB。
 *
 * 数字错了但**没有任何报错**，这比崩溃更难发现。
 * 所以这里显式比对：dist 比任何源文件旧，就拒绝出报告。
 */
function newestMtime(dir, skip = new Set()) {
  let newest = 0
  let newestRel = ''
  const walk = (d) => {
    for (const name of readdirSync(d)) {
      if (skip.has(name)) continue
      const p = join(d, name)
      const st = statSync(p)
      if (st.isDirectory()) {
        walk(p)
      } else if (st.mtimeMs > newest) {
        newest = st.mtimeMs
        newestRel = relative(root, p)
      }
    }
  }
  walk(dir)
  return { newest, newestRel }
}

const distMtime = statSync(join(dist, 'index.html')).mtimeMs
const src = newestMtime(join(root, 'src'), new Set(['components.d.ts']))
const cfg = newestMtime(root, new Set(['node_modules', 'dist', '.git', 'coverage', 'public', 'scripts']))
const newestSrc = Math.max(src.newest, cfg.newest)

if (distMtime < newestSrc) {
  console.error('*** dist 比源文件旧 —— 这份产物是上一次构建的，报告会是错的。***')
  console.error(`    最新的源文件：${src.newestRel || cfg.newestRel}`)
  console.error('    请先 `npm run build:only`（`build` 会先跑 vue-tsc，慢一点）。')
  process.exit(2)
}

const kb = (n) => Math.round(n / 1024)

/** vite 产物里的静态 import 形如 `from"./x-hash.js"` */
function staticDeps(file) {
  const src = readFileSync(file, 'utf8')
  const deps = new Set()
  for (const m of src.matchAll(/from\s*["']\.\/([^"']+\.js)["']/g)) deps.add(m[1])
  for (const m of src.matchAll(/import\s*["']\.\/([^"']+\.js)["']/g)) deps.add(m[1])
  return [...deps]
}

/** 传递闭包。动态 import() 不算 —— 那正是懒加载的边界。 */
function closure(entry) {
  const seen = new Set()
  const stack = [entry]
  while (stack.length) {
    const f = stack.pop()
    if (seen.has(f)) continue
    const p = join(assets, f)
    if (!existsSync(p)) continue
    seen.add(f)
    for (const d of staticDeps(p)) stack.push(d)
  }
  return [...seen]
}

const html = readFileSync(join(dist, 'index.html'), 'utf8')
const entry = html.match(/src="\/assets\/(index-[^"]+\.js)"/)
if (!entry) {
  console.error('index.html 里找不到 entry 脚本 —— 构建配置可能变了，请更新本脚本。')
  process.exit(2)
}

const first = closure(entry[1])

/**
 * 首屏 CSS：**只算 index.html 真正 link 的那些**。
 *
 * ⚠️ 第一版把 `assets/` 下**全部** CSS 加总 —— 那 27 个文件里 26 个是
 * 懒加载页的样式，首屏一个都不加载。于是报出「首屏 CSS 175 KB」，
 * 而 index.html 里只 link 了 **1** 个文件。
 *
 * 症状与本文件开头的陈旧 dist 检查一模一样：数字**偏大**、格式正常、
 * **不报任何错**。而且这属于最讽刺的一类错误 ——
 * 本文件存在的全部意义就是「用对指标」，结果自己把一个只该算进
 * dist 总量的东西算进了首屏。
 *
 * 正确做法：从 index.html 解析 `<link rel="stylesheet">`，
 * 再补上首屏 JS 闭包里静态引用到的 CSS
 * （vite 通常会合并进 entry CSS，但显式检查比假设它会合并可靠）。
 */
const linkedCss = [
  ...html.matchAll(/<link[^>]+rel="stylesheet"[^>]+href="\/assets\/([^"]+\.css)"/g),
].map((m) => m[1])
for (const f of first) {
  const src = readFileSync(join(assets, f), 'utf8')
  for (const m of src.matchAll(/["']\.\/([^"']+\.css)["']/g)) {
    if (!linkedCss.includes(m[1]) && existsSync(join(assets, m[1]))) linkedCss.push(m[1])
  }
}

let jsRaw = 0
let jsGz = 0
for (const f of first) {
  const b = readFileSync(join(assets, f))
  jsRaw += b.length
  jsGz += gzipSync(b, { level: 9 }).length
}
let cssRaw = 0
let cssGz = 0
for (const f of linkedCss) {
  const b = readFileSync(join(assets, f))
  cssRaw += b.length
  cssGz += gzipSync(b, { level: 9 }).length
}

const allJs = readdirSync(assets).filter((f) => f.endsWith('.js'))
const allCss = readdirSync(assets).filter((f) => f.endsWith('.css'))
let distRaw = 0
for (const f of allJs) distRaw += statSync(join(assets, f)).size
let distCssRaw = 0
for (const f of allCss) distCssRaw += statSync(join(assets, f)).size

console.log('=== 首屏（index.html 入口的静态 import 闭包）===')
for (const f of [...first].sort()) {
  console.log('  ' + f.padEnd(32) + kb(statSync(join(assets, f)).size) + ' KB')
}
for (const f of [...linkedCss].sort()) {
  console.log('  ' + f.padEnd(32) + kb(statSync(join(assets, f)).size) + ' KB  <- index.html link 的')
}
console.log('  ' + '-'.repeat(52))
console.log(
  `  JS  ${kb(jsRaw)} KB（gzip ${kb(jsGz)}）  ` +
    `CSS ${kb(cssRaw)} KB（gzip ${kb(cssGz)}；${linkedCss.length}/${allCss.length} 个文件进了首屏）`,
)
console.log(
  `  首屏合计 ${kb(jsRaw + cssRaw)} KB（gzip ${kb(jsGz + cssGz)}）  ` +
    `预算 ${BUDGET.firstLoadRawKB} / ${BUDGET.firstLoadGzipKB} KB`,
)

console.log('')
console.log(
  `=== dist 总量 ${kb(distRaw + distCssRaw)} KB（${allJs.length} js + ${allCss.length} css）` +
    `——仅供参照，不是首屏 ===`,
)

console.log('')
console.log('=== 各路由的额外代价（进那个页面才付）===')
const routes = allJs
  .filter((f) => /View-|Layout/.test(f))
  .map((f) => {
    const extra = closure(f).filter((x) => !first.includes(x))
    return { f, n: extra.length, size: extra.reduce((a, e) => a + statSync(join(assets, e)).size, 0) }
  })
  .sort((a, b) => b.size - a.size)
for (const x of routes) {
  if (x.size < 3 * 1024) continue
  console.log('  ' + x.f.padEnd(32) + '+' + kb(x.size) + ' KB（' + x.n + ' 个额外 chunk）')
}

const check = process.argv.includes('--check')
if (check) {
  const over = []
  if (kb(jsRaw + cssRaw) > BUDGET.firstLoadRawKB) {
    over.push(`首屏 raw ${kb(jsRaw + cssRaw)} KB > ${BUDGET.firstLoadRawKB} KB`)
  }
  if (kb(jsGz + cssGz) > BUDGET.firstLoadGzipKB) {
    over.push(`首屏 gzip ${kb(jsGz + cssGz)} KB > ${BUDGET.firstLoadGzipKB} KB`)
  }
  if (over.length) {
    console.error('')
    console.error('首屏超预算：')
    for (const o of over) console.error('  - ' + o)
    console.error('')
    console.error('先看是不是有人在入口里整包注册了什么（那会让首屏直接多几百 KB）。')
    process.exit(1)
  }
  console.log('')
  console.log('首屏在预算内。')
}
