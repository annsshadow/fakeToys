// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R100 可点击交互行键盘可达性契约（test-first RED）。
 * 真实 fs.readFileSync 读 laiyipao/miniapp/src/pages 下 4 个 .vue，禁止内嵌实现副本。
 * 缺口：8 处可点交互行（battle-row/ann-item/resist-toggle/action-card/grid-item×2/mg-row/elem-chip/chapter-chip）
 *   缺 role="button" / tabindex="0" / @keydown.enter 同名 handler（WCAG 2.1.1 键盘盲区）。
 * 定位：以 @click handler 表达式 + class 锚点稳健定位开标签，不依赖行号（容忍上游插行漂移）。
 * 范式：R60 role=button 四件套；R95 me:11；R99 text.link 19 处；t254 R100-A 侦察裁定。
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

type Case = { name: string; file: string; cls: string; handler: string; tag?: string }

// 8 处候选：3 核心交互行 + 5 精选扩展（grid-item 两处同 handler，须全部覆盖）。
const CASES: Case[] = [
  { name: 'battles 战报展开行', file: 'pages/battles/battles.vue', cls: 'battle-row', handler: 'toggleDetail(it)', tag: '<view' },
  { name: 'index 公告展开行', file: 'pages/index/index.vue', cls: 'ann-item', handler: 'toggleAnn(a.id)', tag: '<view' },
  { name: 'enemies 抗性图展开', file: 'pages/enemies/enemies.vue', cls: 'resist-toggle', handler: 'toggleEnemy(en.id)', tag: '<text' },
  { name: 'index 构筑卡片', file: 'pages/index/index.vue', cls: 'action-card', handler: 'goStage', tag: '<view' },
  { name: 'index 玩法/图鉴卡片', file: 'pages/index/index.vue', cls: 'grid-item', handler: 'go(m.path)', tag: '<view' },
  { name: 'index 边际收益行', file: 'pages/index/index.vue', cls: 'mg-row', handler: 'm.hint && go(m.hint)', tag: '<view' },
  { name: 'index 元素 chip', file: 'pages/index/index.vue', cls: 'elem-chip', handler: "go('/pages/reactions/reactions')", tag: '<view' },
  { name: 'stage 章节切换 chip', file: 'pages/stage/stage.vue', cls: 'chapter-chip', handler: 'switchChapter(ch.id)', tag: '<view' },
]

// 稳健定位：按 @click handler 反查开标签，再用 class 精确区分同 handler 多元素（如 toggleEnemy 的 rxn-name/resist-toggle）。
// 返回全部匹配开标签（grid-item 两处必须都命中），不硬编码行号。
function blocksFor(src: string, c: Case): string[] {
  const out: string[] = []
  const tag = c.tag || '<view'
  const click = '@click="' + c.handler + '"'
  let from = 0
  while (true) {
    const ci = src.indexOf(click, from)
    if (ci < 0) break
    from = ci + 1
    const start = src.lastIndexOf(tag, ci)
    if (start < 0 || ci - start > 600) continue
    const end = src.indexOf('>', ci)
    if (end < 0 || end - ci > 300) continue
    const block = src.slice(start, end + 1)
    // 只认静态 class="..."（前置空白/行首），排除 :class / v-bind:class 干扰
    const cm = block.match(/(?:^|\s)class="([^"]*)"/)
    if (!cm) continue
    if (!cm[1].split(/\s+/).includes(c.cls)) continue
    out.push(block)
  }
  return out
}

const SRC: Record<string, string> = {}
for (const c of CASES) if (!SRC[c.file]) SRC[c.file] = read(c.file)

describe('R100 可点击交互行键盘可达性契约', () => {
  // 缺口断言：8 处 RED 必挂，每处校验三属性 + 同名 handler
  for (const c of CASES) {
    it(c.name + ' 须 role=button + tabindex=0 + @keydown.enter 同名 handler', () => {
      const blocks = blocksFor(SRC[c.file], c)
      expect(blocks.length, '必须能定位 ' + c.file + ' 的 ' + c.cls + ' / ' + c.handler).toBeGreaterThan(0)
      for (const b of blocks) {
        expect(b, '须补 role="button"').toContain('role="button"')
        expect(b, '须补 tabindex="0"').toContain('tabindex="0"')
        expect(b, '须补 @keydown.enter="' + c.handler + '"（同名 handler）')
          .toContain('@keydown.enter="' + c.handler + '"')
      }
    })
  }

  it('8 处原 @click 入口全部保留（不变量）', () => {
    for (const c of CASES) {
      expect(SRC[c.file], c.name + ' 原 @click="' + c.handler + '" 须保留')
        .toContain('@click="' + c.handler + '"')
    }
  })

  it('令牌三锚点不动且无 --r100-（不变量）', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toMatch(/--r100-/)
  })

  it('业务源码零 --r100- 令牌（不变量）', () => {
    for (const f of Object.keys(SRC)) expect(SRC[f], f).not.toMatch(/--r100-/)
  })

  it('排除项不被误改：mastery .node 锁定态 + enemies .rxn-name 防双入口（不变量）', () => {
    const mastery = read('pages/mastery/mastery.vue')
    const nodeBlocks = blocksFor(mastery, { name: '', file: '', cls: 'node', handler: 'toggle(n)', tag: '<view' })
    expect(nodeBlocks.length, '须能定位 mastery .node').toBeGreaterThan(0)
    for (const b of nodeBlocks) {
      expect(b, 'mastery .node 含 isLocked 视觉态，本轮排除，不得补 role').not.toContain('role="button"')
      expect(b, 'mastery .node 本轮排除，不得补 tabindex').not.toContain('tabindex="0"')
      expect(b, 'mastery .node 本轮排除，不得补 @keydown.enter').not.toContain('@keydown.enter')
    }
    // 抗性图二选一：契约锁定 .resist-toggle，.rxn-name 不得同步补键以免双 Tab 停靠
    const enemies = read('pages/enemies/enemies.vue')
    const nameBlocks = blocksFor(enemies, { name: '', file: '', cls: 'rxn-name', handler: 'toggleEnemy(en.id)', tag: '<text' })
    expect(nameBlocks.length, '须能定位 enemies .rxn-name').toBeGreaterThan(0)
    for (const b of nameBlocks) {
      expect(b, '.rxn-name 不纳入本轮（避免与 .resist-toggle 双入口）').not.toContain('role="button"')
      expect(b, '.rxn-name 不纳入本轮（避免双 Tab 停靠）').not.toContain('@keydown.enter')
    }
  })
})
