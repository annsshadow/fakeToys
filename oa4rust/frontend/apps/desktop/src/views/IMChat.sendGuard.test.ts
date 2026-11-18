// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * R263：发送按钮的「发送中」守卫必须覆盖请求在途期。
 * sendLoading 在 mutate() 之后若被同步复位，请求仍在途而按钮已解禁，
 * 快速连点会重复发送并留下多个乐观气泡。node 环境无法挂载 .vue，
 * 因此用源码契约钉死「置位在 sendMessage、复位在 onSettled」的时序。
 */
describe('IMChat 发送在途守卫（R263）', () => {
  const src = readFileSync(resolve(import.meta.dirname, 'IMChat.vue'), 'utf8')
  const mutationStart = src.indexOf('const sendMutation = useMutation({')
  const sendStart = src.indexOf('function sendMessage()')

  it('sendMessage 置位守卫且不在 mutate 后同步复位 sendLoading', () => {
    expect(sendStart).toBeGreaterThan(mutationStart)
    const sendBlock = src.slice(sendStart, sendStart + 400)
    expect(sendBlock).toContain('if (!text || sendLoading.value) return')
    expect(sendBlock).toContain('sendLoading.value = true')
    // mutate 之后直到函数结束，不应再出现 sendLoading.value = false（那会让守卫形同虚设）
    const afterMutate = sendBlock.slice(sendBlock.indexOf('sendMutation.mutate(text)'))
    expect(afterMutate).not.toContain('sendLoading.value = false')
  })

  it('sendLoading 在 onSettled 中复位，覆盖成功与失败两条路径', () => {
    const mutationBlock = src.slice(mutationStart, sendStart)
    const settledAt = mutationBlock.indexOf('onSettled:')
    expect(settledAt).toBeGreaterThan(-1)
    const settled = mutationBlock.slice(settledAt)
    expect(settled).toContain('sendLoading.value = false')
  })
})
