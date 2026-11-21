// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

/**
 * R322 D1 真 bug 回归契约：后端 PUT reply/filter/list/page/{p}/count/{n} 返回
 * 双层信封 { data: { data: [reply...], total }, type: success }，
 * 旧实现 `((resp as any)?.data ?? []) as Reply[]` 只解一层，replies 被赋为对象，
 * 模板 replies.length=undefined、v-for 遍历对象键 → bbs-reply-undefined，已落库回复不渲染。
 * 本契约从源码提取解包 return 行，剥离 TS 类型断言后真执行，
 * 断言双层/单层/空信封均归一为 Reply[] 数组。
 */
const src = readFileSync(fileURLToPath(new URL('./BBSForum.vue', import.meta.url)), 'utf8')

function replyQueryFnBody(source: string): string {
  const anchor = source.indexOf("api.put('/api/bbs/assemble/control/reply/filter/list/page/1/count/50'")
  expect(anchor, '未找到 replies queryFn 端点').toBeGreaterThan(-1)
  const start = source.lastIndexOf('queryFn: async () => {', anchor)
  expect(start, '未找到 replies queryFn').toBeGreaterThan(-1)
  const end = source.indexOf('enabled:', anchor)
  return source.slice(start, end > -1 ? end : anchor + 400)
}

function unpackExpr(source: string): string {
  const body = replyQueryFnBody(source)
  const line = body.split('\n').map((l) => l.trim()).find((l) => l.startsWith('return ') && l.includes('as Reply[]'))
  expect(line, '未找到 reply 解包 return 行').toBeDefined()
  let e = line!.slice('return '.length).trim()
  e = e.replace(/\s+as\s+Reply\[\]/g, '')
  e = e.replace(/\s+as\s+any\b/g, '')
  return e.trim()
}

function unpackReplies(resp: unknown): unknown {
  const fn = new Function('resp', 'return (' + unpackExpr(src) + ');') as (r: unknown) => unknown
  return fn(resp)
}

describe('BBSForum 回复列表信封解包契约（R322 D1）', () => {
  it('双层信封 {data:{data:[...],total}} 解包后必须是数组、长度等于真实条数、每条 id 非 undefined', () => {
    const resp = {
      data: {
        data: [
          { id: 'r1', content: '回复一', author: '甲' },
          { id: 'r2', content: '回复二', author: '乙' },
          { id: 'r3', content: '回复三', author: '丙' },
        ],
        total: 3,
      },
      type: 'success',
    }
    const out = unpackReplies(resp) as Array<Record<string, unknown>>
    expect(Array.isArray(out), 'replies 必须是数组，不能是内层信封对象').toBe(true)
    expect(out.length).toBe(3)
    expect(out.every((r) => r && typeof r.id === 'string' && r.id.length > 0), '每条回复 id 非 undefined').toBe(true)
    expect(out[0]?.content).toBe('回复一')
  })

  it('单层信封 {data:[...]} 亦归一为数组（不回归）', () => {
    const out = unpackReplies({ data: [{ id: 's1', content: '单层' }], type: 'success' }) as Array<Record<string, unknown>>
    expect(Array.isArray(out)).toBe(true)
    expect(out.length).toBe(1)
    expect(out[0]?.id).toBe('s1')
  })

  it('空信封与空双层信封归一为空数组（不回归）', () => {
    expect(unpackReplies({})).toEqual([])
    expect(unpackReplies(null)).toEqual([])
    expect(unpackReplies({ data: { data: [], total: 0 } })).toEqual([])
  })

  it('写读链与模板未回退：GET reply/list/sub、POST reply/create、bbs-reply-{id} 均在', () => {
    expect(src).toContain('api.get(`/api/bbs/assemble/control/reply/list/sub/${subjectId}`)')
    expect(src).toContain("api.post('/api/bbs/assemble/control/reply/create'")
    expect(src).toContain("'bbs-reply-' + reply.id")
  })

  it('replies query 的 enabled 必须是响应式 getter（否则详情初次打开/刷新不触发查询）', () => {
    expect(src).toContain('enabled: () => !!viewingTopic.value')
    expect(src).not.toContain('computed(() => !!viewingTopic.value).value')
  })

  it('作者字段兼容后端 creator（reply/filter/list 不返回 author）', () => {
    expect(src).toContain("author: (r as any).author ?? (r as any).creator ?? ''")
  })
})