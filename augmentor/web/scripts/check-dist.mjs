// 构建产物体检（A207）：对 dist/ 做一组不变量断言，红了就是构建产物劣化
//
// 用法：`npm run build:check`（先构建再体检）。全部判据只降不升——
// 某一档超限了，先问「为什么涨」，修好了再放宽常数，不许直接抬阈值。
import { readFileSync, readdirSync, statSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import path from 'node:path'

const dist = path.join(path.dirname(fileURLToPath(import.meta.url)), '..', 'dist')
const assets = path.join(dist, 'assets')

const fail = (msg) => {
  console.error(`[dist-check] ✗ ${msg}`)
  process.exitCode = 1
}
const ok = (msg) => console.log(`[dist-check] ✓ ${msg}`)

const files = readdirSync(assets)
const kb = (name) => statSync(path.join(assets, name)).size / 1024
const byPrefix = (prefix) => files.filter((f) => f.startsWith(prefix))

// 1) 入口壳：index.html + 入口 js 都很小（壳里的重逻辑全是懒加载路由）
const html = readFileSync(path.join(dist, 'index.html'), 'utf8')
const entryJs = files.filter((f) => /^index-.*\.js$/.test(f))
if (entryJs.length !== 1) fail(`入口 js 恰好 1 份，实际 ${entryJs.length}`)
else if (kb(entryJs[0]) > 50) fail(`入口 js ${entryJs[0]} 超过 50 kB（壳被业务逻辑污染）`)
else ok(`入口 js ${entryJs[0]} = ${kb(entryJs[0]).toFixed(1)} kB ≤ 50`)

// 2) 路由级代码分割在场：11 个页面各一个独立 chunk（懒加载没被摇掉）
const pages = ['Dashboard', 'DataManagement', 'Augmentation', 'Quality', 'Security', 'Export', 'Analysis', 'Multimodal', 'Versions', 'Settings', 'System']
for (const p of pages) {
  if (!byPrefix(`${p}-`).length) fail(`缺少页面 chunk ${p}-*.js（懒加载被摇掉了？）`)
}
if (process.exitCode) console.log('[dist-check] （上一格已红，本格结果仅供参考）')
else ok(`页面 chunk 齐全：${pages.length} 页`)

// 3) 页面 chunk 保持轻量：业务页不该拖进 vendor（vendor 各自成块）
for (const p of pages) {
  for (const f of byPrefix(`${p}-`)) {
    if (kb(f) > 100) fail(`页面 chunk ${f} = ${kb(f).toFixed(1)} kB 超过 100 kB（vendor 漏进业务块？）`)
  }
}
ok('页面 chunk 均 ≤ 100 kB')

// 4) 首屏不加载 echarts：入口壳的 modulepreload 名单里不许出现 echarts
//    （echarts-vendor 只应由 Analysis 路由的动态 import 拉取）
const preloads = [...html.matchAll(/<link rel="modulepreload"[^>]*href="([^"]+)"/g)].map((m) => m[1])
if (preloads.some((p) => p.includes('echarts'))) fail(`首屏 modulepreload 名单含 echarts：${preloads}`)
else ok(`首屏 modulepreload 无 echarts（${preloads.length} 项）`)

// 5) 已知大 vendor 的基线（只降不升：echarts 全量 ~1.1 MB、antd 全量 ~0.95 MB 是现状基准）
for (const [prefix, limitKb] of [['antd-vendor-', 1200], ['echarts-vendor-', 1500], ['react-vendor-', 300]]) {
  for (const f of byPrefix(prefix)) {
    if (kb(f) > limitKb) fail(`${f} = ${kb(f).toFixed(1)} kB 超过基线 ${limitKb} kB（vendor 在涨）`)
  }
}
ok('大 vendor 未超基线（antd ≤1200 / echarts ≤1500 / react ≤300 kB）')

// 6) index.html 引用的资源都真实存在（防「改了构建、忘了同步产物」的半截提交）
for (const ref of [...html.matchAll(/(?:src|href)="\/(assets\/[^"]+)"/g)].map((m) => m[1])) {
  if (!files.includes(ref.replace('assets/', ''))) fail(`index.html 引用了不存在的产物 ${ref}`)
}
ok('index.html 引用的产物全部在场')

if (process.exitCode) {
  console.error('[dist-check] 体检未通过')
} else {
  console.log('[dist-check] 体检通过')
}
