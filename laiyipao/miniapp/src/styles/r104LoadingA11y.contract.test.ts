// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R104 骨架屏加载态 a11y 语义契约（test-first RED）
 * 真实 fs.readFileSync 读 laiyipao/miniapp/src 下 10 个页面源码，禁止内嵌实现。
 * 缺口：7 页 v-if="loading" 裸 view + 3 页 v-if="r6Loading" aria-hidden 骨架根
 *      均缺 role="status" / aria-busy="true" / aria-label="数据加载中"。
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

// 按唯一 class + v-if 锚点定位开标签：向前找最近 <tag，向后找最近 >，跨行容错，不依赖行号
function tagBefore(src: string, needle: string, tag: string): string | null {
  const i = src.indexOf(needle)
  if (i < 0) return null
  const start = src.lastIndexOf(tag, i)
  if (start < 0 || i - start > 600) return null
  const gt = src.indexOf('>', i)
  return gt < 0 ? null : src.slice(start, gt + 1)
}

const ATTRS = [
  { name: 'role="status"', text: 'role="status"' },
  { name: 'aria-busy="true"', text: 'aria-busy="true"' },
  { name: 'aria-label="数据加载中"', text: 'aria-label="数据加载中"' },
]

// A 类：7 页 v-if="loading" class="card" 文字/骨架裸 view 加载根
const LOADING = [
  { file: 'announcements/announcements.vue', anchor: 'v-if="loading" class="card"' },
  { file: 'bag/bag.vue', anchor: 'v-if="loading" class="card"' },
  { file: 'battles/battles.vue', anchor: 'v-if="loading" class="card"' },
  { file: 'mastery/mastery.vue', anchor: 'v-if="loading" class="card"' },
  { file: 'rank/rank.vue', anchor: 'v-if="loading" class="card"' },
  { file: 'shop/shop.vue', anchor: 'v-if="loading" class="card"' },
  { file: 'signin/signin.vue', anchor: 'v-if="loading" class="card"' },
]

// B 类：3 页 v-if="r6Loading" class="r6-skeleton r6-enter" 骨架根（另需去 aria-hidden）
const SKELETON = [
  { file: 'equipment/equipment.vue', anchor: 'v-if="r6Loading" class="r6-skeleton r6-enter"' },
  { file: 'reactions/reactions.vue', anchor: 'v-if="r6Loading" class="r6-skeleton r6-enter"' },
  { file: 'skills/skills.vue', anchor: 'v-if="r6Loading" class="r6-skeleton r6-enter"' },
]

describe('R104 骨架屏加载态 a11y 语义契约', () => {
  for (const c of LOADING) {
    it('A ' + c.file + ' 加载根补 status/busy/label 三属性', () => {
      const src = read('pages/' + c.file)
      const b = tagBefore(src, c.anchor, '<view')
      expect(b, c.file + ' 须能定位加载根开标签').not.toBeNull()
      for (const a of ATTRS) {
        expect(b || '', c.file + ' 加载根须含 ' + a.name).toContain(a.text)
      }
    })
  }

  for (const c of SKELETON) {
    it('B ' + c.file + ' 骨架根去 aria-hidden 并补 status/busy/label 三属性', () => {
      const src = read('pages/' + c.file)
      const b = tagBefore(src, c.anchor, '<view')
      expect(b, c.file + ' 须能定位骨架根开标签').not.toBeNull()
      expect(b || '', c.file + ' 骨架根须去掉 aria-hidden="true"').not.toContain('aria-hidden="true"')
      for (const a of ATTRS) {
        expect(b || '', c.file + ' 骨架根须含 ' + a.name).toContain(a.text)
      }
    })
  }

  it('不变量：令牌三锚点不动且 10 页业务源码零 --r104-', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(tokens).not.toContain('--r104-')
    for (const c of [...LOADING, ...SKELETON]) {
      const src = read('pages/' + c.file)
      expect(src, c.file + ' 零 --r104-').not.toContain('--r104-')
    }
  })

  it('不变量：R60/R100 键盘三件套抽查（announcements CTA + bag 空态 + enemies resist-toggle）仍在', () => {
    const CASES = [
      { file: 'announcements/announcements.vue', anchor: 'class="r3-ai-cta"', handler: 'r5GoBattles()' },
      { file: 'bag/bag.vue', anchor: 'class="r3-empty-action"', handler: 'r4GoBag()' },
      { file: 'enemies/enemies.vue', anchor: 'class="resist-toggle"', handler: 'toggleEnemy(en.id)' },
    ]
    for (const c of CASES) {
      const src = read('pages/' + c.file)
      const b = tagBefore(src, c.anchor, '<')
      expect(b, c.file + ' 键盘入口须能定位').not.toBeNull()
      expect(b || '', c.file + ' 须保留 role="button"').toContain('role="button"')
      expect(b || '', c.file + ' 须保留 tabindex="0"').toContain('tabindex="0"')
      expect(b || '', c.file + ' 须保留 @keydown.enter 同名 handler')
        .toContain('@keydown.enter="' + c.handler + '"')
    }
  })

  it('不变量：R101 aria-expanded 与既有空态 aria-live/role=status 不被误改', () => {
    const enemies = read('pages/enemies/enemies.vue')
    const battles = read('pages/battles/battles.vue')
    expect(enemies).toContain(':aria-expanded="String(expandedEnemy === en.id)"')
    expect(battles).toContain(':aria-expanded="String(expandedId === it.id)"')
    expect(read('pages/announcements/announcements.vue')).toContain('aria-live="polite"')
    expect(read('pages/bag/bag.vue')).toContain('role="status"')
  })
})
