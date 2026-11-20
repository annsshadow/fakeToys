// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment jsdom
/**
 * R217 搜索输入框可访问名称契约测试（WCAG 4.1.2 Name/Role/Value、3.3.2 Labels or Instructions）。
 *
 * 桌面端 40+ 个 CRUD 视图用 placeholder-only 的 <input class="search-input"> 做搜索：
 *   <input v-model="search" placeholder="搜索标题 / 分类..." class="search-input" />
 * placeholder 不是可访问名称——输入首个字符后即消失，多个读屏器不把它当 label，
 * 用户只听到「编辑文本」，无法区分搜索框用途。全局增强器应在作者未提供任何名称
 * （aria-label / aria-labelledby / title / 关联 label）时用 placeholder 补 aria-label；
 * 已有名称一律不动。
 */
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { afterEach, beforeEach, describe, expect, it } from 'vitest'
import { installSearchFieldA11y } from './searchFieldA11y'

const flush = () => new Promise<void>((resolve) => setTimeout(resolve, 0))

function searchInput(placeholder: string): HTMLInputElement {
  const el = document.createElement('input')
  el.className = 'search-input'
  el.placeholder = placeholder
  document.body.appendChild(el)
  return el
}

/** 取含指定 placeholder 的 <input> 起始标签全文，检查真实视图源码里的名称缺失。 */
function tagWithPlaceholder(source: string, placeholder: string): string {
  const idx = source.indexOf('placeholder="' + placeholder + '"')
  expect(idx, '未找到 placeholder=' + placeholder).toBeGreaterThan(-1)
  const start = source.lastIndexOf('<input', idx)
  const end = source.indexOf('>', idx)
  return source.slice(start, end + 1)
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

describe('搜索输入框可访问名称（R217 / WCAG 4.1.2、3.3.2）', () => {
  it('placeholder-only 搜索框安装后获得读屏可见的 aria-label', async () => {
    const el = searchInput('搜索标题 / 分类...')
    dispose = installSearchFieldA11y(document.body)
    await flush()

    expect(el.getAttribute('aria-label')).toBe('搜索标题 / 分类...')
  })

  it('作者已有的 aria-label 不被覆盖', async () => {
    const el = searchInput('搜索帖子...')
    el.setAttribute('aria-label', '论坛帖子搜索')
    dispose = installSearchFieldA11y(document.body)
    await flush()

    expect(el.getAttribute('aria-label')).toBe('论坛帖子搜索')
  })

  it('已有 aria-labelledby、title 或关联 label 的输入不被注入名称', async () => {
    const labelled = searchInput('A')
    labelled.setAttribute('aria-labelledby', 'lbl')
    const titled = searchInput('B')
    titled.title = '标题提示'
    const associated = document.createElement('input')
    associated.className = 'search-input'
    associated.placeholder = 'C'
    associated.id = 'sf-assoc'
    const lbl = document.createElement('label')
    lbl.textContent = '搜索项'
    lbl.htmlFor = 'sf-assoc'
    document.body.append(lbl, associated)

    dispose = installSearchFieldA11y(document.body)
    await flush()

    expect(labelled.hasAttribute('aria-label')).toBe(false)
    expect(titled.hasAttribute('aria-label')).toBe(false)
    expect(associated.hasAttribute('aria-label')).toBe(false)
  })

  it('Vue 异步插入的搜索框由观察器处理', async () => {
    dispose = installSearchFieldA11y(document.body)
    const el = searchInput('搜索文档...')
    await flush()

    expect(el.getAttribute('aria-label')).toBe('搜索文档...')
  })

  it('卸载后移除注入的 aria-label，且不再处理新插入的输入', async () => {
    const el = searchInput('搜索Agent...')
    dispose = installSearchFieldA11y(document.body)
    await flush()
    expect(el.getAttribute('aria-label')).toBeTruthy()

    dispose()
    dispose = () => {}
    expect(el.hasAttribute('aria-label')).toBe(false)

    const later = searchInput('搜索流程...')
    await flush()
    expect(later.hasAttribute('aria-label')).toBe(false)
  })

  it('真实视图的搜索框已有读屏名称，增强器保留作者命名', async () => {
    const here = path.dirname(fileURLToPath(import.meta.url))
    const frontendRoot = path.resolve(here, '../../../..')
    const bbs = readFileSync(path.join(frontendRoot, 'apps/desktop/src/views/BBSForum.vue'), 'utf8')

    // R297 BBSForum 业务化后，搜索框已自带 aria-label，不再依赖 placeholder-only 兜底。
    const tag = tagWithPlaceholder(bbs, '搜索帖子标题或内容…')
    expect(tag).toContain('aria-label="搜索帖子"')

    // 增强器不得覆盖作者已有的 aria-label。
    const real = searchInput('搜索帖子标题或内容…')
    real.setAttribute('aria-label', '搜索帖子')
    dispose = installSearchFieldA11y(document.body)
    await flush()
    expect(real.getAttribute('aria-label')).toBe('搜索帖子')
  })
})
