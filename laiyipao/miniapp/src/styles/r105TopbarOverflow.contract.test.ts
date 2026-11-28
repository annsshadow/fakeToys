// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R105 index 顶栏 768 横溢修复契约（test-first RED）。
 * 真实 fs.readFileSync 读 index.vue / tokens.css，禁止内嵌实现副本。
 * 以唯一 class 锚点定位 CSS 块（不依赖行号）；锚点非唯一则定位失败，测试自责。
 * 缺口 3 it：.player-info min-width:0 / ≤900px .res-row flex-wrap:wrap / .topbar max-width+box-sizing。
 * 不变量 7 it：flex 结构 / 基础 nowrap / R99+R102 三件套 / R100+R101 ann-item / 四胶囊 / 令牌三锚点 / 零 --r105-。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join } from 'node:path'

const dir = dirname(fileURLToPath(import.meta.url))
const srcRoot = join(dir, '..')

function read(rel: string): string {
  return readFileSync(join(srcRoot, rel), 'utf8')
}

function occurrences(src: string, sub: string): number {
  let n = 0, i = 0
  while ((i = src.indexOf(sub, i)) >= 0) { n++; i += sub.length }
  return n
}

// 唯一 class 锚点定位 CSS 声明块；锚点必须全文件恰好一次，否则返回 null 让测试自身失败
function cssBlock(src: string, selector: string): string | null {
  const anchor = selector + ' {'
  if (occurrences(src, anchor) !== 1) return null
  const i = src.indexOf(anchor)
  const open = src.indexOf('{', i)
  const close = src.indexOf('}', open)
  return close < 0 ? null : src.slice(open, close + 1)
}

// 定位首个 CSS 声明块：用于 .res-row 这类「基础块 + 媒体块内同名块」共存的场景，取媒体块之前的基础块
function firstCssBlock(src: string, selector: string): string | null {
  const anchor = selector + ' {'
  const i = src.indexOf(anchor)
  if (i < 0) return null
  const open = src.indexOf('{', i)
  const close = src.indexOf('}', open)
  return close < 0 ? null : src.slice(open, close + 1)
}

// 定位 @media (max-width: Npx) 块，花括号配平；N 唯一，否则返回 null
function mediaBlock(src: string, px: number): string | null {
  const re = new RegExp('@media\\s*\\([^)]*max-width\\s*:\\s*' + px + 'px[^)]*\\)')
  const matches = src.match(new RegExp(re.source, 'g'))
  if (!matches || matches.length !== 1) return null
  const m = re.exec(src)
  if (!m) return null
  const open = src.indexOf('{', m.index)
  if (open < 0) return null
  let depth = 0
  for (let i = open; i < src.length; i++) {
    if (src[i] === '{') depth++
    else if (src[i] === '}') { depth--; if (depth === 0) return src.slice(open, i + 1) }
  }
  return null
}

// 紧凑化：容忍 min-width: 0 / min-width:0 / 多行声明
function compact(s: string): string { return s.replace(/\s+/g, '') }

// 稳定锚点向前找 <tag、向后找最近的 >，跨行开标签容错；不依赖行号
function tagBefore(src: string, needle: string, tag: string, nth = 0): string | null {
  let from = 0, i = -1
  for (let n = 0; n <= nth; n++) {
    i = src.indexOf(needle, from)
    if (i < 0) return null
    from = i + needle.length
  }
  const start = src.lastIndexOf(tag, i)
  if (start < 0 || i - start > 600) return null
  const gt = src.indexOf('>', i)
  return gt < 0 ? null : src.slice(start, gt + 1)
}

const indexSrc = read('pages/index/index.vue')
const tokensSrc = read('styles/tokens.css')

