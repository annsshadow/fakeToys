// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R570 启动首页 index.vue 9 类裸 @click 伪按钮键盘可达契约（源码扫描）。
 * 修复形态：就地追加 role + tabindex=0 + 语义化 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * 方案 A 全局增强器 Fail Loud 排除（越界他页 + 动态名无法可靠派生）；先例 R561/R562/R564/R566/R568。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

const SRC = fileURLToPath(new URL('./index/index.vue', import.meta.url))
const MAIN = fileURLToPath(new URL('../main.ts', import.meta.url))
const CLIENT = fileURLToPath(new URL('../api/client.ts', import.meta.url))
const GAME = fileURLToPath(new URL('../store/game.ts', import.meta.url))
const BATTLES = fileURLToPath(new URL('./battles/battles.vue', import.meta.url))
const RANK = fileURLToPath(new URL('./rank/rank.vue', import.meta.url))
const STAGE = fileURLToPath(new URL('./stage/stage.vue', import.meta.url))
const source = readFileSync(SRC, 'utf8')
const main = readFileSync(MAIN, 'utf8')
const client = readFileSync(CLIENT, 'utf8')
const game = readFileSync(GAME, 'utf8')
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
function lineWith(anchor: string): string {
  return source.split('\n').find((l) => l.includes(anchor)) || ''
}
function hasA11y(b: string, role: string, labelCheck: (b: string) => boolean): void {
  expect(b).toContain('role="' + role + '"')
  expect(b).toContain('tabindex="0"')
  expect(/aria-label=/.test(b)).toBe(true)
  expect(labelCheck(b)).toBe(true)
}

describe('R570 index.vue 9 类伪按钮键盘可达（核心）', () => {
  it('1 retry 补 role=button/aria-label=重试/keydown 复用 retry', () => {
    const b = lineWith('style="margin-top: 20rpx"')
    expect(b).toContain('@click="retry"')
    hasA11y(b, 'button', (s) => s.includes('aria-label="重试"'))
    expect(b).toContain('@keydown.enter="retry"')
    expect(b).toContain('@keydown.space.prevent="retry"')
  })
  it('2 annAll 补 role=link/aria-label/keydown 复用 goAnnouncements', () => {
    const b = lineWith('class="ann-all"')
    expect(b).toContain('@click="goAnnouncements"')
    hasA11y(b, 'link', (s) => s.includes('aria-label="查看全部公告"'))
    expect(b).toContain('@keydown.enter="goAnnouncements"')
    expect(b).toContain('@keydown.space.prevent="goAnnouncements"')
  })
  it('3 annItem 补 role=button/动态 aria-label/aria-expanded/keydown 复用 toggleAnn', () => {
    const b = blockFrom('v-for="(a, i) in visibleAnnouncements"')
    expect(b).toContain('class="ann-item"')
    expect(b).toContain('@click="toggleAnn(a.id)"')
    hasA11y(b, 'button', (s) => s.includes('a.title') && s.includes('aria-label='))
    expect(b).toContain(':aria-expanded="expanded.has(a.id)"')
    expect(b).toContain('@keydown.enter="toggleAnn(a.id)"')
    expect(b).toContain('@keydown.space.prevent="toggleAnn(a.id)"')
  })
  it('4 dimHint 补 role=link/动态 aria-label/keydown 复用 go(d.hint)', () => {
    const b = blockFrom('v-if="d.pct === 0 && d.hint"')
    expect(b).toContain('class="dim-hint link"')
    expect(b).toContain('@click="go(d.hint)"')
    hasA11y(b, 'link', (s) => s.includes('d.label') && s.includes('d.hintLabel'))
    expect(b).toContain('@keydown.enter="go(d.hint)"')
    expect(b).toContain('@keydown.space.prevent="go(d.hint)"')
  })
  it('5 mgRow 补 role=link/动态 aria-label/keydown 复用 m.hint && go', () => {
    const b = blockFrom('v-for="(m, i) in marginalGain.slice(0, 3)"')
    expect(b).toContain('class="mg-row tap"')
    expect(b).toContain('@click="m.hint && go(m.hint)"')
    hasA11y(b, 'link', (s) => s.includes('m.label') && s.includes('m.hintLabel'))
    expect(b).toContain('@keydown.enter="m.hint && go(m.hint)"')
    expect(b).toContain('@keydown.space.prevent="m.hint && go(m.hint)"')
  })
  it('6 actionCard 补 role=link/动态 aria-label/keydown 复用 goStage', () => {
    const b = lineWith('class="action-card card-hero tap"')
    expect(b).toContain('@click="goStage"')
    hasA11y(b, 'link', (s) => s.includes('store.unlockedLevel'))
    expect(b).toContain('@keydown.enter="goStage"')
    expect(b).toContain('@keydown.space.prevent="goStage"')
  })
  it('7 playItem 补 role=link/aria-label=m.label/keydown 复用 go(m.path)', () => {
    const b = lineWith('v-for="m in playMenus"')
    expect(b).toContain('class="grid-item tap"')
    expect(b).toContain('@click="go(m.path)"')
    hasA11y(b, 'link', (s) => s.includes(':aria-label="m.label"'))
    expect(b).toContain('@keydown.enter="go(m.path)"')
    expect(b).toContain('@keydown.space.prevent="go(m.path)"')
  })
  it('8 codexItem 补 role=link/aria-label=m.label/keydown 复用 go(m.path)', () => {
    const b = lineWith('v-for="m in codexMenus"')
    expect(b).toContain('class="grid-item tap"')
    expect(b).toContain('@click="go(m.path)"')
    hasA11y(b, 'link', (s) => s.includes(':aria-label="m.label"'))
    expect(b).toContain('@keydown.enter="go(m.path)"')
    expect(b).toContain('@keydown.space.prevent="go(m.path)"')
  })
  it('9 elemChip 补 role=link/动态 aria-label/keydown 复用 go(reactions)', () => {
    const b = blockFrom('v-for="e in elements"')
    expect(b).toContain('class="elem-chip tap"')
    expect(b).toContain("go('/pages/reactions/reactions')")
    hasA11y(b, 'link', (s) => s.includes('e.name'))
    expect(b).toContain('@keydown.enter="go(')
    expect(b).toContain('@keydown.space.prevent="go(')
    expect((source.match(/@click=/g) || []).length).toBe(9)
  })
})

