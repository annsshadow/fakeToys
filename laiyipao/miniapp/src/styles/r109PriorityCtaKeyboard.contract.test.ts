// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R109 跨页 AI 优先级引导 CTA 键盘可达契约（test-first RED）。
 * 真实 fs.readFileSync 读 10 页 vue，禁止内嵌实现副本。
 * 缺口：10 处 <view class="r12-priority-cta|r3-ai-cta" @click="X"> 裸 view，
 * 无 role=button/tabindex=0/@keydown.enter（WCAG 2.1.1），键盘用户无法激活
 * 「去补强玩法」核心引导入口。定位按 class + @click handler 双锚点，不依赖行号。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const read = (p: string): string => readFileSync(join(srcRoot, p), 'utf8')
const NL = String.fromCharCode(10)

type Cta = { rel: string; cls: string; handler: string }

// 10 改点：4 处 r12-priority-cta + 6 处 r3-ai-cta view 版
const TEN: Cta[] = [
  { rel: 'pages/gems/gems.vue', cls: 'r12-priority-cta', handler: 'r12Go()' },
  { rel: 'pages/skills/skills.vue', cls: 'r12-priority-cta', handler: 'r12Go()' },
  { rel: 'pages/equipment/equipment.vue', cls: 'r12-priority-cta', handler: 'r12Go()' },
  { rel: 'pages/bag/bag.vue', cls: 'r12-priority-cta', handler: 'r12Go()' },
  { rel: 'pages/me/me.vue', cls: 'r3-ai-cta', handler: 'r4GoMe()' },
  { rel: 'pages/bag/bag.vue', cls: 'r3-ai-cta', handler: 'r4GoBag()' },
  { rel: 'pages/battles/battles.vue', cls: 'r3-ai-cta', handler: 'r4GoBattle()' },
  { rel: 'pages/enemies/enemies.vue', cls: 'r3-ai-cta', handler: 'r4FocusEnemy()' },
  { rel: 'pages/rank/rank.vue', cls: 'r3-ai-cta', handler: 'r4GoRank()' },
  { rel: 'pages/skins/skins.vue', cls: 'r3-ai-cta', handler: 'r4FocusSkin()' },
]

function ctaLine(c: Cta): string {
  const lines = read(c.rel).split(NL)
  const hits = lines.filter((l) =>
    l.includes('class="' + c.cls + '"') && l.includes('@click="' + c.handler + '"'))
  expect(hits.length, c.rel + ' ' + c.cls + ' ' + c.handler + ' 应恰有一条 CTA 模板行').toBe(1)
  return hits[0]
}

describe('R109 跨页 AI 优先级引导 CTA 键盘可达契约', () => {
  // ============ 10 缺口 it：三件套齐全（RED 应全失败）============
  for (const c of TEN) {
    it('缺口 ' + c.rel + ' ' + c.cls + ' ' + c.handler + ' 三件套齐全', () => {
      const line = ctaLine(c)
      expect(line, c.rel + ' 须 role=button').toContain('role="button"')
      expect(line, c.rel + ' 须 tabindex=0').toContain('tabindex="0"')
      expect(line, c.rel + ' 须 @keydown.enter 复用同 handler').toContain('@keydown.enter="' + c.handler + '"')
    })
  }

  // ============ 不变量 it（RED 应全通过）============
  it('不变量1：10 处 @click handler 签名未变（自指防护与导航参数不动）', () => {
    for (const c of TEN) {
      expect(ctaLine(c), c.rel + ' @click 未改').toContain('@click="' + c.handler + '"')
    }
  })

  it('不变量2：r12Cta 自指防护逻辑仍在（R48）', () => {
    for (const rel of ['pages/bag/bag.vue', 'pages/equipment/equipment.vue', 'pages/gems/gems.vue', 'pages/skills/skills.vue']) {
      expect(read(rel), rel + ' 自指防护未动').toContain('if (r12Cta.value.self)')
    }
  })

  it('不变量3：同类 text 版 r3-ai-cta 三件套仍在（9 页）', () => {
    const pages = [
      'pages/announcements/announcements.vue', 'pages/equipment/equipment.vue',
      'pages/gems/gems.vue', 'pages/mastery/mastery.vue', 'pages/reactions/reactions.vue',
      'pages/shop/shop.vue', 'pages/signin/signin.vue', 'pages/skills/skills.vue',
      'pages/stage/stage.vue',
    ]
    let n = 0
    for (const rel of pages) {
      const lines = read(rel).split(NL).filter((l) => l.includes('<text class="r3-ai-cta"'))
      expect(lines.length, rel + ' 应有 text 版 r3-ai-cta').toBeGreaterThan(0)
      for (const l of lines) {
        expect(l, rel + ' text 版须 role=button').toContain('role="button"')
        expect(l, rel + ' text 版须 tabindex=0').toContain('tabindex="0"')
        expect(l, rel + ' text 版须 @keydown.enter').toContain('@keydown.enter="')
      }
      n += lines.length
    }
    expect(n, 'text 版 r3-ai-cta 总数应 >=9').toBeGreaterThanOrEqual(9)
  })

  it('不变量4：令牌三锚点未漂移 + 10 页零 --r109-', () => {
    const t = read('styles/tokens.css')
    expect(t).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(t).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(t).toContain('--lyp-r6-version: lyp-tokens-r6')
    for (const c of TEN) expect(read(c.rel), c.rel + ' 零 --r109-').not.toContain('--r109-')
  })

  it('不变量5：R60/R96/R99/R100/R102/R104 既有三件套抽查仍在', () => {
    const cases = [
      { rel: 'pages/index/index.vue', anchor: '@click="retry" @keydown.enter="retry"' },
      { rel: 'pages/index/index.vue', anchor: '@click="go(aiInsight.hint)" @keydown.enter="go(aiInsight.hint)"' },
      { rel: 'pages/me/me.vue', anchor: '@click="goBack" @keydown.enter="goBack"' },
      { rel: 'pages/bag/bag.vue', anchor: 'class="r3-empty-action" role="button"' },
      { rel: 'pages/enemies/enemies.vue', anchor: '@keydown.enter="toggleEnemy(en.id)"' },
      { rel: 'pages/announcements/announcements.vue', anchor: 'aria-busy="true"' },
    ]
    for (const c of cases) expect(read(c.rel), c.rel + ' ' + c.anchor).toContain(c.anchor)
  })

  it('不变量6：10 个目标 vue 均可定位且无解析歧义', () => {
    for (const c of TEN) {
      const src = read(c.rel)
      expect(src, c.rel + ' 非空').toContain('<template>')
      ctaLine(c)
    }
  })
})
