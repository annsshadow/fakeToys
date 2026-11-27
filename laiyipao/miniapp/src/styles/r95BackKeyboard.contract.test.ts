// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R95 我的页返回按钮键盘可达契约（test-first RED）
 * 真实读 laiyipao/miniapp/src/pages/me/me.vue 等源码，禁止内嵌实现副本。
 * 缺口：me.vue:11 的「‹ 返回」只有 @click，无 role/tabindex/@keydown.enter，
 * a11y 树暴露 StaticText、Tab 不可达（R60 同维度遗漏，WCAG 2.1.1）。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync, readdirSync, statSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join, relative } from 'node:path'

const dir = dirname(fileURLToPath(import.meta.url))
const srcRoot = join(dir, '..')
const NL = String.fromCharCode(10)

function read(rel: string): string {
  return readFileSync(join(srcRoot, rel), 'utf8')
}

// 唯一定位 me 页返回按钮模板行：back-btn + @click="goBack" + 「‹ 返回」
function backButtonLine(): string {
  const lines = read('pages/me/me.vue').split(NL)
  const hits = lines.filter((l) => l.includes('class="back-btn') && l.includes('@click="goBack"') && l.includes('‹ 返回'))
  expect(hits.length, 'me.vue 应恰有一条 back-btn 返回模板行').toBe(1)
  return hits[0]
}

function walk(root: string): string[] {
  const out: string[] = []
  for (const name of readdirSync(root)) {
    const p = join(root, name)
    if (statSync(p).isDirectory()) out.push(...walk(p))
    else if (/\.(vue|ts|css)$/.test(name) && !name.includes('.test.')) out.push(p)
  }
  return out
}

describe('R95 我的页返回按钮键盘可达契约', () => {
  it('it1 back-btn 模板行含 role="button"', () => {
    expect(backButtonLine(), '返回按钮须声明 role=button 语义').toContain('role="button"')
  })

  it('it2 back-btn 模板行含 tabindex="0"', () => {
    expect(backButtonLine(), '返回按钮须进入 Tab 序列（WCAG 2.1.1）').toContain('tabindex="0"')
  })

  it('it3 back-btn 模板行含 @keydown.enter="goBack"', () => {
    expect(backButtonLine(), '返回按钮须可被 Enter 键盘激活').toContain('@keydown.enter="goBack"')
  })

  it('it4 canGoBack 与 goBack 不变量保持', () => {
    const me = read('pages/me/me.vue')
    expect(me, 'canGoBack 声明须保留').toContain('const canGoBack =')
    expect(me, 'canGoBack 须保持栈长>1 判定').toContain('getCurrentPages().length > 1')
    expect(me, 'goBack 声明须保留').toContain('function goBack(): void {')
    expect(me, 'goBack 须保持 navigateBack delta:1').toContain('uni.navigateBack({ delta: 1 })')
  })

  it('it5 R60 既有键盘 CTA 范式不回退（抽查≥2处）', () => {
    const spots = ['pages/announcements/announcements.vue', 'pages/equipment/equipment.vue']
    let ok = 0
    for (const rel of spots) {
      const hit = read(rel).split(NL).some((l) =>
        l.includes('role="button"') && l.includes('tabindex="0"') && l.includes('@keydown.enter'))
      if (hit) ok++
    }
    expect(ok, 'R60 role=button + tabindex=0 + @keydown.enter 范式须至少 2 处仍在').toBeGreaterThanOrEqual(2)
  })

  it('it6 令牌三锚点不变且业务源码零 --r95-', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens, 'v1 锚点').toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens, 'r3 锚点').toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens, 'r6 锚点').toContain('--lyp-r6-version: lyp-tokens-r6')
    const bad: string[] = []
    for (const f of walk(srcRoot)) {
      if (readFileSync(f, 'utf8').includes('--r95-')) bad.push(relative(srcRoot, f))
    }
    expect(bad, '业务源码不得出现 --r95-* 令牌: ' + bad.join(', ')).toEqual([])
  })
})
