// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * B7 死代码防复活守卫（2026-11-16 收尾项）。
 *
 * .debug-tag 样式与 rank.vue 里 formatScore 的「同值死分支」已经删除。
 * 这两类改动「看不见也能编译通过」——删除后若被无意或机器人重新加回，
 * 没有任何构建/类型检查会报错。用源码扫描把它们变成结构约束。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync, readdirSync } from 'node:fs'
import { join } from 'node:path'

const SRC = join(__dirname, '..')
const read = (rel: string) => readFileSync(join(SRC, rel), 'utf-8')

/** 递归收集 src 下所有样式与组件源码文件。 */
function collectSources(dir: string, out: string[] = []): string[] {
  for (const name of readdirSync(dir, { withFileTypes: true })) {
    const full = join(dir, name.name)
    if (name.isDirectory()) collectSources(full, out)
    else if (/\.(vue|css|scss)$/.test(name.name)) out.push(full)
  }
  return out
}

describe('B7 死代码防复活守卫', () => {
  it('源码中不存在 .debug-tag 样式规则', () => {
    const offenders = collectSources(SRC).filter((f) => readFileSync(f, 'utf-8').includes('.debug-tag'))
    expect(offenders, '.debug-tag 不得重新出现（调试标记，无业务用途）').toEqual([])
  })

  it('rank.vue 的 formatScore 仍是单一 return，无同值死分支', () => {
    const src = read('pages/rank/rank.vue')
    const m = src.match(/function formatScore\s*\([^)]*\)\s*:\s*string\s*\{([\s\S]*?)\n\s*\}/)
    expect(m, '应能定位到 formatScore 函数体').toBeTruthy()
    const body = m![1]
    // 注释允许保留意图说明；但不得出现任何分支关键字，且只能有一个 return。
    const code = body
      .split('\n')
      .map((line) => line.replace(/\/\/.*$/, ''))
      .join(' ')
      .replace(/\/\*[\s\S]*?\*\//g, '')
    expect(code, 'formatScore 不得包含 if/else/switch 等分支').not.toMatch(/\b(if|else|switch|case)\b/)
    expect((code.match(/\breturn\b/g) ?? []).length).toBe(1)
    expect(code).toContain('return String(v)')
  })
})
