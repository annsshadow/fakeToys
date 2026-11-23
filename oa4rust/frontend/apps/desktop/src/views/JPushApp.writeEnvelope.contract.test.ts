// @vitest-environment node
import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import path from 'node:path'

let source = ''
try {
  source = readFileSync(new URL('./JPushApp.vue', import.meta.url), 'utf8')
} catch {
  source = readFileSync(path.resolve(process.cwd(), 'frontend/apps/desktop/src/views/JPushApp.vue'), 'utf8')
}

const AsyncFn = Object.getPrototypeOf(async function () {}).constructor as any
const BT = String.fromCharCode(96)

function stripTs(s: string): string {
  return s
    .replace(/\s+as\s+any\b/g, '')
    .replace(/:\s*any\b/g, '')
    .replace(/:\s*string\b/g, '')
    .replace(/:\s*boolean\b/g, '')
    .replace(/:\s*number\b/g, '')
}

function bodyOf(sig: string): string {
  const idx = source.indexOf(sig)
  if (idx < 0) throw new Error('sig not found: ' + sig)
  let i = idx + sig.length
  while (i < source.length && source[i] !== '{') i++
  if (i >= source.length) throw new Error('no body: ' + sig)
  const start = i
  let depth = 0
  let quote = ''
  let esc = false
  for (; i < source.length; i++) {
    const c = source[i]
    if (quote) {
      if (esc) { esc = false; continue }
      if (c === '\\') { esc = true; continue }
      if (c === quote) quote = ''
      continue
    }
    if (c === "'" || c === '"' || c === BT) { quote = c; continue }
    if (c === '{') depth++
    else if (c === '}') { depth--; if (depth === 0) return source.slice(start, i + 1) }
  }
  throw new Error('unbalanced: ' + sig)
}

function helper(): (r: any) => boolean {
  if (source.indexOf('function jpushWriteSucceeded') < 0) return (() => false) as any
  return new Function('resp', stripTs(bodyOf('function jpushWriteSucceeded'))) as any
}

function stubs(resp: any, confirm = true) {
  const success: string[] = []
  const errors: string[] = []
  const infos: string[] = []
  const writes: string[] = []
  const api = {
    post: async (u: string, _b?: any) => { writes.push('POST ' + u); return resp },
    put: async (u: string, _b?: any) => { writes.push('PUT ' + u); return resp },
    get: async (u: string) => { writes.push('GET ' + u); return resp },
    delete: async (u: string) => { writes.push('DELETE ' + u); return resp },
  }
  const toast = {
    success: (m: string) => { success.push(String(m)) },
    error: (m: string) => { errors.push(String(m)) },
    info: (m: string) => { infos.push(String(m)) },
  }
  const confirmMsg = async (_m: string) => confirm
  const prompt = (_m: string, _d?: string) => 'R395-X'
  return { success, errors, infos, writes, api, toast, confirmMsg, prompt }
}

async function runWrite(op: string, resp: any, confirm = true) {
  const s = stubs(resp, confirm)
  const fn = new AsyncFn('op', 'api', 'toast', 'confirmMsg', 'prompt', 'jpushWriteSucceeded', stripTs(bodyOf('async function jpushWrite(')))
  await fn(op, s.api, s.toast, s.confirmMsg, s.prompt, helper())
  return s
}

async function runUnbind(resp: any) {
  const s = stubs(resp)
  const fn = new AsyncFn('api', 'toast', 'prompt', 'jpushWriteSucceeded', stripTs(bodyOf('async function jpushUnbindNew(')))
  await fn(s.api, s.toast, s.prompt, helper())
  return s
}

async function runWx(resp: any) {
  const s = stubs(resp)
  const wxForm = { value: { person: 'p', templateId: 't', content: 'c' } }
  const fn = new AsyncFn('api', 'toast', 'wxForm', 'jpushWriteSucceeded', stripTs(bodyOf('async function sendWxTest(')))
  await fn(s.api, s.toast, wxForm, helper())
  return s
}

async function runCtrl(resp: any) {
  const s = stubs(resp)
  const fn = new AsyncFn('_op', 'api', 'toast', 'prompt', 'jpushWriteSucceeded', stripTs(bodyOf('async function jpushCtrlWrite(')))
  await fn('ctrlBind', s.api, s.toast, s.prompt, helper())
  return s
}

async function runDel(resp: any, confirm = true) {
  const s = stubs(resp, confirm)
  const devices = { value: [{ id: 1 }, { id: 2 }] }
  const d = { id: 1, alias: 'dev1' }
  const fn = new AsyncFn('d', 'api', 'toast', 'confirmMsg', 'devices', 'jpushWriteSucceeded', stripTs(bodyOf('async function delDevice(')))
  await fn(d, s.api, s.toast, s.confirmMsg, devices, helper())
  return { ...s, devices }
}

