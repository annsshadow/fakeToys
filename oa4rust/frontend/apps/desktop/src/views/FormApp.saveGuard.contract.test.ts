// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// FormApp.saveGuard.contract.test.ts: 钉住表单新建/编辑弹窗 saveForm 的在途守卫。
// 弹窗按钮 disabled 依赖 Vue nextTick 异步生效，快速双击或回调重入可在 saving=true
// 之前穿透到第二次 saveForm；表单 POST/PUT 非幂等，缺入口守卫会创建两份表单或覆盖两次。
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const src = readFileSync(fileURLToPath(new URL('./FormApp.vue', import.meta.url)), 'utf8')

function saveFormBody(): string {
  const start = src.indexOf('async function saveForm()')
  expect(start, '未找到 saveForm').toBeGreaterThanOrEqual(0)
  const end = src.indexOf('\nasync function deleteForm(', start)
  expect(end, '未找到 saveForm 结束边界').toBeGreaterThan(start)
  return src.slice(start, end)
}

describe('表单弹窗保存防重复提交（R275）', () => {
  const body = saveFormBody()

  it('maintains a saving in-flight ref', () => {
    expect(src).toContain('const saving = ref(false)')
  })

  it('saveForm 入口在写请求前拦截在途重入', () => {
    const guard = body.indexOf('if (saving.value) return')
    expect(guard, 'saveForm 缺少在途守卫').toBeGreaterThanOrEqual(0)
    const occupy = body.indexOf('saving.value = true')
    const firstWrite = Math.min(...[body.indexOf('api.post('), body.indexOf('api.put(')].filter((i) => i >= 0))
    expect(occupy).toBeGreaterThan(guard)
    expect(firstWrite).toBeGreaterThan(occupy)
  })

  it('finally 复位 saving，失败后不永久卡死按钮', () => {
    expect(body).toContain('finally {')
    const finallyIdx = body.indexOf('finally {')
    const resetIdx = body.indexOf('saving.value = false')
    expect(resetIdx).toBeGreaterThan(finallyIdx)
  })

  it('提交按钮在在途时禁用，与入口守卫双保险', () => {
    expect(src).toContain(':disabled="!mform.name || saving"')
  })
})
