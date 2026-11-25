// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R501 WCAG 2.5.3 Label in Name 修复契约：
 * D1 BBSForum.vue:111 发帖浮层 role=dialog 的可访问名称必须含可见 h2「发起新主题」；
 * D2 ProgramCenterApp.vue:384 对象详情抽屉 role=dialog 的可访问名称必须含可见 strong 的 type·name。
 * 只改两处 dialog 容器的 aria-label/aria-labelledby：可见文本/@click/浮层结构/业务处理器/data-testid/样式零改。
 * 真父版 RED、修复版 GREEN。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const bbs = readFileSync(resolve(import.meta.dirname, 'BBSForum.vue'), 'utf8')
const program = readFileSync(resolve(import.meta.dirname, 'ProgramCenterApp.vue'), 'utf8')

function lineWith(src: string, anchor: string): string {
  const at = src.indexOf(anchor)
  if (at < 0) return ''
  const start = src.lastIndexOf(String.fromCharCode(10), at) + 1
  const end = src.indexOf(String.fromCharCode(10), at)
  return src.slice(start, end < 0 ? src.length : end)
}

describe('R501 Label in Name 两处 dialog 容器命名契约', () => {
  const compose = lineWith(bbs, 'class="modal glass-card compose-panel"')
  const drawer = lineWith(program, 'class="detail-drawer" role="dialog"')

  it('D1 发帖浮层 dialog 名称含可见标题「发起新主题」（核心）', () => {
    expect(compose, '必须找到发帖浮层 section').not.toBe('')
    expect(compose).toContain('role="dialog"')
    const named = compose.includes('aria-label="发起新主题 · 发帖区"')
    const labelled = /aria-labelledby="[^"]*bbs-compose-title[^"]*"/.test(compose)
    expect(named || labelled, 'aria-label 含可见文本或用 aria-labelledby 指向 h2 标题').toBe(true)
  })

  it('D2 对象详情抽屉 dialog 名称含可见 strong 的 type·name（核心）', () => {
    expect(drawer, '必须找到对象详情抽屉 div').not.toBe('')
    expect(drawer).toContain('role="dialog"')

    const dynLabel = /:aria-label="[^"]*selectedDetail\.type[^"]*selectedDetail\.name[^"]*"/.test(drawer)
    const dynLabelled = /aria-labelledby="[^"]*detail-drawer-title[^"]*"/.test(drawer)
    expect(dynLabel || dynLabelled, '动态 aria-label 含 type·name 或用 aria-labelledby 指向 strong 标题').toBe(true)
  })

  it('业务零改守卫：可见 h2 与 dialog 容器 class/data-testid/浮层结构保留', () => {
    expect(bbs).toContain('<h2 data-testid="bbs-compose-title">发起新主题</h2>')
    expect(compose).toContain('class="modal glass-card compose-panel"')
    expect(compose).toContain('data-testid="bbs-compose-panel"')
    expect(bbs).toContain('v-if="showNewTopic" class="modal-overlay"')
    expect(bbs).toContain('@click.self="showNewTopic = false"')
    expect(bbs).toContain('@submit.prevent="createTopic"')
  })

  it('业务零改守卫：可见 strong/关闭按钮/详情抽屉结构与样式类保留', () => {
    expect(program).toContain('<strong>{{ selectedDetail.type }} · {{ selectedDetail.name }}</strong>')
    expect(drawer).toContain('v-if="selectedDetail"')
    expect(drawer).toContain('class="detail-drawer"')
    expect(program).toContain('@click="selectedDetail = null"')
    expect(program).toContain('class="detail-drawer-body"')
    expect(program).toContain('class="detail-drawer-head"')
  })

  it('防外溢守卫：不靠改可见文本/不加 role 到别处/不改 props 与业务函数', () => {
    expect(bbs).toContain('const showNewTopic = ref(false)')
    expect(bbs).toContain('function createTopic(): void {')
    expect(program).toContain('const selectedDetail = ref')
    expect((bbs.match(/role="dialog"/g) || []).length).toBe(4)
    expect((program.match(/role="dialog"/g) || []).length).toBe(1)
  })
})
