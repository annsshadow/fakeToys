// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R574 专精页 mastery.vue 3 类裸 @click 伪按钮键盘可达契约（源码扫描）。
 * 修复形态：就地追加 role + tabindex=0 + 语义化 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * 家族 tab 选 role=button + aria-pressed（3 枚全 Tab 可达；WAI-ARIA Tabs roving tabindex 需方向键导航，
 * 原逻辑仅内联 curFamily=f.family 赋值，无方向键逻辑，半吊子 tablist 反伤 AT）；与 R564/R572 先例一致。
 * 方案 A 全局增强器 Fail Loud 排除（越界他页 + 动态名无法可靠派生）；先例 R561–R572。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

const SRC = fileURLToPath(new URL('./mastery/mastery.vue', import.meta.url))
const MAIN = fileURLToPath(new URL('../main.ts', import.meta.url))
const CLIENT = fileURLToPath(new URL('../api/client.ts', import.meta.url))
const BATTLES = fileURLToPath(new URL('./battles/battles.vue', import.meta.url))
const RANK = fileURLToPath(new URL('./rank/rank.vue', import.meta.url))
const STAGE = fileURLToPath(new URL('./stage/stage.vue', import.meta.url))
const source = readFileSync(SRC, 'utf8')
const main = readFileSync(MAIN, 'utf8')
const client = readFileSync(CLIENT, 'utf8')
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
function hasA11y(b: string, labelCheck: (b: string) => boolean): void {
  expect(b).toContain('role="button"')
  expect(b).toContain('tabindex="0"')
  expect(/aria-label=/.test(b)).toBe(true)
  expect(labelCheck(b)).toBe(true)
}

describe('R574 mastery.vue 3 类伪按钮键盘可达（核心）', () => {
  it('1 重铸入口补 role=button/动态 aria-label/keydown 复用 resetMastery', () => {
    const b = blockFrom(':class="{ \'link-off\': resetting || selected.length === 0 || resetTokens <= 0 }"')
    expect(b).toContain('@click="resetMastery"')
    hasA11y(b, (s) => s.includes('resetting') && s.includes('重铸'))
    expect(b).toContain('@keydown.enter="resetMastery"')
    expect(b).toContain('@keydown.space.prevent="resetMastery"')
  })
  it('2 家族 tab 补 role=button/aria-pressed/aria-label/keydown 复用内联赋值', () => {
    const b = blockFrom(':class="{ active: curFamily === f.family }"')
    expect(b).toContain('@click="curFamily = f.family"')
    hasA11y(b, (s) => s.includes(':aria-label="f.name"'))
    expect(b).toContain(':aria-pressed="curFamily === f.family"')
    expect(b).toContain('@keydown.enter="curFamily = f.family"')
    expect(b).toContain('@keydown.space.prevent="curFamily = f.family"')
    expect(source).not.toMatch(/role="tablist"/)
    expect(source).not.toMatch(/role="tab"/)
    expect(source).not.toMatch(/aria-selected=/)
    expect((source.match(/:aria-pressed="curFamily === f.family"/g) || []).length).toBe(1)
    expect((source.match(/@keydown.enter="curFamily = f.family"/g) || []).length).toBe(1)
    expect((source.match(/@keydown.space.prevent="curFamily = f.family"/g) || []).length).toBe(1)
  })
  it('3 专精节点补 role=button/动态 aria-label/keydown 复用 toggle(n)', () => {
    const b = blockFrom('selectable: !selected.includes(n.id) && !isLocked(n),')
    expect(b).toContain('@click="toggle(n)"')
    hasA11y(b, (s) => s.includes('n.name') && s.includes('n.value'))
    expect(b).toContain('@keydown.enter="toggle(n)"')
    expect(b).toContain('@keydown.space.prevent="toggle(n)"')
  })
})

describe('R574 守卫（业务/数据/样式/对照零改）', () => {
  it('4 业务 handler 零改：resetMastery/toggle/load/nodesInLayer/isLocked/curFamily', () => {
    expect(source).toContain('async function resetMastery() {')
    expect(source).toContain('if (resetting.value || allocBusy.value) return')
    expect(source).toContain('const res = await api.resetMastery()')
    expect(source).toContain('async function toggle(n: MasteryNode) {')
    expect(source).toContain('const res = await api.allocateMastery(n.id)')
    expect(source).toContain('if (allocBusy.value || resetting.value) return')
    expect(source).toContain('async function load() {')
    expect(source).toContain('const res = await api.fetchMastery()')
    expect(source).toContain('function nodesInLayer(layer: number): MasteryNode[] {')
    expect(source).toContain('function isLocked(n: MasteryNode): boolean {')
    expect(source).toContain("const curFamily = ref('flame')")
  })
  it('5 数据源/样式/他页/main 零改，未引入全局增强器', () => {
    expect(source).toContain("import * as api from '@/api/client'")
    expect(source).toContain("import { useGameStore } from '@/store/game'")
    expect(source).toContain('.tab-chip.active {')
    expect(source).toContain('.node.selectable {')
    expect(source).toContain('<style scoped>')
    expect(source).not.toMatch(/<button[\s>]/)
    expect(client).toContain('fetchMastery')
    expect(client).toContain('allocateMastery')
    expect(client).toContain('resetMastery')
    expect(main).toContain('createSSRApp(App)')
    expect(main).not.toMatch(/MutationObserver/)
    expect(main).not.toMatch(/install[A-Z][A-Za-z]*A11y/)
  })
  it('6 正面对照零改 + 三处 @click 保留', () => {
    expect(battles).toContain(':aria-label="f.label + (filter === f.key')
    expect(rank).toContain(':aria-pressed="curType === t.key"')
    expect(stage).toContain('@keydown.enter="switchChapter(ch.id)"')
    expect(source).toContain('@click="resetMastery"')
    expect(source).toContain('@click="curFamily = f.family"')
    expect(source).toContain('@click="toggle(n)"')
  })
  it('7 三处 @click 总数恒为 3 且无原生 button（真父/修复版均成立）', () => {
    expect((source.match(/@click=/g) || []).length).toBe(3)
    expect(source).not.toMatch(/<button[\s>]/)
  })
})