describe('jpushWriteSucceeded 信封判定', () => {
  it('type:error 判失败', () => { expect(helper()({ type: 'error', message: 'x' })).toBe(false) })
  it('success:false 判失败', () => { expect(helper()({ type: 'success', success: false })).toBe(false) })
  it('type:success 判成功', () => { expect(helper()({ type: 'success', data: {} })).toBe(true) })
  it('type:success + data:{} 无权威bool 判成功', () => { expect(helper()({ type: 'success', data: { id: 1 } })).toBe(true) })
  it('type缺省且success非false 判成功', () => { expect(helper()({ data: {} })).toBe(true) })
  it('null/undefined/非对象 判失败', () => {
    expect(helper()(null)).toBe(false)
    expect(helper()(undefined)).toBe(false)
    expect(helper()('x')).toBe(false)
  })
})

describe('jpushWrite 十分支写链', () => {
  const ops = ['jpushCreate', 'jpushSave', 'jpushDelete', 'deviceBind', 'deviceUnbindAll', 'deviceUnbind', 'deviceRegister', 'messageSend', 'messageTest', 'coreDeviceDelete']
  for (const op of ops) {
    it('分支 ' + op + ' type:error 不弹成功且透传message', async () => {
      const s = await runWrite(op, { type: 'error', message: 'R395后端拒绝:' + op })
      expect(s.success).toEqual([])
      expect(s.errors.join(' ')).toContain('R395后端拒绝:' + op)
    })
  }
  it('jpushSave success:false 不弹成功', async () => {
    const s = await runWrite('jpushSave', { type: 'success', success: false, message: 'R395-save-false' })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('R395-save-false')
  })
  it('jpushSave type:success 保留原成功提示', async () => {
    const s = await runWrite('jpushSave', { type: 'success', data: {} })
    expect(s.success.some((m) => m.indexOf('推送操作已提交') >= 0)).toBe(true)
    expect(s.errors).toEqual([])
  })
  it('jpushDelete confirm取消零写', async () => {
    const s = await runWrite('jpushDelete', { type: 'success' }, false)
    expect(s.writes.length).toBe(0)
    expect(s.success).toEqual([])
  })
})

describe('jpushUnbindNew 新版解绑(GET写语义)', () => {
  it('type:error 不弹成功且透传message', async () => {
    const s = await runUnbind({ type: 'error', message: 'R395解绑被拒' })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('R395解绑被拒')
  })
  it('type:success 保留原成功提示', async () => {
    const s = await runUnbind({ type: 'success', data: {} })
    expect(s.success.some((m) => m.indexOf('设备已解绑') >= 0)).toBe(true)
    expect(s.errors).toEqual([])
  })
})

describe('jpushCtrlWrite 控制域写链', () => {
  it('type:error 不弹成功', async () => {
    const s = await runCtrl({ type: 'error', message: 'R395控制被拒' })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('R395控制被拒')
  })
  it('type:success 保留原成功提示', async () => {
    const s = await runCtrl({ type: 'success', data: {} })
    expect(s.success.some((m) => m.indexOf('控制操作已提交') >= 0)).toBe(true)
  })
})

describe('sendWxTest 微信测试发送', () => {
  it('type:error 不得提示已受理/已入队', async () => {
    const s = await runWx({ type: 'error', message: 'R395微信被拒' })
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('R395微信被拒')
  })
  it('type:success + accepted 提示已入队', async () => {
    const s = await runWx({ type: 'success', data: { accepted: true } })
    expect(s.success.join(' ')).toContain('已入队')
  })
  it('type:success + 无flags 提示已受理', async () => {
    const s = await runWx({ type: 'success', data: {} })
    expect(s.success.join(' ')).toContain('已受理')
  })
})

describe('delDevice 删除设备', () => {
  it('type:error 不删本地行且透传message', async () => {
    const s = await runDel({ type: 'error', message: 'R395删设备被拒' })
    expect(s.devices.value.length).toBe(2)
    expect(s.success).toEqual([])
    expect(s.errors.join(' ')).toContain('R395删设备被拒')
  })
  it('type:success 才删本地行', async () => {
    const s = await runDel({ type: 'success', data: {} })
    expect(s.devices.value.length).toBe(1)
  })
  it('confirm取消零写不删行', async () => {
    const s = await runDel({ type: 'success' }, false)
    expect(s.writes.length).toBe(0)
    expect(s.devices.value.length).toBe(2)
  })
})
