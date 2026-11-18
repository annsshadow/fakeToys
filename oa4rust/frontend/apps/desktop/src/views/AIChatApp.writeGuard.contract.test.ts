// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R268 AIChat 写操作在途守卫契约测试。
 *
 * createNewChat / addMcp 均为直接写服务端的新建操作，按钮在请求在途期间仍可点击：
 *   - 双击「+ 新对话」会 POST 两次，左侧列表出现两个空对话；
 *   - 双击 MCP「保存」会 POST 两次，创建两条同名服务。
 * 这类写操作必须在 await 前占住守卫并在 finally 复位，模板同步禁用按钮。
 * 源码级契约扫描函数体与按钮开始标签，钉死守卫与请求/复位的相对顺序。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const source = readFileSync(resolve(import.meta.dirname, 'AIChatApp.vue'), 'utf8')

function functionBody(name: string): string {
  const start = source.indexOf(`async function ${name}()`)
  expect(start, `未找到 async function ${name}`).toBeGreaterThan(-1)
  const open = source.indexOf('{', start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === '{') depth++
    if (source[i] === '}' && --depth === 0) return source.slice(open, i + 1)
  }
  throw new Error(`${name} 函数体未闭合`)
}

function buttonTagWithClass(cls: string): string {
  const re = new RegExp('<button\\b[^>]*class="[^"]*\\b' + cls + '\\b[^"]*"[^>]*>')
  const match = source.match(re)
  expect(match, `未找到 button.${cls}`).not.toBeNull()
  return match![0]
}

describe('AIChat 写操作在途守卫（R268）', () => {
  it('createNewChat 在 POST 前占住 creatingChat 守卫并在 finally 复位', () => {
    const body = functionBody('createNewChat')
    const guard = body.indexOf('if (creatingChat.value) return')
    const acquire = body.indexOf('creatingChat.value = true')
    const post = body.indexOf("api.post('/api/ai_assemble_control/chat/completion'")
    const release = body.indexOf('creatingChat.value = false')
    expect(guard, '缺少在途守卫').toBeGreaterThan(-1)
    expect(acquire, '必须有占位赋值').toBeGreaterThan(guard)
    expect(post, 'POST 必须在占位之后').toBeGreaterThan(acquire)
    expect(release, '必须在 finally 复位').toBeGreaterThan(post)
    expect(body.slice(release - 40, release)).toContain('finally')
  })

  it('新建对话按钮在在途期间禁用', () => {
    expect(buttonTagWithClass('btn-new')).toMatch(/:disabled="creatingChat"/)
  })

  it('addMcp 在 POST 前占住 savingMcp 守卫并在 finally 复位', () => {
    const body = functionBody('addMcp')
    const guard = body.indexOf('if (savingMcp.value) return')
    const acquire = body.indexOf('savingMcp.value = true')
    const post = body.indexOf("api.post('/api/ai_assemble_control/config/create/mcp'")
    const release = body.indexOf('savingMcp.value = false')
    expect(guard, '缺少在途守卫').toBeGreaterThan(-1)
    expect(acquire, '必须有占位赋值').toBeGreaterThan(guard)
    expect(post, 'POST 必须在占位之后').toBeGreaterThan(acquire)
    expect(release, '必须在 finally 复位').toBeGreaterThan(post)
    expect(body.slice(release - 40, release)).toContain('finally')
  })

  it('MCP 保存按钮在在途期间禁用', () => {
    const tag = source.match(/<button\b[^>]*@click="addMcp"[^>]*>/)?.[0]
    expect(tag, '未找到 MCP 保存按钮').toBeDefined()
    expect(tag).toMatch(/:disabled="savingMcp"/)
  })
})
