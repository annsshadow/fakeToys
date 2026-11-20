// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R235 搜索/筛选输入框可访问名称覆盖缺口契约测试
 * （WCAG 4.1.2 Name/Role/Value、3.3.2 Labels or Instructions）。
 *
 * R217 的全局增强器最初只匹配 input.search-input / input.search-input-lg，
 * 但桌面端另有若干真正用于“搜索/筛选”的输入框使用了其它 class
 * （.search / .sb-input / .qs-input / .result-filter / .list-input / .fi2 / .ra-filter）。
 * 它们同样是 placeholder-only：用户键入首字符后 placeholder 消失，读屏器只读到“编辑文本”，
 * 违反 WCAG 4.1.2 / 3.3.2。本契约确认真实视图源码里的缺口，并要求增强器覆盖这些类。
 *
 * 注意：数据录入类 .tmp-input（如“备注内容...”）语义不同，不在此覆盖范围内，
 * 以免把值提示误当作可访问名称。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installSearchFieldA11y } from './searchFieldA11y'

const flush = () => new Promise<void>((resolve) => setTimeout(resolve, 0))

function searchLikeInput(placeholder: string, cls: string): HTMLInputElement {
  const el = document.createElement('input')
  el.className = cls
  el.placeholder = placeholder
  document.body.appendChild(el)
  return el
}

/** 取含指定 placeholder 的 <input> 起始标签全文，检查真实视图源码里的名称缺失。 */
function tagWithPlaceholder(source: string, placeholder: string): string {
  const idx = source.indexOf(`placeholder="${placeholder}"`)
  expect(idx, `未找到 placeholder=${placeholder}`).toBeGreaterThan(-1)
  const start = source.lastIndexOf('<input', idx)
  const end = source.indexOf('>', idx)
  return source.slice(start, end + 1)
}

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../../..')
function source(rel: string): string {
  return readFileSync(path.join(frontendRoot, rel), 'utf8')
}

let dispose: () => void = () => {}

beforeEach(() => {
  document.body.innerHTML = ''
})

afterEach(() => {
  dispose()
  dispose = () => {}
  document.body.innerHTML = ''
})

describe('R235 搜索/筛选框可访问名称覆盖（WCAG 4.1.2、3.3.2）', () => {
  const cases = [
    { cls: 'search', placeholder: '搜索...', file: 'apps/desktop/src/components/ScriptWorkbench.vue' },
    { cls: 'sb-input', placeholder: '搜索配置...', file: 'apps/desktop/src/views/ConfigDesignerApp.vue' },
    { cls: 'qs-input', placeholder: '搜索查询...', file: 'apps/desktop/src/views/QueryManagerDeep.vue' },
    { cls: 'result-filter', placeholder: '筛选结果...', file: 'apps/desktop/src/views/QueryStatementDesigner.vue' },
    { cls: 'list-input', placeholder: '搜索...', file: 'apps/desktop/src/views/FormDesigner.vue' },
    { cls: 'fi2', placeholder: '输入筛选条件...', file: 'apps/desktop/src/views/QueryManager.vue' },
  ]

  for (const { cls, placeholder, file } of cases) {
    it(`真实视图 ${file} 的 .${cls} 搜索框是 placeholder-only，增强后应被命名`, async () => {
      const tag = tagWithPlaceholder(source(file), placeholder)
      expect(tag, `标签应带目标 class=${cls}`).toContain(cls)
      expect(tag, '源码缺口：不应已有 aria-label').not.toContain('aria-label')
      expect(tag, '源码缺口：不应已有 aria-labelledby').not.toContain('aria-labelledby')
      expect(tag, '源码缺口：不应已有 title').not.toContain('title=')

      const el = searchLikeInput(placeholder, cls)
      expect(el.hasAttribute('aria-label')).toBe(false)
      dispose = installSearchFieldA11y(document.body)
      await flush()
      expect(el.getAttribute('aria-label')).toBe(placeholder)
    })
  }

  it('QueryManagerDeep.vue 的 .ra-filter 快速筛选框（多行模板）也应被命名', async () => {
    const s = source('apps/desktop/src/views/QueryManagerDeep.vue')
    expect(s).toContain('class="ra-filter"')
    expect(s).toContain('placeholder="快速筛选…"')

    const el = searchLikeInput('快速筛选…', 'ra-filter')
    dispose = installSearchFieldA11y(document.body)
    await flush()
    expect(el.getAttribute('aria-label')).toBe('快速筛选…')
  })

  it('非搜索类 .tmp-input 数据录入框不应被误命名（避免把值提示当名称）', async () => {
    const el = searchLikeInput('备注内容...', 'tmp-input')
    dispose = installSearchFieldA11y(document.body)
    await flush()
    expect(el.hasAttribute('aria-label')).toBe(false)
  })
})
