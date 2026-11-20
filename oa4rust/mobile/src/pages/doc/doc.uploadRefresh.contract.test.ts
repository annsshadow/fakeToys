// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// doc.uploadRefresh.contract.test.ts: 钉住文档页上传成功后的写后列表同步。
// 上传落点就是本页「我的附件（FILE_FILE）」列表；上传成功若不刷新，新附件要等
// 退出重进页面触发 onShow 才出现。刷新请求失败时 loadList 保留旧行，不会误报上传失败。
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'
import { loadList } from '../../utils/loadList'

const src = readFileSync(fileURLToPath(new URL('./doc.vue', import.meta.url)), 'utf8')

function uploadBody(): string {
  const start = src.indexOf('async function doUpload(')
  expect(start).toBeGreaterThanOrEqual(0)
  const next = src.indexOf('\nasync function ', start + 1)
  const end = next === -1 ? src.indexOf('</script>', start) : next
  expect(end).toBeGreaterThan(start)
  return src.slice(start, end)
}

describe('文档页上传成功后刷新附件列表（R258）', () => {
  it('上传成功后刷新附件列表', () => {
    const body = uploadBody()
    expect(body).toMatch(/await\s+fileApi\.upload\(/)
    const success = body.indexOf("uni.showToast({ title: '上传成功'")
    expect(success).toBeGreaterThan(-1)
    const refetch = body.indexOf('await load()', success)
    expect(refetch).toBeGreaterThan(success)
    // 刷新必须在 catch 之前：只刷新成功响应，不在上传失败时刷新。
    const catchAt = body.indexOf('} catch (', success)
    expect(refetch).toBeLessThan(catchAt)
  })

  it('刷新失败时 loadList 保留旧行，不抹空列表', async () => {
    const previousRows = [{ id: 'old' }]
    const result = await loadList<Record<string, unknown>>(() => Promise.reject(new Error('网络抖动')), {
      previousRows,
      label: '文档加载失败',
    })
    expect(result.ok).toBe(false)
    expect(result.rows).toEqual(previousRows)
  })
})
