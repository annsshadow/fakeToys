// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R348 契约：PortalWidgetDesignerApp.vue queryFn 双层信封解析。
 * R347 真浏览器 deep 探针坐实 list 返回 data={count:N,data:[...]}，
 * 旧代码 Array.isArray(r?.data) 对对象返回 false → [] → 列表恒空假空态。
 * 本契约用 readFileSync 读源码 + 剥离 TS token + new Function 注入 api/loading/listEp 桩真执行 queryFn。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, 'PortalWidgetDesignerApp.vue'), 'utf8')

function bodyOf(source: string, signature: string): string {
  const start = source.indexOf(signature)
  if (start < 0) throw new Error('未找到函数: ' + signature)
  const open = source.indexOf('{', start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === '{') depth++
    else if (source[i] === '}') {
      depth--
      if (depth === 0) return source.slice(open + 1, i)
    }
  }
  throw new Error('花括号未配对: ' + signature)
}

function execParse(resp: unknown): Promise<unknown> {
  let body = bodyOf(src, 'queryFn: async () => {')
  body = body
    .split(' as unknown as { data?: unknown }').join('')
    .split('(r as any)').join('r')
    .split(' as Item[]').join('')
  const api = { get: async function (u: string): Promise<unknown> { return resp } }
  const loading = { value: false }
  const listEp = '/api/portal/assemble/designer/widget/list'
  return new Function('api', 'loading', 'listEp', 'return (async () => {' + body + '})()')(api, loading, listEp)
}

const double1 = { data: { count: 1, data: [{ id: 'w-1', name: '组件A' }] }, type: 'success' }
const double2 = { data: { count: 2, data: [{ id: 'w-1', name: '组件A' }, { id: 'w-2', name: '组件B' }] }, type: 'success' }
const single = { data: [{ id: 'w-3', name: '单层' }], type: 'success' }
const bad = { data: 'oops', type: 'success' }

describe('R348 PortalWidgetDesigner 双层信封解析', () => {
  it('双层信封1条真执行返回内层数组（不再恒空）', async () => {
    const got = (await execParse(double1)) as Array<{ id: string }>
    expect(Array.isArray(got)).toBe(true)
    expect(got).toHaveLength(1)
    expect(got[0].id).toBe('w-1')
  })

  it('双层信封2条真执行完整返回2条', async () => {
    const got = (await execParse(double2)) as Array<{ id: string }>
    expect(got).toHaveLength(2)
    expect(got[1].id).toBe('w-2')
  })

  it('保留单层信封兼容', async () => {
    const got = (await execParse(single)) as Array<{ id: string }>
    expect(got).toHaveLength(1)
    expect(got[0].id).toBe('w-3')
  })

  it('坏数据（data为字符串）容错返回空数组', async () => {
    expect(await execParse(bad)).toEqual([])
  })

  it('端点与写链保持原样', () => {
    expect(src).toContain("const listEp = '/api/portal/assemble/designer/widget/list'")
    expect(src).toContain('api.post(createEp, payload)')
    expect(src).toContain('/api/portal/assemble/designer/widget/save/')
    expect(src).toContain('widget/delete/')
  })
})
