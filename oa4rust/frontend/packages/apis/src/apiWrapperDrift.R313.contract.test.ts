// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

/**
 * R313 API 封装路径漂移契约：真断言三封装的 HTTP 方法与 URL，
 * 不以「函数已定义」为覆盖（覆盖率测试对 401/404 也放行，无法发现路径漂移）。
 * 后端注册对照：
 *   cms_assemble_control/src/routes.rs:526  POST   /api/document
 *   cms_assemble_control/src/routes.rs:527  POST   /api/document/{id}/update
 *   program_center/src/routes.rs:566        POST   /api/program_center/dict
 */
import type { Mock } from 'vitest'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const mockGet: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockPost: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockPut: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockDelete: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })

vi.mock('@oa4rust/sdk', () => ({
  api: { get: mockGet, post: mockPost, put: mockPut, delete: mockDelete, upload: vi.fn() },
}))

describe('R313 API 封装路径/方法漂移', () => {
  beforeEach(() => vi.clearAllMocks())

  it('documentApi.create 必须 POST /api/document（非 /api/document/document）', async () => {
    const { documentApi } = await import('./index.ts')
    const payload = { name: 'r313-doc' }
    await documentApi.create(payload)
    expect(mockPost).toHaveBeenCalledTimes(1)
    expect(mockPost).toHaveBeenCalledWith('/api/document', payload)
  })

  it('documentApi.update 必须 POST /api/document/{id}/update（非 PUT /api/document/document/{id}）', async () => {
    const { documentApi } = await import('./index.ts')
    const payload = { name: 'r313-updated' }
    await documentApi.update('doc-42', payload)
    expect(mockPost).toHaveBeenCalledTimes(1)
    expect(mockPost).toHaveBeenCalledWith('/api/document/doc-42/update', payload)
    expect(mockPut).not.toHaveBeenCalled()
  })

  it('programCenterApi.dictCreate 必须 POST /api/program_center/dict（非 /dict/create）', async () => {
    const { programCenterApi } = await import('./index.ts')
    const payload = { name: 'r313-dict' }
    await programCenterApi.dictCreate(payload)
    expect(mockPost).toHaveBeenCalledTimes(1)
    expect(mockPost).toHaveBeenCalledWith('/api/program_center/dict', payload)
  })

  it('apis.general.dictCreate 与 generalApi.dictCreate 同源：走 /api/general/dict/create（后端 general_core_entity 已注册），不应与 programCenterApi 混用', async () => {
    const { apis, generalApi, programCenterApi } = await import('./index.ts')
    expect(apis.general).toBe(generalApi)
    expect(apis.general.dictCreate).toBe(generalApi.dictCreate)
    await apis.general.dictCreate({ name: 'r313-general' })
    expect(mockPost).toHaveBeenCalledWith('/api/general/dict/create', { name: 'r313-general' })
    expect(mockPost).not.toHaveBeenCalledWith('/api/program_center/dict', expect.anything())
  })
})
