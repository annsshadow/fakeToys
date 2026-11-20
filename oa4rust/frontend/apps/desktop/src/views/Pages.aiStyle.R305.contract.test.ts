// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R305 P0 视觉深化契约（按 t50 实测基线校正两类语义）：
 * - 口径差页 org/bam/bbs：源码已有 var(--ai-*)，重点补 ai-page 容器 +
 *   已有令牌下沉 .ai-page 面板/卡片/骨架；attendance 才是双未消费全套落地。
 * - 共同验收：DOM aiPage ≥2、源码 var(--ai-*) ≥5/页、单一 h1、业务 testid 不变。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string) => readFileSync(resolve(import.meta.dirname, name), 'utf8')
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
const aiVars = (s: string) => s.match(/var\(--ai-[a-z0-9-]+\)/g) ?? []
const aiPageClasses = (tpl: string) => tpl.match(/class="[^"]*ai-page[^"]*"/g) ?? []

const org=read('OrgViewer.vue'),att=read('AttendanceApp.vue'),bam=read('BamApp.vue'),bbs=read('BBSForum.vue')
const pages = [['组织架构',org],['考勤',att],['BAM',bam],['论坛',bbs]] as const
// 口径差页：t50 基线 srcVar 4/13/9、DOM=0；改造重点是 ai-page 标记 + 令牌下沉
const markerPages = [['组织架构',org],['BAM',bam],['论坛',bbs]] as const

describe('R305 P0 视觉深化契约', () => {
  it('共同验收：四页源码各 ≥5 令牌且 DOM 各 ≥2 ai-page 容器', () => {
    for (const [label, src] of pages) {
      expect(aiVars(src).length, label+' srcVar<5').toBeGreaterThanOrEqual(5)
      expect(aiPageClasses(templateOf(src)).length, label+' aiPage<2').toBeGreaterThanOrEqual(2)
    }
  })

  it('口径差页 org/bam/bbs：ai-page 标记 + 已有令牌下沉 .ai-page 选择器（玻璃/渐变/栅格/骨架）', () => {
    for (const [label, src] of markerPages) {
      const tpl=templateOf(src)
      expect(tpl,label+' 缺 ai-panel').toMatch(/class="[^"]*\bai-panel\b[^"]*"/)
      expect(tpl,label+' 缺根 ai-page').toMatch(/class="[^"]*\bai-page\b[^"]*"/)
      expect(src,label+' 缺 .ai-page 样式下沉').toMatch(/\.ai-page [.#][a-z]/)
      expect(src,label+' 缺品牌渐变').toContain('var(--ai-gradient-brand)')
      expect(src,label+' 缺 8px 栅格').toMatch(/var\(--ai-grid-[1-8]\)/)
      expect(src,label+' 缺骨架令牌').toMatch(/var\(--ai-skeleton-(?:base|shine)\)/)
      expect(src,label+' 缺玻璃令牌').toMatch(/var\(--ai-glass-(?:bg|border|blur)\)/)
    }
  })

  it('双未消费页 attendance：ai-panel 玻璃面板 + 令牌 + 栅格 + 骨架 + 空态引导 + 真实刷新动作', () => {
    const tpl=templateOf(att)
    expect(tpl).toMatch(/class="[^"]*\bai-panel\b[^"]*"/)
    expect(tpl).toMatch(/class="[^"]*\bai-page\b[^"]*"/)
    expect(att).toMatch(/var\(--ai-glass-(?:bg|border|blur)\)/)
    expect(att).toContain('var(--ai-gradient-brand)')
    expect(att).toMatch(/var\(--ai-grid-[1-8]\)/)
    expect(att).toMatch(/var\(--ai-skeleton-(?:base|shine)\)/)
    expect(tpl).toContain('data-testid="attendance-empty-cta"')
    expect(tpl).toContain('暂无考勤记录，可刷新后查看最新数据')
    expect(tpl).toContain('@click="loadData"')
    expect(att).toContain('function loadData()')
  })

  it('四页保留单一 h1、业务根 testid 与可读空态（不触碰业务接口）', () => {
    expect(tplHas(org,'暂无组织架构')).toBe(true)
    expect(tplHas(bam,'暂无活动记录')).toBe(true)
    expect(tplHas(bbs,'暂无主题')).toBe(true)
    const roots = [['org',org,'org-page'],['attendance',att,'attendance-page'],['bam',bam,'bam-page'],['bbs',bbs,'bbs-root']] as const
    for (const [label,src,testid] of roots) {
      const tpl=templateOf(src)
      expect((tpl.match(/<h1\b/g)??[]).length,label+' h1 应唯一').toBeLessThanOrEqual(1)
      expect(tpl,label+' 缺 '+testid).toContain('data-testid="'+testid+'"')
    }
  })
})
function tplHas(src: string, text: string) { return templateOf(src).includes(text) }
