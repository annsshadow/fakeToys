// @vitest-environment node
// R436 ServiceInvokeDesignerApp runM.onSuccess 200+type:error 误报成功契约
import { readFileSync } from 'node:fs'
import { describe, it, expect } from 'vitest'

const SRC_PATH = new URL('./ServiceInvokeDesignerApp.vue', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1')
const src = readFileSync(SRC_PATH, 'utf8')
const RUN_ANCHOR = 'const runM = useMutation({'

function bodyAfter(signature: string, from: number): string {
  const start = src.indexOf(signature, from)
  if (start < 0) return ''
  const open = src.indexOf('{', start)
  if (open < 0) return ''
  let depth = 0, quote = 0, esc = false
  for (let i = open; i < src.length; i++) {
    const code = src.charCodeAt(i)
    const ch = src[i]
    if (esc) { esc = false; continue }
    if (code === 92) { esc = true; continue }
    if (quote) { if (code === quote) quote = 0; continue }
    if (ch === "'" || ch === '"' || code === 96) { quote = code; continue }
    if (ch === '{') depth++
    else if (ch === '}') { depth--; if (depth === 0) return src.slice(open + 1, i) }
  }
  return ''
}

const helperFound = src.indexOf('function serviceInvokeWriteSucceeded(') >= 0
const helperBody = helperFound ? bodyAfter('function serviceInvokeWriteSucceeded(', 0) : ''
const runAnchorIdx = src.indexOf(RUN_ANCHOR)
const onSuccessBody = bodyAfter('onSuccess:', runAnchorIdx)
const onErrorBody = bodyAfter('onError:', runAnchorIdx)
const mutationFnBody = bodyAfter('mutationFn:', runAnchorIdx)
const Fn = Object.getPrototypeOf(function () {}).constructor

function makeHelper() {
  if (!helperFound || !helperBody) return () => false
  return new Fn('resp', '"use strict"; ' + helperBody)
}

function runOnSuccess(lastData: any): { toasts: Array<{ title: string }>; errors: Array<{ title: string }> } {
  const toasts: Array<{ title: string }> = [], errors: Array<{ title: string }> = []
  const toast = { success: (t: string) => toasts.push({ title: t }), error: (t: string) => errors.push({ title: t }) }
  const onSuccess = new Fn('toast', 'serviceInvokeWriteSucceeded', 'lastData', '"use strict"; ' + onSuccessBody)
  onSuccess(toast, makeHelper(), lastData)
  return { toasts, errors }
}
function runOnError(e: unknown): { toasts: Array<{ title: string }>; errors: Array<{ title: string }> } {
  const toasts: Array<{ title: string }> = [], errors: Array<{ title: string }> = []
  const toast = { success: (t: string) => toasts.push({ title: t }), error: (t: string) => errors.push({ title: t }) }
  const onError = new Fn('toast', 'e', '"use strict"; ' + onErrorBody)
  onError(toast, e)
  return { toasts, errors }
}

describe('serviceInvokeWriteSucceeded helper', () => {
  it('type:error 判失败', () => { expect(helperFound).toBe(true); expect(makeHelper()({ type: 'error', message: 'invoke not found' })).toBe(false) })
  it('success:false 判失败', () => { expect(helperFound).toBe(true); expect(makeHelper()({ type: 'success', success: false })).toBe(false) })
  it('type:success 判成功', () => { expect(helperFound).toBe(true); expect(makeHelper()({ type: 'success', data: { executed: true } })).toBe(true) })
  it('type 缺省且 success 非 false 判成功', () => { expect(helperFound).toBe(true); expect(makeHelper()({ data: { id: 'x' } })).toBe(true) })
  it('非对象 null undefined 数字判失败', () => { expect(helperFound).toBe(true); const h = makeHelper(); expect(h(null)).toBe(false); expect(h(undefined)).toBe(false); expect(h(42)).toBe(false) })
  it('不强求权威 bool，executed:0 仍判成功', () => { expect(helperFound).toBe(true); expect(makeHelper()({ type: 'success', data: { executed: 0 } })).toBe(true) })
})

describe('runM.onSuccess 接响应守卫', () => {
  it('200+type:error → error 透传 message 不弹成功', () => {
    const c = runOnSuccess({ type: 'error', message: 'invoke not found' })
    expect(c.errors.some(t => t.title.indexOf('invoke not found') >= 0)).toBe(true)
    expect(c.toasts.length).toBe(0)
  })
  it('success:false → error 透传不弹成功', () => {
    const c = runOnSuccess({ type: 'success', success: false, message: 'R436 denied' })
    expect(c.errors.some(t => t.title.indexOf('R436 denied') >= 0)).toBe(true)
    expect(c.toasts.length).toBe(0)
  })
  it('非对象/null → error 不弹成功', () => {
    const c = runOnSuccess(null)
    expect(c.errors.length).toBe(1)
    expect(c.toasts.length).toBe(0)
  })
  it('type:success → 绿 toast 服务调用已执行', () => {
    const c = runOnSuccess({ type: 'success', data: { id: 'x', executed: true } })
    expect(c.toasts.some(t => t.title.indexOf('服务调用已执行') >= 0)).toBe(true)
    expect(c.errors.length).toBe(0)
  })
  it('type 缺省 success 非 false → 成功', () => {
    const c = runOnSuccess({ data: { id: 'x' } })
    expect(c.toasts.some(t => t.title.indexOf('服务调用已执行') >= 0)).toBe(true)
    expect(c.errors.length).toBe(0)
  })
  it('executed:0 不强求 bool → 成功', () => {
    const c = runOnSuccess({ type: 'success', data: { executed: 0 } })
    expect(c.toasts.some(t => t.title.indexOf('服务调用已执行') >= 0)).toBe(true)
  })
})

describe('onError / 零改范围', () => {
  it('onError 非2xx 仍 error 透传且不弹成功', () => {
    const c = runOnError(new Error('HTTP 403'))
    expect(c.errors.some(t => t.title.indexOf('HTTP 403') >= 0)).toBe(true)
    expect(c.toasts.length).toBe(0)
  })
  it('mutationFn 端点与 method 不变（POST execute）', () => {
    expect(runAnchorIdx).toBeGreaterThan(-1)
    expect(mutationFnBody).toContain('/api/program_center/invoke/')
    expect(mutationFnBody).toContain('/execute')
    expect(mutationFnBody).toContain('api.post')
  })
  it('saveM/deleteItem 只读零改（仍各自 onError/确认守卫）', () => {
    expect(src).toContain("confirmMsg('确定删除该服务调用？')")
    expect(src.indexOf('const saveM = useMutation({')).toBeGreaterThan(-1)
    expect(src.indexOf('const delM = useMutation({')).toBeGreaterThan(-1)
  })
  it('loadData/read 仍为 reload（零改）', () => {
    expect(src).toContain('function loadData() {\n  reload()\n}')
  })
})
