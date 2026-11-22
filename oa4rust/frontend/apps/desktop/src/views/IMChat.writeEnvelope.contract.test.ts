// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R374 契约：PC IMChat.vue sendFileMessage 在 HTTP200 + type:error（或 success:false）
 * 或 data.fileUrl/url 缺失时必须错误提示、不得误报成功、不刷新不落态。
 * type 判定为主并附加权威 fileUrl/url 非空校验；不强制 bool。
 * 源码级函数体真执行（new Function 注入 api.upload/toast/refresh/document 桩），非文本断言。
 */
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
import { describe, expect, it } from "vitest"

const src = readFileSync(resolve(import.meta.dirname, "./IMChat.vue"), "utf8")

function bodyOf(source: string, signature: string): string {
  const start = source.indexOf(signature)
  if (start < 0) throw new Error("未找到函数: " + signature)
  const open = source.indexOf("{", start)
  let depth = 0
  for (let i = open; i < source.length; i++) {
    if (source[i] === "{") depth++
    else if (source[i] === "}") { depth--; if (depth === 0) return source.slice(open + 1, i) }
  }
  throw new Error("花括号未配对: " + signature)
}
function safeBody(signature: string): string | null {
  try { return bodyOf(src, signature) } catch { return null }
}
function stripTs(body: string): string {
  return body
    .replace(/\(([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|\[\]]*\)/g, "($1)")
    .replace(/catch \(([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|\[\]]*\)/g, "catch ($1)")
    .replace(/\b(const|let) ([A-Za-z_$][\w$]*): ?[A-Za-z_$][\w$<>|\[\]]*/g, "$1 $2")
    .replace(/\s+as\s+[A-Za-z_$][\w$<>|\[\]]*/g, "")
}

const helperBody = safeBody("function imUploadSucceeded(")
const imUploadSucceeded = helperBody
  ? (new Function("resp", stripTs(helperBody)) as (r: unknown) => boolean)
  : (() => false) as (r: unknown) => boolean
const sendBody = bodyOf(src, "function sendFileMessage(")

interface RunOpts { resp?: unknown; reject?: Error }
function run(opts: RunOpts) {
  const toasts = { success: [] as string[], error: [] as string[], info: [] as string[] }
  let refreshCalls = 0
  let uploadCalls = 0
  let resolveFired!: () => void
  let rejectFired!: (e: unknown) => void
  const fired = new Promise<void>((res, rej) => { resolveFired = res; rejectFired = rej })
  const selectedChat = { value: { id: "R374-CONV-1" } }
  const toast = {
    success: (msg: string): void => { toasts.success.push(msg) },
    error: (msg: string): void => { toasts.error.push(msg) },
    info: (msg: string): void => { toasts.info.push(msg) },
  }
  const api = {
    upload: async (_url: string, _form: unknown): Promise<unknown> => {
      uploadCalls++
      if (opts.reject) throw opts.reject
      return opts.resp
    },
  }
  let onChange: (() => Promise<void>) | null = null
  const input = {
    type: "",
    files: [{ name: "r374-契约.png", type: "image/png" }],
    set onchange(fn: () => Promise<void>) { onChange = fn },
    get onchange(): (() => Promise<void>) | null { return onChange },
    click(): void {
      Promise.resolve().then(() => onChange?.()).then(() => resolveFired(), (e) => rejectFired(e))
    },
  }
  const documentStub = { createElement: (tag: string): unknown => { if (tag !== "input") throw new Error("unexpected tag"); return input } }
  class FormDataStub {
    append(_key: string, _value: unknown, _name?: string): void { /* 契约不校验 multipart 内容 */ }
  }
  const compiled = new Function(
    "selectedChat", "toast", "encodeURIComponent", "document", "FormData",
    "api", "refreshFirstPage", "imUploadSucceeded",
    "function sendFileMessage() {" + stripTs(sendBody) + "}\nreturn sendFileMessage;",
  ) as (...args: unknown[]) => () => void
  const send = compiled(
    selectedChat, toast, encodeURIComponent, documentStub, FormDataStub,
    api, () => { refreshCalls++ }, imUploadSucceeded,
  )
  send()
  return { fired, toasts, getRefreshCalls: (): number => refreshCalls, getUploadCalls: (): number => uploadCalls }
}

describe("R374 IMChat.vue sendFileMessage 写链失败信封守卫", () => {
  const helperCases: Array<{ name: string; resp: unknown; want: boolean }> = [
    { name: "type:success + data.fileUrl", resp: { type: "success", data: { fileUrl: "/im/msg/download/374" } }, want: true },
    { name: "type:success + data.url", resp: { type: "success", data: { url: "/im/msg/download/374" } }, want: true },
    { name: "type缺省 + success非false + fileUrl", resp: { success: true, data: { fileUrl: "/im/msg/download/374" } }, want: true },
    { name: "type:error 即使带 fileUrl 也失败", resp: { type: "error", message: "R374 backend error", data: { fileUrl: "/x" } }, want: false },
    { name: "success:false 失败", resp: { type: "success", success: false, data: { fileUrl: "/x" } }, want: false },
    { name: "type:success 但 fileUrl/url 均缺失失败", resp: { type: "success", data: {} }, want: false },
  ]
  for (const c of helperCases) {
    it("helper: " + c.name, () => {
      expect(imUploadSucceeded(c.resp)).toBe(c.want)
    })
  }

  const errorResp = { type: "error", message: "后端拒绝：R374 upload not allowed" }
  it("type:error 错误 toast 透传后端 message，不成功不刷新", async () => {
    const ctx = run({ resp: errorResp })
    await ctx.fired
    expect(ctx.toasts.error.join("|")).toContain("后端拒绝：R374 upload not allowed")
    expect(ctx.toasts.success).toHaveLength(0)
    expect(ctx.getRefreshCalls()).toBe(0)
  })
  it("fileUrl/url 缺失错误 toast，不成功不刷新", async () => {
    const ctx = run({ resp: { type: "success", message: "后端拒绝：R374 missing fileUrl", data: {} } })
    await ctx.fired
    expect(ctx.toasts.error.join("|")).toContain("后端拒绝：R374 missing fileUrl")
    expect(ctx.toasts.success).toHaveLength(0)
    expect(ctx.getRefreshCalls()).toBe(0)
  })
  it("非2xx(HTTP 500) 仍走 catch toast.error，不误报成功", async () => {
    const ctx = run({ reject: new Error("HTTP 500: Internal Server Error") })
    await ctx.fired
    expect(ctx.toasts.error.join("|")).toContain("HTTP 500")
    expect(ctx.toasts.success).toHaveLength(0)
  })
  it("type:success + fileUrl 仍成功带文件名并刷新", async () => {
    const ctx = run({ resp: { type: "success", data: { fileUrl: "/im/msg/download/374" } } })
    await ctx.fired
    expect(ctx.toasts.success.join("|")).toContain("r374-契约.png")
    expect(ctx.toasts.error).toHaveLength(0)
    expect(ctx.getRefreshCalls()).toBe(1)
  })
  it("type缺省 + success非false + fileUrl 仍成功", async () => {
    const ctx = run({ resp: { success: true, data: { fileUrl: "/im/msg/download/374" } } })
    await ctx.fired
    expect(ctx.toasts.success.join("|")).toContain("r374-契约.png")
    expect(ctx.toasts.error).toHaveLength(0)
    expect(ctx.getRefreshCalls()).toBe(1)
  })
  it("type:success + data.url 仍成功", async () => {
    const ctx = run({ resp: { type: "success", data: { url: "/im/msg/download/374" } } })
    await ctx.fired
    expect(ctx.toasts.success.join("|")).toContain("r374-契约.png")
    expect(ctx.toasts.error).toHaveLength(0)
    expect(ctx.getRefreshCalls()).toBe(1)
  })
})
