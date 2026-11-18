// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// start.submitGuard.test.ts: 源码级钉住发起流程 submit 的在途守卫，防止连点重复发起流程实例。
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const src = readFileSync(fileURLToPath(new URL('./start.vue', import.meta.url)), 'utf8')

function submitSource(): string {
  const start = src.indexOf('async function submit()')
  const end = src.indexOf('\nfunction onFieldInput', start)
  return src.slice(start, end >= 0 ? end : undefined)
}

describe('发起流程防重复提交 R261', () => {
  it('submit 开头必须有在途守卫防止连点重复发起流程', () => {
    const submit = submitSource()
    expect(submit).toContain('if (submitting.value) return')
  })

  it('守卫必须先于提交位置与首次 await', () => {
    const submit = submitSource()
    const guard = submit.indexOf('if (submitting.value) return')
    const setTrue = submit.indexOf('submitting.value = true')
    const firstAwait = submit.indexOf('await ')
    const startWork = submit.indexOf('processApi.startWork(')
    expect(guard).toBeGreaterThanOrEqual(0)
    expect(setTrue).toBeGreaterThanOrEqual(0)
    expect(firstAwait).toBeGreaterThanOrEqual(0)
    expect(startWork).toBeGreaterThanOrEqual(0)
    expect(guard).toBeLessThan(setTrue)
    expect(guard).toBeLessThan(firstAwait)
    expect(guard).toBeLessThan(startWork)
  })
})
