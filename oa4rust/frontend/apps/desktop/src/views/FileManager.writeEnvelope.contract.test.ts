// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R369 契约：PC FileManager.vue createRecyclableFile/deleteFile 在后端 HTTP200 +
 * type:error（或 success:false）时必须错误提示、不得误报成功、不得造空 id 记录/落态。
 * type 判定为主：type:success 或(type 缺省且 success 非 false)才成功；不强制 bool。
 * create 成功信封 success({id,name,...}) 无 bool，故额外要求 data.id 非空。
 * 源码级函数体真执行（new Function/new AsyncFn 注入 api/toast/deleteMutation 桩），非文本断言。
 */
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
import { describe, expect, it } from "vitest"

const src = readFileSync(resolve(import.meta.dirname, "./FileManager.vue"), "utf8")

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
    .replace(/const resp: any/g, "const resp")
    .replace(/\(r: any\)/g, "(r)")
    .replace(/\(e: any\)/g, "(e)")
    .replace(/catch \(e: any\)/g, "catch (e)")
    .replace(/as any/g, "")
}

const helperBody = safeBody("function writeSucceeded(")
const succeeded = helperBody
  ? (new Function("resp", stripTs(helperBody)) as (r: unknown) => boolean)
  : (() => false) as (r: unknown) => boolean
const AsyncFn = Object.getPrototypeOf(async function () {}).constructor

function ref<T>(v: T): { value: T } { return { value: v } }
interface Toasts { success: string[]; error: string[] }
interface CtxOpts { resp: unknown; confirmed?: boolean; reject?: Error; deletingId?: string }
function makeCtx(opts: CtxOpts) {
  const toasts: Toasts = { success: [], error: [] }
  const toast = {
    success: (msg: string): void => { toasts.success.push(msg) },
    error: (msg: string): void => { toasts.error.push(msg) },
  }
  const recyclableFiles = ref<Array<{ id: string; name: string; type: string }>>([
    { id: "f369-old", name: "旧文件", type: "file" },
  ])
  const session = { state: { user: { unique: "u369" } } }
  const api = {
    post: async (): Promise<unknown> => { if (opts.reject) throw opts.reject; return opts.resp },
    delete: async (): Promise<unknown> => { if (opts.reject) throw opts.reject; return opts.resp },
  }
  let mutateCalls = 0
  type MutOptions = { onSuccess?: (r: unknown) => void; onError?: (e: unknown) => void; onSettled?: () => void }
  const deleteMutation = {
    mutate: (_id: string, options: MutOptions): void => {
      mutateCalls++
      if (opts.reject) options.onError?.(opts.reject)
      else options.onSuccess?.(opts.resp)
      options.onSettled?.()
    },
  }
  const noop = (): void => {}

  const runCreate = async (): Promise<void> => {
    const body = safeBody("async function createRecyclableFile(")
    expect(body, "createRecyclableFile 函数体应可提取").not.toBeNull()
    const fn = new AsyncFn("prompt", "api", "session", "writeSucceeded", "toast", "recyclableFiles", stripTs(body!)) as (...a: unknown[]) => Promise<void>
    await fn((): string => "R369新文件", api, session, succeeded, toast, recyclableFiles)
  }
  const file = { id: "f369-old", name: "旧文件", type: "file" }
  const runDelete = async (): Promise<void> => {
    const body = safeBody("async function deleteFile(")
    expect(body, "deleteFile 函数体应可提取").not.toBeNull()
    const fn = new AsyncFn("deletingId", "confirmMsg", "deleteMutation", "toast", "loadRecycleBin", "recyclableFiles", "writeSucceeded", "f", stripTs(body!)) as (...a: unknown[]) => Promise<void>
    await fn(ref(opts.deletingId ?? ""), async (): Promise<boolean> => opts.confirmed !== false, deleteMutation, toast, noop, recyclableFiles, succeeded, file)
  }
  return { toasts, recyclableFiles, runCreate, runDelete, getMutateCalls: (): number => mutateCalls, toast, api, succeeded }
}
const all = (c: ReturnType<typeof makeCtx>): string => c.toasts.success.concat(c.toasts.error).join("|")

