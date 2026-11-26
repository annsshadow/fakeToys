// Copyright (C) 2026 annsshadow
// SPDX-License-Identifier: AGPL-3.0-or-later
/**
 * t24 [P0] PC 日历页 TDZ 崩溃回归契约（RED→GREEN）。
 *
 * 复现机理：CalendarApp.vue 的 setup 在 calListError / calEventsError / evtForm 等
 * ref 声明之前同步调用 loadCalendars('my')，而该函数首行即写 calListError.value=''。
 * <script setup> 编译后这些调用与声明保持源顺序，真实浏览器进入 /app/calendar 时
 * 异步链路以 ReferenceError（压缩产物 Cannot access 'q' before initialization）失败，
 * 页面渲染错误占位。旧有 8 条源码结构测试没有捕获这一运行时缺陷。
 *
 * 本契约用 @vue/compiler-sfc 真实编译组件 <script setup>，配合 TypeScript 类型擦除，
 * 在隔离 vm 中注入真实 ref/computed/api/useQuery/日历工具桩后执行 setup：
 *   - RED：修复前调用链以 TDZ ReferenceError 失败；
 *   - GREEN：修复后调用链正常完成，业务绑定完整，不渲染错误提示。
 */
import { readFileSync } from 'node:fs'
import { createRequire } from 'node:module'
import path from 'node:path'
import vm from 'node:vm'
import { fileURLToPath } from 'node:url'
import { describe, expect, it } from 'vitest'

const here = path.dirname(fileURLToPath(import.meta.url))
const frontendRoot = path.resolve(here, '../../../..')
const workspaceRoot = path.resolve(frontendRoot, '..')
const sourcePath = path.join(frontendRoot, 'apps/desktop/src/views/CalendarApp.vue')
const source = readFileSync(sourcePath, 'utf8')

const requireFromFrontend = createRequire(path.join(frontendRoot, 'noop.cjs'))
const sfcRoot = path.resolve(workspaceRoot, 'node_modules/.pnpm/@vue+compiler-sfc@3.5.42/node_modules/@vue/compiler-sfc')
const compiler = requireFromFrontend(path.join(sfcRoot, 'dist/compiler-sfc.cjs.js'))
const ts = requireFromFrontend('typescript')

function typeErase(code: string): string {
  return ts.transpileModule(code, {
    compilerOptions: {
      target: ts.ScriptTarget.ES2022,
      module: ts.ModuleKind.ESNext,
      verbatimModuleSyntax: false,
      isolatedModules: true,
    },
    fileName: 'CalendarApp.setup.ts',
  }).outputText
}

function buildSetup(): vm.Script {
  const start = source.indexOf('<script setup')
  const open = source.indexOf('>', start)
  const close = source.indexOf('</script>', open)
  expect(start, 'script setup 不存在').toBeGreaterThan(-1)
  const body = source.slice(open + 1, close)
  const descriptor = compiler.parse('<script setup>' + typeErase(body) + '</script>').descriptor
  const compiled = compiler.compileScript(descriptor, { id: 't24-calendar', inlineTemplate: false })
  const commonjs = ts.transpileModule(typeErase(compiled.content), {
    compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS },
    fileName: 'CalendarApp.compiled.ts',
  }).outputText
  return new vm.Script(commonjs, { filename: 'CalendarApp.compiled.cjs' })
}

type RunResult = { bindings: Record<string, any> | null; error: unknown; apiCalls: string[]; errors: string[] }

