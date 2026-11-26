// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R568 敌人图鉴页 enemies.vue 5 处裸 @click 文本伪按钮键盘可达契约（源码扫描）。
 * 修复形态：就地追加 role=button + tabindex=0 + 语义化 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * 方案 A 全局增强器 Fail Loud 排除（越界他页 + 动态名无法可靠派生）；先例 R544/R546/R562/R564/R566。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

const SRC = fileURLToPath(new URL('./enemies/enemies.vue', import.meta.url))
const MAIN = fileURLToPath(new URL('../main.ts', import.meta.url))
const BATTLES = fileURLToPath(new URL('./battles/battles.vue', import.meta.url))
const RANK = fileURLToPath(new URL('./rank/rank.vue', import.meta.url))
const STAGE = fileURLToPath(new URL('./stage/stage.vue', import.meta.url))
const source = readFileSync(SRC, 'utf8')
const main = readFileSync(MAIN, 'utf8')
const battles = readFileSync(BATTLES, 'utf8')
const rank = readFileSync(RANK, 'utf8')
const stage = readFileSync(STAGE, 'utf8')

// 从起始锚点行向下扫到开标签结束（trimEnd 以 > 收尾），适配多行开标签
function blockFrom(anchor: string): string {
  const i = source.indexOf(anchor)
  if (i < 0) return ''
  const out: string[] = []
  for (const l of source.slice(i).split('\n')) {
    out.push(l)
    if (l.trimEnd().endsWith('>')) break
  }
  return out.join('\n')
}
// 取包含锚点的单行（单行开标签）
function lineWith(anchor: string): string {
  return source.split('\n').find((l) => l.includes(anchor)) || ''
}
// 5 类控件统一 a11y 断言
function hasA11y(b: string, labelCheck: (b: string) => boolean): void {
  expect(b).toMatch(/role="button"/)
  expect(b).toMatch(/tabindex="0"/)
  expect(labelCheck(b)).toBe(true)
}

describe('R568 enemies.vue 5 处伪按钮键盘可达（核心）', () => {
  it('1 类别 chip 补 role/tabindex/aria-label=c.label/keydown 复用 categoryFilter', () => {
    const b = blockFrom('v-for="c in CATEGORY_FILTERS"')
    expect(b).toContain('v-for="c in CATEGORY_FILTERS"')
    expect(b).toContain('class="chip tap"')
    hasA11y(b, (s) => s.includes(':aria-label="c.label"'))
    expect(b).toContain('@click="categoryFilter = c.key"')
    expect(b).toContain('@keydown.enter="categoryFilter = c.key"')
    expect(b).toContain('@keydown.space.prevent="categoryFilter = c.key"')
  })

  it('2 元素 chip 补 role/tabindex/aria-label=ELEMENT_NAME[el]/keydown 复用 counterElement', () => {
    const b = blockFrom('v-for="el in ELEMENT_ORDER"')
    expect(b).toContain('v-for="el in ELEMENT_ORDER"')
    expect(b).toContain(':style="counterElement === el')
    hasA11y(b, (s) => s.includes(':aria-label="ELEMENT_NAME[el]"'))
    expect(b).toContain('@click="counterElement = el"')
    expect(b).toContain('@keydown.enter="counterElement = el"')
    expect(b).toContain('@keydown.space.prevent="counterElement = el"')
  })

  it('3 敌人名 rxn-name 补 role/tabindex/动态 aria-label/keydown 复用 toggleEnemy', () => {
    const b = lineWith('class="rxn-name tap"')
    expect(b).toContain('toggleEnemy(en.id)')
    hasA11y(b, (s) => s.includes(":aria-label=\"'查看 ' + en.name + ' 抗性'\""))
    expect(b).toContain('@click="toggleEnemy(en.id)"')
    expect(b).toContain('@keydown.enter="toggleEnemy(en.id)"')
    expect(b).toContain('@keydown.space.prevent="toggleEnemy(en.id)"')
  })

  it('4 抗性图 resist-toggle 补 role/tabindex/动态 aria-label/keydown 复用 toggleEnemy', () => {
    const b = lineWith('class="resist-toggle"')
    hasA11y(b, (s) => s.includes(':aria-label="(expandedEnemy === en.id'))
    expect(b).toContain('@click="toggleEnemy(en.id)"')
    expect(b).toContain('@keydown.enter="toggleEnemy(en.id)"')
    expect(b).toContain('@keydown.space.prevent="toggleEnemy(en.id)"')
    // 文案在开标签下一行（多行文本），单独取块断言
    const i = source.indexOf('class="resist-toggle"')
    const block = source.slice(i, i + 600)
    expect(block).toContain("'收起抗性图 ▲'")
    expect(block).toContain("'抗性图 ▼'")
  })

  it('5 清除搜索 clear-search 补 role/tabindex/aria-label/keydown 复用 keyword 清空', () => {
    const b = lineWith('class="clear-search"')
    hasA11y(b, (s) => s.includes('aria-label="清除搜索"'))
    expect(b).toContain('@click="keyword = \'\'"')
    expect(b).toContain('@keydown.enter="keyword = \'\'"')
    expect(b).toContain('@keydown.space.prevent="keyword = \'\'"')
    expect(b).toContain('清除搜索</text>')
    expect((source.match(/@click=/g) || []).length).toBe(5)
  })
})

