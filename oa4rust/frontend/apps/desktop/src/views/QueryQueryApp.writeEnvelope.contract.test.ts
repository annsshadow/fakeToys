// @vitest-environment node
import { readFileSync } from "node:fs"
import { describe, expect, it, vi } from "vitest"

const SRC = new URL("./QueryQueryApp.vue", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1")
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
    .replace(/\b([A-Za-z_$][\w$]*)\s*:\s*any\b/g, "$1")
    .replace(/: Surface/g, "")
    .replace(/\bconst\s+payload\s*=\s*\{/g, "const payload = {")
    .replace(/\bconst\s+([A-Za-z_$][\w$]*)\s*:\s*any\s*=/g, "const $1 =")
    .replace(/\blet\s+([A-Za-z_$][\w$]*)\s*:\s*any\s*=/g, "let $1 =")
    .replace(/\bcatch\s*\(\s*([A-Za-z_$][\w$]*)\s*:\s*any\s*\)/g, "catch ($1)")
    .replace(/\bas\s+any/g, "")
    .replace(/\bconst\s+([A-Za-z_$][\w$]*)\s*:\s*([A-Za-z_$][\w$<>|.\[\]]+)\s*=/g, "const $1 =")
    .replace(/\blet\s+([A-Za-z_$][\w$]*)\s*:\s*([A-Za-z_$][\w$<>|.\[\]]+)\s*=/g, "let $1 =")
    .replace(/\)\s*:\s*Promise<[^>]+>\s*\{/g, "){")
    .replace(/\)\s*:\s*[A-Za-z_$][\w$<>|.\[\]]*\s*\{/g, "){")
    .replace(/\bfunction\s+([A-Za-z_$][\w$]*)\s*:\s*boolean\s*\{/g, "function $1() {")
}
function helperSource(): string {
  const m = src.match(/function\s+querySurfaceWriteSucceeded\s*\(\s*resp[^)]*\)\s*:\s*boolean\s*\{[\s\S]*?\n\}/)
  return m ? stripTs(m[0]) : "function querySurfaceWriteSucceeded(resp){return false}"
}
const params = (name: string): string => {
  const m = src.match(new RegExp("function\\s+" + name + "\\s*\\(([^)]*)\\)"))
  return m ? m[1].split(",").map(s => s.trim()).filter(Boolean).map(s => s.split(":")[0].trim()).join(", ") : ""
}
type Ctx = {
  api: any, toast: any, confirmMsg: any, helper: any, loadSurfaces: any,
  surfaces: any, sfForm: any, sfEditingId: any, sfSaving: any, showSurfaceModal: any, s?: any,
  __postResp?: any, __confirm?: boolean
}
function build(name: string, ctx: Ctx): any {
  // 从真实函数签名提取形参名（剥离 :string 等类型标注），仅用于确认注入形状
  const sig = params(name)
  void sig
  const body = stripTs(bodyOf(name))
  const pre = [
    "return (async function(__ctx){",
    "const { api, toast, confirmMsg, loadSurfaces, surfaces, sfForm, sfEditingId, sfSaving, showSurfaceModal, helper: querySurfaceWriteSucceeded } = __ctx;",

    "var s = __ctx.s;",
    body, "})"
  ].join("\n")
  const AsyncFn = Function('"use strict"; ' + pre)
  return AsyncFn()
}
function context(overrides: Partial<Ctx> = {}): Ctx {
  const calls = { posts: [] as any[], success: [] as any[], error: [] as any[], reload: 0 }
  const ctx: Ctx = {
    api: { post: vi.fn(async (...args: any[]) => { calls.posts.push(args); return overrides.__postResp ?? { type: "success", data: {} } }) },
    toast: { success: vi.fn((m: string) => calls.success.push(m)), error: vi.fn((m: string) => calls.error.push(m)) },
    confirmMsg: vi.fn(async () => overrides.__confirm ?? true),
    helper: Function('"use strict"; ' + helperSource() + "; return querySurfaceWriteSucceeded")(),
    loadSurfaces: vi.fn(async () => { calls.reload++ }),
    surfaces: { value: [{ id: "S1", name: "旧表面" }, { id: "S2", name: "保留表面" }] },
    sfForm: { value: { name: "新表面", category: "cat", content: "query" } },
    sfEditingId: { value: "S1" },
    sfSaving: { value: false },
    showSurfaceModal: { value: true }
  }
  Object.assign(ctx, overrides)
  ;(ctx as any).__calls = calls
  return ctx
}

describe("R402 QueryQueryApp 表面写链失败信封契约", () => {
  const assertHelper = () => {
    const h = Function('"use strict"; ' + helperSource() + "; return querySurfaceWriteSucceeded")()
    expect(typeof h).toBe("function")
    return h
  }
  it("helper: type:error 返回 false", () => {
    expect(assertHelper()({ type: "error", message: "query surface not found" })).toBe(false)
  })
  it("helper: success:false 返回 false", () => {
    expect(assertHelper()({ success: false, message: "nope" })).toBe(false)
  })
  it("helper: type:success + saved:1 返回 true（不强求 bool）", () => {
    expect(assertHelper()({ type: "success", data: { saved: 1 } })).toBe(true)
  })
  it("helper: type:success + updated:1 返回 true（不强求 bool）", () => {
    expect(assertHelper()({ type: "success", data: { updated: 1 } })).toBe(true)
  })
  it("helper: type 缺省且 success 非 false 返回 true", () => {
    expect(assertHelper()({ data: {} })).toBe(true)
  })
  it("helper: undefined/null 返回 false", () => {
    expect(assertHelper()(undefined)).toBe(false)
    expect(assertHelper()(null)).toBe(false)
  })

  const runSave = async (resp: any, editing = true) => {
    const ctx = context({ __postResp: resp })
    if (!editing) ctx.sfEditingId = { value: null }
    await build("saveSurface", ctx)(ctx)
    return { ctx, calls: (ctx as any).__calls }
  }
  const runDelete = async (resp: any, confirmResp = true) => {
    const ctx = context({ __postResp: resp, __confirm: confirmResp, s: { id: "S1", name: "旧表面" } })
    await build("deleteSurface", ctx)(ctx)
    return { ctx, calls: (ctx as any).__calls }
  }
  it("saveSurface: 真后端 200+type:error 只 error toast，不弹成功/不关弹窗/不刷新", async () => {
    const { ctx, calls } = await runSave({ type: "error", message: "query surface not found" })
    expect(calls.error.join(" ")).toContain("query surface not found")
    expect(calls.success).toEqual([])
    expect(ctx.showSurfaceModal.value).toBe(true)
    expect(calls.reload).toBe(0)
  })
  it("saveSurface: success:false 也视为失败", async () => {
    const { calls } = await runSave({ success: false, message: "denied" })
    expect(calls.error.join(" ")).toContain("denied")
    expect(calls.success).toEqual([])
  })
  it("saveSurface: 编辑真 success+saved 保留 toast.success/关弹窗/刷新", async () => {
    const { ctx, calls } = await runSave({ type: "success", data: { saved: 1 } })
    expect(calls.success).toContain("表面已保存")
    expect(ctx.showSurfaceModal.value).toBe(false)
    expect(calls.reload).toBe(1)
  })
  it("saveSurface: 新建分支 create 端点成功也弹成功", async () => {
    const { calls } = await runSave({ type: "success", data: { id: "N1" } }, false)
    expect(calls.success).toContain("表面已保存")
    expect(calls.posts[0][0]).toBe("/api/query/assemble/surface/create")
  })
  it("saveSurface: 空名短路零写", async () => {
    const ctx = context({})
    ctx.sfForm = { value: { name: "   ", category: "cat", content: "" } }
    await build("saveSurface", ctx)(ctx)
    expect((ctx as any).__calls.posts).toEqual([])
  })
  it("deleteSurface: 真后端 200+type:error 只 error toast，保留本地行", async () => {
    const { ctx, calls } = await runDelete({ type: "error", message: "query surface not found" })
    expect(calls.error.join(" ")).toContain("query surface not found")
    expect(calls.success).toEqual([])
    expect(((ctx.surfaces as any).value as any[]).map(x => x.id)).toEqual(["S1", "S2"])
  })
  it("deleteSurface: success:false 保留本地行", async () => {
    const { ctx, calls } = await runDelete({ success: false, message: "denied" })
    expect(calls.error.join(" ")).toContain("denied")
    expect(((ctx.surfaces as any).value as any[]).length).toBe(2)
  })
  it("deleteSurface: 真 success+updated:1 才删行并弹成功", async () => {
    const { ctx, calls } = await runDelete({ type: "success", data: { updated: 1 } })
    expect(calls.success).toContain("表面已删除")
    expect(((ctx.surfaces as any).value as any[]).map(x => x.id)).toEqual(["S2"])
  })
  it("deleteSurface: confirm 取消零写且保留行", async () => {
    const { ctx, calls } = await runDelete({ type: "success" }, false)
    expect(calls.posts).toEqual([])
    expect(((ctx.surfaces as any).value as any[]).length).toBe(2)
    expect(calls.success).toEqual([])
  })
})
