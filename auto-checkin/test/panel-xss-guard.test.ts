import { readFileSync } from 'node:fs'
import { test } from 'node:test'
import assert from 'node:assert/strict'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

// 第 38 轮：面板 XSS 守卫。
//
// 面板曾把 `r.message` / `s.lastMessage` / `s.siteName` / `s.lastScreenshot`
// 直接 innerHTML 插值 —— 这些字段来自**远端站点响应**（justwoker 的
// api.message 等），站点返回 `<img onerror=...>` 就能在 localhost 面板执行
// 脚本。修复后所有不可信插值必须过 `esc()`。本守卫用源码扫描钉住：
// 裸插值模式一旦复活即红。

const here = dirname(fileURLToPath(import.meta.url))
const appJs = readFileSync(join(here, '..', 'src', 'public', 'app.js'), 'utf-8')

const BANNED_PATTERNS: Array<[string, RegExp]> = [
  // 只拦「直接输出」的裸插值：`${s.lastMessage ? esc(...) : ...}` 里的
  // 字段出现在三元**条件**（不输出字符串），是安全的，不算裸插值。
  ['记录表 message 直接输出', />\$\{r\.message(\s*\|\||\}|\s*<)/g],
  ['记录表 siteName 直接输出', />\$\{r\.siteName\}/g],
  ['站点卡 lastMessage 直接输出', />\$\{s\.lastMessage(\s*\|\||\}|\s*<)/g],
  ['站点卡 siteName 直接输出', />\$\{s\.siteName\}/g],
  ['截图 href 裸插值', /href="\/\$\{s\.lastScreenshot\}/g],
  ['图例站名直接输出', />\$\{s\.name\}/g],
  ['提示框站名直接输出', /\$\{site\.name\}:/g],
]

for (const [name, re] of BANNED_PATTERNS) {
  test(`面板不再裸插值：${name}`, () => {
    assert.doesNotMatch(appJs, re, `${name} 必须经过 esc() 转义（第 38 轮 XSS 修复）`)
  })
}

test('esc() 转义函数存在且覆盖五个特殊字符', () => {
  assert.match(appJs, /function esc\(/)
  assert.ok(appJs.includes('&amp;') && appJs.includes('&lt;') && appJs.includes('&gt;')
    && appJs.includes('&quot;') && appJs.includes('&#39;'))
})
