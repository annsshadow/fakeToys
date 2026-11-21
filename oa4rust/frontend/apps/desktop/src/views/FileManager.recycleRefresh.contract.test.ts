// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const src = readFileSync(fileURLToPath(new URL('./FileManager.vue', import.meta.url)), 'utf8')
const template = src.slice(src.indexOf('<template>'), src.indexOf('</template>'))

function fnBody(start: string): string {
  const p = src.indexOf(start)
  if (p < 0) return ''
  let depth = 0
  let seen = false
  for (let i = src.indexOf('{', p); i < src.length; i++) {
    const ch = src[i]
    if (ch === '{') { depth++; seen = true }
    if (ch === '}') { depth--; if (seen && depth === 0) return src.slice(p, i + 1) }
  }
  return ''
}

function mutateSuccessCallback(): string {
  const body = fnBody('async function deleteFile')
  const p = body.indexOf('onSuccess: () => {')
  if (p < 0) return ''
  let depth = 0, end = -1
  for (let i = body.indexOf('{', p); i < body.length; i++) {
    if (body[i] === '{') depth++
    if (body[i] === '}') { depth--; if (depth === 0) { end = i + 1; break } }
  }
  return body.slice(p, end)
}

describe('FileManager R321 D1 删除后回收站刷新回显契约', () => {
  it('loadRecycleBin 通过 GET /api/recycle/list 读取回收站并写入 recycleFiles', () => {
    const body = fnBody('async function loadRecycleBin')
    expect(body).toContain("api.get('/api/recycle/list')")
    expect(body).toContain('recycleFiles.value')
  })

  it('删除 mutate onSuccess 成功后必须调用 loadRecycleBin 刷新回收站', () => {
    const body = fnBody('async function deleteFile')
    expect(body).toContain('deleteMutation.mutate(')
    const callback = mutateSuccessCallback()
    expect(callback).toContain('toast.success')
    expect(callback).toMatch(/loadRecycleBin\s*\(\s*\)/)
  })

  it('删除成功回调必须直接调用刷新（非注释/字符串），且顺序在成功 toast 前', () => {
    const callback = mutateSuccessCallback()
    expect(callback).not.toMatch(/\/\/[^\n]*loadRecycleBin/)
    const p = callback.indexOf('loadRecycleBin()')
    expect(p).toBeGreaterThan(-1)
    expect(p).toBeLessThan(callback.indexOf('toast.success'))
  })

  it('回收站列表绑定真实 recycleFiles 且保留刷新入口', () => {
    expect(template).toContain('data-testid="files-recycle-list"')
    expect(template).toContain('v-for="item in recycleFiles"')
    expect(template).toContain('@submit.prevent="loadRecycleBin"')
  })
})