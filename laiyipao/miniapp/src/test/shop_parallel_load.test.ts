// @vitest-environment node
/**
 * shop 页 load 并行加载守卫（2026-10-31 实质优化第 30 轮）。
 *
 * ## 为什么需要它
 *
 * 商城页 load 同时需要商品列表与钱包余额，两者互不依赖。此前注释
 * 写着「并发」，实现却是先等商品再等钱包 —— 首屏白等一倍时间。
 * 已改为 `Promise.all` 真正并行。
 *
 * 但「并行」很容易被无意改回串行（编译照过、行为测试全绿），
 * 所以用源码扫描把它变成结构约束。
 *
 * 判据：
 *   - `async function load()` 块内必须出现 `Promise.all([`；
 *   - `load` 内不允许出现「先 `await api.fetchShop()` 再单独一行
 *     `await api.fetchWallet()`」的串行模式。
 */

import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const SHOP_VUE = join(__dirname, '../pages/shop/shop.vue')

/** 提取 load 函数块。 */
function loadBlock(): string {
  const src = readFileSync(SHOP_VUE, 'utf-8')
  const start = src.indexOf('async function load()')
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
  throw new Error('未找到 load 函数块闭合')
}

describe('shop 页 load 并行加载守卫', () => {
  it('load 必须使用 Promise.all 并行拉取商品与钱包', () => {
    const block = loadBlock()
    expect(block).toContain('Promise.all([')
    expect(block).toContain('api.fetchShop()')
    expect(block).toContain('api.fetchWallet()')
  })

  it('load 内不允许出现串行的 await api.fetchWallet() 独立行', () => {
    const block = loadBlock()
    // 串行模式：`await api.fetchWallet()` 独立成行（前面只有空白）。
    const serial = block.match(/^\s*await\s+api\.fetchWallet\(\)\s*$/gm)
    expect(serial).toBeNull()
  })
})