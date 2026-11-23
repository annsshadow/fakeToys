// @vitest-environment node
import { readFileSync } from "node:fs"
import { describe, expect, it, vi } from "vitest"

const SRC = new URL("./QueryViewDesignerApp.vue", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1")
const src = readFileSync(SRC, "utf8")

function bodyOf(name: string): string {
  const i = src.indexOf("function " + name + "(")
  if (i < 0) throw new Error("function " + name + " not found")
  let depth = 0, start = -1
  for (let j = i; j < src.length; j++) {
    if (src[j] === "{") { if (start < 0) start = j; depth++ }
    else if (src[j] === "}" && start >= 0) { depth--; if (depth === 0) return src.slice(start + 1, j) }
  }
  throw new Error("function " + name + " body not balanced")
}
function stripTs(code: string): string {
  return code
    .replace(/\.(get|post|put|delete)<[^>]*>\(/g, ".$1(")
    .replace(/\b([A-Za-z_$][\w$]*)\s*:\s*any\b/g, "$1")
    .replace(/\bas\s+any/g, "")
    .replace(/\bcatch\s*\(\s*([A-Za-z_$][\w$]*)\s*:\s*any\s*\)/g, "catch ($1)")
    .replace(/\bconst\s+([A-Za-z_$][\w$]*)\s*:\s*([A-Za-z_$][\w$<>|.[\]]+)\s*=/g, "const $1 =")
    .replace(/\blet\s+([A-Za-z_$][\w$]*)\s*:\s*([A-Za-z_$][\w$<>|.[\]]+)\s*=/g, "let $1 =")
    .replace(/\)\s*:\s*Promise<[^>]+>\s*\{/g, "){")
    .replace(/\)\s*:\s*[A-Za-z_$][\w$<>|.[\]]*\s*\{/g, "){")
}
function helperSource(): string {
  const m = src.match(/function\s+queryViewDesignerWriteSucceeded\s*\(\s*resp[^)]*\)\s*:\s*boolean\s*\{[\s\S]*?\n\}/)
  return m ? stripTs(m[0]) : "function queryViewDesignerWriteSucceeded(resp){return false}"
}
type Ctx = {
  api: any, designerPaths: any, toast: any, loadViews: any, activeId: any, form: any,
  queryFlag: any, canSave: any, saving: any, serializeViewDefinition: any,
  helper: any, result?: any, resultTitle?: any, viewRuntimePayload?: any, __saveResp?: any, __simResp?: any, __postResp?: any, __putThrow?: any
}
function build(name: string, ctx: Ctx): any {
  const body = stripTs(bodyOf(name))
  const pre = [
    "return (async function(__ctx){",
    "const { api, designerPaths, toast, loadViews, activeId, form, queryFlag, canSave, saving, serializeViewDefinition, helper: queryViewDesignerWriteSucceeded } = __ctx;",
    "var saveView = __ctx.__saveViewFn || (async function(){}); var result = __ctx.result || {value:undefined}; var resultTitle = __ctx.resultTitle || {value:''}; var viewRuntimePayload = __ctx.viewRuntimePayload || function(){ return {} };",
    body, "})"
  ].join("\n")
  return Function('"use strict"; ' + pre)()
}
function context(overrides: Partial<Ctx> = {}): Ctx {
  const calls = { puts: [] as any[], posts: [] as any[], success: [] as any[], error: [] as any[], reload: 0 }
  const put = vi.fn(async (url: string, payload: any) => {
    calls.puts.push([url, payload])
    if (overrides.__putThrow) throw overrides.__putThrow
    if (String(url).indexOf("simulate") >= 0) return overrides.__simResp ?? { type: "success", data: { ok: 1 } }
    return overrides.__saveResp ?? { type: "success", data: { id: "V1", viewFlag: "vf" } }
  })
  const post = vi.fn(async (url: string, payload: any) => {
    calls.posts.push([url, payload])
    return overrides.__postResp ?? overrides.__saveResp ?? { type: "success", data: { id: "V2", viewFlag: "vf2" } }
  })
  const ctx: Ctx = {
    api: { put, post },
    designerPaths: {
      viewSave: (id: string) => "/api/query/assemble/designer/view/" + id,
      viewCreate: "/api/query/assemble/designer/view",
      viewSimulate: (id: string) => "/api/query/assemble/designer/view/simulate/" + id,
      viewBundle: (id: string) => "/api/query/assemble/designer/view/bundle/" + id
    },
    toast: { success: vi.fn((m: string) => calls.success.push(m)), error: vi.fn((m: string) => calls.error.push(m)) },
    loadViews: vi.fn(async () => { calls.reload++ }),
    activeId: { value: "V1" },
    form: { value: { name: "测试视图", viewFlag: "old-vf", definition: { sql: "select 1" } } },
    queryFlag: { value: "Q1" },
    canSave: { value: true },
    saving: { value: false },
    serializeViewDefinition: vi.fn(() => "{}"),
    result: { value: null },
    resultTitle: { value: "" },
    viewRuntimePayload: vi.fn(() => ({})),
    helper: Function('"use strict"; ' + helperSource() + "; return queryViewDesignerWriteSucceeded")()
  }
  Object.assign(ctx, overrides)
  ;(ctx as any).__calls = calls
  ;(ctx as any).__saveViewFn = async () => build("saveView", ctx)(ctx)
  return ctx
}

describe("R404 QueryViewDesignerApp saveView 失败信封契约", () => {
  const assertHelper = () => {
    const h = Function('"use strict"; ' + helperSource() + "; return queryViewDesignerWriteSucceeded")()
    expect(typeof h).toBe("function")
    return h
  }
  it("helper: type:error view not found 返回 false", () => {
    expect(assertHelper()({ type: "error", message: "view not found" })).toBe(false)
  })
  it("helper: success:false 返回 false", () => {
    expect(assertHelper()({ success: false, message: "denied" })).toBe(false)
  })
  it("helper: type:success success:{id,viewFlag} 返回 true（不强求 bool）", () => {
    expect(assertHelper()({ type: "success", success: { id: "V9", viewFlag: "vf9" } })).toBe(true)
  })
  it("helper: type:success success:{id,updated} 返回 true（不强求 bool）", () => {
    expect(assertHelper()({ type: "success", success: { id: "V9", updated: 1 } })).toBe(true)
  })
  it("helper: type 缺省且 success 非 false 返回 true", () => {
    expect(assertHelper()({ data: {} })).toBe(true)
  })
  it("helper: 未知 type 返回 false", () => {
    expect(assertHelper()({ type: "weird" })).toBe(false)
  })
  it("helper: undefined/null 返回 false", () => {
    expect(assertHelper()(undefined)).toBe(false)
    expect(assertHelper()(null)).toBe(false)
  })
  const runSave = async (resp: any, activeIdVal = "V1", canSaveVal = true) => {
    const ctx = context({ __saveResp: resp })
    ctx.activeId = { value: activeIdVal }
    ctx.canSave = { value: canSaveVal }
    await build("saveView", ctx)(ctx)
    return { ctx, calls: (ctx as any).__calls }
  }
  it("saveView: 真后端 200+type:error 只 error toast，不弹成功不写id/viewFlag不loadViews", async () => {
    const { ctx, calls } = await runSave({ type: "error", message: "view not found" })
    expect(calls.error.join(" ")).toContain("view not found")
    expect(calls.success).toEqual([])
    expect(ctx.activeId.value).toBe("V1")
    expect(ctx.form.value.viewFlag).toBe("old-vf")
    expect(calls.reload).toBe(0)
  })
  it("saveView: success:false 视为失败", async () => {
    const { calls } = await runSave({ success: false, message: "R404 denied" })
    expect(calls.error.join(" ")).toContain("R404 denied")
    expect(calls.success).toEqual([])
    expect(calls.reload).toBe(0)
  })
  it("saveView: 编辑真 success+viewFlag 才写id/viewFlag+toast.success+loadViews", async () => {
    const { ctx, calls } = await runSave({ type: "success", data: { id: "V9", viewFlag: "vf9" } })
    expect(calls.success).toContain("视图已保存")
    expect(ctx.activeId.value).toBe("V9")
    expect(ctx.form.value.viewFlag).toBe("vf9")
    expect(calls.reload).toBe(1)
  })
  it("saveView: 新建分支 viewCreate 成功也弹成功", async () => {
    const { calls } = await runSave({ type: "success", data: { id: "V2", viewFlag: "vf2" } }, "")
    expect(calls.success).toContain("视图已保存")
    expect(calls.posts[0][0]).toBe("/api/query/assemble/designer/view")
  })
  it("saveView: canSave=false 短路零写", async () => {
    const { calls } = await runSave({ type: "success" }, "V1", false)
    expect(calls.puts).toEqual([])
    expect(calls.posts).toEqual([])
  })
  it("saveView: 非2xx 抛错走 catch toast.error 保存失败", async () => {
    const ctx = context({ __putThrow: { message: "HTTP 500" } })
    await build("saveView", ctx)(ctx)
    expect(((ctx as any).__calls.error as string[]).join(" ")).toContain("保存失败")
    expect(((ctx as any).__calls.success as string[])).toEqual([])
  })
  it("simulate: 复用 saveView，保存响应 type:error 时不弹视图已保存", async () => {
    const ctx = context({ __saveResp: { type: "error", message: "view not found" } })
    await build("simulate", ctx)(ctx)
    expect(((ctx as any).__calls.success as string[])).toEqual([])
    expect(((ctx as any).__calls.error as string[]).join(" ")).toContain("view not found")
  })
  it("simulate: 保存成功后仍执行模拟并设 result，无独立成功 toast", async () => {
    const ctx = context({ __saveResp: { type: "success", data: { id: "V1", viewFlag: "vf" } } })
    await build("simulate", ctx)(ctx)
    expect(((ctx as any).__calls.success as string[])).toContain("视图已保存")
    expect((ctx.result as any).value).toEqual({ ok: 1 })
  })
  it("bundle: 无成功 toast，不动", async () => {
    const ctx = context({})
    await build("bundle", ctx)(ctx)
    expect(((ctx as any).__calls.success as string[])).toEqual([])
  })
})
