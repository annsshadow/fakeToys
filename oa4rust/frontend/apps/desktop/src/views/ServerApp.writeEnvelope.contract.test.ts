// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R387 契约：PC ServerApp.vue console 五个写分支在 HTTP200 + type:error，或 200 + type:success
 * 但权威 cleared/sent/saved/deleted:false 时，必须 execError 透传后端 message、不得写
 * '控制台操作已提交' 误报成功。P1-B：confirmMsg 未 import 导致 stopServer/cacheClear/deployDelete
 * 三处高危守卫 ReferenceError，契约做源码级 import 断言。
 * type 判定为主：type:success（或 type 缺省且 success 非 false）为成功前提；
 * 传 flagKey 时额外要求 data[flagKey] 或顶层 [flagKey] 严格 === true。
 * 源码级函数体真执行（Function/AsyncFunction 注入 op/prompt/confirmMsg/api/refs），非文本断言。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, './ServerApp.vue'), 'utf8')

function bodyOf(source: string, signature: string): string {
  const start = source.indexOf(signature)
  if (start < 0) throw new Error('未找到函数: ' + signature)
  const open = source.indexOf('{', start)
  if (open < 0) throw new Error('未找到函数体: ' + signature)
  let depth = 0, quote: string | null = null, esc = false
  for (let i = open; i < source.length; i++) {
    const c = source[i]
    if (quote) {
      if (esc) { esc = false; continue }
      if (c === '\\') { esc = true; continue }
      if (c === quote) quote = null
      continue
    }
    if (c === "'" || c === '"' || c === '\`') { quote = c; continue }
    if (c === '{') depth++
    else if (c === '}') { depth--; if (depth === 0) return source.slice(open + 1, i) }
  }
  throw new Error('花括号未配对: ' + signature)
}
function safeBody(signature: string): string | null {
  try { return bodyOf(src, signature) } catch { return null }
}
function stripTs(body: string): string {
  return body
    .replace(/\(([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|[\]]*\)/g, '($1)')
    .replace(/catch \(([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|[\]]*\)/g, 'catch ($1)')
    .replace(/\b(const|let) ([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|[\]]*/g, '$1 $2')
    .replace(/\s+as\s+[A-Za-z_$][\w$<>|[\]]*/g, '')
}

const AsyncFn = Object.getPrototypeOf(async function () {}).constructor
type Succeeded = (resp: unknown, flagKey?: string) => boolean
const helperBody = safeBody('function consoleWriteSucceeded(')
const succeeded: Succeeded = helperBody
  ? (new Function('resp', 'flagKey', stripTs(helperBody!)) as Succeeded)
  : (() => false) as Succeeded

function ref<T>(v: T): { value: T } { return { value: v } }
interface Opts {
  op?: string
  resp?: unknown
  reject?: Error
  prompts?: string[]
  confirmed?: boolean
}
function compile(sig: string, names: string[]): (...a: unknown[]) => Promise<void> {
  const body = safeBody(sig)
  expect(body, sig + ' 函数体应可提取').not.toBeNull()
  return new AsyncFn(...names, stripTs(body!)) as (...a: unknown[]) => Promise<void>
}
function makeCtx(opts: Opts) {
  let postCalls = 0, putCalls = 0, getCalls = 0, confirmCalls = 0, promptCalls = 0
  const promptQueue = [...(opts.prompts ?? ['R387输入'])]
  const prompt = (): string => { promptCalls++; return promptQueue.shift() ?? 'R387输入' }
  const confirmMsg = async (): Promise<boolean> => { confirmCalls++; return opts.confirmed !== false }
  const respond = async (): Promise<unknown> => { if (opts.reject) throw opts.reject; return opts.resp }
  const api = {
    post: async (): Promise<unknown> => { postCalls++; return respond() },
    put: async (): Promise<unknown> => { putCalls++; return respond() },
    get: async (): Promise<unknown> => { getCalls++; return respond() },
  }
  const execOutput = ref('')
  const execError = ref('')
  const names = ['op', 'prompt', 'confirmMsg', 'api', 'execOutput', 'execError', 'consoleWriteSucceeded']
  const args = [opts.op ?? 'cacheClear', prompt, confirmMsg, api, execOutput, execError, succeeded]
  const run = (sig: string): Promise<void> => (compile(sig, names) as (...a: unknown[]) => Promise<void>)(...args)
  return {
    execOutput, execError,
    runOps: (): Promise<void> => run('async function serverConsoleOps('),
    counts: (): Record<string, number> => ({ postCalls, putCalls, getCalls, confirmCalls, promptCalls }),
  }
}

describe('R387 consoleWriteSucceeded 真执行（type 为主，flagKey 可选）', () => {
  it('type:success 无 bool 判成功（flagKey 未传不强求 bool）', () => {
    expect(succeeded({ type: 'success', data: {} })).toBe(true)
    expect(succeeded({ type: 'success', data: { cleared: false } })).toBe(true)
  })
  it('type 缺省且 success 非 false 判成功（legacy）', () => {
    expect(succeeded({ success: true, data: {} })).toBe(true)
    expect(succeeded({ data: {} })).toBe(true)
  })
  it('type:error / success:false 判失败', () => {
    expect(succeeded({ type: 'error', message: 'x' })).toBe(false)
    expect(succeeded({ type: 'success', success: false })).toBe(false)
  })
  it('flagKey 传入时权威 bool 必须严格 true（data 或顶层）', () => {
    expect(succeeded({ type: 'success', data: { cleared: true } }, 'cleared')).toBe(true)
    expect(succeeded({ type: 'success', sent: true }, 'sent')).toBe(true)
    expect(succeeded({ type: 'success', data: { cleared: false } }, 'cleared')).toBe(false)
    expect(succeeded({ type: 'success', data: {} }, 'cleared')).toBe(false)
    expect(succeeded({ type: 'success', data: { saved: 1 } }, 'saved')).toBe(false)
  })
  it('undefined/null/非对象判失败', () => {
    expect(succeeded(undefined)).toBe(false)
    expect(succeeded(null)).toBe(false)
    expect(succeeded('x')).toBe(false)
  })
})

describe('R387 P1-B confirmMsg 导入与三处高危守卫', () => {
  it('confirmMsg 必须从 ../utils/toast 导入', () => {
    expect(src).toMatch(/import\s*\{[^}]*\bconfirmMsg\b[^}]*\}\s*from\s*['"]\.\.\/utils\/toast['"]/)
  })
  it('stopServer/cacheClear/deployDelete 三处 confirmMsg 守卫仍在', () => {
    expect((src.match(/await confirmMsg\(/g) ?? []).length).toBeGreaterThanOrEqual(3)
  })
})

describe('R387 cacheClear 写链（权威 cleared）', () => {
  it('cleared:false：execError 透传 message、不写成功', async () => {
    const c = makeCtx({ op: 'cacheClear', resp: { type: 'success', data: { cleared: false }, message: 'R387 未清理' } })
    await c.runOps()
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
    expect(c.execError.value).toContain('R387 未清理')
  })
  it('type:error：execError 透传 message、不写成功', async () => {
    const c = makeCtx({ op: 'cacheClear', resp: { type: 'error', message: 'R387 cache rejected' } })
    await c.runOps()
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
    expect(c.execError.value).toContain('R387 cache rejected')
  })
  it('cleared:true：成功提示保留', async () => {
    const c = makeCtx({ op: 'cacheClear', resp: { type: 'success', data: { cleared: true } } })
    await c.runOps()
    expect(c.execOutput.value).toBe('控制台操作已提交')
    expect(c.execError.value).toBe('')
  })
  it('confirm 取消：零 POST、零成功', async () => {
    const c = makeCtx({ op: 'cacheClear', confirmed: false, resp: { type: 'success', data: { cleared: true } } })
    await c.runOps()
    expect(c.counts().postCalls).toBe(0)
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
  })
})

describe('R387 cmdExecute 写链（无权威 bool，靠 type/非2xx）', () => {
  it('type:error：execError 透传 message、不写成功', async () => {
    const c = makeCtx({ op: 'cmdExecute', prompts: ['whoami'], resp: { type: 'error', message: 'R387 cmd rejected' } })
    await c.runOps()
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
    expect(c.execError.value).toContain('R387 cmd rejected')
  })
  it('type:success 无 bool：成功提示保留', async () => {
    const c = makeCtx({ op: 'cmdExecute', prompts: ['whoami'], resp: { type: 'success', data: { exitCode: 0 } } })
    await c.runOps()
    expect(c.execOutput.value).toBe('控制台操作已提交')
    expect(c.execError.value).toBe('')
  })
  it('非2xx 400：catch 文案保留、不写成功', async () => {
    const c = makeCtx({ op: 'cmdExecute', prompts: ['whoami'], reject: new Error('HTTP 400') })
    await c.runOps()
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
    expect(c.execError.value).toContain('HTTP 400')
  })
})

describe('R387 sendMessage 写链（权威 sent）', () => {
  it('sent:false：execError 透传 message、不写成功', async () => {
    const c = makeCtx({ op: 'sendMessage', prompts: ['R387广播'], resp: { type: 'success', data: { sent: false }, message: 'R387 未发送' } })
    await c.runOps()
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
    expect(c.execError.value).toContain('R387 未发送')
  })
  it('type:error：execError 透传 message、不写成功', async () => {
    const c = makeCtx({ op: 'sendMessage', prompts: ['R387广播'], resp: { type: 'error', message: 'R387 msg rejected' } })
    await c.runOps()
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
    expect(c.execError.value).toContain('R387 msg rejected')
  })
  it('sent:true：成功提示保留', async () => {
    const c = makeCtx({ op: 'sendMessage', prompts: ['R387广播'], resp: { type: 'success', data: { sent: true } } })
    await c.runOps()
    expect(c.execOutput.value).toBe('控制台操作已提交')
    expect(c.execError.value).toBe('')
  })
})

describe('R387 deploySave 写链（权威 saved）', () => {
  it('saved:false：execError 透传 message、不写成功', async () => {
    const c = makeCtx({ op: 'deploySave', prompts: ['R387-FAKE-ID'], resp: { type: 'success', data: { saved: false }, message: 'R387 未保存' } })
    await c.runOps()
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
    expect(c.execError.value).toContain('R387 未保存')
  })
  it('type:error：execError 透传 message、不写成功', async () => {
    const c = makeCtx({ op: 'deploySave', prompts: ['R387-FAKE-ID'], resp: { type: 'error', message: 'R387 save rejected' } })
    await c.runOps()
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
    expect(c.execError.value).toContain('R387 save rejected')
  })
  it('saved:true：成功提示保留', async () => {
    const c = makeCtx({ op: 'deploySave', prompts: ['R387-FAKE-ID'], resp: { type: 'success', data: { saved: true } } })
    await c.runOps()
    expect(c.execOutput.value).toBe('控制台操作已提交')
    expect(c.execError.value).toBe('')
  })
})

describe('R387 deployDelete 写链（权威 deleted）', () => {
  it('deleted:false：execError 透传 message、不写成功', async () => {
    const c = makeCtx({ op: 'deployDelete', prompts: ['R387-FAKE-ID'], resp: { type: 'success', data: { deleted: false }, message: 'R387 未删除' } })
    await c.runOps()
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
    expect(c.execError.value).toContain('R387 未删除')
  })
  it('type:error：execError 透传 message、不写成功', async () => {
    const c = makeCtx({ op: 'deployDelete', prompts: ['R387-FAKE-ID'], resp: { type: 'error', message: 'R387 delete rejected' } })
    await c.runOps()
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
    expect(c.execError.value).toContain('R387 delete rejected')
  })
  it('confirm 取消：零 POST、零成功', async () => {
    const c = makeCtx({ op: 'deployDelete', prompts: ['R387-FAKE-ID'], confirmed: false, resp: { type: 'success', data: { deleted: true } } })
    await c.runOps()
    expect(c.counts().postCalls).toBe(0)
    expect(c.execOutput.value).not.toBe('控制台操作已提交')
  })
  it('deleted:true：成功提示保留', async () => {
    const c = makeCtx({ op: 'deployDelete', prompts: ['R387-FAKE-ID'], resp: { type: 'success', data: { deleted: true } } })
    await c.runOps()
    expect(c.execOutput.value).toBe('控制台操作已提交')
    expect(c.execError.value).toBe('')
  })
})

describe('R387 sysConfig 只读分支保持原语义', () => {
  it('系统配置读仍输出 JSON 且不触发写守卫', async () => {
    const c = makeCtx({ op: 'sysConfig', resp: { data: { ok: true } } })
    await c.runOps()
    expect(c.execOutput.value).toContain('ok')
    expect(c.execError.value).toBe('')
    expect(c.counts().postCalls + c.counts().putCalls).toBe(0)
  })
})
