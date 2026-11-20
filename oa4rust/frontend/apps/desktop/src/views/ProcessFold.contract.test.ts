// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const dir = import.meta.dirname

function read(name: string): string {
  return readFileSync(resolve(dir, name), 'utf8')
}

function template(src: string): string {
  const start = src.indexOf('<template>')
  const script = src.indexOf('<script', start)
  return src.slice(start, script === -1 ? undefined : script)
}

function buttonCount(tmpl: string, cls: string): number {
  return (tmpl.match(new RegExp(`class="${cls}"`, 'g')) || []).length
}

describe('R301 流程/业务/笔记页端点折叠契约', () => {
  it('ProcessManagerApp 使用 EndpointFold 三分组且保留全部 112 个调试按钮', () => {
    const src = read('ProcessManagerApp.vue')
    const tmpl = template(src)
    expect(tmpl, 'PM 未引用 EndpointFold').toContain('<EndpointFold')
    expect(tmpl, 'PM 缺少 read/write/config 三个插槽').toMatch(/#group-read[\s\S]*#group-write[\s\S]*#group-config/)
    expect(buttonCount(tmpl, 'btn-refresh'), 'PM 调试按钮数量变化').toBe(112)
    expect(tmpl, 'PM 仍暴露 /api 端点副标题').not.toMatch(/subtitle="\/api\//)
  })

  it('ProcessTaskCenterApp 使用 EndpointFold 分组、保留全部 21 个调试按钮，且列表标题走 displayTitle', () => {
    const src = read('ProcessTaskCenterApp.vue')
    const tmpl = template(src)
    expect(tmpl, 'PTC 未引用 EndpointFold').toContain('<EndpointFold')
    expect(tmpl, 'PTC 缺少分组插槽').toMatch(/#group-read[\s\S]*#group-write/)
    expect(buttonCount(tmpl, 'btn-refresh'), 'PTC 调试按钮数量变化').toBe(21)
    expect(tmpl, 'PTC 仍暴露 /api 端点副标题').not.toMatch(/subtitle="\/api\//)
    expect(tmpl, '任务列表标题未调用 displayTitle').toContain('{{ displayTitle(item) }}')
    expect(src, '缺少 displayTitle 实现').toMatch(/function displayTitle\(item: Item\)/)
    expect(src, 'displayTitle 未处理 start: 原始工作 id').toMatch(/\^start:/)
    expect(src, 'displayTitle 未处理 work- 原始工作 id')
      .toMatch(/startsWith\('work-'\)|工作实例|\^work-/)
  })

  it('BamApp 使用 EndpointFold 分组、保留全部 25 个 API 调试按钮，且删除 /api 副标题', () => {
    const src = read('BamApp.vue')
    const tmpl = template(src)
    expect(tmpl, 'BAM 未引用 EndpointFold').toContain('<EndpointFold')
    expect(tmpl, 'BAM 缺少分组插槽').toMatch(/#group-read[\s\S]*#group-write/)
    expect(buttonCount(tmpl, 'btn-primary ghost'), 'BAM 调试按钮数量变化').toBe(25)
    expect(tmpl, 'BAM 仍暴露 /api 端点副标题').not.toMatch(/class="subtitle">\/api\//)
  })

  it('NoteApp 使用 EndpointFold 收纳刷新按钮，且删除 /api 副标题', () => {
    const src = read('NoteApp.vue')
    const tmpl = template(src)
    expect(tmpl, 'Note 未引用 EndpointFold').toContain('<EndpointFold')
    expect(buttonCount(tmpl, 'btn-refresh'), 'Note 刷新按钮应保留 1 个').toBe(1)
    expect(tmpl, 'Note 仍暴露 /api 端点副标题').not.toMatch(/class="subtitle">\/api\//)
  })
})
