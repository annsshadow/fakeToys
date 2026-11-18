// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// recycle.writeRefresh.contract.test.ts: 钉住回收站写后刷新的本地状态一致性。
// 删除/清空成功后刷新列表；若刷新请求失败，loadList 会回退到 previousRows。
// 必须传入"乐观排除已删行"的 previousRows，否则已删条目会在"已删除/已清空"提示后重现。
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'
import { loadList } from '../../utils/loadList'

const src = readFileSync(fileURLToPath(new URL('./recycle.vue', import.meta.url)), 'utf8')

function bodyOf(name: 'remove' | 'emptyAll'): string {
  const start = src.indexOf(`async function ${name}`)
  expect(start).toBeGreaterThanOrEqual(0)
  const next = src.indexOf('\nasync function ', start + 1)
  const end = next === -1 ? src.indexOf('</script>', start) : next
  return src.slice(start, end)
}

describe('recycle 写后刷新的本地状态一致性（第105轮）', () => {
  it('remove() 删除成功后刷新时乐观排除已删行', () => {
    const body = bodyOf('remove')
    expect(body).toMatch(/await\s+recycleApi\.remove\(id\)/)
    expect(body).toMatch(/await\s+load\(\s*rows\.value\.filter\(/)
    expect(body).toMatch(/String\(r\.id\s*\?\?\s*''\)\s*!==\s*id/)
  })

  it('emptyAll() 清空成功后刷新传空 previousRows', () => {
    const body = bodyOf('emptyAll')
    expect(body).toMatch(/await\s+recycleApi\.empty\(\)/)
    expect(body).toMatch(/await\s+load\(\[\]\)/)
  })

  it('行为闭环：刷新失败时只保留乐观排除后的行，已删条目不重现', async () => {
    type Row = Record<string, unknown>
    const rows: Row[] = [{ id: 'a' }, { id: 'b' }]
    const removedId = 'a'
    const optimistic = rows.filter((r) => String(r.id ?? '') !== removedId)
    const fetchFail = () => Promise.reject(new Error('网络抖动'))
    const result = await loadList<Row>(fetchFail, { previousRows: optimistic, label: '回收站加载失败' })
    expect(result.ok).toBe(false)
    expect(result.rows.map((r) => r.id)).toEqual(['b'])
    // 反证：若沿用默认 previousRows（含已删行），刷新失败时已删条目会重现。
    const stale = await loadList<Row>(fetchFail, { previousRows: rows, label: '回收站加载失败' })
    expect(stale.rows.map((r) => r.id)).toEqual(['a', 'b'])
  })
})
