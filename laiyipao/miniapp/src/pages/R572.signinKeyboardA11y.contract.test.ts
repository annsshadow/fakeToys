// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R572 签到页 signin.vue 5 类裸 @click 伪按钮键盘可达契约（源码扫描）。
 * 修复形态：就地追加 role + tabindex=0 + 语义化 aria-label + @keydown.enter/.space.prevent 复用原 handler。
 * 周期 tab 选 role=button + aria-pressed（3 枚全 Tab 可达；WAI-ARIA Tabs roving tabindex 需方向键导航，switchScope 未实现，半吊子 tablist 反伤 AT）；与 R564 rank.vue 先例一致。
 * 方案 A 全局增强器 Fail Loud 排除（越界他页 + 动态名无法可靠派生）；先例 R561–R570。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath, URL } from 'node:url'

const SRC = fileURLToPath(new URL('./signin/signin.vue', import.meta.url))
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
function lineWith(anchor: string): string {
  return source.split('\n').find((l) => l.includes(anchor)) || ''
}
function hasA11y(b: string, labelCheck: (b: string) => boolean): void {
  expect(b).toContain('role="button"')
  expect(b).toContain('tabindex="0"')
  expect(/aria-label=/.test(b)).toBe(true)
  expect(labelCheck(b)).toBe(true)
}

describe('R572 signin.vue 5 类伪按钮键盘可达（核心）', () => {
  it('1 签到按钮补 role=button/动态 aria-label/keydown 复用 doSignIn', () => {
    const b = blockFrom("style=\"margin-top: 20rpx\"")
    expect(b).toContain('@click="doSignIn"')
    hasA11y(b, (s) => s.includes('canSign') && s.includes('cycleCompleteText'))
    expect(b).toContain('@keydown.enter="doSignIn"')
    expect(b).toContain('@keydown.space.prevent="doSignIn"')
  })
  it('2 离线领取补 role=button/动态 aria-label/keydown 复用 claimOffline', () => {
    const b = blockFrom('class="btn btn-mini btn-primary offline-claim"')
    expect(b).toContain('@click="claimOffline"')
    hasA11y(b, (s) => s.includes('offlineReward.coin'))
    expect(b).toContain('@keydown.enter="claimOffline"')
    expect(b).toContain('@keydown.space.prevent="claimOffline"')
  })
  it('3 周期 tab 补 role=button/aria-pressed/aria-label/keydown 复用 switchScope', () => {
    const b = blockFrom('v-for="s in SCOPES"')
    expect(b).toContain('@click="switchScope(s.key)"')
    hasA11y(b, (s) => s.includes(':aria-label="s.label"'))
    expect(b).toContain(':aria-pressed="scope === s.key"')
    expect(b).toContain('@keydown.enter="switchScope(s.key)"')
    expect(b).toContain('@keydown.space.prevent="switchScope(s.key)"')
    expect(source).not.toMatch(/role="tablist"/)
    expect(source).not.toMatch(/role="tab"/)
    expect(source).not.toMatch(/aria-selected=/)
  })
  it('4 任务领取补 role=button/动态 aria-label/keydown 复用 claim(t.id)', () => {
    const b = blockFrom('style="width: 140rpx; height: 64rpx; font-size: 24rpx"')
    expect(b).toContain('@click="claim(t.id)"')
    hasA11y(b, (s) => s.includes('t.name'))
    expect(b).toContain('@keydown.enter="claim(t.id)"')
    expect(b).toContain('@keydown.space.prevent="claim(t.id)"')
  })
  it('5 兑换补 role=button/aria-label=兑换/keydown 复用 doRedeem，五处 @click 保留', () => {
    const b = lineWith('@click="doRedeem"')
    hasA11y(b, (s) => s.includes('aria-label="兑换"'))
    expect(b).toContain('@keydown.enter="doRedeem"')
    expect(b).toContain('@keydown.space.prevent="doRedeem"')
    expect((source.match(/@click=/g) || []).length).toBe(5)
  })
})

describe('R572 守卫（业务/数据/样式/对照零改）', () => {
  it('6 业务 handler 与竞态守卫零改：switchScope/claimOffline/doSignIn/claim/doRedeem', () => {
    expect(source).toContain('function switchScope(k: ScopeKey) {')
    expect(source).toContain('if (scope.value === k) return')
    expect(source).toContain('async function claimOffline() {')
    expect(source).toContain('await api.claimOfflineReward()')
    expect(source).toContain('async function doSignIn() {')
    expect(source).toContain('if (signingIn.value || !canSign.value) return')
    expect(source).toContain('async function claim(')
    expect(source).toContain('async function doRedeem() {')
    expect(source).toContain('let loadSeq = 0')
    expect(source).toContain('if (seq !== loadSeq) return')
  })
  it('7 数据源/样式/他页/main 零改，未引入全局增强器', () => {
    expect(source).toContain("import * as api from '@/api/client'")
    expect(source).toContain("import { useGameStore } from '@/store/game'")
    expect(source).toContain('.offline-msg {')
    expect(source).toContain('.task-claimable {')
    expect(source).toContain('<style scoped>')
    expect(source).not.toMatch(/<button[\s>]/)
    expect(client).toContain('fetchSignInStatus')
    expect(client).toContain('signIn')
    expect(main).toContain('createSSRApp(App)')
    expect(main).not.toMatch(/MutationObserver/)
    expect(main).not.toMatch(/install[A-Z][A-Za-z]*A11y/)
  })
  it('8 正面对照零改 + 五处 @click 保留', () => {
    expect(battles).toContain(':aria-label="f.label + (filter === f.key')
    expect(rank).toContain(':aria-pressed="curType === t.key"')
    expect(stage).toContain('@keydown.enter="switchChapter(ch.id)"')
    expect(source).toContain('@click="doSignIn"')
    expect(source).toContain('@click="claimOffline"')
    expect(source).toContain('@click="switchScope(s.key)"')
    expect(source).toContain('@click="claim(t.id)"')
    expect(source).toContain('@click="doRedeem"')
  })
  it('9 周期三枚均带 aria-pressed 且 @click 总数恰为 5', () => {
    expect((source.match(/:aria-pressed="scope === s.key"/g) || []).length).toBe(1)
    expect((source.match(/@keydown.enter="switchScope\(s.key\)"/g) || []).length).toBe(1)
    expect((source.match(/@click=/g) || []).length).toBe(5)
  })
})
