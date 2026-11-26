// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R566 选关页 stage.vue 3 处裸 @click 伪按钮键盘可达契约（源码扫描）。
 * 修复形态：就地追加 role=button + tabindex=0 + 语义化 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * 方案 A 全局增强器 Fail Loud 排除（越界他页 + 动态名无法可靠派生）；先例 R561/R562/R564。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

const SRC = fileURLToPath(new URL('./stage/stage.vue', import.meta.url))
const MAIN = fileURLToPath(new URL('../main.ts', import.meta.url))
const BATTLES = fileURLToPath(new URL('./battles/battles.vue', import.meta.url))
const RANK = fileURLToPath(new URL('./rank/rank.vue', import.meta.url))
const source = readFileSync(SRC, 'utf8')
const main = readFileSync(MAIN, 'utf8')
const battles = readFileSync(BATTLES, 'utf8')
const rank = readFileSync(RANK, 'utf8')

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

describe('R566 stage.vue 3 处伪按钮键盘可达（核心）', () => {
  it('1 章节 chip 开标签补 role/tabindex/语义 aria-label/keydown 复用 switchChapter', () => {
    const b = blockFrom('v-for="ch in chapters"')
    expect(b).toContain('v-for="ch in chapters"')
    expect(b).toContain('class="chapter-chip"')
    expect(b).toMatch(/role="button"/)
    expect(b).toMatch(/tabindex="0"/)
    expect(b).toContain(':aria-label="ch.name + (curChapter === ch.id ?')
    expect(b).toContain('@keydown.enter="switchChapter(ch.id)"')
    expect(b).toContain('@keydown.space.prevent="switchChapter(ch.id)"')
    expect(b).toContain('@click="switchChapter(ch.id)"')
  })

  it('2 关卡格开标签补 role/tabindex/语义 aria-label/keydown 复用 enter', () => {
    const b = blockFrom('v-for="lv in chapterLevels"')
    expect(b).toContain('v-for="lv in chapterLevels"')
    expect(b).toContain('class="level-cell"')
    expect(b).toMatch(/role="button"/)
    expect(b).toMatch(/tabindex="0"/)
    expect(b).toContain(':aria-label="\'第 \' + lv.id + \' 关 \' + lv.name"')
    expect(b).toContain('@keydown.enter="enter(lv.id)"')
    expect(b).toContain('@keydown.space.prevent="enter(lv.id)"')
    expect(b).toContain('@click="enter(lv.id)"')
  })

  it('3 开始战斗按钮补 role/tabindex/aria-label 绑定 startLabel/keydown 复用 startBattle', () => {
    const b = blockFrom('class="btn btn-primary start-btn"')
    expect(b).toMatch(/role="button"/)
    expect(b).toMatch(/tabindex="0"/)
    expect(b).toContain(':aria-label="startLabel"')
    expect(b).toContain('@keydown.enter="startBattle"')
    expect(b).toContain('@keydown.space.prevent="startBattle"')
    expect(b).toContain('@click="startBattle"')
    expect((source.match(/@click=/g) || []).length).toBe(3)
  })
})

describe('R566 守卫（业务/数据/样式/对照零改）', () => {
  it('4 switchChapter / enter / startBattle 与 computed 业务零改', () => {
    expect(source).toContain('function switchChapter(id: number) {')
    expect(source).toContain('curChapter.value = id')
    expect(source).toContain('selectedId.value = 0')
    expect(source).toContain('function enter(id: number) {')
    expect(source).toContain('void nextTick(')
    expect(source).toContain('function startBattle() {')
    expect(source).toContain('battleNavigating.value')
    expect(source).toContain('const startLabel = computed(')
    expect(source).toContain('const canEnter = computed(')
    expect(source).toContain('const chapters = computed(')
    expect(source).toContain('const chapterLevels = computed')
  })

  it('5 api / store / 样式零改，未引入全局增强器', () => {
    expect(source).toContain("import * as api from '@/api/client'")
    expect(source).toContain("import { useGameStore } from '@/store/game'")
    expect(source).toContain('.chapter-chip {')
    expect(source).toContain('.level-cell {')
    expect(source).toContain('.start-btn {')
    expect(source).not.toMatch(/<button[\s>]/)
    expect(main).toContain('createSSRApp(App)')
    expect(main).not.toMatch(/MutationObserver/)
    expect(main).not.toMatch(/install[A-Z][A-Za-z]*A11y/)
  })

  it('6 正面对照 battles/rank 已修控件与 stage svg aria-label 零改', () => {
    expect(battles).toContain('class="filter-chip"')
    expect(battles).toContain(':aria-label="f.label + (filter === f.key')
    expect(rank).toContain('v-for="t in tabs"')
    expect(rank).toContain(':aria-pressed="curType === t.key"')
    expect(source).toContain('aria-label="关卡难度曲线"')
  })

  it('7 三处原 @click 保留且无非法 tablist/tab/aria-selected', () => {
    expect(source).toContain('@click="switchChapter(ch.id)"')
    expect(source).toContain('@click="enter(lv.id)"')
    expect(source).toContain('@click="startBattle"')
    expect(source).not.toMatch(/role="tablist"/)
    expect(source).not.toMatch(/role="tab"/)
    expect(source).not.toMatch(/aria-selected=/)
  })
})
