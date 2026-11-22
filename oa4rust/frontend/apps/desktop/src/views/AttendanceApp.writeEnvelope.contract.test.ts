// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// R371: AttendanceApp.vue 写响应信封守护契约（源码级真执行，非文本断言）
// @vitest-environment node
import { describe, expect, it } from 'vitest'
import { readFileSync } from 'node:fs'
import { fileURLToPath } from 'node:url'

const source = readFileSync(fileURLToPath(new URL('./AttendanceApp.vue', import.meta.url)), 'utf8')

function extractFn(sig: string): string | null {
  const i = source.indexOf(sig)
  if (i < 0) return null
  const open = source.indexOf('{', i)
  if (open < 0) return null
  let depth = 0, quote: string | null = null, esc = false
  for (let j = open; j < source.length; j++) {
    const c = source[j]
    if (quote) {
      if (esc) { esc = false; continue }
      if (c === '\\') { esc = true; continue }
      if (c === quote) quote = null
      continue
    }
    if (c === "'" || c === '"' || c === '`') { quote = c; continue }
    if (c === '{') depth++
    else if (c === '}') { depth--; if (depth === 0) return source.slice(i, j + 1) }
  }
  return null
}

function stripTs(s: string): string {
  return s
    .replace(/\(resp: any\)/g, '(resp)')
    .replace(/\(r: any\)/g, '(r)')
    .replace(/\(p: Promise<any>\)/g, '(p)')
    .replace(/\(e: any\)/g, '(e)')
    .replace(/\(e: unknown\)/g, '(e)')
    .replace(/<T>/g, '')
    .replace(/: T \| null/g, '')
    .replace(/: Promise<void>/g, '')
    .replace(/: Promise<any>/g, '')
    .replace(/: boolean/g, '')
    .replace(/: string/g, '')
    .replace(/: number/g, '')
    .replace(/: A(?=[,)\s])/g, '')
    .replace(/: R(?=[,)\s])/g, '')
    .replace(/: WP(?=[,)\s])/g, '')
    .replace(/: ST(?=[,)\s])/g, '')
    .replace(/: MoreItem(?=[,)\s])/g, '')
    .replace(/: any/g, '')
    .replace(/\s+as\s+(MoreItem|A|R|WP|ST|string|number|boolean|any)\b/g, '')
    .replace(/: T/g, '')
}

const wsSrc = extractFn('function writeSucceeded(')
const gwSrc = extractFn('async function guardedWrite(')
const succeeded: (resp: any) => boolean = wsSrc
  ? new Function(stripTs(wsSrc) + '\nreturn writeSucceeded')()
  : () => true
const guarded: (p: Promise<any>) => Promise<any> = gwSrc
  ? new Function('writeSucceeded', stripTs(gwSrc) + '\nreturn guardedWrite')(succeeded)
  : async (p: Promise<any>) => await p

type Opts = { prompts?: string[]; confirm?: boolean; moreTab?: string }
function makeCtx(resp: any, opts: Opts = {}) {
  const calls: any = { api: [], toast: [], loadAppeals: 0, loadData: 0, switchMore: 0, confirm: null, prompts: [] }
  const api: any = new Proxy({}, {
    get: (_t, method: string) => async (...args: any[]) => {
      calls.api.push({ method, args })
      const r = typeof resp === 'function' ? resp(method, args) : resp
      if (r && r.__reject) throw new Error(r.__reject)
      return r
    },
  })
  const toast = {
    success: (m: string) => calls.toast.push(['success', m]),
    error: (m: string) => calls.toast.push(['error', m]),
    info: (m: string) => calls.toast.push(['info', m]),
  }
  const q = [...(opts.prompts ?? [])]
  const prompt = (...a: any[]) => { calls.prompts.push(a); return q.shift() ?? '' }
  const confirmMsg = async () => { calls.confirm = opts.confirm === undefined ? true : opts.confirm; return calls.confirm }
  const rows = { value: [{ id: 'R371-ROW' }] }
  calls.rows = rows
  const moreItems = { value: [] }
  const moreTab = { value: opts.moreTab ?? 'v2group' }
  const params = [api, toast, prompt, confirmMsg, succeeded, guarded, calls, rows, moreItems, moreTab]
  const names = ['__api', '__toast', '__prompt', '__confirmMsg', '__succeeded', '__guarded', '__calls', '__rows', '__moreItems', '__moreTab']
  const prelude = `
    const api = __api, toast = __toast, prompt = __prompt, confirmMsg = __confirmMsg,
      writeSucceeded = __succeeded, guardedWrite = __guarded,
      loadAppeals = async () => { __calls.loadAppeals++ },
      loadData = () => { __calls.loadData++ },
      loadRules = () => {}, loadWorkplaces = () => {}, loadSettings = () => {},
      switchMore = () => { __calls.switchMore++ },
      records = __rows, moreItems = __moreItems, moreTab = __moreTab,
      businessTodayYMD = () => '2026-11-22',
      qc = { invalidateQueries() {} }, am = { mutate() {} };
  `
  const compile = (sig: string, name: string) => {
    const body = extractFn(sig)
    if (!body) throw new Error('无法提取函数: ' + sig)
    return new Function(...names, prelude + '\n' + stripTs(body) + '\nreturn ' + name)(...params)
  }
  return { calls, compile }
}

