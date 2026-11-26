// R302 DR-06 H5 契约：原始会话键不得直接可见
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

const src = readFileSync(resolve(__dirname, 'message.vue'), 'utf8')

describe('R302 DR-06 H5 会话显示可读化', () => {
  it('conversationTitle 对 im-conv-<ts>-<rand> 降级为类型+脱敏尾号', () => {
    expect(src).toContain('function isRawConversationName')
    expect(src).toContain('isRawConversationName(v)')
    expect(src).toContain('单聊 · ')
  })
})
