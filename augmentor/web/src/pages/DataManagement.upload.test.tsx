/**
 * `DataManagement` 的上传面：渲染出来的下拉框与失败提示
 *
 * 这份用例的存在理由是把 A162 的两格从「文本扫描」升级成「真渲染」：
 * Python 侧的守卫（`tests/unit/test_upload_declared_format_l92.py`）只能证明源码里
 * 写了那个属性，证明不了 antd 的控件真的把它渲染出来、也证明不了失败时用户看得见
 * 服务端那句判决。渲染档补的正是这半格。
 *
 * 服务层用 `importOriginal` 做**部分替身**：`apiErrorDetail` / `bulkErrorDetail` /
 * `isTimeoutError` 必须留真身，否则本用例断言的是替身的行为而不是页面的行为。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { message } from 'antd'

// antd v5 的主题与响应式 hook 依赖这两个浏览器 API，jsdom 不提供。
window.matchMedia =
  window.matchMedia ||
  ((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: () => {},
    removeListener: () => {},
    addEventListener: () => {},
    removeEventListener: () => {},
    dispatchEvent: () => false,
  }) as unknown as MediaQueryList)
class RO {
  observe() {}
  unobserve() {}
  disconnect() {}
}
window.ResizeObserver = window.ResizeObserver || (RO as unknown as typeof ResizeObserver)

const spies = vi.hoisted(() => ({
  getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
  loadData: null as unknown as ReturnType<typeof vi.fn>,
  uploadData: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.getDataFiles = vi.fn().mockResolvedValue({ files: [] })
  spies.loadData = vi.fn().mockResolvedValue({ items: [], total: 0 })
  spies.uploadData = vi.fn().mockResolvedValue({ success: true, path: 'x.json', count: 2 })
  return { ...actual, getDataFiles: spies.getDataFiles, loadData: spies.loadData,
    uploadData: spies.uploadData }
})

import DataManagement from './DataManagement'

const LIE_DETAIL =
  '上传内容整份是一份合法 JSON，但按 csv 解析：名字或 input_format 与内容不符。' +
  'JSON 请去掉扩展名或用 input_format=json，JSONL 请每行一条记录'

/**
 * 卸载与清场
 *
 * vitest 配置里 `globals` 没开，RTL 的自动 cleanup 因此**不生效**（它挂在
 * `globalThis.afterEach` 上）—— 实测第二条用例就会因为 body 里同时存在两个
 * `<select>` 而报「multiple elements」。antd 的 `message` 容器挂在 RTL 容器之外，
 * 必须显式 `destroy()`，否则上一条用例的失败提示会替下一条把断言蒙对。
 */
afterEach(() => {
  cleanup()
  message.destroy()
})

beforeEach(() => {
  spies.uploadData.mockClear()
})

describe('上传控件的渲染面', () => {
  it('下拉框渲染出「自动」+ 读边那 13 种格式，默认是自动', () => {
    render(<DataManagement />)

    const select = screen.getByLabelText('上传源格式')
    expect(select).toHaveProperty('value', '')
    const options = Array.from(select.querySelectorAll('option')).map((o) => o.value)
    // 14 = 「自动」+ 13 项；与权威表的**内容**一致性由 Python 侧每轮全量守卫，
    // 这里只证明「渲染出来的确实这些项」，不重复维护第二份名单。
    expect(options).toHaveLength(14)
    expect(options[0]).toBe('')
    expect(options).toContain('alpaca')
  })

  it('选中 csv 后选文件，声明值真的跟着文件进了服务层', async () => {
    const { container } = render(<DataManagement />)

    fireEvent.change(screen.getByLabelText('上传源格式'), { target: { value: 'csv' } })
    const file = new File(['a,b\n1,2'], 'lie.csv', { type: 'text/csv' })
    await pickFile(container, file)

    await waitFor(() => expect(spies.uploadData).toHaveBeenCalledWith(file, 'csv'))
  })

  it('失败提示显示服务端那句可 actions 的判决，而不是「上传失败」', async () => {
    spies.uploadData.mockRejectedValue({ response: { data: { detail: LIE_DETAIL } } })
    const { container } = render(<DataManagement />)

    fireEvent.change(screen.getByLabelText('上传源格式'), { target: { value: 'csv' } })
    await pickFile(container, new File(['a,b\n1,2'], 'lie.csv', { type: 'text/csv' }))

    // antd 的 `message.error` 渲进 body 而不是组件容器，所以查 document。
    // 这条钉的是 A162 ② 的**产品后果**：用户看得见「按 csv 解析」那句，而不是一句「上传失败」。
    await waitFor(
      () => expect(document.body.textContent).toContain('上传内容整份是一份合法 JSON'),
      { timeout: 4000 }
    )
  })

  /**
   * 超时不是「失败」，而且要有动作陪着那句话
   *
   * 这条钉的是 A167 的产品后果：客户端停止等待时服务端往往还在跑（实测那份
   * 45 840 001 B 的文件在客户端 0.60 s 放弃之后照样落盘）。所以说「上传失败」
   * 会把用户推向重试＝同一份数据落两次。页面在这里要做两件事：说清「可能已落盘」，
   * 并主动重读一次列表让已经落盘的文件自己现身。
   */
  it('上传超时说的是「服务端可能已落盘」，并且真的重读了一次列表', async () => {
    spies.uploadData.mockRejectedValueOnce({
      code: 'ECONNABORTED',
      message: 'timeout of 1800000ms exceeded',
    })
    const { container } = render(<DataManagement />)
    const baseline = spies.getDataFiles.mock.calls.length

    await pickFile(container, new File(['a,b\n1,2'], 'big.csv', { type: 'text/csv' }))

    await waitFor(() => expect(document.body.textContent).toContain('客户端已停止等待'), {
      timeout: 4000,
    })
    expect(document.body.textContent).toContain('已把结果落盘')
    expect(spies.getDataFiles.mock.calls.length).toBeGreaterThan(baseline)
  })
})

/**
 * 驱动 antd 的上传控件：把文件塞进那个隐藏 input 并派发 change
 *
 * `beforeUpload` 在 rc-upload 里返回 `false` 就会中止后续（不发 XHR），所以这里只可能
 * 走到页面自己的 `handleUpload`，测的就是「控件 → 状态 → 服务层」这段真接线。
 */
async function pickFile(container: HTMLElement, file: File): Promise<void> {
  const input = container.querySelector('input[type="file"]') as HTMLInputElement
  expect(input).toBeTruthy()
  Object.defineProperty(input, 'files', { value: [file], configurable: true })
  fireEvent.change(input)
  // rc-upload 在 beforeUpload 之前有一层 `await`（它把结果当 Promise 处理），
  // 交还一次微任务/宏任务队列才能看到 `uploadData` 的调用。
  await new Promise((resolve) => setTimeout(resolve, 0))
}
