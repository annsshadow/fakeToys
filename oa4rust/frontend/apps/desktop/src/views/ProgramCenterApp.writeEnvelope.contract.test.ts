// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
// @vitest-environment node
/**
 * R367 契约：PC ProgramCenterApp 写链在后端 HTTP200 + type:error（或 success:false）时
 * 必须错误提示、不得误报成功；type:success 或 type 缺省且 success 非 false 才判成功。
 * 后端 ActionResult 的 288 处 success(json) 无专门 bool，故不强制 bool===true。
 * 源码级函数体真执行（new AsyncFn 注入 api/toast 桩），不引入新依赖。
 */
import { readFileSync } from "node:fs"
import { resolve } from "node:path"
import { describe, expect, it } from "vitest"

const src = readFileSync(resolve(import.meta.dirname, "./ProgramCenterApp.vue"), "utf8")

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
    .replace(/\(r as any\)/g, "r")
    .replace(/const r: any/g, "const r")
    .replace(/let body: unknown/g, "let body")
    .replace(/let body: any/g, "let body")
    .replace(/let data: any/g, "let data")
    .replace(/catch \(e: any\)/g, "catch (e)")
    .replace(/catch \(e: unknown\)/g, "catch (e)")
    .replace(/\(r as any\)/g, "r")
    .replace(/as any/g, "")
}

const helperBody = safeBody("function writeSucceeded(")
const succeeded = helperBody
  ? (new Function("resp", stripTs(helperBody)) as (r: unknown) => boolean)
  : (() => false) as (r: unknown) => boolean
const AsyncFn = Object.getPrototypeOf(async function () {}).constructor
const guardBody = safeBody("async function guardedWrite(")
const guarded = guardBody
  ? ((p: Promise<unknown>): Promise<unknown> => (new AsyncFn("writeSucceeded", "p", stripTs(guardBody)))(succeeded, p))
  : (async (p: Promise<unknown>): Promise<unknown> => await p)
const FN_PARAMS = "api,toast,loadAgents,loadScripts,loadDict,loadConfigs,loadInvokes,checkVersion,prompt,loadCollect,confirmMsg,encodeURIComponent,writeSucceeded,guardedWrite,a,iv,m,kind,op,agentEdit,scriptEdit,scriptEditSaving,showScriptEdit,dictDataSaving,dictDataText,dictDataError,dictDataFlag,showDictData,agentSaving,showEditAgent"
function compile(signature: string): ((...args: unknown[]) => Promise<void>) | null {
  const body = safeBody(signature)
  return body ? (new AsyncFn(FN_PARAMS, stripTs(body)) as (...args: unknown[]) => Promise<void>) : null
}

interface CtxOpts { resp: unknown; prompts?: string[]; confirmed?: boolean; reject?: Error }
function ref<T>(v: T): { value: T } { return { value: v } }
function makeCtx(opts: CtxOpts) {
  const toasts: { success: string[]; error: string[]; warning: string[] } = { success: [], error: [], warning: [] }
  const capturedMutations: Record<string, any> = {}
  const queue = opts.prompts ? opts.prompts.slice() : []
  const api = {
    get: async (): Promise<unknown> => { if (opts.reject) throw opts.reject; return opts.resp },
    post: async (): Promise<unknown> => { if (opts.reject) throw opts.reject; return opts.resp },
    put: async (): Promise<unknown> => { if (opts.reject) throw opts.reject; return opts.resp },
    delete: async (): Promise<unknown> => { if (opts.reject) throw opts.reject; return opts.resp },
  }
  const toast = {
    success: (msg: string): void => { toasts.success.push(msg) },
    error: (msg: string): void => { toasts.error.push(msg) },
    warning: (msg: string): void => { toasts.warning.push(msg) },
  }
  const noop = (): void => {}
  const agent = { id: "A367", flag: "f367", enabled: false, name: "probe" }
  const values: unknown[] = [
    api, toast, noop, noop, noop, noop, noop, noop,
    (): string => queue.shift() ?? "",
    noop,
    async (): Promise<boolean> => opts.confirmed !== false,
    encodeURIComponent, succeeded, guarded,
    agent, { id: "I367", name: "invoke" }, { id: "M367", name: "market" },
    "appTop", "warnlog",
    ref({ id: "A367", name: "probe", flag: "f367", description: "" }),
    ref({ flag: "s367", name: "script", content: "" }), ref(false), ref(false),
    ref(false), ref("{}"), ref(""), ref("d367"), ref(false), ref(false), ref(false),
  ]
  const useMutation = (cfg: any): any => {
    const tag = String(cfg?.mutationFn ?? "").match(/program_center\/[a-z_/]+/)?.[0] ?? String(cfg?.mutationFn ?? "")
    capturedMutations[tag] = cfg
    return { mutate: (_v: unknown): void => {} }
  }
  const run = async (sig: string): Promise<void> => {
    const fn = compile(sig)
    expect(fn, "函数体应可提取: " + sig).not.toBeNull()
    await fn!(...values)
  }
  return { toasts, run, api, toast, succeeded, guarded }
}
const all = (c: ReturnType<typeof makeCtx>): string[] => c.toasts.success.concat(c.toasts.error, c.toasts.warning)