const CASES: Array<{ id: string; sig: string; fn: string; args: any[]; prompts: string[]; confirm?: boolean; moreTab?: string; ok: string; eff: string }> = [
  { id: "auditAppealV2", sig: "async function auditAppealV2(a: A, auditStatus: string)", fn: "auditAppealV2", args: [{ id: 'R371-A' }, 'approved'], prompts: [], ok: "已审核申诉", eff: "loadAppeals" },
  { id: "checkAppeal", sig: "async function checkAppeal(a: A)", fn: "checkAppeal", args: [{ id: 'R371-A' }], prompts: [], ok: "已标记检查", eff: "loadAppeals" },
  { id: "updateAppealStatus", sig: "async function updateAppealStatus(a: A, status: string)", fn: "updateAppealStatus", args: [{ id: 'R371-A' }, 'processing'], prompts: [], ok: "已更新申诉状态", eff: "loadAppeals" },
  { id: "archiveAppeal", sig: "async function archiveAppeal(a: A)", fn: "archiveAppeal", args: [{ id: 'R371-A' }], prompts: [], confirm: true, ok: "已归档申诉", eff: "loadAppeals" },
  { id: "attV2Write/workplaceCreate", sig: "async function attV2Write(op: string)", fn: "attV2Write", args: ['workplaceCreate'], prompts: ["R371地点","R371地址"], ok: "考勤 v2 操作已提交", eff: "" },
  { id: "attCoreEntity/recordCreate", sig: "async function attCoreEntity(op: string)", fn: "attCoreEntity", args: ['recordCreate'], prompts: ["R371人员","2026-11-22 09:00"], ok: "考勤核心实体操作已提交", eff: "" },
  { id: "attWrite3/adminCreate", sig: "async function attWrite3(op: string)", fn: "attWrite3", args: ['adminCreate'], prompts: [], ok: "考勤操作已提交", eff: "" },
  { id: "attV2Ops/appealMgrStatus", sig: "async function attV2Ops(op: string)", fn: "attV2Ops", args: ['appealMgrStatus'], prompts: ["R371-OP"], ok: "考勤操作已提交", eff: "" },
  { id: "attAppealWrite/appealById", sig: "async function attAppealWrite(op: string)", fn: "attAppealWrite", args: ['appealById'], prompts: ["R371-APPEAL"], ok: "申诉流程已发起", eff: "" },
  { id: "reciveDetailById", sig: "async function reciveDetailById()", fn: "reciveDetailById", args: [], prompts: ["R371-DETAIL"], ok: "明细接收已提交", eff: "" },
  { id: "createAttDetail", sig: "async function createAttDetail()", fn: "createAttDetail", args: [], prompts: ["R371-PERSON","2026-11-22","normal"], ok: "考勤明细已创建", eff: "loadData" },
  { id: "reciveMobileDetail", sig: "async function reciveMobileDetail()", fn: "reciveMobileDetail", args: [], prompts: ["R371-PERSON","2026-11-22"], ok: "移动端接收已提交", eff: "" },
  { id: "myMobileDetail", sig: "async function myMobileDetail()", fn: "myMobileDetail", args: [], prompts: ["R371-PERSON"], ok: "我的考勤明细", eff: "" },
  { id: "importLeave", sig: "async function importLeave()", fn: "importLeave", args: [], prompts: ["sick","R371-PERSON"], ok: "请假导入已提交", eff: "" },
  { id: "importAttV2Records/daily", sig: "async function importAttV2Records(mode: string)", fn: "importAttV2Records", args: ['daily'], prompts: ["2026-11-22","R371-PERSON","OnDuty"], ok: "按日导入", eff: "" },
  { id: "checkMyRestDate", sig: "async function checkMyRestDate()", fn: "checkMyRestDate", args: [], prompts: ["2026-11-22"], ok: "休息日校验已提交", eff: "" },
  { id: "analyseOne", sig: "async function analyseOne(r: R)", fn: "analyseOne", args: [{ id: 'R371-D' }], prompts: [], ok: "已标记分析", eff: "" },
  { id: "archiveOne", sig: "async function archiveOne(r: R)", fn: "archiveOne", args: [{ id: 'R371-D' }], prompts: [], ok: "已归档", eff: "" },
  { id: "deleteDetail", sig: "async function deleteDetail(r: R)", fn: "deleteDetail", args: [{ id: 'R371-ROW' }], prompts: [], confirm: true, ok: "已删除", eff: "loadData" },
  { id: "addGroup", sig: "async function addGroup()", fn: "addGroup", args: [], prompts: ["R371组"], ok: "已新建考勤组", eff: "switchMore" },
  { id: "addShift", sig: "async function addShift()", fn: "addShift", args: [], prompts: ["R371班","09:00","18:00"], ok: "已新建班次", eff: "switchMore" },
  { id: "addLeave", sig: "async function addLeave()", fn: "addLeave", args: [], prompts: ["事假","2026-11-22 09:00","2026-11-22 18:00"], ok: "已新建请假", eff: "switchMore" },
  { id: "addSelfHoliday", sig: "async function addSelfHoliday()", fn: "addSelfHoliday", args: [], prompts: ["2026-11-22","R371"], ok: "假期申请已提交", eff: "switchMore" },
  { id: "syncDingding", sig: "async function syncDingding()", fn: "syncDingding", args: [], prompts: ["2026-11-01","2026-11-22"], ok: "钉钉同步已启动", eff: "" },
  { id: "syncQywx", sig: "async function syncQywx()", fn: "syncQywx", args: [], prompts: ["2026-11-01","2026-11-22"], ok: "企微同步已启动", eff: "" },
  { id: "statDingdingPerson", sig: "async function statDingdingPerson()", fn: "statDingdingPerson", args: [], prompts: ["2026","11"], ok: "个人月统计已触发", eff: "" },
  { id: "syncGroupSchedule", sig: "async function syncGroupSchedule()", fn: "syncGroupSchedule", args: [], prompts: ["G371","U371","2026-11"], ok: "群排班同步已受理", eff: "" },
  { id: "deleteMore", sig: "async function deleteMore(it: MoreItem)", fn: "deleteMore", args: [{ id: 'R371-M' }], prompts: [], confirm: true, ok: "已删除", eff: "switchMore" },
  { id: "addCycle", sig: "async function addCycle()", fn: "addCycle", args: [], prompts: ["2026","11"], ok: "已新建统计周期", eff: "" },
]

