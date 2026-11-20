// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// t33 RED→GREEN：执行真实 ProcessTaskCenterApp.vue 的 title 纯函数。
import { readFileSync } from 'node:fs'
import path from 'node:path'
import vm from 'node:vm'
import { fileURLToPath } from 'node:url'
import { transformWithEsbuild } from 'vite'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const source = readFileSync(path.join(here, 'ProcessTaskCenterApp.vue'), 'utf8')
async function extracted(): Promise<Record<string, any>> {
  const open = source.indexOf('>', source.indexOf('<script setup'))
  const close = source.indexOf('</script>', open)
  const body = source.slice(open + 1, close)
  const start = body.indexOf('// 原始流程 ID/实例键')
  const end = body.indexOf('const startM =', start)
  expect(start).toBeGreaterThan(-1)
  expect(end).toBeGreaterThan(start)
  const snippet = (await transformWithEsbuild(body.slice(start, end), 't33-titles.ts', { loader: 'ts', format: 'cjs', target: 'es2022' })).code
  const sandbox: Record<string, any> = { exports: {}, module: { exports: {} } }
  vm.createContext(sandbox)
  new vm.Script(snippet + '\nmodule.exports = { displayTitle, workTitle, isRawProcessKey, shortId }').runInContext(sandbox)
  return sandbox.module.exports
}
const { displayTitle, isRawProcessKey } = await extracted()
const uuid = '71d52ed3-6d4d-4c0e-9b1e-0a9b1e163700e'

describe('ProcessTaskCenterApp 标题运行时回归', () => {
  it('UUID 作为唯一标题字段时回退为可读流程任务', () => {
    expect(displayTitle({ title: uuid, work: uuid, id: uuid })).toBe('流程任务')
  })
  it('probe-w- 形态原始键不能作为主标题', () => {
    expect(displayTitle({ title: 'probe-w-1795102580411-scqxm4', work: '0251b1b2-704d-47e2-9df3-d717402e' })).toBe('流程任务')
    expect(source).toContain('流程任务')
  })
})
