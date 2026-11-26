// R302 DR-06 显示层契约：原始会话键不得直接可见
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

const im = readFileSync(resolve(__dirname, 'IMChat.vue'), 'utf8')

describe('R302 DR-06 IM 会话显示可读化', () => {
  it('渲染层使用 displayConversationName/displayConversationPreview 而非原始 name/lastMessage', () => {
    expect(im).toContain('function displayConversationName')
    expect(im).toContain('function displayConversationPreview')
    expect(im).toContain('{{ displayConversationName(conv) }}')
    expect(im).toContain('{{ displayConversationPreview(conv) }}')
  })
})
