// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R101 语义可访问性补全契约（test-first RED）
 * 真实 fs.readFileSync 读 laiyipao/miniapp/src 下 .vue 与既有契约源码，禁止内嵌实现。
 * 合并 R101-A（3 处 aria-expanded 缺口）与 R101-B（me 昵称反馈语义 + r61 NODES 增补）。
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

// 按稳定锚点向前找 <tag、向后找最近的 '>'，跨行开标签容错；不依赖行号
function tagBefore(src: string, needle: string, tag: string): string | null {
  const i = src.indexOf(needle)
  if (i < 0) return null
  const start = src.lastIndexOf(tag, i)
  if (start < 0 || i - start > 600) return null
  const gt = src.indexOf('>', i)
  return gt < 0 ? null : src.slice(start, gt + 1)
}

const indexSrc = read('pages/index/index.vue')
const enemiesSrc = read('pages/enemies/enemies.vue')
const battlesSrc = read('pages/battles/battles.vue')
const meSrc = read('pages/me/me.vue')
const r61Src = read('styles/r61FormA11y.contract.test.ts')
const r100Src = read('styles/r100ClickableView.contract.test.ts')

describe('R101 语义可访问性补全契约', () => {
  it('index.vue .ann-item 开标签含 :aria-expanded 且与展开态同源', () => {
    const b = tagBefore(indexSrc, 'class="ann-item"', '<view')
    expect(b, '须能定位 .ann-item 开标签').not.toBeNull()
    expect(b || '', '须补 :aria-expanded="String(expanded.has(a.id))"')
      .toContain(':aria-expanded="String(expanded.has(a.id))"')
    expect(indexSrc, '展开态须与 .ann-toggle 同源'
      ).toContain(':class="{ open: expanded.has(a.id) }"')
  })

  it('enemies.vue .resist-toggle 开标签含 :aria-expanded 且与展开态同源', () => {
    const b = tagBefore(enemiesSrc, 'class="resist-toggle"', '<text')
    expect(b, '须能定位 .resist-toggle 开标签').not.toBeNull()
    expect(b || '', '须补 :aria-expanded="String(expandedEnemy === en.id)"')
      .toContain(':aria-expanded="String(expandedEnemy === en.id)"')
    expect(enemiesSrc, '展开态须与抗性图文案同源').toContain('expandedEnemy === en.id')
  })

  it('battles.vue .battle-row 开标签含 :aria-expanded 且与展开态同源', () => {
    const b = tagBefore(battlesSrc, 'class="battle-row tap"', '<view')
    expect(b, '须能定位 .battle-row 开标签').not.toBeNull()
    expect(b || '', '须补 :aria-expanded="String(expandedId === it.id)"')
      .toContain(':aria-expanded="String(expandedId === it.id)"')
    expect(battlesSrc, '展开态须与详情 v-if 同源').toContain('v-if="expandedId === it.id"')
  })

  it('me.vue nicknameMsg 补 role=status + aria-live=polite（与 passwordMsg 同构）', () => {
    const b = tagBefore(meSrc, '{{ nicknameMsg }}', '<text')
    expect(b, '须能定位 nicknameMsg text 开标签').not.toBeNull()
    expect(b || '', '须补 role="status"').toContain('role="status"')
    expect(b || '', '须补 aria-live="polite"').toContain('aria-live="polite"')
  })

  it('r61 NODES 增补 nicknameMsg 第 7 处语义守卫', () => {
    expect(r61Src, 'NODES 须含 nicknameMsg')
      .toContain("['pages/me/me.vue', '{{ nicknameMsg }}', 'state', 'me nicknameMsg']")
  })

  it('不变量：三处展开控件键盘三件套与原 @click 入口保留', () => {
    const CASES = [
      { src: indexSrc, tag: '<view', anchor: 'class="ann-item"', handler: 'toggleAnn(a.id)', name: 'index .ann-item' },
      { src: enemiesSrc, tag: '<text', anchor: 'class="resist-toggle"', handler: 'toggleEnemy(en.id)', name: 'enemies .resist-toggle' },
      { src: battlesSrc, tag: '<view', anchor: 'class="battle-row tap"', handler: 'toggleDetail(it)', name: 'battles .battle-row' },
    ]
    for (const c of CASES) {
      const b = tagBefore(c.src, c.anchor, c.tag)
      expect(b, c.name + ' 须能定位 @click 开标签').not.toBeNull()
      expect(b || '', c.name + ' 须保留 role="button"').toContain('role="button"')
      expect(b || '', c.name + ' 须保留 tabindex="0"').toContain('tabindex="0"')
      expect(b || '', c.name + ' 须保留 @keydown.enter 同名 handler')
        .toContain('@keydown.enter="' + c.handler + '"')
      expect(b || '', c.name + ' 须保留原 @click').toContain('@click="' + c.handler + '"')
    }
    expect(meSrc, 'me 保存入口须保留').toContain('@click="submitNickname"')
  })

  it('不变量：R61/R100 契约本体断言保留', () => {
    expect(r61Src, 'r61 state 断言须保留').toContain('role="status"')
    expect(r61Src, 'r61 aria-live 断言须保留').toContain('aria-live="polite"')
    expect(r100Src, 'R100 @keydown 断言须保留').toContain('@keydown.enter')
    expect(r100Src, 'R100 tabindex 断言须保留').toContain('tabindex="0"')
  })

  it('不变量：令牌三锚点不动且业务零 --r101-', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toContain('--r101-')
    for (const [name, src] of [
      ['index', indexSrc], ['enemies', enemiesSrc], ['battles', battlesSrc], ['me', meSrc],
    ] as [string, string][]) expect(src, name + ' 零 --r101-').not.toContain('--r101-')
  })

  it('不变量：排除项不被误改（导航/选择态/dialog/link 详情/双入口）', () => {
    const mastery = read('pages/mastery/mastery.vue')
    const bag = read('pages/bag/bag.vue')
    const battle = read('pages/battle/battle.vue')
    expect(mastery, 'mastery 选择态不补 aria-expanded').not.toContain('aria-expanded')
    expect(bag, 'bag 全量配方导航不补 aria-expanded').not.toContain('aria-expanded')
    expect(battle, 'battle 帮助图例 dialog 不补 aria-expanded').not.toContain('aria-expanded')
    const meDetail = tagBefore(meSrc, '@click="toggleBattleDetail(b)"', '<text')
    expect(meDetail, 'me 战报详情 link 须能定位').not.toBeNull()
    expect(meDetail || '', 'me link 详情不补 aria-expanded').not.toContain('aria-expanded')
    const rxnName = tagBefore(enemiesSrc, 'class="rxn-name', '<text')
    expect(rxnName, '.rxn-name 须能定位').not.toBeNull()
    expect(rxnName || '', '.rxn-name 防双入口').not.toContain('role="button"')
    expect(rxnName || '', '.rxn-name 防双 Tab 停靠').not.toContain('@keydown.enter')
  })
})
