// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R562 战报页 battles.vue 6 类裸 @click 伪按钮键盘可达契约（源码扫描）。
 * 核心：6 类锚点均就地补 role=button / tabindex=0 / aria-label / @keydown.enter / @keydown.space，
 * 复用原 handler；守卫：业务 handler / API / @click / svg 正面对照 / main.ts / 样式零改。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

const SRC = fileURLToPath(new URL('./battles/battles.vue', import.meta.url))
const MAIN = fileURLToPath(new URL('../main.ts', import.meta.url))
const source = readFileSync(SRC, 'utf8')
const main = readFileSync(MAIN, 'utf8')

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

// 单行开标签：取包含 needle 的整行
function lineBlock(needle: string): string {
  const i = source.indexOf(needle)
  if (i < 0) return ''
  const start = source.lastIndexOf('\n', i) + 1
  const nl = source.indexOf('\n', i)
  return source.slice(start, nl < 0 ? source.length : nl)
}

function hasA11y(block: string): boolean {
  return /role="button"/.test(block)
    && /tabindex="0"/.test(block)
    && /aria-label=/.test(block)
    && /@keydown\.enter/.test(block)
    && /@keydown\.space/.test(block)
}

describe('R562 battles.vue 6 类伪按钮键盘可达（核心）', () => {
  it('1 filter-chip 三态键盘可达且动态名含筛选态', () => {
    const b = blockFrom('class="filter-chip"')
    expect(b).toContain('class="filter-chip"')
    expect(hasA11y(b)).toBe(true)
    expect(b).toContain('f.label')
    expect(b).toContain('filter === f.key')
    expect(b).toContain('@keydown.enter="filter = f.key"')
  })

  it('2 空态去关卡链接键盘可达', () => {
    const b = lineBlock('class="empty-hint link"')
    expect(hasA11y(b)).toBe(true)
    expect(b).toContain('aria-label="去关卡页挑一关开打"')
    expect(b).toContain('@click="goStage"')
    expect(b).toContain('@keydown.enter="goStage"')
  })

  it('3 战报行键盘可达且动态名含关卡/胜负/分数', () => {
    const b = blockFrom('class="battle-row tap"')
    expect(b).toContain('class="battle-row tap"')
    expect(hasA11y(b)).toBe(true)
    expect(b).toContain('it.level_id')
    expect(b).toContain('it.result')
    expect(b).toContain('it.score')
    expect(b).toContain('@keydown.enter="toggleDetail(it)"')
  })

  it('4 复制回放哈希键盘可达且 keydown .stop 防父行误触', () => {
    const b = lineBlock('class="btn copy-hash-btn"')
    expect(hasA11y(b)).toBe(true)
    expect(b).toContain('aria-label="复制回放哈希"')
    expect(b).toContain('@click="copyHash(detail.replay_hash)"')
    expect(b).toMatch(/@keydown\.enter\.stop="copyHash\(detail\.replay_hash\)"/)
    expect(b).toMatch(/@keydown\.space\.stop\.prevent="copyHash\(detail\.replay_hash\)"/)
  })

  it('5 去验真此局键盘可达且 keydown .stop 防父行误触', () => {
    const b = lineBlock('class="btn verify-btn"')
    expect(hasA11y(b)).toBe(true)
    expect(b).toContain('aria-label="去验真此局"')
    expect(b).toContain('@click="goVerify(it.id)"')
    expect(b).toMatch(/@keydown\.enter\.stop="goVerify\(it\.id\)"/)
    expect(b).toMatch(/@keydown\.space\.stop\.prevent="goVerify\(it\.id\)"/)
  })

  it('6 加载更多键盘可达', () => {
    const b = lineBlock('v-if="hasMore" class="load-more"')
    expect(hasA11y(b)).toBe(true)
    expect(b).toContain('aria-label="加载更多"')
    expect(b).toContain('@click="loadMore"')
    expect(b).toContain('@keydown.enter="loadMore"')
  })
})

describe('R562 守卫（业务/API/对照/零改）', () => {
  it('7 业务 handler 函数体零改', () => {
    expect(source).toContain('async function reload(): Promise<void> {')
    expect(source).toContain('let listReqSeq = 0')
    expect(source).toContain('const seq = ++listReqSeq')
    expect(source).toContain('async function toggleDetail(it: BattleHistoryItem) {')
    expect(source).toContain('detailLoadingIds.value.add(it.id)')
    expect(source).toContain('async function loadDetail(it: BattleHistoryItem): Promise<void> {')
    expect(source).toContain('function goVerify(battleId: number) {')
    expect(source).toContain('function goStage() {')
    expect(source).toContain('function copyHash(hash: string) {')
    expect(source).toContain('async function loadMore() {')
    expect(source).toContain('uni.setClipboardData({')
  })

  it('8 API 与六处 @click 原 handler 全保留', () => {
    expect(source).toContain("import * as api from '@/api/client'")
    expect(source).toContain('api.myBattleHistory(20, 0,')
    expect(source).toContain('api.myBattleDetail(it.id)')
    expect(source).toContain('@click="filter = f.key"')
    expect(source).toContain('@click="goStage"')
    expect(source).toContain('@click="toggleDetail(it)"')
    expect(source).toContain('@click="copyHash(detail.replay_hash)"')
    expect(source).toContain('@click="goVerify(it.id)"')
    expect(source).toContain('@click="loadMore"')
  })

  it('9 main.ts 零全局增强器 + 样式零改', () => {
    expect(main).toContain('createSSRApp(App)')
    expect(main).not.toMatch(/MutationObserver/)
    expect(main).not.toMatch(/install[A-Z][A-Za-z]*A11y/)
    expect(source).toContain('.filter-chip.active {')
    expect(source).toContain('.copy-hash-btn {')
    expect(source).toContain('.load-more {')
    expect(source).toContain('.trend-svg {')
  })

  it('10 svg aria-label 正面对照零改 + 未新增原生 button', () => {
    expect(source).toContain('aria-label="战绩分数趋势"')
    expect(source).toContain('<svg class="trend-svg" viewBox="0 0 300 96" aria-label="战绩分数趋势">')
    // 本页零原生 <button>： uni-button 另一族不属本轮，禁止误改
    expect(source).not.toMatch(/<button[\s>]/)
  })
})
