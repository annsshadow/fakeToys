// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// t33 RED→GREEN：执行真实 H5 process.vue 的 title 纯函数。
import { readFileSync } from 'node:fs'
import path from 'node:path'
import vm from 'node:vm'
import { fileURLToPath } from 'node:url'
import { transformWithEsbuild } from 'vite'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const source = readFileSync(path.join(here, 'process.vue'), 'utf8')
async function extracted(): Promise<Record<string, any>> {
  const open = source.indexOf('>', source.indexOf('<script setup'))
  const close = source.indexOf('</script>', open)
  const body = source.slice(open + 1, close)
  const start = body.indexOf('function isRawProcessKey')
  const end = body.indexOf('function metaOf', start)
  expect(start).toBeGreaterThan(-1)
  expect(end).toBeGreaterThan(start)
  const snippet = (await transformWithEsbuild(body.slice(start, end), 't33-h5-titles.ts', { loader: 'ts', format: 'cjs', target: 'es2022' })).code
  const sandbox: Record<string, any> = { exports: {}, module: { exports: {} }, workIdOf: (row: any) => row.work || row.workId || row.id }
  vm.createContext(sandbox)
  new vm.Script(snippet + '\nmodule.exports = { titleOf, taskIdOf, isRawProcessKey, shortId }').runInContext(sandbox)
  return sandbox.module.exports
}
const { titleOf, isRawProcessKey } = await extracted()
const uuid = '71d52ed3-6d4d-4c0e-9b1e-0a9b1e163700e'

describe('H5 process 标题运行时回归', () => {
  it('start:S3 运行闭环 + work 实例键不得剥成空白标题', () => {
    expect(titleOf({ title: 'start:S3 运行闭环 work-1795166639123-yltovm', work: uuid }, 0)).toBe('S3 运行闭环')
  })
  it('全为原始键/UUID 时回退为可读流程标题而不是空白', () => {
    expect(titleOf({ title: uuid, work: uuid, id: uuid }, 0)).not.toBe('')
    expect(titleOf({ title: uuid, work: uuid, id: uuid }, 0)).toBe('流程 #1')
  })
  it('probe-w- 形态原始键不能作为主标题', () => {
    const key = 'probe-w-1795102580411-scqxm4'
    expect(typeof isRawProcessKey).toBe('function')
    expect(titleOf({ title: key, work: '0251b1b2-704d-47e2-9df3-d717402e' }, 8)).toBe('流程 #9')
    expect(source).toMatch(/probe-w-|流程 #\$\{i \+ 1\}/)
  })
})
