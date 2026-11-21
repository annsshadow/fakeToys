// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R326 D1：会议「我的会议」搜索/状态筛选不生效回归。
 * 根因：loadMeetings 构造 p.key/p.status 却调用固定 URL（死变量），后端也无 Query 过滤。
 * 本契约直接执行源码中的 loadMeetings 函数体，断言 query string 真的进入 api.get URL。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'MeetingApp.vue'), 'utf8')

function loadMeetingsBody(): string {
  const start = source.indexOf('async function loadMeetings()')
  expect(start, '找不到 loadMeetings 函数').toBeGreaterThan(-1)
  const end = source.indexOf('function statusTxt', start)
  expect(end, '找不到 loadMeetings 函数边界').toBeGreaterThan(start)
  return source.slice(start, end)
    .replace(/:\s*Record<[^>]*>/g, '')
    .replace(/\s+as\s+M\[\]/g, '')
}

interface Harness {
  urls: string[]
  run: (key: string, status: string) => Promise<{ rows: unknown[] }>
}

function harness(): Harness {
  const urls: string[] = []
  const api = {
    get: async (url: string) => {
      urls.push(url)
      return { data: [{ id: 'm1', title: 'R326' }] }
    },
  }
  const searchKey = { value: '' }
  const statusFilter = { value: '' }
  const meetings = { value: [] as unknown[] }
  const loading = { value: false }
  const loadError = { value: '' }
  const body = loadMeetingsBody()
  const factory = new Function(
    'api', 'searchKey', 'statusFilter', 'meetings', 'loading', 'loadError', 'describeListError',
    body + '\nreturn loadMeetings;',
  )
  const runFn = factory(api, searchKey, statusFilter, meetings, loading, loadError, (p: string, e: unknown) => String(e)) as () => Promise<void>
  return {
    urls,
    run: async (key: string, status: string) => {
      urls.length = 0
      searchKey.value = key
      statusFilter.value = status
      meetings.value = []
      await runFn()
      return { rows: meetings.value }
    },
  }
}

describe('MeetingApp 会议搜索/筛选 URL 契约（R326 D1）', () => {
  it('无筛选时保留原 URL（向后兼容）', async () => {
    const h = harness()
    const { rows } = await h.run('', '')
    expect(rows).toHaveLength(1)
    expect(h.urls[0]).toBe('/api/meeting/assemble/control/meeting/list/apply/1/size/50')
  })

  it('searchKey 非空时 URL 携带 key 参数（p 不是死变量）', async () => {
    const h = harness()
    await h.run('NO_SUCH_MEETING', '')
    expect(h.urls[0]).toContain('key=NO_SUCH_MEETING')
  })

  it('statusFilter 非空时 URL 携带 status 参数', async () => {
    const h = harness()
    await h.run('', '1')
    expect(h.urls[0]).toContain('status=1')
  })

  it('key+status 同时存在时两个参数都携带', async () => {
    const h = harness()
    await h.run('周会', '2')
    expect(h.urls[0]).toContain('key=')
    expect(h.urls[0]).toContain('status=2')
  })

  it('写读链与业务模板未回退', () => {
    expect(source).toContain('@submit.prevent="loadMeetings"')
    expect(source).toContain('placeholder="搜索会议主题/会议室"')
    expect(source).toContain('value="0">未开始')
    expect(source).toContain('list/apply/1/size/50')
  })
})