describe("R367 writeSucceeded / guardedWrite 判定 helper 真执行", () => {
  it("type:success 判成功", () => { expect(succeeded({ type: "success", data: {}, success: true })).toBe(true) })
  it("type 缺省且 success:true 判成功", () => { expect(succeeded({ success: true, data: {} })).toBe(true) })
  it("type:success 但 success:false 判失败", () => { expect(succeeded({ type: "success", success: false })).toBe(false) })
  it("type:error 判失败", () => { expect(succeeded({ type: "error", message: "agent not found" })).toBe(false) })
  it("success:false 判失败", () => { expect(succeeded({ success: false, message: "not found" })).toBe(false) })
  it("undefined/非对象判失败", () => { expect(succeeded(undefined)).toBe(false); expect(succeeded(null)).toBe(false) })
  it("guardedWrite 对 type:error 抛后端 message", async () => {
    await expect(guarded(Promise.resolve({ type: "error", message: "R367 agent not found" }))).rejects.toThrow("R367 agent not found")
  })
  it("guardedWrite 对 type:success 透传响应", async () => {
    await expect(guarded(Promise.resolve({ type: "success", success: true }))).resolves.toMatchObject({ type: "success" })
  })
})

const CASES = [
  { name: "toggleAgent", sig: "async function toggleAgent(", prompts: [], ok: "已启用" },
  { name: "executeAgent", sig: "async function executeAgent(", prompts: [], ok: "已触发执行" },
  { name: "saveAgentEdit", sig: "async function saveAgentEdit(", prompts: [], ok: "Agent 属性已保存" },
  { name: "saveDictData", sig: "async function saveDictData(", prompts: [], ok: "字典数据已保存" },
  { name: "saveScript", sig: "async function saveScript(", prompts: [], ok: "脚本已保存" },
  { name: "pcStyleErase", sig: "async function pcStyleErase(", prompts: [], ok: "已清除风格图片" },
  { name: "pcCollectDelete", sig: "async function pcCollectDelete(", prompts: ["C367"], ok: "采集配置已删除" },
  { name: "pcInvokeSaveById", sig: "async function pcInvokeSaveById(", prompts: ["I367", "n367"], ok: "接口配置已保存" },
  { name: "pcInvokeDeleteById", sig: "async function pcInvokeDeleteById(", prompts: ["I367"], ok: "接口配置已删除" },
  { name: "pcConfigSave", sig: "async function pcConfigSave(", prompts: [], ok: "配置已保存" },
  { name: "pcTokenThreshold", sig: "async function pcTokenThreshold(", prompts: ["1000"], ok: "令牌阈值已更新" },
  { name: "pcU3", sig: "async function pcU3(", prompts: [], ok: "程序中心操作已提交" },
  { name: "pcU4Write", sig: "async function pcU4Write(", prompts: [], ok: "程序中心操作已提交" },
  { name: "pcU5Write", sig: "async function pcU5Write(", prompts: [], ok: "程序中心写操作已提交" },
  { name: "pcU6", sig: "async function pcU6(", prompts: [], ok: "程序中心操作已提交" },
  { name: "pcU7", sig: "async function pcU7(", prompts: [], ok: "程序中心操作已提交" },
  { name: "pcU19", sig: "async function pcU19(", prompts: ["{}"], ok: "模块输出同步已触发" },
  { name: "installMarket", sig: "async function installMarket(", prompts: [], ok: "安装/更新已触发" },
  { name: "uninstallMarket", sig: "async function uninstallMarket(", prompts: [], ok: "卸载已触发" },
]

