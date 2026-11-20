// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'PortalPageDesignerApp.vue'), 'utf8')
const editStart = source.indexOf('function editItem(item: Item) {')
const editSrc = source.slice(editStart, source.indexOf('async function loadPageMeta(', editStart))
const metaStart = source.indexOf('async function loadPageMeta(id: string) {')
const metaSrc = source.slice(metaStart, source.indexOf('function closeModal()', metaStart))

describe('门户页面设计器详情回填竞态守卫', () => {
  it('竞态真实可达：编辑按钮无 disabled，详情说明在浮层内渲染，无 loading 短路', () => {
    const editBtn = source.split('\n').find((l) => l.includes('@click="editItem(item)"')) ?? ''
    expect(editBtn).toContain('editItem(item)')
    expect(editBtn.includes('disabled')).toBe(false)
    expect(source).toContain('v-if="showEdit && pageMeta"')
    expect(metaSrc.includes('if (loading.value)')).toBe(false)
  })

  it('loadPageMeta 用递增序号只认最新一次，过期响应不得覆盖当前页面说明', () => {
    expect(source).toContain('let detailSeq = 0')
    expect(metaSrc).toContain('const seq = ++detailSeq')
    const begin = metaSrc.indexOf('const seq = ++detailSeq')
    expect(begin).toBeGreaterThan(-1)
    expect(begin).toBeLessThan(metaSrc.indexOf('await Promise.all'))
    const guard = metaSrc.indexOf('if (seq !== detailSeq) return')
    expect(guard).toBeGreaterThan(-1)
    expect(guard).toBeLessThan(metaSrc.indexOf('pageMeta.value = `分类'))
    const secondGuard = metaSrc.indexOf('if (seq !== detailSeq) return', guard + 1)
    expect(secondGuard).toBeGreaterThan(guard)
    expect(secondGuard).toBeLessThan(metaSrc.indexOf('pageMeta.value = `分类'))
  })
})
