// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R96 六页空态 CTA 键盘可达补漏契约（test-first RED）
 * 真实 fs.readFileSync 读 6 页源码，禁止内嵌实现副本。
 * 缺口：6 处 <view class="r3-empty-action" @click="X()">文案</view> 无 role/tabindex/@keydown.enter，
 * a11y 树不暴露 button、Tab 不命中、Enter 无效（WCAG 2.1.1）；R60 契约只扫已带 role 的元素，属结构盲区。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync, readdirSync, statSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join, relative } from 'node:path'

const dir = dirname(fileURLToPath(import.meta.url))
const srcRoot = join(dir, '..')
const NL = String.fromCharCode(10)

function read(rel: string): string {
  return readFileSync(join(srcRoot, rel), 'utf8')
}

const SIX = [
  { rel: 'pages/bag/bag.vue', fn: 'r4GoBag', click: 'r4GoBag()', label: '去关卡页 →' },
  { rel: 'pages/battles/battles.vue', fn: 'r4GoBattle', click: 'r4GoBattle()', label: '去关卡页挑一关开打 →' },
  { rel: 'pages/enemies/enemies.vue', fn: 'loadPage', click: 'loadPage()', label: '重新加载配置' },
  { rel: 'pages/me/me.vue', fn: 'r4GoMe', click: 'r4GoMe()', label: '去专精页 →' },
  { rel: 'pages/rank/rank.vue', fn: 'r4GoRank', click: 'r4GoRank()', label: '去冲榜关卡 →' },
  { rel: 'pages/skins/skins.vue', fn: 'loadPage', click: 'loadPage()', label: '重新加载配置' },
] as const

type Cta = (typeof SIX)[number]

// 三条件锁定唯一模板行：class + @click 原函数 + 原文案；定位本身即不变量守卫。
function ctaLine(s: Cta): string {
  const lines = read(s.rel).split(NL)
  const hits = lines.filter((l) =>
    l.includes('class="r3-empty-action"') &&
    l.includes('@click="' + s.click + '"') &&
    l.includes('>' + s.label + '</'))
  expect(hits.length, s.rel + ' 应恰有一条 r3-empty-action 空态 CTA 模板行').toBe(1)
  return hits[0]
}

function walk(root: string): string[] {
  const out: string[] = []
  for (const name of readdirSync(root)) {
    const p = join(root, name)
    if (statSync(p).isDirectory()) out.push(...walk(p))
    else if (/.(vue|ts|css)$/.test(name) && !name.includes('.test.')) out.push(p)
  }
  return out
}

describe('R96 六页空态 CTA 键盘可达契约', () => {
  it('it1 六处空态 CTA 均含 role="button"', () => {
    for (const s of SIX) {
      expect(ctaLine(s), s.rel + ' 须声明 role=button 语义').toContain('role="button"')
    }
  })

  it('it2 六处空态 CTA 均含 tabindex="0"', () => {
    for (const s of SIX) {
      expect(ctaLine(s), s.rel + ' 须进入 Tab 序列（WCAG 2.1.1）').toContain('tabindex="0"')
    }
  })

  it('it3 六处空态 CTA 均含 @keydown.enter="X()"', () => {
    for (const s of SIX) {
      expect(ctaLine(s), s.rel + ' 须可被 Enter 键盘激活').toContain('@keydown.enter="' + s.click + '"')
    }
  })

  it('it4 属性顺序 role -> @click -> tabindex -> @keydown.enter', () => {
    for (const s of SIX) {
      const line = ctaLine(s)
      const iRole = line.indexOf('role="button"')
      const iClick = line.indexOf('@click="')
      const iTab = line.indexOf('tabindex="0"')
      const iKey = line.indexOf('@keydown.enter="')
      expect(iRole, s.rel + ' role 位').toBeGreaterThan(-1)
      expect(iClick, s.rel + ' click 位').toBeGreaterThan(-1)
      expect(iTab, s.rel + ' tabindex 位').toBeGreaterThan(-1)
      expect(iKey, s.rel + ' keydown 位').toBeGreaterThan(-1)
      expect(iRole, s.rel + ' role 须在 click 前').toBeLessThan(iClick)
      expect(iClick, s.rel + ' click 须在 tabindex 前').toBeLessThan(iTab)
      expect(iTab, s.rel + ' tabindex 须在 keydown 前').toBeLessThan(iKey)
    }
  })

  it('it5 六处焦点函数声明保留（不变量）', () => {
    for (const s of SIX) {
      expect(read(s.rel), s.rel + ' 须保留 ' + s.fn + ' 声明').toContain('function ' + s.fn + '(')
    }
  })

  it('it6 空态四段槽位顺序不变（不变量）', () => {
    for (const s of SIX) {
      const src = read(s.rel)
      const iSlot = src.indexOf('r3-empty-slot')
      const iTitle = src.indexOf('r3-empty-title')
      const iReason = src.indexOf('r3-empty-reason')
      const iAction = src.indexOf('r3-empty-action')
      expect(iSlot, s.rel + ' slot').toBeGreaterThan(-1)
      expect(iTitle, s.rel + ' title').toBeGreaterThan(-1)
      expect(iReason, s.rel + ' reason').toBeGreaterThan(-1)
      expect(iAction, s.rel + ' action').toBeGreaterThan(-1)
      expect(iSlot, s.rel + ' 槽位顺序 slot<title').toBeLessThan(iTitle)
      expect(iTitle, s.rel + ' 槽位顺序 title<reason').toBeLessThan(iReason)
      expect(iReason, s.rel + ' 槽位顺序 reason<action').toBeLessThan(iAction)
    }
  })

  it('it7 令牌三锚点不变且业务源码零 --r96-（不变量）', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens, 'v1 锚点').toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens, 'r3 锚点').toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens, 'r6 锚点').toContain('--lyp-r6-version: lyp-tokens-r6')
    const bad: string[] = []
    for (const f of walk(srcRoot)) {
      if (readFileSync(f, 'utf8').includes('--r96-')) bad.push(relative(srcRoot, f))
    }
    expect(bad, '业务源码不得出现 --r96-* 令牌: ' + bad.join(', ')).toEqual([])
  })

  it('it8 R60 既有键盘 CTA 范式不回退（抽查≥2处，不变量）', () => {
    const spots = ['pages/announcements/announcements.vue', 'pages/equipment/equipment.vue', 'pages/skills/skills.vue']
    let ok = 0
    for (const rel of spots) {
      const hit = read(rel).split(NL).some((l) =>
        l.includes('role="button"') && l.includes('tabindex="0"') && l.includes('@keydown.enter'))
      if (hit) ok++
    }
    expect(ok, 'R60 role=button + tabindex=0 + @keydown.enter 范式须至少 2 处仍在').toBeGreaterThanOrEqual(2)
  })
})
