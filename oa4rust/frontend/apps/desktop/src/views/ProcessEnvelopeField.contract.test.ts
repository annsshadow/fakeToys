// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R341 契约：3 页双层信封列表恒空 + CmsColumnManager 字段映射 P2。
 * 源码级静态断言 + new Function 真执行 queryFn 返回链（不引入新依赖，不渲染组件）。
 * 双层信封：后端 list 返回 {"data":{"count":N,"data":[...]},"type":"success"}，
 * 旧代码 Array.isArray(r?.data) 对对象返回 false → [] → 列表恒空。
 * 字段映射：后端返回 columnid/personid/updatetime 小写，模板列 key 用驼峰，
 * queryFn 需映射为 columnId/personId/updateTime，确保具名插槽显示真实值。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const dir = import.meta.dirname
const PAGES = [
  'ProcessDictDesignerApp.vue',
  'ProcessFormDesignerApp.vue',
  'ProcessXformApp.vue',
] as const
const CMS = 'CmsColumnManagerApp.vue'
const srcOf = (f: string): string => readFileSync(resolve(dir, f), 'utf8')

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

function execParse(source: string, resp: unknown): Promise<unknown> {
  let body = bodyOf(source, 'queryFn: async () => {')
  // Strip only TypeScript-only tokens; keep JS expression structure intact.
  body = body
    .replace(/\sas\s+unknown\s+as\s+\{\s*data\?:\s*unknown\s*\}/g, '')
    .replace(/\(r as any\)/g, 'r')
    .replace(/\(row: any\)/g, '(row)')
    .replace(/\sas\s+Item\[\]/g, '')
  const api = { get: async function (): Promise<unknown> { return resp } }
  const loading = { value: false }
  const listEp = '/api/_contract/list'
  return new Function('api', 'loading', 'listEp', 'return (async () => {' + body + '})()')(api, loading, listEp)
}

const doubleEnvelope = { data: { count: 1, data: [{ id: 'u-1', name: '流程记录' }] }, type: 'success' }
const singleEnvelope = { data: [{ id: 'u-2', name: '旧端点' }], type: 'success' }

describe('R341 三页双层信封解析', () => {
  for (const page of PAGES) {
    it(page + ' queryFn 双层信封返回内层数组（不再恒空）', async () => {
      const got = (await execParse(srcOf(page), doubleEnvelope)) as Array<{ id: string }>
      expect(Array.isArray(got)).toBe(true)
      expect(got).toHaveLength(1)
      expect(got[0].id).toBe('u-1')
    })

    it(page + ' queryFn 保留单层信封兼容', async () => {
      const got = (await execParse(srcOf(page), singleEnvelope)) as Array<{ id: string }>
      expect(Array.isArray(got)).toBe(true)
      expect(got).toHaveLength(1)
      expect(got[0].id).toBe('u-2')
    })
  }
})

describe('R341 CmsColumnManager 字段映射', () => {
  const cmsSrc = srcOf(CMS)
  const lowerRow = {
    id: 'cm-1',
    columnid: 'COL-123',
    personid: 'PER-456',
    role: 'manager',
    updatetime: '2026-11-22 02:00:00',
    createtime: '2026-11-21 01:00:00',
  }

  it('小写后端字段映射为模板所需驼峰 key', async () => {
    const got = (await execParse(cmsSrc, { data: [lowerRow], type: 'success' })) as Record<string, unknown>[]
    expect(Array.isArray(got)).toBe(true)
    expect(got).toHaveLength(1)
    expect(got[0].columnId).toBe('COL-123')
    expect(got[0].personId).toBe('PER-456')
    expect(got[0].updateTime).toBe('2026-11-22 02:00:00')
  })

  it('列 key 与具名插槽保持驼峰，不改列定义', () => {
    expect(cmsSrc).toContain("key: 'columnId'")
    expect(cmsSrc).toContain("key: 'personId'")
    expect(cmsSrc).toContain('#cell-columnId')
    expect(cmsSrc).toContain('#cell-personId')
  })

  it('create/save/delete 端点与调用未改动', () => {
    expect(cmsSrc).toContain("const createEp = '/api/cms/core/entity/column_manager/create'")
    expect(cmsSrc).toContain('api.put')
    expect(cmsSrc).toContain('api.post')
  })
})
