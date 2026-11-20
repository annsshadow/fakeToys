// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// start.formRace.contract.test.ts: 源码级钉住发起流程表单加载只认最新一次
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const src = readFileSync(fileURLToPath(new URL('./start.vue', import.meta.url)), 'utf8')

describe('start 表单加载只认最新一次（第87轮）', () => {
  it('引入 createLatestGate 并创建 formGate 令牌', () => {
    expect(src).toMatch(/import\s+\{\s*createLatestGate\s*\}\s+from\s+['"]@\/utils\/latest['"]/)
    expect(src).toMatch(/formGate\s*=\s*createLatestGate\(\)/)
  })

  it('loadBoundForm 开头 begin 令牌', () => {
    const fn = src.slice(src.indexOf('async function loadBoundForm'))
    expect(fn).toMatch(/const\s+token\s*=\s*formGate\.begin\(\)/)
  })

  it('getProcess 响应后、写表单字段前守卫', () => {
    const fn = src.slice(src.indexOf('async function loadBoundForm'))
    const guard1 = fn.indexOf('if (!formGate.isLatest(token)) return', fn.indexOf('processApi.getProcess(processId)'))
    const assign1 = fn.indexOf('fields.value = form.fields')
    expect(guard1).toBeGreaterThanOrEqual(0)
    expect(guard1).toBeLessThan(assign1)
  })

  it('getForm 响应后、写表单字段前守卫', () => {
    const fn = src.slice(src.indexOf('async function loadBoundForm'))
    const guard = fn.indexOf('if (!formGate.isLatest(token)) return', fn.indexOf('processApi.getForm(formFlag)'))
    const assign = fn.indexOf('fields.value = form.fields')
    expect(guard).toBeGreaterThanOrEqual(0)
    expect(guard).toBeLessThan(assign)
  })

  it('catch 分支也守卫，过期错误不写旧表单', () => {
    const fn = src.slice(src.indexOf('async function loadBoundForm'))
    const catchPos = fn.indexOf('} catch (e) {')
    const guard = fn.indexOf('if (!formGate.isLatest(token)) return', catchPos)
    const write = fn.indexOf('formError.value = describeLoadError', catchPos)
    expect(guard).toBeGreaterThanOrEqual(0)
    expect(guard).toBeLessThan(write)
  })

  it('finally 只在最新令牌时清 formLoading', () => {
    const fn = src.slice(src.indexOf('async function loadBoundForm'))
    expect(fn).toMatch(/finally\s*\{\s*if\s*\(formGate\.isLatest\(token\)\)\s*formLoading\.value\s*=\s*false/)
  })
})
