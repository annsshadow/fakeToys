// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R363 契约：PC MeetingApp 五写端点在 HTTP 200 + type:success + bool:false 或 200 + type:error 下
 * 必须错误提示、不得误报成功；仅 type:success（或 type 缺省且 success 非 false）且对应 bool:true 才成功。
 * R362 审计坐实：假 ID 真实可达 200+type:success+bool:false，前端五函数无条件成功 toast。
 * 源码级函数体真执行（new AsyncFn），不引入新依赖。
 */
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
import { describe, expect, it } from "vitest"

const src = readFileSync(resolve(import.meta.dirname, "./MeetingApp.vue"), "utf8")

function bodyOf(source: string, signature: string): string {
  const start = source.indexOf(signature)
  if (start < 0) throw new Error("未找到函数: " + signature)
  const open = source.indexOf("{", start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === "{") depth++
    else if (source[i] === "}") {
      depth--
      if (depth === 0) return source.slice(open + 1, i)
    }
  }
  throw new Error("花括号未配对: " + signature)
}
function safeBody(signature: string): string | null {
  try { return bodyOf(src, signature) } catch { return null }
}
function stripTs(body: string): string {
  return body
    .replace(/const r: any/g, "const r")
    .replace(/catch \(e: any\)/g, "catch (e)")
}

const helperBody = safeBody("function meetingWriteSucceeded(")
const succeeded = helperBody
  ? (new Function("resp", "flagKey", stripTs(helperBody)) as (r: unknown, k: string) => boolean)
  : (() => false) as (r: unknown, k: string) => boolean

const AsyncFn = Object.getPrototypeOf(async function () {}).constructor
const FN_PARAMS = "api,toast,loadMeetings,prompt,confirmMsg,meetingCompletedFromResponse,meetingWriteSucceeded,encodeURIComponent,m"
function compile(signature: string): ((...args: unknown[]) => Promise<void>) | null {
  const body = safeBody(signature)
  return body ? (new AsyncFn(FN_PARAMS, stripTs(body)) as (...args: unknown[]) => Promise<void>) : null
}

interface CtxOpts { reject?: Error; prompts?: string[]; confirmed?: boolean }
function makeCtx(resp: unknown, opts: CtxOpts = {}) {
  const toasts: { success: string[]; info: string[]; error: string[] } = { success: [], info: [], error: [] }
  let loadCount = 0
  const queue = opts.prompts ? opts.prompts.slice() : ["2026-11-22 10:00:00", "2026-11-22 11:00:00"]
  const api = { post: async (): Promise<unknown> => { if (opts.reject) throw opts.reject; return resp } }
  const toast = {
    success: (t: string): void => { toasts.success.push(t) },
    info: (t: string): void => { toasts.info.push(t) },
    error: (t: string): void => { toasts.error.push(t) },
  }
  const loadMeetings = (): void => { loadCount++ }
  const promptStub = (): string => queue.shift() ?? ""
  const confirmMsg = async (): Promise<boolean> => opts.confirmed !== false
  const mcfr = (r: unknown): boolean => !!(r && (r as any).data && (r as any).data.completed === true)
  const meeting: any = { id: "M363", status: "wait" }
  const run = async (fn: ((...args: unknown[]) => Promise<void>) | null): Promise<void> => {
    expect(fn, "函数体应可提取").not.toBeNull()
    await fn!(api, toast, loadMeetings, promptStub, confirmMsg, mcfr, succeeded, encodeURIComponent, meeting)
  }
  return { run, toasts, meeting, loadCount: (): number => loadCount }
}
function all(c: ReturnType<typeof makeCtx>): string[] {
  return c.toasts.success.concat(c.toasts.info, c.toasts.error)
}

const FNS = [
  { name: "approveMeeting", sig: "async function approveMeeting(", flag: "confirmed", ok: "" },
  { name: "joinMeeting", sig: "async function joinMeeting(", flag: "accepted", ok: "已加入会议" },
  { name: "leaveMeeting", sig: "async function leaveMeeting(", flag: "rejected", ok: "" },
  { name: "modifyMeetingTime", sig: "async function modifyMeetingTime(", flag: "modified", ok: "会议时间已更新" },
  { name: "markMeetingCompleted", sig: "async function markMeetingCompleted(", flag: "completed", ok: "已标记完成" },
]

