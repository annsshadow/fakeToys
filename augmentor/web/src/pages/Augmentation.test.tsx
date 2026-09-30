/**
 * Augmentation 页的渲染面（L107，A206）
 *
 * 数据增强页此前零组件测试。四条要点：① 未选输入文件就点「开始增强」得到
 * warning、不调服务层；② 选了文件后 startAugmentation 收到五个参数（输入/输出/
 * 三个开关）且默认输出名是 augmented_output.json；③ 加载文件列表失败有可见文案；
 * ④ 轮询拿到带 task_id 的进度后渲染任务详情（否则显示「暂无进行中的任务」）。
 *
 * 轮询用 `setInterval(2s)`：进度用例用假定时器推进一拍，其余用例把 getProgress
 * 固定成 `{ status: 'no_checkpoint' }`（不带 task_id ⇒ 不进详情分支）。
 */
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'
import { cleanup, fireEvent, render, screen, waitFor } from '@testing-library/react'
import { message } from 'antd'

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
  startAugmentation: null as unknown as ReturnType<typeof vi.fn>,
  getProgress: null as unknown as ReturnType<typeof vi.fn>,
  getDataFiles: null as unknown as ReturnType<typeof vi.fn>,
}))

vi.mock('../services/api', async (importOriginal) => {
  const actual = await importOriginal<typeof import('../services/api')>()
  spies.startAugmentation = vi.fn().mockResolvedValue({ success: true, message: 'ok' })
  spies.getProgress = vi.fn().mockResolvedValue({ status: 'no_checkpoint' })
  spies.getDataFiles = vi.fn().mockResolvedValue({ files: [{ name: 'seed.json' }] })
  return {
    ...actual,
    startAugmentation: spies.startAugmentation,
    getProgress: spies.getProgress,
    getDataFiles: spies.getDataFiles,
  }
})

import Augmentation from './Augmentation'

afterEach(() => {
  cleanup()
  message.destroy()
  vi.clearAllMocks()
  vi.useRealTimers()
})

describe('Augmentation 启动校验与负载', () => {
  beforeEach(() => {
    spies.getProgress.mockResolvedValue({ status: 'no_checkpoint' })
  })

  it('未选输入文件时点开始得到 warning，不调服务层', async () => {
    render(<Augmentation />)

    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
    fireEvent.click(screen.getByRole('button', { name: /开始增强/ }))

    await waitFor(() => expect(document.body.textContent).toContain('请选择输入文件'))
    expect(spies.startAugmentation).not.toHaveBeenCalled()
  })

  it('选文件后 startAugmentation 收到五参数与默认输出名', async () => {
    const { container } = render(<Augmentation />)

    await waitFor(() => expect(spies.getDataFiles).toHaveBeenCalled())
    // 直接驱动受控 Select：mousedown 打开后点选项（用 option 类精确定位，避免与
    // 选中态回显的同名节点冲突）
    fireEvent.mouseDown(container.querySelector('.ant-select-selector')!)
    await waitFor(() =>
      expect(document.querySelector('.ant-select-item-option')).toBeInTheDocument()
    )
    fireEvent.click(document.querySelector('.ant-select-item-option')!)

    fireEvent.click(screen.getByRole('button', { name: /开始增强/ }))

    await waitFor(() => expect(spies.startAugmentation).toHaveBeenCalledTimes(1))
    expect(spies.startAugmentation).toHaveBeenCalledWith(
      'seed.json',
      'augmented_output.json',
      true,
      true,
      true
    )
  })
})

describe('Augmentation 文件列表失败', () => {
  it('加载失败时看得见专属文案', async () => {
    spies.getDataFiles.mockRejectedValue(new Error('boom'))

    render(<Augmentation />)

    await waitFor(() => expect(document.body.textContent).toContain('加载文件列表失败'), {
      timeout: 4000,
    })
  })
})

describe('Augmentation 进度轮询', () => {
  it('轮询拿到带 task_id 的进度后渲染任务详情', async () => {
    vi.useFakeTimers()
    spies.getProgress.mockResolvedValue({
      task_id: 't-42',
      total_items: 10,
      processed_items: 4,
      failed_items: 1,
      progress: 0.4,
      progress_percent: '40.0%',
      elapsed_time: '0:00:12',
      remaining_time: '0:00:18',
      start_time: 's',
      last_update: 'u',
      avg_quality_score: 0.812,
    })

    render(<Augmentation />)
    // 推进一拍触发第一次 checkProgress，并放行其 await 链
    await vi.advanceTimersByTimeAsync(2100)
    vi.useRealTimers()

    await waitFor(() => expect(screen.getByText('t-42')).toBeInTheDocument())
    expect(screen.getByText('平均质量评分: 0.812')).toBeInTheDocument()
  })
})
