// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R300 B4 业务工作域契约：流程、文件、论坛、会议、BAM、笔记、回收站、任务中心
 * 必须消费 t1 的 --ai-* 令牌；保留业务 testid、接口调用与交互入口；
 * 同时清理审计中可见的原始 UUID 与端点调试侵入。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const views = resolve(import.meta.dirname, '.')
const read = (file: string) => readFileSync(resolve(views, file), 'utf8')
const template = (src: string) => {
  // 页面模板内部含具名 <template #slot>，需按层级配对取到根模板结尾。
  const start = src.indexOf('<template>')
  if (start < 0) return ''
  let depth = 0
  const tag = /<template\b|<\/template>/g
  tag.lastIndex = start
  let m: RegExpExecArray | null
  while ((m = tag.exec(src))) {
    depth += m[0] === '</template>' ? -1 : 1
    if (depth === 0) return src.slice(start, m!.index + m![0].length)
  }
  return src.slice(start)
}
const testids = (src: string): string[] => Array.from(src.matchAll(/data-testid="([^"]+)"/g), (m) => m[1])

const processWork = read('ProcessWork.vue')
const fileManager = read('FileManager.vue')
const bbs = read('BBSForum.vue')
const meeting = read('MeetingApp.vue')
const bam = read('BamApp.vue')
const note = read('NoteApp.vue')
const recycle = read('RecycleApp.vue')
const taskCenter = read('ProcessTaskCenterApp.vue')

const pages: Array<[string, string]> = [
  ['流程', processWork], ['文件', fileManager], ['论坛', bbs], ['会议', meeting],
  ['BAM', bam], ['笔记', note], ['回收站', recycle], ['任务中心', taskCenter],
]

describe('R300 B4 业务工作域 AI 风格契约', () => {
  it('八个业务页面都消费 --ai-* 令牌并保留单一 h1', () => {
    for (const [label, src] of pages) {
      expect(src, label + ' 未消费 --ai-* 令牌').toMatch(/var\(--ai-[a-z0-9-]+\)/)
      expect(src, label + ' 未消费品牌渐变令牌').toContain('var(--ai-gradient-brand)')
      expect(src, label + ' 未消费 spring 动效令牌').toContain('var(--ai-ease-spring)')
      const tpl = template(src)
      const h1 = tpl.match(/<h1\b/g) ?? []
      if (h1.length > 0) {
        expect(h1.length, label + ' h1 数量应为 1').toBe(1)
      } else {
        // 主标题可演进为带 data-testid 的 h2（如流程任务中心），仍需唯一可读标题。
        expect(tpl, label + ' 无 h1 时缺少带 data-testid 的 h2 主标题')
          .toMatch(/<h2\b[^>]*data-testid="[^"]*(?:title|h1)[^"]*"[^>]*>/i)
      }
    }
  })

  it('每个页面使用统一 modern 原语与 AI 面板底座', () => {
    for (const [label, src] of pages) {
      const tpl = template(src)
      expect(tpl, label + ' 缺少 modern-page').toMatch(/class="[^"]*\bmodern-page\b/)
      expect(tpl, label + ' 缺少 modern-hero').toMatch(/class="[^"]*\bmodern-hero\b/)
      expect(tpl, label + ' 缺少 ai-panel').toMatch(/class="[^"]*\bai-panel\b/)
      expect(tpl, label + ' 缺少 modern-grid/card').toMatch(/class="[^"]*\bmodern-(?:grid|card)\b/)
    }
  })

  it('保留全部关键业务 data-testid', () => {
    const ids = testids(template(processWork))
    for (const id of ['work-pending-list', 'work-started-list', 'work-completed-list']) {
      expect(ids, '流程缺少 ' + id).toContain(id)
    }
    const fileIds = testids(template(fileManager))
    for (const id of ['files-mine-list', 'files-shared-list', 'files-recycle-list']) {
      expect(fileIds, '文件缺少 ' + id).toContain(id)
    }
    const bbsIds = testids(template(bbs))
    for (const id of ['bbs-root', 'bbs-title', 'bbs-search-input', 'bbs-topic-list', 'bbs-compose-panel']) {
      expect(bbsIds, '论坛缺少 ' + id).toContain(id)
    }
    const recycleIds = testids(template(recycle))
    for (const id of ['recycle-business-view', 'recycle-stats', 'recycle-toolbar', 'recycle-list']) {
      expect(recycleIds, '回收站缺少 ' + id).toContain(id)
    }
  })

  it('原始 ID 不再是主视觉，详情按业务名称展示', () => {
    expect(template(processWork)).not.toContain('{{ item.id }}')
    expect(template(processWork)).not.toContain('{{ opened.id }}')
    expect(template(recycle)).not.toContain('{{ item.id?.slice(0,8) }}...')
    expect(template(note)).not.toContain('{{ item.personId')
    expect(template(bbs)).not.toContain('🆔 {{ sectionInfo.id }}')
    expect(template(fileManager)).not.toContain('{{ item.name || item.id }}')
    expect(template(taskCenter)).not.toContain('{{ item.work||')
  })

  it('业务页不直接暴露 /api/ 路径', () => {
    for (const [label, src] of pages) {
      expect(template(src), label + ' 模板含 /api/').not.toMatch(/\/api\//)
    }
  })
})
