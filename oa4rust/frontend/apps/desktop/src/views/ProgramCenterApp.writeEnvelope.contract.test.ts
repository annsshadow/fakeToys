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
  const run = async (sig: string): Promise<void> => {
    const fn = compile(sig)
    expect(fn, "函数体应可提取: " + sig).not.toBeNull()
    await fn!(...values)
  }
  return { toasts, run }
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
