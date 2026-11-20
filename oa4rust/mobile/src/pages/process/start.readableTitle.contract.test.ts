// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// t33 RED→GREEN：H5 可发起流程列表优先显示业务名称，ID 折叠为副文本。
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(__dirname, './start.vue'), 'utf8')
const script = source.match(/<script setup[^>]*>([\s\S]*?)<\/script>/)?.[1] ?? ''

describe('H5 process/start 可发起流程标题契约', () => {
  it('流程选项结构包含独立 name 与 id', () => {
    expect(script).toMatch(/Array<\{ id: string; name: string \}>/)
    expect(script).toMatch(/name: String\(row\.name \?\? ''\)/)
    expect(script).toMatch(/id: String\(row\.id \?\? ''\)/)
    expect(script).not.toMatch(/name: String\(row\.name \?\? row\.id \?\? ''\)/)
  })

  it('流程条目用可读名称作主文本，ID 仅作副文本', () => {
    expect(source).toMatch(/data-testid="h5-start-process-item"/)
    expect(source).toMatch(/data-testid="h5-start-process-name"[\s\S]*?\{\{ p\.name \|\| '未命名流程' \}\}/)
    expect(source).toMatch(/data-testid="h5-start-process-id"[\s\S]*?\{\{ shortProcessId\(p\.id\) \}\}/)
    expect(source).not.toMatch(/\{\{ p\.name \|\| p\.id \}\}/)
    expect(script).toMatch(/function shortProcessId\(/)
  })

  it('空 ID 行仍被过滤，保持原有加载语义', () => {
    expect(script).toMatch(/\.filter\(\(p\) => p\.id\)/)
  })
})
