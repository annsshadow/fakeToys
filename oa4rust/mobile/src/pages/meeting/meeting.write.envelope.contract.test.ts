// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R360 契约：H5 meeting 五写端点在 HTTP 200 + type:success + bool:false 或 200 + type:error 下
 * 必须错误提示、不得误报成功；仅 type:success（或 type 缺省且 success 非 false）且对应 bool:true 才成功。
 * R359 审计坐实：act() 无条件 toast + index.ts 五端点 discardResponse:true 吞响应体。
 * 源码级函数体真执行（new AsyncFn），不引入新依赖。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const meetingSrc = readFileSync(resolve(import.meta.dirname, './meeting.vue'), 'utf8')
const idxSrc = readFileSync(resolve(import.meta.dirname, '../../services/index.ts'), 'utf8')

function bodyOf(source: string, signature: string): string {
  const start = source.indexOf(signature)
  if (start < 0) throw new Error('未找到函数: ' + signature)
  const open = source.indexOf('{', start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === '{') depth++
    else if (source[i] === '}') {
      depth--
      if (depth === 0) return source.slice(open + 1, i)
    }
  }
  throw new Error('花括号未配对: ' + signature)
}

const actBody = bodyOf(meetingSrc, 'async function act(')
  .replace(/const resp: any/g, 'const resp')
  .replace(/catch \(e: unknown\)/g, 'catch (e)')

const AsyncFn = Object.getPrototypeOf(async function () {}).constructor

interface Toast { title: string; icon: string }

function runAct(envelope: unknown, flagKey: string, opts: { reject?: Error; busy?: string } = {}) {
  const toasts: Toast[] = []
  const busyId: { value: string } = { value: opts.busy ?? '' }
  let loadCount = 0
  const uniStub = { showToast: (o: Toast): void => { toasts.push(o) } }
  const loadStub = async (): Promise<void> => { loadCount++ }
  const fnStub = async (): Promise<unknown> => {
    if (opts.reject) throw opts.reject
    return envelope
  }
  const invoke = new AsyncFn('uni', 'busyId', 'load', 'id', 'fn', 'okText', 'flagKey', actBody)
  return {
    toasts,
    busyId,
    invoke: (): Promise<void> => invoke(uniStub, busyId, loadStub, 'M360', fnStub, '已接受', flagKey),
    loadCount: (): number => loadCount,
  }
}

describe('R360 meeting 五写端点业务信封判定契约', () => {
  it('index.ts 五端点去 discardResponse:true 且端点路径不变', () => {
    const specs: Array<[string, string]> = [
      ['accept: (id: string)', '/accept`'],
      ['reject: (id: string)', '/reject`'],
      ['confirmAllow: (id: string)', '/confirm/allow`'],
      ['confirmDeny: (id: string)', '/confirm/deny`'],
      ['checkin: (id: string)', '/checkin`'],
    ]
    for (const [sig, suffix] of specs) {
      const start = idxSrc.indexOf(sig)
      expect(start, sig).toBeGreaterThan(-1)
      const seg = idxSrc.slice(start, start + 240)
      expect(seg, sig).toContain('/api/meeting/assemble/control/meeting/')
      expect(seg, sig).toContain(suffix)
      expect(seg, sig).not.toContain('discardResponse: true')
    }
  })

  it('五个调用点分别传入对应 bool 字段名', () => {
    expect(meetingSrc).toContain("'已接受', 'accepted')")
    expect(meetingSrc).toContain("'已拒绝', 'rejected')")
    expect(meetingSrc).toContain("'已确认出席', 'confirmed')")
    expect(meetingSrc).toContain("'已确认缺席', 'denied')")
    expect(meetingSrc).toContain("'签到成功', 'checkedIn')")
  })

  it('200+type:success+bool:true 才弹成功 toast 并刷新列表', async () => {
    const c = runAct({ type: 'success', data: { accepted: true } }, 'accepted')
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).toContain('已接受')
    expect(c.toasts[0]?.icon).toBe('success')
    expect(c.loadCount()).toBe(1)
  })

  it('200+type:success+bool:false 不误报成功，弹缺省错误文案', async () => {
    const c = runAct({ type: 'success', data: { accepted: false } }, 'accepted')
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).not.toContain('已接受')
    expect(c.toasts.map((t) => t.title)).toContain('操作未生效，请刷新后重试')
    expect(c.toasts[0]?.icon).toBe('none')
  })

  it('200+type:error 显示后端 message 且不误报成功', async () => {
    const c = runAct({ type: 'error', message: '后端拒绝：会议已结束', data: {} }, 'accepted')
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).toContain('后端拒绝：会议已结束')
    expect(c.toasts.map((t) => t.title)).not.toContain('已接受')
  })

  it('success:false 无 type 信封按错误处理并显示后端 message', async () => {
    const c = runAct({ success: false, message: '会议未生效' }, 'accepted')
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).toContain('会议未生效')
    expect(c.toasts.map((t) => t.title)).not.toContain('已接受')
  })

  it('type 缺省且 success 非 false 的 bool:true 信封仍判成功', async () => {
    const c = runAct({ data: { accepted: true } }, 'accepted')
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).toContain('已接受')
  })

  it('bool 字段平铺在信封根时同样判定成功', async () => {
    const c = runAct({ type: 'success', accepted: true }, 'accepted')
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).toContain('已接受')
  })

  it('非2xx reject 走 catch 显示错误提示', async () => {
    const c = runAct(undefined, 'accepted', { reject: new Error('HTTP 403') })
    await c.invoke()
    expect(c.toasts.map((t) => t.title)).toContain('HTTP 403')
    expect(c.loadCount()).toBe(0)
  })

  it('busyId 在途拦截，不执行请求也不弹 toast', async () => {
    const c = runAct({ type: 'success', data: { accepted: true } }, 'accepted', { busy: 'OTHER' })
    await c.invoke()
    expect(c.toasts).toHaveLength(0)
    expect(c.loadCount()).toBe(0)
    expect(c.busyId.value).toBe('OTHER')
  })
})
