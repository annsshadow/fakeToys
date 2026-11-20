// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * DR-02 回归契约：工作台「待审批」统计卡必须与「我的待办」列表同源。
 * 历史 bug：统计卡取进程计数接口（返回全 0），列表取 task/list/my/paging（真实 5 条），
 * 同屏出现「待审批 0 / 待办 5 条」的自相矛盾。此处把「同源」编码为可执行断言。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'Dashboard.vue'), 'utf8')
const script = source.slice(source.indexOf('<script setup'), source.indexOf('</script>'))

describe('DR-02 待审批统计与待办列表同源', () => {
  it('「待审批」卡的取值来自分页待办计数，而不是进程统计接口的第 0 项', () => {
    const idx = script.indexOf("label: '待审批'")
    expect(idx, '缺少「待审批」统计卡').toBeGreaterThan(-1)
    const card = script.slice(idx, idx + 240)
    expect(card, '待审批卡必须与 pendingItems/approvalCount 同源').toMatch(
      /value:\s*(?:approvalCount\.value|pendingItems\.value\.length)/,
    )
    expect(card, '待审批卡不得再用 statCount(0,...) 进程计数').not.toContain('statCount(0')
  })

  it('存在从分页待办派生的 approvalCount，加载/失败时显示占位而非硬编码 0', () => {
    const idx = script.indexOf('const approvalCount')
    expect(idx, '缺少 approvalCount 派生计数').toBeGreaterThan(-1)
    const decl = script.slice(idx, idx + 280)
    expect(decl).toContain('pendingItems.value.length')
    expect(decl, '取数失败/加载中必须返回 null 占位').toContain('? null')
  })

  it('待办列表仍来自真实分页接口（禁止假数据替代）', () => {
    expect(script).toContain('task/list/my/paging/1/size/20')
  })
})