describe("R369 writeSucceeded helper 真执行（type 判定为主，不强制 bool）", () => {
  it("type:success 判成功（无 bool 也成功）", () => { expect(succeeded({ type: "success", data: { id: "f1" } })).toBe(true) })
  it("type 缺省且 success 非 false 判成功", () => { expect(succeeded({ success: true, data: {} })).toBe(true) })
  it("type:success 但 success:false 判失败", () => { expect(succeeded({ type: "success", success: false, data: {} })).toBe(false) })
  it("type:error 判失败", () => { expect(succeeded({ type: "error", message: "file not found" })).toBe(false) })
  it("success:false 判失败", () => { expect(succeeded({ success: false, message: "x" })).toBe(false) })
  it("undefined/null/非对象判失败", () => { expect(succeeded(undefined)).toBe(false); expect(succeeded(null)).toBe(false); expect(succeeded("x")).toBe(false) })
})

describe("R369 createRecyclableFile type:error 误报成功修复", () => {
  it("200+type:error：不弹成功、toast.error 透传后端 message、不造空 id 记录", async () => {
    const c = makeCtx({ resp: { type: "error", message: "name, person and reference_type are required", success: false, data: {} } })
    await c.runCreate()
    expect(c.toasts.success.join("|")).not.toContain("文件已创建")
    expect(all(c)).toContain("name, person and reference_type are required")
    expect(c.recyclableFiles.value.length).toBe(1)
  })
  it("200+success:false：不弹成功、不造记录", async () => {
    const c = makeCtx({ resp: { type: "success", success: false, message: "业务失败", data: {} } })
    await c.runCreate()
    expect(c.toasts.success.length).toBe(0)
    expect(c.recyclableFiles.value.length).toBe(1)
  })
  it("200+type:success 但 data 无 id：不造空 id 记录且错误提示", async () => {
    const c = makeCtx({ resp: { type: "success", success: true, data: {} } })
    await c.runCreate()
    expect(c.toasts.success.join("|")).not.toContain("文件已创建")
    expect(c.recyclableFiles.value.length).toBe(1)
    expect(c.toasts.error.length).toBeGreaterThan(0)
  })
  it("200+type:success + data.id：成功提示且造记录", async () => {
    const c = makeCtx({ resp: { type: "success", success: true, data: { id: "f369-new" } } })
    await c.runCreate()
    expect(c.toasts.success.join("|")).toContain("文件已创建")
    expect(c.recyclableFiles.value.length).toBe(2)
    expect(c.recyclableFiles.value[0]?.id).toBe("f369-new")
  })
  it("非2xx：catch 错误提示且不误报成功", async () => {
    const c = makeCtx({ resp: {}, reject: new Error("HTTP 500: Internal Server Error") })
    await c.runCreate()
    expect(c.toasts.success.length).toBe(0)
    expect(all(c)).toContain("HTTP 500")
    expect(c.recyclableFiles.value.length).toBe(1)
  })
})

describe("R369 deleteFile onSuccess type:error 误报成功修复", () => {
  it("200+type:error：toast.error 透传 message、不弹成功、本地列表不落态（旧行保留）", async () => {
    const c = makeCtx({ resp: { type: "error", message: "file not found", success: false } })
    await c.runDelete()
    expect(c.toasts.success.join("|")).not.toContain("文件已删除")
    expect(all(c)).toContain("file not found")
    expect(c.recyclableFiles.value.some((x) => x.id === "f369-old")).toBe(true)
  })
  it("200+success:false：不弹成功、不落态", async () => {
    const c = makeCtx({ resp: { type: "success", success: false, message: "业务失败" } })
    await c.runDelete()
    expect(c.toasts.success.length).toBe(0)
    expect(c.recyclableFiles.value.some((x) => x.id === "f369-old")).toBe(true)
  })
  it("200+type:success：成功提示且本地过滤掉该行", async () => {
    const c = makeCtx({ resp: { type: "success", success: true, data: { deleted: true } } })
    await c.runDelete()
    expect(c.toasts.success.join("|")).toContain("文件已删除")
    expect(c.recyclableFiles.value.some((x) => x.id === "f369-old")).toBe(false)
  })
  it("confirm 取消：0 写请求、0 toast", async () => {
    const c = makeCtx({ resp: { type: "success", success: true }, confirmed: false })
    await c.runDelete()
    expect(c.getMutateCalls()).toBe(0)
    expect(c.toasts.success.length + c.toasts.error.length).toBe(0)
  })
  it("deletingId 守卫：已有删除任务时直接返回，0 写请求", async () => {
    const c = makeCtx({ resp: { type: "success", success: true }, deletingId: "f369-old" })
    await c.runDelete()
    expect(c.getMutateCalls()).toBe(0)
  })
})
