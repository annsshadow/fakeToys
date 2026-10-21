#!/usr/bin/env node
/**
 * 小程序包体积报告 / 闸门（P6-9）。
 *
 * ## 为什么小程序要单独量
 *
 * admin 的 `size-report.mjs` 量的是「首屏」—— 那是 SPA 的概念
 * （路由懒加载，首屏只等 index.html 的静态 import 闭包）。
 *
 * 小程序不是 SPA：它按**包**分发。本项目**没有分包配置**，
 * 所以 `dist/build/mp-weixin` 里的**全部**文件都进「主包」，
 * 而微信主包上限是 **2 MB**。
 *
 * 所以这里的指标是：**主包总大小**（而不是首屏）。
 * 换句话说：微信的 2 MB 闸门对我们是「全量」的，
 * 任何新增页面/依赖都会**立刻**吃掉这个预算。
 *
 * ## 用法
 *
 *   pnpm build:mp-weixin && pnpm size         # 只报告
 *   pnpm build:mp-weixin && pnpm size:check   # 超预算则退出码 1
 *
 * ## 为什么是独立脚本而不是一个 vitest 用例
 *
 * 与 admin 同款理由：它需要先有 `dist`，而构建要十几秒 ——
 * 把构建塞进单元测试会让整个测试套件依赖打包器。
 * 源码层的结构性格���（比如「不许整包注册」）由单元测试守，
 * 本脚本负责**量**。
 */
import { existsSync, readdirSync, statSync } from 'node:fs'
import { join, dirname, relative } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = join(dirname(fileURLToPath(import.meta.url)), '..')
const dist = join(root, 'dist', 'build', 'mp-weixin')

/**
 * 预算。
 *
 * 微信主包硬上限是 2 MB —— 我们把闸门设在 **1.5 MB**：
 * 留 25% 余量，让正常加功能不会误报，
 * 而一次「整包塞进一个大依赖」的回归会必红。
 *
 * ⚠️ 这个数字必须**由本脚本实测**、改动后**重测**，
 * 不要从提交信息里抄一个数过来 —— 基线抄错就会变成一道
 * 「永远绿」的闸门（那比没有闸门更坏）。
 */
export const BUDGET = {
  mainPackageKB: 1536, // 1.5 MB；微信硬上限 2 MB，实测当前约 232 KB
}

if (!existsSync(dist)) {
  console.error('找不到 dist/build/mp-weixin —— 请先 `pnpm build:mp-weixin`。')
  process.exit(2)
}

const kb = (n) => Math.round(n / 1024)

/**
 * ⚠️ 陈旧产物检查。
 *
 * 本脚本**只读 dist**，不构建。忘记 build 时它会拿上一次的产物
 * 给出一个**看起来完全正常的错误数字**。
 *
 * 数字错了但**没有任何报错**，这比崩溃更难发现 ——
 * 所以这里显式比对：产物比任何源文件旧，就拒绝出报告。
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

const distMtime = statSync(join(dist, 'app.json')).mtimeMs
const src = newestMtime(join(root, 'src'), new Set(['components.d.ts']))
const cfg = newestMtime(root, new Set(['node_modules', 'dist', '.git', 'coverage', 'scripts']))
const newestSrc = Math.max(src.newest, cfg.newest)
if (distMtime < newestSrc) {
  console.error('*** 产物比源文件旧 —— 这份产物是上一次构建的，报告会是错的。***')
  console.error(`    最新的源文件：${src.newestRel || cfg.newestRel}`)
  console.error('    请先 `pnpm build:mp-weixin`。')
  process.exit(2)
}

const files = []
const walk = (d) => {
  for (const name of readdirSync(d)) {
    const p = join(d, name)
    const st = statSync(p)
    if (st.isDirectory()) walk(p)
    else files.push({ path: p, size: st.size })
  }
}
walk(dist)

const total = files.reduce((a, f) => a + f.size, 0)
const byExt = new Map()
for (const f of files) {
  const ext = f.path.slice(f.path.lastIndexOf('.') + 1) || '(no-ext)'
  byExt.set(ext, (byExt.get(ext) ?? 0) + f.size)
}

console.log('=== 小程序主包（dist/build/mp-weixin，本项目未分包 → 全部进主包）===')
for (const [ext, size] of [...byExt.entries()].sort((a, b) => b[1] - a[1])) {
  console.log(`  .${ext.padEnd(8)} ${kb(size).toString().padStart(6)} KB`)
}
console.log('  ' + '-'.repeat(30))
console.log(
  `  合计 ${kb(total)} KB（${files.length} 个文件）  预算 ${BUDGET.mainPackageKB} KB（微信硬上限 2048 KB）`,
)

console.log('')
console.log('=== 最大的 5 个文件 ===')
for (const f of files.sort((a, b) => b.size - a.size).slice(0, 5)) {
  console.log(`  ${relative(dist, f.path).padEnd(40)} ${kb(f.size)} KB`)
}

if (process.argv.includes('--check')) {
  if (kb(total) > BUDGET.mainPackageKB) {
    console.error('')
    console.error(`主包超预算：${kb(total)} KB > ${BUDGET.mainPackageKB} KB`)
    console.error('（微信硬上限 2048 KB —— 撞上就传不上去。）')
    console.error('先看是不是新加了什么大依赖；必要时改成分包（pages.json 的 subPackages）。')
    process.exit(1)
  }
  console.log('')
  console.log('主包在预算内。')
}