describe("R367 ProgramCenterApp 裸写函数 type:error 误报成功修复", () => {
  for (const c of CASES) {
    it(c.name + ": 200+type:error 不弹成功且显示后端 message", async () => {
      const ctx = makeCtx({ resp: { type: "error", message: "R367 agent not found", success: false } })
      await ctx.run(c.sig)
      expect(ctx.toasts.success.join("|")).not.toContain(c.ok)
      expect(all(ctx).join("|")).toContain("R367 agent not found")
    })
    it(c.name + ": 200+type:success 仍成功提示", async () => {
      const ctx = makeCtx({ resp: { type: "success", success: true, data: {} } })
      await ctx.run(c.sig)
      expect(ctx.toasts.success.join("|")).toContain(c.ok)
    })
    it(c.name + ": 非2xx由catch错误提示且不误报成功", async () => {
      const ctx = makeCtx({ resp: {}, reject: new Error("HTTP 500: Internal Server Error") })
      await ctx.run(c.sig)
      expect(ctx.toasts.success.length).toBe(0)
    })
  }
})

// ── t182 F1: useMutation onSuccess 回调真执行 ──
function onSuccessBody(name: string): string {
  const at = src.indexOf("const " + name + " = useMutation(")
  if (at < 0) throw new Error("未找到 mutation: " + name)
  const open = src.indexOf("{", src.indexOf("onSuccess:", at))
  let depth = 0
  for (let i = open; i < src.length; i++) {
    if (src[i] === "{") depth++
    else if (src[i] === "}") { depth--; if (depth === 0) return src.slice(open + 1, i) }
  }
  throw new Error("onSuccess 花括号未配对: " + name)
}
const MUTATION_CASES = [
  { name: "createAgentM", ok: "Agent已创建" },
  { name: "deleteAgentM", ok: "Agent已删除" },
  { name: "deleteAppM", ok: "Application已删除" },
  { name: "deleteScriptM", ok: "Script已删除" },
  { name: "deleteDictM", ok: "字典已删除" },
  { name: "createDictM", ok: "字典已创建" },
  { name: "compareM", ok: "对比完成" },
  { name: "runScriptM", ok: "脚本已执行" },
  { name: "eraseImageM", ok: "图片已清除" },
]
function runOnSuccess(name: string, resp: unknown, ctx: ReturnType<typeof makeCtx>): void {
  const body = stripTs(onSuccessBody(name))
  const noop = (): void => {}
  const compileOne = new Function("r", "toast", "loadAgents", "loadApps", "loadScripts", "loadDict", "showCreateAgent", "agentForm", "showCreateDict", "dictForm", "writeSucceeded", body)
  compileOne(resp, ctx.toast, noop, noop, noop, noop, ref(false), ref({ name: "", flag: "" }), ref(false), ref({ name: "", dictFlag: "" }), ctx.succeeded)
}
describe("t182 F1 useMutation onSuccess type:error 误报成功修复", () => {
  for (const c of MUTATION_CASES) {
    it(c.name + ": 200+type:error 走 toast.error 且不弹成功", () => {
      const ctx = makeCtx({ resp: {} })
      const bad = { type: "error", message: "R367 agent not found", success: false }
      runOnSuccess(c.name, bad, ctx)
      expect(ctx.toasts.success.join("|")).not.toContain(c.ok)
      expect(all(ctx).join("|")).toContain("R367 agent not found")
    })
    it(c.name + ": 200+type:success 仍成功提示", () => {
      const ctx = makeCtx({ resp: {} })
      runOnSuccess(c.name, { type: "success", success: true, data: {} }, ctx)
      expect(ctx.toasts.success.join("|")).toContain(c.ok)
    })
  }
})

