import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'

const src = readFileSync(new URL('./OrgDirectoryApp.vue', import.meta.url), 'utf-8')

function bodyOf(sig: string): string {
  const i = src.indexOf(sig)
  if (i < 0) return ''
  let depth = 0
  let started = false
  for (let j = i; j < src.length; j++) {
    const ch = src[j]
    if (ch === '{') { depth++; started = true }
    else if (ch === '}') { depth--; if (started && depth === 0) return src.slice(i, j + 1) }
  }
  return ''
}

function stripTs(s: string): string {
  return s
    .replace(/\(([A-Za-z_$][\w$]*)\s*:\s*[A-Za-z_$][\w$<>|\[\], ]*\)/g, '($1)')
    .replace(/\)\s*:\s*[A-Za-z_$][\w$<>|\[\], ]*\s*\{/g, '){')
    .replace(/\b(const|let|var)\s+([A-Za-z_$][\w$]*)\s*:\s*[A-Za-z_$][\w$<>|\[\], ]*\s*=/g, '$1 $2 =')
    .replace(/\s+as\s+[A-Za-z_$][\w$<>|\[\], ]*/g, '')
}

function makeSucceeded(): (r: any) => boolean {
  const helperSrc = bodyOf('function writeSucceeded(')
  if (!helperSrc) return () => false
  return new Function(stripTs(helperSrc) + '\nreturn writeSucceeded;')() as (r: any) => boolean
}

const succeeded = makeSucceeded()

function makeCtx(resp: any, opts: any = {}) {
  const calls: any = { api: [], toastOk: [], toastErr: [], loadDeptTree: 0 }
  const api = new Proxy({}, {
    get(_t: any, method: string) {
      return (...args: any[]) => {
        calls.api.push({ method, args })
        return resp instanceof Error ? Promise.reject(resp) : Promise.resolve(resp)
      }
    },
  })
  const toast = {
    success: (m: any) => { calls.toastOk.push(String(m)) },
    info: (m: any) => { calls.toastOk.push(String(m)) },
    error: (m: any) => { calls.toastErr.push(String(m)) },
  }
  const withTimeout = (p: Promise<any>) => p
  const loadDeptTree = () => { calls.loadDeptTree++ }
  const newDeptName = { value: opts.name === undefined ? 'R380测试部门' : opts.name }
  const newDeptParent = { value: opts.parent === undefined ? '' : opts.parent }
  const showUnitCreate = { value: true }
  const unitCreating = { value: false }
  const unitCreateError = { value: '' }
  return { calls, api, toast, withTimeout, loadDeptTree, newDeptName, newDeptParent, showUnitCreate, unitCreating, unitCreateError }
}

function compile(ctx: any): () => Promise<void> {
  const body = bodyOf('async function createUnitFromForm(')
  if (!body) throw new Error('createUnitFromForm body not found')
  const names = ['api','toast','withTimeout','loadDeptTree','newDeptName','newDeptParent','showUnitCreate','unitCreating','unitCreateError','DEPT_TREE_TIMEOUT_MS','writeSucceeded']
  const args = [ctx.api, ctx.toast, ctx.withTimeout, ctx.loadDeptTree, ctx.newDeptName, ctx.newDeptParent, ctx.showUnitCreate, ctx.unitCreating, ctx.unitCreateError, 10000, succeeded]
  return new Function(...names, stripTs(body) + '\nreturn createUnitFromForm;')(...args) as () => Promise<void>
}

describe('R380 OrgDirectoryApp createUnitFromForm write envelope (real execution)', () => {
  it('helper: type:error fails', () => { expect(succeeded({ type: 'error', message: 'x' })).toBe(false) })
  it('helper: success:false fails', () => { expect(succeeded({ success: false })).toBe(false) })
  it('helper: type:success with data.id passes', () => { expect(succeeded({ type: 'success', data: { id: 'U1', name: 'x' } })).toBe(true) })
  it('helper: type:success without data.id fails', () => { expect(succeeded({ type: 'success', data: { name: 'x' } })).toBe(false) })
  it('helper: missing type with data.id passes', () => { expect(succeeded({ data: { id: 'U1' } })).toBe(true) })
  it('helper: no bool requirement - data.id wins over bool:false', () => { expect(succeeded({ type: 'success', data: { id: 'U1', enabled: false } })).toBe(true) })

  it('empty name -> zero write, inline error', async () => {
    const ctx = makeCtx({ type: 'success', data: { id: 'U1' } }, { name: '   ' })
    await compile(ctx)()
    expect(ctx.calls.api.length).toBe(0)
    expect(ctx.unitCreateError.value).toContain('请输入部门名称')
    expect(ctx.calls.toastOk.length).toBe(0)
  })

  it('type:error -> error toast, form retained, no refresh', async () => {
    const ctx = makeCtx({ type: 'error', message: 'R380 backend error' })
    await compile(ctx)()
    expect(ctx.calls.api.length).toBe(1)
    expect(ctx.calls.toastOk.length).toBe(0)
    expect(ctx.calls.toastErr.join(' ')).toContain('R380 backend error')
    expect(ctx.newDeptName.value).toBe('R380测试部门')
    expect(ctx.showUnitCreate.value).toBe(true)
    expect(ctx.calls.loadDeptTree).toBe(0)
  })

  it('type:success without data.id -> failure, form retained', async () => {
    const ctx = makeCtx({ type: 'success', data: { name: 'x' } })
    await compile(ctx)()
    expect(ctx.calls.toastOk.length).toBe(0)
    expect(ctx.calls.toastErr.length).toBeGreaterThanOrEqual(1)
    expect(ctx.newDeptName.value).toBe('R380测试部门')
    expect(ctx.showUnitCreate.value).toBe(true)
    expect(ctx.calls.loadDeptTree).toBe(0)
  })

  it('type:success with data.id -> success, form cleared and refreshed', async () => {
    const ctx = makeCtx({ type: 'success', data: { id: 'U1', name: 'R380测试部门' } })
    await compile(ctx)()
    expect(ctx.calls.toastErr.length).toBe(0)
    expect(ctx.calls.toastOk.join(' ')).toContain('已创建')
    expect(ctx.newDeptName.value).toBe('')
    expect(ctx.newDeptParent.value).toBe('')
    expect(ctx.showUnitCreate.value).toBe(false)
    expect(ctx.calls.loadDeptTree).toBe(1)
  })

  it('403 catch -> error toast, form retained', async () => {
    const ctx = makeCtx(new Error('没有权限执行该操作'))
    await compile(ctx)()
    expect(ctx.calls.toastOk.length).toBe(0)
    expect(ctx.calls.toastErr.join(' ')).toContain('没有权限')
    expect(ctx.newDeptName.value).toBe('R380测试部门')
    expect(ctx.calls.loadDeptTree).toBe(0)
  })

  it('500 catch -> error toast, form retained', async () => {
    const ctx = makeCtx(new Error('HTTP 500'))
    await compile(ctx)()
    expect(ctx.calls.toastOk.length).toBe(0)
    expect(ctx.calls.toastErr.join(' ')).toContain('500')
    expect(ctx.newDeptName.value).toBe('R380测试部门')
    expect(ctx.calls.loadDeptTree).toBe(0)
  })
})
