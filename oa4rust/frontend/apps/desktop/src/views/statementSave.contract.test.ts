// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R614b QueryStatementDesigner 保存按钮 saveStatement 死绑定真修复契约。
 * 真父 :15 @click="saveStatement" 无函数定义 → 点击静默 no-op（RED）。
 * 修复后定义 saveStatement：按 id 复用 PUT /api/query/assemble/designer/save/{id}，
 * 经 queryStatementWriteSucceeded 守卫 envelope，不误报成功，无 prompt 副作用。
 */
// @vitest-environment node
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const SRC = readFileSync(resolve(import.meta.dirname, 'QueryStatementDesigner.vue'), 'utf8')

function bodyOf(name: string): string {
  const sig = 'function ' + name + '('
  const at = SRC.indexOf(sig)
  if (at < 0) return ''
  const open = SRC.indexOf('{', at)
  if (open < 0) return ''
  let depth = 0,
    inS = 0,
    inD = 0,
    inT = 0,
    esc = false
  for (let j = open; j < SRC.length; j++) {
    const c = SRC.charCodeAt(j)
    if (esc) {
      esc = false
      continue
    }
    if (c === 92) {
      esc = true
      continue
    }
    if (c === 39 && !inD && !inT) {
      inS = inS ? 0 : 1
      continue
    }
    if (c === 34 && !inS && !inT) {
      inD = inD ? 0 : 1
      continue
    }
    if (c === 96 && !inS && !inD) {
      inT = inT ? 0 : 1
      continue
    }
    if (inS || inD || inT) continue
    if (c === 123) depth++
    else if (c === 125) {
      depth--
      if (depth === 0) return SRC.slice(open + 1, j)
    }
  }
  return ''
}

function stripTs(s: string): string {
  return s.split(': any').join('')
}

function makeCtx(resp: any, selected: any, throwErr?: string): any {
  const calls: any[] = [],
    info: any[] = [],
    errors: any[] = [],
    invalidations: any[] = []
  return {
    currentStatement: { value: selected },
    sql: { value: selected ? selected.sql : '' },
    api: {
      put: async (url: string, payload: any) => {
        calls.push({ method: 'PUT', url, payload })
        if (throwErr) throw new Error(throwErr)
        return resp
      },
    },
    toast: {
      success: (m: string) => info.push(m),
      error: (m: string) => errors.push(m),
    },
    queryClient: { invalidateQueries: (o: any) => invalidations.push(o) },
    queryStatementWriteSucceeded: (r: any) => {
      if (!r || typeof r !== 'object') return false
      if (r.type === 'error' || r.success === false) return false
      if (r.type === 'success') return true
      if (r.type) return false
      return true
    },
    calls,
    info,
    errors,
    invalidations,
  }
}

async function run(ctx: any): Promise<any> {
  const body = stripTs(bodyOf('saveStatement'))
  const pre = [
    '"use strict";',
    'var currentStatement = ctx.currentStatement;',
    'var sql = ctx.sql;',
    'var api = ctx.api;',
    'var toast = ctx.toast;',
    'var queryClient = ctx.queryClient;',
    'var queryStatementWriteSucceeded = ctx.queryStatementWriteSucceeded;',
    'return async function saveStatement() {' + body + '};',
  ].join('\n')
  const fn = new Function('ctx', pre)(ctx)
  await fn()
  return ctx
}

const ITEM = { id: 'R614b-FAKE-ID', name: '测试语句', flag: 'r614b-flag', sql: 'select 1', category: 'CAT', desc: 'd' }

describe('R614b 保存按钮绑定静态守卫', () => {
  it('模板 @click=saveStatement 保留且存在真函数定义（真父无定义 → RED）', () => {
    expect(SRC).toContain('@click="saveStatement"')
    expect(bodyOf('saveStatement')).not.toBe('')
  })
  it('按 id 复用现有 PUT save 端点，不引入 prompt 副作用', () => {
    const body = bodyOf('saveStatement')
    expect(body).toContain('/api/query/assemble/designer/save/')
    expect(body).not.toContain('prompt(')
  })
})

describe('R614b saveStatement 动态行为', () => {
  it('选中语句点保存 → PUT 按 id 端点，payload 含 sql/name，成功 toast + invalidate', async () => {
    const c = await run(makeCtx({ type: 'success' }, ITEM))
    expect(c.calls.length).toBe(1)
    expect(c.calls[0].method).toBe('PUT')
    expect(c.calls[0].url).toContain('/api/query/assemble/designer/save/R614b-FAKE-ID')
    expect(c.calls[0].payload.sql).toBe('select 1')
    expect(c.calls[0].payload.name).toBe('测试语句')
    expect(c.info.length).toBeGreaterThan(0)
    expect(c.errors.length).toBe(0)
    expect(c.invalidations.length).toBe(1)
  })
  it('type:error envelope 不误报成功，弹错误且不 invalidate', async () => {
    const c = await run(makeCtx({ type: 'error', message: 'no such statement' }, ITEM))
    expect(c.info.length).toBe(0)
    expect(c.errors.some((m: string) => m.indexOf('no such statement') >= 0)).toBe(true)
    expect(c.invalidations.length).toBe(0)
  })
  it('success:false envelope 同样拦截', async () => {
    const c = await run(makeCtx({ type: 'success', success: false, message: 'denied' }, ITEM))
    expect(c.info.length).toBe(0)
    expect(c.errors.some((m: string) => m.indexOf('denied') >= 0)).toBe(true)
    expect(c.invalidations.length).toBe(0)
  })
  it('非对象响应判失败不误报成功', async () => {
    const c = await run(makeCtx(null, ITEM))
    expect(c.info.length).toBe(0)
    expect(c.errors.length).toBe(1)
    expect(c.invalidations.length).toBe(0)
  })
  it('HTTP 异常走 catch 红 toast', async () => {
    const c = await run(makeCtx(undefined, ITEM, 'HTTP 500'))
    expect(c.info.length).toBe(0)
    expect(c.errors.some((m: string) => m.indexOf('HTTP 500') >= 0)).toBe(true)
  })
  it('未选中语句零写（合理 no-op 守卫）', async () => {
    const c = await run(makeCtx({ type: 'success' }, null))
    expect(c.calls.length).toBe(0)
    expect(c.info.length).toBe(0)
  })
})
