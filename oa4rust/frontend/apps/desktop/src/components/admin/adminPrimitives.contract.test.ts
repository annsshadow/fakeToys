// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R298 共享管理原语契约：AdminPageHeader / AdminToolbar / AdminTable / AdminFormSection。
 * t2-t4 约 58 个管理页复用这四个原语拼出「标题 + 工具条 + 列表 + 表单分区」的标准页，
 * 不再各页自造端点堆叠控制台。全部消费设计令牌；错误态复用 ListLoadError。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const read = (name: string): string => readFileSync(resolve(import.meta.dirname, name), 'utf8')

describe('R298 共享管理原语契约', () => {
  it('AdminPageHeader：标题 + 副标题 + 主操作插槽', () => {
    const src = read('AdminPageHeader.vue')
    expect(src).toMatch(/title: string/)
    expect(src).toMatch(/subtitle\?: string/)
    expect(src).toMatch(/<slot name="actions"/)
    expect(src).toMatch(/<h2[^>]*>\{\{ title \}\}<\/h2>/)
  })

  it('AdminToolbar：搜索 v-model + 筛选插槽 + 刷新/新建事件', () => {
    const src = read('AdminToolbar.vue')
    expect(src).toMatch(/'update:modelValue'/)
    expect(src).toMatch(/'search'/)
    expect(src).toMatch(/'refresh'/)
    expect(src).toMatch(/'create'/)
    expect(src).toMatch(/<slot name="filters"/)
    expect(src).toMatch(/@keydown\.enter="emit\('search'\)"/)
    expect(src).toMatch(/<button[^>]*type="button"/)
  })

  it('AdminTable：表头/行/骨架/空态，错误态复用 ListLoadError 并转发 retry', () => {
    const src = read('AdminTable.vue')
    expect(src).toContain("import ListLoadError from '../../views/ListLoadError.vue'")
    expect(src).toMatch(/columns: AdminTableColumn\[\]/)
    expect(src).toMatch(/rows: Record<string, unknown>\[\]/)
    expect(src).toMatch(/loading\?: boolean/)
    expect(src).toMatch(/error\?: string/)
    expect(src).toMatch(/<ListLoadError/)
    expect(src).toMatch(/@retry="emit\('retry'\)"/)
    expect(src).toMatch(/class="skeleton"/)
    expect(src).toMatch(/admin-table-empty/)
    expect(src).toMatch(/<th[^>]*v-for="col in columns"/)
    expect(src).toMatch(/<slot :name="'cell-' \+ col\.key"/)
  })

  it('AdminFormSection：分区标题 + 描述 + 默认插槽', () => {
    const src = read('AdminFormSection.vue')
    expect(src).toMatch(/title: string/)
    expect(src).toMatch(/description\?: string/)
    expect(src).toMatch(/<slot\s*\/>/)
    expect(src).toMatch(/glass-card/)
  })

  it('四个原语都消费设计令牌且不重复定义 glass-card / 裸 hex 颜色', () => {
    for (const name of ['AdminPageHeader.vue', 'AdminToolbar.vue', 'AdminTable.vue', 'AdminFormSection.vue']) {
      const src = read(name)
      expect(src, name).toMatch(/var\(--(space|radius|color|text|bg)-[a-z0-9-]+\)/)
      expect(src, name).not.toMatch(/\.glass-card\s*\{/)
      expect(src, name).not.toMatch(/#[0-9a-fA-F]{3,8}\b/)
    }
  })

  it('barrel index.ts 导出四个原语与分组数据，供 t2-t4 页面统一导入', () => {
    const src = read('index.ts')
    for (const name of ['AdminPageHeader', 'AdminToolbar', 'AdminTable', 'AdminFormSection', 'adminNavGroups']) {
      expect(src, name).toContain(name)
    }
  })
})
