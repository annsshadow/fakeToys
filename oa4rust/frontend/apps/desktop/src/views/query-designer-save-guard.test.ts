// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

function read(rel: string): string {
  return readFileSync(resolve(import.meta.dirname, rel), 'utf8')
}

const src = read('./QueryDesigner.vue')

// 按花括号配对切出 saveQuery 函数体，避免断言误命中其他函数
function saveQueryBody(source: string): string {
  const start = source.indexOf('async function saveQuery() {')
  if (start < 0) return ''
  let depth = 0
  let opened = false
  for (let i = start; i < source.length; i++) {
    const ch = source[i]
    if (ch === '{') {
      depth++
      opened = true
    } else if (ch === '}') {
      depth--
      if (opened && depth === 0) return source.slice(start, i + 1)
    }
  }
  return ''
}

describe('QueryDesigner 保存查询在途守卫（新建走非幂等 POST create，连点会重复建单）', () => {
  it('定义 querySaving 在途标志', () => {
    expect(src).toContain('const querySaving = ref(false)')
  })

  it('saveQuery 开头有在途守卫', () => {
    const body = saveQueryBody(src)
    expect(body).not.toBe('')
    expect(body).toContain('if (querySaving.value) return')
  })

  it('saveQuery 进入请求前置在途标志', () => {
    expect(saveQueryBody(src)).toContain('querySaving.value = true')
  })

  it('saveQuery finally 复位在途标志', () => {
    const body = saveQueryBody(src)
    const finallyIdx = body.lastIndexOf('finally {')
    expect(finallyIdx).toBeGreaterThan(-1)
    expect(body.slice(finallyIdx)).toContain('querySaving.value = false')
  })

  it('保存按钮在途期间禁用', () => {
    expect(src).toContain(':disabled="!mform.name || querySaving"')
  })
})
