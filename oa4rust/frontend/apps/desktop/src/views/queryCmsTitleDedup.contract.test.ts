// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R300 查询/CMS/设计器页契约：
 * 路由标题由 AdminLayout 唯一渲染；页面不再保留自带 h1 与 /api/* 副标题，
 * 原操作按钮、tab、事件与 API 调用必须原样保留。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const viewsDir = resolve(import.meta.dirname)

const legacyTitlePages = [
  'QueryManager.vue',
  'QueryDesigner.vue',
  'ConfigDesignerApp.vue',
  'FindDesignerApp.vue',
  'ServiceInvokeDesignerApp.vue',
  'PdfViewerApp.vue',
  'ViewApp.vue',
  'FileInfoApp.vue',
  'LogViewerApp.vue',
]

const wrappedPages = [
  'QueryViewApp.vue',
  'QueryViewDesignerApp.vue',
  'QueryQueryApp.vue',
  'QueryExplorerApp.vue',
  'QueryStatDesignerApp.vue',
  'QueryImporterDesignerApp.vue',
  'FormDesigner.vue',
  'QueryStatementDesigner.vue',
  'CmsIndexApp.vue',
]

const read = (name: string): string => readFileSync(resolve(viewsDir, name), 'utf8')

function templateOf(name: string): string {
  const src = read(name)
  const end = src.indexOf('<script')
  return end === -1 ? src : src.slice(0, end)
}

describe('R300 查询/CMS/设计器页标题去重契约', () => {
  it.each(legacyTitlePages)('%s 删除页面自带 title h1 与 /api/* 副标题', (name) => {
    expect(templateOf(name)).not.toMatch(/<h1[\s>]/)
  })

  it.each(wrappedPages)('%s 不再保留旧 title h1 与 /api/* 副标题，仅保留统一页头', (name) => {
    const template = templateOf(name)
    expect(template).not.toMatch(/<h1[\s>]/)
    expect(template).toContain('<AdminPageHeader')
    expect(template).toMatch(/<AdminPageHeader[^>]*\/>|<\/AdminPageHeader>/)
  })

  it('全部目标页保留原有操作按钮、tab 与事件处理', () => {
    const buttonTotals = new Map<string, number>()
    for (const name of [...legacyTitlePages, ...wrappedPages]) {
      const template = templateOf(name)
      expect(template, `${name} 应保留按钮`).toMatch(/<button\b/)
      expect(template, `${name} 应保留事件绑定`).toMatch(/@click=/)
      buttonTotals.set(name, (template.match(/<button\b/g) ?? []).length)
    }
    expect(buttonTotals.get('QueryStatementDesigner.vue')).toBeGreaterThanOrEqual(25)
    expect(buttonTotals.get('CmsIndexApp.vue')).toBeGreaterThanOrEqual(60)
    expect(buttonTotals.get('QueryViewApp.vue')).toBeGreaterThanOrEqual(60)
  })

  it('全部目标页保留原有 API 调用', () => {
    for (const [name, needle] of [
      ['QueryManager.vue', 'api/query/assemble/designer'],
      ['QueryViewApp.vue', 'api/queryview'],
      ['QueryDesigner.vue', 'api/query/assemble/designer'],
      ['CmsIndexApp.vue', 'api/cms/core/entity/index'],
      ['ConfigDesignerApp.vue', 'api/config'],
      ['FindDesignerApp.vue', 'api/query/assemble/designer/list/all'],
      ['ServiceInvokeDesignerApp.vue', 'api/program_center/invoke'],
      ['PdfViewerApp.vue', 'api/'],
      ['ViewApp.vue', 'api/view'],
      ['FileInfoApp.vue', 'api/fileinfo'],
      ['LogViewerApp.vue', 'api/log/list'],
    ] as const) {
      expect(read(name), `${name} 应保留 API 端点`).toContain(needle)
    }
  })
})
