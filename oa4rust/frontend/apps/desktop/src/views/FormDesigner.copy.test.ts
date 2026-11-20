// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'FormDesigner.vue'), 'utf8')

describe('FormDesigner 复制不再误报成功', () => {
  it('复制统一走 copyText，真实 await 并按成败提示，不再裸调 writeText 且无条件成功 toast', () => {
    // 修复前：模板/函数里裸调 navigator.clipboard.writeText 且无条件 showToast 成功，
    // 复制失败（权限 / CSP / 非安全上下文）时用户被误报成功，并产生 unhandledrejection。
    expect(source).toContain("import { copyText, toast } from '../utils/toast'")
    expect(source).not.toContain('navigator.clipboard.writeText')
    expect(source).not.toContain("showToast('已复制','success')")
    expect(source).not.toContain("showToast('已复制到剪贴板', 'success')")
    // 模板两处按钮改为调用 copyText
    expect(source).toContain('@click="copyText(jsonSchemaText)"')
    expect(source).toContain('@click="copyText(fieldDocsText)"')
    // 两处 helper 改为 async + await
    expect(source).toContain('async function copyExportResult()')
    expect(source).toContain('await copyText(exportResult.value)')
    expect(source).toContain('async function copyToClipboard(text: string)')
    expect(source).toContain('await copyText(text)')
  })
})
