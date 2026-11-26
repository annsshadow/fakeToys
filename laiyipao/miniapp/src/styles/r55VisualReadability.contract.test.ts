// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R55 契约：768 反应图鉴溢出 + 背包占位对比度（test-first，真实读源码/审计/WCAG）。
 * 不内嵌参考实现；不碰 engine/replay/hash/score/settle/loot/tokens/后端；不新增 --r55-*。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const auditDir = join(__dirname, '../../../../docs/laiyipao/audits/round54-visual-walk/data')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')
const readAudit = (f: string) => JSON.parse(readFileSync(join(auditDir, f), 'utf-8'))

const BAG = 'pages/bag/bag.vue'
const REACTIONS = 'pages/reactions/reactions.vue'
const CSS = 'styles/common.css'
const TOKENS = 'styles/tokens.css'

const presetInputBlock = (src: string): string => {
  const at = src.indexOf('placeholder="预设名（最多 20 字）"')
  if (at < 0) return ''
  const open = src.lastIndexOf('<input', at)
  const close = src.indexOf('/>', at)
  return close > open ? src.slice(open, close + 2) : ''
}
const ruleBody = (css: string, selector: string): string => {
  const at = css.indexOf(selector)
  if (at < 0) return ''
  const open = css.indexOf('{', at)
  const close = css.indexOf('}', open)
  return close > open ? css.slice(open, close) : ''
}
const compact = (s: string): string => s.split(' ').join('').split('\t').join('').split('\n').join('')
const rgbOf = (s: string): number[] => {
  const inner = s.slice(s.indexOf('(') + 1, s.indexOf(')'))
  return inner.split(',').map((x) => Number(x.trim()))
}
const hexOf = (s: string): number[] => {
  const h = s.replace('#', '')
  return [0, 2, 4].map((i) => parseInt(h.slice(i, i + 2), 16))
}
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

describe('R55 视觉可读性契约（真实源码守卫）', () => {
  it('候选2：bag.vue 预设名 input 必须显式 placeholder-class="ph"', () => {
    const src = readSrc(BAG)
    const block = presetInputBlock(src)
    expect(block, '未定位到预设名 input').not.toBe('')
    expect(block, '预设名 input 缺少 placeholder-class="ph"（对比度 4.03 < AA 4.5）').toContain('placeholder-class="ph"')
    expect(block, '占位文案不得被改').toContain('placeholder="预设名（最多 20 字）"')
  })

  it('权威口径：.ph 走 var(--dim)，真实 WCAG 计算对同背景 ≥4.5，审计确认 gap=4.03', () => {
    const ph = ruleBody(readSrc(CSS), '.ph {')
    expect(ph, '.ph 规则缺失').not.toBe('')
    expect(ph, '.ph 必须消费 --dim 令牌').toContain('var(--dim)')
    const tokens = readSrc(TOKENS)
    const dimHex = /--dim: (#[0-9a-fA-F]{6})/.exec(tokens)
    expect(dimHex, 'tokens.css 缺 --dim 色值').toBeTruthy()
    const dim = hexOf(dimHex![1])
    const gems = readAudit('ph-gems.json').inputs[0]
    const bag = readAudit('ph-bag.json').inputs[0]
    expect(rgbOf(gems.color), 'ph-gems 占色应为 --dim').toEqual(dim)
    expect(contrast(rgbOf(gems.color), rgbOf(gems.bg))).toBeGreaterThanOrEqual(4.5)
    expect(contrast(rgbOf(bag.color), rgbOf(bag.bg))).toBeLessThan(4.5)
  })

  it('候选1：common.css 必须给出 R14 空态不溢出守卫（收缩/换行/CTA 单行/行容器可折行）', () => {
    const css = readSrc(CSS)
    const advice = compact(ruleBody(css, '.r14-advice {'))
    const empty = compact(ruleBody(css, '.r14-advice-empty {'))
    const cta = compact(ruleBody(css, '.r14-advice-empty-cta {'))
    const gridRow = compact(ruleBody(css, '.r3-data-grid .list-row,'))
    expect(advice, '.r14-advice 规则缺失').not.toBe('')
    expect(empty, '.r14-advice-empty 规则缺失').not.toBe('')
    expect(cta, '.r14-advice-empty-cta 规则缺失').not.toBe('')
    expect(gridRow, '.r3-data-grid .list-row 规则缺失').not.toBe('')
    expect(advice, '.r14-advice 需 min-width:0 才能在 flex 行内收缩').toContain('min-width:0')
    expect(advice, '.r14-advice 需 max-width:100% 约束在卡片内').toContain('max-width:100%')
    expect(empty, '.r14-advice-empty 需任意处换行避免长消息撑宽父容器').toContain('overflow-wrap:anywhere')
    expect(cta, 'CTA 需 white-space:nowrap 防止竖排 h=633').toContain('white-space:nowrap')
    expect(cta, 'CTA 需 max-width:100% 留在父容器内').toContain('max-width:100%')
    expect(gridRow, '.r3-data-grid .list-row 需可折行/改列容纳第五个子项').toContain('flex-wrap:wrap')
  })

  it('不变量：R14 行为锚点、审计基线与令牌三锚点保留，且无 --r55-*', () => {
    const v = readSrc(REACTIONS)
    expect(v, '空态 CTA 的 @click 不得被改').toContain('r14GoEmpty(r14AdviceOf(r))')
    expect(v, '空态数据源标记不得被改').toContain('data-r14-source="reactions"')
    expect(v, '空态文案绑定不得被改').toContain('empty?.headline')
    expect(v, '空态 CTA 文案绑定不得被改').toContain('empty?.cta')
    const probe = readAudit('probe-768-reactions-reactions.json')
    expect(probe.hOverflow).toBe(77)
    expect(probe.innerW).toBe(768)
    const loc = readAudit('loc2-768-reactions.json')
    expect(loc.overflow[0].right).toBe(845)
    const tokens = readSrc(TOKENS)
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toContain('--r55-')
    expect(readSrc(CSS)).not.toContain('--r55-')
    expect(v).not.toContain('--r55-')
    expect(readSrc(BAG)).not.toContain('--r55-')
  })
})