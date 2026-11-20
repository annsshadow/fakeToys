// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'Settings.vue'), 'utf8')

function persistBody(): string {
  const from = source.indexOf('async function persistConfigs() {')
  if (from < 0) return ''
  // 函数体内层闭合均缩进，列 0 的 "\n}" 即函数结尾。
  const end = source.indexOf('\n}', from)
  return source.slice(from, end < 0 ? undefined : end + 2)
}

describe('系统设置保存重入守卫（R267）', () => {
  it('persistConfigs 同步占用 saving 并在 finally 复位', () => {
    expect(source).toContain('const saving = ref(false)')

    const body = persistBody()
    expect(body).not.toBe('')
    const guardIdx = body.indexOf('if (saving.value) return')
    const occupyIdx = body.indexOf('saving.value = true')
    const finallyIdx = body.indexOf('finally {')
    const resetIdx = body.indexOf('saving.value = false', finallyIdx < 0 ? 0 : finallyIdx)
    // 守卫与占用必须在请求之前同步执行；复位必须在 finally，且只出现一次。
    expect(guardIdx).toBeGreaterThan(-1)
    expect(occupyIdx).toBeGreaterThan(guardIdx)
    expect(finallyIdx).toBeGreaterThan(occupyIdx)
    expect(resetIdx).toBeGreaterThan(finallyIdx)
    expect(body.match(/saving\.value = false/g)?.length ?? 0).toBe(1)
  })

  it('三个保存按钮全部绑定 disabled 到 saving', () => {
    const buttons = source.match(/<button[^>]*class="save-btn"[^>]*>/g) ?? []
    expect(buttons.length).toBe(3)
    for (const btn of buttons) {
      expect(btn).toContain(':disabled="saving"')
    }
  })
})
