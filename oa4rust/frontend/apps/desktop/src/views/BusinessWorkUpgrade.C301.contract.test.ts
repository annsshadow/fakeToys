// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

function rootTemplate(src: string): string {
  const start = src.indexOf('<template>')
  const end = src.lastIndexOf('</template>')
  return src.slice(start, end)
}
function scriptOf(src: string): string {
  const start = src.indexOf('<script setup')
  const end = src.indexOf('</script>', start)
  return src.slice(start, end)
}

const recycle = readFileSync(fileURLToPath(new URL('./RecycleApp.vue', import.meta.url)), 'utf8')
const note = readFileSync(fileURLToPath(new URL('./NoteApp.vue', import.meta.url)), 'utf8')
const rt = rootTemplate(recycle)
const rs = scriptOf(recycle)
const nt = rootTemplate(note)
const ns = scriptOf(note)

describe('C301 回收站筛选与批量操作实质升级契约', () => {
  it('保留既有业务区块与单一 h1 语义层级', () => {
    for (const id of ['recycle-business-view', 'recycle-header', 'recycle-stats', 'recycle-toolbar', 'recycle-list']) {
      expect(rt, `缺少业务区块 ${id}`).toContain(`data-testid="${id}"`)
    }
    expect(rt.match(/<h1\b/g) ?? []).toHaveLength(1)
    expect(rt).toContain('批量操作')
    expect(rt).toContain('v-for="item in items"')
  })

  it('提供关键词+类型双筛选，列表来自计算属性且可清除筛选', () => {
    expect(rt).toContain('data-testid="recycle-keyword-filter"')
    expect(rt).toContain('data-testid="recycle-type-filter"')
    expect(rt).toContain('v-model="keyword"')
    expect(rt).toContain('v-model="typeFilter"')
    expect(rt).toContain('data-testid="recycle-filter-count"')
    expect(rs).toContain('const allItems = ref<RecycleItem[]>([])')
    expect(rs).toContain('const items = computed(() =>')
    expect(rs).toContain("const keyword = ref('')")
    expect(rs).toContain("const typeFilter = ref('all')")
    expect(rs).toContain('function itemType(')
    expect(rs).toContain('function clearFilters(')
  })

  it('提供勾选、全选、批量恢复与批量永久删除，复用旧端点不改接口', () => {
    expect(rt).toContain('data-testid="recycle-select-all"')
    expect(rt).toContain('data-testid="recycle-batch-resume"')
    expect(rt).toContain('data-testid="recycle-batch-delete"')
    expect(rt).toContain('selectedIds.includes(item.id)')
    expect(rs).toContain('const selectedIds = ref<string[]>([])')
    expect(rs).toContain('function toggleSelect(')
    expect(rs).toContain('function toggleSelectAll(')
    expect(rs).toContain('async function batchResume(')
    expect(rs).toContain('async function batchDelete(')
    expect(rs).toContain('Promise.allSettled')
    expect(rs).toContain('/api/recycle/resume/${id}')
    expect(rs).toContain('/api/recycle/${id}/delete')
    expect(rs).toContain("api.get('/api/recycle/list')")
    expect(rs).toContain("api.delete('/api/recycle/empty')")
  })

  it('空态有说明文案与主行动按钮，区分筛选空与真回收站空', () => {
    expect(rt).toContain('data-testid="recycle-empty-cta"')
    expect(rt).toContain('没有符合当前筛选条件的回收项目')
    expect(rt).toContain('回收站为空，暂无可恢复或清理的项目')
    expect(rt).not.toContain('<p>回收站为空</p>')
  })

  it('笔记表单不再暴露技术 ID 字段，成员输入可读且保留原 payload', () => {
    expect(nt).not.toContain('成员 ID')
    expect(nt).not.toContain('placeholder="personId"')
    expect(nt).toContain('data-testid="note-member-input"')
    expect(nt).toContain('placeholder="成员姓名 / 工号（可选）"')
    expect(nt).toContain('data-testid="note-member-cell"')
    expect(nt.match(/<h1\b/g) ?? []).toHaveLength(1)
    expect(ns).toContain('personId')
    expect(ns).toContain("api.post(createEp, payload)")
  })
})
