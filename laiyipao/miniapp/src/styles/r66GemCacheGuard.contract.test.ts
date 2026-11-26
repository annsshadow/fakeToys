// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R66 宝石图鉴缓存 JSON 解析兜底契约（test-first RED）。
 * 损坏的 localStorage 缓存不得让 setup 或 onShow 抛 SyntaxError 整页崩溃；
 * JSON.parse 必须纳入 try/catch，catch 回退 normalizeGemGain(null) 诚实空态。
 * 先 RED 再实现；锚点 v1 / v3-r3 / r6 不变，禁新增 r66 视觉令牌。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { normalizeGemGain } from '../pages/_shared/r10GemGain'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

function blockAfter(text: string, keyword: string): { start: number; end: number; text: string } | null {
  const k = text.indexOf(keyword)
  if (k < 0) return null
  const open = text.indexOf('{', k)
  if (open < 0) return null
  let depth = 0
  for (let i = open; i < text.length; i++) {
    const ch = text[i]
    if (ch === '{') depth++
    else if (ch === '}') {
      depth--
      if (depth === 0) return { start: open, end: i, text: text.slice(open, i + 1) }
    }
  }
  return null
}

describe('R66 宝石图鉴缓存 JSON 解析兜底契约', () => {
  const gems = readSrc('pages/gems/gems.vue')

  it('JSON.parse 纳入 try/catch，损坏缓存回退诚实空态而非整页崩溃', () => {
    const fn = blockAfter(gems, 'function loadGemGain()')
    expect(fn, '必须能定位 loadGemGain 函数块').not.toBeNull()
    const b = fn!.text
    const tryBlock = blockAfter(b, 'try')
    expect(tryBlock, 'loadGemGain 必须存在 try 块').not.toBeNull()
    const parseIdx = b.indexOf('JSON.parse')
    expect(parseIdx, 'loadGemGain 必须解析缓存').toBeGreaterThan(-1)
    const inTry = parseIdx > tryBlock!.start && parseIdx < tryBlock!.end
    expect(inTry, 'JSON.parse 必须位于 try 块内（原始缺陷：裸在 try 外，损坏缓存抛 SyntaxError 致整页崩溃）').toBe(true)
    const catchBlock = blockAfter(b, 'catch')
    expect(catchBlock, '必须存在 catch 兜底').not.toBeNull()
    const fallback = /normalizeGemGain\(\s*null\s*\)/.test(catchBlock!.text) || /=\s*null/.test(catchBlock!.text)
    expect(fallback, 'catch 必须回退：normalizeGemGain(null) 或将解析结果置 null 后统一 normalizeGemGain').toBe(true)
  })

  it('setup 与 onShow 两处调用及诚实空态模板锚点保留', () => {
    const calls = gems.match(/loadGemGain\(\)/g) || []
    expect(calls.length, 'setup 同步调用与 onShow 内调用都必须在').toBeGreaterThanOrEqual(2)
    expect(gems, 'onShow 中必须再次调用 loadGemGain').toMatch(/onShow\(\(\)\s*=>\s*\{[^}]*loadGemGain\(\)/)
    expect(gems).toContain('class="r10-gem-gain')
    expect(gems).toContain('gemGain.available')
    expect(gems).toContain('佩戴收益未上报，先看词条描述')
    expect(gems).toContain('r10-gem-empty')
    expect(gems).toContain('role="status"')
  })

  it('不变量：helper 契约、bag 写入方、模板诚实空态与令牌三锚点不变', () => {
    const helper = readSrc('pages/_shared/r10GemGain.ts')
    const bag = readSrc('pages/bag/bag.vue')
    const tokens = readSrc('styles/tokens.css')
    expect(helper).toContain('export function normalizeGemGain')
    expect(helper).toContain('export function r10GemGainCacheKey')
    expect(helper).toContain('lyp:r10:gem-gain:')
    const sets = bag.match(/localStorage\.setItem\(\s*r10GemGainCacheKey\(/g) || []
    expect(sets.length, 'bag.vue 必须保留两处 setItem 写入方').toBeGreaterThanOrEqual(2)
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(gems + tokens, '不得新增 r66 视觉令牌').not.toMatch(/--r66-/)
  })

  it('normalizeGemGain(null) 返回诚实空态（真实运行时调用 helper）', () => {
    const v = normalizeGemGain(null)
    expect(v.available).toBe(false)
    expect(v.text).toContain('佩戴收益未上报')
    expect(v.rows).toEqual([])
  })
})