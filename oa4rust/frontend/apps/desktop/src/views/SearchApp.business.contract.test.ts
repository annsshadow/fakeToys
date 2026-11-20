// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

/** A 类页面业务化契约：全局搜索以搜索框 + 结果分类 tab 组织。 */
describe('SearchApp 业务视图契约', () => {
  const source = readFileSync(fileURLToPath(new URL('./SearchApp.vue', import.meta.url)), 'utf8')
  const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))

  it('默认视图提供搜索面板、搜索表单与结果区业务区块', () => {
    expect(template).toContain('全局搜索')
    expect(template).toContain('跨文档、流程与组织的统一检索')
    for (const id of ['search-panel', 'search-form', 'search-results', 'search-results-tabs']) {
      expect(template, `缺少业务区块 data-testid: ${id}`).toContain(`data-testid="${id}"`)
    }
  })

  it('搜索框是带提交按钮的真实表单', () => {
    const form = template.slice(template.indexOf('<form'))
    expect(form).toContain('<form')
    expect(form).toContain('type="search"')
    expect(form).toMatch(/<button[^>]*type="submit"/)
    expect(form).toContain('v-model="query"')
  })

  it('结果区按全部/文档/流程/组织分类呈现 tab', () => {
    expect(template).toContain('全部结果')
    expect(template).toContain('文档')
    expect(template).toContain('流程')
    expect(template).toContain('组织')
    expect(template).toMatch(/role="tablist"/)
    expect(template).toMatch(/v-for="r in results"/)
  })

  it('默认视图不渲染 /api/* 端点调试按钮，副标题不暴露 API 路径', () => {
    expect(template).not.toContain('/api/')
    expect(template).not.toContain('loadSearchTwin')
    expect(template).not.toContain('孪生端点')
  })
})
