/**
 * taskHandle 分支覆盖测试
 *
 * 策略：mock SDK api 对象（真实的 I/O 外部依赖，不在本层覆盖范围内），
 * 但让 taskHandle 的函数体真正执行，覆盖 approve / reject 两条分支。
 */

import { beforeEach, describe, expect, it, vi } from 'vitest'

const mockPost = vi.fn(() => Promise.resolve({ data: 'task-result' }))

vi.mock('@oa4rust/sdk', () => ({
  api: {
    get: vi.fn(() => Promise.resolve({})),
    post: mockPost,
    put: vi.fn(() => Promise.resolve({})),
    delete: vi.fn(() => Promise.resolve({})),
  },
}))

let taskHandle: (taskId: string, action: 'approve' | 'reject', data?: Record<string, unknown>) => Promise<unknown>

beforeAll(async () => {
  const mod = await import('./index.ts')
  taskHandle = mod.processApi.taskHandle
})

beforeEach(() => {
  vi.clearAllMocks()
})

describe('taskHandle() 分支覆盖', () => {
  it('approve 分支：action === "approve" -> POST /complete', async () => {
    const result = await taskHandle('task-123', 'approve', { opinion: 'ok' })
    expect(mockPost).toHaveBeenCalledOnce()
    const [url, body] = mockPost.mock.calls[0] as [string, Record<string, unknown>]
    expect(url).toBe('/api/task/task-123/complete')
    expect(body.action).toBe('approve')
    expect(body.opinion).toBe('ok')
    expect(body.data).toEqual({})
    expect(result).toEqual({ data: 'task-result' })
  })

  it('approve 分支：data 为空对象时仍带默认字段', async () => {
    const result = await taskHandle('task-456', 'approve')
    const [url, body] = mockPost.mock.calls[0] as [string, Record<string, unknown>]
    expect(url).toBe('/api/task/task-456/complete')
    expect(body.action).toBe('approve')
    expect(body.opinion).toBe('')
    expect(body.data).toEqual({})
    expect(result).toEqual({ data: 'task-result' })
  })

  it('reject 分支：action === "reject" -> POST /reject', async () => {
    const result = await taskHandle('task-789', 'reject', { opinion: 'bad' })
    expect(mockPost).toHaveBeenCalledOnce()
    const [url, body] = mockPost.mock.calls[0] as [string, Record<string, unknown>]
    expect(url).toBe('/api/task/task-789/reject')
    expect(body.action).toBe('reject')
    expect(body.opinion).toBe('bad')
    expect(body.data).toEqual({})
    expect(result).toEqual({ data: 'task-result' })
  })

  it('reject 分支：data 与默认字段合并，data 字段覆盖默认', async () => {
    const result = await taskHandle('task-abc', 'reject', { opinion: 'overridden', extra: 1 })
    const [url, body] = mockPost.mock.calls[0] as [string, Record<string, unknown>]
    expect(url).toBe('/api/task/task-abc/reject')
    expect(body.action).toBe('reject')
    expect(body.opinion).toBe('overridden') // 被 data 覆盖
    expect(body.extra).toBe(1)
    expect(body.data).toEqual({}) // 默认 data 仍在
    expect(result).toEqual({ data: 'task-result' })
  })

  it('approve 分支：空 data 时 opinion 默认为空字符串', async () => {
    await taskHandle('task-xyz', 'approve')
    const [, body] = mockPost.mock.calls[0] as [string, Record<string, unknown>]
    expect(body.opinion).toBe('')
    expect(body.action).toBe('approve')
  })

  it('reject 分支：空 data 时 opinion 默认为空字符串', async () => {
    await taskHandle('task-zzz', 'reject')
    const [, body] = mockPost.mock.calls[0] as [string, Record<string, unknown>]
    expect(body.opinion).toBe('')
    expect(body.action).toBe('reject')
  })
})
