// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R71 验真页记录时间本地化契约（test-first RED）。
 * 记录时间行必须通过 fmtVerifyTime 本地化 y-mm-dd hh:mi，不得裸直出 UTC RFC3339 ISO；
 * 非法 ISO 回退原串。先 RED 再实现；令牌锚点 v1/v3-r3/r6 不变，禁新增 r71 视觉令牌。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync } from 'node:fs'
import { join } from 'node:path'

const srcRoot = join(__dirname, '..')
const readSrc = (p: string) => readFileSync(join(srcRoot, p), 'utf-8')

function blockAfter(text: string, keyword: string): { start: number; end: number; text: string } | null {
  const k = text.indexOf(keyword)
  if (k < 0) return null
  const open = text.indexOf('{', k)
  if (open < 0) return null
  let depth = 0
  for (let i = open; i < text.length; i++) {
    const ch = text[i]
    if (ch === '{') depth++
    else if (ch === '}') {
      depth--
      if (depth === 0) return { start: open, end: i, text: text.slice(open, i + 1) }
    }
  }
  return null
}

describe('R71 验真页记录时间本地化契约', () => {
  const verify = readSrc('pages/verify/verify.vue')

  it('fmtVerifyTime 存在：本地日历 y-mm-dd hh:mi + 非法 ISO 回退原串', () => {
    const fn = blockAfter(verify, 'function fmtVerifyTime')
    expect(fn, 'verify.vue 必须定义 fmtVerifyTime').not.toBeNull()
    const b = fn!.text
    expect(b, 'fmtVerifyTime 必须解析 ISO').toContain('new Date(iso)')
    expect(b, 'fmtVerifyTime 必须有 NaN 守卫').toContain('Number.isNaN(d.getTime())')
    expect(b, '非法 ISO 必须回退原串').toContain('return iso')
    expect(b, '必须本地日历 getFullYear').toContain('getFullYear')
    expect(b, '必须本地月份 getMonth').toContain('getMonth() + 1')
    expect(b, '必须补零 padStart').toContain("padStart(2, '0')")
  })

  it('记录时间行绑定 fmtVerifyTime，不得裸直出 created_at', () => {
    const lines = verify.split(String.fromCharCode(10))
    const recLine = lines.find(l => l.indexOf('记录时间') > -1 && l.indexOf('class="kv"') > -1)
    expect(recLine, '必须存在「记录时间」kv 行').toBeTruthy()
    expect(recLine, '记录时间必须经 fmtVerifyTime 本地化').toContain('fmtVerifyTime(replayInfo.created_at)')
    expect(recLine, '记录时间不得裸直出 UTC ISO').not.toContain('{{ replayInfo.created_at }}')
  })

  it('kv 结构保留：muted 标签 + mono class + 哈希行同构', () => {
    const lines = verify.split(String.fromCharCode(10))
    const recLine = lines.find(l => l.indexOf('记录时间') > -1 && l.indexOf('class="kv"') > -1)
    expect(recLine, 'kv 容器 class 必须保留').toContain('class="kv"')
    expect(recLine, 'muted 标签「记录时间」必须保留').toContain('class="muted">记录时间')
    expect(recLine, 'mono class 必须保留').toContain('class="mono"')
    expect(verify, '服务端哈希行 pretty(expected_hash) 必须保留').toContain('pretty(replayInfo.expected_hash)')
  })

  it('不变量：fetchReplay 守卫/finally/清结论 + buildSnapshotCard 保留', () => {
    const fn = blockAfter(verify, 'async function fetchReplay')
    expect(fn, 'fetchReplay 必须存在').not.toBeNull()
    const b = fn!.text
    expect(b, '在途守卫必须保留').toContain('if (fetching.value) return')
    expect(b, 'finally 复位必须保留').toContain('fetching.value = false')
    expect(b, '失败清结论必须保留').toContain('outcome.value = null')
    expect(verify, 'buildSnapshotCard 必须保留').toContain('v-for="s in buildSnapshotCard"')
  })

  it('令牌三锚点保留，业务源码无 r71 视觉令牌', () => {
    const tokens = readSrc('styles/tokens.css')
    expect(tokens).toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens).toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens).toContain('--lyp-r6-version: lyp-tokens-r6')
    expect(verify + tokens, '不得新增 r71 视觉令牌').not.toContain('--r71-')
  })
})