describe('R570 守卫（业务/数据/样式/对照零改）', () => {
  it('10 业务 handler 函数体零改：go/goAnnouncements/goStage/retry/toggleAnn', () => {
    expect(source).toContain('function go(path: string) {')
    expect(source).toContain('uni.navigateTo({ url: path })')
    expect(source).toContain('function goAnnouncements() {')
    expect(source).toContain('/pages/announcements/announcements')
    expect(source).toContain('function goStage() {')
    expect(source).toContain('stage?level=')
    expect(source).toContain('async function retry() {')
    expect(source).toContain('await store.login()')
    expect(source).toContain('function toggleAnn(id: number): void {')
    expect(source).toContain('expanded.value = next')
  })
  it('11 数据源/样式/他页/main 零改，未引入全局增强器', () => {
    expect(source).toContain("import * as api from '@/api/client'")
    expect(source).toContain("import { useGameStore } from '@/store/game'")
    expect(source).toContain('.radar-svg {')
    expect(source).toContain('.ann-all {')
    expect(source).not.toMatch(/<button[\s>]/)
    expect(client).toContain('fetchAnnouncements')
    expect(game).toContain('useGameStore')
    expect(main).toContain('createSSRApp(App)')
    expect(main).not.toMatch(/MutationObserver/)
    expect(main).not.toMatch(/install[A-Z][A-Za-z]*A11y/)
  })
  it('12 正面对照零改 + 九处 @click 保留且无非法 tablist/tab/aria-selected', () => {
    expect(source).toContain('aria-label="构筑五维雷达图"')
    expect(source).toContain('aria-label="闯关进度"')
    expect(battles).toContain(':aria-label="f.label + (filter === f.key')
    expect(rank).toContain(':aria-pressed="curType === t.key"')
    expect(stage).toContain('@keydown.enter="switchChapter(ch.id)"')
    expect(source).toContain('@click="retry"')
    expect(source).toContain('@click="goAnnouncements"')
    expect(source).toContain('@click="toggleAnn(a.id)"')
    expect(source).toContain('@click="go(d.hint)"')
    expect(source).toContain('@click="m.hint && go(m.hint)"')
    expect(source).toContain('@click="goStage"')
    expect((source.match(/@click="go\(m\.path\)"/g) || []).length).toBe(2)
    expect(source).not.toMatch(/role="tablist"/)
    expect(source).not.toMatch(/role="tab"/)
    expect(source).not.toMatch(/aria-selected=/)
  })
})
