// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// t37 RED→GREEN：IM 机器会话键不能当姓名/主标题，摘要必须是真实 lastMessage。
import { readFileSync } from 'node:fs'
import path from 'node:path'
import vm from 'node:vm'
import { fileURLToPath } from 'node:url'
import { transformWithEsbuild } from 'vite'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const shared = readFileSync(path.join(here, '..', 'utils', 'processTitle.ts'), 'utf8')
const im = readFileSync(path.join(here, 'IMChat.vue'), 'utf8')
const machineName = 'im-conv-1795102580411-jf05'

async function api(): Promise<Record<string, any>> {
  const snippet = (await transformWithEsbuild(shared, 'processTitle.ts', { loader: 'ts', format: 'cjs', target: 'es2022' })).code
  const sandbox: Record<string, any> = { exports: {}, module: { exports: {} } }
  vm.createContext(sandbox)
  new vm.Script(snippet).runInContext(sandbox)
  return sandbox.module.exports
}
const helpers = await api()

describe('IMChat 会话名可读性（R302-DR06-R）', () => {
  it('RED：断言旧随机尾号降级逻辑已移除', () => {
    expect(im).not.toContain('const tail = name.slice(-4)')
    expect(im).not.toContain('data-testid="im-chat-name">{{ selectedChat.name }}')
  })
  it('GREEN：机器键降级为稳定序号，真人姓名与真实摘要原样展示', () => {
    const id = 'im-conv-1795102580411-jf05'
    const name = helpers.displayConversationName(machineName, 'single', id)
    expect(name).toMatch(/^单聊 \d+$/)
    expect(name).not.toContain('jf05')
    expect(name).not.toContain(machineName)
    expect(helpers.displayConversationName(machineName, 'single', id)).toBe(name)
    expect(helpers.displayConversationName('张伟', 'single', id)).toBe('张伟')
    expect(helpers.displayConversationPreview('明天 9 点开会')).toBe('明天 9 点开会')
    expect(helpers.displayConversationPreview(machineName)).toBe('暂无消息，点击开始对话')
    expect(im).toContain('displayConversationName')
    expect(im).toContain('displayConversationPreview')
    expect(im).toContain('data-testid="im-chat-name"')
  })
})
