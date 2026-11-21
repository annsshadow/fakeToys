// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

/**
 * R324 考勤三处前端修复契约（D1 明细双形态解包 / D2 error 信封分支 / R1 建删后刷新）。
 * 从 AttendanceApp.vue 源码提取真实代码块，剥离 TS 类型标注后用 new Function 真执行，
 * 断言业务语义（非空 records、toast 分支、loadData 调用），而非仅字符串存在。
 */
const src = readFileSync(fileURLToPath(new URL('./AttendanceApp.vue', import.meta.url)), 'utf8')

function stripTs(code: string): string {
  return code
    .replace(/\s+as\s+any\b/g, '')
    .replace(/\s+as\s+R\[\]/g, '')
    .replace(/\bconst\s+r\s*:\s*any\s*=/g, 'const r =')
    .replace(/catch\s*\(\s*e\s*:\s*any\s*\)/g, 'catch (e)')
    .replace(/function deleteDetail\(r:\s*R\)/g, 'function deleteDetail(r)')
}

function sliceBetween(startAnchor: string, endAnchor: string): string {
  const start = src.indexOf(startAnchor)
  expect(start, '未找到起始锚点: ' + startAnchor).toBeGreaterThan(-1)
  const end = src.indexOf(endAnchor, start)
  expect(end, '未找到结束锚点: ' + endAnchor).toBeGreaterThan(start)
  return stripTs(src.slice(start, end))
}

// ---- D1: 明细 queryFn 的 records/totalPages 赋值块 ----
function detailUnpackBlock(): string {
  const anchor = src.indexOf('attendancedetail/filter/list?month=')
  expect(anchor, '未找到明细列表端点').toBeGreaterThan(-1)
  const thenAt = src.indexOf('.then((r: any) => {', anchor)
  expect(thenAt, '未找到明细 queryFn then').toBeGreaterThan(anchor)
  const blockStart = src.indexOf('\n', thenAt) + 1
  const clampComment = src.indexOf('// 切换月份', blockStart)
  expect(clampComment, '未找到 clampPage 注释').toBeGreaterThan(blockStart)
  return stripTs(src.slice(blockStart, clampComment))
}
function runDetailUnpack(resp: unknown): { list: unknown[]; totalPages: number } {
  const fn = new Function('r', 'records', 'totalPages',
    detailUnpackBlock() + '\nreturn { list: records.value, totalPages: totalPages.value };') as
    (r: unknown, rec: { value: unknown[] }, tp: { value: number }) => { list: unknown[]; totalPages: number }
  return fn(resp, { value: [] }, { value: 0 })
}

// ---- D2: toggleAttType 真执行，捕获 toast 分支 ----
async function runToggle(resp: unknown): Promise<{ success: string[]; error: string[] }> {
  const body = sliceBetween('async function toggleAttType()', 'const exporting = ref(false)')
  const factory = new Function('prompt', 'confirmMsg', 'api', 'toast', body + '\nreturn toggleAttType;') as
    (...a: unknown[]) => () => Promise<void>
  const calls = { success: [] as string[], error: [] as string[] }
  const fn = factory(
    () => 'X',
    async () => true,
    { post: async () => resp },
    { success: (m: string) => { calls.success.push(m) }, error: (m: string) => { calls.error.push(m) } },
  )
  await fn()
  return calls
}

// ---- R1: createAttDetail / deleteDetail 真执行，捕获 loadData 调用 ----
async function runCreate(): Promise<number> {
  const body = sliceBetween('async function createAttDetail()', 'async function reciveMobileDetail()')
  const factory = new Function('prompt', 'businessTodayYMD', 'api', 'toast', 'loadData', body + '\nreturn createAttDetail;') as
    (...a: unknown[]) => () => Promise<void>
  let refresh = 0
  const fn = factory(
    () => 'admin',
    () => '2026-11-21',
    { post: async () => ({}) },
    { success: () => {}, error: () => {} },
    () => { refresh++ },
  )
  await fn()
  return refresh
}
async function runDelete(): Promise<number> {
  const body = sliceBetween('async function deleteDetail(', '// rev476')
  const factory = new Function('confirmMsg', 'api', 'toast', 'records', 'loadData', body + '\nreturn deleteDetail;') as
    (...a: unknown[]) => (r: { id: string }) => Promise<void>
  let refresh = 0
  const fn = factory(
    async () => true,
    { delete: async () => ({}) },
    { success: () => {}, error: () => {} },
    { value: [{ id: 'd1' }] },
    () => { refresh++ },
  )
  await fn({ id: 'd1' })
  return refresh
}

describe('R324 考勤明细列表双形态解包契约（D1）', () => {
  it('后端数组 data=[2条]+count=2 → records 非空长度2、totalPages>=1', () => {
    const out = runDetailUnpack({ data: [{ id: 'a' }, { id: 'b' }], count: 2, type: 'success' })
    expect(Array.isArray(out.list), 'records 必须是数组').toBe(true)
    expect(out.list.length).toBe(2)
    expect(out.totalPages).toBeGreaterThanOrEqual(1)
  })
  it('{list,total} 信封 3条/total=45 → records 长度3、totalPages=3', () => {
    const out = runDetailUnpack({ data: { list: [{ id: '1' }, { id: '2' }, { id: '3' }], total: 45 }, type: 'success' })
    expect(out.list.length).toBe(3)
    expect(out.totalPages).toBe(3)
  })
  it('空信封归一为空数组且 totalPages>=1', () => {
    const out = runDetailUnpack({})
    expect(out.list).toEqual([])
    expect(out.totalPages).toBeGreaterThanOrEqual(1)
  })
})

describe('R324 考勤类型开关 error 信封分支契约（D2）', () => {
  it('type:error 必须走 toast.error 且不走 toast.success', async () => {
    const calls = await runToggle({ type: 'error', message: 'setting not found' })
    expect(calls.error.length, 'error 分支必须调用 toast.error').toBeGreaterThanOrEqual(1)
    expect(calls.error[0]).toContain('setting not found')
    expect(calls.success.length, 'error 分支不得调用 toast.success').toBe(0)
  })
  it('type:success 必须走 toast.success 且不走 toast.error', async () => {
    const calls = await runToggle({ type: 'success', data: { updated: 1 } })
    expect(calls.success.length).toBeGreaterThanOrEqual(1)
    expect(calls.error.length).toBe(0)
  })
})

describe('R324 考勤建删后刷新契约（R1）', () => {
  it('createAttDetail 成功后必须触发 loadData 刷新', async () => {
    expect(await runCreate(), '建明细后须刷新列表/统计').toBeGreaterThanOrEqual(1)
  })
  it('deleteDetail 成功后必须触发 loadData 刷新', async () => {
    expect(await runDelete(), '删明细后须刷新列表/统计').toBeGreaterThanOrEqual(1)
  })
})

describe('R324 写读链未回退', () => {
  it('关键端点与删除链仍在', () => {
    expect(src).toContain('attendancedetail/filter/list?month=')
    expect(src).toContain('/api/attendance/assemble/control/attendancedetail')
    expect(src).toContain('/attendancedetail/\${r.id}')
    expect(src).toContain('attendancesetting/enable/type')
  })
})
