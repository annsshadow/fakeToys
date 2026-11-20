// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// doc.uploadGuard.contract.test.ts: 钉住文档上传入口的并发在途守卫。
// doUpload 由 chooseImage/chooseMessageFile 的 success 回调以 void 方式触发，回调
// 不受在途状态约束；上传期间按钮 disabled 依赖 Vue nextTick 异步生效，快速双击或
// 回调重入可在 uploading=true 之前穿透到第二份 doUpload。缺入口守卫 => 同一文件写两次
// 附件 + 两个成功 toast。函数入口必须幂等拦截。
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const src = readFileSync(fileURLToPath(new URL('./doc.vue', import.meta.url)), 'utf8')

function uploadBody(): string {
  const start = src.indexOf('async function doUpload(')
  expect(start).toBeGreaterThanOrEqual(0)
  const next = src.indexOf('\nasync function ', start + 1)
  const end = next === -1 ? src.indexOf('</script>', start) : next
  expect(end).toBeGreaterThan(start)
  return src.slice(start, end)
}

describe('文档页上传入口并发守卫（R269）', () => {
  it('uploading 为真时 doUpload 入口立即返回，不重复写附件', () => {
    const body = uploadBody()
    const setTrueAt = body.indexOf('uploading.value = true')
    expect(setTrueAt).toBeGreaterThan(-1)
    const guard = body.indexOf('if (uploading.value) return')
    expect(guard).toBeGreaterThan(-1)
    expect(guard).toBeLessThan(setTrueAt)
  })
})
