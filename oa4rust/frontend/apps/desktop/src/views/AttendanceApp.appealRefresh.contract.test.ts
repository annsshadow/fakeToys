// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AttendanceApp.vue'), 'utf8')

function functionBody(signature: string): string {
  const start = source.indexOf(signature)
  expect(start, `未找到 ${signature}`).toBeGreaterThan(-1)
  const open = source.indexOf('{', start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === '{') depth++
    if (source[i] === '}' && --depth === 0) return source.slice(open, i + 1)
  }
  throw new Error(`${signature} 函数体未闭合`)
}

const writes = [
  ['auditAppealV2', 'async function auditAppealV2(a: A, auditStatus: string) {'],
  ['checkAppeal', 'async function checkAppeal(a: A) {'],
  ['updateAppealStatus', 'async function updateAppealStatus(a: A, status: string) {'],
  ['archiveAppeal', 'async function archiveAppeal(a: A) {'],
] as const

describe('考勤申诉写操作后消费服务器权威态（R280）', () => {
  for (const [name, signature] of writes) {
    it(`${name} 成功后刷新请假申请列表，避免渲染层误报旧状态`, () => {
      const body = functionBody(signature)
      const post = body.indexOf('api.post(')
      const refresh = body.indexOf('loadAppeals()')
      const catchIdx = body.indexOf('} catch')
      expect(post, '必须存在 api.post 写操作').toBeGreaterThan(-1)
      expect(refresh, '成功后必须调用 loadAppeals() 消费服务器权威态').toBeGreaterThan(-1)
      // 刷新必须在成功分支（api.post 之后、catch 之前），不能在失败分支或只靠 toast。
      expect(refresh).toBeGreaterThan(post)
      expect(catchIdx).toBeGreaterThan(refresh)
    })
  }
})