describe("R363 meetingWriteSucceeded 判定 helper 真执行", () => {
  it("type:success + data.flag:true 判成功", () => {
    expect(succeeded({ type: "success", data: { accepted: true } }, "accepted")).toBe(true)
  })
  it("type:success + data.flag:false 判失败", () => {
    expect(succeeded({ type: "success", data: { accepted: false } }, "accepted")).toBe(false)
  })
  it("type:error 即使 flag:true 也判失败", () => {
    expect(succeeded({ type: "error", message: "x", data: { accepted: true } }, "accepted")).toBe(false)
  })
  it("success:false 无 type 即使 flag:true 也判失败", () => {
    expect(succeeded({ success: false, data: { accepted: true } }, "accepted")).toBe(false)
  })
  it("type 缺省且 success 非 false + flag:true 判成功", () => {
    expect(succeeded({ data: { accepted: true } }, "accepted")).toBe(true)
  })
  it("flag 平铺在信封根同样判成功", () => {
    expect(succeeded({ type: "success", accepted: true }, "accepted")).toBe(true)
  })
})

describe("R363 五写端点结构（flagKey / helper / 端点路径）", () => {
  it("五函数体均调用判定 helper", () => {
    for (const c of FNS) {
      const b = safeBody(c.sig) ?? ""
      expect(b, c.name).toContain("meetingWriteSucceeded")
    }
  })
  it("五个调用点各自传入对应 flagKey", () => {
    for (const c of FNS) {
      const b = safeBody(c.sig) ?? ""
      expect(b, c.name).toContain("meetingWriteSucceeded(r, " + String.fromCharCode(34) + c.flag + String.fromCharCode(34) + ")")
    }
  })
  it("端点路径保持不变", () => {
    expect(src).toContain("/confirm/allow")
    expect(src).toContain("/accept")
    expect(src).toContain("/reject")
    expect(src).toContain("/modify/starttime")
    expect(src).toContain("/manual/completed")
  })
})

describe("R363 五函数真执行 fail-loud", () => {
  for (const c of FNS) {
    it(c.name + " bool:false 不误报成功并弹缺省错误文案", async () => {
      const ctx = makeCtx({ type: "success", data: { [c.flag]: false } })
      await ctx.run(compile(c.sig))
      if (c.ok) expect(all(ctx), c.name).not.toContain(c.ok)
      expect(all(ctx), c.name).toContain("操作未生效，请刷新后重试")
      expect(ctx.loadCount(), c.name).toBe(0)
    })
    it(c.name + " type:error 显示后端 message 且不误报成功", async () => {
      const ctx = makeCtx({ type: "error", message: "后端拒绝：会议已结束", data: {} })
      await ctx.run(compile(c.sig))
      if (c.ok) expect(all(ctx), c.name).not.toContain(c.ok)
      expect(all(ctx), c.name).toContain("后端拒绝：会议已结束")
      expect(ctx.loadCount(), c.name).toBe(0)
    })
    it(c.name + " 非2xx reject 走 catch 显示错误且不刷新", async () => {
      const ctx = makeCtx(undefined, { reject: new Error("HTTP 403") })
      await ctx.run(compile(c.sig))
      expect(all(ctx).join(" "), c.name).toContain("HTTP 403")
      expect(ctx.loadCount(), c.name).toBe(0)
    })
  }
  it("joinMeeting bool:true 弹成功 info 并刷新", async () => {
    const ctx = makeCtx({ type: "success", data: { accepted: true } })
    await ctx.run(compile("async function joinMeeting("))
    expect(all(ctx)).toContain("已加入会议")
    expect(ctx.loadCount()).toBe(1)
  })
  it("modifyMeetingTime bool:true 弹成功并刷新", async () => {
    const ctx = makeCtx({ type: "success", data: { modified: true } })
    await ctx.run(compile("async function modifyMeetingTime("))
    expect(all(ctx)).toContain("会议时间已更新")
    expect(ctx.loadCount()).toBe(1)
  })
  it("markMeetingCompleted bool:true 落态 completed 并弹成功", async () => {
    const ctx = makeCtx({ type: "success", data: { completed: true } })
    await ctx.run(compile("async function markMeetingCompleted("))
    expect(ctx.meeting.status).toBe("completed")
    expect(all(ctx)).toContain("已标记完成")
  })
  it("markMeetingCompleted bool:false 不落态不弹成功", async () => {
    const ctx = makeCtx({ type: "success", data: { completed: false } })
    await ctx.run(compile("async function markMeetingCompleted("))
    expect(ctx.meeting.status).toBe("wait")
    expect(all(ctx)).not.toContain("已标记完成")
    expect(all(ctx)).toContain("操作未生效，请刷新后重试")
  })
  it("markMeetingCompleted 取消守卫：confirmMsg=false 直接返回", async () => {
    const ctx = makeCtx({ type: "success", data: { completed: true } }, { confirmed: false })
    await ctx.run(compile("async function markMeetingCompleted("))
    expect(all(ctx)).toHaveLength(0)
    expect(ctx.loadCount()).toBe(0)
    expect(ctx.meeting.status).toBe("wait")
  })
})
