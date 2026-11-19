// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'ProcessWork.vue'), 'utf8')

/** 提取 loadTaskInfo 函数体（到下一个函数开始前），避免命中文件其它位置的 expire 调用。 */
function extractLoadTaskInfo(src: string): string {
  const start = src.indexOf('async function loadTaskInfo(')
  if (start < 0) throw new Error('未找到 loadTaskInfo 定义')
  const end = src.indexOf('async function pressTask(', start)
  if (end < 0) throw new Error('未找到 loadTaskInfo 结束边界')
  return src.slice(start, end)
}

/** 提取 end_time → taskExpireText 的本地推导片段；缺失即业务失败。 */
function extractDerivation(fn: string): string {
  const match = fn.match(/const endTime[^\n]*\n[ \t]*taskExpireText\.value\s*=\s*[^\n]+/)
  if (!match) throw new Error('BUG-E：loadTaskInfo 未从 end_time 本地推导 taskExpireText')
  return match[0]
}

/** 在受控 taskInfo 上执行源码里的推导片段，返回真实算出的超时文本。 */
function deriveExpireText(fn: string, info: Record<string, unknown> | null): string {
  const snippet = extractDerivation(fn)
  const js = `const taskInfo = { value: __INFO__ }; const taskExpireText = {}; ${snippet}\nreturn taskExpireText.value;`
  return new Function('__INFO__', js)(info) as string
}

describe('BUG-E：详情加载误调写端点把 active 任务置 expired', () => {
  const fn = extractLoadTaskInfo(source)

  it('loadTaskInfo 不再请求 task/expire 写端点（该 GET 会 UPDATE task_status=expired）', () => {
    // 旧代码：settle(api.get('/api/processplatform/service/processing/task/expire/' + id))
    // 后端 task_id_expire 执行 UPDATE x_task SET task_status='expired'，导致随后 complete 400。
    // 去掉注释行后检查：不得有任何 api.get 命中 expire 路径（注释里说明为何不调用是允许的）。
    const code = fn.split('\n').filter((l) => !l.trimStart().startsWith('//')).join('\n')
    expect(code).not.toMatch(/api\.get\([^)]*task\/expire/)
    expect(code).not.toMatch(/expire\}/)  // 不得再从 expire 端点取数据
  })

  it('仍通过只读 task/{id} 端点加载任务详情', () => {
    expect(fn).toMatch(/processing\/task\/\$\{taskId\}/)
    expect(fn).toMatch(/taskInfo\.value/)
  })

  it('end_time 存在时超时文本取该时间值（本地推导，无网络请求）', () => {
    expect(deriveExpireText(fn, { end_time: '2026-11-20 10:30:00' })).toContain('2026-11-20 10:30:00')
  })

  it('end_time 缺失（空串/字段不存在/null）时超时文本为空', () => {
    expect(deriveExpireText(fn, {})).toBe('')
    expect(deriveExpireText(fn, { end_time: '' })).toBe('')
    expect(deriveExpireText(fn, null)).toBe('')
  })
})
