// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'ProcessDesigner.vue'), 'utf8')

describe('ProcessDesigner 复制不再误报成功', () => {
  it('复制统一走 copyText，真实 await 并按成败提示，不再裸调 writeText', () => {
    // 修复前：模板/函数里裸调 navigator.clipboard.writeText 无 catch，
    // 复制失败（权限 / CSP / 非安全上下文）时静默无提示，并产生 unhandledrejection。
    expect(source).toContain("import { copyText, toast } from '../utils/toast'")
    expect(source).not.toContain('navigator.clipboard.writeText')
    // 模板按钮改为调用 copyText
    expect(source).toContain('@click="copyText(exportFlowVars())"')
    // 三处 helper 改为 async + await
    expect(source).toContain('async function copySvg()')
    expect(source).toContain('await copyText(svg)')
    expect(source).toContain('async function copyExportJson()')
    expect(source).toContain('await copyText(exportJson())')
  })
})
