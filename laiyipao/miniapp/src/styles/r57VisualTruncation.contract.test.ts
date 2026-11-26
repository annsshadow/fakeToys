// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R57 契约：768 公告页横向溢出 + 768 敌人图鉴占位截断（test-first，真实读源码/审计/WCAG）。
 * 不内嵌参考实现；不碰 engine/replay/hash/score/settle/loot/tokens/后端；不新增 --r57-*；禁改 enemies.vue。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const auditDir = join(__dirname, '../../../../docs/laiyipao/audits/round56-visual-walk/data')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const readAudit = (f: string) => JSON.parse(readFileSync(join(auditDir, f), 'utf-8'))

const ANN = 'pages/announcements/announcements.vue'
const ENEMIES = 'pages/enemies/enemies.vue'
const CSS = 'styles/common.css'
const TOKENS = 'styles/tokens.css'

const ruleBody = (css: string, selector: string): string => {
  const at = css.indexOf(selector)
  if (at < 0) return ''
  const open = css.indexOf('{', at)
  const close = css.indexOf('}', open)
  return close > open ? css.slice(open, close) : ""
}
const compact = (s: string): string => s.split(' ').join('').split('\t').join('').split('\n').join('').split('\r').join('')
const rgbOf = (s: string): number[] => s.slice(s.indexOf('(') + 1, s.indexOf(')')).split(',').map((x) => Number(x.trim()))
const lum = (c: number[]): number => {
  const f = (v: number) => {
    const x = v / 255
    return x <= 0.03928 ? x / 12.92 : Math.pow((x + 0.055) / 1.055, 2.4)
  }
  return 0.2126 * f(c[0]) + 0.7152 * f(c[1]) + 0.0722 * f(c[2])
}
const contrast = (a: number[], b: number[]): number => {
  const x = lum(a), y = lum(b)
  return (Math.max(x, y) + 0.05) / (Math.min(x, y) + 0.05)
}

describe('R57 视觉截断契约（真实源码/审计守卫）', () => {
  it('候选1：.ann-title 收缩或 .ann-head 折行守卫必须存在（768 不再溢出）', () => {
    const v = readSrc(ANN)
    const title = compact(ruleBody(v, '.ann-title {'))
    const head = compact(ruleBody(v, '.ann-head {'))
    expect(title, '.ann-title 规则缺失').not.toBe('')
    expect(head, '.ann-head 规则缺失').not.toBe('')
    const shrinkGuard = title.includes('min-width:0') || head.includes('flex-wrap:wrap')
    expect(shrinkGuard, '须给 .ann-title 加 min-width:0（配合省略）或给 .ann-head 加 flex-wrap:wrap 才能容纳长标题，否则 768 溢出 27px').toBe(true)
  })

  it('候选1不变量：日期绑定与 .ann-date flex-shrink:0 保留', () => {
    const v = readSrc(ANN)
    expect(v, '日期元素 class 不得被改').toContain('class="ann-date mono"')
    expect(v, '日期值绑定不得被改').toContain('formatDate(a.published_at)')
    const date = compact(ruleBody(v, '.ann-date {'))
    expect(date, '.ann-date 规则缺失').not.toBe('')
    expect(date, '.ann-date 须保持不收缩').toContain('flex-shrink:0')
  })

  it('候选2：.search-inp 或 .ph 占位必须有 ellipsis/nowrap 或字号适配守卫（768 不再截断）', () => {
    const css = readSrc(CSS)
    const search = compact(ruleBody(css, '.search-inp {'))
    const ph = compact(ruleBody(css, '.ph {'))
    expect(search, '.search-inp 规则缺失').not.toBe('')
    expect(ph, '.ph 规则缺失').not.toBe('')
    const guard = (search.includes('text-overflow:ellipsis') && search.includes('white-space:nowrap')) || (ph.includes('text-overflow:ellipsis') && ph.includes('white-space:nowrap')) || (search.includes('font-size:') && search.includes('placeholder')) || ph.includes('font-size:')
    expect(guard, '占位截断 57px：须给 .search-inp 或 .ph 加 text-overflow:ellipsis + white-space:nowrap（或字号适配），禁止改 enemies.vue').toBe(true)
  })

  it('候选2不变量+令牌：R55 ph 惯例保留、审计基线真实、对比度非缺陷、无 --r57-', () => {
    const e = readSrc(ENEMIES)
    expect(e, '不得回退 R55：enemies.vue 必须保留 placeholder-class="ph"').toContain('placeholder-class="ph"')
    expect(e, '搜索占位文案不得被改').toContain('placeholder="搜索敌人名称 / 类别（如 Boss、飞行）"')

    const pAnn = readAudit('probe-768-announcements-announcements.json')
    expect(pAnn.innerW).toBe(768)
    expect(pAnn.hOverflow).toBe(27)
    const locAnn = readAudit('loc-768-announcements-announcements.json')
    expect(locAnn.overflow[0].right).toBe(795)
    const pEn = readAudit('probe-768-enemies-enemies.json')
    expect(pEn.trunc[0].sw - pEn.trunc[0].cw).toBe(57)
    const dom = readAudit('enemy-dom-768.json')
    const phAudit = readAudit('locph-768-enemies-enemies.json').inputs[0]
    expect(contrast(rgbOf(phAudit.color), rgbOf(phAudit.bg))).toBeGreaterThanOrEqual(4.5)
    expect(dom.ph[0].cls).toContain('ph')

    const tokens = readSrc(TOKENS)
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    for (const s of [tokens, readSrc(CSS), readSrc(ANN), e]) expect(s).not.toContain('--r57-')
  })
})