function runSetup(): RunResult {
  const apiCalls: string[] = []
  const errors: string[] = []
  const reactive = (initial: unknown) => ({ value: initial })
  const api = {
    get: async (url: string) => { apiCalls.push('GET ' + url); return { data: { list: [] } } },
    post: async (url: string) => { apiCalls.push('POST ' + url); return { data: {} } },
    put: async (url: string) => { apiCalls.push('PUT ' + url); return { data: [] } },
    del: async (url: string) => { apiCalls.push('DELETE ' + url); return { data: {} } },
  }
  const sandbox: Record<string, any> = {
    exports: {},
    module: { exports: {} },
    require: (id: string) => {
      switch (id) {
        case 'vue':
          return { ref: reactive, computed: (fn: () => unknown) => ({ value: fn() }) }
        case '@oa4rust/sdk':
          return { api }
        case '@tanstack/vue-query':
          return { useQuery: () => ({ data: reactive([]) }) }
        case '../utils/calendarGrid':
          return {
            calendarBusinessToday: () => ({ year: 2026, month: 11, day: 21 }),
            calendarCellKey: (y: number, m: number, d: number) => y + '-' + m + '-' + d,
            shiftCalendarMonth: (y: number, m: number, delta: number) => ({ year: y, month: m + delta }),
          }
        case '../utils/toast':
          return {
            confirmMsg: async () => true,
            toast: { success: () => {}, error: (msg: string) => errors.push(String(msg)) },
          }
        case './listQuery':
          return { describeListError: () => '加载失败' }
        case './ListLoadError.vue':
          return {}
        default:
          throw new Error('unexpected import: ' + id)
      }
    },
    console,
    setTimeout,
    clearTimeout,
    Promise,
    Date,
    Math,
    String,
    Number,
    Array,
    Object,
    JSON,
    Error,
  }
  const context = vm.createContext(sandbox)
  let error: unknown = null
  let bindings: Record<string, any> | null = null
  try {
    buildSetup().runInContext(context)
    const component = sandbox.module.exports.default ?? sandbox.exports.default
    expect(component, '编译产物缺少默认组件').toBeTruthy()
    const setupResult = component.setup({}, { expose: () => {} })
    bindings = setupResult && typeof setupResult === 'object' ? setupResult : null
  } catch (e) {
    error = e
  }
  return { bindings, error, apiCalls, errors }
}

describe('日历编辑加载 TDZ 崩溃回归（t24 P0）', () => {
  it('源码脚本顺序中 loadCalendars(my) 调用必须晚于 calListError 声明', () => {
    const script = source.slice(source.indexOf('<script setup'), source.indexOf('</script>'))
    const call = script.indexOf("loadCalendars('my')")
    const decl = script.indexOf('const calListError')
    expect(call, 'loadCalendars(my) 调用不存在').toBeGreaterThan(-1)
    expect(decl, 'calListError 声明不存在').toBeGreaterThan(-1)
    expect(call, '调用仍位于 calListError 声明之前，TDZ 未修复').toBeGreaterThan(decl)
  })

  it('真实编译后的 setup 同步阶段不再抛错，业务绑定完整', () => {
    const result = runSetup()
    expect(result.error, 'setup 不得抛同步错误：' + String(result.error)).toBeNull()
    expect(result.bindings, 'setup 必须返回模板绑定').toBeTruthy()
    const required = [
      'calListError', 'calEventsError', 'evtForm', 'myCals', 'pubCals',
      'calendarCells', 'allEvents', 'loadCalendars', 'selectCalendar', 'createEvent',
    ]
    for (const key of required) {
      expect(result.bindings, '缺少绑定: ' + key).toHaveProperty(key)
    }
  })

  it('初始我的日历列表真实进入异步加载链路，不发生 TDZ 拒绝', async () => {
    const result = runSetup()
    expect(result.bindings, 'setup 必须返回模板绑定').toBeTruthy()
    // setup 内 loadCalendars('my') 在首个 await 前只读 calListError，TDZ 在同步阶段即抛出；
    // 若后续改为延迟调用，给微任务一个周期后断言 API 真的被调用。
    await Promise.resolve()
    await Promise.resolve()
    expect(result.error, 'setup 启动阶段不得抛错：' + String(result.error)).toBeNull()
    expect(result.apiCalls, '应实际调用我的日历列表接口').toContain('GET /api/calendar/calendar/list/my')
    expect(result.errors, '默认加载不得渲染错误提示').toEqual([])
  })
})