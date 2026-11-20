// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R305 P0 视觉深化契约：组织架构/考勤/BAM/论坛四页从「双重未消费」升级为 AI 令牌深度消费。
 * DOM aiPage ≥2（class 含 ai-page 子串，对齐 C1 审计口径）、源码 var(--ai-*) ≥5/页；
 * 玻璃面板、品牌渐变、8px 栅格、令牌骨架屏、空态引导均须落地；业务 testid/h1/接口保持不变。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string) => readFileSync(resolve(import.meta.dirname, name), 'utf8')

// 配对取根 <template>（页内含具名 <template #slot>，须按层级配对）
const templateOf = (src: string): string => {
  const start = src.indexOf('<template>')
  if (start < 0) return ''
  let depth = 0
  const tag = /<template\b|<\/template>/g
  tag.lastIndex = start
  let m: RegExpExecArray | null
  while ((m = tag.exec(src))) {
    depth += m[0] === '</template>' ? -1 : 1
    if (depth === 0) return src.slice(start, m.index + m[0].length)
  }
  return src.slice(start)
}

const pages = [
  ['组织架构', read('OrgViewer.vue')],
  ['考勤', read('AttendanceApp.vue')],
  ['BAM', read('BamApp.vue')],
  ['论坛', read('BBSForum.vue')],
] as const

const aiVars = (s: string) => s.match(/var\(--ai-[a-z0-9-]+\)/g) ?? []
// 对齐 C1 审计口径 [class*="ai-page"]：统计 class 属性值含 ai-page 子串的元素数
const aiPageClasses = (tpl: string) => tpl.match(/class="[^"]*ai-page[^"]*"/g) ?? []

describe('R305 P0 视觉深化契约', () => {
  it('四页源码各消费至少 5 处 var(--ai-*) 令牌', () => {
    for (const [label, src] of pages) {
      expect(aiVars(src).length, label + ' 的 var(--ai-*) 少于 5').toBeGreaterThanOrEqual(5)
    }
  })

  it('四页 DOM 各有至少 2 个 ai-page 容器（真浏览器 class 含 ai-page 口径）', () => {
    for (const [label, src] of pages) {
      expect(aiPageClasses(templateOf(src)).length, label + ' 的 ai-page 容器少于 2').toBeGreaterThanOrEqual(2)
    }
  })

  it('四页落地玻璃面板 + 品牌渐变 + 8px 栅格 + 令牌骨架屏', () => {
    for (const [label, src] of pages) {
      const tpl = templateOf(src)
      expect(tpl, label + ' 缺 ai-panel 玻璃面板').toMatch(/class="[^"]*\bai-panel\b[^"]*"/)
      expect(src, label + ' 缺品牌渐变令牌').toContain('var(--ai-gradient-brand)')
      expect(src, label + ' 缺 8px 栅格令牌').toMatch(/var\(--ai-grid-[1-8]\)/)
      expect(src, label + ' 缺骨架令牌').toMatch(/var\(--ai-skeleton-(?:base|shine)\)/)
    }
  })

  it('四页空态含可读引导文案，且保留单一 h1 与业务根 testid', () => {
    expect(templateOf(read('OrgViewer.vue'))).toContain('暂无组织架构')
    expect(templateOf(read('AttendanceApp.vue'))).toContain('暂无考勤记录')
    expect(templateOf(read('BamApp.vue'))).toContain('暂无活动记录')
    expect(templateOf(read('BBSForum.vue'))).toContain('暂无主题')
    const roots = [
      ['org', read('OrgViewer.vue'), 'org-page'],
      ['attendance', read('AttendanceApp.vue'), 'attendance-page'],
      ['bam', read('BamApp.vue'), 'bam-page'],
      ['bbs', read('BBSForum.vue'), 'bbs-root'],
] as const
    for (const [label, src, testid] of roots) {
      const tpl = templateOf(src)
      expect((tpl.match(/<h1\b/g) ?? []).length, label + ' h1 应唯一').toBeLessThanOrEqual(1)
      expect(tpl, label + ' 缺业务 testid ' + testid).toContain('data-testid="' + testid + '"')
    }
  })
})