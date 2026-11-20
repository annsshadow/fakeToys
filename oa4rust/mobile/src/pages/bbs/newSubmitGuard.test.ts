// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'new.vue'), 'utf8')

function submitSource(): string {
  const start = source.indexOf('async function submit()')
  const end = source.indexOf('onLoad(async', start)
  expect(start).toBeGreaterThan(-1)
  expect(end).toBeGreaterThan(start)
  return source.slice(start, end)
}

describe('移动端发帖防重复提交（R249）', () => {
  it('成功后到跳转的延时窗口内保持提交守卫，防止重复发帖', () => {
    const submit = submitSource()
    const timerStart = submit.indexOf('setTimeout(() => {')
    expect(timerStart).toBeGreaterThan(-1)
    const timerEnd = submit.indexOf('}, 600)', timerStart)
    expect(timerEnd).toBeGreaterThan(timerStart)
    // 守卫释放必须发生在延时回调内部（跳转那一刻），不能在成功响应后立即释放，
    // 否则 600ms 窗口内用户可再次点「发布主题」触发第二次 topicCreate。
    const timerCallback = submit.slice(timerStart, timerEnd)
    expect(timerCallback).toContain('submitting.value = false')
  })

  it('发布失败后立即释放守卫，允许用户重试', () => {
    const submit = submitSource()
    const match = submit.match(/\} catch \{\s*([\s\S]*?)\n {2}\}/)
    expect(match).not.toBeNull()
    expect(match?.[1]).toContain('submitting.value = false')
  })

  it('不存在 finally 中无条件释放守卫（会提前打开重复提交窗口）', () => {
    const submit = submitSource()
    const match = submit.match(/\n {2}\} finally \{([\s\S]*?)\n {2}\}/)
    if (match) {
      // 允许带条件的释放（如 if (!ok) 后释放），但不允许无条件独立成行释放。
      const unconditional = match[1].split('\n').some((line) => line.trim() === 'submitting.value = false')
      expect(unconditional).toBe(false)
    }
  })
})
