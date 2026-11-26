// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R564 排行榜页 rank.vue 3 枚 @click 伪按钮 tab 键盘可达契约（源码扫描）。
 * 修复形态选方案 b（toggle button）：role=button + aria-pressed + tabindex=0 + keydown 复用 switchTab。
 * 理由见文档：验收要求 3 枚均 Tab 可达，方案 a 的 roving tabindex 需方向键配套（未要求），半吊子 tablist 反伤 AT。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

const SRC = fileURLToPath(new URL('./rank/rank.vue', import.meta.url))
const MAIN = fileURLToPath(new URL('../main.ts', import.meta.url))
const BATTLES = fileURLToPath(new URL('./battles/battles.vue', import.meta.url))
const source = readFileSync(SRC, 'utf8')
const main = readFileSync(MAIN, 'utf8')
const battles = readFileSync(BATTLES, 'utf8')

// 从起始锚点行向下扫到开标签结束（trimEnd 以 '>' 收尾），适配多行开标签
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

describe('R564 rank.vue 3 枚 tab 键盘可达（核心，方案 b toggle button）', () => {
  it('1 三枚 tab v-for 开标签补 role=button/tabindex=0/aria-label/aria-pressed/keydown', () => {
    const b = blockFrom('v-for="t in tabs"')
    expect(b).toContain('v-for="t in tabs"')
    expect(b).toContain('class="tab"')
    expect(b).toMatch(/role="button"/)
    expect(b).toMatch(/tabindex="0"/)
    expect(b).toContain(':aria-label="t.label"')
    expect(b).toContain(':aria-pressed="curType === t.key"')
    expect(b).toContain('@keydown.enter="switchTab(t.key)"')
    expect(b).toContain('@keydown.space.prevent="switchTab(t.key)"')
    expect(b).toContain('@click="switchTab(t.key)"')
  })

  it('2 选中态程序化可读：aria-pressed 绑定 curType 与 active class 同源', () => {
    const b = blockFrom('v-for="t in tabs"')
    expect(b).toContain(':class="{ active: curType === t.key }"')
    expect(b).toContain(':aria-pressed="curType === t.key"')
    // 方案 b 明确不选 tablist/tab，防半吊子 WAI-ARIA Tabs
    expect(source).not.toMatch(/role="tablist"/)
    expect(source).not.toMatch(/role="tab"/)
    expect(source).not.toMatch(/aria-selected=/)
  })

  it('3 三枚 tab 静态文案硬编码保留（战力/关卡/效率榜）且 @click 唯一', () => {
    expect(source).toContain("const tabs = [")
    expect(source).toContain("{ key: 'power', label: '战力榜' }")
    expect(source).toContain("{ key: 'stage', label: '关卡榜' }")
    expect(source).toContain("{ key: 'efficiency', label: '效率榜' }")
    expect((source.match(/@click=/g) || []).length).toBe(1)
  })
})

describe('R564 守卫（业务/榜单/样式/对照零改）', () => {
  it('4 switchTab / curType / load 业务函数体零改', () => {
    expect(source).toContain("const curType = ref('power')")
    expect(source).toContain('function switchTab(key: string) {')
    expect(source).toContain('curType.value = key')
    expect(source).toContain('myRank.value = null')
    expect(source).toContain('load()')
    expect(source).toContain('async function load() {')
    expect(source).toContain('const seq = ++loadSeq')
    expect(source).toContain('if (seq !== loadSeq) return')
  })

  it('5 榜单展示与 API 零改', () => {
    expect(source).toContain("import * as api from '@/api/client'")
    expect(source).toContain('api.fetchLeaderboard(type)')
    expect(source).toContain('api.fetchMyRank(type).catch(() => null)')
    expect(source).toContain('class="rank-row"')
    expect(source).toContain('class="card my-rank"')
    expect(source).toContain('const curDesc = computed(() => {')
    expect(source).toContain('按最高通关关卡排名。')
  })

  it('6 main.ts 零全局增强器 + 本页样式零改', () => {
    expect(main).toContain('createSSRApp(App)')
    expect(main).not.toMatch(/MutationObserver/)
    expect(main).not.toMatch(/install[A-Z][A-Za-z]*A11y/)
    expect(source).toContain('.tabs {')
    expect(source).toContain('.tab {')
    expect(source).toContain('.tab.active {')
    expect(source).toContain('.rank-row.me {')
    expect(source).not.toMatch(/<button[\s>]/)
  })

  it('7 battles 正面对照（已修控件/svg aria-label）零改', () => {
    expect(battles).toContain('class="filter-chip"')
    expect(battles).toContain(':aria-label="f.label + (filter === f.key')
    expect(battles).toContain('aria-label="战绩分数趋势"')
    expect(battles).toContain('<svg class="trend-svg" viewBox="0 0 300 96" aria-label="战绩分数趋势">')
  })
})
