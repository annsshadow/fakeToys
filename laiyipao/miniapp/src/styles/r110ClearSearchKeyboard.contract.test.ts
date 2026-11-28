// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R110 搜索空态「清除搜索」键盘可达契约（test-first RED）。
 * 真实 fs.readFileSync 读 6 页 vue + announcements 对照 + common.css + tokens.css，禁止内嵌实现副本。
 * 缺口：6 处 <text class="clear-search" @click="keyword = ''"> 裸元素缺
 * role="button" / tabindex="0" / @keydown.enter（WCAG 2.1.1），键盘 Tab 不可达。
 * 强对照：announcements.vue 同类 .link 已带完整三件套（R99_A 所修）。
 * 定位按 class="clear-search" + @click handler 双锚点，不依赖行号。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const read = (p: string): string => readFileSync(join(srcRoot, p), 'utf8')
const NL = String.fromCharCode(10)
const HANDLER = "keyword = ''"

type Case = { rel: string }
const SIX: Case[] = [
  { rel: 'pages/enemies/enemies.vue' },
  { rel: 'pages/equipment/equipment.vue' },
  { rel: 'pages/gems/gems.vue' },
  { rel: 'pages/reactions/reactions.vue' },
  { rel: 'pages/skills/skills.vue' },
  { rel: 'pages/skins/skins.vue' },
]

function clearSearchLine(c: Case): string {
  const lines = read(c.rel).split(NL)
  const hits = lines.filter((l) =>
    l.includes('class="clear-search"') && l.includes('@click="' + HANDLER + '"'))
  expect(hits.length, c.rel + ' 应恰有一条 clear-search 开标签').toBe(1)
  return hits[0]
}

describe('R110 搜索空态清除搜索键盘可达契约', () => {
  // ============ 6 缺口 it：三件套齐全（RED 应全失败）============
  for (const c of SIX) {
    it('缺口 ' + c.rel + ' clear-search 三件套齐全', () => {
      const line = clearSearchLine(c)
      expect(line, c.rel + ' 须 role=button').toContain('role="button"')
      expect(line, c.rel + ' 须 tabindex=0').toContain('tabindex="0"')
      expect(line, c.rel + ' 须 @keydown.enter 复用同 handler').toContain('@keydown.enter="' + HANDLER + '"')
    })
  }

  // ============ 不变量 it（RED 应全通过）============
  it('不变量1：announcements.vue 已修 .link 清除搜索三件套仍在（R99_A）', () => {
    const src = read('pages/announcements/announcements.vue')
    const hits = src.split(NL).filter((l) =>
      l.includes('class="link"') && l.includes("@click=\"annKeyword = ''\"") && l.includes('清除搜索'))
    expect(hits.length, 'announcements 清除搜索 link 应恰有一条').toBe(1)
    const line = hits[0]
    expect(line).toContain('role="button"')
    expect(line).toContain('tabindex="0"')
    expect(line).toContain("@keydown.enter=\"annKeyword = ''\"")
  })

  it('不变量2：r72SearchEmpty 空态结构仍在（关键词回显 + 清除搜索 CTA）', () => {
    const pages = [
      { rel: 'pages/announcements/announcements.vue', token: '{{ annKeyword }}' },
      { rel: 'pages/enemies/enemies.vue', token: '{{ keyword }}' },
      { rel: 'pages/equipment/equipment.vue', token: '{{ keyword }}' },
      { rel: 'pages/gems/gems.vue', token: '{{ keyword }}' },
      { rel: 'pages/reactions/reactions.vue', token: '{{ keyword }}' },
      { rel: 'pages/skills/skills.vue', token: '{{ keyword }}' },
      { rel: 'pages/skins/skins.vue', token: '{{ keyword }}' },
    ]
    for (const p of pages) {
      const src = read(p.rel)
      expect(src, p.rel + ' 必须保留关键词变量').toContain(p.token)
      expect(src, p.rel + ' 必须保留清除搜索 CTA').toContain('清除搜索')
      expect(src, p.rel + ' 必须保留空态回显').toContain('没有匹配')
    }
  })

  it('不变量3：令牌三锚点未漂移 + 7 页零 --r110-', () => {
    const t = read('styles/tokens.css')
    expect(t).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(t).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(t).toContain('--lyp-r6-version: lyp-tokens-r6')
    const all = ['pages/announcements/announcements.vue'].concat(SIX.map((c) => c.rel))
    for (const rel of all) expect(read(rel), rel + ' 零 --r110-').not.toContain('--r110-')
  })

  it('不变量4：R99/R100/R102/R104/R109 既有三件套抽查无回退', () => {
    const cases = [
      { rel: 'pages/announcements/announcements.vue', anchor: '@click="annKeyword = ' + String.fromCharCode(39) + String.fromCharCode(39) + '" tabindex="0"' },
      { rel: 'pages/index/index.vue', anchor: '@click="retry" @keydown.enter="retry"' },
      { rel: 'pages/me/me.vue', anchor: '@click="goBack" @keydown.enter="goBack"' },
      { rel: 'pages/enemies/enemies.vue', anchor: '@keydown.enter="toggleEnemy(en.id)"' },
      { rel: 'pages/announcements/announcements.vue', anchor: 'aria-busy="true"' },
      { rel: 'pages/bag/bag.vue', anchor: '@click="r12Go()" role="button"' },
      { rel: 'pages/battle/battle.vue', anchor: '@click="closeReactionTimeline" @keydown.enter="closeReactionTimeline"' },
      { rel: 'pages/battle/battle.vue', anchor: '@click.stop="openReactionTimeline" @keydown.enter="openReactionTimeline"' },
    ]
    for (const c of cases) expect(read(c.rel), c.rel + ' ' + c.anchor).toContain(c.anchor)
  })

  it('不变量5：6 处 @click handler 未变 + 双锚点唯一定位', () => {
    for (const c of SIX) {
      const line = clearSearchLine(c)
      expect(line, c.rel + ' @click 原样保留').toContain('@click="' + HANDLER + '"')
      expect(line, c.rel + ' class 原样保留').toContain('class="clear-search"')
    }
  })

  it('不变量6：common.css .clear-search 视觉样式守卫未动', () => {
    const css = read('styles/common.css')
    const k = css.indexOf('.clear-search')
    expect(k, 'common.css 须含 .clear-search 视觉定义').toBeGreaterThanOrEqual(0)
    const open = css.indexOf('{', k)
    const close = css.indexOf('}', open)
    const block = css.slice(open, close + 1)
    expect(block, '.clear-search 须保留 fire 色描边').toContain('var(--fire)')
    expect(block, '.clear-search 须保留 pill 圆角').toContain('border-radius')
  })
})
