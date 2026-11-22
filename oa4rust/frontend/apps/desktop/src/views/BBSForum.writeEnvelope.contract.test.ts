// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R376 契约：PC BBSForum.vue 四条真实写链在 HTTP200 + type:error，或 HTTP200 +
 * type:success 但权威 saved/deleted:false 时，必须错误提示、不得误报成功、不得刷新/落态。
 * type 判定为主：type:success（或 type 缺省且 success 非 false）才成功；提供 flagKey 时要求 data[flagKey] === true。
 * 源码级函数体真执行（Function/AsyncFunction 注入 api/toast/qc/confirmMsg/mutation 桩），非文本断言。
 * mutation 桩按 vue-query 语义先执行源码中的默认 onSuccess(data)，再执行 mutate options.onSuccess(data)。
 */
import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { describe, expect, it } from 'vitest'

const src = readFileSync(resolve(import.meta.dirname, './BBSForum.vue'), 'utf8')

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
/** 从 useMutation 定义区间中提取默认 onSuccess(data) 的函数体。 */
function mutationOnSuccessBody(startMark: string, endMark: string): string | null {
  const s = src.indexOf(startMark)
  const e = s < 0 ? -1 : src.indexOf(endMark, s)
  if (s < 0 || e < 0) return null
  const seg = src.slice(s, e)
  const idx = seg.indexOf('onSuccess:')
  if (idx < 0) return null
  const open = seg.indexOf('{', idx + 'onSuccess:'.length)
  if (open < 0) return null
  let depth = 0, quote: string | null = null, esc = false
  for (let i = open; i < seg.length; i++) {
    const c = seg[i]
    if (quote) {
      if (esc) { esc = false; continue }
      if (c === '\\') { esc = true; continue }
      if (c === quote) quote = null
      continue
    }
    if (c === "'" || c === '"' || c === '\`') { quote = c; continue }
    if (c === '{') depth++
    else if (c === '}') { depth--; if (depth === 0) return seg.slice(open + 1, i) }
  }
  return null
}
function stripTs(body: string): string {
  return body
    .replace(/\(([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|\[\]]*\)/g, '($1)')
    .replace(/catch \(([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|\[\]]*\)/g, 'catch ($1)')
    .replace(/\b(const|let) ([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|\[\]]*/g, '$1 $2')
    .replace(/\s+as\s+[A-Za-z_$][\w$<>|\[\]]*/g, '')
}

const AsyncFn = Object.getPrototypeOf(async function () {}).constructor
type Succeeded = (resp: unknown, flagKey?: string) => boolean
const helperBody = safeBody('function bbsWriteSucceeded(')
const succeeded: Succeeded = helperBody
  ? (new Function('resp', 'flagKey', stripTs(helperBody!)) as Succeeded)
  : (() => false) as Succeeded

function ref<T>(v: T): { value: T } { return { value: v } }
interface Toasts { success: string[]; error: string[] }
interface Opts {
  resp?: unknown
  reject?: Error
  mode?: 'edit' | 'create'
  confirmed?: boolean
  rejectMutate?: boolean
}

function compile(sig: string, names: string[], isAsync: boolean): (...a: unknown[]) => unknown {
  const body = safeBody(sig)
  expect(body, sig + ' 函数体应可提取').not.toBeNull()
  const plain = stripTs(body!)
  if (isAsync) return new AsyncFn(...names, plain) as (...a: unknown[]) => Promise<void>
  return new Function(...names, plain) as (...a: unknown[]) => void
}

function makeCtx(opts: Opts) {
  const toasts: Toasts = { success: [], error: [] }
  const toast = {
    success: (msg: string): void => { toasts.success.push(msg) },
    error: (msg: string): void => { toasts.error.push(msg) },
  }
  let postCalls = 0, invalidateCalls = 0, closeCalls = 0, refetchCalls = 0, refetchReplyCalls = 0, mutateCalls = 0
  const api = {
    post: async (): Promise<unknown> => { postCalls++; if (opts.reject) throw opts.reject; return opts.resp },
  }
  const qc = { invalidateQueries: async (): Promise<void> => { invalidateCalls++ } }
  const confirmMsg = async (): Promise<boolean> => opts.confirmed !== false
  const sec = { id: 'R376-SEC-1', name: 'R376版块' }
  const sectionName = ref('R376新名称')
  const sectionBusy = ref(false)
  const sectionError = ref('')
  const sectionModalMode = ref<string>(opts.mode ?? 'edit')
  const sectionModalTarget = ref<{ id: string } | null>(sec)
  const selectedSection = ref<{ id: string } | null>(sec)
  const showSectionModal = ref(true)
  const closeSectionModal = (): void => { closeCalls++ }
  const saveNames = ['sectionName', 'sectionBusy', 'sectionError', 'sectionModalMode', 'sectionModalTarget', 'api', 'toast', 'closeSectionModal', 'qc', 'bbsWriteSucceeded']
  const saveArgs = [sectionName, sectionBusy, sectionError, sectionModalMode, sectionModalTarget, api, toast, closeSectionModal, qc, succeeded]
  const runSave = async (): Promise<void> => { await (compile('async function saveSection(', saveNames, true) as (...a: unknown[]) => Promise<void>)(...saveArgs) }
  const deleteNames = ['sec', 'sectionBusy', 'confirmMsg', 'api', 'toast', 'selectedSection', 'qc', 'bbsWriteSucceeded']
  const deleteArgs = [sec, sectionBusy, confirmMsg, api, toast, selectedSection, qc, succeeded]
  const runDelete = async (): Promise<void> => { await (compile('async function deleteSection(', deleteNames, true) as (...a: unknown[]) => Promise<void>)(...deleteArgs) }

  // 模拟 vue-query：mutation 默认 onSuccess(data) 先执行，再执行 mutate 的 options.onSuccess(data)。
  const defaultNames = ['data', 'showNewTopic', 'refetch', 'newTopic', 'createError', 'replyText', 'refetchReplies', 'toast', 'bbsWriteSucceeded']
  const makeDefault = (body: string | null): ((...a: unknown[]) => void) => {
    if (!body) return () => {}
    return new Function(...defaultNames, stripTs(body).replace(/\(data: any\)/g, '(data)')) as (...a: unknown[]) => void
  }
  const newTopic = ref({ sectionId: 'R376-SEC-1', title: 'R376新帖', content: '正文' })
  const topicSaving = ref(false)
  const createError = ref('')
  const topicDefault = makeDefault(mutationOnSuccessBody('const createMutation = useMutation(', 'const newTopic = ref('))
  const topicMutate = {
    mutate: (_data: unknown, options: { onSuccess?: (r: unknown) => void; onError?: (e: unknown) => void; onSettled?: () => void }): void => {
      mutateCalls++
      if (opts.rejectMutate) options.onError?.(new Error('HTTP 500'))
      else {
        topicDefault(opts.resp, showSectionModal, (): void => { refetchCalls++ }, newTopic, createError, undefined, undefined, toast, succeeded)
        options.onSuccess?.(opts.resp)
      }
      options.onSettled?.()
    },
  }
  // 源码 createTopic 的函数体使用 showNewTopic 变量；契约以 showSectionModal 对象注入该位置。
  const topicNames = ['newTopic', 'topicSaving', 'createError', 'showNewTopic', 'toast', 'createMutation', 'refetch', 'bbsWriteSucceeded']
  const topicArgs = [newTopic, topicSaving, createError, showSectionModal, toast, topicMutate, (): void => { refetchCalls++ }, succeeded]
  const runCreate = (): void => { compile('function createTopic(', topicNames, false)(...topicArgs) }

  const replyText = ref('R376回复')
  const replySaving = ref(false)
  const viewingTopic = ref<{ id: string; topicRef: string } | null>({ id: 'R376-TOPIC-1', topicRef: 'R376-TOPIC-1' })
  const replyDefault = makeDefault(mutationOnSuccessBody('const replyMutation = useMutation(', 'function submitReply('))
  const replyMutate = {
    mutate: (_content: string, options: { onSuccess?: (r: unknown) => void; onError?: (e: unknown) => void; onSettled?: () => void }): void => {
      mutateCalls++
      if (opts.rejectMutate) options.onError?.(new Error('HTTP 500'))
      else {
        replyDefault(opts.resp, undefined, (): void => { refetchCalls++ }, undefined, undefined, replyText, (): void => { refetchReplyCalls++ }, toast, succeeded)
        options.onSuccess?.(opts.resp)
      }
      options.onSettled?.()
    },
  }
  const replyNames = ['replyText', 'replySaving', 'viewingTopic', 'toast', 'replyMutation', 'refetchReplies', 'refetch', 'bbsWriteSucceeded']
  const replyArgs = [replyText, replySaving, viewingTopic, toast, replyMutate, (): void => { refetchReplyCalls++ }, (): void => { refetchCalls++ }, succeeded]
  const runReply = (): void => { compile('function submitReply(', replyNames, false)(...replyArgs) }

  return {
    toasts, sectionError, showSectionModal, selectedSection, newTopic, replyText,
    runSave, runDelete, runCreate, runReply,
    counts: (): Record<string, number> => ({ postCalls, invalidateCalls, closeCalls, refetchCalls, refetchReplyCalls, mutateCalls }),
  }
}
const allText = (c: ReturnType<typeof makeCtx>): string => c.toasts.success.concat(c.toasts.error).join('|') + '|' + c.sectionError.value

describe('R376 bbsWriteSucceeded 真执行（type 为主，有权威 bool 时严格）', () => {
  it('type:success 无 bool 判成功', () => { expect(succeeded({ type: 'success', data: {} })).toBe(true) })
  it('type 缺省且 success 非 false 判成功', () => { expect(succeeded({ success: true, data: {} })).toBe(true) })
  it('type:error / success:false 判失败', () => {
    expect(succeeded({ type: 'error', message: 'x' })).toBe(false)
    expect(succeeded({ type: 'success', success: false })).toBe(false)
  })
  it('flagKey 提供时权威 bool 必须严格 true', () => {
    expect(succeeded({ type: 'success', data: { saved: true } }, 'saved')).toBe(true)
    expect(succeeded({ type: 'success', data: { saved: false } }, 'saved')).toBe(false)
    expect(succeeded({ type: 'success', data: {} }, 'saved')).toBe(false)
    expect(succeeded({ type: 'success', data: { deleted: 1 } }, 'deleted')).toBe(false)
  })
  it('undefined/null/非对象判失败', () => { expect(succeeded(undefined)).toBe(false); expect(succeeded(null)).toBe(false); expect(succeeded('x')).toBe(false) })
})

describe('R376 saveSection 失败信封修复', () => {
  it('edit 分支 saved:false：不弹成功、不关闭弹窗、不刷新，透传 message', async () => {
    const c = makeCtx({ mode: 'edit', resp: { type: 'success', message: '保存未生效', data: { saved: false } } })
    await c.runSave()
    expect(c.toasts.success.join('|')).not.toContain('版块已重命名')
    expect(allText(c)).toContain('保存未生效')
    expect(c.showSectionModal.value).toBe(true)
    expect(c.counts().closeCalls).toBe(0)
    expect(c.counts().invalidateCalls).toBe(0)
  })
  it('create 分支 type:error：透传 message、不落态', async () => {
    const c = makeCtx({ mode: 'create', resp: { type: 'error', message: 'create rejected', success: false } })
    await c.runSave()
    expect(c.toasts.success.join('|')).not.toContain('版块已创建')
    expect(allText(c)).toContain('create rejected')
    expect(c.counts().invalidateCalls).toBe(0)
  })
  it('edit 分支 saved:true：成功路径保留', async () => {
    const c = makeCtx({ mode: 'edit', resp: { type: 'success', data: { saved: true } } })
    await c.runSave()
    expect(c.toasts.success.join('|')).toContain('版块已重命名')
    expect(c.counts().invalidateCalls).toBe(1)
  })
})

describe('R376 deleteSection 失败信封修复', () => {
  it('deleted:false：不弹成功、不清空选中、不刷新', async () => {
    const c = makeCtx({ resp: { type: 'success', message: '删除未生效', data: { deleted: false } } })
    await c.runDelete()
    expect(c.toasts.success.join('|')).not.toContain('版块已删除')
    expect(c.toasts.error.join('|')).toContain('删除未生效')
    expect(c.selectedSection.value).not.toBeNull()
    expect(c.counts().invalidateCalls).toBe(0)
  })
  it('type:error：透传后端 message 且不弹成功', async () => {
    const c = makeCtx({ resp: { type: 'error', message: 'section not found' } })
    await c.runDelete()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.join('|')).toContain('section not found')
  })
  it('deleted:true：成功路径保留', async () => {
    const c = makeCtx({ resp: { type: 'success', data: { deleted: true } } })
    await c.runDelete()
    expect(c.toasts.success.join('|')).toContain('版块已删除')
    expect(c.selectedSection.value).toBeNull()
    expect(c.counts().invalidateCalls).toBe(1)
  })
  it('confirm 取消：0 写请求、0 toast', async () => {
    const c = makeCtx({ resp: { type: 'success', data: { deleted: true } }, confirmed: false })
    await c.runDelete()
    expect(c.counts().postCalls).toBe(0)
    expect(c.toasts.success.length + c.toasts.error.length).toBe(0)
  })
  it('非2xx：catch 错误提示、不误报成功', async () => {
    const c = makeCtx({ reject: new Error('HTTP 403: forbidden') })
    await c.runDelete()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.join('|')).toContain('HTTP 403')
  })
})

describe('R376 createTopic / submitReply mutation 响应判定修复', () => {
  it('createTopic type:error：不弹成功、不清空表单、不刷新', () => {
    const c = makeCtx({ resp: { type: 'error', message: '标题重复' } })
    c.runCreate()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.join('|')).toContain('标题重复')
    expect(c.showSectionModal.value).toBe(true)
    expect(c.newTopic.value.title).toBe('R376新帖')
    expect(c.counts().refetchCalls).toBe(0)
  })
  it('createTopic success:false：不弹成功', () => {
    const c = makeCtx({ resp: { type: 'success', success: false, message: '发布未生效' } })
    c.runCreate()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.join('|')).toContain('发布未生效')
  })
  it('createTopic 成功仍发布并清空表单', () => {
    const c = makeCtx({ resp: { type: 'success', data: { id: 'topic-1' } } })
    c.runCreate()
    expect(c.toasts.success.join('|')).toContain('帖子已发布')
    expect(c.showSectionModal.value).toBe(false)
    expect(c.newTopic.value.title).toBe('')
  })
  it('createTopic 非2xx onError：不误报成功', () => {
    const c = makeCtx({ resp: {}, rejectMutate: true })
    c.runCreate()
    expect(c.toasts.success.length).toBe(0)
  })
  it('submitReply type:error：不弹成功、不清空回复', () => {
    const c = makeCtx({ resp: { type: 'error', message: '无权回复' } })
    c.runReply()
    expect(c.toasts.success.length).toBe(0)
    expect(c.toasts.error.join('|')).toContain('无权回复')
    expect(c.replyText.value).toBe('R376回复')
    expect(c.counts().refetchReplyCalls).toBe(0)
  })
  it('submitReply 成功仍发送并清空', () => {
    const c = makeCtx({ resp: { type: 'success', data: { id: 'reply-1' } } })
    c.runReply()
    expect(c.toasts.success.join('|')).toContain('回复已发送')
    expect(c.replyText.value).toBe('')
    expect(c.counts().refetchReplyCalls).toBe(1)
  })
})