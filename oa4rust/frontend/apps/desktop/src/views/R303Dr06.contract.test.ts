// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// t37 R303Dr06 汇总契约：两处主标题均不得出现原始 work-/im-conv- 串。
import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { displayConversationName, displayTodoTitle, foldRawKeySubtext, isRawConversationName, isRawProcessKey } from '../utils/processTitle'

const here = path.dirname(fileURLToPath(import.meta.url))
const dash = readFileSync(path.join(here, 'Dashboard.vue'), 'utf8')
const im = readFileSync(path.join(here, 'IMChat.vue'), 'utf8')

describe('R302-DR06-R 原始键可读性汇总', () => {
  const rawWork = 'work-1795102580411-71d52ed36d4d4c0e'
  const rawConv = 'im-conv-1795102580411-jf05'
  it('业务名与真人姓名原样保留', () => {
    expect(displayTodoTitle('费用报销', rawWork)).toBe('费用报销')
    expect(displayConversationName('李娜', 'single', rawConv)).toBe('李娜')
  })
  it('原始键不进入主标题且折叠为副文本/稳定序号', () => {
    const todo = displayTodoTitle(rawWork, undefined)
    const conv = displayConversationName(rawConv, 'single', rawConv)
    expect(isRawProcessKey(todo)).toBe(false)
    expect(isRawConversationName(conv)).toBe(false)
    expect(todo).not.toContain('work-')
    expect(conv).not.toContain('im-conv-')
    expect(conv).not.toContain('jf05')
    expect(foldRawKeySubtext(rawWork)).toContain('流程 · ')
  })
  it('SFC 主标题节点由共享 helper 渲染，保留稳定 testid', () => {
    expect(dash).toContain('displayTodoTitle(')
    expect(dash).toContain('data-testid="dashboard-todo-list"')
    expect(im).toContain('displayConversationName(')
    expect(im).toContain("'im-conv-' + (convIndex + 1)")
  })
})