describe('R105 index 顶栏 768 横溢修复契约', () => {
  // ---- 缺口 3 it（业务未改时 RED）----
  it('缺口：.player-info 块含 min-width:0（允许 flex 收缩）', () => {
    const b = cssBlock(indexSrc, '.player-info')
    expect(b, '须唯一锚点定位 .player-info CSS 块').not.toBeNull()
    expect(compact(b || ''), '.player-info 须声明 min-width:0').toContain('min-width:0')
  })

  it('缺口：≤900px 媒体块内 .res-row 允许换行（flex-wrap:wrap）解除硬撑', () => {
    const mb = mediaBlock(indexSrc, 900)
    expect(mb, '须唯一锚点定位 @media (max-width: 900px) 块').not.toBeNull()
    const rb = cssBlock(mb || '', '.res-row')
    expect(rb, '媒体块内须唯一锚点定位 .res-row CSS 块').not.toBeNull()
    expect(compact(rb || ''), '媒体块 .res-row 须声明 flex-wrap:wrap').toContain('flex-wrap:wrap')
  })

  it('缺口：.topbar 块含 max-width:100% 与 box-sizing:border-box 溢出兜底', () => {
    const b = cssBlock(indexSrc, '.topbar')
    expect(b, '须唯一锚点定位 .topbar CSS 块').not.toBeNull()
    const c = compact(b || '')
    expect(c, '.topbar 须声明 max-width:100%').toContain('max-width:100%')
    expect(c, '.topbar 须声明 box-sizing:border-box').toContain('box-sizing:border-box')
  })

  // ---- 不变量 7 it（基线即绿）----
  it('不变量：.topbar 保留 display:flex 与 justify-content:space-between', () => {
    const c = compact(cssBlock(indexSrc, '.topbar') || '')
    expect(c, '.topbar 须保留 display:flex').toContain('display:flex')
    expect(c, '.topbar 须保留 justify-content:space-between').toContain('justify-content:space-between')
  })

  it('不变量：.res-row 基础块保留 white-space:nowrap（桌面/手机态不回退）', () => {
    const c = compact(firstCssBlock(indexSrc, '.res-row') || '')
    expect(c, '.res-row 基础块须保留 white-space:nowrap').toContain('white-space:nowrap')
  })

  it('不变量：R99/R102 键盘三件套保留（retry / ann-all / go(d.hint)）', () => {
    const cases = [
      { needle: '@click="retry"', tag: '<view', h: 'retry' },
      { needle: 'class="ann-all"', tag: '<text', h: 'goAnnouncements' },
      { needle: '@click="go(d.hint)"', tag: '<text', h: 'go(d.hint)' },
    ]
    for (const c of cases) {
      const b = tagBefore(indexSrc, c.needle, c.tag)
      expect(b, '须定位 ' + c.h + ' 开标签').not.toBeNull()
      const t = b || ''
      expect(t).toContain('role="button"')
      expect(t).toContain('tabindex="0"')
      expect(t).toContain('@keydown.enter="' + c.h + '"')
    }
  })

  it('不变量：R100/R101 .ann-item 三件套与 aria-expanded 不回退', () => {
    const b = tagBefore(indexSrc, 'class="ann-item"', '<view')
    expect(b, '须定位 .ann-item 开标签').not.toBeNull()
    const t = b || ''
    expect(t).toContain('role="button"')
    expect(t).toContain('tabindex="0"')
    expect(t).toContain('@keydown.enter="toggleAnn(a.id)"')
    expect(t).toContain(':aria-expanded="String(expanded.has(a.id))"')
  })

  it('不变量：四货币胶囊插值 coin/gem/energy/keys 原样保留', () => {
    for (const k of ['store.wallet.coin', 'store.wallet.gem', 'store.wallet.energy', 'store.wallet.keys ?? 0']) {
      expect(indexSrc, '须保留 ' + k).toContain(k)
    }
  })

  it('不变量：令牌三锚点不动', () => {
    expect(tokensSrc).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokensSrc).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokensSrc).toContain('--lyp-r6-version: lyp-tokens-r6')
  })

  it('不变量：业务源码零 --r105- 视觉令牌', () => {
    expect(indexSrc + tokensSrc, '不得新增 --r105- 视觉令牌').not.toContain('--r105-')
  })
})
