// @vitest-environment node
/**
 * me 页 onShow 并行加载守卫（2026-10-31 实质优化第 29 轮）。
 *
 * ## 为什么需要它
 *
 * me 页 onShow 会拉取 8 个独立数据源（资料/槽位/防线/挑战历史/战报
 * 历史/购买/兑换/订单）。这些请求互不依赖，串行 await 会让首屏
 * 等 N 个 RTT；并行后只等最慢的那一个。
 *
 * 但「并行」是一个很容易被无意改回串行的性质：只要有人把
 * `Promise.all` 拆回一行一个 `await`，编译照过、现有测试全绿，
 * 页面却悄悄变慢。所以这里用源码扫描把它变成结构约束。
 *
 * 判据：
 *   - onShow 块内必须出现 `Promise.all([`；
 *   - onShow 块内不允许出现「单独一行的 `await loadX()`」（串行模式）。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const ME_VUE = join(__dirname, '../pages/me/me.vue')

/** 提取 onShow 块（从 `onShow(async` 到 `})` 闭合）。 */
function onShowBlock(): string {
  const src = readFileSync(ME_VUE, 'utf-8')
  const start = src.indexOf('onShow(async')
  expect(start).toBeGreaterThan(-1)
  const blockStart = src.indexOf('{', start)
  let depth = 0
  for (let i = blockStart; i < src.length; i++) {
    if (src[i] === '{') depth++
    else if (src[i] === '}') {
      depth--
      if (depth === 0) return src.slice(start, i + 1)
    }
  }
  throw new Error('未找到 onShow 块闭合')
}

describe('me 页 onShow 并行加载守卫', () => {
  it('onShow 必须使用 Promise.all 并行加载', () => {
    const block = onShowBlock()
    expect(block).toContain('Promise.all([')
  })

  it('onShow 内不允许出现串行的单独 await loadX() 行', () => {
    const block = onShowBlock()
    // 串行模式：`await loadX()` 独立成行（前面只有空白）。
    const serial = block.match(/^\s*await\s+load\w+\(\)\s*$/gm)
    expect(serial).toBeNull()
  })
})