/**
 * apis/index.ts 尾段转发器形状测试（覆盖补盲）。
 *
 * index.ts 里 62 个 API 模块的绝大多数一行转发器已由
 * api-coverage-generated.test.ts 实跑覆盖；本文件钉死仍无覆盖的
 * 5 个单方法模块（editor/uuid/exportDetail/importDetail/appConfig）与
 * processplatformSurfaceApi 全部 15 个方法——断言每个方法转发到正确的
 * 动词 + 路径 + 请求体，防止生成器/手工编辑改坏端点。
 */
import { beforeEach, describe, expect, it, vi } from 'vitest'

const mockCalls: Array<{ verb: 'get' | 'post' | 'put' | 'delete'; path: string; body?: unknown }> = []

vi.mock('@oa4rust/sdk', () => ({
  api: {
    get: (path: string) => {
      mockCalls.push({ verb: 'get', path })
      return Promise.resolve({ success: true, data: {} })
    },
    post: (path: string, body?: unknown) => {
      mockCalls.push({ verb: 'post', path, body })
      return Promise.resolve({ success: true, data: undefined })
    },
    put: (path: string, body?: unknown) => {
      mockCalls.push({ verb: 'put', path, body })
      return Promise.resolve({ success: true, data: undefined })
    },
    delete: (path: string) => {
      mockCalls.push({ verb: 'delete', path })
      return Promise.resolve({ success: true, data: undefined })
    },
    upload: (path: string) => {
      mockCalls.push({ verb: 'post', path })
      return Promise.resolve({ success: true, data: {} })
    },
  },
}))

beforeEach(() => {
  mockCalls.length = 0
})

describe('uncovered single-method module forwarders', () => {
  it('pins verb + path for editor / uuid / exportDetail / importDetail / appConfig', async () => {
    const m = await import('./index.ts')
    await m.editorApi.list()
    await m.uuidApi.random()
    await m.exportDetailApi.appInfo()
    await m.importDetailApi.appInfo()
    await m.appConfigApi.get('app-1')
    expect(mockCalls).toEqual([
      { verb: 'get', path: '/api/editor/list' },
      { verb: 'get', path: '/api/uuid/random' },
      { verb: 'get', path: '/api/export/appInfo' },
      { verb: 'get', path: '/api/import/appInfo' },
      { verb: 'get', path: '/api/appconfig/app-1' },
    ])
  })
})

describe('processplatformSurface forwarders (workflow surface, 963 routes)', () => {
  it('pins verb + path for all 15 surface methods', async () => {
    const { processplatformSurfaceApi: s } = await import('./index.ts')
    const base = '/api/processplatform/assemble/surface'
    await s.openapi()
    await s.get('x1')
    await s.sign('x1')
    await s.snap('x1')
    await s.task('x1')
    await s.work('x1')
    await s.draft('x1')
    await s.route('x1')
    await s.form('f1')
    await s.review('x1')
    await s.preview('x1')
    await s.handover('x1')
    await s.create({ a: 1 })
    await s.save('x1', { a: 1 })
    await s.delete('x1')

    expect(mockCalls.map((c) => [c.verb, c.path])).toEqual([
      ['get', `${base}/openapi`],
      ['get', `${base}/get/x1`],
      ['get', `${base}/sign/x1`],
      ['get', `${base}/snap/x1`],
      ['get', `${base}/task/x1`],
      ['get', `${base}/work/x1`],
      ['get', `${base}/draft/x1`],
      ['get', `${base}/route/x1`],
      ['get', `${base}/form/f1`],
      ['get', `${base}/review/x1`],
      ['get', `${base}/preview/x1`],
      ['post', `${base}/handover/x1`],
      ['post', `${base}/create`],
      ['put', `${base}/save/x1`],
      ['delete', `${base}/x1`],
    ])
    expect(mockCalls[12].body).toEqual({ a: 1 })
    expect(mockCalls[13].body).toEqual({ a: 1 })
  })
})
