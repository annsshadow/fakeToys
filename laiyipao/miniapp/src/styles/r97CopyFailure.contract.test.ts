// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * R97 两处复制失败反馈补漏契约（test-first RED）。
 * 真实读 laiyipao/miniapp/src/pages/me/me.vue 与 battles/battles.vue，
 * 用大括号配对解析 uni.setClipboardData 参数对象，禁止内嵌实现副本。
 * 缺口：copyUserId / copyHash 只有 success 回调、缺 fail 回调，
 * 剪贴板失败时静默吞错（admin UsersView.vue:462-469 为权威口径）。
 */
import { describe, it, expect } from 'vitest'
import { readFileSync, readdirSync, statSync } from 'node:fs'
import { fileURLToPath } from 'node:url'
import { dirname, join, relative } from 'node:path'

const dir = dirname(fileURLToPath(import.meta.url))
const srcRoot = join(dir, '..')

function read(rel: string): string {
  return readFileSync(join(srcRoot, rel), 'utf8')
}

/** 从 anchor 之后的首个「{」做大括号配对，返回完整对象/函数体文本。 */
function blockAfter(text: string, anchor: string): string {
  const at = text.indexOf(anchor)
  expect(at, '锚点缺失: ' + anchor).toBeGreaterThan(-1)
  const open = text.indexOf('{', at)
  expect(open, '锚点后无起始大括号: ' + anchor).toBeGreaterThan(-1)
  let depth = 0
  for (let i = open; i < text.length; i++) {
    const ch: string = text.charAt(i)
    if (ch === '{') depth++
    else if (ch === '}') {
      depth--
      if (depth === 0) return text.slice(open, i + 1)
    }
  }
  throw new Error('大括号未闭合: ' + anchor)
}

/** 提取指定复制 handler 内 uni.setClipboardData 的参数对象文本。 */
function clipParams(src: string, fnAnchor: string): string {
  const body = blockAfter(src, fnAnchor)
  return blockAfter(body, 'uni.setClipboardData')
}

function walk(root: string): string[] {
  const out: string[] = []
  for (const name of readdirSync(root)) {
    const p = join(root, name)
    if (statSync(p).isDirectory()) out.push(...walk(p))
    else if ((name.endsWith('.vue') || name.endsWith('.ts') || name.endsWith('.css')) && !name.includes('.test.')) out.push(p)
  }
  return out
}

describe('R97 两处复制失败反馈补漏契约', () => {
  const me = read('pages/me/me.vue')
  const battles = read('pages/battles/battles.vue')
  const meParams = (): string => clipParams(me, 'function copyUserId()')
  const battleParams = (): string => clipParams(battles, 'function copyHash(hash: string)')

  it('it1 copyUserId 的 setClipboardData 参数含 fail 回调（缺口，RED 失败）', () => {
    expect(meParams(), 'copyUserId 须补 fail 回调').toContain('fail:')
  })

  it('it2 copyHash 的 setClipboardData 参数含 fail 回调（缺口，RED 失败）', () => {
    expect(battleParams(), 'copyHash 须补 fail 回调').toContain('fail:')
  })

  it('it3 两处 fail 文案均为权威口径「复制失败，请手动选择复制」（缺口，RED 失败）', () => {
    const FAIL = '复制失败，请手动选择复制'
    expect(meParams(), 'copyUserId fail 文案').toContain(FAIL)
    expect(battleParams(), 'copyHash fail 文案').toContain(FAIL)
  })

  it('it4 两处 success 路径文案与图标不变量保留（RED 通过）', () => {
    expect(meParams(), 'copyUserId success 文案保留').toContain('已复制用户 ID')
    expect(meParams(), 'copyUserId success 图标保留').toContain("icon: 'success'")
    expect(battleParams(), 'copyHash success 文案保留').toContain('已复制回放哈希')
    expect(battleParams(), 'copyHash success 图标保留').toContain("icon: 'success'")
  })

  it('it5 令牌三锚点不变量保留（RED 通过）', () => {
    const tokens = read('styles/tokens.css')
    expect(tokens, 'v1 锚点').toContain('--lyp-token-version: lyp-tokens-v1')
    expect(tokens, 'r3 锚点').toContain('--lyp-r3-version: lyp-tokens-v3-r3')
    expect(tokens, 'r6 锚点').toContain('--lyp-r6-version: lyp-tokens-r6')
  })

  it('it6 业务源码零 --r97- 私有令牌且 share.ts 分享纯函数不动（RED 通过）', () => {
    const bad: string[] = []
    for (const f of walk(srcRoot)) {
      if (readFileSync(f, 'utf8').includes('--r97-')) bad.push(relative(srcRoot, f))
    }
    expect(bad, '业务源码不得出现 --r97-* 令牌: ' + bad.join(', ')).toEqual([])
    const share = read('utils/share.ts')
    expect(share, 'indexShare 保留').toContain('export function indexShare()')
    expect(share, 'rankShare 保留').toContain('export function rankShare(')
    expect(share, 'battleShare 保留').toContain('export function battleShare(')
  })
})
