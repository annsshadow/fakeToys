/**
 * 移动端业务 API 层单测：纯函数语义钉死。
 * （端点路径本身由 tests/contracts/mobile-endpoints.test.ts 的契约守卫覆盖，
 * 这里不重复钉 URL，只钉数据解析与请求体形状。）
 */
import { afterEach, describe, expect, it, vi } from 'vitest'
import { getApiBase, setApiBase } from './http'
import { fileApi, messageApi, messageConversationId, processApi } from './index'

/** 捕获 uni.request 的最小 stub（服务方法本身只关心 method/url/data/options）。 */
function installRequestCapture() {
  const calls: Array<{ method: string; url: string; data?: unknown }> = []
  vi.stubGlobal('uni', {
    request: (opts: { method: string; url: string; data?: unknown; success: (r: unknown) => void }) => {
      calls.push({ method: opts.method, url: opts.url, data: opts.data })
      opts.success({ statusCode: 200, data: { success: true } })
    },
    uploadFile: () => {},
  })
  return calls
}

describe('processApi approval bodies (真实审批引擎 /api/task/{id}/complete|reject)', () => {
  it('completeTask sends the engine-required action "approve" with data/opinion defaults', async () => {
    // 引擎缺 action 会 400；data 缺省 {} 也是引擎约定。
    const calls = installRequestCapture()
    await processApi.completeTask('t-1', { opinion: '同意', data: { x: 1 } })
    const call = calls[0]
    expect(call.method).toBe('POST')
    expect(call.url).toBe('/api/task/t-1/complete')
    expect(call.data).toEqual({ data: { x: 1 }, opinion: '同意', action: 'approve' })
  })

  it('completeTask without payload sends the full default body', async () => {
    const calls = installRequestCapture()
    await processApi.completeTask('t-2')
    expect(calls[0].data).toEqual({ data: {}, opinion: '', action: 'approve' })
  })

  it('rejectTask pins action "reject" on the reject endpoint', async () => {
    const calls = installRequestCapture()
    await processApi.rejectTask('t-3', { opinion: '材料不全' })
    expect(calls[0].url).toBe('/api/task/t-3/reject')
    expect(calls[0].data).toEqual({ data: {}, opinion: '材料不全', action: 'reject' })
  })

  afterEach(() => {
    vi.unstubAllGlobals()
  })
})

describe('messageApi.send (IM 会话归属)', () => {
  it('sends the plain conversationId key only', async () => {
    // 后端已修复「键名带引号」缺陷（docs/plans/2026-09-20-001 §6.7），
    // 只发普通键即可，不再需要双键兼容。
    const calls = installRequestCapture()
    await messageApi.send('conv-9', 'hi', 'alice')
    expect(calls[0].url).toBe('/api/message/assemble/communicate/im/msg')
    expect(calls[0].data).toMatchObject({
      conversationId: 'conv-9',
      content: 'hi',
      sender: 'alice',
      type: 'text',
    })
    expect(Object.keys(calls[0].data as Record<string, unknown>)).not.toContain('"conversationId"')
  })

  afterEach(() => {
    vi.unstubAllGlobals()
  })
})

describe('fileApi download URLs (原生 App / 小程序绝对地址)', () => {
  it('joins the configured apiBase so offline targets reach the backend', () => {
    setApiBase('http://10.0.0.8:5432/')
    expect(fileApi.fileDownloadUrl('f-1')).toBe('http://10.0.0.8:5432/api/file/assemble/control/file/f-1/download')
    expect(fileApi.attachmentDownloadUrl('a-1')).toBe('http://10.0.0.8:5432/api/attachment/a-1/download')
    setApiBase('')
    expect(fileApi.fileDownloadUrl('f-2')).toBe('/api/file/assemble/control/file/f-2/download')
    expect(getApiBase()).toBe('')
  })
})

describe('messageConversationId (IM 会话键解析)', () => {
  it('reads the plain conversationId key', () => {
    // 后端已修复「键名带引号」缺陷（见 docs/plans/2026-09-20-001 §6.7），
    // 现在统一按普通键 conversationId 归属消息。
    expect(messageConversationId({ conversationId: 'c-2' })).toBe('c-2')
  })

  it('ignores a legacy quoted key and still resolves the plain key', () => {
    expect(messageConversationId({ '"conversationId"': 'c-1', conversationId: 'c-2' })).toBe('c-2')
    expect(messageConversationId({ '"conversationId"': '', conversationId: 'c-2' })).toBe('c-2')
  })

  it('non-string values are ignored', () => {
    expect(messageConversationId({ '"conversationId"': 42, conversationId: 'ok' })).toBe('ok')
    expect(messageConversationId({ conversationId: 7 })).toBe('')
  })

  it('a row with no conversation attribution resolves to empty string', () => {
    expect(messageConversationId({ id: 'm-1', content: 'hi' })).toBe('')
    expect(messageConversationId({})).toBe('')
  })
})
