// @vitest-environment node
import { readFileSync } from "node:fs"
import { describe, expect, it, vi } from "vitest"

const SRC = new URL("./QueryStatementDesigner.vue", import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, "$1")
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
  const m = src.match(/function\s+queryStatementWriteSucceeded\s*\(\s*resp[^)]*\)\s*:\s*boolean\s*\{[\s\S]*?\n\}/)
  return m ? stripTs(m[0]) : "function queryStatementWriteSucceeded(resp){return false}"
}
type Ctx = {
  api: any, toast: any, queryClient: any, prompt: any, currentStatement: any,
  helper: any, __resp?: any, __flag?: string | null, __throw?: any
}
function build(name: string, ctx: Ctx): any {
  const body = stripTs(bodyOf(name))
  const pre = [
    "return (async function(__ctx){",
    "const { api, toast, queryClient, prompt, currentStatement, helper: queryStatementWriteSucceeded } = __ctx;",
    body, "})"
  ].join("\n")
  return Function('"use strict"; ' + pre)()
}
function context(overrides: Partial<Ctx> = {}): Ctx {
  const calls = { puts: [] as any[], success: [] as any[], error: [] as any[], invalidate: 0, prompts: [] as any[] }
  const put = vi.fn(async (url: string, payload: any) => {
    calls.puts.push([url, payload])
    if (overrides.__throw) throw overrides.__throw
    if (Object.prototype.hasOwnProperty.call(overrides, "__resp")) return overrides.__resp
    return { type: "success", success: { id: "S1", updated: 1 } }
  })
  const ctx: Ctx = {
    api: { put },
    toast: { success: vi.fn((m: string) => calls.success.push(m)), error: vi.fn((m: string) => calls.error.push(m)) },
    queryClient: { invalidateQueries: vi.fn(() => { calls.invalidate++ }) },
    prompt: vi.fn((_label: string, _def: string) => {
      const f = overrides.__flag === undefined ? "R406-FAKE-FLAG" : overrides.__flag
      calls.prompts.push(f)
      return f
    }),
    currentStatement: { value: { id: "S1", flag: "old-flag", name: "测试语句", sql: "select 1" } },
    helper: Function('"use strict"; ' + helperSource() + "; return queryStatementWriteSucceeded")()
  }
  Object.assign(ctx, overrides)
  ;(ctx as any).__calls = calls
  return ctx
}
const helper = () => Function('"use strict"; ' + helperSource() + "; return queryStatementWriteSucceeded")()
const run = async (overrides: Partial<Ctx> = {}) => {
  const ctx = context(overrides)
  await build("saveByFlag", ctx)(ctx)
  return { ctx, calls: (ctx as any).__calls }
}

describe("R406 QueryStatementDesigner saveByFlag 失败信封契约", () => {
  it("helper: 200+type:error statement not found 返回 false", () => {
    expect(helper()({ type: "error", message: "statement not found" })).toBe(false)
  })
  it("helper: success:false 返回 false", () => {
    expect(helper()({ success: false, message: "denied" })).toBe(false)
  })
  it("helper: type:success success:{id,updated:1} 返回 true（不强求 bool）", () => {
    expect(helper()({ type: "success", success: { id: "S9", updated: 1 } })).toBe(true)
  })
  it("helper: type:success success:{id} 返回 true（无权威 bool）", () => {
    expect(helper()({ type: "success", success: { id: "S9" } })).toBe(true)
  })
  it("helper: type 缺省且 success 非 false 返回 true", () => {
    expect(helper()({ data: {} })).toBe(true)
  })
  it("helper: 未知 type 返回 false", () => {
    expect(helper()({ type: "weird" })).toBe(false)
  })
  it("helper: undefined/null/非对象 返回 false", () => {
    expect(helper()(undefined)).toBe(false)
    expect(helper()(null)).toBe(false)
    expect(helper()("x")).toBe(false)
  })
  it("saveByFlag: 真后端 200+type:error 只 error toast 透传 message，不弹成功不刷新", async () => {
    const { calls } = await run({ __resp: { type: "error", message: "statement not found" } })
    expect(calls.error.join(" ")).toContain("statement not found")
    expect(calls.success).toEqual([])
    expect(calls.invalidate).toBe(0)
  })
  it("saveByFlag: success:false 视为失败不弹成功不刷新", async () => {
    const { calls } = await run({ __resp: { success: false, message: "R406 denied" } })
    expect(calls.error.join(" ")).toContain("R406 denied")
    expect(calls.success).toEqual([])
    expect(calls.invalidate).toBe(0)
  })
  it("saveByFlag: 非对象响应视为失败", async () => {
    const { calls } = await run({ __resp: undefined })
    expect(calls.success).toEqual([])
    expect(calls.error.length).toBeGreaterThan(0)
    expect(calls.invalidate).toBe(0)
  })
  it("saveByFlag: 真 success 才 toast.success 并刷新一次", async () => {
    const { calls } = await run({ __resp: { type: "success", success: { id: "S9", updated: 1 } } })
    expect(calls.success.join(" ")).toContain("按 flag")
    expect(calls.error).toEqual([])
    expect(calls.invalidate).toBe(1)
  })
  it("saveByFlag: type 缺省且 success 非 false 也视为成功", async () => {
    const { calls } = await run({ __resp: { data: { updated: 1 } } })
    expect(calls.success.length).toBe(1)
    expect(calls.invalidate).toBe(1)
  })
  it("saveByFlag: prompt 返回空 flag 时零写零 toast 零刷新", async () => {
    const { calls } = await run({ __flag: "" })
    expect(calls.puts).toEqual([])
    expect(calls.success).toEqual([])
    expect(calls.error).toEqual([])
    expect(calls.invalidate).toBe(0)
  })
  it("saveByFlag: 非2xx 抛错走 catch toast.error 按 flag 保存失败", async () => {
    const { calls } = await run({ __throw: { message: "HTTP 500" } })
    expect(calls.error.join(" ")).toContain("按 flag 保存失败")
    expect(calls.success).toEqual([])
  })
  it("saveByFlag: 失败信封下仍发出真实 PUT 且 URL/payload 正确（改判定不改端点）", async () => {
    const { calls } = await run({ __resp: { type: "error", message: "statement not found" }, __flag: "FLAG/X" })
    expect(calls.puts.length).toBe(1)
    expect(calls.puts[0][0]).toContain("/api/query/assemble/designer/statement/")
    expect(calls.puts[0][0]).toContain("FLAG%2FX")
    expect(calls.puts[0][1]).toHaveProperty("alias")
    expect(calls.puts[0][1]).toHaveProperty("data")
  })
})