describe('R371 writeSucceeded / guardedWrite', () => {
  it('type:success 无权威 bool 仍判成功', () => expect(succeeded({ type: 'success', data: {} })).toBe(true))
  it('type 缺省且 success 非 false 判成功', () => expect(succeeded({ success: true, data: {} })).toBe(true))
  it('type:error 判失败', () => expect(succeeded({ type: 'error', message: 'x' })).toBe(false))
  it('success:false 判失败', () => expect(succeeded({ type: 'success', success: false })).toBe(false))
  it('undefined/null 判失败', () => { expect(succeeded(undefined)).toBe(false); expect(succeeded(null)).toBe(false) })
  it('guardedWrite 对 type:error 抛后端 message，对成功透传', async () => {
    await expect(guarded(Promise.resolve({ type: 'error', message: 'R371 backend error' }))).rejects.toThrow('R371 backend error')
    await expect(guarded(Promise.resolve({ type: 'success', data: { id: 'X' } }))).resolves.toMatchObject({ type: 'success' })
  })
})

describe('R371 真实写函数 200+type:error 不误报成功', () => {
  for (const c of CASES) {
    it(`${c.id}: type:error → toast.error 透传 message 且不落态`, async () => {
      const ctx = makeCtx({ type: 'error', message: 'R371 backend error' }, { prompts: c.prompts, confirm: (c as any).confirm, moreTab: (c as any).moreTab })
      const fn = ctx.compile(c.sig, c.fn)
      await fn(...c.args)
      const succ = ctx.calls.toast.filter((t: any) => t[0] === 'success')
      expect(succ.some((t: any) => String(t[1]).includes(c.ok)), '不应弹成功 toast').toBe(false)
      const err = ctx.calls.toast.filter((t: any) => t[0] === 'error')
      expect(err.some((t: any) => String(t[1]).includes('R371 backend error')), '应透传后端 message').toBe(true)
      if (c.eff) expect((ctx.calls as any)[c.eff], '成功副作用不应触发').toBe(0)
      if (c.id === 'deleteDetail') expect((ctx.calls.rows?.value || []).some((x: any) => x.id === 'R371-ROW'), '行不应被移除').toBe(true)
    })
    it(`${c.id}: type:success → 成功 toast 与副作用保持`, async () => {
      const ctx = makeCtx({ type: 'success', data: { id: 'R371-NEW', inserted: 1, count: 1 } }, { prompts: c.prompts, confirm: (c as any).confirm, moreTab: (c as any).moreTab })
      const fn = ctx.compile(c.sig, c.fn)
      await fn(...c.args)
      expect(ctx.calls.toast.some((t: any) => t[0] === 'success' && String(t[1]).includes(c.ok)), '成功 toast 应保留').toBe(true)
      if (c.eff) expect((ctx.calls as any)[c.eff], '成功副作用应触发').toBeGreaterThan(0)
    })
  }
})
