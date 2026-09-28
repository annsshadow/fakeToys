/**
 * 视图测试共享工具。
 *
 * 约定：一律 mount 真组件 + 真实 Element Plus（组件树真实渲染），
 * 只 mock 网络层（@/api）与 ElMessageBox（命令式弹窗需要人工点击，无法在测试里交互）。
 * ElMessage 保持真实实现，断言直接查 document.body 上真实渲染出的消息条。
 */
import { flushPromises, mount, type VueWrapper } from '@vue/test-utils'
import ElementPlus from 'element-plus'
import { nextTick } from 'vue'
import { vi } from 'vitest'

/** 等待一轮微任务；ms > 0 时额外等待真实定时器（如 DashboardView 里 30ms 的图表延迟） */
export async function settle(wrapper?: VueWrapper, ms = 0): Promise<void> {
  if (wrapper) await wrapper.vm.$nextTick()
  await flushPromises()
  if (ms > 0) await new Promise((r) => setTimeout(r, ms))
  await flushPromises()
  await nextTick()
}

/** 按可见文本点击按钮；找不到时显式失败，避免静默跳过 */
export async function clickButtonByText(wrapper: VueWrapper, text: string): Promise<void> {
  const btn = wrapper.findAll('button').find((b) => b.text().includes(text))
  if (!btn) throw new Error(`找不到文本为「${text}」的按钮`)
  await btn.trigger('click')
  await flushPromises()
}

/** ElMessage 是真实渲染到 body 的，直接查全文 */
export function bodyText(): string {
  return document.body.textContent ?? ''
}

/** mount 并自动装上真实 Element Plus（注册全部 el-* 组件与 v-loading 指令） */
export function mountEl<T>(comp: T, options: Record<string, unknown> = {}): VueWrapper {
  const { global, ...rest } = options
  const g = (global ?? {}) as { plugins?: unknown[]; components?: Record<string, unknown> }
  return mount(comp as never, {
    ...rest,
    global: {
      plugins: [ElementPlus, ...(g.plugins ?? [])],
      components: g.components,
    },
  })
}

/** ElMessageBox 的受控替身：测试里用 box.prompt.mockResolvedValueOnce({ value }) 决定弹窗结果 */
export function createMessageBoxStub() {
  return {
    prompt: vi.fn(),
    confirm: vi.fn(),
  }
}