// ── t182 F2: pcOpenPlatform 7 分支真执行 ──
const OPEN_OPS = [
  { op: "tokenPut", prompts: ["R367-token"], ok: "系统令牌已更新" },
  { op: "thresholdPut", prompts: ["1000"], ok: "令牌阈值（PUT）已更新" },
  { op: "invokeTokenExec", prompts: ["f367", "web", "R367-token"], ok: "令牌执行已提交" },
  { op: "moduleComparePost", prompts: [], ok: "模块对比上传已提交" },
  { op: "moduleComparePut", prompts: [], ok: "模块对比同步(PUT)已提交" },
  { op: "avatarPut", prompts: ["R367-path"], ok: "登录头像图已保存" },
  { op: "avatarEraseGet", prompts: [], ok: "登录头像图已清除" },
]
async function runOpenOp(op: string, prompts: string[], resp: unknown): Promise<ReturnType<typeof makeCtx>> {
  const ctx = makeCtx({ resp })
  const fn = compile("async function pcOpenPlatform(")!
  const queue = prompts.slice()
  const noop = (): void => {}
  // FN_PARAMS 顺序：api,toast,loadAgents×6,prompt,loadCollect,confirmMsg,encodeURIComponent,writeSucceeded,guardedWrite,a,iv,m,kind,op,agentEdit,scriptEdit,scriptEditSaving,showScriptEdit,dictDataSaving,dictDataText,dictDataError,dictDataFlag,showDictData,agentSaving,showEditAgent
  const values: unknown[] = [
    ctx.api, ctx.toast, noop, noop, noop, noop, noop, noop,
    (): string => queue.shift() ?? "", noop, async (): Promise<boolean> => true, encodeURIComponent, ctx.succeeded, ctx.guarded,
    { id: "A367" }, { id: "I367" }, { id: "M367" },
    "appTop", op,
    ref({ id: "A367" }), ref({ flag: "s367" }), ref(false), ref(false), ref(false), ref("{}"), ref(""), ref("d367"), ref(false), ref(false), ref(false),
  ]
  await fn(...values)
  return ctx
}
describe("t182 F2 pcOpenPlatform 7 分支 type:error 误报成功修复", () => {
  for (const c of OPEN_OPS) {
    it(c.op + ": 200+type:error 走 catch toast.error 且不弹成功", async () => {
      const ctx = await runOpenOp(c.op, c.prompts, { type: "error", message: "R367 agent not found", success: false })
      expect(ctx.toasts.success.join("|")).not.toContain(c.ok)
      expect(all(ctx).join("|")).toContain("R367 agent not found")
    })
    it(c.op + ": 200+type:success 仍成功提示", async () => {
      const ctx = await runOpenOp(c.op, c.prompts, { type: "success", success: true, data: {} })
      expect(ctx.toasts.success.join("|")).toContain(c.ok)
    })
  }
})

// ── t182 F3: createInvoke/deleteInvoke + pcU10/13/15/16/17 真执行两态 ──
const F3_CASES = [
  { name: "createInvoke", sig: "async function createInvoke(", prompts: ["I367"] },
  { name: "deleteInvoke", sig: "async function deleteInvoke(", prompts: [] },
  { name: "pcU10", sig: "async function pcU10(", prompts: [], ok: "集成同步已触发" },
  { name: "pcU13", sig: "async function pcU13(", prompts: [], ok: "同步/异常日志已上报" },
  { name: "pcU15", sig: "async function pcU15(", prompts: ["pack367", "1.0.0", ""], ok: "打包/同步已提交" },
  { name: "pcU16", sig: "async function pcU16(", prompts: ["", ""], ok: "注册/输出/调用器已提交" },
  { name: "pcU17", sig: "async function pcU17(", prompts: [""], ok: "部署/注册已提交" },
]
describe("t182 F3 同类残留 type:error 误报成功修复", () => {
  for (const c of F3_CASES) {
    it(c.name + ": 200+type:error 不弹成功且显示后端 message", async () => {
      const ctx = makeCtx({ resp: { type: "error", message: "R367 agent not found", success: false }, prompts: c.prompts, confirmed: true })
      await ctx.run(c.sig)
      if (c.ok) expect(ctx.toasts.success.join("|")).not.toContain(c.ok)
      expect(all(ctx).join("|")).toContain("R367 agent not found")
    })
    it(c.name + ": 200+type:success 仍成功提示", async () => {
      const ctx = makeCtx({ resp: { type: "success", success: true, data: {} }, prompts: c.prompts, confirmed: true })
      await ctx.run(c.sig)
      if (c.ok) expect(ctx.toasts.success.join("|")).toContain(c.ok)
    })
  }
})
