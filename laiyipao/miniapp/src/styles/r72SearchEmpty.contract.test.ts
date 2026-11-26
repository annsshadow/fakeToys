// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R72 搜索空态极端值溢出修复契约（test-first RED）。
 * 7 页搜索空态关键词回显文本须共用 search-empty-text 类（块级 + 宽度约束 + 溢出处理），
 * 消除极端长串下的整页横向溢出。先 RED 再实现；令牌锚点 v1/v3-r3/r6 不变，禁新增 r72 视觉令牌。
 * 本契约只读展示层样式与页面源码，不触碰受保护域。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

function blockAfter(text: string, keyword: string): { start: number; end: number; text: string } | null {
  const k = text.indexOf(keyword)
  if (k < 0) return null
  const open = text.indexOf('{', k)
  if (open < 0) return null
  let depth = 0
  for (let i = open; i < text.length; i++) {
    const ch = text[i]
    if (ch === '{') depth++
    else if (ch === '}') {
      depth--
      if (depth === 0) return { start: open, end: i, text: text.slice(open, i + 1) }
    }
  }
  return null
}

interface PageSpec { name: string; path: string; token: string }
const PAGES: PageSpec[] = [
  { name: 'skills', path: 'pages/skills/skills.vue', token: '{{ keyword }}' },
  { name: 'skins', path: 'pages/skins/skins.vue', token: '{{ keyword }}' },
  { name: 'announcements', path: 'pages/announcements/announcements.vue', token: '{{ annKeyword }}' },
  { name: 'enemies', path: 'pages/enemies/enemies.vue', token: '{{ keyword }}' },
  { name: 'equipment', path: 'pages/equipment/equipment.vue', token: '{{ keyword }}' },
  { name: 'gems', path: 'pages/gems/gems.vue', token: '{{ keyword }}' },
  { name: 'reactions', path: 'pages/reactions/reactions.vue', token: '{{ keyword }}' },
]

// 定位承载关键词回显的 text 开标签所在行；announcements 的 class 与关键词分行，按 v-if 定位开标签行。
function echoLine(src: string, page: PageSpec): string | undefined {
  const lines = src.split(String.fromCharCode(10))
  if (page.name === 'announcements') {
    return lines.find(l => l.indexOf('annKeyword && filteredItems.length === 0') > -1)
  }
  return lines.find(l => l.indexOf('没有匹配') > -1 && l.indexOf(page.token) > -1)
}

describe('R72 搜索空态极端值溢出修复契约', () => {
  const commonCss = readSrc('styles/common.css')
  const tokens = readSrc('styles/tokens.css')

  it('common.css 新增专用类：块级 + 宽度约束 + 溢出处理', () => {
    const block = blockAfter(commonCss, '.search-empty-text')
    expect(block, 'common.css 必须新增 search-empty-text 规则块').not.toBeNull()
    const b = block!.text
    expect(b, '必须块级显示').toMatch(/display:\s*block/)
    expect(b, '必须限宽 100%').toMatch(/max-width:\s*100%/)
    expect(b, '必须有溢出处理（word-break 或 ellipsis）').toMatch(/word-break:\s*break-all|text-overflow:\s*ellipsis/)
  })

  it('7 页空态关键词回显 text 均追加 search-empty-text', () => {
    for (const page of PAGES) {
      const src = readSrc(page.path)
      const line = echoLine(src, page)
      expect(line, page.name + ' 必须存在空态关键词回显行').toBeTruthy()
      expect(line, page.name + ' 关键词回显 text 必须追加 search-empty-text').toContain('search-empty-text')
    }
  })

  it('7 处搜索空态结构保留：条件 + 清除搜索 CTA + 关键词回显', () => {
    for (const page of PAGES) {
      const src = readSrc(page.path)
      expect(src, page.name + ' 必须保留关键词回显').toContain('没有匹配')
      expect(src, page.name + ' 必须保留关键词变量').toContain(page.token)
      expect(src, page.name + ' 必须保留清除搜索 CTA').toContain('清除搜索')
      if (page.name === 'announcements') {
        expect(src, 'announcements 必须保留空态条件').toContain('annKeyword && filteredItems.length === 0')
      } else {
        expect(src, page.name + ' 必须保留 search-empty 容器').toContain('class="search-empty"')
      }
    }
  })

  it('不变量原样：muted / search-empty / ellipsis 权威范式 + 令牌三锚点', () => {
    expect(commonCss, 'muted 权威范式不得改').toMatch(/\.muted\s*\{[^}]*color:\s*var\(--muted\)/)
    expect(commonCss, 'search-empty 权威范式不得改').toMatch(/\.search-empty\s*\{[^}]*flex-direction:\s*column/)
    expect(commonCss, 'ellipsis 权威范式不得改').toMatch(/\.ellipsis\s*\{[^}]*text-overflow:\s*ellipsis[^}]*white-space:\s*nowrap/)
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
  })

  it('业务源码零 r72 视觉令牌；写域边界：仅读展示层白名单', () => {
    const allSrc = PAGES.map(p => readSrc(p.path)).join(String.fromCharCode(10)) + String.fromCharCode(10) + commonCss + tokens
    expect(allSrc, '不得新增 r72 视觉令牌').not.toContain('--r72-')
    const self = readFileSync(__filename, 'utf-8')
    const allowed = new Set(['styles/common.css', 'styles/tokens.css'].concat(PAGES.map(p => p.path)))
    const calls = self.match(/readSrc\('[^']+'\)/g) || []
    for (const call of calls) {
      const m = call.match(/readSrc\('([^']+)'\)/)
      if (m) expect(allowed.has(m[1]), '写域越界: ' + m[1]).toBe(true)
    }
  })
})
