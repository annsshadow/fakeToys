// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R315 F3 五处 API 封装方法/路径漂移真断言契约（vi.mock + toHaveBeenCalledWith，同 R313 范式）。
 * 后端真实注册（已逐处核对 .route 源码 + fixture，不臆造）：
 *   general_core_entity/lib.rs:1078 POST /api/general/dict/item/create（dictId 从 body 读）
 *   query_assemble_surface/lib.rs:617 POST /api/queryview/search（body.key）
 *   query_assemble_surface/lib.rs:404 POST /api/queryview/importmodel/list/{query}/{flag}
 *   program_center/routes.rs:632 POST /api/program_center/invoke/{flag}/execute
 *   jpush_core_entity/lib.rs:274 GET /api/jpush/core/entity/template/{id}（无 update 方法）
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import type { Mock } from 'vitest'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const mockGet: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockPost: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockPut: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })
const mockDelete: Mock = vi.fn().mockResolvedValue({ success: true, data: {} })

vi.mock('@oa4rust/sdk', () => ({
  api: { get: mockGet, post: mockPost, put: mockPut, delete: mockDelete, upload: vi.fn() },
}))

describe('R315 F3 五处封装方法/路径漂移', () => {
  beforeEach(() => vi.clearAllMocks())

  it('generalApi.dictItemCreate 必须 POST /api/general/dict/item/create（dictId 入 body）', async () => {
    const { generalApi } = await import('./index.ts')
    await generalApi.dictItemCreate('dict-7', { name: 'n', value: 'v' })
    expect(mockPost).toHaveBeenCalledTimes(1)
    expect(mockPost).toHaveBeenCalledWith('/api/general/dict/item/create', { dictId: 'dict-7', name: 'n', value: 'v' })
  })

  it('queryViewApi.search 必须 POST /api/queryview/search（非 GET，body.key）', async () => {
    const { queryViewApi } = await import('./index.ts')
    await queryViewApi.search('abc')
    expect(mockPost).toHaveBeenCalledTimes(1)
    expect(mockPost).toHaveBeenCalledWith('/api/queryview/search', { key: 'abc' })
    expect(mockGet).not.toHaveBeenCalled()
  })

  it('queryViewApi.importModelList 必须 POST /api/queryview/importmodel/list/{query}/{flag}', async () => {
    const { queryViewApi } = await import('./index.ts')
    await queryViewApi.importModelList('q1', 'f1')
    expect(mockPost).toHaveBeenCalledTimes(1)
    expect(mockPost).toHaveBeenCalledWith('/api/queryview/importmodel/list/q1/f1', {})
    expect(mockGet).not.toHaveBeenCalled()
  })

  it('programCenterApi.moduleInvoke 必须 POST /api/program_center/invoke/{flag}/execute', async () => {
    const { programCenterApi } = await import('./index.ts')
    const payload = { a: 1 }
    await programCenterApi.moduleInvoke('flag-9', payload)
    expect(mockPost).toHaveBeenCalledTimes(1)
    expect(mockPost).toHaveBeenCalledWith('/api/program_center/invoke/flag-9/execute', payload)
  })

  it('jpushApi.templateUpdate 后端无 update 方法，必须显式标注 @dead（不臆造方法）', () => {
    const src = readFileSync(resolve(import.meta.dirname, 'index.ts'), 'utf8')
    const start = src.indexOf('templateUpdate:')
    expect(start).toBeGreaterThan(-1)
    const window = src.slice(Math.max(0, start - 140), start)
    expect(window).toContain('@dead')
    const fixture = readFileSync(resolve(import.meta.dirname, '../../../../tests/contracts/backend-route-methods.fixture.ts'), 'utf8')
    expect(fixture).toContain("'/api/jpush/core/entity/template/{}': ['GET']")
    expect(fixture).not.toMatch(/'\/api\/jpush\/core\/entity\/template\/\{\}':[^\n]*(PUT|POST)/)
  })
})
