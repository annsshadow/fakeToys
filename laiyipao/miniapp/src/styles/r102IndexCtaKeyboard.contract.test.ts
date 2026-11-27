// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R102-B 首页 CTA 键盘可达性契约（test-first RED）。
 * 真实 fs.readFileSync 读 index.vue / tokens.css，禁止内嵌实现副本。
 * 5 处裸 view/text CTA 缺 role="button" + tabindex="0" + @keydown.enter 同 handler。
 * 不变量：R99 go(d.hint)、R100/R101 ann-item 三件套不回退；令牌三锚点；零 --r102-。
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

// 按稳定锚点（唯一属性串/文本第 nth 次出现）向前找 <tag、向后找最近的 >，跨行开标签容错；不依赖行号
function tagBefore(src: string, needle: string, tag: string, nth = 0): string | null {
  let from = 0
  let i = -1
  for (let n = 0; n <= nth; n++) {
    i = src.indexOf(needle, from)
    if (i < 0) return null
    from = i + needle.length
  }
  const start = src.lastIndexOf(tag, i)
  if (start < 0 || i - start > 600) return null
  const gt = src.indexOf('>', i)
  return gt < 0 ? null : src.slice(start, gt + 1)
}

const indexSrc = read('pages/index/index.vue')
const tokensSrc = read('styles/tokens.css')

type Case = { name: string; needle: string; tag: string; nth: number; handler: string }
const CASES: Case[] = [
  { name: '错误态重试 retry', needle: '@click="retry"', tag: '<view', nth: 0, handler: 'retry' },
  { name: '新手引导卡 first-run-guide', needle: 'class="card first-run-guide tap"', tag: '<view', nth: 0, handler: 'goStage' },
  { name: 'AI 洞察去补强', needle: '@click="go(aiInsight.hint)"', tag: '<view', nth: 0, handler: 'go(aiInsight.hint)' },
  { name: 'AI 洞察去战斗', needle: '@click="goStage"', tag: '<view', nth: 1, handler: 'goStage' },
  { name: '公告全部 ann-all', needle: 'class="ann-all"', tag: '<text', nth: 0, handler: 'goAnnouncements' },
]

describe('R102-B 首页 CTA 键盘可达性契约', () => {
  for (const c of CASES) {
    it(c.name + ' 补键盘三件套且 @keydown.enter 复用 @click', () => {
      const b = tagBefore(indexSrc, c.needle, c.tag, c.nth)
      expect(b, '须能定位 ' + c.name + ' 开标签').not.toBeNull()
      const tag = b || ''
      expect(tag, c.name + ' 须有 role="button"').toContain('role="button"')
      expect(tag, c.name + ' 须有 tabindex="0"').toContain('tabindex="0"')
      expect(tag, c.name + ' 须保留 @click="' + c.handler + '"').toContain('@click="' + c.handler + '"')
      expect(tag, c.name + ' 须有 @keydown.enter="' + c.handler + '"').toContain('@keydown.enter="' + c.handler + '"')
    })
  }

  it('不变量：R99 空维补导 go(d.hint) 三件套不回退', () => {
    const b = tagBefore(indexSrc, '@click="go(d.hint)"', '<text')
    expect(b, '须能定位 go(d.hint) 开标签').not.toBeNull()
    expect(b || '').toContain('role="button"')
    expect(b || '').toContain('tabindex="0"')
    expect(b || '').toContain('@keydown.enter="go(d.hint)"')
  })

  it('不变量：R100/R101 ann-item 三件套与 aria-expanded 不回退', () => {
    const b = tagBefore(indexSrc, 'class="ann-item"', '<view')
    expect(b, '须能定位 .ann-item 开标签').not.toBeNull()
    expect(b || '').toContain('role="button"')
    expect(b || '').toContain('tabindex="0"')
    expect(b || '').toContain('@keydown.enter="toggleAnn(a.id)"')
    expect(b || '').toContain(':aria-expanded="String(expanded.has(a.id))"')
  })

  it('不变量：四个 @click handler 在 index.vue 保留', () => {
    for (const h of ['retry', 'goStage', 'go(aiInsight.hint)', 'goAnnouncements']) {
      expect(indexSrc, '须保留 @click="' + h + '"').toContain('@click="' + h + '"')
    }
  })

  it('不变量：令牌三锚点不动且业务源码零 --r102-', () => {
    expect(tokensSrc).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokensSrc).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokensSrc).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(indexSrc + tokensSrc, '不得新增 --r102- 视觉令牌').not.toContain('--r102-')
  })
})
