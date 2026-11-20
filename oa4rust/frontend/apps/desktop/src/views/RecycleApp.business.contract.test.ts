// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

/** A 类页面业务化契约：回收站以概览/批量操作/回收列表组织，操作是真实业务动作。 */
describe('RecycleApp 业务视图契约', () => {
  const source = readFileSync(fileURLToPath(new URL('./RecycleApp.vue', import.meta.url)), 'utf8')
  const template = source.slice(source.indexOf('<template>'), source.indexOf('</template>'))

  it('默认视图提供回收概览/批量操作/回收列表业务区块', () => {
    expect(template).toContain('回收站')
    expect(template).toContain('回收概览')
    expect(template).toContain('批量操作')
    expect(template).toContain('回收项目')
    for (const id of ['recycle-header', 'recycle-stats', 'recycle-toolbar', 'recycle-list']) {
      expect(template, `缺少业务区块 data-testid: ${id}`).toContain(`data-testid="${id}"`)
    }
  })

  it('副标题描述业务语义，不暴露 API 路径', () => {
    expect(template).toContain('已删除项目的恢复与永久清理')
    expect(template).not.toContain('/api/')
  })

  it('回收列表有表头、行项目与恢复/永久删除操作', () => {
    expect(template).toContain('table-header')
    expect(template).toMatch(/v-for="item in items"/)
    expect(template).toContain('col-name')
    expect(template).toContain('col-date')
    expect(template).toContain('>恢复</button>')
    expect(template).toContain('>永久删除</button>')
  })

  it('批量操作区有刷新与清空回收站按钮，无端点调试入口', () => {
    const bar = template.slice(template.indexOf('recycle-toolbar'))
    expect(bar).toContain('刷新列表')
    expect(bar).toContain('清空回收站')
    expect(template).not.toContain('loadRecycleTwin')
    expect(template).not.toContain('孪生端点')
  })
})