describe('R568 守卫（业务/数据/样式/对照零改）', () => {
  it('6 业务状态逻辑零改：categoryFilter/counterElement/keyword/expandedEnemy 与函数/computed', () => {
    expect(source).toContain("const keyword = ref('')")
    expect(source).toContain("const categoryFilter = ref('all')")
    expect(source).toContain('const filteredEnemies = computed(')
    expect(source).toContain('function categoryLabel(c: string): string {')
    expect(source).toContain('const counterElement = ref<Element>(ELEMENT_ORDER[0]!)')
    expect(source).toContain('const counterRanking = computed(')
    expect(source).toContain('const hpDistribution = computed(')
    expect(source).toContain('const expandedEnemy = ref<number | null>(null)')
    expect(source).toContain('function toggleEnemy(id: number) {')
    expect(source).toContain('expandedEnemy.value = expandedEnemy.value === id ? null : id')
  })

  it('7 数据源/样式零改，未引入全局增强器', () => {
    expect(source).toContain("import { useGameStore } from '@/store/game'")
    expect(source).toContain("import { ELEMENT_NAME, ELEMENT_ORDER, ELEMENT_COLOR, type Element } from '@/game/elements'")
    expect(source).toContain('.resist-toggle {')
    expect(source).toContain('.res-up { color: var(--bad); }')
    expect(source).toContain('.res-down { color: var(--ok); }')
    expect(source).not.toMatch(/<button[\s>]/)
    expect(main).toContain('createSSRApp(App)')
    expect(main).not.toMatch(/MutationObserver/)
    expect(main).not.toMatch(/install[A-Z][A-Za-z]*A11y/)
  })

  it('8 正面对照 battles/rank/stage 已修控件与 enemies svg aria-label 零改', () => {
    expect(battles).toContain('class="filter-chip"')
    expect(battles).toContain(':aria-label="f.label + (filter === f.key')
    expect(rank).toContain('v-for="t in tabs"')
    expect(rank).toContain(':aria-pressed="curType === t.key"')
    expect(stage).toContain('v-for="ch in chapters"')
    expect(stage).toContain('@keydown.enter="switchChapter(ch.id)"')
    expect(source).toContain('aria-label="敌人抗性雷达图"')
  })

  it('9 五处原 @click 全部保留且无非法 tablist/tab/aria-selected', () => {
    expect(source).toContain('@click="categoryFilter = c.key"')
    expect(source).toContain('@click="counterElement = el"')
    expect((source.match(/@click="toggleEnemy\(en\.id\)"/g) || []).length).toBe(2)
    expect(source).toContain('@click="keyword = \'\'"')
    expect(source).not.toMatch(/role="tablist"/)
    expect(source).not.toMatch(/role="tab"/)
    expect(source).not.toMatch(/aria-selected=/)
  })
})
