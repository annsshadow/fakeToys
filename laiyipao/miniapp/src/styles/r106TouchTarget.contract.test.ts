// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R106 次级可点击控件触摸目标契约（test-first RED）。
 * 真实 fs.readFileSync 读 index.vue / common.css / tokens.css，禁止内嵌实现副本。
 * 以唯一 class 锚点定位 CSS 块（tagBefore/cssBlock 风格，不依赖行号）。
 * 缺口 5 it：.mg-row / .elem-chip / .ann-all / .dim-hint 各含 min-height:88rpx；
 *           且 .dim-hint 原 min-height:auto 必须被替换（非仅追加）。
 * 不变量 6 it：四类模板键盘三件套齐备且 click/keydown 同 handler；common.css 两层
 *              :focus-visible 兜底；令牌三锚点；业务零 --r106-；既有声明 surgical 保留 ×2。
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

// 紧凑化：容忍 min-height: 88rpx / min-height:88rpx / 多行声明
function compact(s: string): string { return s.replace(/\s+/g, '') }

// 稳定锚点向前找 <tag、向后找最近的 >，跨行开标签容错；不依赖行号
function tagBefore(src: string, needle: string, tag: string): string | null {
  if (occurrences(src, needle) !== 1) return null
  const i = src.indexOf(needle)
  const start = src.lastIndexOf(tag, i)
  if (start < 0 || i - start > 600) return null
  const gt = src.indexOf('>', i)
  return gt < 0 ? null : src.slice(start, gt + 1)
}

function attr(tag: string, name: string): string | null {
  const m = new RegExp(name + '="([^"]*)"').exec(tag)
  return m ? m[1] : null
}

const indexSrc = read('pages/index/index.vue')
const commonSrc = read('styles/common.css')
const tokensSrc = read('styles/tokens.css')

// 四类目标：CSS 选择器 + 模板唯一 class 锚点 + 期望 @click 处理器
const TARGETS = [
  { name: '.mg-row', selector: '.mg-row', cls: 'class="mg-row tap"', tag: '<view', handler: 'm.hint && go(m.hint)' },
  { name: '.elem-chip', selector: '.elem-chip', cls: 'class="elem-chip tap"', tag: '<view', handler: "go('/pages/reactions/reactions')" },
  { name: '.ann-all', selector: '.ann-all', cls: 'class="ann-all"', tag: '<text', handler: 'goAnnouncements' },
  { name: '.dim-hint', selector: '.dim-hint', cls: 'class="dim-hint link"', tag: '<text', handler: 'go(d.hint)' },
]

describe('R106 次级可点击控件触摸目标契约', () => {
  // ---- 缺口 5 it（业务未改时 RED）----
  for (const t of TARGETS) {
    it('缺口：' + t.name + ' CSS 块含 min-height:88rpx', () => {
      const b = cssBlock(indexSrc, t.selector)
      expect(b, '须唯一锚点定位 ' + t.name + ' CSS 块').not.toBeNull()
      expect(compact(b || ''), t.name + ' 须声明 min-height:88rpx').toContain('min-height:88rpx')
    })
  }
  it('缺口：.dim-hint 原 min-height:auto 须被替换（非仅追加导致被覆盖）', () => {
    const c = compact(cssBlock(indexSrc, '.dim-hint') || '')
    expect(c, '.dim-hint 块须存在').not.toBe('')
    expect(c, '.dim-hint 不得保留 min-height:auto').not.toContain('min-height:auto')
  })

  // ---- 不变量 6 it（基线即绿）----
  it('不变量：四类模板键盘三件套齐备且 @click/@keydown.enter 同 handler', () => {
    for (const t of TARGETS) {
      const tag = tagBefore(indexSrc, t.cls, t.tag)
      expect(tag, '须唯一锚点定位 ' + t.name + ' 开标签').not.toBeNull()
      const s = tag || ''
      expect(s, t.name + ' 须保留 role="button"').toContain('role="button"')
      expect(s, t.name + ' 须保留 tabindex="0"').toContain('tabindex="0"')
      const click = attr(s, '@click')
      const key = attr(s, '@keydown.enter')
      expect(click, t.name + ' 须保留 @click').not.toBeNull()
      expect(key, t.name + ' 须保留 @keydown.enter').not.toBeNull()
      expect(key, t.name + ' 的 @keydown.enter 须与 @click 同 handler').toBe(click)
    }
  })

  it('不变量：common.css 两层 :focus-visible 焦点环兜底保留（outline 3rpx + offset 2rpx）', () => {
    expect(occurrences(commonSrc, ':where(a, button, input, textarea, select, [tabindex]):focus-visible'),
      '须保留 :where(...) :focus-visible 兜底').toBe(1)
    expect(occurrences(commonSrc, '\n:focus-visible {'), '须保留通用 :focus-visible 兜底').toBe(1)
    for (const m of [':where(a, button, input, textarea, select, [tabindex]):focus-visible', '\n:focus-visible {']) {
      const i = commonSrc.indexOf(m)
      const open = commonSrc.indexOf('{', i)
      const close = commonSrc.indexOf('}', open)
      const c = compact(commonSrc.slice(open, close + 1))
      expect(c, '焦点环须为 outline:3rpx solid var(--accent)').toContain('outline:3rpxsolidvar(--accent)')
      expect(c, '焦点环须带 outline-offset:2rpx').toContain('outline-offset:2rpx')
    }
  })

  it('不变量：令牌三锚点原样（tokens.css:14-16）', () => {
    expect(tokensSrc).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokensSrc).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokensSrc).toContain('--lyp-r6-version: lyp-tokens-r6')
  })

  it('不变量：业务 index.vue 零 --r106-', () => {
    expect(occurrences(indexSrc, '--r106-'), 'index.vue 不得出现 --r106- 令牌').toBe(0)
  })

  it('不变量：surgical —— .mg-row/.elem-chip 保留 display:flex 与既有 padding/gap', () => {
    const mg = compact(cssBlock(indexSrc, '.mg-row') || '')
    expect(mg, '.mg-row 须保留 display:flex').toContain('display:flex')
    expect(mg, '.mg-row 须保留 padding:10rpx0').toContain('padding:10rpx0')
    const ec = compact(cssBlock(indexSrc, '.elem-chip') || '')
    expect(ec, '.elem-chip 须保留 display:flex').toContain('display:flex')
    expect(ec, '.elem-chip 须保留 gap:8rpx').toContain('gap:8rpx')
  })

  it('不变量：surgical —— .ann-all/.dim-hint 保留 font-size:22rpx 与既有 padding', () => {
    const aa = compact(cssBlock(indexSrc, '.ann-all') || '')
    expect(aa, '.ann-all 须保留 font-size:22rpx').toContain('font-size:22rpx')
    expect(aa, '.ann-all 须保留 padding:4rpx04rpx16rpx').toContain('padding:4rpx04rpx16rpx')
    const dh = compact(cssBlock(indexSrc, '.dim-hint') || '')
    expect(dh, '.dim-hint 须保留 font-size:22rpx').toContain('font-size:22rpx')
    expect(dh, '.dim-hint 须保留 padding:0').toContain('padding:0')
  })
})
