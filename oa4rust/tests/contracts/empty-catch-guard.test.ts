// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later

import { readdirSync, readFileSync, statSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

/**
 * 空 catch 守卫：`catch { ... }` 若为空体（无语句）必须带解释注释，说明为何吞掉该错误。
 * 历史教训（优化轮24）：desktop 曾有 10 处 `catch {}` 静默吞错（批量导入假成功、
 * 删除失败静默等），逐处改为「逐项 await + 成败计数 / toast 提示」。
 * 本守卫把「纯空 catch 且无注释」的写法钉死不得复发——有注释的空 catch 视为
 * 有意的 best-effort 吞错（如格式解析失败保留原值），合法。
 */

function listFiles(dir: string, out: string[] = []): string[] {
  for (const name of readdirSync(dir)) {
    if (name === 'node_modules' || name === 'dist' || name === '.git') continue
    const p = resolve(dir, name)
    const st = statSync(p)
    if (st.isDirectory()) listFiles(p, out)
    else if (/\.(vue|ts)$/.test(name)) out.push(p)
  }
  return out
}

/** 单行开闭的 catch（如 `} catch {}`）：{ 与 } 同行，捕获其中 body（group 1，不含 }）。 */
const RE_SINGLE_CATCH = /catch\s*(?:\([^)]*\))?\s*\{([^}]*)\}\s*$/
/** 多行 catch 头：行以 `catch (...) {` 或 `catch {` 结尾（} 在后续行）。 */
const RE_MULTI_OPEN = /catch\s*(?:\([^)]*\))?\s*\{$/

function emptyCatches(source: string): number[] {
  const lines = source.split('\n')
  const bad: number[] = []
  let i = 0
  while (i < lines.length) {
    const t = lines[i].trim()
    const single = t.match(RE_SINGLE_CATCH)
    if (single && !RE_MULTI_OPEN.test(t)) {
      // 单行开闭 catch：body 为空且无注释 = 静默吞错违规；空但有注释 = 有意 best-effort，放行
      if (single[1].trim() === '') bad.push(i + 1)
      i++
      continue
    }
    if (RE_MULTI_OPEN.test(t)) {
      // 多行：catch 头行以 { 结尾，向后按大括号深度找闭合 }
      let depth = 1
      let j = i + 1
      while (j < lines.length && depth > 0) {
        for (const ch of lines[j]) {
          if (ch === '{') depth++
          else if (ch === '}') depth--
        }
        if (depth > 0) j++
        else break
      }
      const body = lines.slice(i + 1, j).map((x) => x.trim())
      const hasStmt = body.some((x) => x !== '' && !x.startsWith('//') && !x.startsWith('*') && !x.startsWith('/*'))
      const isCommentOnly = body.some((x) => x.startsWith('//') || x.startsWith('/*') || x.startsWith('*'))
      if (!hasStmt && !isCommentOnly) bad.push(i + 1)
      i = j + 1
    } else {
      i++
    }
  }
  return bad
}

const desktopSrcRoot = resolve(import.meta.dirname, '../../frontend/apps/desktop/src')
const mobileSrcRoot = resolve(import.meta.dirname, '../../mobile/src')

describe('空 catch 必须带解释注释（不得静默吞错）', () => {
  it('全部 .vue/.ts 源文件无「纯空且无注释」的 catch 块', () => {
    const files = [...listFiles(desktopSrcRoot), ...listFiles(mobileSrcRoot)]
    expect(files.length).toBeGreaterThan(0)
    const offenders: string[] = []
    for (const f of files) {
      if (/\.test\.ts$/.test(f)) continue
      const src = readFileSync(f, 'utf8')
      for (const ln of emptyCatches(src)) offenders.push(`${f}:${ln}`)
    }
    expect(offenders, `发现无注释的空 catch（须补注释或改为实际处理）:\n${offenders.join('\n')}`).toEqual([])
  })
})
