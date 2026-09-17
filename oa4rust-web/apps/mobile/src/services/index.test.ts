/**
 * 移动端业务 API 层单测：纯函数语义钉死。
 * （端点路径本身由 tests/contracts/mobile-endpoints.test.ts 的契约守卫覆盖，
 * 这里不重复钉 URL，只钉数据解析与请求体形状。）
 */
import { describe, expect, it } from 'vitest'
import { messageConversationId } from './index'

describe('messageConversationId (IM 会话键解析)', () => {
  it('prefers the quoted "conversationId" key emitted by the legacy generator', () => {
    // 后端生成 handler 把会话键序列化为带引号字面量（JSON 键本身含引号字符）——
    // IM 会话列表/历史页全靠它归属消息，丢了键就整条 IM 无会话。
    expect(messageConversationId({ '"conversationId"': 'c-1', conversationId: 'c-2' })).toBe('c-1')
  })

  it('falls back to the plain conversationId key', () => {
    expect(messageConversationId({ conversationId: 'c-plain' })).toBe('c-plain')
  })

  it('an empty quoted key falls through to the plain key', () => {
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
